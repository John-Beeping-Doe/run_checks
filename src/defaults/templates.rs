// File: src/defaults/templates.rs

// Templates live in src/defaults/templates/*.template

pub const DEFAULT_GITIGNORE: &str = include_str!("./templates/.gitignore.template");
pub const DEFAULT_RUSTFMT: &str = include_str!("./templates/rustfmt.toml.template");
pub const DEFAULT_SCRIPT: &str = include_str!("./templates/run_checks.sh.template");
pub const DEFAULT_LICENSE: &str = include_str!("./templates/LICENSE.template");
pub const DEFAULT_README: &str = include_str!("./templates/README.md.template");
pub const DEFAULT_CHANGELOG: &str = include_str!("./templates/CHANGELOG.md.template");
pub const DEFAULT_CONTRIBUTING: &str = include_str!("./templates/CONTRIBUTING.md.template");
pub const DEFAULT_EDITORCONFIG: &str = include_str!("./templates/.editorconfig.template");
pub const DEFAULT_CARGO_CONFIG: &str = include_str!("./templates/config.toml.template");

// new: starter for src/main.rs
pub const DEFAULT_MAIN_RS: &str = include_str!("./templates/main.rs.template");
