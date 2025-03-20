#!/bin/bash

# Exit on any error
set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

echo "Neo WASM toolchain setup"
echo "------------------------"
echo "This script is now deprecated in favor of setup_wasm_toolchain.sh"
echo "Running setup_wasm_toolchain.sh instead..."
echo ""

# Run the setup_wasm_toolchain.sh script
"${SCRIPT_DIR}/setup_wasm_toolchain.sh"

echo ""
echo "Setup complete. You can now build WASM modules using: ./scripts/build/build_wasm.sh <crate-name>"
echo "For example: ./scripts/build/build_wasm.sh neo-contract"
