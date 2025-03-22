#!/bin/bash
set -e

echo "===== Testing Neo Syscall Support ====="

# Set environment variables for debugging and verbose output
export NEO_WASM_DEBUG=1
export NEO_WASM_VERBOSE=1
export GO_DEBUG=1
OUTPUT_DIR="./build"
mkdir -p "$OUTPUT_DIR"

# Compile the syscall-test contract to WASM
echo "Compiling syscall-test contract..."
cargo build --target=wasm32-unknown-unknown --release --package syscall-test

# The WASM file is in the workspace target folder
WASM_FILE="target/wasm32-unknown-unknown/release/syscall_test.wasm"

# Check if the wasm file exists
if [ ! -f "$WASM_FILE" ]; then
    echo "Error: WASM file not found at $WASM_FILE"
    exit 1
fi

# Run neo-wasm translator to generate NEF and manifest files
echo "Running neo-wasm translator..."
./neo-wasm/neo-wasm translate --input "$WASM_FILE" --output "$OUTPUT_DIR/syscall_test.nef" --manifest "$OUTPUT_DIR/syscall_test.manifest.json" --save-neo-ops

# Check the generated NEF file for syscall tokens
echo "Checking NEF content..."
./neo-wasm/neo-wasm dump --input "$OUTPUT_DIR/syscall_test.nef" > "$OUTPUT_DIR/syscall_test.dump.txt"

# Check the generated assembly file for SYSCALL operations
echo "Checking generated assembly for SYSCALL operations..."
ASSEMBLY_FILE="${WASM_FILE%.wasm}.neo.asm"
if [ ! -f "$ASSEMBLY_FILE" ]; then
    echo "Warning: Assembly file not found at $ASSEMBLY_FILE"
else
    echo "Assembly file found, checking for syscalls..."
    grep -i "syscall" "$ASSEMBLY_FILE" > "$OUTPUT_DIR/syscall_test.syscalls.txt" || echo "No syscalls found in assembly"
    
    # Count the number of syscalls
    SYSCALL_COUNT=$(grep -c -i "syscall" "$ASSEMBLY_FILE" || echo 0)
    echo "Found $SYSCALL_COUNT SYSCALL operations in the assembly"
    
    # List all the unique syscalls
    echo "Unique syscalls used:"
    grep -i "syscall" "$ASSEMBLY_FILE" | sort | uniq || echo "No syscalls found"
fi

# Check manifest for method offsets
echo "Checking method offsets in manifest..."
cat "$OUTPUT_DIR/syscall_test.manifest.json" | grep -A 5 "methods" || echo "No methods found in manifest"

# Check binary for syscall tokens
echo "Checking for syscall tokens in NEF file..."
strings "$OUTPUT_DIR/syscall_test.nef" | grep -i "System\." || echo "No syscall tokens found in NEF"

echo "===== Syscall Test Complete ====="