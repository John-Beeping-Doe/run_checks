// Package run_checks
// File: src/main.rs

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::io::Write as IoWrite;
use std::process::{Command, Stdio};

mod defaults;
mod display_all;
mod run_checks;
mod tree;

/// CLI for one-shot checks and project introspection.
#[derive(Parser)]
#[command(
    name = "run_checks",
    version,
    about = "Run checks and helpers, then exit.",
    after_help = "\
Examples:
  cargo run -- checks
      Run rustfmt, clippy, cargo check, and cargo test. Print a summary table.
      The Security/Privacy table skips Extra scans.

  cargo run -- checks-extras
      Same as 'checks' but runs the Extra scans row.

  cargo run -- create-defaults
      Create .gitignore, rustfmt.toml, run_checks.sh, LICENSE if missing.

  cargo run -- all --depth 3 --clear
      Run checks (skip extras), then display all source files and a directory tree
      up to depth 3, clearing the screen before each section.

  ./run_checks checks-extras
      Use the compiled binary and include Extra scans.

  ./run_checks all --depth 3 --clear
      Run checks (skip extras), show file contents and a directory tree using the installed binary."
)]
struct Cli {
    /// Optional global clear before printing each subcommand output
    #[arg(long)]
    clear: bool,

    #[command(subcommand)]
    cmd: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Run rustfmt, clippy, check, test. Prints summary tables. Extra scans are skipped.
    Checks,
    /// Same as `checks` but runs the Extra scans row.
    #[command(name = "checks-extras")]
    ChecksExtras,
    /// Create default project files if absent (.gitignore, rustfmt.toml, run_checks.sh, LICENSE).
    #[command(name = "create-defaults")]
    CreateDefaults,
    /// Print a directory tree. Default depth=2.
    Tree {
        #[arg(long, default_value_t = 2)]
        depth: usize,
    },
    /// Print all .rs files under src and copy to clipboard.
    Files,
    /// Run `checks` (skip extras), then `files`, then `tree`.
    All {
        #[arg(long, default_value_t = 2)]
        depth: usize,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut exit_code = 0usize;

    match cli.cmd {
        CommandKind::Checks => {
            maybe_clear(cli.clear);
            let ok = run_checks::run_checks(false).await;
            if !ok {
                eprintln!("{}", "Some checks failed.".red());
                exit_code = 1;
            }
        }
        CommandKind::ChecksExtras => {
            maybe_clear(cli.clear);
            let ok = run_checks::run_checks(true).await;
            if !ok {
                eprintln!("{}", "Some checks failed.".red());
                exit_code = 1;
            }
        }
        CommandKind::CreateDefaults => {
            maybe_clear(cli.clear);
            defaults::ensure_defaults();
        }
        CommandKind::Tree { depth } => {
            maybe_clear(cli.clear);
            tree::display_tree(depth);
        }
        CommandKind::Files => {
            maybe_clear(cli.clear);
            let blob = display_all::collect_all_rs();
            print!("{blob}");
            let blob_clean = strip_ansi_sgr(&blob);
            if copy_to_clipboard(&blob_clean) {
                println!("{}", "[files] Copied output to clipboard.".green());
            } else {
                println!("{}", "[files] Clipboard copy not available.".yellow());
            }
        }
        CommandKind::All { depth } => {
            maybe_clear(cli.clear);
            let ok = run_checks::run_checks(false).await;
            if !ok {
                eprintln!("{}", "[all] Checks failed, continuing with files/tree.".yellow());
                exit_code = 1;
            }
            let blob = display_all::collect_all_rs();
            print!("{blob}");
            let blob_clean = strip_ansi_sgr(&blob);
            if copy_to_clipboard(&blob_clean) {
                println!("{}", "[all] Copied files output to clipboard.".green());
            } else {
                println!("{}", "[all] Clipboard copy not available.".yellow());
            }
            tree::display_tree(depth);
        }
    }

    std::process::exit(exit_code as i32);
}

fn maybe_clear(clear: bool) {
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

/// Cross-platform clipboard copy using system tools.
/// macOS: pbcopy
/// Windows: clip
/// Linux: wl-copy, else xclip, else xsel.
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
fn strip_ansi_sgr(s: &str) -> String {
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
