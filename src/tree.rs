// Package run_checks
// File: src/tree.rs

use owo_colors::OwoColorize;
use std::{fs, path::Path};

/// Print a directory tree from `.` to `max_depth`.
pub fn display_tree(max_depth: usize) {
    println!("{}", format!("Directory structure (up to {max_depth} levels):").cyan());
    if let Err(e) = visit_dirs(Path::new("."), 0, max_depth) {
        eprintln!("Error: {e}");
    }
}

fn visit_dirs(dir: &Path, level: usize, max_depth: usize) -> std::io::Result<()> {
    if level > max_depth {
        return Ok(());
    }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("{:indent$}[DIR]  {}", "", path.display(), indent = level * 2);
                visit_dirs(&path, level + 1, max_depth)?;
            } else {
                println!("{:indent$}[FILE] {}", "", path.display(), indent = level * 2);
            }
        }
    }
    Ok(())
}
