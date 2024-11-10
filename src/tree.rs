// src/tree.rs

// Import modules for file system operations and path handling
use std::{fs, path::Path};

// Import utility for colored terminal output
use owo_colors::OwoColorize;

/// Displays the directory structure starting from the current directory.
///
/// This function prints the directory structure up to two levels deep.
/// If any error occurs during traversal, it will be displayed in the terminal.
pub fn display_tree() {
    println!("{}", "Directory structure (up to 2 levels):".cyan());

    // Start directory traversal and handle potential errors
    if let Err(e) = visit_dirs(Path::new("."), 0) {
        eprintln!("Error: {}", e); // Print error if traversal fails
    }
}

/// Recursively visits directories and prints their structure.
///
/// # Parameters
/// - `dir`: A reference to the starting directory path.
/// - `level`: The current depth of the traversal (0 for root level).
///
/// # Returns
/// - `Ok(())` on successful traversal.
/// - `Err(std::io::Error)` if any error occurs.
///
/// # Behavior
/// - Stops recursion once the level exceeds 1.
/// - Prints directories and files with proper indentation based on their depth.
pub fn visit_dirs(dir: &Path, level: usize) -> std::io::Result<()> {
    // Stop recursion if the current level exceeds the maximum depth (1)
    if level > 1 {
        return Ok(());
    }

    // Check if the given path is a directory
    if dir.is_dir() {
        // Iterate over directory entries
        for entry in fs::read_dir(dir)? {
            let entry = entry?; // Unwrap the directory entry
            let path = entry.path(); // Get the full path of the entry

            if path.is_dir() {
                // Print directory name with indentation
                println!("{:indent$}[DIR] {}", "", path.display(), indent = level * 2);

                // Recursively visit the subdirectory
                visit_dirs(&path, level + 1)?;
            } else {
                // Print file name with indentation
                println!(
                    "{:indent$}[FILE] {}",
                    "",
                    path.display(),
                    indent = level * 2
                );
            }
        }
    }

    Ok(()) // Return success
}
