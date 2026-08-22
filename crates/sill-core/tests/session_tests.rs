//! Integration tests against real PTYs and real shells (unix).
//! These exercise the full path: spawn → PTY → reader thread → engine →
//! snapshot, plus lifecycle events and the churn/leak contract.

#![cfg(unix)]

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::{Duration, Instant};

use sill_core::{SessionEvent, SessionId, SessionManager, SessionOptions};

const SH: &str = "/bin/sh";

/// Serialize all tests in this binary: they spawn real processes/threads,
/// and the thread-count assertions below are only meaningful without
/// concurrent sibling tests.
fn serial_guard() -> std::sync::MutexGuard<'static, ()> {
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

fn opts(cols: u16, rows: u16) -> SessionOptions {
    SessionOptions {
        cols,
        rows,
        shell: Some(SH.to_string()),
        cwd: None,
        scrollback_lines: Some(1000),
    }
}

fn visible_text(mgr: &Arc<SessionManager>, id: SessionId) -> String {
    let snap = mgr.snapshot(id).expect("snapshot");
    snap.rows
        .iter()
        .map(|r| {
            r.runs
                .iter()
                .map(|run| run.text.as_str())
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Poll the visible grid until `pred` matches or the timeout elapses.
fn wait_for_screen(
    mgr: &Arc<SessionManager>,
    id: SessionId,
    timeout: Duration,
    pred: impl Fn(&str) -> bool,
) -> String {
    let start = Instant::now();
    loop {
        let text = visible_text(mgr, id);
        if pred(&text) {
            return text;
        }
        if start.elapsed() > timeout {
            panic!("timeout waiting for screen condition; last screen:\n{text}");
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn wait_for_event(
    rx: &Receiver<SessionEvent>,
    timeout: Duration,
    pred: impl Fn(&SessionEvent) -> bool,
) -> SessionEvent {
    let deadline = Instant::now() + timeout;
    loop {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .unwrap_or_default();
        match rx.recv_timeout(remaining.max(Duration::from_millis(1))) {
            Ok(ev) if pred(&ev) => return ev,
            Ok(_) => continue,
            Err(_) => panic!("timeout waiting for event"),
        }
    }
}

#[test]
fn spawn_echo_roundtrip() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    mgr.input(id, b"printf 'sill-says-%s\\n' hello\n").unwrap();
    let screen = wait_for_screen(&mgr, id, Duration::from_secs(10), |s| {
        s.contains("sill-says-hello")
    });
    assert!(screen.contains("sill-says-hello"));

    mgr.close(id).unwrap();
}

#[test]
fn resize_propagates_to_child() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    mgr.resize(id, 120, 40).unwrap();
    // `stty size` prints "rows cols" as seen by the child through the PTY.
    mgr.input(id, b"stty size\n").unwrap();
    wait_for_screen(&mgr, id, Duration::from_secs(10), |s| s.contains("40 120"));

    mgr.close(id).unwrap();
}

#[test]
fn exit_event_carries_status() {
    let _serial = serial_guard();
    let (mgr, events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    mgr.input(id, b"exit 7\n").unwrap();
    let ev = wait_for_event(
        &events,
        Duration::from_secs(10),
        |e| matches!(e, SessionEvent::Exited { id: eid, .. } if *eid == id),
    );
    match ev {
        SessionEvent::Exited { exit_code, .. } => assert_eq!(exit_code, Some(7)),
        _ => unreachable!(),
    }
    assert!(!mgr.is_alive(id));
    mgr.close(id).unwrap();
}

#[test]
fn kill_terminates_session() {
    let _serial = serial_guard();
    let (mgr, events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    mgr.kill(id).unwrap();
    wait_for_event(
        &events,
        Duration::from_secs(10),
        |e| matches!(e, SessionEvent::Exited { id: eid, .. } if *eid == id),
    );
    mgr.close(id).unwrap();
    assert!(mgr.is_empty());
}

#[test]
fn dirty_notifications_fire_on_output() {
    let _serial = serial_guard();
    let (mgr, _events, dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    // Shell startup output already fires one; drain, then trigger another.
    while dirty.try_recv().is_ok() {}
    let _ = mgr.snapshot(id); // clear flag so the next output re-notifies
    mgr.input(id, b"printf 'ping\\n'\n").unwrap();

    let got = dirty.recv_timeout(Duration::from_secs(10)).expect("dirty");
    assert_eq!(got, id);

    mgr.close(id).unwrap();
}

#[test]
fn scrollback_and_display_offset() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 10)).expect("create session");

    mgr.input(
        id,
        b"i=0; while [ $i -lt 100 ]; do echo line-$i; i=$((i+1)); done\n",
    )
    .unwrap();
    wait_for_screen(&mgr, id, Duration::from_secs(10), |s| s.contains("line-99"));

    let snap = mgr.snapshot(id).unwrap();
    assert!(snap.history > 0, "expected scrollback history");

    mgr.scroll(id, 5).unwrap();
    let scrolled = mgr.snapshot(id).unwrap();
    assert_eq!(scrolled.display_offset, 5);
    assert!(
        !scrolled.cursor.visible,
        "cursor hidden while scrolled back"
    );

    mgr.scroll_to_bottom(id).unwrap();
    let bottom = mgr.snapshot(id).unwrap();
    assert_eq!(bottom.display_offset, 0);

    mgr.close(id).unwrap();
}

#[test]
fn title_osc_reaches_events() {
    let _serial = serial_guard();
    let (mgr, events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    mgr.input(id, b"printf '\\033]0;sill-test-title\\007'\n")
        .unwrap();
    let ev = wait_for_event(
        &events,
        Duration::from_secs(10),
        |e| matches!(e, SessionEvent::TitleChanged { id: eid, title } if *eid == id && title == "sill-test-title"),
    );
    drop(ev);
    assert_eq!(mgr.title(id).as_deref(), Some("sill-test-title"));

    mgr.close(id).unwrap();
}

/// Churn contract: repeated create/close must not accumulate sessions.
/// (An RSS-based leak harness lands with benchmarks; this guards the
/// object-lifetime half.)
#[test]
fn create_close_churn_leaves_no_sessions() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    for _ in 0..25 {
        let id = mgr.create(opts(60, 16)).expect("create session");
        mgr.input(id, b"printf x\n").unwrap();
        mgr.close(id).unwrap();
    }
    assert!(mgr.is_empty());
}

#[test]
fn unknown_session_errors_are_typed() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let bogus = SessionId(999_999);
    assert!(matches!(
        mgr.input(bogus, b"x"),
        Err(sill_core::CoreError::UnknownSession(_))
    ));
    assert!(mgr.snapshot(bogus).is_err());
}

/// Count live sill worker threads by name (Linux): every session worker is
/// named `sill-*` (pty-read/events/wait/reap). Counting only our own named
/// threads keeps the assertion immune to test-harness threads — libtest
/// runs sibling tests on threads that queue on the serial guard and would
/// pollute a process-wide count.
#[cfg(target_os = "linux")]
fn sill_thread_names() -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(tasks) = std::fs::read_dir("/proc/self/task") {
        for t in tasks.flatten() {
            if let Ok(comm) = std::fs::read_to_string(t.path().join("comm")) {
                let comm = comm.trim();
                if comm.starts_with("sill") {
                    names.push(comm.to_string());
                }
            }
        }
    }
    names
}

#[cfg(target_os = "linux")]
fn wait_sill_threads_zero(timeout: Duration) -> Vec<String> {
    let start = Instant::now();
    loop {
        let names = sill_thread_names();
        if names.is_empty() {
            return names;
        }
        if start.elapsed() > timeout {
            return names;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// The leak Greptile caught: a dispatcher holding a strong Arc to its own
/// session pins the engine (and its channel sender) forever. All three
/// worker threads must exit after close, returning the process to its
/// thread baseline.
#[cfg(target_os = "linux")]
#[test]
fn close_releases_all_worker_threads() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();

    for _ in 0..10 {
        let id = mgr.create(opts(60, 16)).expect("create session");
        mgr.input(id, b"printf x\n").unwrap();
        mgr.close(id).unwrap();
    }

    let leftover = wait_sill_threads_zero(Duration::from_secs(15));
    assert!(
        leftover.is_empty(),
        "worker threads leaked after churn: {leftover:?}"
    );
    assert!(mgr.is_empty());
}

/// A grandchild holding the PTY slave open must not survive close() and pin
/// the reader thread: close sends SIGHUP+SIGKILL to the process GROUP.
#[cfg(target_os = "linux")]
#[test]
fn close_terminates_grandchildren_holding_the_pty() {
    let _serial = serial_guard();
    let (mgr, events, _dirty) = SessionManager::new();

    let id = mgr.create(opts(80, 24)).expect("create session");
    // Background grandchild inheriting the slave fds, then prove it's up.
    mgr.input(id, b"sleep 300 &\nprintf 'grandchild-up-%s\\n' yes\n")
        .unwrap();
    wait_for_screen(&mgr, id, Duration::from_secs(10), |s| {
        s.contains("grandchild-up-yes")
    });

    mgr.close(id).unwrap();
    wait_for_event(
        &events,
        Duration::from_secs(10),
        |e| matches!(e, SessionEvent::Exited { id: eid, .. } if *eid == id),
    );

    let leftover = wait_sill_threads_zero(Duration::from_secs(15));
    assert!(
        leftover.is_empty(),
        "grandchild pinned session workers: {leftover:?}"
    );
}

/// Paste wrapping is decided in Rust against live engine mode — no stale
/// frontend cache. Interactive bash enables mode 2004 itself at every
/// prompt (readline would consume the delimiters), so we paste into `cat`
/// (no readline): the tty's ECHOCTL then renders the wrapped ESC as ^[ on
/// screen, which is what we assert.
#[test]
fn paste_wraps_when_bracketed_mode_is_active() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(100, 24)).expect("create session");

    // Interactive bash toggles 2004 off while a command runs and back on at
    // each prompt (the engine tracks this faithfully), so the FOREGROUND
    // program must enable the mode itself for it to be active during the
    // paste. `cat -v` then proves what the child actually RECEIVED, with
    // control bytes rendered printably (^[) — independent of tty echo
    // settings (with ECHOCTL off, echoed raw delimiters round-trip through
    // our own parser and are consumed as unknown CSI).
    mgr.input(id, b"printf '\\033[?2004h'; cat -v\n").unwrap();
    let start = Instant::now();
    while !mgr.snapshot(id).unwrap().bracketed_paste {
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "bracketed paste mode never became active"
        );
        std::thread::sleep(Duration::from_millis(25));
    }
    std::thread::sleep(Duration::from_millis(200));

    mgr.paste(id, "pasted-bit\n").unwrap();
    // The close-delimiter has no trailing newline; flush cat's canonical
    // input buffer so it prints.
    mgr.input(id, b"\n").unwrap();
    wait_for_screen(&mgr, id, Duration::from_secs(10), |s| {
        s.contains("200~pasted-bit") && s.contains("201~")
    });

    mgr.close(id).unwrap();
}

/// With mode 2004 explicitly disabled (and no readline prompt to re-enable
/// it), paste must arrive unwrapped.
#[test]
fn paste_stays_raw_without_bracketed_mode() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(100, 24)).expect("create session");

    mgr.input(id, b"printf '\\033[?2004l'\n").unwrap();
    let start = Instant::now();
    while mgr.snapshot(id).unwrap().bracketed_paste {
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "bracketed paste mode never deactivated"
        );
        std::thread::sleep(Duration::from_millis(25));
    }

    mgr.input(id, b"cat > /dev/null\n").unwrap();
    std::thread::sleep(Duration::from_millis(300));

    mgr.paste(id, "raw-bit\n").unwrap();
    let screen = wait_for_screen(&mgr, id, Duration::from_secs(10), |s| s.contains("raw-bit"));
    assert!(
        !screen.contains("200~"),
        "unexpected bracketed delimiters:\n{screen}"
    );

    mgr.close(id).unwrap();
}

