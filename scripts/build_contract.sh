#!/bin/bash
# Script for building Neo smart contracts from Rust source
# Usage: ./build_contract.sh <path-to-contract-directory>
# Compiles Rust source to WASM and then converts to Neo NEF format

set -e

if [ $# -lt 1 ]; then
  echo "Usage: $0 <path-to-contract-directory>"
  exit 1
fi

CONTRACT_DIR=$1
CONTRACT_NAME=$(basename "$CONTRACT_DIR")
BUILD_DIR="$CONTRACT_DIR/build"
WASM_TARGET="wasm32-unknown-unknown"

echo "Building Neo N3 contract: $CONTRACT_NAME"
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

# Find neo-compiler
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if command -v neo-compiler &> /dev/null; then
  NEO_COMPILER="neo-compiler"
elif [ -f "$SCRIPT_DIR/../target/release/neo-compiler" ]; then
  NEO_COMPILER="$SCRIPT_DIR/../target/release/neo-compiler"
else
  echo "Warning: neo-compiler not found in PATH or workspace target directory"
  echo "Attempting to build neo-compiler..."
  (cd "$SCRIPT_DIR/.." && cargo build --release -p neo-compiler)
  NEO_COMPILER="$SCRIPT_DIR/../target/release/neo-compiler"
  
  if [ ! -f "$NEO_COMPILER" ]; then
    echo "Error: Failed to build neo-compiler"
    exit 1
  fi
fi

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
  echo "   For smaller contracts, install wasm-opt: npm install -g wasm-opt"
fi

# Optional: Strip WASM file if wasm-strip is available
if command -v wasm-strip &> /dev/null; then
  echo "3. Stripping WebAssembly debug info..."
  wasm-strip "$WASM_FILE"
else
  echo "3. Skipping WASM stripping (wasm-strip not found)"
  echo "   For smaller contracts, install wasm-strip: apt install wabt or brew install wabt"
fi

echo "4. Converting to Neo N3 smart contract..."
# Capture the original file size
WASM_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
echo "   WASM file size: $WASM_SIZE"

# Delete existing NEF/manifest files to avoid confusion
rm -f "$BUILD_DIR/$CONTRACT_NAME.nef" "$BUILD_DIR/$CONTRACT_NAME.manifest.json"

# Use the neo-compiler to compile WASM to NEF
$NEO_COMPILER compile "$WASM_FILE" --output "$BUILD_DIR" --name "$CONTRACT_NAME" --force

if [ $? -ne 0 ]; then
  echo "Error: Failed to compile WASM to Neo N3 smart contract"
  exit 1
fi

NEF_FILE="$BUILD_DIR/$CONTRACT_NAME.nef"
MANIFEST_FILE="$BUILD_DIR/$CONTRACT_NAME.manifest.json"

if [ ! -f "$NEF_FILE" ] || [ ! -f "$MANIFEST_FILE" ]; then
  echo "Error: Expected output files not found!"
  exit 1
fi

# Verify the NEF file
echo "5. Verifying NEF file integrity..."
if $NEO_COMPILER info "$NEF_FILE" | grep -q "Checksum verification: OK"; then
  echo "   NEF file checksum verification passed"
else
  echo "   NEF file checksum verification failed"
  echo "   This may indicate a problem with the neo-compiler or the contract"
fi

# Get file sizes
NEF_SIZE=$(ls -lh "$NEF_FILE" | awk '{print $5}')
MANIFEST_SIZE=$(ls -lh "$MANIFEST_FILE" | awk '{print $5}')

echo "----------------------------------------"
echo "Contract successfully compiled!"
echo "   WASM:     $WASM_FILE ($WASM_SIZE)"
echo "   NEF:      $NEF_FILE ($NEF_SIZE)"
echo "   Manifest: $MANIFEST_FILE ($MANIFEST_SIZE)"
echo "----------------------------------------"
echo "Contract information:"
$NEO_COMPILER info "$NEF_FILE" | grep -v "NEF File:" | sed 's/^/   /'
echo "----------------------------------------"
echo "To deploy using Neo CLI:"
echo "   neo-cli deploy $NEF_FILE $MANIFEST_FILE"
echo "----------------------------------------"