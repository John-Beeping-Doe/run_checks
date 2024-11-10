// src/main.rs

use crate::run_checks::run_checks;
use crate::tree::display_tree;

use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;
use std::{
    fs, io,
    path::Path,
    process::{exit, Command},
};

mod run_checks;
mod tree;

fn main() {
    loop {
        clear_terminal();

        println!("{}", "=== Rust CLI Menu ===".cyan());
        let options = &["Check and Run", "Run Checks", "Tree", "Display All", "Exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => check_and_run(),
            1 => {
                if !run_checks() {
                    eprintln!("{}", "Run Checks completed with failures.".red());
                }
            }
            2 => display_tree(),
            3 => display_all(),
            4 => {
                println!("{}", "Exiting...".green());
                exit(0);
            }
            _ => unreachable!(),
        }

        println!("\nPress Enter to return to the menu...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}

fn check_and_run() {
    if run_checks() {
        println!(
            "{}",
            "All checks passed. Running the application...".green()
        );
        if !run_command_with_env("cargo", &["run"], &[("RUST_BACKTRACE", "1")]) {
            eprintln!("{}", "Failed to run the application.".red());
        }
    } else {
        eprintln!("{}", "Checks failed. Application will not run.".red());
    }
}

fn display_all() {
    clear_terminal();
    println!(
        "{}",
        "Displaying contents of Rust files in 'src' directory:".cyan()
    );

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
                            println!("{}", contents);
                        }
                        Err(_) => {
                            eprintln!("{}", format!("Failed to read contents of {:?}", path).red())
                        }
                    }
                }
            }
        }
        Err(_) => eprintln!("{}", "Failed to read 'src' directory.".red()),
    }
}

fn run_command(command: &str, args: &[&str]) -> bool {
    match Command::new(command).args(args).status() {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn run_command_with_env(command: &str, args: &[&str], env_vars: &[(&str, &str)]) -> bool {
    let mut cmd = Command::new(command);
    cmd.args(args);
    for &(key, value) in env_vars {
        cmd.env(key, value);
    }
    match cmd.status() {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
