// src/display_all.rs

// Import standard library modules for file system operations and path handling
use std::{fs, path::Path};

// Import external crate for colored terminal output
use owo_colors::OwoColorize;

// Import `clear_terminal` function from the main module
use crate::clear_terminal;

/// Displays the contents of all Rust (`.rs`) files in the `src` directory.
///
/// - Clears the terminal before displaying the file contents.
/// - Iterates over the files in the `src` directory.
/// - Reads and prints the contents of files with a `.rs` extension.
/// - Skips files that cannot be read or opened.
pub fn display_all() {
    // Clear the terminal screen for better readability
    clear_terminal();
    println!(
        "{}",
        "Displaying contents of Rust files in 'src' directory:".cyan()
    );

    // Define the path to the `src` directory
    let src_path = Path::new("src");

    // Check if the `src` directory exists and is valid
    if !src_path.exists() || !src_path.is_dir() {
        eprintln!("{}", "'src' directory not found.".red()); // Error message if directory is missing
        return; // Exit the function early
    }

    // Read the contents of the `src` directory
    match fs::read_dir(src_path) {
        Ok(entries) => {
            // Iterate over all directory entries
            for entry in entries.flatten() {
                let path = entry.path(); // Get the file path

                // Check if the file has a `.rs` extension
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    // Attempt to read the file's contents
                    match fs::read_to_string(&path) {
                        Ok(contents) => {
                            // Print file separator and path
                            println!(
                                "{}\n{}\n{}",
                                "========================================".green(),
                                path.display(),
                                "========================================".green()
                            );
                            println!("{}", contents); // Print file contents
                        }
                        Err(_) => {
                            // Error message if the file cannot be read
                            eprintln!("{}", format!("Failed to read contents of {:?}", path).red())
                        }
                    }
                }
            }
        }
        // Error message if the directory cannot be read
        Err(_) => eprintln!("{}", "Failed to read 'src' directory.".red()),
    }
}
