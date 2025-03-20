#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

TARGET_DIR="${PROJECT_ROOT}/target/wasm32-unknown-unknown/release"
OUTPUT_DIR="${PROJECT_ROOT}/wasm"

# List available crates in the workspace
list_crates() {
    echo "Available crates in the workspace:"
    echo "==================================="
    
    # Extract package names from Cargo.toml files
    EXAMPLE_CRATES=$(find "${PROJECT_ROOT}/examples" -name "Cargo.toml" -exec grep -l 'name = ' {} \; | 
                      xargs grep -h 'name = ' | 
                      sed 's/name = "\(.*\)"/\1/' | 
                      sed "s/name = '\(.*\)'/\1/" |
                      sort)
    
    echo "Example contracts:"
    echo "$EXAMPLE_CRATES" | sed 's/^/  - /'
    echo ""
    
    echo "Core crates:"
    echo "  - neo-contract"
    echo "  - neo-macros"
    echo "  - neo-macros-core"
    echo "  - neo-compiler"
    echo ""
}

# Usage message
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <crate-name> [output-name]"
    echo "Example: $0 hello_world my-contract.wasm"
    echo ""
    list_crates
    exit 1
fi

# Get the actual crate name - handle both directory paths and package names
if [[ "$1" == */* ]]; then
    # It's a path, extract just the crate name
    CRATE_PATH="$1"
    if [ -f "${CRATE_PATH}/Cargo.toml" ]; then
        CRATE_NAME=$(grep -m 1 "name" "${CRATE_PATH}/Cargo.toml" | cut -d '"' -f 2 || 
                     grep -m 1 "name" "${CRATE_PATH}/Cargo.toml" | cut -d "'" -f 2)
        if [ -z "$CRATE_NAME" ]; then
            echo "Error: Could not extract crate name from ${CRATE_PATH}/Cargo.toml"
            exit 1
        fi
        echo "Building crate '$CRATE_NAME' from path '$CRATE_PATH'"
    else
        echo "Error: No Cargo.toml found at ${CRATE_PATH}"
        list_crates
        exit 1
    fi
else
    # It's already a crate name
    CRATE_NAME="$1"
    echo "Building crate '$CRATE_NAME'"
fi

OUTPUT_NAME="${2:-${CRATE_NAME}.wasm}"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

echo "Building ${CRATE_NAME} as WASM..."
cargo build --release --target wasm32-unknown-unknown -p "${CRATE_NAME}" || {
    echo "Error: Failed to build ${CRATE_NAME}"
    list_crates
    exit 1
}

# Copy and optimize WASM file if wasm-opt is available
if [ -f "${TARGET_DIR}/${CRATE_NAME}.wasm" ]; then
    cp "${TARGET_DIR}/${CRATE_NAME}.wasm" "${OUTPUT_DIR}/${OUTPUT_NAME}"
    echo "WASM file copied to ${OUTPUT_DIR}/${OUTPUT_NAME}"
    
    if command -v wasm-opt &> /dev/null; then
        echo "Optimizing WASM..."
        wasm-opt -Oz "${OUTPUT_DIR}/${OUTPUT_NAME}" -o "${OUTPUT_DIR}/${OUTPUT_NAME}"
        echo "WASM optimized"
    else
        echo "wasm-opt not found, skipping optimization"
    fi
else
    echo "Error: WASM file not found at ${TARGET_DIR}/${CRATE_NAME}.wasm"
    exit 1
fi

# Check file size
FILE_SIZE=$(wc -c < "${OUTPUT_DIR}/${OUTPUT_NAME}")
echo "WASM file size: ${FILE_SIZE} bytes"

echo "Build completed successfully"
