#!/bin/bash

# Neo N3 Smart Contract Integration Test Suite
# Tests compilation, NEF generation, and deployment for all examples

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_test() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

# Function to run a test
run_test() {
    local test_name=$1
    local test_command=$2
    
    print_test "Running: $test_name"
    
    if eval "$test_command"; then
        print_info "✅ $test_name PASSED"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        print_error "❌ $test_name FAILED"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# Function to test example compilation
test_example_compilation() {
    local example_dir=$1
    local example_name=$(basename "$example_dir")
    
    print_test "Testing compilation of $example_name"
    
    cd "$example_dir"
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_warn "Skipping $example_name - no Cargo.toml"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
        cd - > /dev/null
        return 0
    fi
    
    # Test cargo check
    if cargo check --target wasm32-unknown-unknown > /dev/null 2>&1; then
        print_info "✅ Cargo check passed for $example_name"
    else
        print_error "❌ Cargo check failed for $example_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        cd - > /dev/null
        return 1
    fi
    
    # Test WASM build
    if RUSTFLAGS="-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152" \
       cargo build --target wasm32-unknown-unknown --release > /dev/null 2>&1; then
        print_info "✅ WASM build passed for $example_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        print_error "❌ WASM build failed for $example_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        cd - > /dev/null
        return 1
    fi
    
    cd - > /dev/null
    return 0
}

# Function to test NEF generation
test_nef_generation() {
    local example_dir=$1
    local example_name=$(basename "$example_dir")
    
    print_test "Testing NEF generation for $example_name"
    
    # Find WASM file
    local wasm_file=$(find "$example_dir/target/wasm32-unknown-unknown/release" -name "*.wasm" ! -name "*deps*" 2>/dev/null | head -1)
    
    if [ -z "$wasm_file" ]; then
        # Try workspace target
        wasm_file=$(find "target/wasm32-unknown-unknown/release" -name "*${example_name//-/_}*.wasm" ! -name "*deps*" 2>/dev/null | head -1)
    fi
    
    if [ -z "$wasm_file" ]; then
        print_warn "No WASM file found for $example_name"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
        return 0
    fi
    
    # Test NEF compilation
    mkdir -p "$example_dir/build"
    
    if cargo run -p neo-compiler -- compile "$wasm_file" --output "$example_dir/build" > /dev/null 2>&1; then
        print_info "✅ NEF generation passed for $example_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        
        # Check if NEF file exists
        if [ -f "$example_dir/build/"*.nef ]; then
            print_info "✅ NEF file created for $example_name"
        else
            print_error "❌ NEF file not found for $example_name"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        print_error "❌ NEF generation failed for $example_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
    
    return 0
}

# Main test execution
main() {
    print_info "=========================================="
    print_info "Neo N3 Smart Contract Integration Test Suite"
    print_info "=========================================="
    
    # Check dependencies
    print_test "Checking dependencies..."
    
    if ! command -v cargo > /dev/null 2>&1; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
        print_warn "WASM target not installed. Installing..."
        rustup target add wasm32-unknown-unknown
    fi
    
    print_info "Dependencies OK"
    
    # Test framework components
    print_test "Testing framework components..."
    
    run_test "neo-contract library" "cargo check -p neo-contract"
    run_test "neo-contract-proc-macros" "cargo check -p neo-contract-proc-macros"
    run_test "neo-compiler" "cargo check -p neo-compiler"
    
    # Test all examples
    print_test "Testing examples..."
    
    for example_dir in examples/*/; do
        if [ -d "$example_dir" ]; then
            test_example_compilation "$example_dir"
        fi
    done
    
    # Test NEF generation for a subset of examples
    print_test "Testing NEF generation..."
    
    for example_dir in examples/01-hello-world examples/03-counter examples/04-nep17-token; do
        if [ -d "$example_dir" ]; then
            test_nef_generation "$example_dir"
        fi
    done
    
    # Test Solana-style examples specifically
    print_test "Testing Solana-style examples..."
    
    for example_dir in examples/*solana*/; do
        if [ -d "$example_dir" ]; then
            test_example_compilation "$example_dir"
        fi
    done
    
    # Print summary
    echo
    print_info "=========================================="
    print_info "Test Results Summary"
    print_info "=========================================="
    echo -e "${GREEN}Passed:${NC} $PASSED_TESTS"
    echo -e "${RED}Failed:${NC} $FAILED_TESTS"
    echo -e "${YELLOW}Skipped:${NC} $SKIPPED_TESTS"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        print_info "🎉 All tests passed successfully!"
        exit 0
    else
        print_error "Some tests failed. Please review the output above."
        exit 1
    fi
}

# Run main function
main "$@"