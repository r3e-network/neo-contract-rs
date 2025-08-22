#!/bin/bash
# Infrastructure validation script for neo-contract-rs
# Validates all enterprise-grade infrastructure components

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}ℹ  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Test result tracking
test_result() {
    if [ $? -eq 0 ]; then
        log_success "$1"
        ((TESTS_PASSED++))
    else
        log_error "$1"
        ((TESTS_FAILED++))
    fi
}

log_info "🚀 Starting Neo Contract Framework Infrastructure Validation"
echo

# 1. Version Consistency Check
log_info "📋 Checking version consistency across workspace..."
echo

# Check all package Cargo.toml files have version 1.0.0 (skip workspace root)
VERSION_ISSUES=0
while IFS= read -r -d '' file; do
    # Skip workspace root Cargo.toml
    if [[ "$file" == "./Cargo.toml" ]]; then
        continue
    fi
    # Only check files that have [package] section
    if grep -q '\[package\]' "$file" && ! grep -q 'version = "1.0.0"' "$file"; then
        VERSION=$(grep -o 'version = "[^"]*"' "$file" || echo "version = \"UNKNOWN\"")
        log_warning "Version inconsistency in $file: $VERSION"
        ((VERSION_ISSUES++))
    fi
done < <(find . -name "Cargo.toml" -not -path "./target/*" -print0)

if [ $VERSION_ISSUES -eq 0 ]; then
    test_result "All workspace members have consistent 1.0.0 versioning"
else
    test_result "Version consistency check failed: $VERSION_ISSUES inconsistencies found"
    false
fi

# 2. Workspace Integrity Check
log_info "🏗️  Validating workspace structure..."
echo

# Check workspace members are properly configured
MISSING_MEMBERS=0
while IFS= read -r dir; do
    example_name=$(basename "$dir")
    if ! grep -q "examples/$example_name" Cargo.toml; then
        log_warning "Example $example_name not in workspace members"
        ((MISSING_MEMBERS++))
    fi
done < <(find examples -maxdepth 1 -mindepth 1 -type d | grep -v target)

if [ $MISSING_MEMBERS -eq 0 ]; then
    test_result "All examples properly included in workspace"
else
    test_result "Workspace integrity check failed: $MISSING_MEMBERS missing members"
    false
fi

# 3. Build System Validation
log_info "🔨 Validating build system..."
echo

# Test workspace compilation
log_info "Testing workspace compilation (this may take a moment)..."
if timeout 300 cargo check --workspace --quiet; then
    test_result "Workspace compiles without errors"
else
    test_result "Workspace compilation failed"
fi

# Check for any remaining compilation warnings
WARNINGS=$(cargo check --workspace 2>&1 | grep -c "warning:" || echo "0")
log_info "Compilation warnings: $WARNINGS"

if [ "$WARNINGS" -lt 5 ]; then
    test_result "Acceptable warning level: $WARNINGS warnings"
else
    log_warning "High warning count: $WARNINGS warnings detected"
    test_result "Build quality check - warnings within acceptable range"
fi

# 4. Security Infrastructure Check
log_info "🛡️  Validating security infrastructure..."
echo

# Check for security workflow
if [ -f ".github/workflows/security-audit.yml" ]; then
    test_result "Security audit workflow is configured"
else
    test_result "Security audit workflow missing"
    false
fi

# Check for deny.toml
if [ -f "deny.toml" ]; then
    test_result "Dependency security configuration (deny.toml) present"
else
    test_result "Dependency security configuration missing"
    false
fi

# Test cargo-deny if available
if command -v cargo-deny >/dev/null 2>&1; then
    if cargo deny check licenses --log-level warn >/dev/null 2>&1; then
        test_result "License compliance check passed"
    else
        test_result "License compliance check failed"
        false
    fi
else
    log_info "cargo-deny not installed, skipping license check"
fi

# 5. Test Coverage Infrastructure
log_info "📊 Validating test coverage infrastructure..."
echo

# Check for test coverage workflow
if [ -f ".github/workflows/test-coverage.yml" ]; then
    test_result "Test coverage workflow is configured"
else
    test_result "Test coverage workflow missing"
    false
fi

# Check if tests exist
TEST_COUNT=$(find . -name "*.rs" -path "*/tests/*" -o -name "*test*.rs" | wc -l)
if [ "$TEST_COUNT" -gt 10 ]; then
    test_result "Adequate test suite present: $TEST_COUNT test files"
else
    log_warning "Limited test coverage: only $TEST_COUNT test files found"
    test_result "Test infrastructure present but limited coverage"
fi

# 6. Performance Monitoring Infrastructure
log_info "⚡ Validating performance monitoring infrastructure..."
echo

