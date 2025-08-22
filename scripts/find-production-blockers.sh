#!/bin/bash

# Production Blocker Detection Script
# Finds all placeholders, TODOs, and non-production ready code

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                Production Blocker Detection                   ║${NC}"
echo -e "${BLUE}║            Finding Non-Production Ready Code                  ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

cd "$PROJECT_ROOT"

TOTAL_ISSUES=0

# Function to search and report issues
search_issues() {
    local pattern="$1"
    local description="$2"
    local exclude_patterns="$3"
    
    echo -e "\n${YELLOW}🔍 Searching for: $description${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    local found=0
    local exclude_args=""
    
    # Build exclude arguments for grep
    if [ -n "$exclude_patterns" ]; then
        for exclude_pattern in $exclude_patterns; do
            exclude_args="$exclude_args --exclude-dir=$exclude_pattern"
        done
    fi
    
    # Search for the pattern
    local results=$(grep -r -i -n $exclude_args \
        --exclude-dir=".hive-mind" \
        --exclude-dir=".claude-flow" \
        --exclude="*.md" \
        --exclude="*.toml" \
        --exclude="*.sh" \
        --exclude="*.yml" \
        --exclude="*.yaml" \
        --exclude="*.json" \
        --exclude="*.nef" \
        --exclude="*.manifest.json" \
        "$pattern" . 2>/dev/null || true)
    
    if [ -n "$results" ]; then
        echo -e "${RED}❌ Found issues:${NC}"
        echo "$results" | while IFS= read -r line; do
            found=$((found + 1))
            TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
            file=$(echo "$line" | cut -d: -f1)
            line_num=$(echo "$line" | cut -d: -f2)
            content=$(echo "$line" | cut -d: -f3-)
            echo -e "   ${RED}•${NC} $file:$line_num → $content"
        done
        return 1
    else
        echo -e "${GREEN}✅ No issues found${NC}"
        return 0
    fi
}

# Function to check for compilation errors
check_compilation_errors() {
    echo -e "\n${YELLOW}🔍 Checking for compilation errors${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    local failed_examples=()
    
    # Check core library
    echo -n "  Checking neo-contract... "
    if cargo check -p neo-contract >/dev/null 2>&1; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${RED}❌${NC}"
        TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
    fi
    
    # Check compiler
    echo -n "  Checking neo-compiler... "
    if cargo check -p neo-compiler >/dev/null 2>&1; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${RED}❌${NC}"
        TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
    fi
    
    # Check proc macros
    echo -n "  Checking neo-contract-proc-macros... "
    if cargo check -p neo-contract-proc-macros >/dev/null 2>&1; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${RED}❌${NC}"
        TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
    fi
    
    # Check examples
    echo "  Checking examples..."
    local total_examples=0
    local failed_examples=0
    
    for example_dir in $(find examples -name "Cargo.toml" -not -path "*/build/*" | xargs dirname | sort); do
        total_examples=$((total_examples + 1))
        example_name=$(basename "$example_dir")
        echo -n "    $example_name... "
        
        if (cd "$example_dir" && cargo check >/dev/null 2>&1); then
            echo -e "${GREEN}✅${NC}"
        else
            echo -e "${RED}❌${NC}"
            failed_examples=$((failed_examples + 1))
            TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
        fi
    done
    
    echo -e "  Examples summary: ${GREEN}$((total_examples - failed_examples))${NC}/${BLUE}$total_examples${NC} passing"
    
    if [ $failed_examples -gt 0 ]; then
        echo -e "  ${RED}$failed_examples examples have compilation errors${NC}"
    fi
}

# Search for various types of production blockers
search_issues "todo\|TODO\|Todo" "TODO comments and tasks" "target build .git"
search_issues "fixme\|FIXME\|Fixme" "FIXME comments" "target build .git"
search_issues "placeholder\|Placeholder\|PLACEHOLDER" "Placeholder implementations" "target build .git"
search_issues "unimplemented!\|unimplemented\(\)" "Unimplemented macros" "target build .git"
search_issues "panic!\|panic\(\)" "Panic macros" "target build .git"
search_issues "unwrap()" "Unwrap calls that can panic" "target build .git tests"
search_issues "expect(" "Expect calls that can panic" "target build .git tests"
search_issues "for now\|for later\|temporary\|temp\|hack\|quick fix" "Temporary code markers" "target build .git"
search_issues "in a real implementation\|in production\|in a production implementation" "Production readiness comments" "target build .git"
search_issues "simplified\|simple version\|basic version" "Simplified implementations" "target build .git"
search_issues "not implemented\|not supported\|not available" "Missing implementations" "target build .git"
search_issues "stub\|mock\|fake\|dummy" "Test/mock code in production" "target build .git tests"
search_issues "unreachable!\|unreachable\(\)" "Unreachable macros" "target build .git"
search_issues "deprecated\|obsolete" "Deprecated code" "target build .git"
search_issues "broken\|doesnt work\|doesn't work\|not working" "Broken code markers" "target build .git"

