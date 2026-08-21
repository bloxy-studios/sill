//! Integration tests against real PTYs and real shells (unix).
//! These exercise the full path: spawn → PTY → reader thread → engine →
//! snapshot, plus lifecycle events and the churn/leak contract.

#![cfg(unix)]

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::{Duration, Instant};

use sill_core::{SessionEvent, SessionId, SessionManager, SessionOptions};

const SH: &str = "/bin/sh";

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
    let (mgr, _events, _dirty) = SessionManager::new();
    let bogus = SessionId(999_999);
    assert!(matches!(
        mgr.input(bogus, b"x"),
        Err(sill_core::CoreError::UnknownSession(_))
    ));
    assert!(mgr.snapshot(bogus).is_err());
}
