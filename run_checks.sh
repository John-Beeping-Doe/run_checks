#!/bin/bash

(

  cargo test --all-features
  cargo clippy --all-targets --all-features -- -D warnings 2>&1
  cargo fmt -- --check
  cargo audit
  cargo deny check

  cat Cargo.toml
  cat *.md
  find src -type f -name "*.rs" -exec cat {} +

) | tee >(pbcopy)

