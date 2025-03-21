#!/bin/bash
set -e

# This is a modified version of compile-neo.sh that includes a fix for method offsets

# Set up debug log file for neo-wasm
NEOWASM_LOG="neo-wasm/neo-wasm.log"
echo "Setting up debug log for neo-wasm at $NEOWASM_LOG"
export NEO_WASM_DEBUG=1
rm -f "$NEOWASM_LOG"

# Run the standard compilation script
./compile-neo.sh "$@"

# Extract the output directory and contract name from arguments
OUTPUT_DIR="./build"
PROJECT_PATH=""

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -o|--output) OUTPUT_DIR="$2"; shift ;;
    -r|--repo-root) shift ;; # Ignore repo root for this script
    -h|--help) exit 0 ;;
    *) 
      if [[ -z "$PROJECT_PATH" ]]; then
        PROJECT_PATH="$1"
      fi
      ;;
  esac
  shift
done

# Check if PROJECT_PATH is provided
if [[ -z "$PROJECT_PATH" ]]; then
  echo "Error: Path to project is required"
  exit 1
fi

# Get full path to project directory
PROJECT_DIR=$(realpath "$PROJECT_PATH")

# Contract name is based on directory name
CONTRACT_NAME=$(basename "$PROJECT_DIR")
echo "Using directory name as contract name: $CONTRACT_NAME"

# Get the absolute output directory
ABSOLUTE_OUTPUT_DIR=$(realpath "$OUTPUT_DIR")

# Manifest and ASM file paths
MANIFEST_FILE="${ABSOLUTE_OUTPUT_DIR}/${CONTRACT_NAME}.manifest.json"
ASM_FILE="${ABSOLUTE_OUTPUT_DIR}/${CONTRACT_NAME}.neo.asm"

# Step 5: Fix method offsets in the manifest
echo "Fixing method offsets in manifest..."
if [[ -f "$MANIFEST_FILE" ]]; then
  if [[ -f "$ASM_FILE" ]]; then
    ./fix-offsets.sh -a "$ASM_FILE" "$MANIFEST_FILE"
  else
    ./fix-offsets.sh "$MANIFEST_FILE"
  fi
  echo "Method offsets have been fixed in the manifest."
else
  echo "Error: Manifest file not found at $MANIFEST_FILE"
  exit 1
fi

# Verify the offsets after fixing
echo "Checking final manifest offsets..."
cat "$MANIFEST_FILE" | grep -A 1 '"name":' | grep -A 1 '"offset":'

echo "===================================================================="
echo "Compilation completed with fixed method offsets!"
echo "Contract files generated:"
echo "  - $ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.wasm    (WebAssembly binary)"
echo "  - $MANIFEST_FILE    (Neo N3 contract manifest with fixed offsets)"
echo "  - $ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.nef    (Neo N3 executable format)"
echo "  - $ASM_FILE    (Neo assembly for debugging)"
echo "====================================================================" 