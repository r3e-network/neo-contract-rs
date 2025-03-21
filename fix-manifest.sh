#!/bin/bash
set -e

# Usage info
function show_usage {
  echo "Usage: fix-manifest.sh [options] <path/to/project>"
  echo "Options:"
  echo "  -m, --manifest <file> Path to manifest file (default: ./build/<project>.manifest.json)"
  echo "  -s, --source <file>   Path to Rust source file (default: <project>/src/lib.rs)"
  echo "  -h, --help            Show this help message"
  exit 1
}

# Parse arguments
MANIFEST_FILE=""
SOURCE_FILE=""
PROJECT_PATH=""

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -m|--manifest) MANIFEST_FILE="$2"; shift ;;
    -s|--source) SOURCE_FILE="$2"; shift ;;
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

# Set default manifest and source files if not provided
if [[ -z "$MANIFEST_FILE" ]]; then
  MANIFEST_FILE="./build/${CONTRACT_NAME}.manifest.json"
fi
if [[ -z "$SOURCE_FILE" ]]; then
  SOURCE_FILE="${PROJECT_DIR}/src/lib.rs"
fi

# Check if files exist
if [[ ! -f "$MANIFEST_FILE" ]]; then
  echo "Error: Manifest file not found: $MANIFEST_FILE"
  exit 1
fi
if [[ ! -f "$SOURCE_FILE" ]]; then
  echo "Error: Source file not found: $SOURCE_FILE"
  exit 1
fi

# Get absolute paths
MANIFEST_FILE=$(realpath "$MANIFEST_FILE")
SOURCE_FILE=$(realpath "$SOURCE_FILE")

echo "Manifest file: $MANIFEST_FILE"
echo "Source file: $SOURCE_FILE"

# Build neo-wasm if needed
REPO_ROOT=$(dirname "$(realpath "$0")")
NEOWASM_DIR="${REPO_ROOT}/neo-wasm"
if [[ ! -d "$NEOWASM_DIR" ]]; then
  echo "Error: neo-wasm directory not found at $NEOWASM_DIR"
  exit 1
fi

cd "$NEOWASM_DIR"
if [[ ! -f "neo-wasm" ]]; then
  echo "Building neo-wasm..."
  go build -o neo-wasm .
fi

if [[ ! -f "neo-wasm" ]]; then
  echo "Error: Failed to build neo-wasm"
  exit 1
fi

# Run the fix-manifest command
echo "Fixing manifest file..."
./neo-wasm fix-manifest --manifest "$MANIFEST_FILE" --source "$SOURCE_FILE"

echo "Manifest file has been updated successfully!" 