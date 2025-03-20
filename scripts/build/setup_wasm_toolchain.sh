#!/bin/bash

# Exit on any error
set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

echo "Setting up Rust WASM toolchain for Neo contracts..."

# Install wasm32 target
echo "Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Install wasm-pack if needed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Install wasm-opt if needed
if ! command -v wasm-opt &> /dev/null; then
    if command -v npm &> /dev/null; then
        echo "Installing wasm-opt via binaryen..."
        npm install -g binaryen
    elif command -v brew &> /dev/null; then
        echo "Installing binaryen via Homebrew..."
        brew install binaryen
    elif command -v apt-get &> /dev/null; then
        echo "Installing binaryen via apt..."
        sudo apt-get update && sudo apt-get install -y binaryen
    else
        echo "Warning: Could not install wasm-opt. Please install binaryen manually."
        echo "Visit: https://github.com/WebAssembly/binaryen"
    fi
fi

# Create a simple build script
cat > "${PROJECT_ROOT}/scripts/build/build_wasm.sh" << 'EOF'
#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

TARGET_DIR="${PROJECT_ROOT}/target/wasm32-unknown-unknown/release"
OUTPUT_DIR="${PROJECT_ROOT}/wasm"

# Usage message
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <crate-name> [output-name]"
    echo "Example: $0 my-contract my-contract.wasm"
    exit 1
fi

CRATE_NAME="$1"
OUTPUT_NAME="${2:-${CRATE_NAME}.wasm}"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

echo "Building ${CRATE_NAME} as WASM..."
cargo build --release --target wasm32-unknown-unknown -p "${CRATE_NAME}"

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
EOF

chmod +x "${PROJECT_ROOT}/scripts/build/build_wasm.sh"

echo "WASM toolchain setup complete"
echo "You can build WASM modules using: ./scripts/build/build_wasm.sh <crate-name>"
echo "For example: ./scripts/build/build_wasm.sh neo-contract" 