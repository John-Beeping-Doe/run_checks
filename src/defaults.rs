// Package run_checks
// File: src/defaults.rs

use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, Table};
use owo_colors::OwoColorize;
use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Ensure default files exist; create them if missing.
/// Files: .gitignore, rustfmt.toml, run_checks.sh, LICENSE
pub fn ensure_defaults() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL).set_header(vec!["File", "Action", "Details"]);

    for (path, contents, post) in [
        (".gitignore", DEFAULT_GITIGNORE, None::<fn()>),
        ("rustfmt.toml", DEFAULT_RUSTFMT, None::<fn()>),
        ("run_checks.sh", DEFAULT_SCRIPT, Some(make_script_executable as fn())),
        ("LICENSE", DEFAULT_LICENSE, None::<fn()>),
    ] {
        let p = Path::new(path);
        if p.exists() {
            table.add_row(vec![
                Cell::new(path),
                Cell::new("exists").add_attribute(Attribute::Bold).fg(Color::Green),
                Cell::new("no changes"),
            ]);
            continue;
        }

        match write_new_file(p, contents) {
            Ok(_) => {
                if let Some(f) = post {
                    f();
                }
                table.add_row(vec![
                    Cell::new(path),
                    Cell::new("created").add_attribute(Attribute::Bold).fg(Color::Yellow),
                    Cell::new("default template written"),
                ]);
            }
            Err(e) => {
                table.add_row(vec![
                    Cell::new(path),
                    Cell::new("error").add_attribute(Attribute::Bold).fg(Color::Red),
                    Cell::new(e.to_string()),
                ]);
            }
        }
    }

    println!("{}", "Create Defaults? results:".cyan());
    println!("{table}");
}

fn write_new_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut f = fs::File::create(path)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(unix)]
fn make_script_executable() {
    if let Ok(md) = fs::metadata("run_checks.sh") {
        let mut perms = md.permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions("run_checks.sh", perms);
    }
}
#[cfg(not(unix))]
fn make_script_executable() {
    // No-op on non-Unix.
}

const DEFAULT_GITIGNORE: &str = r#"# File: .gitignore
# Rust
/target
**/*.rs.bk

# Build artifacts
*.o
*.rlib
*.rmeta

# Lockfiles (keep if you want reproducible builds)
/Cargo.lock

# Coverage and profiles
coverage/
*.profraw

# IDE
.vscode/
.idea/
*.iml

# OS cruft
.DS_Store
Thumbs.db

# Logs
*.log

# Env files
.env
.env.*
"#;

const DEFAULT_RUSTFMT: &str = r#"# File: rustfmt.toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
reorder_imports = true
use_small_heuristics = "Max"
"#;

const DEFAULT_SCRIPT: &str = r#"#!/usr/bin/env bash
# File: run_checks.sh
set -euo pipefail

echo "[1/6] cargo fmt --all"
cargo fmt --all

echo "[2/6] cargo clippy --all-targets --all-features -- -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "[3/6] cargo check"
cargo check

echo "[4/6] cargo test"
cargo test

echo "[5/6] cargo build --release"
cargo build --release

echo "[6/6] ./target/release/run_checks checks"
./target/release/run_checks checks
"#;

const DEFAULT_LICENSE: &str = r#"MIT License

Copyright (c) 2025 Joshua Wood

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the “Software”), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
"#;
