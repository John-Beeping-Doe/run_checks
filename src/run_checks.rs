// src/run_checks.rs

use owo_colors::OwoColorize;

use crate::run_command;

pub fn run_checks() -> bool {
    println!("{}", "Running rustfmt to format Rust files...".cyan());
    if !run_command("cargo", &["fmt", "--all"]) {
        eprintln!("{}", "rustfmt failed!".red());
        return false;
    }
    println!("{}", "All files formatted successfully!".green());

    println!("{}", "Running clippy for lint checks...".cyan());
    if !run_command(
        "cargo",
        &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    ) {
        eprintln!(
            "{}",
            "Clippy checks failed! Please review the warnings/errors above.".red()
        );
        return false;
    }
    println!("{}", "Clippy checks passed with no warnings!".green());

    println!(
        "{}",
        "Running cargo check for type and syntax validation...".cyan()
    );
    if !run_command("cargo", &["check"]) {
        eprintln!(
            "{}",
            "Cargo check failed! Please review the errors above.".red()
        );
        return false;
    }
    println!("{}", "Cargo check passed successfully!".green());

    println!(
        "{}",
        "Running cargo test to verify code functionality...".cyan()
    );
    if !run_command("cargo", &["test"]) {
        eprintln!(
            "{}",
            "Some tests failed! Please review the test output above.".red()
        );
        return false;
    }
    println!("{}", "All tests passed successfully!".green());

    println!(
        "{}",
        "All checks passed successfully! Ready to proceed!".green()
    );
    true
}
