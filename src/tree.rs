// src/tree.rs

use std::{fs, path::Path};

use owo_colors::OwoColorize;

pub fn display_tree() {
    println!("{}", "Directory structure (up to 2 levels):".cyan());
    if let Err(e) = visit_dirs(Path::new("."), 0) {
        eprintln!("Error: {}", e);
    }
}

pub fn visit_dirs(dir: &Path, level: usize) -> std::io::Result<()> {
    if level > 1 {
        return Ok(());
    }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("{:indent$}[DIR] {}", "", path.display(), indent = level * 2);
                visit_dirs(&path, level + 1)?;
            } else {
                println!(
                    "{:indent$}[FILE] {}",
                    "",
                    path.display(),
                    indent = level * 2
                );
            }
        }
    }
    Ok(())
}
