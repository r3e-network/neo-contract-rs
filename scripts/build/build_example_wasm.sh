#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

TARGET_DIR="${PROJECT_ROOT}/target/wasm32-unknown-unknown/release"
OUTPUT_DIR="${PROJECT_ROOT}/wasm"

# List available examples
list_examples() {
    echo "Available example contracts:"
    echo "============================"
    
    find "${PROJECT_ROOT}/examples" -type d -depth 1 | while read -r dir; do
        if [ -f "${dir}/Cargo.toml" ]; then
            NAME=$(grep -m 1 "name" "${dir}/Cargo.toml" | sed 's/name = "\(.*\)"/\1/' | sed "s/name = '\(.*\)'/\1/")
            echo "  - $(basename "$dir") (crate: $NAME)"
        fi
    done
    echo ""
}

# Usage message
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <example-name> [output-name]"
    echo "Example: $0 hello_world my-contract.wasm"
    echo ""
    list_examples
    exit 1
fi

# Get the example name
EXAMPLE_NAME="$1"
EXAMPLE_DIR="${PROJECT_ROOT}/examples/${EXAMPLE_NAME}"

if [ ! -d "$EXAMPLE_DIR" ]; then
    echo "Error: Example directory not found: $EXAMPLE_DIR"
    list_examples
    exit 1
fi

if [ ! -f "${EXAMPLE_DIR}/Cargo.toml" ]; then
    echo "Error: No Cargo.toml found in $EXAMPLE_DIR"
    exit 1
fi

# Extract the crate name from Cargo.toml
CRATE_NAME=$(grep -m 1 "name" "${EXAMPLE_DIR}/Cargo.toml" | sed 's/name = "\(.*\)"/\1/' | sed "s/name = '\(.*\)'/\1/")
if [ -z "$CRATE_NAME" ]; then
    echo "Error: Could not extract crate name from ${EXAMPLE_DIR}/Cargo.toml"
    exit 1
fi

OUTPUT_NAME="${2:-${CRATE_NAME}.wasm}"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

echo "Building example '${EXAMPLE_NAME}' (crate: ${CRATE_NAME}) as WASM..."

# Create a temporary Cargo config that removes the dependency on neo-contract
TEMP_DIR=$(mktemp -d)
TEMP_CARGO="${TEMP_DIR}/Cargo.toml"

# Copy the original Cargo.toml
cp "${EXAMPLE_DIR}/Cargo.toml" "${TEMP_CARGO}"

# Make clean example crate to avoid neo-contract dependencies
echo "Creating standalone example crate..."
mkdir -p "${TEMP_DIR}/src"
cp -r "${EXAMPLE_DIR}/src"/* "${TEMP_DIR}/src/"

# Modify the Cargo.toml to use wasm-bindgen instead of neo-contract
cat > "${TEMP_CARGO}" << EOF
[package]
name = "${CRATE_NAME}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
EOF

cd "${TEMP_DIR}" || exit 1

# Create a dummy lib.rs if none exists
if [ ! -f "${TEMP_DIR}/src/lib.rs" ]; then
    echo "Creating minimal lib.rs..."
    cat > "${TEMP_DIR}/src/lib.rs" << EOF
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "Hello, world!".to_string()
}
EOF
fi

# Build the example
echo "Building standalone example..."
cargo build --release --target wasm32-unknown-unknown || {
    echo "Error: Failed to build example"
    rm -rf "${TEMP_DIR}"
    exit 1
}

# Copy the WASM file to output directory
if [ -f "${TEMP_DIR}/target/wasm32-unknown-unknown/release/${CRATE_NAME}.wasm" ]; then
    cp "${TEMP_DIR}/target/wasm32-unknown-unknown/release/${CRATE_NAME}.wasm" "${OUTPUT_DIR}/${OUTPUT_NAME}"
    echo "WASM file copied to ${OUTPUT_DIR}/${OUTPUT_NAME}"
    
    if command -v wasm-opt &> /dev/null; then
        echo "Optimizing WASM..."
        wasm-opt -Oz "${OUTPUT_DIR}/${OUTPUT_NAME}" -o "${OUTPUT_DIR}/${OUTPUT_NAME}"
        echo "WASM optimized"
    else
        echo "wasm-opt not found, skipping optimization"
    fi
else
    echo "Error: WASM file not found in temporary build directory"
    rm -rf "${TEMP_DIR}"
    exit 1
fi

# Clean up
rm -rf "${TEMP_DIR}"

# Check file size
FILE_SIZE=$(wc -c < "${OUTPUT_DIR}/${OUTPUT_NAME}")
echo "WASM file size: ${FILE_SIZE} bytes"

echo "Example build completed successfully!"
echo "Note: This is a simplified build that does not use the neo-contract framework."
echo "It creates a standalone WASM file with wasm-bindgen instead." 