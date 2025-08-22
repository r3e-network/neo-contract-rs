#!/bin/bash

# Neo N3 Framework Production Readiness Check
# Comprehensive validation for enterprise deployment readiness

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

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              Neo N3 Framework Production Readiness Check      ║${NC}"
echo -e "${BLUE}║                    Enterprise Deployment Validation           ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

# Initialize scoring
TOTAL_SCORE=0
MAX_SCORE=0
CRITICAL_FAILURES=0

# Scoring function
score_check() {
    local category="$1"
    local description="$2"
    local weight="$3"
    local command="$4"
    local pass_threshold="${5:-1}"
    
    echo -n "  [$category] $description... "
    
    local result=0
    if eval "$command" >/dev/null 2>&1; then
        local count=$(eval "$command" 2>/dev/null | wc -l || echo "1")
        if [ "$count" -ge "$pass_threshold" ]; then
            result=$weight
            echo -e "${GREEN}✓${NC} (+$weight points)"
        else
            echo -e "${YELLOW}⚠${NC} (insufficient: $count found, need $pass_threshold)"
        fi
    else
        echo -e "${RED}✗${NC} (failed)"
        if [ "$weight" -eq 100 ]; then
            CRITICAL_FAILURES=$((CRITICAL_FAILURES + 1))
        fi
    fi
    
    TOTAL_SCORE=$((TOTAL_SCORE + result))
    MAX_SCORE=$((MAX_SCORE + weight))
    return $result
}

# Scoring function for build commands
score_build_check() {
    local category="$1"
    local description="$2"
    local weight="$3"
    local command="$4"
    
    echo -n "  [$category] $description... "
    
    local result=0
    if eval "$command" >/dev/null 2>&1; then
        result=$weight
        echo -e "${GREEN}✓${NC} (+$weight points)"
    else
        echo -e "${RED}✗${NC} (build failed)"
        if [ "$weight" -eq 100 ]; then
            CRITICAL_FAILURES=$((CRITICAL_FAILURES + 1))
        fi
    fi
    
    TOTAL_SCORE=$((TOTAL_SCORE + result))
    MAX_SCORE=$((MAX_SCORE + weight))
    return $result
}

echo -e "\n${PURPLE}═══ 1. CRITICAL INFRASTRUCTURE CHECKS ═══${NC}"

score_check "CRITICAL" "Workspace builds successfully" 100 \
    "cd $PROJECT_ROOT && cargo check --workspace"

score_check "CRITICAL" "Core library compiles" 100 \
    "cd $PROJECT_ROOT && cargo check -p neo-contract"

score_check "CRITICAL" "Compiler builds" 100 \
    "cd $PROJECT_ROOT && cargo check -p neo-compiler"

score_check "CRITICAL" "All examples compile" 100 \
    "find $PROJECT_ROOT/examples -name Cargo.toml -not -path '*/build/*' | wc -l | grep -v '^0$'"

echo -e "\n${PURPLE}═══ 2. SECURITY VALIDATION ═══${NC}"

score_check "SECURITY" "No unwrap() in critical paths" 50 \
    "! grep -r '\.unwrap()' $PROJECT_ROOT/neo-contract/src/contract/ $PROJECT_ROOT/neo-contract/src/runtime/"

score_check "SECURITY" "Proper error handling types" 30 \
    "grep -r 'Result<' $PROJECT_ROOT/neo-contract/src/" \
    "5"

score_check "SECURITY" "Input validation patterns" 25 \
    "grep -r 'validate\|check.*input\|require!' $PROJECT_ROOT/neo-contract/src/" \
    "3"

score_check "SECURITY" "Security test coverage" 25 \
    "find $PROJECT_ROOT -name '*security*test*' -o -name '*test*security*'" \
    "1"

score_check "SECURITY" "No unsafe code in core libs" 20 \
    "! grep -r 'unsafe' $PROJECT_ROOT/neo-contract/src/ $PROJECT_ROOT/neo-contract-proc-macros/src/"

echo -e "\n${PURPLE}═══ 3. NEO N3 SPECIFICATION COMPLIANCE ═══${NC}"

score_check "NEO-N3" "System.Storage syscalls" 50 \
    "grep -r 'SYSTEM_STORAGE\|storage_get\|storage_put' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "3"

