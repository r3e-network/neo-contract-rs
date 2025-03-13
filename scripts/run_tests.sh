#!/bin/bash
# Script to run the entire test suite for neo-contract-rs
# This script generates test fixtures, runs unit tests and integration tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."

echo "Running Neo Contract Framework Test Suite"
echo "========================================"

# Step 1: Generate test fixtures
echo "1. Generating test fixtures"
cd "$PROJECT_ROOT/neo-compiler/tests/fixtures"
bash ./generate_test_wasm.sh
if [ $? -ne 0 ]; then
  echo "❌ Failed to generate test fixtures"
  echo "Continuing with tests..."
else
  echo "✅ Test fixtures generated successfully"
fi

# Step 2: Run unit tests
echo "2. Running unit tests"
cd "$PROJECT_ROOT"
cargo test --lib
if [ $? -ne 0 ]; then
  echo "❌ Unit tests failed"
  echo "Continuing with integration tests..."
else
  echo "✅ Unit tests passed"
fi

# Step 3: Run integration tests
echo "3. Running integration tests"
cd "$PROJECT_ROOT"
cargo test --test '*'
if [ $? -ne 0 ]; then
  echo "❌ Integration tests failed"
  TEST_RESULT=1
else
  echo "✅ Integration tests passed"
  TEST_RESULT=0
fi

# Step 4: Run doc tests
echo "4. Running documentation tests"
cd "$PROJECT_ROOT"
cargo test --doc
if [ $? -ne 0 ]; then
  echo "❌ Documentation tests failed"
  DOC_TEST_RESULT=1
else
  echo "✅ Documentation tests passed"
  DOC_TEST_RESULT=0
fi

echo "========================================"
echo "Test Suite Summary:"

if [ $TEST_RESULT -eq 0 ] && [ $DOC_TEST_RESULT -eq 0 ]; then
  echo "✅ All tests passed!"
  exit 0
else
  echo "❌ Some tests failed!"
  exit 1
fi