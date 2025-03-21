#!/bin/bash

# Test script to verify method offset calculation
# Usage: ./verify_method_offsets.sh

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Method Offset Verification Test${NC}"
echo "==============================="

# Step 1: Compile an example contract
echo -e "\n${YELLOW}Step 1: Compiling example contract...${NC}"
cd "$(dirname "$0")/../"
EXAMPLE_DIR="examples/hello_world"

if [ ! -d "$EXAMPLE_DIR" ]; then
  echo -e "${RED}Error: Example directory $EXAMPLE_DIR not found!${NC}"
  exit 1
fi

# Compile the contract
echo "Compiling contract in $EXAMPLE_DIR..."
cd $EXAMPLE_DIR
cargo build --target=wasm32-unknown-unknown --release
cd -

WASM_FILE="$EXAMPLE_DIR/target/wasm32-unknown-unknown/release/hello_world.wasm"
if [ ! -f "$WASM_FILE" ]; then
  echo -e "${RED}Error: WASM file $WASM_FILE not found!${NC}"
  exit 1
fi

# Step 2: Compile to NEF
echo -e "\n${YELLOW}Step 2: Converting WASM to NEF...${NC}"
./neo-wasm/neo-wasm -wasm $WASM_FILE -save-neoasm

# Check if manifest was created
MANIFEST_FILE="${WASM_FILE%.wasm}.manifest.json"
if [ ! -f "$MANIFEST_FILE" ]; then
  echo -e "${RED}Error: Manifest file $MANIFEST_FILE not found!${NC}"
  exit 1
fi

# Step 3: Verify method offsets
echo -e "\n${YELLOW}Step 3: Verifying method offsets...${NC}"

# Check the manifest for method entries
METHOD_COUNT=$(grep -o '"name":' "$MANIFEST_FILE" | wc -l)
OFFSET_COUNT=$(grep -o '"offset":' "$MANIFEST_FILE" | wc -l)

echo "Found $METHOD_COUNT methods and $OFFSET_COUNT offset entries in manifest"

if [ "$METHOD_COUNT" -ne "$OFFSET_COUNT" ]; then
  echo -e "${RED}Error: Method count ($METHOD_COUNT) doesn't match offset count ($OFFSET_COUNT)!${NC}"
  exit 1
fi

# Check if any method has offset 0 (which might indicate an error)
ZERO_OFFSETS=$(grep -o '"offset": *0' "$MANIFEST_FILE" | wc -l)
if [ "$ZERO_OFFSETS" -gt 0 ]; then
  echo -e "${YELLOW}Warning: Found $ZERO_OFFSETS methods with offset 0!${NC}"
  echo "This might be correct for some methods, but verify it's intentional."
fi

# Check NEO assembly file for method addresses
NEO_ASM_FILE="${WASM_FILE%.wasm}.neo.asm"
if [ -f "$NEO_ASM_FILE" ]; then
  echo -e "\nMethod start positions from NEO assembly:"
  grep -n "^// " "$NEO_ASM_FILE" | grep -v "// $WASM_FILE" | sort -n | sed 's/^/  /'
else
  echo -e "${YELLOW}Warning: NEO assembly file $NEO_ASM_FILE not found.${NC}"
fi

echo -e "\n${GREEN}Method offset verification completed successfully!${NC}"
echo "Manifest file: $MANIFEST_FILE"

# Print all methods and their offsets
echo -e "\n${YELLOW}Methods and their offsets:${NC}"
grep -A 1 '"name":' "$MANIFEST_FILE" | grep -v '"parameters":' | sed 's/--//'

exit 0 