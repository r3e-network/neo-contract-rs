#!/bin/bash

# Neo Contract RS - Test and Example Runner
# This script runs all unit tests and builds all examples to verify the framework

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script directory and project root
SCRIPT_DIR=$(dirname "$(realpath "$0")")
ROOT_DIR=$(realpath "$SCRIPT_DIR/..")
EXAMPLES_DIR="$ROOT_DIR/examples"
NEO_CONTRACT_DIR="$ROOT_DIR/neo-contract"

# Header
echo -e "${BLUE}=======================================${NC}"
echo -e "${BLUE}Neo Contract RS - Testing Framework${NC}"
echo -e "${BLUE}=======================================${NC}"
echo ""

# Function to log results
log_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

log_failure() {
    echo -e "${RED}✗ $1${NC}"
}

log_info() {
    echo -e "${BLUE}➤ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Function to run all unit tests
run_tests() {
    log_info "Running unit tests..."
    echo ""
    
    # Run tests for the neo-contract library
    cd "$NEO_CONTRACT_DIR"
    
    RUST_BACKTRACE=1 cargo test
    
    if [ $? -eq 0 ]; then
        echo ""
        log_success "All tests passed successfully"
    else
        echo ""
        log_failure "Some tests failed"
        return 1
    fi
    
    return 0
}

# Function to build all examples
build_examples() {
    log_info "Building all examples..."
    echo ""
    
    local failures=0
    local success=0
    
    # Loop through all examples
    for example_dir in "$EXAMPLES_DIR"/*; do
        if [ -d "$example_dir" ]; then
            local example_name=$(basename "$example_dir")
            
            echo -e "${YELLOW}Building example:${NC} $example_name"
            cd "$example_dir"
            
            # Check for Cargo.toml to ensure it's a Rust project
            if [ ! -f "Cargo.toml" ]; then
                log_warning "No Cargo.toml found, skipping"
                continue
            fi
            
            # Build with wasm32 target
            RUSTFLAGS="-C link-arg=-s" cargo build --release --target wasm32-unknown-unknown
            
            if [ $? -eq 0 ]; then
                log_success "Successfully built $example_name"
                ((success++))
            else
                log_failure "Failed to build $example_name"
                ((failures++))
            fi
            
            echo ""
        fi
    done
    
    echo -e "${BLUE}Example build summary:${NC}"
    echo -e "${GREEN}✓ $success examples built successfully${NC}"
    
    if [ $failures -gt 0 ]; then
        echo -e "${RED}✗ $failures examples failed to build${NC}"
        return 1
    fi
    
    return 0
}

# Main execution
log_info "Starting tests and examples verification"
echo ""

# Ensure wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    log_info "Installing wasm32 target..."
    rustup target add wasm32-unknown-unknown
fi

# Run tests
run_tests
tests_result=$?

# Build examples
build_examples
examples_result=$?

# Summary
echo -e "${BLUE}=======================================${NC}"
echo -e "${BLUE}Testing Summary${NC}"
echo -e "${BLUE}=======================================${NC}"

if [ $tests_result -eq 0 ]; then
    log_success "Unit tests: Passed"
else
    log_failure "Unit tests: Failed"
fi

if [ $examples_result -eq 0 ]; then
    log_success "Examples: All built successfully"
else
    log_failure "Examples: Some failed to build"
fi

# Overall result
if [ $tests_result -eq 0 ] && [ $examples_result -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ All tests and examples verified successfully${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}✗ Some tests or examples failed verification${NC}"
    exit 1
fi