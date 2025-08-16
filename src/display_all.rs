// Package run_checks
// File: src/display_all.rs

use owo_colors::OwoColorize;
use std::{fs, path::Path};

/// Print all `.rs` files under `src`.
pub fn display_all() {
    println!("{}", "Displaying contents of Rust files in 'src' directory:".cyan());

    let src_path = Path::new("src");
    if !src_path.exists() || !src_path.is_dir() {
        eprintln!("{}", "'src' directory not found.".red());
        return;
    }

    match fs::read_dir(src_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    match fs::read_to_string(&path) {
                        Ok(contents) => {
                            println!(
                                "{}\n{}\n{}",
                                "========================================".green(),
                                path.display(),
                                "========================================".green()
                            );
                            println!("{contents}");
                        }
                        Err(_) => {
                            eprintln!("{}", format!("Failed to read contents of {path:?}").red())
                        }
                    }
                }
            }
        }
        Err(_) => eprintln!("{}", "Failed to read 'src' directory.".red()),
    }
}
