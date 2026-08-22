//! Tauri shell: typed IPC commands over `sill-core` plus two pump threads
//! (docs/design/ipc.md):
//! - dirty pump: coalesces per-session dirty notifications to ~one snapshot
//!   emit per frame (16ms) — a flooding `yes`(1) costs one render per frame,
//!   not one per read
//! - event pump: forwards typed session events (title/bell/exit/…)
//!
//! Commands are verbs on the session model; the webview never gets generic
//! power (threat model T4).

use std::collections::HashSet;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use sill_core::{SessionEvent, SessionId, SessionManager, SessionOptions, Snapshot};

struct Core {
    manager: std::sync::Arc<SessionManager>,
}

#[derive(Clone, Serialize)]
struct SnapshotPayload {
    id: SessionId,
    snapshot: Snapshot,
}

#[derive(Clone, Serialize)]
struct EventPayload {
    event: SessionEvent,
}

/// Frame window for coalescing snapshot emits.
const FRAME: Duration = Duration::from_millis(16);

fn emit_snapshot(app: &AppHandle, manager: &SessionManager, id: SessionId) {
    if let Ok(snapshot) = manager.snapshot(id) {
        let _ = app.emit("sill://snapshot", SnapshotPayload { id, snapshot });
    }
}

#[tauri::command]
fn create_session(
    state: State<'_, Core>,
    cols: u16,
    rows: u16,
    shell: Option<String>,
) -> Result<SessionId, String> {
    state
        .manager
        .create(SessionOptions {
            cols,
            rows,
            shell,
            cwd: None,
            scrollback_lines: None,
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn session_input(state: State<'_, Core>, id: SessionId, data: String) -> Result<(), String> {
    state
        .manager
        .input(id, data.as_bytes())
        .map_err(|e| e.to_string())
}

/// Paste text into a session. Bracketed-paste wrapping is decided in RUST
/// against the engine's live mode — the frontend never caches protocol
/// state (a stale cache strips delimiters right after a program enables
/// the mode).
#[tauri::command]
fn session_paste(state: State<'_, Core>, id: SessionId, text: String) -> Result<(), String> {
    state.manager.paste(id, &text).map_err(|e| e.to_string())
}

#[tauri::command]
fn session_resize(
    state: State<'_, Core>,
    id: SessionId,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    state
        .manager
        .resize(id, cols, rows)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn session_scroll(state: State<'_, Core>, id: SessionId, delta: i32) -> Result<(), String> {
    state.manager.scroll(id, delta).map_err(|e| e.to_string())
}

#[tauri::command]
fn session_scroll_to_bottom(state: State<'_, Core>, id: SessionId) -> Result<(), String> {
    state
        .manager
        .scroll_to_bottom(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn session_snapshot(state: State<'_, Core>, id: SessionId) -> Result<Snapshot, String> {
    state.manager.snapshot(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn session_kill(state: State<'_, Core>, id: SessionId) -> Result<(), String> {
    state.manager.kill(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn session_close(state: State<'_, Core>, id: SessionId) -> Result<(), String> {
    state.manager.close(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_sessions(state: State<'_, Core>) -> Vec<SessionId> {
    state.manager.list()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (manager, events_rx, dirty_rx) = SessionManager::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Core {
            manager: manager.clone(),
        })
        .setup(move |app| {
            // Dirty pump: block on the first notification, then drain the
            // channel for one frame window so N flooding sessions produce
            // at most N snapshot emits per frame.
            {
                let app = app.handle().clone();
                let manager = manager.clone();
                std::thread::Builder::new()
                    .name("sill-dirty-pump".into())
                    .spawn(move || {
                        while let Ok(first) = dirty_rx.recv() {
                            let mut batch: HashSet<SessionId> = HashSet::new();
                            batch.insert(first);
                            let deadline = std::time::Instant::now() + FRAME;
                            loop {
                                let now = std::time::Instant::now();
                                if now >= deadline {
                                    break;
                                }
                                match dirty_rx.recv_timeout(deadline - now) {
                                    Ok(id) => {
                                        batch.insert(id);
                                    }
                                    Err(RecvTimeoutError::Timeout) => break,
                                    Err(RecvTimeoutError::Disconnected) => return,
                                }
                            }
                            for id in batch {
                                emit_snapshot(&app, &manager, id);
                            }
                        }
                    })
                    .expect("spawn dirty pump");
            }

            // Event pump: typed session events straight to the frontend.
            {
                let app = app.handle().clone();
                std::thread::Builder::new()
                    .name("sill-event-pump".into())
                    .spawn(move || {
                        while let Ok(event) = events_rx.recv() {
                            let _ = app.emit("sill://session-event", EventPayload { event });
                        }
                    })
                    .expect("spawn event pump");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_session,
            session_input,
            session_paste,
            session_resize,
            session_scroll,
            session_scroll_to_bottom,
            session_snapshot,
            session_kill,
            session_close,
            list_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