# Check for compilation errors
check_compilation_errors

# Check for missing struct definitions
echo -e "\n${YELLOW}🔍 Checking for missing struct/type definitions${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Common missing types found in compilation errors
missing_types=(
    "Crowdfunding"
    "NeoFeaturesShowcase" 
    "NEP17TokenSolanaStyle"
    "MultisigWallet"
    "Governance"
    "Staking"
    "HelloWorldSolanaSimple"
    "Nep24RoyaltyNft"
)

for missing_type in "${missing_types[@]}"; do
    echo -n "  Checking for missing type $missing_type... "
    
    # Look for impl blocks without struct definitions
    impl_files=$(grep -r "impl $missing_type" --include="*.rs" . 2>/dev/null | cut -d: -f1 | sort -u || true)
    struct_files=$(grep -r "struct $missing_type\|pub struct $missing_type" --include="*.rs" . 2>/dev/null | cut -d: -f1 | sort -u || true)
    
    if [ -n "$impl_files" ] && [ -z "$struct_files" ]; then
        echo -e "${RED}❌ Missing definition${NC}"
        echo "    Found impl in: $impl_files"
        TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
    elif [ -n "$struct_files" ]; then
        echo -e "${GREEN}✅ Defined${NC}"
    else
        echo -e "${BLUE}➖ Not used${NC}"
    fi
done

# Check for unsafe code
echo -e "\n${YELLOW}🔍 Checking for unsafe code blocks${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

unsafe_code=$(grep -r -n "unsafe" --include="*.rs" . \
    --exclude-dir=target \
    --exclude-dir=build \
    --exclude-dir=.git \
    --exclude-dir=tests \
    2>/dev/null || true)

if [ -n "$unsafe_code" ]; then
    echo -e "${YELLOW}⚠️ Found unsafe code blocks:${NC}"
    echo "$unsafe_code" | while IFS= read -r line; do
        file=$(echo "$line" | cut -d: -f1)
        line_num=$(echo "$line" | cut -d: -f2)
        content=$(echo "$line" | cut -d: -f3-)
        echo -e "   ${YELLOW}•${NC} $file:$line_num → $content"
    done
    echo -e "${YELLOW}   Note: Review all unsafe blocks for production safety${NC}"
else
    echo -e "${GREEN}✅ No unsafe code found in core libraries${NC}"
fi

# Check for dead code
echo -e "\n${YELLOW}🔍 Checking for dead/unused code${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo -n "  Running cargo check with warnings... "
dead_code_warnings=$(cargo check --workspace 2>&1 | grep -E "dead_code|unused" | wc -l)

if [ "$dead_code_warnings" -gt 20 ]; then
    echo -e "${RED}❌ $dead_code_warnings dead code warnings${NC}"
    echo -e "   ${RED}Significant dead code detected - requires cleanup${NC}"
    TOTAL_ISSUES=$((TOTAL_ISSUES + 1))
elif [ "$dead_code_warnings" -gt 5 ]; then
    echo -e "${YELLOW}⚠️ $dead_code_warnings dead code warnings${NC}"
    echo -e "   ${YELLOW}Some dead code detected - recommend cleanup${NC}"
else
    echo -e "${GREEN}✅ Minimal dead code ($dead_code_warnings warnings)${NC}"
fi

# Final summary
echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                     PRODUCTION BLOCKER REPORT                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${PURPLE}📊 SUMMARY${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Total production blockers found: ${RED}$TOTAL_ISSUES${NC}"

if [ $TOTAL_ISSUES -eq 0 ]; then
    echo -e "\n${GREEN}🎉 PRODUCTION READY - NO BLOCKERS FOUND${NC}"
    echo "The codebase appears to be free of major production blockers."
    echo -e "✅ No TODOs, FIXMEs, or placeholder code"
    echo -e "✅ No compilation errors"
    echo -e "✅ No obvious production readiness issues"
    exit 0
elif [ $TOTAL_ISSUES -le 5 ]; then
    echo -e "\n${YELLOW}⚠️ MINOR ISSUES - MOSTLY PRODUCTION READY${NC}"
    echo "Found minor issues that should be addressed before production."
    echo -e "⚠️ Address the issues listed above"
    echo -e "⚠️ Consider additional code review"
    exit 1
elif [ $TOTAL_ISSUES -le 20 ]; then
    echo -e "\n${RED}❌ MODERATE ISSUES - NEEDS WORK${NC}"
    echo "Found moderate number of production blockers."
    echo -e "❌ Significant cleanup required"
    echo -e "❌ Address compilation errors and placeholders"
    echo -e "❌ Comprehensive code review needed"
    exit 1
else
    echo -e "\n${RED}🚨 CRITICAL ISSUES - NOT PRODUCTION READY${NC}"
    echo "Found significant production blockers that must be resolved."
    echo -e "🚨 Major development work required"
    echo -e "🚨 Systematic cleanup and refactoring needed"
    echo -e "🚨 Not suitable for any production deployment"
    exit 1
fi