#!/bin/bash
# Script for building all example contracts
# This script finds all examples and builds them using the build_contract.sh script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR/../examples"
BUILD_SCRIPT="$SCRIPT_DIR/build_contract.sh"

# Make the build script executable
chmod +x "$BUILD_SCRIPT"

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
FAILED_COUNT=0
FAILED_EXAMPLES=()

# Build each example
echo "Found ${#EXAMPLES[@]} examples to build."
echo "=========================================="

for example in "${EXAMPLES[@]}"; do
  echo "Building example: $example"
  
  if "$BUILD_SCRIPT" "$example"; then
    echo "✅ Success: $example"
    ((SUCCESS_COUNT++))
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
echo "  ✅ Successful: $SUCCESS_COUNT"
echo "  ❌ Failed: $FAILED_COUNT"

if [ $FAILED_COUNT -gt 0 ]; then
  echo "Failed examples:"
  for failed in "${FAILED_EXAMPLES[@]}"; do
    echo "  - $failed"
  done
  exit 1
else
  echo "All examples built successfully!"
fi
