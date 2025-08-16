# Run Checks - Rust CLI Tool

## Overview

`run_checks` is a Rust CLI tool to automate common development tasks such as formatting, linting, type checking, and testing.  
It also includes utilities for inspecting source files and directory structures.  

The tool runs once and exits — no interactive menus or persistent TUI.

## Features

- **Code Formatting**: Run `rustfmt` across the codebase.
- **Linting**: Run `clippy` with `-D warnings` for strict linting.
- **Type Checking**: Validate syntax and types via `cargo check`.
- **Testing**: Run all unit tests with `cargo test`.
- **Directory Tree Display**: Print a project tree up to a configurable depth.
- **Source File Viewer**: Print the contents of all `.rs` files in `src`.

## Usage

### 1. Prerequisites
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

### 2. Clone the Repository
```bash
git clone https://github.com/John-Beeping-Doe/run_checks.git
cd run_checks
````

### 3. Build and Run

Run with `cargo run` and pass a subcommand:

```bash
cargo run -- <command> [options]
```

### 4. Available Commands

* `checks` → run `rustfmt`, `clippy`, `cargo check`, and `cargo test`. Print summary table.
* `files` → print the contents of all `.rs` files in `src/`.
* `tree [--depth N]` → print directory tree (default depth 2).
* `all [--depth N]` → run all checks, then show files and tree.

### 5. Global Options

* `--clear` → clear the terminal before printing each section.
* `-h, --help` → show help with examples.
* `-V, --version` → show version.

### Examples

Run all checks:

```bash
cargo run -- checks
```

Run everything (checks + source files + directory tree, depth 3, clearing output):

```bash
cargo run -- all --depth 3 --clear
```

View project tree only:

```bash
cargo run -- tree --depth 1
```

## Example Output

```
┌─────────────────────┬───────────┬───────────────┐
│ Tool                │ Status    │ Time Elapsed  │
╞═════════════════════╪═══════════╪═══════════════╡
│ rustfmt             │ Success   │ 0.113 seconds │
│ clippy              │ Success   │ 0.238 seconds │
│ cargo check         │ Success   │ 0.130 seconds │
│ cargo test          │ Success   │ 0.388 seconds │
├─────────────────────┼───────────┼───────────────┤
│ Total time elapsed: │           │ 0.391 seconds │
└─────────────────────┴───────────┴───────────────┘
```

## File Structure

```
.
├── Cargo.toml
├── README.md
└── src
    ├── display_all.rs   # Print Rust source files
    ├── main.rs          # CLI entrypoint
    ├── run_checks.rs    # Formatting, linting, type checking, tests
    └── tree.rs          # Directory tree printer
```

## Contributing

Contributions are welcome. Fork the repository, make changes, and open a PR.

## License

MIT License