/// A child that floods terminal queries (ESC[6n) without EVER reading the
/// replies must not freeze the session. The old deadlock chain: PTY buffer
/// fills -> dispatcher blocks writing replies -> PtyWrite queue fills ->
/// reader blocks sending WHILE HOLDING THE ENGINE LOCK -> snapshot()/close()
/// hang forever. Replies now drop under flood; the engine lock stays live.
#[test]
fn query_flood_without_reader_stays_responsive() {
    let _serial = serial_guard();
    let (mgr, _events, _dirty) = SessionManager::new();
    let id = mgr.create(opts(80, 24)).expect("create session");

    // Replace the shell with a pure flooder that never reads stdin.
    mgr.input(id, b"exec sh -c 'while :; do printf \"\\033[6n\"; done'\n")
        .unwrap();
    std::thread::sleep(Duration::from_millis(500));

    // Liveness probe: the snapshot path takes the engine lock — exactly
    // what the old deadlock parked forever. Run it off-thread so a
    // regression fails the assertion instead of hanging the harness.
    let probe_mgr = mgr.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for _ in 0..20 {
            let _ = probe_mgr.snapshot(id);
            std::thread::sleep(Duration::from_millis(25));
        }
        let _ = tx.send(());
    });
    rx.recv_timeout(Duration::from_secs(10))
        .expect("snapshot path deadlocked under query flood");

    mgr.close(id).unwrap();
    #[cfg(target_os = "linux")]
    {
        let leftover = wait_sill_threads_zero(Duration::from_secs(15));
        assert!(leftover.is_empty(), "workers pinned by flood: {leftover:?}");
    }
}