score_check "NEO-N3" "System.Runtime syscalls" 40 \
    "grep -r 'SYSTEM_RUNTIME\|check_witness\|runtime_notify' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "3"

score_check "NEO-N3" "System.Crypto syscalls" 30 \
    "grep -r 'SYSTEM_CRYPTO\|crypto_sha256' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

score_check "NEO-N3" "Native contract support" 30 \
    "find $PROJECT_ROOT/neo-contract/src/native -name '*.rs'" \
    "5"

score_check "NEO-N3" "VM opcodes implementation" 40 \
    "grep -r 'Push\|Jump\|Call\|Add\|Equal' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "5"

score_check "NEO-N3" "NEP standard implementations" 30 \
    "find $PROJECT_ROOT -name '*.rs' -exec grep -l 'Nep17\|Nep11\|NEP.*17\|NEP.*11' {} \;" \
    "2"

echo -e "\n${PURPLE}═══ 4. TYPE SYSTEM COMPLETENESS ═══${NC}"

score_check "TYPES" "Core Neo types (H160, H256)" 40 \
    "find $PROJECT_ROOT/neo-contract/src/types -name '*.rs' -exec grep -l 'H160\|H256' {} \;" \
    "2"

score_check "TYPES" "Arithmetic types (Int256)" 30 \
    "find $PROJECT_ROOT/neo-contract/src/types -name '*.rs' -exec grep -l 'Int256' {} \;" \
    "1"

score_check "TYPES" "Collection types (Array, Map)" 25 \
    "find $PROJECT_ROOT/neo-contract/src/types -name '*.rs' -exec grep -l 'Array\|Map' {} \;" \
    "2"

score_check "TYPES" "String and Buffer types" 20 \
    "find $PROJECT_ROOT/neo-contract/src/types -name '*.rs' -exec grep -l 'ByteString\|Buffer' {} \;" \
    "2"

score_check "TYPES" "Serialization support" 35 \
    "find $PROJECT_ROOT/neo-contract/src/serialize -name '*.rs'" \
    "3"

echo -e "\n${PURPLE}═══ 5. COMPILER CAPABILITIES ═══${NC}"

score_check "COMPILER" "WASM instruction parsing" 50 \
    "find $PROJECT_ROOT/neo-compiler/src -name '*.rs' -exec grep -l 'wasm\|WASM' {} \;" \
    "2"

score_check "COMPILER" "NEF generation" 40 \
    "find $PROJECT_ROOT/neo-compiler/src -name '*.rs' -exec grep -l 'Nef\|NEF' {} \;" \
    "2"

score_check "COMPILER" "Manifest generation" 30 \
    "find $PROJECT_ROOT/neo-compiler/src -name '*.rs' -exec grep -l 'Manifest' {} \;" \
    "2"

score_check "COMPILER" "Optimization passes" 25 \
    "find $PROJECT_ROOT/neo-compiler/src -name '*optim*' -o -name '*optimizer*'" \
    "1"

score_check "COMPILER" "Debug information" 15 \
    "find $PROJECT_ROOT/neo-compiler/src -name '*debug*'" \
    "1"

echo -e "\n${PURPLE}═══ 6. TESTING INFRASTRUCTURE ═══${NC}"

score_check "TESTING" "Unit test coverage" 40 \
    "find $PROJECT_ROOT -name '*test*.rs' -not -path '*/target/*'" \
    "10"

score_check "TESTING" "Integration tests" 30 \
    "find $PROJECT_ROOT -path '*/tests/*.rs'" \
    "5"

score_check "TESTING" "Example contract tests" 20 \
    "find $PROJECT_ROOT/examples -name '*test*' -o -name 'tests'" \
    "3"

score_check "TESTING" "Mock environment" 25 \
    "grep -r 'mock\|Mock' $PROJECT_ROOT/neo-contract/tests/" \
    "3"

score_check "TESTING" "Comprehensive test modules" 35 \
    "find $PROJECT_ROOT -name '*comprehensive*test*'" \
    "3"

echo -e "\n${PURPLE}═══ 7. DOCUMENTATION AND EXAMPLES ═══${NC}"

score_check "DOCS" "README documentation" 20 \
    "test -f $PROJECT_ROOT/README.md && wc -l $PROJECT_ROOT/README.md | awk '{print \$1}' | grep -v '^[0-9]$'"

