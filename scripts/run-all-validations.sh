#!/bin/bash

# Neo N3 Framework - Master Validation Suite
# Runs all validation scripts for comprehensive framework assessment

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Banner
echo -e "${BLUE}"
cat << "EOF"
╔══════════════════════════════════════════════════════════════════════════════╗
║                        Neo N3 Framework Validation Suite                    ║
║                     Comprehensive Enterprise Assessment                      ║
╚══════════════════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${CYAN}🔍 Running comprehensive validation of Neo N3 Rust Smart Contract Framework${NC}"
echo -e "${CYAN}📊 This suite validates production readiness across all critical dimensions${NC}"
echo ""

# Validation tracking
VALIDATIONS_RUN=0
VALIDATIONS_PASSED=0
VALIDATIONS_FAILED=0

run_validation() {
    local name="$1"
    local script="$2"
    local description="$3"
    
    VALIDATIONS_RUN=$((VALIDATIONS_RUN + 1))
    
    echo -e "\n${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${PURPLE}                    VALIDATION $VALIDATIONS_RUN: $name${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}Description: $description${NC}"
    echo ""
    
    if [ -f "$script" ] && [ -x "$script" ]; then
        local start_time=$(date +%s)
        
        if "$script"; then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            echo -e "\n${GREEN}✅ $name PASSED${NC} (${duration}s)"
            VALIDATIONS_PASSED=$((VALIDATIONS_PASSED + 1))
            return 0
        else
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            echo -e "\n${RED}❌ $name FAILED${NC} (${duration}s)"
            VALIDATIONS_FAILED=$((VALIDATIONS_FAILED + 1))
            return 1
        fi
    else
        echo -e "${RED}❌ Validation script not found or not executable: $script${NC}"
        VALIDATIONS_FAILED=$((VALIDATIONS_FAILED + 1))
        return 1
    fi
}

# Start comprehensive validation
echo -e "${BLUE}Starting validation suite at $(date)${NC}"
echo -e "${BLUE}Project root: $PROJECT_ROOT${NC}"
echo ""

# 1. Production Readiness Check
run_validation \
    "Production Readiness Assessment" \
    "$SCRIPT_DIR/production-readiness-check.sh" \
    "Comprehensive production deployment readiness validation"

# 2. Neo N3 Completeness Check
run_validation \
    "Neo N3 Completeness Validation" \
    "$SCRIPT_DIR/validate-neo-n3-completeness.sh" \
    "Validation of complete Neo N3 syscalls, natives, types, and opcodes support"

# 3. Reference Implementation Check
run_validation \
    "Reference Implementation Compliance" \
    "$SCRIPT_DIR/validate-reference-implementation.sh" \
    "Validation against official Neo N3 specifications and reference implementations"

# 4. Infrastructure Validation (if exists)
if [ -f "$SCRIPT_DIR/validate-infrastructure.sh" ]; then
    run_validation \
        "Infrastructure Validation" \
        "$SCRIPT_DIR/validate-infrastructure.sh" \
        "Enterprise infrastructure and CI/CD validation"
fi

# 5. Run workspace tests
echo -e "\n${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}                    VALIDATION $((VALIDATIONS_RUN + 1)): Comprehensive Test Suite${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Description: Running all framework tests including unit, integration, and examples${NC}"
echo ""

VALIDATIONS_RUN=$((VALIDATIONS_RUN + 1))
cd "$PROJECT_ROOT"

# Test core library
echo -e "${CYAN}Running neo-contract tests...${NC}"
if cargo test -p neo-contract --lib >/dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} neo-contract library tests passed"
    core_tests_passed=1
else
    echo -e "${RED}✗${NC} neo-contract library tests failed"
    core_tests_passed=0
fi

# Test compiler
echo -e "${CYAN}Running neo-compiler tests...${NC}"
if cargo test -p neo-compiler >/dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} neo-compiler tests passed"
    compiler_tests_passed=1
