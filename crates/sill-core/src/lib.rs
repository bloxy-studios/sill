//! Sill's terminal core: PTY management, terminal emulation state, and
//! session lifecycle. Deliberately webview-free — this crate builds and
//! tests headless (docs/ARCHITECTURE.md: "Rust owns reality").
//!
//! Layering:
//! - [`shell`]: default-shell resolution (zsh-first on macOS, honest fallbacks)
//! - [`engine`]: emulation state over `alacritty_terminal` (ADR-0006)
//! - [`snapshot`]: render-ready grid DTOs crossing the IPC boundary
//! - [`session`]: PTY spawn/lifecycle, reader threads, typed events

pub mod engine;
pub mod session;
pub mod shell;
pub mod snapshot;

use std::fmt;

pub use session::{SessionEvent, SessionId, SessionManager, SessionOptions};
pub use snapshot::Snapshot;

/// Errors surfaced by the terminal core. Typed and contextual per
/// docs/PR_REVIEW.md's error-handling expectations.
#[derive(Debug)]
pub enum CoreError {
    /// PTY layer failure (open, spawn, resize, io).
    Pty(String),
    /// The referenced session does not exist (or was closed).
    UnknownSession(SessionId),
    /// Writing to the session's input failed.
    Input(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::Pty(msg) => write!(f, "pty error: {msg}"),
            CoreError::UnknownSession(id) => write!(f, "unknown session: {id:?}"),
            CoreError::Input(msg) => write!(f, "input error: {msg}"),
        }
    }
}

impl std::error::Error for CoreError {}

pub type Result<T> = std::result::Result<T, CoreError>;
