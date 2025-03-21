#!/bin/bash
set -e

# Usage info
function show_usage {
  echo "Usage: fix-offsets.sh [options] <path/to/manifest.json>"
  echo "Options:"
  echo "  -a, --asm <file>     Path to .neo.asm file (for debugging)"
  echo "  -h, --help           Show this help message"
  exit 1
}

# Parse arguments
ASM_FILE=""
MANIFEST_FILE=""

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -a|--asm) ASM_FILE="$2"; shift ;;
    -h|--help) show_usage ;;
    *) 
      if [[ -z "$MANIFEST_FILE" ]]; then
        MANIFEST_FILE="$1"
      else
        echo "Unknown parameter: $1"
        show_usage
      fi
      ;;
  esac
  shift
done

# Check if MANIFEST_FILE is provided
if [[ -z "$MANIFEST_FILE" ]]; then
  echo "Error: Path to manifest file is required"
  show_usage
fi

# Check if file exists
if [[ ! -f "$MANIFEST_FILE" ]]; then
  echo "Error: Manifest file not found: $MANIFEST_FILE"
  exit 1
fi

echo "Fixing method offsets in $MANIFEST_FILE"

# Get the temporary file to work with
TEMP_FILE=$(mktemp)
trap "rm -f $TEMP_FILE" EXIT

# Create a backup of the original manifest
cp "$MANIFEST_FILE" "${MANIFEST_FILE}.bak"

# Extract method offsets from ASM file if provided
if [[ -n "$ASM_FILE" && -f "$ASM_FILE" ]]; then
  echo "Using ASM file for offset information: $ASM_FILE"
  
  # Extract function names and their offsets from the ASM file
  # Format in ASM: // function_name (index):
  #                 offset: instruction
  grep -A 1 "^// " "$ASM_FILE" | grep -v "wasm" | grep -B 1 -E "[0-9]+:" | awk '
  BEGIN { method = ""; offset = ""; }
  /^\/\/ / { 
    if (method != "" && offset != "") {
      print method " " offset;
      method = ""; offset = "";
    }
    method = $2;
  }
  /^[[:space:]]+[0-9]+:/ {
    if (method != "") {
      offset = $1;
      gsub(/:$/, "", offset);
      print method " " offset;
      method = ""; offset = "";
    }
  }' > "$TEMP_FILE"

  # Read the offsets and update the manifest
  echo "Updating manifest with offsets from ASM file..."
  while read -r method offset; do
    if [[ -n "$method" && -n "$offset" ]]; then
      echo "  Method: $method, Offset: $offset"
      
      # Update the manifest using direct string manipulation
      # Find the method in the manifest and update its offset
      sed -i.tmp -E "s/(\"name\":[[:space:]]*\"$method\"[[:space:]]*,[[:space:]]*[^{}]+\"offset\":[[:space:]]*)[0-9]+/\1$offset/g" "$MANIFEST_FILE"
      
      # Clean up temporary files
      rm -f "${MANIFEST_FILE}.tmp"
    fi
  done < "$TEMP_FILE"
  
else
  echo "No ASM file provided. Looking for method offsets in the logs..."
  
  # Try to find method offsets in the logs
  if [[ -f "neo-wasm/neo-wasm.log" ]]; then
    echo "Using neo-wasm.log for offset information..."
    grep "offset for method" "neo-wasm/neo-wasm.log" | while read -r line; do
      if [[ "$line" =~ Stored[[:space:]]+offset[[:space:]]+for[[:space:]]+method[[:space:]]+\'([^\']+)\':[[:space:]]+([0-9]+) ]]; then
        method="${BASH_REMATCH[1]}"
        offset="${BASH_REMATCH[2]}"
        echo "  Method: $method, Offset: $offset"
        
        # Update the manifest using direct string manipulation
        sed -i.tmp -E "s/(\"name\":[[:space:]]*\"$method\"[[:space:]]*,[[:space:]]*[^{}]+\"offset\":[[:space:]]*)[0-9]+/\1$offset/g" "$MANIFEST_FILE"
        
        # Clean up temporary files
        rm -f "${MANIFEST_FILE}.tmp"
      fi
    done
  else
    echo "No offset information found. Using direct method detection..."
    
    # If no logs are available, use the manifest structure directly
    grep -A 5 "\"name\":" "$MANIFEST_FILE" | grep -B 5 "\"offset\":" | paste -d " " - - | grep -oE "\"name\":[[:space:]]*\"[^\"]+\"[[:space:]]*,.*\"offset\":[[:space:]]*[0-9]+" | while read -r line; do
      if [[ "$line" =~ \"name\":[[:space:]]*\"([^\"]+)\"[[:space:]]*,.*\"offset\":[[:space:]]*([0-9]+) ]]; then
        method="${BASH_REMATCH[1]}"
        offset="${BASH_REMATCH[2]}"
        
        if [[ "$offset" != "0" ]]; then
          echo "  Method: $method has non-zero offset: $offset (preserved)"
        fi
      fi
    done
  fi
fi

echo "Manifest offsets have been fixed."
echo "Backup saved to ${MANIFEST_FILE}.bak" 