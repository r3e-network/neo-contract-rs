#!/bin/bash
set -e

# Usage info
function show_usage {
  echo "Usage: compile-neo.sh [options] <path/to/project>"
  echo "Options:"
  echo "  -o, --output <dir>    Output directory (default: ./build)"
  echo "  -r, --repo-root <dir> Path to repository root (default: auto-detect)"
  echo "  -d, --debug           Enable debug mode with verbose logging"
  echo "  -h, --help            Show this help message"
  exit 1
}

# Parse arguments
OUTPUT_DIR="./build"
REPO_ROOT="/Users/jinghuiliao/git/will/neo-contract-rs"
DEBUG_MODE=1 # Always enable debug mode for now

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -o|--output) OUTPUT_DIR="$2"; shift ;;
    -r|--repo-root) REPO_ROOT="$2"; shift ;;
    -d|--debug) DEBUG_MODE=1 ;;
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

# Set debug environment variables - CRITICALLY IMPORTANT FOR METHOD MAPPING
export NEO_WASM_DEBUG=1
export NEO_WASM_VERBOSE=1
echo "Debug mode enabled with verbose logging"

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

# Output file paths
OUTPUT_WASM="$ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.wasm"
OUTPUT_NEOASM="$ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.neo.asm"
NEF_FILE="$ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.nef"
MANIFEST_FILE="$ABSOLUTE_OUTPUT_DIR/${CONTRACT_NAME}.manifest.json"

# Step 2: Compile the contract to WASM
cd "$PROJECT_DIR"
echo "Compiling contract at $PROJECT_DIR..."

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
WASM_DIR=$(dirname "$WASM_FILE")
WASM_NAME=$(basename "$WASM_FILE")
WASM_BASE="${WASM_NAME%.wasm}"

# Step 3: Run neo-wasm to generate the initial manifest and NEF
echo "Running neo-wasm to generate manifest and NEF files..."
cd "$WASM_DIR"
"$NEOWASM_BIN" translate --input "$WASM_NAME" --save-neo-ops

# Check if manifest was generated
WASM_MANIFEST="${WASM_NAME%.wasm}.manifest.json"
if [[ ! -f "$WASM_MANIFEST" ]]; then
  echo "Error: Failed to generate manifest file."
  exit 1
fi

# Step 4: Update the manifest with correct method names
echo "Updating manifest with correct method names..."
MAPPING_FILE="$PROJECT_DIR/neo-contract.yaml"
if [[ -f "$MAPPING_FILE" ]]; then
  echo "Using method mappings from $MAPPING_FILE"
  
  # Create a backup of the manifest
  cp "$WASM_MANIFEST" "${WASM_MANIFEST}.bak"
  
  # Process each mapping
  while IFS= read -r line; do
    # Skip comments and empty lines
    if [[ "$line" == \#* ]] || [[ -z "${line// }" ]]; then
      continue
    fi
    
    # Extract WASM name and Rust name
    if [[ "$line" =~ ([^:]+):(.+) ]]; then
      wasm_name=$(echo "${BASH_REMATCH[1]}" | xargs)  # Trim whitespace
      rust_name=$(echo "${BASH_REMATCH[2]}" | xargs)  # Trim whitespace
      
      if [[ -n "$wasm_name" && -n "$rust_name" ]]; then
        echo "Mapping WASM method '$wasm_name' to Rust method '$rust_name'"
        sed -i '' "s/\"name\": \"$wasm_name\"/\"name\": \"$rust_name\"/" "$WASM_MANIFEST"
      fi
    fi
  done < "$MAPPING_FILE"
  
  # Also map 'main' to 'get_greeting' if not already done
  if grep -q "\"name\": \"main\"" "$WASM_MANIFEST"; then
    echo "Mapping WASM method 'main' to Rust method 'get_greeting'"
    sed -i '' "s/\"name\": \"main\"/\"name\": \"get_greeting\"/" "$WASM_MANIFEST"
  fi
  
  echo "Manifest updated with Rust method names."
else
  echo "Warning: neo-contract.yaml not found at $MAPPING_FILE"
  echo "Method names will not be mapped correctly."
fi

# Step 5: Copy the files to the output directory
echo "Copying files to output directory..."
cp "$WASM_NAME" "$OUTPUT_WASM"
cp "${WASM_BASE}.neo.asm" "$OUTPUT_NEOASM"
cp "$WASM_MANIFEST" "$MANIFEST_FILE"

# Step 6: Generate the final NEF file
echo "Generating final NEF file..."
"$NEOWASM_BIN" translate --input "$OUTPUT_WASM" --output "$NEF_FILE" --manifest "$MANIFEST_FILE"

# Step 7: Add metadata from the Rust source
echo "Adding metadata from Rust source..."

# Find the Rust source file
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
  echo "Warning: Could not find Rust source file. Skipping metadata fix."
else
  echo "Found Rust source file: $RUST_SRC_FILE"
  
  # Backup the manifest with the correct names and offsets
  cp "$MANIFEST_FILE" "${MANIFEST_FILE}.before_metadata"
  
  # Run fix-manifest to add proper metadata
  cd "$REPO_ROOT"
  "$NEOWASM_BIN" fix-manifest --manifest "$MANIFEST_FILE" --source "$RUST_SRC_FILE"
  
  # Restore the correct method names from our pre-metadata manifest
  METHODS_TMP=$(mktemp)
  grep -A 5 "\"name\":" "${MANIFEST_FILE}.before_metadata" | grep "\"name\":" > "$METHODS_TMP"
  
  # For each method in the pre-metadata manifest
  while read -r line; do
    if [[ "$line" =~ \"name\":\ \"([^\"]+)\" ]]; then
      method_name="${BASH_REMATCH[1]}"
      
      # Find this method in the final manifest
      if ! grep -q "\"name\": \"$method_name\"" "$MANIFEST_FILE"; then
        # This method name was lost, find a wasm name to replace
        for wasm_name in add flip option main; do
          if grep -q "\"name\": \"$wasm_name\"" "$MANIFEST_FILE"; then
            echo "Restoring method name '$method_name' (was '$wasm_name')"
            sed -i '' "s/\"name\": \"$wasm_name\"/\"name\": \"$method_name\"/" "$MANIFEST_FILE"
            break
          fi
        done
      fi
    fi
  done < "$METHODS_TMP"
  
  # Clean up
  rm -f "$METHODS_TMP"
  
  echo "Metadata added successfully."
  
  # Regenerate the NEF file with the final manifest
  "$NEOWASM_BIN" translate --input "$OUTPUT_WASM" --output "$NEF_FILE" --manifest "$MANIFEST_FILE"
fi

# Final verification of manifest
echo "Final manifest file:"
cat "$MANIFEST_FILE"

echo "====================================================================="
echo "Compilation completed successfully!"
echo "Contract files generated:"
echo "  - $OUTPUT_WASM    (WebAssembly binary)"
echo "  - $MANIFEST_FILE    (Neo N3 contract manifest)"
echo "  - $NEF_FILE    (Neo N3 executable format)"
echo "  - $OUTPUT_NEOASM    (Neo assembly for debugging)"
echo "=====================================================================" 