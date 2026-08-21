//! Session lifecycle: PTY spawn, reader/waiter threads, typed events.
//!
//! Threading model (deliberately boring):
//! - one *reader* thread per session: blocking PTY reads → engine feed →
//!   dirty notification (coalesced by the consumer)
//! - one *waiter* thread per session: blocks on child exit → `Exited` event
//! - one *dispatcher* thread per manager: drains engine events, answers
//!   PTY write-backs (DSR/DA responses), forwards the rest as typed
//!   [`SessionEvent`]s
//!
//! Backpressure: the reader thread parses synchronously into the engine, so
//! PTY intake is naturally bounded by parse speed; the UI only ever pulls
//! bounded snapshots. No unbounded queues of terminal output exist
//! (docs/design/performance.md).

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use portable_pty::{native_pty_system, ChildKiller, CommandBuilder, MasterPty, PtySize};

use crate::engine::{EngineEvent, TermDims, TermState};
use crate::shell::resolve_shell;
use crate::snapshot::Snapshot;
use crate::{CoreError, Result};

/// Stable identifier for a terminal session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SessionId(pub u64);

/// Typed session events (docs/design/ipc.md). Command-level semantic events
/// (started/finished/failed) arrive with shell integration in Phase 3 —
/// they are deliberately absent rather than faked.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum SessionEvent {
    Created {
        id: SessionId,
    },
    TitleChanged {
        id: SessionId,
        title: String,
    },
    Bell {
        id: SessionId,
    },
    Exited {
        id: SessionId,
        exit_code: Option<u32>,
    },
    Closed {
        id: SessionId,
    },
}

/// Options for creating a session.
#[derive(Debug, Clone, Default)]
pub struct SessionOptions {
    pub cols: u16,
    pub rows: u16,
    /// Explicit shell override; otherwise resolution policy applies
    /// (crate::shell).
    pub shell: Option<String>,
    /// Working directory; defaults to the process cwd.
    pub cwd: Option<String>,
    /// Scrollback lines (default 10_000 — configurable, never unbounded).
    pub scrollback_lines: Option<usize>,
}

pub const DEFAULT_SCROLLBACK_LINES: usize = 10_000;

struct SessionInner {
    id: SessionId,
    engine: Mutex<TermState>,
    writer: Mutex<Box<dyn Write + Send>>,
    master: Mutex<Box<dyn MasterPty + Send>>,
    killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
    alive: AtomicBool,
    title: Mutex<String>,
}

/// Owns all sessions. Cheap to clone handles out of; internally locked
/// per-session so one busy session never blocks another's input.
pub struct SessionManager {
    sessions: Mutex<HashMap<SessionId, Arc<SessionInner>>>,
    next_id: AtomicU64,
    events_tx: Sender<SessionEvent>,
    /// Dirty notifications: session ids whose grid changed. The consumer
    /// coalesces (recv + drain + frame delay); senders only fire on the
    /// false→true edge so a flooding session sends one wakeup per frame,
    /// not one per read.
    dirty_tx: Sender<SessionId>,
    dirty_flags: Mutex<HashMap<SessionId, Arc<AtomicBool>>>,
}

impl SessionManager {
    /// Create a manager plus its event/dirty receivers.
    pub fn new() -> (Arc<Self>, Receiver<SessionEvent>, Receiver<SessionId>) {
        let (events_tx, events_rx) = mpsc::channel();
        let (dirty_tx, dirty_rx) = mpsc::channel();
        let mgr = Arc::new(Self {
            sessions: Mutex::new(HashMap::new()),
            next_id: AtomicU64::new(1),
            events_tx,
            dirty_tx,
            dirty_flags: Mutex::new(HashMap::new()),
        });
        (mgr, events_rx, dirty_rx)
    }

    pub fn create(self: &Arc<Self>, opts: SessionOptions) -> Result<SessionId> {
        let id = SessionId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let cols = if opts.cols == 0 { 80 } else { opts.cols };
        let rows = if opts.rows == 0 { 24 } else { opts.rows };
        let scrollback = opts
            .scrollback_lines
            .unwrap_or(DEFAULT_SCROLLBACK_LINES)
            // Hard safety cap: "never unbounded" is a product rule.
            .min(200_000);

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| CoreError::Pty(e.to_string()))?;

