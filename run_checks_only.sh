#!/bin/bash

(
  cargo fmt
  cargo clippy --all-targets --all-features -- -D warnings 2>&1
) | tee >(pbcopy)

