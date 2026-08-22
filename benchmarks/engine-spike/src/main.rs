//! ADR-0006 engine spike: feed identical synthetic corpora through candidate
//! terminal emulation engines; report throughput and peak RSS.
//!
//! Run one (engine, corpus) pair per process so RSS is clean:
//!   cargo run --release -- <alacritty|vt100> <plain|ansi|unicode|cursor> <MB>
//!
//! Corpora are generated deterministically in-memory — no I/O noise.

use std::time::Instant;

const COLS: u16 = 120;
const ROWS: u16 = 40;
const SCROLLBACK: usize = 10_000;

fn build_corpus(kind: &str, target_mb: usize) -> Vec<u8> {
    let target = target_mb * 1024 * 1024;
    let mut out = Vec::with_capacity(target + 4096);
    let mut i = 0usize;
    while out.len() < target {
        match kind {
            // Build-log style plain lines.
            "plain" => {
                out.extend_from_slice(
                    format!("[{i:08}] compiling module core::alloc::layout — ok in 12ms\r\n")
                        .as_bytes(),
                );
            }
            // SGR-heavy: colored ls/test-runner style output.
            "ansi" => {
                out.extend_from_slice(
                    format!(
                        "\x1b[32mPASS\x1b[0m \x1b[1m\x1b[34mtest_{i:06}\x1b[0m \
                         \x1b[33m12ms\x1b[0m \x1b[38;5;245mcrates/core/src/lib.rs\x1b[0m \
                         \x1b[38;2;120;180;240mrgb\x1b[0m\r\n"
                    )
                    .as_bytes(),
                );
            }
            // Wide chars, emoji, combining marks.
            "unicode" => {
                out.extend_from_slice(
                    format!("[{i:06}] 构建成功 ✅ émojis🚀 とても速い ﷽ e\u{301}\r\n").as_bytes(),
                );
            }
            // Cursor-movement heavy: full-screen repaint pattern (vim-ish).
            "cursor" => {
                let row = (i % ROWS as usize) + 1;
                out.extend_from_slice(
                    format!("\x1b[{row};1H\x1b[2Kline {i:06} redrawn in place").as_bytes(),
                );
                if i % ROWS as usize == 0 {
                    out.extend_from_slice(b"\x1b[H");
                }
            }
            other => panic!("unknown corpus: {other}"),
        }
        i += 1;
    }
    out
}

fn peak_rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
}

mod alacritty_engine {
    use alacritty_terminal::event::{Event, EventListener};
    use alacritty_terminal::grid::Dimensions;
    use alacritty_terminal::term::{Config, Term};
    use alacritty_terminal::vte::ansi::Processor;

    struct Sink;
    impl EventListener for Sink {
        fn send_event(&self, _event: Event) {}
    }

    struct Dims;
    impl Dimensions for Dims {
        fn total_lines(&self) -> usize {
            super::ROWS as usize
        }
        fn screen_lines(&self) -> usize {
            super::ROWS as usize
        }
        fn columns(&self) -> usize {
            super::COLS as usize
        }
    }

    pub fn run(corpus: &[u8]) -> usize {
        let config = Config {
            scrolling_history: super::SCROLLBACK,
            ..Config::default()
        };
        let mut term = Term::new(config, &Dims, Sink);
        let mut parser: Processor = Processor::new();
        for chunk in corpus.chunks(32 * 1024) {
            parser.advance(&mut term, chunk);
        }
        // Prevent the work from being optimized away.
        term.grid().total_lines()
    }
}

mod vt100_engine {
    pub fn run(corpus: &[u8]) -> usize {
        let mut parser = vt100::Parser::new(super::ROWS, super::COLS, super::SCROLLBACK);
        for chunk in corpus.chunks(32 * 1024) {
            parser.process(chunk);
        }
        parser.screen().cursor_position().0 as usize
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let engine = args.next().expect("engine: alacritty|vt100");
    let corpus_kind = args.next().expect("corpus: plain|ansi|unicode|cursor");
    let mb: usize = args
        .next()
        .expect("size in MB")
        .parse()
        .expect("MB must be a number");

    let corpus = build_corpus(&corpus_kind, mb);
    let bytes = corpus.len();

    let start = Instant::now();
    let sink = match engine.as_str() {
        "alacritty" => alacritty_engine::run(&corpus),
        "vt100" => vt100_engine::run(&corpus),
        other => panic!("unknown engine: {other}"),
    };
    let elapsed = start.elapsed();

    let mbps = (bytes as f64 / (1024.0 * 1024.0)) / elapsed.as_secs_f64();
    let rss = peak_rss_kb().unwrap_or(0);
    println!(
        "engine={engine} corpus={corpus_kind} bytes={bytes} secs={:.3} MBps={mbps:.1} peak_rss_kb={rss} sink={sink}",
        elapsed.as_secs_f64()
    );
}