        let shell = resolve_shell(opts.shell.as_deref());
        let mut cmd = CommandBuilder::new(&shell);
        if let Some(cwd) = &opts.cwd {
            cmd.cwd(cwd);
        }
        cmd.env("TERM", "xterm-256color");

        let mut child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| CoreError::Pty(e.to_string()))?;
        // Slave stays open in the child; drop our handle so reader EOF works.
        drop(pair.slave);

        let killer = child.clone_killer();
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| CoreError::Pty(e.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| CoreError::Pty(e.to_string()))?;

        // Engine + its event channel (drained by the dispatcher thread).
        let (engine_tx, engine_rx) = mpsc::channel::<EngineEvent>();
        let engine = TermState::new(TermDims { cols, rows }, scrollback, engine_tx);

        let inner = Arc::new(SessionInner {
            id,
            engine: Mutex::new(engine),
            writer: Mutex::new(writer),
            master: Mutex::new(pair.master),
            killer: Mutex::new(killer),
            alive: AtomicBool::new(true),
            title: Mutex::new(String::new()),
        });

        let dirty = Arc::new(AtomicBool::new(false));
        self.dirty_flags.lock().unwrap().insert(id, dirty.clone());
        self.sessions.lock().unwrap().insert(id, inner.clone());

        // Reader thread: PTY → engine → dirty edge notification.
        {
            let inner = inner.clone();
            let dirty = dirty.clone();
            let dirty_tx = self.dirty_tx.clone();
            let mut reader = reader;
            thread::Builder::new()
                .name(format!("sill-pty-read-{}", id.0))
                .spawn(move || {
                    let mut buf = [0u8; 32 * 1024];
                    loop {
                        match reader.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                {
                                    let mut engine = inner.engine.lock().unwrap();
                                    engine.feed(&buf[..n]);
                                }
                                if !dirty.swap(true, Ordering::AcqRel) {
                                    let _ = dirty_tx.send(inner.id);
                                }
                            }
                            // EIO on Linux when the last slave closes: EOF.
                            Err(_) => break,
                        }
                    }
                })
                .expect("spawn pty reader thread");
        }

        // Dispatcher thread: engine events for THIS session.
        {
            let inner = inner.clone();
            let events_tx = self.events_tx.clone();
            let dirty = dirty.clone();
            let dirty_tx = self.dirty_tx.clone();
            thread::Builder::new()
                .name(format!("sill-events-{}", id.0))
                .spawn(move || {
                    while let Ok(ev) = engine_rx.recv() {
                        match ev {
                            EngineEvent::PtyWrite(text) => {
                                // Emulation replies (DSR/DA/…) go straight
                                // back to the PTY.
                                if let Ok(mut w) = inner.writer.lock() {
                                    let _ = w.write_all(text.as_bytes());
                                    let _ = w.flush();
                                }
                            }
                            EngineEvent::Title(title) => {
                                *inner.title.lock().unwrap() = title.clone();
                                let _ = events_tx.send(SessionEvent::TitleChanged {
                                    id: inner.id,
                                    title,
                                });
                            }
                            EngineEvent::ResetTitle => {
                                inner.title.lock().unwrap().clear();
                                let _ = events_tx.send(SessionEvent::TitleChanged {
                                    id: inner.id,
                                    title: String::new(),
                                });
                            }
                            EngineEvent::Bell => {
                                let _ = events_tx.send(SessionEvent::Bell { id: inner.id });
                            }
                            EngineEvent::Wakeup => {
                                if !dirty.swap(true, Ordering::AcqRel) {
                                    let _ = dirty_tx.send(inner.id);
                                }
                            }
                        }
                    }
                })
                .expect("spawn session event dispatcher thread");
        }

        // Waiter thread: child exit → Exited event.
        {
            let inner = inner.clone();
            let events_tx = self.events_tx.clone();
            thread::Builder::new()
                .name(format!("sill-wait-{}", id.0))
                .spawn(move || {
                    let exit_code = child.wait().ok().map(|status| status.exit_code());
                    inner.alive.store(false, Ordering::Release);
                    let _ = events_tx.send(SessionEvent::Exited {
                        id: inner.id,
                        exit_code,
                    });
                })
                .expect("spawn child waiter thread");
        }

        let _ = self.events_tx.send(SessionEvent::Created { id });
        Ok(id)
    }

    fn get(&self, id: SessionId) -> Result<Arc<SessionInner>> {
        self.sessions
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(CoreError::UnknownSession(id))
    }

    /// Write user input bytes to the session's PTY.
    pub fn input(&self, id: SessionId, bytes: &[u8]) -> Result<()> {
        let inner = self.get(id)?;
        let mut writer = inner.writer.lock().unwrap();
        writer
            .write_all(bytes)
            .and_then(|_| writer.flush())
            .map_err(|e| CoreError::Input(e.to_string()))
    }

    /// Resize PTY + emulation.
    pub fn resize(&self, id: SessionId, cols: u16, rows: u16) -> Result<()> {
        let inner = self.get(id)?;
        inner
            .master
            .lock()
            .unwrap()
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| CoreError::Pty(e.to_string()))?;
        inner.engine.lock().unwrap().resize(TermDims { cols, rows });
        self.mark_dirty(id);
        Ok(())
    }

    /// Scroll the viewport into history (positive) or toward live (negative).
    pub fn scroll(&self, id: SessionId, delta_lines: i32) -> Result<()> {
        let inner = self.get(id)?;
        inner.engine.lock().unwrap().scroll_display(delta_lines);
        self.mark_dirty(id);
        Ok(())
    }

    pub fn scroll_to_bottom(&self, id: SessionId) -> Result<()> {
        let inner = self.get(id)?;
        inner.engine.lock().unwrap().scroll_to_bottom();
        self.mark_dirty(id);
        Ok(())
    }

    /// Render-ready snapshot of the session's visible viewport. Clears the
    /// dirty flag: snapshot-after-notify is the coalescing contract.
    pub fn snapshot(&self, id: SessionId) -> Result<Snapshot> {
        let inner = self.get(id)?;
        if let Some(flag) = self.dirty_flags.lock().unwrap().get(&id) {
            flag.store(false, Ordering::Release);
        }
        let engine = inner.engine.lock().unwrap();
        Ok(engine.snapshot())
    }

    /// Ask for a redraw notification without content change (e.g. after
    /// consumer-side state loss).
    pub fn mark_dirty(&self, id: SessionId) {
        if let Some(flag) = self.dirty_flags.lock().unwrap().get(&id) {
            if !flag.swap(true, Ordering::AcqRel) {
                let _ = self.dirty_tx.send(id);
            }
        }
    }

    pub fn is_alive(&self, id: SessionId) -> bool {
        self.sessions
            .lock()
            .unwrap()
            .get(&id)
            .map(|s| s.alive.load(Ordering::Acquire))
            .unwrap_or(false)
    }

    pub fn title(&self, id: SessionId) -> Option<String> {
        self.sessions
            .lock()
            .unwrap()
            .get(&id)
            .map(|s| s.title.lock().unwrap().clone())
    }

    pub fn list(&self) -> Vec<SessionId> {
        let mut ids: Vec<_> = self.sessions.lock().unwrap().keys().copied().collect();
        ids.sort();
        ids
    }

    /// Terminate the child process (SIGKILL-equivalent via the PTY child
    /// handle). The waiter thread emits `Exited`.
    pub fn kill(&self, id: SessionId) -> Result<()> {
        let inner = self.get(id)?;
        let result = inner.killer.lock().unwrap().kill();
        result.map_err(|e| CoreError::Pty(e.to_string()))
    }

    /// Kill (if alive) and remove the session. Dropping the master PTY
    /// unblocks the reader thread; dropping the engine sender ends the
    /// dispatcher thread.
    pub fn close(&self, id: SessionId) -> Result<()> {
        let inner = self.get(id)?;
        if inner.alive.load(Ordering::Acquire) {
            let _ = inner.killer.lock().unwrap().kill();
        }
        self.sessions.lock().unwrap().remove(&id);
        self.dirty_flags.lock().unwrap().remove(&id);
        let _ = self.events_tx.send(SessionEvent::Closed { id });
        Ok(())
    }

    /// Number of live session objects (for tests / diagnostics).
    pub fn len(&self) -> usize {
        self.sessions.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
