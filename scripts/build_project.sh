#!/bin/bash
# Script for building the entire neo-contract framework
# This builds the libraries, compiler, and examples

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."

echo "Building Neo Contract Framework"
echo "==============================="

# Step 1: Build the libraries and compiler
echo "1. Building core libraries and compiler"
cd "$PROJECT_ROOT"
cargo build --release
if [ $? -ne 0 ]; then
  echo "❌ Failed to build core libraries and compiler"
  exit 1
fi
echo "✅ Core libraries and compiler built successfully"

# Step 2: Run tests
echo "2. Running tests"
cargo test --release
if [ $? -ne 0 ]; then
  echo "❌ Some tests failed"
  echo "Continuing with build process..."
else
  echo "✅ All tests passed"
fi

# Step 3: Ensure neo-compiler is executable
echo "3. Setting up neo-compiler"
COMPILER_PATH="$PROJECT_ROOT/target/release/neo-compiler"
if [ -f "$COMPILER_PATH" ]; then
  chmod +x "$COMPILER_PATH"
  echo "✅ neo-compiler is ready"
else
  echo "❌ neo-compiler binary not found at $COMPILER_PATH"
  exit 1
fi

# Step 4: Build examples
echo "4. Building examples"
"$SCRIPT_DIR/build_examples.sh"
EXAMPLES_RESULT=$?
if [ $EXAMPLES_RESULT -ne 0 ]; then
  echo "❌ Some examples failed to build"
else
  echo "✅ All examples built successfully"
fi

echo "==============================="
echo "Build process completed!"
echo "Neo compiler located at: $COMPILER_PATH"
echo "You can now use the compiler to build your own contracts:"
echo "  $COMPILER_PATH compile your_contract.wasm"
echo "Or use the build_contract.sh script for a complete build process:"
echo "  $SCRIPT_DIR/build_contract.sh path/to/your/contract"
echo "==============================="

# Return success only if everything built correctly
if [ $EXAMPLES_RESULT -ne 0 ]; then
  exit 1
fi
exit 0