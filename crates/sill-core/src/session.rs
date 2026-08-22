//! Session lifecycle: PTY spawn, reader/waiter/dispatcher threads, typed
//! events.
//!
//! Threading model (deliberately boring):
//! - one *reader* thread per session: blocking PTY reads → engine feed →
//!   dirty notification (coalesced by the consumer)
//! - one *waiter* thread per session: blocks on child exit → `Exited` event
//! - one *dispatcher* thread per session: drains engine events, answers
//!   PTY write-backs (DSR/DA responses), forwards the rest as typed
//!   [`SessionEvent`]s. Holds only a **Weak** reference to the session so a
//!   closed session's engine (and its scrollback memory) can actually drop —
//!   the sender side of its channel lives inside the session, so a strong
//!   reference here would be a leak-by-reference-cycle.
//!
//! Backpressure & bounds: the reader parses synchronously into the engine,
//! so PTY intake is bounded by parse speed. Event channels are **bounded**:
//! spammable events (bell, title, wakeup) are dropped when full — missing
//! the 10,000th bell is correct behavior — while must-deliver events
//! (PTY write-backs, lifecycle) use blocking sends. No unbounded queues of
//! terminal-derived data exist (docs/design/performance.md).
//!
//! Close semantics: closing a session sends SIGHUP to the child's process
//! leader first (what real terminals do on window close) so job-control
//! shells can forward it to their jobs, then SIGKILL to the group, so
//! grandchildren holding the PTY slave open cannot pin the reader
//! thread and file descriptors forever. Processes that properly daemonize
//! (setsid/nohup with fd redirection) leave the group/tty and survive, by
//! design.

use std::collections::HashMap;
#[cfg(not(unix))]
use std::io::Read;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::sync::{mpsc, Arc, Mutex, Weak};
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

impl SessionEvent {
    /// Spammable events may be dropped under pressure; lifecycle events
    /// must always be delivered.
    fn droppable(&self) -> bool {
        matches!(
            self,
            SessionEvent::TitleChanged { .. } | SessionEvent::Bell { .. }
        )
    }
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

/// Capacity of the per-session engine-event channel. Small: only PtyWrite
/// is must-deliver (blocking send), everything else drops when full.
const ENGINE_EVENT_CAPACITY: usize = 256;
/// Capacity of the manager-level session-event channel.
const SESSION_EVENT_CAPACITY: usize = 1024;

struct SessionInner {
    id: SessionId,
    engine: Mutex<TermState>,
    writer: Mutex<Box<dyn Write + Send>>,
    master: Mutex<Box<dyn MasterPty + Send>>,
    killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
    /// Child pid == its process-group id (PTY spawn makes it session leader).
    child_pid: Option<u32>,
    alive: AtomicBool,
    /// Set by close(): reader exits its loop even if more output races in.
    closing: AtomicBool,
    title: Mutex<String>,
}

impl SessionInner {
    /// Graceful terminal-close semantics, matching what real terminals do:
    /// SIGHUP the session leader FIRST and give it a grace period — a
    /// job-control shell (bash/zsh) forwards SIGHUP to its jobs, which live
    /// in their **own process groups** that plain group-signaling would
    /// miss. After the grace period, SIGKILL the leader's group for
    /// anything stubborn. Runs on a detached reaper thread so close() never
    /// blocks the caller.
    fn terminate_tree_graceful(self: &Arc<Self>) {
        #[cfg(unix)]
        {
            if let Some(pid) = self.child_pid {
                let inner = self.clone();
                let _ = thread::Builder::new()
                    .name(format!("sill-reap-{}", self.id.0))
                    .spawn(move || {
                        unsafe {
                            libc::kill(pid as libc::pid_t, libc::SIGHUP);
                        }
                        thread::sleep(std::time::Duration::from_millis(150));
                        unsafe {
                            libc::killpg(pid as libc::pid_t, libc::SIGKILL);
                        }
                        let _ = inner.killer.lock().unwrap().kill();
                    });
                return;
            }
        }
        // Fallback / non-unix: kill the direct child via the PTY handle.
        // (On Windows, ConPTY teardown takes the console tree with it.)
        let _ = self.killer.lock().unwrap().kill();
    }

