#!/bin/bash
set -e

# This script bridges the gap between WASM method names and Rust method names
# Usage: ./bridge-methods.sh <path/to/manifest.json>

MANIFEST_FILE=$1
if [[ -z "$MANIFEST_FILE" ]]; then
  echo "Error: Path to manifest file is required"
  exit 1
fi

if [[ ! -f "$MANIFEST_FILE" ]]; then
  echo "Error: Manifest file not found: $MANIFEST_FILE"
  exit 1
fi

echo "Bridging WASM method offsets to Rust method names in $MANIFEST_FILE"

# Create a backup of the manifest
cp "$MANIFEST_FILE" "${MANIFEST_FILE}.bridge.bak"

# The known method mappings from WASM export names to Rust method names
# Format: "wasm_name:rust_name"
METHOD_MAPPINGS=(
  "add:hello"
  "flip:contract_info"
  "option:store_greeting" 
  "main:get_greeting"
)

# Read the current method offsets from the manifest
TEMP_FILE=$(mktemp)
trap "rm -f $TEMP_FILE" EXIT

# Extract the offsets for each WASM method from logs
echo "Extracting method offsets from logs..."
OFFSETS_FILE="neo-wasm/neo-wasm.log"
if [[ ! -f "$OFFSETS_FILE" ]]; then
  echo "Error: Offsets file not found. Run compilation with NEO_WASM_DEBUG=1"
  exit 1
fi

# Find all method offsets in the log file
OFFSETS=$(grep "Stored offset for method" "$OFFSETS_FILE" | sed -E "s/.*'([^']+)':[[:space:]]+([0-9]+).*/\1:\2/g")

echo "Found method offsets:"
echo "$OFFSETS"

# For each Rust method in the manifest, find the corresponding WASM method and update its offset
for mapping in "${METHOD_MAPPINGS[@]}"; do
  WASM_NAME=$(echo "$mapping" | cut -d':' -f1)
  RUST_NAME=$(echo "$mapping" | cut -d':' -f2)
  
  # Get the offset for this WASM method
  OFFSET=$(echo "$OFFSETS" | grep "^${WASM_NAME}:" | cut -d':' -f2)
  
  if [[ -n "$OFFSET" ]]; then
    echo "Mapping WASM method '$WASM_NAME' (offset $OFFSET) to Rust method '$RUST_NAME'"
    
    # Update the manifest file using sed to replace the offset for the corresponding Rust method
    sed -i '' -E "s/(\"name\":[[:space:]]*\"${RUST_NAME}\"[[:space:]]*,[[:space:]]*\"parameters\".*\"offset\":[[:space:]]*)[0-9]+/\1${OFFSET}/g" "$MANIFEST_FILE"
  else
    echo "Warning: No offset found for WASM method '$WASM_NAME'"
  fi
done

echo "Manifest updated with bridged method offsets."
echo "Backup saved at ${MANIFEST_FILE}.bridge.bak" 