#!/bin/bash

# Define color codes
CYAN='\033[0;36m'  # Cyan color
RED='\033[0;31m'   # Red color for errors
GREEN='\033[0;32m' # Green color for success
NC='\033[0m'       # No Color (reset)

# Clear the CLI first
clear
clear
echo "..."
echo "Clearing the deck ..."
echo "..."

# Exit immediately if a command exits with a non-zero status.
set -e

# Define directories to ignore during rustfmt
IGNORED_DIRS="target"

# Function to check if a directory is ignored
should_ignore() {
  for ignored in $IGNORED_DIRS; do
    if [[ "$1" == *"$ignored"* ]]; then
      return 0
    fi
  done
  return 1
}

# Run rustfmt on all Rust files, specifying the edition
echo -e "${CYAN}Running rustfmt to format Rust files...${NC}"
find . -name "*.rs" | while read -r file; do
  if should_ignore "$file"; then
    echo -e "${CYAN}Skipping $file (ignored directory)${NC}"
  else
    echo -e "${CYAN}Formatting $file${NC}"
    rustfmt --edition 2021 "$file" || {
      echo -e "${RED}Formatting failed for $file!${NC}"; exit 1;
    }
  fi
done
echo -e "${GREEN}All files formatted successfully!${NC}"
echo "..."

# Run clippy for linting
echo -e "${CYAN}Running clippy for lint checks...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings; then
  echo -e "${GREEN}Clippy checks passed with no warnings!${NC}"
else
  echo -e "${RED}Clippy checks failed! Please review the warnings/errors above.${NC}"; exit 1;
fi
echo "..."

# Run cargo check for type checking and basic checks
echo -e "${CYAN}Running cargo check for type and syntax validation...${NC}"
if cargo check; then
  echo -e "${GREEN}Cargo check passed successfully!${NC}"
else
  echo -e "${RED}Cargo check failed! Please review the errors above.${NC}"; exit 1;
fi
echo "..."

# Run cargo test to ensure tests pass (if any)
echo -e "${CYAN}Running cargo test to verify code functionality...${NC}"
if cargo test; then
  echo -e "${GREEN}All tests passed successfully!${NC}"
else
  echo -e "${RED}Some tests failed! Please review the test output above.${NC}"; exit 1;
fi
echo "..."

# Provide a final completion message
echo -e "${GREEN}All checks passed successfully! Ready to proceed!${NC}"
# End of script%  

