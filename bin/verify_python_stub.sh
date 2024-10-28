#!/bin/bash

# The purpose of this script is to verify that python stub generation has been performed
# before merge. So this script will generate new python stubs and if the file has changed
# will fail

(cd ./rust; cargo run --bin stub_gen)

diff_stat=$(git diff --stat rust/configler-pyo3/configler_pyo3.pyi)
echo "Stub Diff"
echo "${diff_stat}"

# Check if any files have been modified
if [[ $diff_stat == *"changed"* ]]; then
    echo "Changes Detected after generating python type stubs"
    echo "Please run 'make build-python-bindings' to correct this"
    exit 1
else
    echo "All python stubs up to date"
fi