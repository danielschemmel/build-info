#!/bin/bash

set -e
set -u
set -o pipefail

cargo clippy -- -D warnings
cargo test --all-features
git diff --exit-code  # check if unstaged changes exist
git diff --cached --exit-code  # check if staged, uncommitted changes exist
exec cargo workspaces version --all --exact --no-individual-tags --force \*
