// Package run_checks
// File: src/run_checks.rs

use comfy_table::{presets::UTF8_FULL, Attribute, Cell, CellAlignment, Color, Row, Table};
use futures::future::join_all;
use std::{collections::BTreeSet, env, fs, path::Path, time::Instant};
use tokio::process::Command;
use walkdir::WalkDir;

/// Run rustfmt, clippy, cargo check, cargo test, then privacy/security scans.
/// Returns true if all tool checks succeeded.
pub async fn run_checks() -> bool {
    // ---------- primary tool checks ----------
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

    // ---------- privacy & security checks ----------
    let sec_table = build_privacy_security_table();
    println!("\n{sec_table}");

    all_ok
}

// Build the second table with privacy/security scan results.
fn build_privacy_security_table() -> Table {
    let usernames = gather_usernames();
    let hostnames = gather_hostnames();
    let ips = gather_ips();

    // tokens -> (files_with_hits, total_hits, locations)
    let mut rows: Vec<(String, String, bool, String, String)> = Vec::new();

    // Scan repository text files once; then check each token against each file.
    let file_texts = collect_project_text_files();

    for u in &usernames {
        let (files, hits, locations) = search_token_in_repo(u, &file_texts);
        let found = hits > 0;
        rows.push((
            "Username".to_string(),
            u.clone(),
            found,
            if found { format!("{files} files, {hits} hits") } else { "not found".to_string() },
            locations,
        ));
    }

    for h in &hostnames {
        let (files, hits, locations) = search_token_in_repo(h, &file_texts);
        let found = hits > 0;
        rows.push((
            "Hostname".to_string(),
            h.clone(),
            found,
            if found { format!("{files} files, {hits} hits") } else { "not found".to_string() },
            locations,
        ));
    }

    for ip in &ips {
        let (files, hits, locations) = search_token_in_repo(ip, &file_texts);
        let found = hits > 0;
        rows.push((
            "IP".to_string(),
            ip.clone(),
            found,
            if found { format!("{files} files, {hits} hits") } else { "not found".to_string() },
            locations,
        ));
    }

    // Build table
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

    if rows.is_empty() {
        t.add_row(vec![
            Cell::new("Scan"),
            Cell::new("No candidates"),
            Cell::new("N/A").fg(Color::Yellow),
            Cell::new("0"),
            Cell::new(""),
        ]);
        return t;
    }

    for (kind, value, found, details, locations) in rows {
        let status = if found {
            Cell::new("Found").add_attribute(Attribute::Bold).fg(Color::Red)
        } else {
            Cell::new("Not found").add_attribute(Attribute::Bold).fg(Color::Green)
        };
        t.add_row(vec![
            Cell::new(kind),
            Cell::new(value),
            status,
            Cell::new(details),
            Cell::new(locations),
        ]);
    }

    t
}

// Collect candidate usernames from env and common user directories.
fn gather_usernames() -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();

    for key in ["USER", "LOGNAME"].iter() {
        if let Ok(v) = env::var(key) {
            if !v.trim().is_empty() {
                set.insert(v);
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

    // whoami
    if let Ok(output) = std::process::Command::new("whoami").output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let s = s.trim().to_string();
                if !s.is_empty() {
                    set.insert(s);
                }
            }
        }
    }

    // Directory names under /Users (macOS) and /home (Linux)
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

    for key in ["HOSTNAME", "COMPUTERNAME"].iter() {
        if let Ok(v) = env::var(key) {
            if !v.trim().is_empty() {
                set.insert(v);
            }
        }
    }

    if let Ok(output) = std::process::Command::new("hostname").output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let s = s.trim().to_string();
                if !s.is_empty() {
                    set.insert(s);
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
            let ip = iface.ip();
            match ip {
                std::net::IpAddr::V4(v4) => {
                    if !v4.is_loopback() && !v4.is_link_local() {
                        set.insert(v4.to_string());
                    }
                }
                std::net::IpAddr::V6(v6) => {
                    if !v6.is_loopback() && !v6.is_unspecified() && !v6.is_unique_local() {
                        // Skip link-local (fe80::/10); clippy-friendly form.
                        let seg = v6.segments();
                        if (seg[0] & 0xffc0) != 0xfe80 {
                            set.insert(v6.to_string());
                        }
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
            if name == ".git" || name == "target" || name == "node_modules" {
                return false;
            }
            true
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        // Skip large files (> 1 MB)
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

/// Search a single token across collected files.
/// Returns (files_with_hits, total_hits, locations_string).
fn search_token_in_repo(token: &str, files: &[(String, String)]) -> (usize, usize, String) {
    if token.is_empty() {
        return (0, 0, String::new());
    }
    let mut files_with_hits = 0usize;
    let mut total_hits = 0usize;
    let mut locs: Vec<String> = Vec::new();

    for (path, text) in files {
        let mut file_hit = false;
        let mut line_nums: Vec<usize> = Vec::new();

        for (idx, line) in text.lines().enumerate() {
            let mut off = 0usize;
            let mut line_hits = 0usize;
            while let Some(p) = line[off..].find(token) {
                total_hits += 1;
                line_hits += 1;
                off += p + token.len();
            }
            if line_hits > 0 {
                file_hit = true;
                line_nums.push(idx + 1); // 1-based
            }
        }

        if file_hit {
            files_with_hits += 1;
            // Format: path:line1,line2,...
            let joined = line_nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
            locs.push(format!("{path}:{joined}"));
        }
    }

    // Keep the string compact if many hits.
    let locations = if locs.is_empty() {
        String::new()
    } else if locs.len() <= 5 {
        locs.join(" | ")
    } else {
        let shown = locs[..5].join(" | ");
        format!("{shown} | +{} more files", locs.len() - 5)
    };

    (files_with_hits, total_hits, locations)
}
