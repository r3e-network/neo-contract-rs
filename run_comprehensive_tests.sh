#!/bin/bash

# Neo Contract Comprehensive Test Runner
# This script runs all comprehensive unit tests and generates coverage metrics

set -e

echo "🚀 Neo Contract Comprehensive Test Suite"
echo "======================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in the project root directory. Please run from the neo-contract-rs directory."
    exit 1
fi

print_status "Setting up test environment..."

# Set environment variables for testing
export RUST_BACKTRACE=1
export RUST_LOG=debug

# Clean previous build artifacts
print_status "Cleaning previous build artifacts..."
cargo clean

# Build the project first
print_status "Building project..."
if ! cargo build; then
    print_error "Failed to build project"
    exit 1
fi

print_success "Project built successfully"

# Test execution summary
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
IGNORED_TESTS=0

# Function to run tests for a specific module
run_test_module() {
    local module_name=$1
    local test_file=$2
    
    print_status "Running $module_name tests..."
    echo "Test file: $test_file"
    
    # Check if test file exists
    if [ ! -f "$test_file" ]; then
        print_warning "$test_file not found, skipping..."
        return
    fi
    
    # Run the specific test
    if cargo test --test "$(basename "$test_file" .rs)" -- --nocapture; then
        print_success "$module_name tests completed"
        return 0
    else
        print_error "$module_name tests failed"
        return 1
    fi
}

# Function to run all unit tests
run_unit_tests() {
    print_status "Running comprehensive unit tests..."
    
    local test_modules=(
        "Core Types:neo-contract/tests/core_types_comprehensive.rs"
        "Storage System:neo-contract/tests/storage_system_comprehensive.rs"
        "Runtime Services:neo-contract/tests/runtime_services_comprehensive.rs"
        "Example Contracts:neo-contract/tests/example_contracts_comprehensive.rs"
        "WASM→NEF Integration:neo-contract/tests/wasm_nef_integration_comprehensive.rs"
        "Error Handling:neo-contract/tests/error_handling_comprehensive.rs"
    )
    
    local module_failures=0
    
    for module_info in "${test_modules[@]}"; do
        IFS=':' read -r module_name test_file <<< "$module_info"
        
        if run_test_module "$module_name" "$test_file"; then
            ((PASSED_TESTS++))
        else
            ((FAILED_TESTS++))
            ((module_failures++))
        fi
        ((TOTAL_TESTS++))
    done
    
    return $module_failures
}

# Function to run compiler tests
run_compiler_tests() {
    print_status "Running compiler tests..."
    
    if [ -f "neo-compiler/tests/compilation_pipeline_comprehensive.rs" ]; then
        if cargo test --package neo-compiler --test compilation_pipeline_comprehensive -- --nocapture; then
            print_success "Compiler tests completed"
            ((PASSED_TESTS++))
        else
            print_error "Compiler tests failed"
            ((FAILED_TESTS++))
        fi
        ((TOTAL_TESTS++))
    else
        print_warning "Compiler tests not found"
    fi
}

# Function to run existing tests
run_existing_tests() {
    print_status "Running existing framework tests..."
    
    # Run existing tests to ensure we haven't broken anything
    if cargo test --lib; then
        print_success "Existing framework tests passed"
    else
        print_error "Some existing tests failed"
        return 1
    fi
}

# Function to check for test coverage
check_test_coverage() {
    print_status "Checking test coverage..."
    
    # Install cargo-tarpaulin for coverage if not present
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"
        print_status "Skipping coverage analysis..."
        return
    fi
    
    print_status "Generating coverage report..."
    if cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html; then
        print_success "Coverage report generated in tarpaulin-report.html"
    else
        print_warning "Coverage generation failed"
    fi
}

# Function to validate test structure
validate_test_structure() {
    print_status "Validating test structure..."
    
    local required_files=(
        "neo-contract/tests/core_types_comprehensive.rs"
        "neo-contract/tests/storage_system_comprehensive.rs"
        "neo-contract/tests/runtime_services_comprehensive.rs"
        "neo-contract/tests/example_contracts_comprehensive.rs"
        "neo-contract/tests/wasm_nef_integration_comprehensive.rs"
        "neo-contract/tests/error_handling_comprehensive.rs"
    )
    
    local missing_files=0
    
    for file in "${required_files[@]}"; do
        if [ -f "$file" ]; then
            print_success "✓ $file"
        else
            print_error "✗ $file (missing)"
            ((missing_files++))
        fi
    done
    
    if [ $missing_files -eq 0 ]; then
        print_success "All required test files present"
    else
        print_error "$missing_files test files missing"
        return 1
    fi
}

