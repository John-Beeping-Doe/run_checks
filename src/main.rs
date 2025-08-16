// Package run_checks
// File: src/main.rs

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

mod display_all;
mod run_checks;
mod tree;

/// CLI for one-shot checks and project introspection.
#[derive(Parser)]
#[command(
    name = "run_checks",
    version,
    about = "Run checks and helpers, then exit.",
    after_help = "\
Examples:
  cargo run -- checks
      Run rustfmt, clippy, cargo check, and cargo test. Print a summary table.

  cargo run -- all --depth 3 --clear
      Run all checks, then display all source files and a directory tree
      up to depth 3, clearing the screen before each section.

  ./run_checks checks
      Use the compiled binary in production or CI to run the checks and print the tables.

  ./run_checks all --depth 3 --clear
      Run checks, show file contents and a directory tree using the installed binary."
)]
struct Cli {
    /// Optional global clear before printing each subcommand output
    #[arg(long)]
    clear: bool,

    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run rustfmt, clippy, check, test. Print a summary table.
    Checks,
    /// Print a directory tree. Default depth=2.
    Tree {
        #[arg(long, default_value_t = 2)]
        depth: usize,
    },
    /// Print all .rs files under src.
    Files,
    /// Run `checks`, then `files`, then `tree`.
    All {
        #[arg(long, default_value_t = 2)]
        depth: usize,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut exit_code = 0usize;

    match cli.cmd {
        Command::Checks => {
            maybe_clear(cli.clear);
            let ok = run_checks::run_checks().await;
            if !ok {
                eprintln!("{}", "Some checks failed.".red());
                exit_code = 1;
            }
        }
        Command::Tree { depth } => {
            maybe_clear(cli.clear);
            tree::display_tree(depth);
        }
        Command::Files => {
            maybe_clear(cli.clear);
            display_all::display_all();
        }
        Command::All { depth } => {
            maybe_clear(cli.clear);
            let ok = run_checks::run_checks().await;
            if !ok {
                eprintln!("{}", "Checks failed. Skipping files/tree.".red());
                exit_code = 1;
            } else {
                display_all::display_all();
                tree::display_tree(depth);
            }
        }
    }

    std::process::exit(exit_code as i32);
}

fn maybe_clear(clear: bool) {
    if !clear {
        return;
    }
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd").args(["/C", "cls"]).status();
    }
    #[cfg(not(windows))]
    {
        let _ = std::process::Command::new("clear").status();
    }
}
