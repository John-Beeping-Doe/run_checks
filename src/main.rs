// Snippet
// File: src/main.rs

use clap::{Parser, Subcommand};

mod defaults; // now a module directory: src/defaults/mod.rs + templates.rs
mod display_all;
mod run_checks; // orchestrates core tools + privacy table
mod tree;
mod util; // clipboard, clear screen, ANSI stripping

/// CLI for one-shot checks and project introspection.
#[derive(Parser)]
#[command(
    name = "run_checks",
    version,
    about = "Run checks and helpers, then exit.",
    after_help = "\
Examples:
  cargo run -- checks
  cargo run -- \"Checks plus Extras\"
  cargo run -- create-defaults
  cargo run -- all --depth 3 --clear
  ./run_checks all --depth 3 --clear"
)]
struct Cli {
    /// Clear the screen before each subcommand output
    #[arg(long)]
    clear: bool,

    #[command(subcommand)]
    cmd: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    /// Run rustfmt, clippy, check, test. Prints tables. Extra scans are skipped. Copies to clipboard.
    Checks,

    /// Same as `checks` but runs the Extra scans row. Copies to clipboard.
    #[command(name = "checks-extras", visible_alias = "Checks plus Extras")]
    ChecksExtras,

    /// Create default project files/folders if absent. Copies to clipboard.
    #[command(
        name = "create-defaults",
        visible_aliases = ["Create Defaults?", "create-defaults?"]
    )]
    CreateDefaults,

    /// Print a directory tree. Default depth=2. Copies to clipboard.
    Tree {
        #[arg(long, default_value_t = 2)]
        depth: usize,
    },

    /// Print all .rs/.md/.sh/.toml files. Copies to clipboard.
    Files,

    /// Run `checks` (skip extras), then `files`, then `tree`. One clipboard copy with all sections.
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
        CommandKind::Checks => {
            util::maybe_clear(cli.clear);
            let (ok, blob) = run_checks::run_checks(false).await;
            println!("{blob}");
            util::copy_report("checks", &blob);
            if !ok {
                eprintln!("Some checks failed.");
                exit_code = 1;
            }
        }
        CommandKind::ChecksExtras => {
            util::maybe_clear(cli.clear);
            let (ok, blob) = run_checks::run_checks(true).await;
            println!("{blob}");
            util::copy_report("checks-extras", &blob);
            if !ok {
                eprintln!("Some checks failed.");
                exit_code = 1;
            }
        }
        CommandKind::CreateDefaults => {
            util::maybe_clear(cli.clear);
            let out = defaults::ensure_defaults();
            println!("{out}");
            util::copy_report("create-defaults", &out);
        }
        CommandKind::Tree { depth } => {
            util::maybe_clear(cli.clear);
            let blob = tree::collect_tree(depth);
            println!("{blob}");
            util::copy_report("tree", &blob);
        }
        CommandKind::Files => {
            util::maybe_clear(cli.clear);
            let blob = display_all::collect_all_rs();
            print!("{blob}");
            util::copy_report("files", &blob);
        }
        CommandKind::All { depth } => {
            util::maybe_clear(cli.clear);

            // 1) checks
            let (ok, checks_blob) = run_checks::run_checks(false).await;
            println!("{checks_blob}");
            if !ok {
                eprintln!("[all] Checks failed, continuing with files/tree.");
                exit_code = 1;
            }

            // 2) files
            let files_blob = display_all::collect_all_rs();
            print!("{files_blob}");

            // 3) tree
            let tree_blob = tree::collect_tree(depth);
            println!("{tree_blob}");

            // One combined clipboard copy
            let mut all_blob = String::new();
            all_blob.push_str(&checks_blob);
            if !all_blob.ends_with('\n') {
                all_blob.push('\n');
            }
            all_blob.push_str(&files_blob);
            if !all_blob.ends_with('\n') {
                all_blob.push('\n');
            }
            all_blob.push_str(&tree_blob);

            util::copy_report("all", &all_blob);
        }
    }

    std::process::exit(exit_code as i32);
}
