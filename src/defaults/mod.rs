// Snippet
// File: src/defaults/mod.rs

use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, Table};
use owo_colors::OwoColorize;
use std::fmt::Write as _; // for write! to String
use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

mod templates;
use templates::{
    DEFAULT_CARGO_CONFIG, DEFAULT_CHANGELOG, DEFAULT_CONTRIBUTING, DEFAULT_EDITORCONFIG,
    DEFAULT_GITIGNORE, DEFAULT_LICENSE, DEFAULT_MAIN_RS, DEFAULT_README, DEFAULT_RUSTFMT,
    DEFAULT_SCRIPT,
};

/// Ensure default files and folders exist; create them if missing.
/// Also creates `src/main.rs` from template if absent.
pub fn ensure_defaults() -> String {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL).set_header(vec!["Path", "Action", "Details"]);

    // Minimal required directory: .cargo
    {
        let d = ".cargo";
        let p = Path::new(d);
        if p.exists() {
            table.add_row(vec![
                Cell::new(d),
                Cell::new("exists").add_attribute(Attribute::Bold).fg(Color::Green),
                Cell::new("dir present"),
            ]);
        } else if let Err(e) = fs::create_dir_all(p) {
            table.add_row(vec![
                Cell::new(d),
                Cell::new("error").add_attribute(Attribute::Bold).fg(Color::Red),
                Cell::new(e.to_string()),
            ]);
        } else {
            table.add_row(vec![
                Cell::new(d),
                Cell::new("created").add_attribute(Attribute::Bold).fg(Color::Yellow),
                Cell::new("directory"),
            ]);
        }
    }

    // Files to materialize from templates.
    for (path, contents, post) in [
        (".gitignore", DEFAULT_GITIGNORE, None::<fn()>),
        ("rustfmt.toml", DEFAULT_RUSTFMT, None::<fn()>),
        ("run_checks.sh", DEFAULT_SCRIPT, Some(make_script_executable as fn())),
        ("LICENSE", DEFAULT_LICENSE, None::<fn()>),
        ("README.md", DEFAULT_README, None::<fn()>),
        ("CHANGELOG.md", DEFAULT_CHANGELOG, None::<fn()>),
        ("CONTRIBUTING.md", DEFAULT_CONTRIBUTING, None::<fn()>),
        (".editorconfig", DEFAULT_EDITORCONFIG, None::<fn()>),
        (".cargo/config.toml", DEFAULT_CARGO_CONFIG, None::<fn()>),
        ("src/main.rs", DEFAULT_MAIN_RS, None::<fn()>),
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
            Ok(()) => {
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

    let mut out = String::new();
    let _ = writeln!(out, "{}", "Create Defaults? results:".cyan());
    let _ = writeln!(out, "{table}");
    out
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
