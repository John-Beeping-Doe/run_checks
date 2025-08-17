// Package run_checks
// File: src/display_all.rs

use owo_colors::OwoColorize;
use std::{fs, path::Path};
use walkdir::WalkDir;

const ALLOWED_EXTS: &[&str] = &["rs", "md", "sh", "toml"];

fn is_allowed_file(p: &Path) -> bool {
    p.extension().and_then(|s| s.to_str()).map(|ext| ALLOWED_EXTS.contains(&ext)).unwrap_or(false)
}

/// Build a single String that contains all allowed text files in the repo,
/// skipping `target/`, `.git/`, `node_modules/`, and symlinks.
pub fn collect_all_rs() -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", "Displaying contents of .rs/.md/.sh/.toml files:".cyan()));

    // Gather & sort paths for stable output.
    let mut paths = Vec::new();
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let p = e.path();
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            !(name == ".git" || name == "target" || name == "node_modules" || p.is_symlink())
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let p = entry.path().to_path_buf();
        if is_allowed_file(&p) {
            paths.push(p);
        }
    }
    paths.sort();

    for path in paths {
        match fs::read_to_string(&path) {
            Ok(contents) => {
                out.push_str(&format!(
                    "{}\n{}\n{}\n",
                    "========================================".green(),
                    path.display(),
                    "========================================".green()
                ));
                out.push_str(&contents);
                if !contents.ends_with('\n') {
                    out.push('\n');
                }
            }
            Err(_) => {
                // Non-UTF8 or unreadable: skip (avoids binaries).
            }
        }
    }

    out
}
