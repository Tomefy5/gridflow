#!/bin/bash
set -e
echo "Building all GridFlow crates..."
cargo build --workspace
