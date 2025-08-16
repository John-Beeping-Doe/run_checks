// Package run_checks
// File: src/run_checks.rs

use aho_corasick::AhoCorasickBuilder;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, CellAlignment, Color, Row, Table};
use futures::future::join_all;
use std::{collections::BTreeSet, env, fs, path::Path, time::Instant};
use tokio::process::Command;
use walkdir::WalkDir;

/// Run rustfmt, clippy, cargo check, cargo test, then privacy/security scans.
/// Returns true if all tool checks succeeded.
pub async fn run_checks() -> bool {
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
    println!("\n{table}");

    let sec_table = build_privacy_security_table();
    println!("\n{sec_table}");

    all_ok
}

/// Build the privacy/security scan table using a single-pass multi-pattern search.
fn build_privacy_security_table() -> Table {
    let usernames = gather_usernames();
    let hostnames = gather_hostnames();
    let ips = gather_ips();

    // Token registry: (kind, value)
    let mut kinds: Vec<&'static str> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for u in usernames {
        kinds.push("Username");
        values.push(u);
    }
    for h in hostnames {
        kinds.push("Hostname");
        values.push(h);
    }
    for ip in ips {
        kinds.push("IP");
        values.push(ip);
    }

    // Nothing to check.
    if values.is_empty() {
        let mut t = Table::new();
        t.load_preset(UTF8_FULL).set_header(vec![
            "Security/Privacy Check",
            "Value",
            "Status",
            "Details",
            "Locations",
        ]);
        t.add_row(vec![
            Cell::new("Scan"),
            Cell::new("No candidates"),
            Cell::new("N/A").fg(Color::Yellow),
            Cell::new("0"),
            Cell::new(""),
        ]);
        return t;
    }

    // Aho-Corasick automaton for all tokens.
    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(false)
        .build(&values)
        .expect("failed to build Aho-Corasick automaton");

    // Per-token accumulators.
    let mut files_with_hits = vec![0usize; values.len()];
    let mut total_hits = vec![0usize; values.len()];
    let mut locations: Vec<Vec<String>> = vec![Vec::new(); values.len()];

    // Walk repo files once.
    for (path, content) in collect_project_text_files() {
        // Scan per-line to collect line numbers for each token.
        let mut line_hits: Vec<Vec<usize>> = vec![Vec::new(); values.len()];
        for (lineno0, line) in content.lines().enumerate() {
            let lineno = lineno0 + 1;
            let mut matched_on_line: Vec<usize> = Vec::new();

            for m in ac.find_iter(line) {
                let idx = m.pattern().as_usize();
                total_hits[idx] += 1;
                if matched_on_line.last().copied() != Some(idx) {
                    matched_on_line.push(idx);
                }
            }

            for idx in matched_on_line {
                line_hits[idx].push(lineno);
            }
        }

        // Summarize per file.
        for (idx, lines) in line_hits.into_iter().enumerate() {
            if !lines.is_empty() {
                files_with_hits[idx] += 1;
                let list = lines.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
                locations[idx].push(format!("{path}:{list}"));
            }
        }
    }

    // Build table rows.
    let mut t = Table::new();
    t.load_preset(UTF8_FULL).set_header(vec![
        "Security/Privacy Check",
        "Value",
        "Status",
        "Details",
        "Locations (file:lines)",
    ]);
    if let Some(col) = t.column_mut(3) {
        col.set_cell_alignment(CellAlignment::Right);
    }

    for i in 0..values.len() {
        let found = total_hits[i] > 0;
        let status = if found {
            Cell::new("Found").add_attribute(Attribute::Bold).fg(Color::Red)
        } else {
            Cell::new("Not found").add_attribute(Attribute::Bold).fg(Color::Green)
        };

        let locs = if locations[i].is_empty() {
            String::new()
        } else if locations[i].len() <= 5 {
            locations[i].join(" | ")
        } else {
            let shown = locations[i][..5].join(" | ");
            format!("{shown} | +{} more files", locations[i].len() - 5)
        };

        t.add_row(vec![
            Cell::new(kinds[i]),
            Cell::new(&values[i]),
            status,
            Cell::new(if found {
                format!("{} files, {} hits", files_with_hits[i], total_hits[i])
            } else {
                "not found".to_string()
            }),
            Cell::new(locs),
        ]);
    }

    t
}

// Collect candidate usernames from env and common user directories.
fn gather_usernames() -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();

    for key in ["USER", "LOGNAME"] {
        if let Ok(v) = env::var(key) {
            let v = v.trim();
            if !v.is_empty() {
                set.insert(v.to_string());
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        if let Some(name) = Path::new(&home).file_name().and_then(|s| s.to_str()) {
            if !name.is_empty() {
                set.insert(name.to_string());
            }
        }
    }

    if let Ok(output) = std::process::Command::new("whoami").output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let s = s.trim();
                if !s.is_empty() {
                    set.insert(s.to_string());
                }
            }
        }
    }

    for base in ["/Users", "/home"] {
        let p = Path::new(base);
        if p.is_dir() {
            if let Ok(read) = fs::read_dir(p) {
                for e in read.flatten() {
                    if let Some(name) = e.file_name().to_str() {
                        if !name.starts_with('.') && name.len() > 1 {
                            set.insert(name.to_string());
                        }
                    }
                }
            }
        }
    }

    set.into_iter().collect()
}

// Collect hostname candidates from env and `hostname`.
fn gather_hostnames() -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();

    for key in ["HOSTNAME", "COMPUTERNAME"] {
        if let Ok(v) = env::var(key) {
            let v = v.trim();
            if !v.is_empty() {
                set.insert(v.to_string());
            }
        }
    }

    if let Ok(output) = std::process::Command::new("hostname").output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let s = s.trim();
                if !s.is_empty() {
                    set.insert(s.to_string());
                }
            }
        }
    }

    set.into_iter().collect()
}

// Collect local IP addresses (v4 and v6), excluding loopback and link-local.
fn gather_ips() -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    if let Ok(ifaces) = get_if_addrs::get_if_addrs() {
        for iface in ifaces {
            match iface.ip() {
                std::net::IpAddr::V4(v4) => {
                    if !v4.is_loopback() && !v4.is_link_local() {
                        set.insert(v4.to_string());
                    }
                }
                std::net::IpAddr::V6(v6) => {
                    if !v6.is_loopback()
                        && !v6.is_unspecified()
                        && !v6.is_unique_local()
                        && !v6.is_unicast_link_local()
                    {
                        set.insert(v6.to_string());
                    }
                }
            }
        }
    }
    set.into_iter().collect()
}

// Gather small text files from the repo to search. Skips target/, .git/, node_modules/.
fn collect_project_text_files() -> Vec<(String, String)> {
    let mut files = Vec::new();
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let p = e.path();
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            // Simplified per clippy::nonminimal-bool
            !(name == ".git" || name == "target" || name == "node_modules" || p.is_symlink())
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Ok(md) = path.metadata() {
            if md.len() > 1_000_000 {
                continue;
            }
        }
        if let Ok(content) = fs::read_to_string(path) {
            if let Some(p) = path.to_str() {
                files.push((p.to_string(), content));
            }
        }
    }
    files
}
