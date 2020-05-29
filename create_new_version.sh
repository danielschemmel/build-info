#!/bin/bash

set -e
set -u
set -o pipefail

exec cargo workspaces version --all --exact --no-individual-tags --force \*
