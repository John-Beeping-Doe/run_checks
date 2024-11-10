# Run Checks - Rust CLI Tool

## Overview

The `run_checks` project is a Rust CLI tool designed to streamline and automate common development tasks such as formatting, linting, type checking, and testing. It also provides utility functions for viewing project files and directory structures. 

## Features

- **Code Formatting**: Uses `rustfmt` to format all Rust files.
- **Linting**: Runs `clippy` with strict checks to ensure code quality.
- **Type Checking**: Validates types and syntax using `cargo check`.
- **Testing**: Executes unit tests with `cargo test` to verify code functionality.
- **Directory Structure Display**: Shows a tree view of the project directory up to two levels deep.
- **Source File Viewer**: Displays the contents of all `.rs` files in the `src` directory.
- **Interactive CLI**: User-friendly menu-driven interface for accessing features.

## Usage

### 1. Prerequisites
Ensure you have the following installed:
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

### 2. Clone the Repository
```bash
git clone https://github.com/John-Beeping-Doe/run_checks.git
cd run_checks

3. Build and Run

Run the application directly:

cargo run

4. Interactive Menu Options

	•	Check and Run: Executes all checks (formatting, linting, type checking, testing) and runs the project if all checks pass.
	•	Run Checks: Runs all predefined checks individually.
	•	Tree: Displays the directory structure up to two levels deep.
	•	Display All: Displays the contents of all .rs files in the src folder.
	•	Exit: Exits the CLI tool.

File Structure

.
├── Cargo.toml      # Project metadata and dependencies
├── src
│   ├── display_all.rs # Handles displaying Rust source file contents
│   ├── main.rs        # Main entry point for the application
│   ├── run_checks.rs  # Implements formatting, linting, and testing routines
│   ├── tree.rs        # Displays the directory structure

Example Output

Running cargo run:

=== Rust CLI Menu ===
1. Check and Run
2. Run Checks
3. Tree
4. Display All
5. Exit
Choose an option:

Sample Tree Output:

Directory structure (up to 2 levels):
[DIR] ./src
  [FILE] ./src/main.rs
  [FILE] ./src/display_all.rs
  [FILE] ./src/run_checks.rs
  [FILE] ./src/tree.rs

Displaying Rust File Contents:

========================================
src/main.rs
========================================
<Contents of main.rs>

Contributing

Contributions are welcome! Please fork the repository, make changes, and submit a pull request.

License

This project is licensed under the MIT License.

Acknowledgments

Special thanks to the maintainers of Rust and its excellent ecosystem of libraries.


