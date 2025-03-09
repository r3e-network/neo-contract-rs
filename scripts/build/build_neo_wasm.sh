#!/bin/bash

# Exit on any error
set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"
TOOLS_DIR="$PROJECT_ROOT/tools/neo-wasm"
BIN_DIR="$PROJECT_ROOT/bin"

# Create bin directory if it doesn't exist
mkdir -p "$BIN_DIR"

# Build neo-wasm
echo "Building neo-wasm..."
cd "$TOOLS_DIR"

# Initialize Go modules if go.mod doesn't exist
if [ ! -f go.mod ]; then
    echo "Initializing Go modules..."
    go mod init github.com/neo-project/neo-contract-rs/tools/neo-wasm
    go mod tidy
fi

# Build the tool
go build -o "$BIN_DIR/neo-wasm" ./cmd

# Make the executable executable
chmod +x "$BIN_DIR/neo-wasm"

echo "neo-wasm has been built and placed in $BIN_DIR/neo-wasm"
