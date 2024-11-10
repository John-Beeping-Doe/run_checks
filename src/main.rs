// src/main.rs

use crate::display_all::display_all;
use crate::run_checks::{check_and_run, run_checks};
use crate::tree::display_tree;

use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;
use std::process::{exit, Command};

mod display_all;
mod run_checks;
mod tree;

/// Main entry point of the program, displaying the CLI menu in a loop.
#[tokio::main]
async fn main() {
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
            0 => check_and_run().await,
            1 => {
                if !run_checks().await {
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
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

/// Clears the terminal screen for a clean UI.
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

/// Executes a command with custom environment variables.
///
/// # Parameters
/// - `command`: The command to run.
/// - `args`: Arguments to pass to the command.
/// - `env_vars`: Environment variables as a slice of key-value pairs.
///
/// # Returns
/// - `true` if the command succeeds, `false` otherwise.
fn run_command_with_env(command: &str, args: &[&str], env_vars: &[(&str, &str)]) -> bool {
    let mut cmd = Command::new(command);
    cmd.args(args); // Add command-line arguments
    for &(key, value) in env_vars {
        cmd.env(key, value); // Set environment variables for the command
    }
    match cmd.status() {
        Ok(status) => status.success(), // Return true if the command exits successfully
        Err(_) => false,                // Return false on error
    }
}
