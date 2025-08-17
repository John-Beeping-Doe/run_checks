// Package run_checks
// File: src/tree.rs

use owo_colors::OwoColorize;
use std::{fs, path::Path};

const ALLOWED_EXTS: &[&str] = &["rs", "md", "sh", "toml"];

fn should_skip_dir(p: &Path) -> bool {
    let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
    name == ".git" || name == "target" || name == "node_modules"
}

fn is_allowed_file(p: &Path) -> bool {
    p.extension().and_then(|s| s.to_str()).map(|ext| ALLOWED_EXTS.contains(&ext)).unwrap_or(false)
}

/// Build a directory tree string from `.` to `max_depth`, showing only allowed files
/// and skipping `target/`, `.git/`, `node_modules/`.
pub fn collect_tree(max_depth: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        format!("Directory structure (allowed files only, up to {max_depth} levels):").cyan()
    ));
    if let Err(e) = visit_dirs(Path::new("."), 0, max_depth, &mut out) {
        out.push_str(&format!("Error: {e}\n"));
    }
    out
}

fn visit_dirs(dir: &Path, level: usize, max_depth: usize, out: &mut String) -> std::io::Result<()> {
    if level > max_depth {
        return Ok(());
    }
    if dir.is_dir() {
        if should_skip_dir(dir) && level > 0 {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if should_skip_dir(&path) {
                    continue;
                }
                out.push_str(&format!(
                    "{:indent$}[DIR]  {}\n",
                    "",
                    path.display(),
                    indent = level * 2
                ));
                visit_dirs(&path, level + 1, max_depth, out)?;
            } else if is_allowed_file(&path) {
                out.push_str(&format!(
                    "{:indent$}[FILE] {}\n",
                    "",
                    path.display(),
                    indent = level * 2
                ));
            }
        }
    }
    Ok(())
}