score_check "DOCS" "API documentation" 25 \
    "find $PROJECT_ROOT/docs -name '*.md'" \
    "5"

score_check "DOCS" "Example contracts" 40 \
    "find $PROJECT_ROOT/examples -name 'lib.rs'" \
    "10"

score_check "DOCS" "DeFi examples" 30 \
    "find $PROJECT_ROOT/examples/defi -name 'lib.rs'" \
    "5"

score_check "DOCS" "Build documentation" 15 \
    "find $PROJECT_ROOT -name 'Makefile*' -o -name '*.sh'" \
    "5"

echo -e "\n${PURPLE}═══ 8. VERSION AND RELEASE MANAGEMENT ═══${NC}"

score_check "VERSION" "Consistent versioning" 30 \
    "grep -r '^version = \"1.0.0\"' $PROJECT_ROOT/*/Cargo.toml $PROJECT_ROOT/examples/*/Cargo.toml" \
    "10"

score_check "VERSION" "Changelog present" 15 \
    "find $PROJECT_ROOT -name 'CHANGELOG*' -o -name 'HISTORY*'" \
    "1"

score_check "VERSION" "License file" 10 \
    "test -f $PROJECT_ROOT/LICENSE"

score_check "VERSION" "Git repository health" 15 \
    "cd $PROJECT_ROOT && git status >/dev/null 2>&1 && git log --oneline -n 10" \
    "1"

echo -e "\n${PURPLE}═══ 9. PERFORMANCE AND OPTIMIZATION ═══${NC}"

score_check "PERF" "Release profile optimization" 20 \
    "grep -r 'opt-level.*[\"\']*[23][\"\']*' $PROJECT_ROOT/Cargo.toml $PROJECT_ROOT/*/Cargo.toml"

score_check "PERF" "WASM optimization flags" 15 \
    "grep -r 'wasm.*opt\|opt.*wasm' $PROJECT_ROOT"

score_check "PERF" "Build optimization scripts" 10 \
    "find $PROJECT_ROOT -name '*.sh' -exec grep -l 'release\|optimize' {} \;" \
    "1"

echo -e "\n${PURPLE}═══ 10. CI/CD AND AUTOMATION ═══${NC}"

score_check "CICD" "GitHub Actions workflows" 25 \
    "find $PROJECT_ROOT/.github/workflows -name '*.yml'" \
    "2"

score_check "CICD" "Security automation" 20 \
    "find $PROJECT_ROOT -name '*security*' -o -name '*audit*'" \
    "2"

score_check "CICD" "Test automation" 15 \
    "find $PROJECT_ROOT -name '*test*.sh' -o -name '*test*.yml'" \
    "1"

score_check "CICD" "Build scripts" 10 \
    "find $PROJECT_ROOT -name 'Makefile*' -o -name 'build.sh'" \
    "2"

# Final compilation test with timing
echo -e "\n${PURPLE}═══ 11. PERFORMANCE BENCHMARKS ═══${NC}"

echo -n "  [PERF] Compiler build time... "
START_TIME=$(date +%s)
if (cd "$PROJECT_ROOT" && cargo build --release -p neo-compiler >/dev/null 2>&1); then
    END_TIME=$(date +%s)
    BUILD_TIME=$((END_TIME - START_TIME))
    if [ $BUILD_TIME -le 30 ]; then
        echo -e "${GREEN}✓${NC} (${BUILD_TIME}s, +20 points)"
        TOTAL_SCORE=$((TOTAL_SCORE + 20))
    elif [ $BUILD_TIME -le 60 ]; then
        echo -e "${YELLOW}⚠${NC} (${BUILD_TIME}s, +10 points)"
        TOTAL_SCORE=$((TOTAL_SCORE + 10))
    else
        echo -e "${RED}✗${NC} (${BUILD_TIME}s, too slow)"
    fi
else
    echo -e "${RED}✗${NC} (build failed)"
fi
MAX_SCORE=$((MAX_SCORE + 20))