    /// Hard kill (user-initiated `kill`): SIGKILL the leader's group
    /// immediately, then the PTY child handle as fallback.
    fn terminate_tree_hard(&self) {
        #[cfg(unix)]
        if let Some(pid) = self.child_pid {
            unsafe {
                libc::killpg(pid as libc::pid_t, libc::SIGKILL);
            }
        }
        let _ = self.killer.lock().unwrap().kill();
    }
}

/// Owns all sessions. Internally locked per-session so one busy session
/// never blocks another's input.
pub struct SessionManager {
    sessions: Mutex<HashMap<SessionId, Arc<SessionInner>>>,
    next_id: AtomicU64,
    events_tx: SyncSender<SessionEvent>,
    /// Dirty notifications: session ids whose grid changed. Edge-triggered —
    /// at most one in-flight notification per session (the flag only re-arms
    /// after a snapshot), so an unbounded channel is bounded in practice by
    /// the session count.
    dirty_tx: mpsc::Sender<SessionId>,
    dirty_flags: Mutex<HashMap<SessionId, Arc<AtomicBool>>>,
}

/// Send with drop-vs-must-deliver policy.
fn send_session_event(tx: &SyncSender<SessionEvent>, ev: SessionEvent) {
    if ev.droppable() {
        match tx.try_send(ev) {
            Ok(()) | Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => {}
        }
    } else {
        // Lifecycle events are rare; blocking briefly on a full queue is
        // acceptable and guarantees delivery while a receiver exists.
        let _ = tx.send(ev);
    }
}

impl SessionManager {
    /// Create a manager plus its event/dirty receivers.
    pub fn new() -> (Arc<Self>, Receiver<SessionEvent>, Receiver<SessionId>) {
        let (events_tx, events_rx) = mpsc::sync_channel(SESSION_EVENT_CAPACITY);
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

        let child_pid = child.process_id();
        let killer = child.clone_killer();
        // Unix reads the master fd directly with poll() timeouts so the
        // reader thread is interruptible; other platforms fall back to the
        // blocking cloned reader.
        #[cfg(unix)]
        let master_fd = pair.master.as_raw_fd();
        #[cfg(not(unix))]
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| CoreError::Pty(e.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| CoreError::Pty(e.to_string()))?;

        // Engine + its bounded event channel (drained by the dispatcher).
        let (engine_tx, engine_rx) = mpsc::sync_channel::<EngineEvent>(ENGINE_EVENT_CAPACITY);
        let engine = TermState::new(TermDims { cols, rows }, scrollback, engine_tx);

        let inner = Arc::new(SessionInner {
            id,
            engine: Mutex::new(engine),
            writer: Mutex::new(writer),
            master: Mutex::new(pair.master),
            killer: Mutex::new(killer),
            child_pid,
            alive: AtomicBool::new(true),
            closing: AtomicBool::new(false),
            title: Mutex::new(String::new()),
        });

        let dirty = Arc::new(AtomicBool::new(false));
        self.dirty_flags.lock().unwrap().insert(id, dirty.clone());
        self.sessions.lock().unwrap().insert(id, inner.clone());

        // Reader thread: PTY → engine → dirty edge notification.
        //
        // The reader must be INTERRUPTIBLE: a descendant that keeps the PTY
        // slave open (nohup-style, or a SIGHUP-trapping job in its own
        // process group) would otherwise pin a blocking read — and with it
        // this thread, the session's memory, and the master fd — forever.
        // Unix polls the master fd with a timeout and re-checks `closing`;
        // like a real terminal, Sill frees its side of the PTY on close and
        // lets the kernel deal with slave-holding survivors.
        {
            let inner = inner.clone();
            let dirty = dirty.clone();
            let dirty_tx = self.dirty_tx.clone();
            #[cfg(not(unix))]
            let mut reader = reader;
            thread::Builder::new()
                .name(format!("sill-pty-read-{}", id.0))
                .spawn(move || {
                    let mut buf = [0u8; 32 * 1024];
                    #[cfg(unix)]
                    {
                        let Some(fd) = master_fd else {
                            return;
                        };
                        loop {
                            if inner.closing.load(Ordering::Acquire) {
                                break;
                            }
                            let mut pfd = libc::pollfd {
                                fd,
                                events: libc::POLLIN,
                                revents: 0,
                            };
                            let ready = unsafe { libc::poll(&mut pfd, 1, 100) };
                            if ready < 0 {
                                let err = std::io::Error::last_os_error();
                                if err.kind() == std::io::ErrorKind::Interrupted {
                                    continue;
                                }
                                break;
                            }
                            if ready == 0 {
                                continue; // timeout: re-check closing
                            }
                            let n = unsafe {
                                libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len())
                            };
                            match n {
                                0 => break, // EOF: all slaves closed
                                n if n < 0 => {
                                    let err = std::io::Error::last_os_error();
                                    if err.kind() == std::io::ErrorKind::Interrupted {
                                        continue;
                                    }
                                    break; // EIO on last-slave close, etc.
                                }
                                n => {
                                    if inner.closing.load(Ordering::Acquire) {
                                        break;
                                    }
                                    {
                                        let mut engine = inner.engine.lock().unwrap();
                                        engine.feed(&buf[..n as usize]);
                                    }
                                    if !dirty.swap(true, Ordering::AcqRel) {
                                        let _ = dirty_tx.send(inner.id);
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(not(unix))]
                    loop {
                        if inner.closing.load(Ordering::Acquire) {
                            break;
                        }
                        match reader.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                if inner.closing.load(Ordering::Acquire) {
                                    break;
                                }
                                {
                                    let mut engine = inner.engine.lock().unwrap();
                                    engine.feed(&buf[..n]);
                                }
                                if !dirty.swap(true, Ordering::AcqRel) {
                                    let _ = dirty_tx.send(inner.id);
                                }
                            }
                            Err(_) => break,
                        }
                    }
                })
                .expect("spawn pty reader thread");
        }

        // Dispatcher thread: engine events for THIS session. Holds only a
        // Weak reference — the engine (which owns the sender) must be able
        // to drop when the session closes, which then ends this thread via
        // channel disconnect.
        {
            let weak: Weak<SessionInner> = Arc::downgrade(&inner);
            let events_tx = self.events_tx.clone();
            let dirty = dirty.clone();
            let dirty_tx = self.dirty_tx.clone();
            let session_id = id;
            thread::Builder::new()
                .name(format!("sill-events-{}", id.0))
                .spawn(move || {
                    while let Ok(ev) = engine_rx.recv() {
                        let Some(inner) = weak.upgrade() else {
                            // Session dropped: drain silently until the
                            // sender disconnects.
                            continue;
                        };
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
                                send_session_event(
                                    &events_tx,
                                    SessionEvent::TitleChanged {
                                        id: session_id,
                                        title,
                                    },
                                );
                            }
                            EngineEvent::ResetTitle => {
                                inner.title.lock().unwrap().clear();
                                send_session_event(
                                    &events_tx,
                                    SessionEvent::TitleChanged {
                                        id: session_id,
                                        title: String::new(),
                                    },
                                );
                            }
                            EngineEvent::Bell => {
                                send_session_event(
                                    &events_tx,
                                    SessionEvent::Bell { id: session_id },
                                );
                            }
                            EngineEvent::Wakeup => {
                                if !dirty.swap(true, Ordering::AcqRel) {
                                    let _ = dirty_tx.send(session_id);
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
                    send_session_event(
                        &events_tx,
                        SessionEvent::Exited {
                            id: inner.id,
                            exit_code,
                        },
                    );
                })
                .expect("spawn child waiter thread");
        }

        send_session_event(&self.events_tx, SessionEvent::Created { id });
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

    /// Paste text into the session. The wrapping decision is made HERE,
    /// against the engine's current mode — the frontend never caches
    /// protocol state (a stale cache can strip bracketed-paste delimiters
    /// right after a program enables the mode).
    pub fn paste(&self, id: SessionId, text: &str) -> Result<()> {
        let inner = self.get(id)?;
        let normalized = normalize_paste(text);
        let bracketed = inner.engine.lock().unwrap().bracketed_paste();
        let payload = if bracketed {
            format!("\x1b[200~{normalized}\x1b[201~")
        } else {
            normalized
        };
        let mut writer = inner.writer.lock().unwrap();
        writer
            .write_all(payload.as_bytes())
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

    /// Terminate the child's process tree (group SIGHUP + SIGKILL). The
    /// waiter thread emits `Exited`.
    pub fn kill(&self, id: SessionId) -> Result<()> {
        let inner = self.get(id)?;
        inner.terminate_tree_hard();
        Ok(())
    }

    /// Kill the process tree (if alive) and remove the session. Group
    /// signaling closes every PTY-slave holder, which EOFs the reader
    /// thread; dropping the session drops the engine, whose sender
    /// disconnect ends the dispatcher thread.
    pub fn close(&self, id: SessionId) -> Result<()> {
        let inner = self.get(id)?;
        inner.closing.store(true, Ordering::Release);
        if inner.alive.load(Ordering::Acquire) {
            inner.terminate_tree_graceful();
        }
        self.sessions.lock().unwrap().remove(&id);
        self.dirty_flags.lock().unwrap().remove(&id);
        send_session_event(&self.events_tx, SessionEvent::Closed { id });
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

/// Normalize pasted text for PTY input: terminals receive carriage returns,
/// not newlines.
pub fn normalize_paste(text: &str) -> String {
    text.replace("\r\n", "\r").replace('\n', "\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paste_normalization_converts_newlines() {
        assert_eq!(normalize_paste("a\r\nb\nc"), "a\rb\rc");
        assert_eq!(normalize_paste("plain"), "plain");
    }

    #[test]
    fn droppable_classification() {
        assert!(SessionEvent::Bell { id: SessionId(1) }.droppable());
        assert!(SessionEvent::TitleChanged {
            id: SessionId(1),
            title: String::new()
        }
        .droppable());
        assert!(!SessionEvent::Exited {
            id: SessionId(1),
            exit_code: None
        }
        .droppable());
        assert!(!SessionEvent::Closed { id: SessionId(1) }.droppable());
    }
}
