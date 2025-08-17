// Snippet
// File: src/run_checks/run_tools.rs

use comfy_table::{presets::UTF8_FULL, Attribute, Cell, CellAlignment, Color, Row, Table};
use futures::future::join_all;
use std::time::Instant;
use tokio::process::Command;

/// Execute core cargo tools and return (all_ok, table_string).
pub async fn run_core_tools_table() -> (bool, String) {
    async fn run_tool(name: &str, cmd: &[&str]) -> (String, bool, String) {
        let start = Instant::now();
        let ok = Command::new(cmd[0])
            .args(&cmd[1..])
            .status()
            .await
            .ok()
            .map(|s| s.success())
            .unwrap_or(false);
        let elapsed = format!("{:.3} seconds", start.elapsed().as_secs_f64());
        (name.to_string(), ok, elapsed)
    }

    let started = Instant::now();
    let tools: &[(&str, &[&str])] = &[
        ("rustfmt", &["cargo", "fmt", "--all"]),
        ("clippy", &["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"]),
        ("cargo check", &["cargo", "check"]),
        ("cargo test", &["cargo", "test"]),
    ];

    let results = join_all(tools.iter().map(|(n, c)| run_tool(n, c))).await;

    let mut table = Table::new();
    table.load_preset(UTF8_FULL).set_header(vec!["Tool", "Status", "Time Elapsed"]);
    if let Some(col) = table.column_mut(2) {
        col.set_cell_alignment(CellAlignment::Right);
    }

    let mut all_ok = true;
    for (name, ok, elapsed) in &results {
        if !ok {
            all_ok = false;
        }
        let status_cell = if *ok {
            Cell::new("Success").add_attribute(Attribute::Bold).fg(Color::Green)
        } else {
            Cell::new("Failed").add_attribute(Attribute::Bold).fg(Color::Red)
        };
        table.add_row(vec![
            Cell::new(name),
            status_cell,
            Cell::new(elapsed).set_alignment(CellAlignment::Right),
        ]);
    }

    table.add_row(Row::from(vec![
        Cell::new("Total time elapsed:").add_attribute(Attribute::Bold),
        Cell::new(""),
        Cell::new(format!("{:.3} seconds", started.elapsed().as_secs_f64()))
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
    ]));

    (all_ok, table.to_string())
}
