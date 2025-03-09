#!/bin/bash
# Script for building Neo smart contracts from Rust source
# Usage: ./build_contract.sh <path-to-contract-directory>

set -e

if [ $# -lt 1 ]; then
  echo "Usage: $0 <path-to-contract-directory>"
  exit 1
fi

CONTRACT_DIR=$1
CONTRACT_NAME=$(basename "$CONTRACT_DIR")
BUILD_DIR="$CONTRACT_DIR/build"
WASM_TARGET="wasm32-unknown-unknown"

echo "Building Neo contract: $CONTRACT_NAME"
echo "----------------------------------------"

# Check if the contract directory exists
if [ ! -d "$CONTRACT_DIR" ]; then
  echo "Error: Contract directory '$CONTRACT_DIR' does not exist."
  exit 1
fi

# Make sure we're in the contract directory
cd "$CONTRACT_DIR"

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

echo "1. Compiling Rust to WebAssembly..."
cargo build --release --target "$WASM_TARGET"

WASM_FILE="target/$WASM_TARGET/release/$CONTRACT_NAME.wasm"

if [ ! -f "$WASM_FILE" ]; then
  echo "Error: WASM file was not generated at $WASM_FILE"
  exit 1
fi

# Optional: Optimize WASM file if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
  echo "2. Optimizing WebAssembly..."
  wasm-opt -Oz "$WASM_FILE" -o "$WASM_FILE.opt"
  mv "$WASM_FILE.opt" "$WASM_FILE"
else
  echo "2. Skipping WASM optimization (wasm-opt not found)"
fi

# Optional: Strip WASM file if wasm-strip is available
if command -v wasm-strip &> /dev/null; then
  echo "3. Stripping WebAssembly debug info..."
  wasm-strip "$WASM_FILE"
else
  echo "3. Skipping WASM stripping (wasm-strip not found)"
fi

echo "4. Converting to Neo smart contract..."
# Assuming neo-compiler is in PATH or in the workspace target directory
if command -v neo-compiler &> /dev/null; then
  NEO_COMPILER="neo-compiler"
elif [ -f "../../target/release/neo-compiler" ]; then
  NEO_COMPILER="../../target/release/neo-compiler"
else
  echo "Error: neo-compiler not found in PATH or workspace target/release directory"
  exit 1
fi

$NEO_COMPILER compile "$WASM_FILE" --output "$BUILD_DIR" --name "$CONTRACT_NAME"

if [ $? -ne 0 ]; then
  echo "Error: Failed to compile WASM to Neo smart contract"
  exit 1
fi

NEF_FILE="$BUILD_DIR/$CONTRACT_NAME.nef"
MANIFEST_FILE="$BUILD_DIR/$CONTRACT_NAME.manifest.json"

if [ ! -f "$NEF_FILE" ] || [ ! -f "$MANIFEST_FILE" ]; then
  echo "Error: Expected output files not found!"
  exit 1
fi

echo "----------------------------------------"
echo "Contract successfully compiled!"
echo "NEF file: $NEF_FILE"
echo "Manifest: $MANIFEST_FILE"
echo "----------------------------------------"
echo "To deploy using Neo CLI:"
echo "neo-cli deploy $NEF_FILE $MANIFEST_FILE"
echo "----------------------------------------"