// src/run_checks.rs

use comfy_table::{presets::UTF8_FULL, Cell, Row, Table};
use futures::future::join_all;
use owo_colors::OwoColorize;
use std::time::Instant;
use tokio::process::Command;

use crate::run_command_with_env;

pub async fn run_checks() -> bool {
    let start_all = Instant::now();
    let mut summary = Vec::new();

    // Helper to execute tools
    async fn run_tool(name: &str, cmd: Vec<&str>) -> (String, String, String) {
        let start = Instant::now();
        let status = Command::new(cmd[0])
            .args(&cmd[1..])
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false);
        let elapsed = format!("{:.3} seconds", start.elapsed().as_secs_f64());
        let result = if status {
            "Success".green().to_string()
        } else {
            "Failed".red().to_string()
        };
        (name.to_string(), result, elapsed)
    }

    let tools = vec![
        ("rustfmt", vec!["cargo", "fmt", "--all"]),
        (
            "clippy",
            vec![
                "cargo",
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("cargo check", vec!["cargo", "check"]),
        ("cargo test", vec!["cargo", "test"]),
    ];

    // Run all tools concurrently
    let results = join_all(
        tools
            .into_iter()
            .map(|(name, cmd)| run_tool(name, cmd.into_iter().collect())),
    )
    .await;

    for (name, result, elapsed) in results {
        summary.push((name, result, elapsed));
    }

    let total_time = format!("{:.3} seconds", start_all.elapsed().as_secs_f64());

    // Create a table using `comfy-table`
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL) // Use a nice UTF-8 preset for borders
        .set_header(vec!["Tool", "Status", "Time Elapsed"]);

    for (tool, status, time) in &summary {
        table.add_row(vec![Cell::new(tool), Cell::new(status), Cell::new(time)]);
    }

    table.add_row(Row::from(vec![
        Cell::new("Total time elapsed:").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(""),
        Cell::new(total_time).add_attribute(comfy_table::Attribute::Bold),
    ]));

    // Print the table
    println!("\n{}", table);

    // Return whether all tools succeeded
    summary
        .iter()
        .all(|(_, status, _)| status.contains("Success"))
}

/// Combines `run_checks` with application execution.
///
/// This function runs all project checks and, if successful:
/// - Executes the application using `cargo run`.
/// - Includes `RUST_BACKTRACE=1` to enable debugging information.
///
/// If the checks fail, the function outputs an error message.
pub async fn check_and_run() {
    if run_checks().await {
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
