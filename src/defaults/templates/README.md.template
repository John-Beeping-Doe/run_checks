````markdown
# Run Checks - Rust CLI Tool

## Overview
`run_checks` is a Rust-based CLI tool that automates common development checks and reports results in a clean table format.  
It is designed to quickly validate code quality, security, and project health before commits or releases.

## Features
- **Formatting**: Runs `rustfmt` on all `.rs` files.  
- **Linting**: Executes `cargo clippy` with strict warnings.  
- **Type Checking**: Validates types and compilation using `cargo check`.  
- **Testing**: Runs project unit tests with `cargo test`.  
- **File & Directory Tools**: List source files and show project structure.  
- **Security & Privacy Checks**:
  - Scans for secrets, keys, tokens, and credentials in source/config.
  - Detects unignored sensitive files (`.env`, `.pem`, `.kube/config`, etc.).  

## Installation
Clone the repository and build the binary:

```bash
git clone https://github.com/John-Beeping-Doe/run_checks.git
cd run_checks
cargo build --release
````

The binary will be available at:

```
target/release/run_checks
```

For convenience, copy it to your Desktop or PATH:

```bash
cp target/release/run_checks ~/Desktop/
```

## Usage

Run with a subcommand:

```bash
./run_checks <COMMAND>
```

### Available Commands

| Command  | Description                                          |
| -------- | ---------------------------------------------------- |
| `format` | Run `rustfmt` on all project files                   |
| `lint`   | Run `cargo clippy` with strict checks                |
| `check`  | Run `cargo check` type validation                    |
| `test`   | Run all project tests (`cargo test`)                 |
| `files`  | List tracked project files                           |
| `tree`   | Display project directory tree                       |
| `checks` | Run all checks (format, lint, check, test, security) |

### Examples

**Run all checks:**

```bash
./run_checks checks
```

**Run only linting:**

```bash
./run_checks lint
```

**Check for leaked secrets and sensitive files:**

```bash
./run_checks checks
```

**List all source files in the project:**

```bash
./run_checks files
```

**Display directory tree:**

```bash
./run_checks tree
```

## Security/Privacy Scan

The tool runs secret-detection scans using `ripgrep` patterns:

* Common credential keywords: `api`, `secret`, `token`, `key`, `password`, etc.
* AWS, GitHub, Slack token formats.
* PEM/SSH key blocks.

It also checks for risky files:

```
.env*
.envrc
.aws/
kubeconfig
id_rsa*
id_ed25519*
*.pem
*.p12
*.crt
*.key
*.kube/config
```

Results are displayed in a table as a single **PASS/FAIL** line with file+line details if issues are found.

## Example Output

```
+-----------------------+---------+-------------------------+---------------+
| Check                 | Status  | Details                 | Time Elapsed  |
+-----------------------+---------+-------------------------+---------------+
| Format (rustfmt)      | PASS    | All files formatted     | 0.45s         |
| Lint (clippy)         | PASS    | No warnings             | 1.22s         |
| Type Check (cargo)    | PASS    | Build check successful  | 0.80s         |
| Tests (cargo test)    | PASS    | 12 passed               | 2.10s         |
| Security/Privacy      | FAIL    | src/config.rs:12: token | 0.60s         |
+-----------------------+---------+-------------------------+---------------+
```

## Contributing

Contributions are welcome. To contribute:

1. **Fork the repository** and create your branch from `main`.
2. **Make your changes** with clear, concise commits.
3. **Add or update tests** to cover your changes.
4. **Run `./run_checks checks`** locally to ensure all checks pass.
5. **Open a Pull Request (PR)** describing your changes and their purpose.
6. **Link related issues** in the PR description if applicable.

### Issues

* Use the **GitHub Issues** tab to report bugs or suggest features.
* When filing a bug, include:

  * OS and Rust version
  * Steps to reproduce
  * Expected vs. actual behavior
* For feature requests, describe the use case and potential implementation ideas.

## License

This project is licensed under the [MIT License](LICENSE).

---

**Repository**: [John-Beeping-Doe/run\_checks](https://github.com/John-Beeping-Doe/run_checks)

```

Do you also want me to include a **"Development"** section showing how to run the project directly with `cargo run` instead of the compiled binary, or is that unnecessary for your intended audience?
```