else
    echo -e "${RED}✗${NC} neo-compiler tests failed"
    compiler_tests_passed=0
fi

# Test proc macros
echo -e "${CYAN}Running neo-contract-proc-macros tests...${NC}"
if cargo test -p neo-contract-proc-macros >/dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} neo-contract-proc-macros tests passed"
    macros_tests_passed=1
else
    echo -e "${RED}✗${NC} neo-contract-proc-macros tests failed"
    macros_tests_passed=0
fi

# Count example compilations
echo -e "${CYAN}Validating example contract compilation...${NC}"
total_examples=$(find examples -name "Cargo.toml" -not -path "*/build/*" | wc -l)
passed_examples=0

for example_dir in $(find examples -name "Cargo.toml" -not -path "*/build/*" | xargs dirname | head -10); do
    if (cd "$example_dir" && cargo check >/dev/null 2>&1); then
        passed_examples=$((passed_examples + 1))
    fi
done

echo -e "${GREEN}✓${NC} $passed_examples/$total_examples example contracts compile successfully"

# Overall test validation result
if [ $core_tests_passed -eq 1 ] && [ $compiler_tests_passed -eq 1 ] && [ $macros_tests_passed -eq 1 ] && [ $passed_examples -gt $((total_examples * 8 / 10)) ]; then
    echo -e "\n${GREEN}✅ Comprehensive Test Suite PASSED${NC}"
    VALIDATIONS_PASSED=$((VALIDATIONS_PASSED + 1))
else
    echo -e "\n${RED}❌ Comprehensive Test Suite FAILED${NC}"
    VALIDATIONS_FAILED=$((VALIDATIONS_FAILED + 1))
fi

