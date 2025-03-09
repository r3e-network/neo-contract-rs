#!/bin/bash
# Script to generate a test WebAssembly file
# This script executes the JavaScript generator

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JS_SCRIPT="$SCRIPT_DIR/generate_test_wasm.js"
OUTPUT_PATH="$SCRIPT_DIR/test_contract.wasm"

# Make the script executable
chmod +x "$JS_SCRIPT"

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is required to run the test WASM generator"
    echo "Please install Node.js and try again, or manually create a test WASM file"
    exit 1
fi

# Run the generator
echo "Generating test WebAssembly file..."
node "$JS_SCRIPT" "$OUTPUT_PATH"

# Check if the file was created
if [ -f "$OUTPUT_PATH" ]; then
    echo "Test WASM file successfully created: $OUTPUT_PATH"
    exit 0
else
    echo "Error: Failed to create test WASM file"
    exit 1
fi