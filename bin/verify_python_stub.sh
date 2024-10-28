#!/bin/bash

# The purpose of this script is to verify that python stub generation has been performed
# before merge. So this script will generate new python stubs and if the file has changed
# will fail

(cd ./rust; cargo run --bin stub_gen)

# Check if any files have been modified
if git diff-index --quiet HEAD --; then
    echo "No changes detected."
else
    echo "File(s) have been modified. Exiting."
    exit 1
fi