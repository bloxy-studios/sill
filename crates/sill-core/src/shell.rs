//! Default-shell resolution.
//!
//! Policy (prompt: "zsh default, never hardcoded, graceful fallback"):
//! 1. An explicit override always wins (settings surface, later).
//! 2. `$SHELL` if set and the binary exists — the user's actual choice.
//! 3. Platform default: macOS → zsh (the OS default since Catalina),
//!    other unix → bash, then sh; Windows → `%COMSPEC%` or cmd.exe.
//!
//! Nothing here assumes zsh exists everywhere.

use std::path::Path;

/// Resolve the shell command to spawn.
pub fn resolve_shell(override_shell: Option<&str>) -> String {
    if let Some(sh) = override_shell {
        if !sh.trim().is_empty() {
            return sh.to_string();
        }
    }

    if let Ok(sh) = std::env::var("SHELL") {
        if !sh.trim().is_empty() && Path::new(&sh).exists() {
            return sh;
        }
    }

    platform_default()
}

#[cfg(target_os = "macos")]
fn platform_default() -> String {
    for candidate in ["/bin/zsh", "/bin/bash", "/bin/sh"] {
        if Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }
    "/bin/sh".to_string()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_default() -> String {
    for candidate in [
        "/usr/bin/zsh",
        "/bin/zsh",
        "/usr/bin/bash",
        "/bin/bash",
        "/bin/sh",
    ] {
        if Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }
    "/bin/sh".to_string()
}

#[cfg(windows)]
fn platform_default() -> String {
    std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_wins() {
        assert_eq!(resolve_shell(Some("/opt/fancy/fish")), "/opt/fancy/fish");
    }

    #[test]
    fn blank_override_is_ignored() {
        let resolved = resolve_shell(Some("  "));
        assert_ne!(resolved.trim(), "");
        assert_ne!(resolved, "  ");
    }

    #[cfg(unix)]
    #[test]
    fn resolves_to_an_existing_binary_without_override() {
        // Regardless of $SHELL in the environment, the result must exist
        // or be the final /bin/sh fallback.
        let resolved = resolve_shell(None);
        assert!(
            Path::new(&resolved).exists() || resolved == "/bin/sh",
            "resolved shell should exist: {resolved}"
        );
    }
}
