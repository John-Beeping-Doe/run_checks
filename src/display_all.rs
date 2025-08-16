// Package run_checks
// File: src/display_all.rs

use owo_colors::OwoColorize;
use std::{fs, path::Path};

/// Build a single String that contains all `.rs` files under `src`.
pub fn collect_all_rs() -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", "Displaying contents of Rust files in 'src' directory:".cyan()));

    let src_path = Path::new("src");
    if !src_path.exists() || !src_path.is_dir() {
        out.push_str(&format!("{}\n", "'src' directory not found.".red()));
        return out;
    }

    // Collect .rs files then sort for stable output.
    let mut files = match fs::read_dir(src_path) {
        Ok(entries) => entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
            .collect::<Vec<_>>(),
        Err(_) => {
            out.push_str(&format!("{}\n", "Failed to read 'src' directory.".red()));
            return out;
        }
    };
    files.sort();

    for path in files {
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
                out.push_str(&format!("{}\n", format!("Failed to read contents of {path:?}").red()))
            }
        }
    }

    out
}