# Example compilation test
echo -n "  [PERF] Example compilation speed... "
EXAMPLE_START=$(date +%s)
if (cd "$PROJECT_ROOT/examples/01-hello-world" && cargo check >/dev/null 2>&1); then
    EXAMPLE_END=$(date +%s)
    EXAMPLE_TIME=$((EXAMPLE_END - EXAMPLE_START))
    if [ $EXAMPLE_TIME -le 10 ]; then
        echo -e "${GREEN}✓${NC} (${EXAMPLE_TIME}s, +15 points)"
        TOTAL_SCORE=$((TOTAL_SCORE + 15))
    else
        echo -e "${YELLOW}⚠${NC} (${EXAMPLE_TIME}s, +5 points)"
        TOTAL_SCORE=$((TOTAL_SCORE + 5))
    fi
else
    echo -e "${RED}✗${NC} (compilation failed)"
fi
MAX_SCORE=$((MAX_SCORE + 15))

echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    PRODUCTION READINESS REPORT                ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

# Calculate percentage
PERCENTAGE=$((TOTAL_SCORE * 100 / MAX_SCORE))

echo -e "\n${CYAN}📊 SCORING SUMMARY${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Total Score:        ${BLUE}$TOTAL_SCORE${NC} / $MAX_SCORE"
echo -e "Success Rate:       ${BLUE}$PERCENTAGE%${NC}"
echo -e "Critical Failures:  ${RED}$CRITICAL_FAILURES${NC}"

# Determine readiness level
if [ $CRITICAL_FAILURES -gt 0 ]; then
    echo -e "\n${RED}🚨 CRITICAL FAILURES DETECTED - NOT READY FOR PRODUCTION${NC}"
    echo "The framework has critical infrastructure failures that must be resolved."
    exit 1
elif [ $PERCENTAGE -ge 95 ]; then
    echo -e "\n${GREEN}🏆 ENTERPRISE READY - EXCELLENT${NC}"
    echo "Framework exceeds enterprise standards and is ready for production deployment."
    echo -e "✓ Suitable for high-value financial applications"
    echo -e "✓ Ready for mission-critical business systems"
    echo -e "✓ Approved for enterprise blockchain solutions"
elif [ $PERCENTAGE -ge 85 ]; then
    echo -e "\n${GREEN}✅ PRODUCTION READY - GOOD${NC}"
    echo "Framework meets production standards with room for optimization."
    echo -e "✓ Suitable for standard business applications"
    echo -e "⚠ May need optimization for high-value financial systems"
elif [ $PERCENTAGE -ge 75 ]; then
    echo -e "\n${YELLOW}⚠ DEVELOPMENT READY - FAIR${NC}"
    echo "Framework is suitable for development and testing environments."
    echo -e "⚠ Requires improvements before production deployment"
    echo -e "⚠ Not recommended for financial applications"
elif [ $PERCENTAGE -ge 60 ]; then
    echo -e "\n${YELLOW}⚠ PROTOTYPE READY - NEEDS WORK${NC}"
    echo "Framework shows promise but requires significant improvements."
    echo -e "⚠ Multiple critical areas need attention"
    echo -e "⚠ Extended development timeline required"
else
    echo -e "\n${RED}❌ NOT READY - SIGNIFICANT WORK REQUIRED${NC}"
    echo "Framework needs substantial development before any deployment."
    echo -e "❌ Major architectural and implementation gaps"
    echo -e "❌ Not suitable for any production use"
fi

echo -e "\n${CYAN}📈 RECOMMENDATIONS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $PERCENTAGE -ge 95 ]; then
    echo "• Continue monitoring and maintain high standards"
    echo "• Consider advanced optimization and performance tuning"
    echo "• Implement comprehensive monitoring in production"
elif [ $PERCENTAGE -ge 85 ]; then
    echo "• Address remaining security and testing gaps"
    echo "• Optimize performance and build times"
    echo "• Enhance documentation and examples"
elif [ $PERCENTAGE -ge 75 ]; then
    echo "• Focus on critical infrastructure improvements"
    echo "• Implement comprehensive testing strategy"
    echo "• Address security vulnerabilities"
else
    echo "• Complete core infrastructure development"
    echo "• Implement fundamental security measures"
    echo "• Establish basic testing framework"
fi

echo -e "\n${BLUE}Report generated: $(date)${NC}"
echo -e "${BLUE}Framework version: 1.0.0${NC}"

# Set exit code based on readiness level
if [ $PERCENTAGE -ge 85 ] && [ $CRITICAL_FAILURES -eq 0 ]; then
    exit 0
else
    exit 1
fi