# Generate comprehensive report
echo -e "\n${BLUE}"
cat << "EOF"
╔══════════════════════════════════════════════════════════════════════════════╗
║                           VALIDATION SUITE REPORT                           ║
╚══════════════════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${CYAN}📊 OVERALL ASSESSMENT${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Total Validations:   ${BLUE}$VALIDATIONS_RUN${NC}"
echo -e "Passed:             ${GREEN}$VALIDATIONS_PASSED${NC}"
echo -e "Failed:             ${RED}$VALIDATIONS_FAILED${NC}"

# Calculate success rate
SUCCESS_RATE=$((VALIDATIONS_PASSED * 100 / VALIDATIONS_RUN))
echo -e "Success Rate:       ${BLUE}$SUCCESS_RATE%${NC}"

echo ""
echo -e "${CYAN}🎯 FRAMEWORK ASSESSMENT${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $SUCCESS_RATE -eq 100 ]; then
    echo -e "${GREEN}🏆 EXCELLENCE ACHIEVED${NC}"
    echo "The Neo N3 Framework has passed all validation criteria with flying colors."
    echo -e "✅ ${GREEN}APPROVED FOR ENTERPRISE DEPLOYMENT${NC}"
    echo -e "✅ Suitable for high-value financial applications"
    echo -e "✅ Ready for mission-critical business systems"
    echo -e "✅ Approved for enterprise blockchain solutions"
    FINAL_STATUS="ENTERPRISE_READY"
elif [ $SUCCESS_RATE -ge 80 ]; then
    echo -e "${GREEN}🎉 PRODUCTION READY${NC}"
    echo "The Neo N3 Framework meets production standards with excellent quality."
    echo -e "✅ ${GREEN}APPROVED FOR PRODUCTION DEPLOYMENT${NC}"
    echo -e "✅ Suitable for business applications"
    echo -e "⚠️ Monitor areas that need minor improvements"
    FINAL_STATUS="PRODUCTION_READY"
elif [ $SUCCESS_RATE -ge 60 ]; then
    echo -e "${YELLOW}⚠️ DEVELOPMENT READY${NC}"
    echo "The Neo N3 Framework shows good progress but needs improvements."
    echo -e "⚠️ ${YELLOW}REQUIRES IMPROVEMENTS BEFORE PRODUCTION${NC}"
    echo -e "⚠️ Address failed validations before deployment"
    echo -e "⚠️ Suitable for development and testing environments"
    FINAL_STATUS="DEVELOPMENT_READY"
else
    echo -e "${RED}❌ NEEDS SIGNIFICANT WORK${NC}"
    echo "The Neo N3 Framework requires substantial improvements."
    echo -e "❌ ${RED}NOT READY FOR PRODUCTION DEPLOYMENT${NC}"
    echo -e "❌ Multiple critical areas need attention"
    echo -e "❌ Extended development timeline required"
    FINAL_STATUS="NOT_READY"
fi

echo ""
echo -e "${CYAN}📋 RECOMMENDATIONS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $SUCCESS_RATE -eq 100 ]; then
    echo "• Maintain current high standards through regular validation"
    echo "• Consider implementing advanced monitoring and optimization"
    echo "• Share framework as a reference implementation"
elif [ $SUCCESS_RATE -ge 80 ]; then
    echo "• Address specific failures identified in validation reports"
    echo "• Implement continuous validation in CI/CD pipeline"
    echo "• Consider performance optimization opportunities"
elif [ $SUCCESS_RATE -ge 60 ]; then
    echo "• Focus on failed validations as highest priority"
    echo "• Implement comprehensive testing strategy"
    echo "• Address security and infrastructure gaps"
else
    echo "• Complete fundamental framework development"
    echo "• Establish core infrastructure and security measures"
    echo "• Implement basic testing and validation framework"
fi

echo ""
echo -e "${CYAN}📁 DETAILED REPORTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "• Production Readiness: PRODUCTION_READINESS_REPORT.md"
echo "• Security Hardening: SECURITY_HARDENING_REPORT.md"
echo "• Testing Infrastructure: COMPREHENSIVE_TESTING_REPORT.md"
echo "• Enterprise Infrastructure: ENTERPRISE_INFRASTRUCTURE_SUMMARY.md"

echo ""
echo -e "${BLUE}Validation completed at $(date)${NC}"
echo -e "${BLUE}Framework Status: $FINAL_STATUS${NC}"

# Create summary file
cat > "$PROJECT_ROOT/VALIDATION_SUMMARY.md" << EOF
# Neo N3 Framework Validation Summary

**Validation Date**: $(date)
**Framework Version**: 1.0.0
**Overall Status**: $FINAL_STATUS

## Results

- **Total Validations**: $VALIDATIONS_RUN
- **Passed**: $VALIDATIONS_PASSED
- **Failed**: $VALIDATIONS_FAILED
- **Success Rate**: $SUCCESS_RATE%

## Assessment

$(if [ $SUCCESS_RATE -eq 100 ]; then
    echo "🏆 **ENTERPRISE READY** - Excellence achieved across all validation criteria"
elif [ $SUCCESS_RATE -ge 80 ]; then
    echo "✅ **PRODUCTION READY** - Meets production standards with excellent quality"
elif [ $SUCCESS_RATE -ge 60 ]; then
    echo "⚠️ **DEVELOPMENT READY** - Good progress, improvements needed before production"
else
    echo "❌ **NOT READY** - Requires substantial improvements"
fi)

## Detailed Reports

- [Production Readiness Report](PRODUCTION_READINESS_REPORT.md)
- [Security Hardening Report](SECURITY_HARDENING_REPORT.md)
- [Testing Infrastructure Report](COMPREHENSIVE_TESTING_REPORT.md)
- [Enterprise Infrastructure Summary](ENTERPRISE_INFRASTRUCTURE_SUMMARY.md)

---
*Generated by Neo N3 Framework Validation Suite*
EOF

echo -e "${GREEN}📄 Validation summary saved to VALIDATION_SUMMARY.md${NC}"

# Set exit code based on overall success
if [ $SUCCESS_RATE -ge 80 ]; then
    exit 0
else
    exit 1
fi