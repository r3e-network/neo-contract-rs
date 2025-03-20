#!/bin/bash

# Exit on any error
set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

TARGET_DIR="${PROJECT_ROOT}/target/wasm32-unknown-unknown/release"
OUTPUT_DIR="${PROJECT_ROOT}/wasm"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

echo "Building Neo WASM contract..."
echo "This is a complete Neo contract built as WASM with our special adaptations."

# First ensure the WASM target is installed
rustup target add wasm32-unknown-unknown

# Build the example
cd "${PROJECT_ROOT}/examples/neo_wasm_contract" || exit 1

# Build the WASM
cargo build --release --target wasm32-unknown-unknown

# Check if the build succeeded
if [ ! -f "${TARGET_DIR}/neo_wasm_contract.wasm" ]; then
    echo "Error: WASM build failed, file not found: ${TARGET_DIR}/neo_wasm_contract.wasm"
    exit 1
fi

# Copy the WASM file to the output directory
cp "${TARGET_DIR}/neo_wasm_contract.wasm" "${OUTPUT_DIR}/neo_wasm_contract.wasm"
echo "WASM file copied to ${OUTPUT_DIR}/neo_wasm_contract.wasm"

# Optimize the WASM file if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
    echo "Optimizing WASM..."
    wasm-opt -Oz "${OUTPUT_DIR}/neo_wasm_contract.wasm" -o "${OUTPUT_DIR}/neo_wasm_contract.wasm"
    echo "WASM optimized"
else
    echo "wasm-opt not found, skipping optimization"
fi

# Create the JS binding if wasm-bindgen is available
if command -v wasm-bindgen &> /dev/null; then
    echo "Creating JS bindings..."
    wasm-bindgen --target web --out-dir "${OUTPUT_DIR}/js" "${TARGET_DIR}/neo_wasm_contract.wasm"
    echo "JS bindings created in ${OUTPUT_DIR}/js"
else
    echo "wasm-bindgen CLI not found, skipping JS binding generation"
    echo "To install: cargo install wasm-bindgen-cli"
fi

# Check file size
FILE_SIZE=$(wc -c < "${OUTPUT_DIR}/neo_wasm_contract.wasm")
echo "WASM file size: ${FILE_SIZE} bytes"

echo "Build completed successfully!"
echo "You can find the WASM file at: ${OUTPUT_DIR}/neo_wasm_contract.wasm"
