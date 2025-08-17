// Snippet
// File: src/util.rs

use owo_colors::OwoColorize;
use std::io::Write as _;
use std::process::{Command, Stdio};

pub fn maybe_clear(clear: bool) {
    if !clear {
        return;
    }
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd").args(["/C", "cls"]).status();
    }
    #[cfg(not(windows))]
    {
        let _ = std::process::Command::new("clear").status();
    }
}

pub fn copy_report(label: &str, s: &str) {
    let clean = strip_ansi_sgr(s);
    if copy_to_clipboard(&clean) {
        println!("{}", format!("[{label}] Copied output to clipboard.").green());
    } else {
        println!("{}", format!("[{label}] Clipboard copy not available.").yellow());
    }
}

/// Cross-platform clipboard copy using system tools.
fn copy_to_clipboard(text: &str) -> bool {
    #[cfg(target_os = "macos")]
    {
        return pipe_to("pbcopy", text);
    }
    #[cfg(target_os = "windows")]
    {
        return pipe_to("clip", text);
    }
    #[cfg(target_os = "linux")]
    {
        if pipe_to("wl-copy", text) {
            return true;
        }
        if Command::new("xclip").arg("-version").stdout(Stdio::null()).status().is_ok() {
            if let Ok(mut child) = Command::new("xclip")
                .args(["-selection", "clipboard"])
                .stdin(Stdio::piped())
                .spawn()
            {
                if let Some(stdin) = child.stdin.as_mut() {
                    let _ = stdin.write_all(text.as_bytes());
                }
                return child.wait().map(|s| s.success()).unwrap_or(false);
            }
        }
        return pipe_to_with_args("xsel", &["--clipboard", "--input"], text);
    }
    #[allow(unreachable_code)]
    false
}

fn pipe_to(cmd: &str, text: &str) -> bool {
    pipe_to_with_args(cmd, &[], text)
}

fn pipe_to_with_args(cmd: &str, args: &[&str], text: &str) -> bool {
    let mut child = match Command::new(cmd).args(args).stdin(Stdio::piped()).spawn() {
        Ok(c) => c,
        Err(_) => return false,
    };
    if let Some(stdin) = child.stdin.as_mut() {
        if stdin.write_all(text.as_bytes()).is_err() {
            return false;
        }
    }
    child.wait().map(|s| s.success()).unwrap_or(false)
}

/// Remove ANSI SGR escape sequences.
pub fn strip_ansi_sgr(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            i += 2;
            while i < bytes.len() {
                let b = bytes[i];
                if (b'@'..=b'~').contains(&b) {
                    i += 1;
                    break;
                }
                i += 1;
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}
