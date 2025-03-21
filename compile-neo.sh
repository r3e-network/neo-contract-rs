#!/bin/bash
set -e

# Usage info
function show_usage {
  echo "Usage: compile-neo.sh [options] <path/to/project>"
  echo "Options:"
  echo "  -o, --output <dir>    Output directory (default: ./build)"
  echo "  -r, --repo-root <dir> Path to repository root (default: auto-detect)"
  echo "  -h, --help            Show this help message"
  exit 1
}

# Parse arguments
OUTPUT_DIR="./build"
REPO_ROOT="/Users/jinghuiliao/git/will/neo-contract-rs"

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -o|--output) OUTPUT_DIR="$2"; shift ;;
    -r|--repo-root) REPO_ROOT="$2"; shift ;;
    -h|--help) show_usage ;;
    *) 
      if [[ -z "$PROJECT_PATH" ]]; then
        PROJECT_PATH="$1"
      else
        echo "Unknown parameter: $1"
        show_usage
      fi
      ;;
  esac
  shift
done

# Check if PROJECT_PATH is provided
if [[ -z "$PROJECT_PATH" ]]; then
  echo "Error: Path to project is required"
  show_usage
fi

# Get full path to project directory
PROJECT_DIR=$(realpath "$PROJECT_PATH")

# Check if directory exists
if [[ ! -d "$PROJECT_DIR" ]]; then
  echo "Error: Project directory not found: $PROJECT_DIR"
  exit 1
fi

# Contract name is based on directory name
CONTRACT_NAME=$(basename "$PROJECT_DIR")
echo "Using directory name as contract name: $CONTRACT_NAME"

# Create output directory
mkdir -p "$OUTPUT_DIR"
ABSOLUTE_OUTPUT_DIR=$(realpath "$OUTPUT_DIR")
echo "Output directory: $ABSOLUTE_OUTPUT_DIR"

# Auto-detect repository root if not provided
if [[ -z "$REPO_ROOT" ]]; then
  # Try to find repository root by looking for neo-wasm directory
  CURRENT_DIR="$PROJECT_DIR"
  while [[ "$CURRENT_DIR" != "/" ]]; do
    if [[ -d "$CURRENT_DIR/neo-wasm" ]]; then
      REPO_ROOT="$CURRENT_DIR"
      break
    fi
    CURRENT_DIR=$(dirname "$CURRENT_DIR")
  done
  
  if [[ -z "$REPO_ROOT" ]]; then
    echo "Error: Could not auto-detect repository root."
    echo "Please specify with --repo-root option."
    exit 1
  else
    echo "Repository root detected at: $REPO_ROOT"
  fi
fi

# Check if neo-wasm directory exists
NEOWASM_DIR="/Users/jinghuiliao/git/will/neo-contract-rs/neo-wasm"
if [[ ! -d "$NEOWASM_DIR" ]]; then
  echo "Error: neo-wasm directory not found at $NEOWASM_DIR"
  exit 1
fi

# Step 1: Build neo-wasm compiler
echo "Building neo-wasm compiler..."
cd "$NEOWASM_DIR"

# Check if Go is installed
if ! command -v go &> /dev/null; then
  echo "Error: go command not found. Go is required to build neo-wasm."
  exit 1
fi

# Build neo-wasm
go build -o neo-wasm .

if [[ ! -f "neo-wasm" ]]; then
  echo "Error: Failed to build neo-wasm"
  exit 1
fi

echo "Successfully built neo-wasm"
NEOWASM_BIN="$NEOWASM_DIR/neo-wasm"

# Step 2: Compile the contract to WASM
echo "Compiling contract at $PROJECT_DIR..."
cd "$PROJECT_DIR"

# Check if Makefile exists
if [[ ! -f "Makefile" ]]; then
  echo "Error: Makefile not found in project directory"
  exit 1
fi

# Run make to compile the contract
make

# Find the WASM file
# Common locations to check for WASM files
POTENTIAL_PATHS=(
  "$PROJECT_DIR/target/wasm32-unknown-unknown/release/$CONTRACT_NAME.wasm"
  "$PROJECT_DIR/target/wasm32-unknown-unknown/debug/$CONTRACT_NAME.wasm"
  "$PROJECT_DIR/$CONTRACT_NAME.wasm"
  "$REPO_ROOT/neo-wasm/examples/hello-world/hello-world.wasm"
)

WASM_FILE=""
for path in "${POTENTIAL_PATHS[@]}"; do
  if [[ -f "$path" ]]; then
    WASM_FILE="$path"
    break
  fi
done

if [[ -z "$WASM_FILE" ]]; then
  echo "Error: Could not find WASM file after compilation."
  echo "Please check the Makefile output."
  exit 1
fi

echo "Found WASM file: $WASM_FILE"

# Step 3: Use neo-wasm to translate WASM to NEF
echo "Translating WASM to NEF..."

# Output file paths
NEF_FILE="$ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.nef"
MANIFEST_FILE="$ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.manifest.json"

# Run neo-wasm translation
"$NEOWASM_BIN" translate --input "$WASM_FILE" --output "$NEF_FILE" --save-neo-ops

# Step 4: Fix the manifest file by analyzing the Rust source code
echo "Fixing manifest file with source code analysis..."

# Find the Rust source file (typically lib.rs in src directory)
RUST_SRC_FILE=""
POTENTIAL_SRC_PATHS=(
  "$PROJECT_DIR/src/lib.rs"
  "$PROJECT_DIR/lib.rs"
)

for path in "${POTENTIAL_SRC_PATHS[@]}"; do
  if [[ -f "$path" ]]; then
    RUST_SRC_FILE="$path"
    break
  fi
done

if [[ -z "$RUST_SRC_FILE" ]]; then
  echo "Warning: Could not find Rust source file. Skipping manifest fix."
else
  echo "Found Rust source file: $RUST_SRC_FILE"
  
  # Run the fix-manifest command
  cd "$REPO_ROOT"
  "$NEOWASM_BIN" fix-manifest --manifest "$MANIFEST_FILE" --source "$RUST_SRC_FILE"
  
  echo "Manifest file has been fixed."
fi

# Verify the manifest was created
if [[ ! -f "$MANIFEST_FILE" ]]; then
  echo "Error: Manifest file was not generated."
  echo "Translation process failed. Check neo-wasm output for errors."
  exit 1
fi

# Copy the WASM file to output directory if it's not already there
if [[ "$(dirname "$WASM_FILE")" != "$ABSOLUTE_OUTPUT_DIR" ]]; then
  cp "$WASM_FILE" "$ABSOLUTE_OUTPUT_DIR/$(basename "$WASM_FILE")"
fi

echo "====================================================================="
echo "Compilation completed successfully!"
echo "Contract files generated:"
echo "  - $ABSOLUTE_OUTPUT_DIR/$(basename "$WASM_FILE")    (WebAssembly binary)"
echo "  - $MANIFEST_FILE    (Neo N3 contract manifest)"
echo "  - $NEF_FILE    (Neo N3 executable format)"
echo "  - $ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.neo.asm    (Neo assembly for debugging)"
echo "=====================================================================" 