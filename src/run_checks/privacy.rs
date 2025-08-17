// Snippet
// File: src/run_checks/privacy.rs

use aho_corasick::AhoCorasickBuilder;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, CellAlignment, Color, Table};
use regex::Regex;
use std::{collections::BTreeSet, env, fs, path::Path};
use walkdir::WalkDir;

pub fn build_privacy_security_table(run_extras: bool) -> Table {
    let usernames = gather_usernames();
    let hostnames = gather_hostnames();
    let ips = gather_ips();

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

    if values.is_empty() {
        let mut t = base_table();
        t.add_row(vec![
            Cell::new("Scan"),
            Cell::new("No candidates"),
            Cell::new("N/A").fg(Color::Yellow),
            Cell::new("0"),
            Cell::new(""),
        ]);
        add_extras_row(run_extras, &mut t);
        return t;
    }

    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(false)
        .build(&values)
        .expect("failed to build Aho-Corasick automaton");

    let mut files_with_hits = vec![0usize; values.len()];
    let mut total_hits = vec![0usize; values.len()];
    let mut locations: Vec<Vec<String>> = vec![Vec::new(); values.len()];

    for (path, content) in collect_project_text_files() {
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

        for (idx, lines) in line_hits.into_iter().enumerate() {
            if !lines.is_empty() {
                files_with_hits[idx] += 1;
                let list = join_usize(&lines);
                locations[idx].push(format!("{path}:{list}"));
            }
        }
    }

    let mut t = base_table();
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
            Cell::new(if i < kinds.len() { kinds[i] } else { "?" }),
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

    add_extras_row(run_extras, &mut t);
    t
}

fn base_table() -> Table {
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
    t
}

fn add_extras_row(run_extras: bool, t: &mut Table) {
    if run_extras {
        let (extra_found, extra_details, extra_locs) = run_extra_scans();
        let status = if extra_found {
            Cell::new("Found").add_attribute(Attribute::Bold).fg(Color::Red)
        } else {
            Cell::new("Not found").add_attribute(Attribute::Bold).fg(Color::Green)
        };
        let locs = if extra_locs.is_empty() {
            String::new()
        } else if extra_locs.len() <= 5 {
            extra_locs.join(" | ")
        } else {
            let shown = extra_locs[..5].join(" | ");
            format!("{shown} | +{} more files", extra_locs.len() - 5)
        };
        t.add_row(vec![
            Cell::new("Extra scans"),
            Cell::new("secrets, PEM, leak-files, docs/examples/tests"),
            status,
            Cell::new(extra_details),
            Cell::new(locs),
        ]);
    } else {
        t.add_row(vec![
            Cell::new("Extra scans"),
            Cell::new("secrets, PEM, leak-files, docs/examples/tests"),
            Cell::new("Skipped").add_attribute(Attribute::Bold).fg(Color::Yellow),
            Cell::new("Run with: `run_checks checks-extras`"),
            Cell::new(""),
        ]);
    }
}

// Helpers

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

// Gather ALLOWED small text files in the repo.
fn collect_project_text_files() -> Vec<(String, String)> {
    const ALLOWED_EXTS: &[&str] = &["rs", "md", "sh", "toml"];
    fn is_allowed_file(p: &Path) -> bool {
        p.extension()
            .and_then(|s| s.to_str())
            .map(|ext| ALLOWED_EXTS.contains(&ext))
            .unwrap_or(false)
    }

    let mut files = Vec::new();
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let p = e.path();
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            !(name == ".git" || name == "target" || name == "node_modules" || p.is_symlink())
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if !is_allowed_file(path) {
            continue;
        }
        if let Ok(md) = path.metadata() {
            if md.len() > 1_000_000 {
                continue;
            }
        }
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Some(p) = path.to_str() {
                files.push((p.to_string(), content));
            }
        }
    }
    files
}

fn run_extra_scans() -> (bool, String, Vec<String>) {
    let re_words =
        Regex::new(r"(?i)\b(api|secret|token|key|password|passwd|bearer|authorization)\b").unwrap();
    let re_aws = Regex::new(r"AKIA[0-9A-Z]{16}").unwrap();
    let re_gh = Regex::new(r"ghp_[A-Za-z0-9]{36,}").unwrap();
    let re_slack = Regex::new(r"xox[baprs]-[A-Za-z0-9-]{10,}").unwrap();
    let re_pem = Regex::new(r"BEGIN (RSA|DSA|EC|OPENSSH) (PRIVATE|PUBLIC) KEY").unwrap();
    let re_pii =
        Regex::new(r"(?i)(email|@example|phone|address|SIN|SSN|passport|license)").unwrap();

    let text_files = collect_project_text_files();

    let mut total_hits = 0usize;
    let mut locs: Vec<String> = Vec::new();
    let mut files_with_issues = BTreeSet::new();

    for (path, content) in &text_files {
        let mut line_nums: Vec<usize> = Vec::new();
        for (i0, line) in content.lines().enumerate() {
            let i = i0 + 1;
            if re_words.is_match(line)
                || re_aws.is_match(line)
                || re_gh.is_match(line)
                || re_slack.is_match(line)
                || re_pem.is_match(line)
            {
                total_hits += 1;
                if line_nums.last().copied() != Some(i) {
                    line_nums.push(i);
                }
            }
        }
        if !line_nums.is_empty() {
            files_with_issues.insert(path.clone());
            locs.push(format!("{path}:{}", join_usize(&line_nums)));
        }
    }

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let p = e.path();
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            !(name == ".git" || name == "target" || name == "node_modules" || p.is_symlink())
        })
        .filter_map(Result::ok)
    {
        let p = entry.path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let path_str = p.to_string_lossy();

        let is_leak_file = name.starts_with(".env")
            || name == ".envrc"
            || name == "kubeconfig"
            || name.starts_with("id_rsa")
            || name.starts_with("id_ed25519")
            || name.ends_with(".pem")
            || name.ends_with(".p12")
            || name.ends_with(".crt")
            || name.ends_with(".key")
            || path_str.ends_with(".kube/config")
            || name == ".npmrc"
            || name == ".pypirc"
            || name == ".netrc"
            || name == ".git-credentials"
            || name == ".aws";

        if is_leak_file {
            files_with_issues.insert(path_str.to_string());
            locs.push(path_str.to_string());
            total_hits += 1;
        }
    }

    for (path, content) in &text_files {
        let lower = path.to_lowercase();
        if !(lower.starts_with("docs/")
            || lower.contains("/docs/")
            || lower.starts_with("examples/")
            || lower.contains("/examples/")
            || lower.starts_with("tests/")
            || lower.contains("/tests/"))
        {
            continue;
        }
        let mut line_nums: Vec<usize> = Vec::new();
        for (i0, line) in content.lines().enumerate() {
            let i = i0 + 1;
            if re_pii.is_match(line) {
                total_hits += 1;
                if line_nums.last().copied() != Some(i) {
                    line_nums.push(i);
                }
            }
        }
        if !line_nums.is_empty() {
            files_with_issues.insert(path.clone());
            locs.push(format!("{path}:{}", join_usize(&line_nums)));
        }
    }

    let found = !files_with_issues.is_empty();
    let details = if found {
        format!("{} files, {} findings", files_with_issues.len(), total_hits)
    } else {
        "not found".to_string()
    };
    (found, details, locs)
}

fn join_usize(nums: &[usize]) -> String {
    nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")
}
