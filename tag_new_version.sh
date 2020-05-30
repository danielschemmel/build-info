#!/bin/bash

set -e
set -u
set -o pipefail

cargo clippy -- -D warnings
exec cargo workspaces version --all --exact --no-individual-tags --force \*
