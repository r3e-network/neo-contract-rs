#!/bin/bash
# Make all scripts in the project executable
# This ensures users can run build scripts without permission issues

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Making all scripts executable..."

# Make all .sh files in the scripts directory executable
find "$SCRIPT_DIR" -name "*.sh" -type f -exec chmod +x {} \;
echo "✅ Made $(find "$SCRIPT_DIR" -name "*.sh" -type f | wc -l | xargs) script files executable"

# Make all .sh files in the tools directory executable if it exists
if [ -d "$ROOT_DIR/tools" ]; then
  find "$ROOT_DIR/tools" -name "*.sh" -type f -exec chmod +x {} \;
  echo "✅ Made $(find "$ROOT_DIR/tools" -name "*.sh" -type f | wc -l | xargs) tool scripts executable"
fi

echo "✅ All scripts are now executable"
echo ""
echo "To build all examples to both WASM and NEF format, run:"
echo "  ./scripts/build_examples.sh"
echo ""
echo "To build a specific example, run:"
echo "  ./scripts/build_contract.sh examples/path/to/example"