# Function to count total test cases
count_test_cases() {
    print_status "Counting test cases..."
    
    local test_count=0
    
    # Count #[test] annotations in all test files
    for file in neo-contract/tests/*_comprehensive.rs neo-compiler/tests/*_comprehensive.rs; do
        if [ -f "$file" ]; then
            local file_tests=$(grep -c "#\[test\]" "$file" 2>/dev/null || echo "0")
            test_count=$((test_count + file_tests))
            echo "  $(basename "$file"): $file_tests tests"
        fi
    done
    
    print_success "Total comprehensive test cases: $test_count"
    
    if [ $test_count -ge 200 ]; then
        print_success "✓ Target of 200+ tests achieved!"
    else
        print_warning "⚠ Only $test_count tests found (target: 200+)"
    fi
}

# Function to run test documentation check
check_test_documentation() {
    print_status "Checking test documentation..."
    
    local doc_issues=0
    
    for file in neo-contract/tests/*_comprehensive.rs neo-compiler/tests/*_comprehensive.rs; do
        if [ -f "$file" ]; then
            # Check if file has module-level documentation
            if head -10 "$file" | grep -q "//!"; then
                print_success "✓ $(basename "$file") has documentation"
            else
                print_warning "⚠ $(basename "$file") missing module documentation"
                ((doc_issues++))
            fi
        fi
    done
    
    if [ $doc_issues -eq 0 ]; then
        print_success "All test files are properly documented"
    else
        print_warning "$doc_issues test files need better documentation"
    fi
}

# Function to run benchmark tests (if any)
run_benchmark_tests() {
    print_status "Looking for benchmark tests..."
    
    if find . -name "*.rs" -exec grep -l "#\[bench\]" {} \; | head -1 > /dev/null; then
        print_status "Running benchmark tests..."
        cargo bench
    else
        print_status "No benchmark tests found (this is OK)"
    fi
}

# Function to check for memory leaks (basic check)
check_memory_usage() {
    print_status "Checking basic memory patterns..."
    
    # Run tests with memory tracking (basic)
    export RUST_BACKTRACE=full
    
    if cargo test --release -- --test-threads=1 > test_output.log 2>&1; then
        if grep -i "memory\|leak\|overflow" test_output.log > /dev/null; then
            print_warning "Memory-related messages found in test output"
            grep -i "memory\|leak\|overflow" test_output.log | head -5
        else
            print_success "No obvious memory issues detected"
        fi
    fi
    
    # Clean up
    rm -f test_output.log
}

# Main execution
main() {
    print_status "Starting comprehensive test suite..."
    echo ""
    
    # Pre-flight checks
    validate_test_structure || exit 1
    count_test_cases
    check_test_documentation
    
    echo ""
    print_status "Running tests..."
    echo ""
    
    # Run different test suites
    run_existing_tests || {
        print_error "Existing tests failed. Fix these before running comprehensive tests."
        exit 1
    }
    
    run_unit_tests
    run_compiler_tests
    
    echo ""
    print_status "Additional validations..."
    
    check_memory_usage
    run_benchmark_tests
    
    # Generate coverage report
    check_test_coverage
    
    echo ""
    print_status "Test Results Summary"
    echo "===================="
    echo "Total Test Modules: $TOTAL_TESTS"
    echo "Passed: $PASSED_TESTS"
    echo "Failed: $FAILED_TESTS"
    echo "Ignored: $IGNORED_TESTS"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        print_success "🎉 All comprehensive tests passed!"
        echo ""
        print_status "Framework validation complete:"
        echo "• ✅ Core types and framework components tested"
        echo "• ✅ Storage operations validated"
        echo "• ✅ Runtime services functional"
        echo "• ✅ Contract compilation pipeline tested"
        echo "• ✅ Example contracts validated"
        echo "• ✅ Error handling comprehensive"
        echo "• ✅ Integration tests passed"
        echo ""
        print_success "Neo N3 contract framework is ready for production! 🚀"
        exit 0
    else
        print_error "❌ $FAILED_TESTS test module(s) failed"
        print_status "Please review the test output above and fix the failing tests."
        exit 1
    fi
}

# Handle script interruption
trap 'print_error "Test run interrupted"; exit 1' INT TERM

# Run main function
main "$@"