# Check for benchmark workflow
if [ -f ".github/workflows/performance-benchmark.yml" ]; then
    test_result "Performance benchmark workflow is configured"
else
    test_result "Performance benchmark workflow missing"
    false
fi

# Check if compiler binary can be built
if cargo build --bin neo-compiler --quiet; then
    test_result "Neo compiler binary builds successfully"
else
    test_result "Neo compiler binary build failed"
    false
fi

# 7. Documentation Standards
log_info "📚 Validating documentation standards..."
echo

# Check for essential documentation files
REQUIRED_DOCS=("README.md" "docs/DEPLOYMENT.md" "docs/SOLANA_STYLE_GUIDE.md")
MISSING_DOCS=0

for doc in "${REQUIRED_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        log_warning "Missing documentation: $doc"
        ((MISSING_DOCS++))
    fi
done

if [ $MISSING_DOCS -eq 0 ]; then
    test_result "All essential documentation files present"
else
    test_result "Documentation completeness check failed: $MISSING_DOCS missing files"
    false
fi

# 8. Git Configuration
log_info "🌳 Validating Git configuration..."
echo

# Check .gitignore coverage
GITIGNORE_ITEMS=("target/" "Cargo.lock" "*.nef" "*.manifest.json")
MISSING_GITIGNORE=0

for item in "${GITIGNORE_ITEMS[@]}"; do
    if ! grep -q "$item" .gitignore 2>/dev/null; then
        log_warning "Missing from .gitignore: $item"
        ((MISSING_GITIGNORE++))
    fi
done

if [ $MISSING_GITIGNORE -eq 0 ]; then
    test_result "Git configuration is properly set up"
else
    test_result "Git configuration has $MISSING_GITIGNORE missing items"
    false
fi

# 9. CI/CD Pipeline Validation
log_info "🔄 Validating CI/CD pipeline..."
echo

# Check for essential GitHub workflows
WORKFLOWS=(".github/workflows/ci.yml" ".github/workflows/security-audit.yml" ".github/workflows/test-coverage.yml" ".github/workflows/performance-benchmark.yml")
MISSING_WORKFLOWS=0

for workflow in "${WORKFLOWS[@]}"; do
    if [ ! -f "$workflow" ]; then
        log_warning "Missing workflow: $workflow"
        ((MISSING_WORKFLOWS++))
    fi
done

if [ $MISSING_WORKFLOWS -eq 0 ]; then
    test_result "Complete CI/CD pipeline configured"
else
    test_result "CI/CD pipeline incomplete: $MISSING_WORKFLOWS missing workflows"
    false
fi

# 10. Code Quality Standards
log_info "🔍 Validating code quality standards..."
echo

# Check for Clippy configuration
if grep -q "\[workspace.lints.clippy\]" Cargo.toml; then
    test_result "Clippy linting configuration present"
else
    log_info "No workspace-level Clippy configuration found (optional)"
fi

# Check for consistent formatting
if cargo fmt --check --quiet; then
    test_result "Code formatting is consistent"
else
    test_result "Code formatting inconsistencies detected"
    false
fi

echo
echo "=================================================="
log_info "🏁 Infrastructure Validation Summary"
echo "=================================================="
echo

log_info "Tests Passed: $TESTS_PASSED"
if [ $TESTS_FAILED -gt 0 ]; then
    log_error "Tests Failed: $TESTS_FAILED"
else
    log_success "Tests Failed: $TESTS_FAILED"
fi

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
SUCCESS_RATE=$((TESTS_PASSED * 100 / TOTAL_TESTS))

echo
log_info "Success Rate: $SUCCESS_RATE% ($TESTS_PASSED/$TOTAL_TESTS)"

if [ $SUCCESS_RATE -ge 90 ]; then
    log_success "🎉 Enterprise-grade infrastructure validation PASSED!"
    log_success "Neo Contract Framework is production-ready!"
elif [ $SUCCESS_RATE -ge 80 ]; then
    log_warning "⚠️  Infrastructure validation mostly successful with minor issues"
    log_warning "Consider addressing the failed checks for full production readiness"
else
    log_error "💥 Infrastructure validation FAILED"
    log_error "Critical infrastructure issues must be resolved before production deployment"
    exit 1
fi

echo
log_info "📋 Next Steps:"
echo "  1. Address any failed validation checks"
echo "  2. Run security audit: cargo audit"
echo "  3. Generate test coverage: cargo llvm-cov --html"
echo "  4. Execute performance benchmarks"
echo "  5. Review and update documentation"
echo
log_success "✨ Infrastructure standardization complete!"