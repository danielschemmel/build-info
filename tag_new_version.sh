#!/bin/bash

set -e
set -u
set -o pipefail

cargo clippy -- -D warnings
cargo test
cargo test --all-features
git diff --exit-code  # check if unstaged changes exist
git diff --cached --exit-code  # check if staged, uncommitted changes exist
for x in build-info build-info-build build-info-common build-info-proc ; do
	pushd $x
	cargo msrv verify
	popd
done
exec cargo workspaces version --all --exact --no-individual-tags --allow-branch main --force \*
