#!/bin/bash
set -e

# Usage info
function show_usage {
  echo "Usage: fix-manifest.sh [options] <path/to/project>"
  echo "Options:"
  echo "  -m, --manifest <file> Path to manifest file (default: ./build/<project>.manifest.json)"
  echo "  -s, --source <file>   Path to Rust source file (default: <project>/src/lib.rs)"
  echo "  -a, --asm <file>      Path to .neo.asm file (for offset debugging)"
  echo "  -h, --help            Show this help message"
  exit 1
}

# Parse arguments
MANIFEST_FILE=""
SOURCE_FILE=""
ASM_FILE=""
PROJECT_PATH=""

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -m|--manifest) MANIFEST_FILE="$2"; shift ;;
    -s|--source) SOURCE_FILE="$2"; shift ;;
    -a|--asm) ASM_FILE="$2"; shift ;;
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
NEOWASM_BIN="${NEOWASM_DIR}/neo-wasm"

if [[ ! -f "$NEOWASM_BIN" ]]; then
  echo "Building neo-wasm..."
  (cd "$NEOWASM_DIR" && go build)
fi

# Create a backup of the original manifest
echo "Creating backup of original manifest..."
cp "$MANIFEST_FILE" "${MANIFEST_FILE}.original"

# Phase 1: Use the fix-manifest command to parse Rust source and update metadata
echo "Phase 1: Updating manifest metadata from Rust source..."
"$NEOWASM_BIN" fix-manifest --manifest "$MANIFEST_FILE" --source "$SOURCE_FILE"

# Phase 2: Verify method offsets if ASM file is provided
if [[ -n "$ASM_FILE" && -f "$ASM_FILE" ]]; then
  echo "Phase 2: Verifying method offsets using ASM file..."
  
  # Extract method offsets from ASM file
  echo "Extracting method offsets from ASM file..."
  TEMP_FILE=$(mktemp)
  trap "rm -f $TEMP_FILE" EXIT
  
  # Parse ASM file to extract method offsets
  # Format: // methodName (index):
  #           000: INSTRUCTION
  grep -A 1 "^// " "$ASM_FILE" | grep -v "wasm" | sed -n 'N;s/\/\/ \([^ ]*\) .*\n *\([0-9]*\):.*/\1 \2/p' > "$TEMP_FILE"
  
  # Update the manifest with extracted offsets
  while read -r method offset; do
    if [[ -n "$method" && -n "$offset" ]]; then
      echo "  Method: $method, Offset: $offset"
      
      # Update the manifest using jq if available
      if command -v jq &> /dev/null; then
        # Use jq to update the offset (more reliable)
        jq --arg method "$method" --argjson offset "$offset" '
          .abi.methods = (.abi.methods | map(
            if .name == $method then .offset = $offset | . else . end
          ))
        ' "$MANIFEST_FILE" > "${MANIFEST_FILE}.tmp" && mv "${MANIFEST_FILE}.tmp" "$MANIFEST_FILE"
      else
        # Fallback to sed if jq is not available
        sed -i.tmp -E "s/(\"name\":[[:space:]]*\"$method\"[[:space:]]*,[[:space:]]*[^{}]+\"offset\":[[:space:]]*)[0-9]+/\1$offset/g" "$MANIFEST_FILE"
        rm -f "${MANIFEST_FILE}.tmp"
      fi
    fi
  done < "$TEMP_FILE"
  
  echo "Method offsets updated based on ASM file."
fi

# Phase 3: Apply method name mappings from neo-contract.yaml if it exists
MAPPING_FILE="${PROJECT_DIR}/neo-contract.yaml"
if [[ -f "$MAPPING_FILE" ]]; then
  echo "Phase 3: Applying method name mappings from $MAPPING_FILE..."
  
  # First backup current manifest with correct offsets
  cp "$MANIFEST_FILE" "${MANIFEST_FILE}.with_offsets"
  
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
        echo "  Mapping WASM method '$wasm_name' to Rust method '$rust_name'"
        
        # Get offset for wasm_name
        offset=$(grep -A 5 "\"name\":[[:space:]]*\"$wasm_name\"" "$MANIFEST_FILE" | grep "\"offset\":" | sed -E 's/.*"offset":([0-9]+).*/\1/')
        
        if [[ -n "$offset" ]]; then
          echo "    Found offset for '$wasm_name': $offset"
          
          # Replace name but preserve offset
          if command -v jq &> /dev/null; then
            # Use jq for more reliable replacement
            jq --arg wasm "$wasm_name" --arg rust "$rust_name" --argjson offset "$offset" '
              .abi.methods = (.abi.methods | map(
                if .name == $wasm then .name = $rust | .offset = $offset | . else . end
              ))
            ' "$MANIFEST_FILE" > "${MANIFEST_FILE}.tmp" && mv "${MANIFEST_FILE}.tmp" "$MANIFEST_FILE"
          else
            # Fallback to sed if jq is not available
            sed -i '' "s/\"name\":[[:space:]]*\"$wasm_name\"/\"name\": \"$rust_name\"/" "$MANIFEST_FILE"
          fi
        else
          echo "    WARNING: Could not find offset for method '$wasm_name'"
          sed -i '' "s/\"name\":[[:space:]]*\"$wasm_name\"/\"name\": \"$rust_name\"/" "$MANIFEST_FILE"
        fi
      fi
    fi
  done < "$MAPPING_FILE"
  
  echo "Method name mappings applied."
else
  echo "No neo-contract.yaml found for method name mappings."
fi

echo "Manifest updated successfully!"
echo "Original manifest backed up to: ${MANIFEST_FILE}.original"

# Display the final method offsets in the manifest
echo "Final manifest method names and offsets:"
grep -A 2 "\"name\":" "$MANIFEST_FILE" | grep -A 1 "\"offset\":" | cat

exit 0 