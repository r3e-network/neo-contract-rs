#!/bin/bash
# Script for building all example contracts
# This script finds all examples and builds them using the build_contract.sh script
# Ensures compilation to both WASM and Neo NEF files

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/../examples"
BUILD_SCRIPT="$SCRIPT_DIR/build_contract.sh"

# Make the build script executable
chmod +x "$BUILD_SCRIPT"

# Check if neo-compiler is available
if command -v neo-compiler &> /dev/null; then
  NEO_COMPILER="neo-compiler"
elif [ -f "$SCRIPT_DIR/../target/release/neo-compiler" ]; then
  NEO_COMPILER="$SCRIPT_DIR/../target/release/neo-compiler"
else
  echo "⚠️ Warning: neo-compiler not found! Building the compiler first..."
  # Build the neo-compiler
  (cd "$SCRIPT_DIR/.." && cargo build --release -p neo-compiler)
  NEO_COMPILER="$SCRIPT_DIR/../target/release/neo-compiler"
  
  if [ ! -f "$NEO_COMPILER" ]; then
    echo "❌ Error: Failed to build neo-compiler!"
    exit 1
  fi
  
  echo "✅ Successfully built neo-compiler"
fi

# Find all example directories that contain a Cargo.toml file
echo "Finding examples..."
EXAMPLES=()
while IFS= read -r -d '' dir; do
  if [ -f "$dir/Cargo.toml" ]; then
    rel_path=$(realpath --relative-to="$SCRIPT_DIR/.." "$dir")
    EXAMPLES+=("$rel_path")
  fi
done < <(find "$EXAMPLES_DIR" -type d -print0)

# Count of successful and failed builds
SUCCESS_COUNT=0
WASM_ONLY_COUNT=0
FAILED_COUNT=0
FAILED_EXAMPLES=()
WASM_ONLY_EXAMPLES=()

# Build each example
echo "Found ${#EXAMPLES[@]} examples to build."
echo "=========================================="

for example in "${EXAMPLES[@]}"; do
  echo "Building example: $example"
  
  if "$BUILD_SCRIPT" "$example"; then
    # Check if NEF file was generated
    CONTRACT_NAME=$(basename "$example")
    BUILD_DIR="$SCRIPT_DIR/../$example/build"
    NEF_FILE="$BUILD_DIR/$CONTRACT_NAME.nef"
    MANIFEST_FILE="$BUILD_DIR/$CONTRACT_NAME.manifest.json"
    WASM_FILE="$SCRIPT_DIR/../$example/target/wasm32-unknown-unknown/release/$CONTRACT_NAME.wasm"
    
    if [ -f "$NEF_FILE" ] && [ -f "$MANIFEST_FILE" ]; then
      echo "✅ Success: $example (WASM + NEF)"
      echo "   - WASM: $(ls -lh "$WASM_FILE" | awk '{print $5}')"
      echo "   - NEF: $(ls -lh "$NEF_FILE" | awk '{print $5}')"
      ((SUCCESS_COUNT++))
    elif [ -f "$WASM_FILE" ]; then
      echo "⚠️ Partial Success: $example (WASM only)"
      echo "   Attempting direct NEF compilation..."
      
      # Try to compile directly with neo-compiler
      mkdir -p "$BUILD_DIR"
      if $NEO_COMPILER compile "$WASM_FILE" --output "$BUILD_DIR" --name "$CONTRACT_NAME" --force; then
        echo "✅ Success: $example (NEF compilation recovered)"
        ((SUCCESS_COUNT++))
      else
        echo "⚠️ Warning: NEF compilation failed for $example"
        WASM_ONLY_EXAMPLES+=("$example")
        ((WASM_ONLY_COUNT++))
      fi
    else
      echo "❓ Strange state: No WASM or NEF files found but build script reported success"
      FAILED_EXAMPLES+=("$example")
      ((FAILED_COUNT++))
    fi
  else
    echo "❌ Failed: $example"
    FAILED_EXAMPLES+=("$example")
    ((FAILED_COUNT++))
  fi
  
  echo "------------------------------------------"
done

# Print summary
echo "=========================================="
echo "Build Summary:"
echo "  ✅ Complete Success (WASM + NEF): $SUCCESS_COUNT"
if [ "$WASM_ONLY_COUNT" -gt 0 ]; then
  echo "  ⚠️ Partial Success (WASM only): $WASM_ONLY_COUNT"
  echo "     Examples: ${WASM_ONLY_EXAMPLES[*]}"
fi
echo "  ❌ Failed: $FAILED_COUNT"

if [ "$FAILED_COUNT" -gt 0 ]; then
  echo "     Failed examples: ${FAILED_EXAMPLES[*]}"
  exit 1
fi

if [ "$WASM_ONLY_COUNT" -gt 0 ]; then
  echo ""
  echo "⚠️ Warning: Some examples only compiled to WASM but not to NEF."
  echo "   This may be due to incompatibility with the Neo VM or issues with the neo-compiler."
  exit 2
fi

echo ""
echo "✅ All examples successfully compiled to both WASM and NEF format!"
exit 0
