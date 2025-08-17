mod privacy;
mod run_tools;

use privacy::build_privacy_security_table;
use run_tools::run_core_tools_table;

/// Run rustfmt, clippy, check, test, and the privacy/security scans.
/// `run_extras` toggles the Extra scans row.
/// Returns (all_ok, printable_blob).
pub async fn run_checks(run_extras: bool) -> (bool, String) {
    let (all_ok, tools_table) = run_core_tools_table().await;
    let sec_table = build_privacy_security_table(run_extras);

    let mut out = String::new();
    out.push('\n');
    out.push_str(&tools_table);
    out.push_str("\n\n");
    out.push_str(&sec_table.to_string());
    out.push('\n');

    (all_ok, out)
}
