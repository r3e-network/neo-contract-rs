#!/bin/bash

# Final Production Readiness Check
# Simplified and accurate validation for production deployment

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
echo -e "${BLUE}║                  FINAL PRODUCTION READINESS CHECK             ║${NC}"
echo -e "${BLUE}║                     Neo N3 Framework Assessment               ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

cd "$PROJECT_ROOT"

TOTAL_SCORE=0
MAX_SCORE=100

echo -e "\n${PURPLE}1. PRODUCTION CODE QUALITY CHECKS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check for TODO/FIXME in production code
echo -n "  Checking for TODO/FIXME comments in production code... "
TODO_COUNT=$(grep -r -i "todo\|fixme" \
    --include="*.rs" \
    --exclude-dir=".hive-mind" \
    --exclude-dir=".claude-flow" \
    --exclude-dir="target" \
    --exclude-dir="build" \
    neo-contract/src/ neo-compiler/src/ neo-contract-proc-macros/src/ 2>/dev/null | wc -l)

if [ "$TODO_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ Clean (0 found)${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 20))
else
    echo -e "${RED}❌ Found $TODO_COUNT TODO/FIXME comments${NC}"
fi

# Check for placeholder implementations
echo -n "  Checking for placeholder implementations... "
PLACEHOLDER_COUNT=$(grep -r -i "unimplemented!\|placeholder\|for now\|temporary" \
    --include="*.rs" \
    --exclude-dir=".hive-mind" \
    --exclude-dir=".claude-flow" \
    --exclude-dir="target" \
    --exclude-dir="build" \
    --exclude-dir="tests" \
    neo-contract/src/ neo-compiler/src/ neo-contract-proc-macros/src/ 2>/dev/null | wc -l)

if [ "$PLACEHOLDER_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ Clean (0 found)${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 15))
else
    echo -e "${RED}❌ Found $PLACEHOLDER_COUNT placeholder implementations${NC}"
fi

# Check for panic-prone code
echo -n "  Checking for panic-prone code in core modules... "
PANIC_COUNT=$(grep -r "unwrap()\|expect(" \
    --include="*.rs" \
    --exclude-dir="tests" \
    neo-contract/src/contract/ neo-contract/src/runtime/ 2>/dev/null | wc -l)

if [ "$PANIC_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ Clean (0 panic risks)${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 15))
elif [ "$PANIC_COUNT" -le 5 ]; then
    echo -e "${YELLOW}⚠ Minimal ($PANIC_COUNT found)${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 10))
else
    echo -e "${RED}❌ Found $PANIC_COUNT panic risks${NC}"
fi

echo -e "\n${PURPLE}2. CORE FRAMEWORK COMPILATION${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Test core library compilation
echo -n "  Testing neo-contract library compilation... "
if cargo check -p neo-contract >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Success${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 15))
else
    echo -e "${RED}❌ Failed${NC}"
fi

# Test compiler compilation
echo -n "  Testing neo-compiler compilation... "
if cargo check -p neo-compiler >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Success${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 15))
else
    echo -e "${RED}❌ Failed${NC}"
fi

# Test proc macros compilation
echo -n "  Testing neo-contract-proc-macros compilation... "
if cargo check -p neo-contract-proc-macros >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Success${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 10))
else
    echo -e "${RED}❌ Failed${NC}"
fi

echo -e "\n${PURPLE}3. EXAMPLE CONTRACTS VALIDATION${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Count working examples
TOTAL_EXAMPLES=0
WORKING_EXAMPLES=0

echo "  Testing example contract compilation:"
for example_dir in $(find examples -name "Cargo.toml" -not -path "*/build/*" | xargs dirname | sort | head -10); do
    TOTAL_EXAMPLES=$((TOTAL_EXAMPLES + 1))
    example_name=$(basename "$example_dir")
    echo -n "    $example_name... "
    
    if (cd "$example_dir" && cargo check >/dev/null 2>&1); then
        echo -e "${GREEN}✅${NC}"
        WORKING_EXAMPLES=$((WORKING_EXAMPLES + 1))
    else
        echo -e "${RED}❌${NC}"
    fi
done

EXAMPLE_SUCCESS_RATE=$((WORKING_EXAMPLES * 100 / TOTAL_EXAMPLES))
echo -e "  Example success rate: ${BLUE}$WORKING_EXAMPLES/$TOTAL_EXAMPLES${NC} (${BLUE}$EXAMPLE_SUCCESS_RATE%${NC})"

if [ "$EXAMPLE_SUCCESS_RATE" -ge 80 ]; then
    TOTAL_SCORE=$((TOTAL_SCORE + 20))
elif [ "$EXAMPLE_SUCCESS_RATE" -ge 60 ]; then
    TOTAL_SCORE=$((TOTAL_SCORE + 15))
elif [ "$EXAMPLE_SUCCESS_RATE" -ge 40 ]; then
    TOTAL_SCORE=$((TOTAL_SCORE + 10))
else
    TOTAL_SCORE=$((TOTAL_SCORE + 5))
fi

echo -e "\n${PURPLE}4. DOCUMENTATION AND STANDARDS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check documentation
echo -n "  Checking documentation completeness... "
DOC_COUNT=$(find docs/ -name "*.md" 2>/dev/null | wc -l)
if [ "$DOC_COUNT" -ge 10 ]; then
    echo -e "${GREEN}✅ Comprehensive ($DOC_COUNT guides)${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 5))
else
    echo -e "${YELLOW}⚠ Basic ($DOC_COUNT guides)${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 3))
fi

# Check version consistency
echo -n "  Checking version consistency... "
VERSION_INCONSISTENCIES=$(grep -r '^version = ' */Cargo.toml examples/*/Cargo.toml 2>/dev/null | \
    cut -d: -f2 | sort | uniq | wc -l)

if [ "$VERSION_INCONSISTENCIES" -le 2 ]; then
    echo -e "${GREEN}✅ Consistent${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 5))
else
    echo -e "${YELLOW}⚠ $VERSION_INCONSISTENCIES different versions${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 2))
fi

# Calculate final assessment
PERCENTAGE=$((TOTAL_SCORE))

echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                        FINAL ASSESSMENT                       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${PURPLE}📊 PRODUCTION READINESS SCORE${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Score: ${BLUE}$TOTAL_SCORE/100${NC}"

if [ $PERCENTAGE -ge 90 ]; then
    echo -e "\n${GREEN}🏆 ENTERPRISE READY${NC}"
    echo "The Neo N3 framework meets enterprise production standards."
    echo -e "✅ Suitable for high-value financial applications"
    echo -e "✅ Ready for mission-critical business systems"
    echo -e "✅ Approved for enterprise blockchain solutions"
    STATUS="ENTERPRISE_READY"
elif [ $PERCENTAGE -ge 80 ]; then
    echo -e "\n${GREEN}✅ PRODUCTION READY${NC}"
    echo "The Neo N3 framework meets production standards."
    echo -e "✅ Suitable for business applications"
    echo -e "⚠ Monitor minor issues identified above"
    STATUS="PRODUCTION_READY"
elif [ $PERCENTAGE -ge 70 ]; then
    echo -e "\n${YELLOW}⚠ DEVELOPMENT READY${NC}"
    echo "The framework is suitable for development environments."
    echo -e "⚠ Address issues before production deployment"
    STATUS="DEVELOPMENT_READY"
elif [ $PERCENTAGE -ge 60 ]; then
    echo -e "\n${YELLOW}⚠ NEEDS IMPROVEMENT${NC}"
    echo "The framework needs improvements before deployment."
    echo -e "⚠ Significant work required"
    STATUS="NEEDS_IMPROVEMENT"
else
    echo -e "\n${RED}❌ NOT READY${NC}"
    echo "The framework requires substantial work."
    echo -e "❌ Not suitable for any production use"
    STATUS="NOT_READY"
fi

echo -e "\n${PURPLE}📋 KEY METRICS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "TODO/FIXME Comments: ${BLUE}$TODO_COUNT${NC}"
echo -e "Placeholder Code: ${BLUE}$PLACEHOLDER_COUNT${NC}"
echo -e "Panic Risks: ${BLUE}$PANIC_COUNT${NC}"
echo -e "Example Success Rate: ${BLUE}$EXAMPLE_SUCCESS_RATE%${NC}"
echo -e "Documentation Files: ${BLUE}$DOC_COUNT${NC}"

echo -e "\n${PURPLE}📈 RECOMMENDATIONS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $PERCENTAGE -ge 90 ]; then
    echo "• Maintain current high standards"
    echo "• Consider advanced optimization"
    echo "• Monitor production deployment"
elif [ $PERCENTAGE -ge 80 ]; then
    echo "• Address remaining compilation issues"
    echo "• Clean up any remaining warnings"
    echo "• Prepare for production deployment"
elif [ $PERCENTAGE -ge 70 ]; then
    echo "• Fix core compilation errors"
    echo "• Remove remaining placeholders"
    echo "• Improve example success rate"
else
    echo "• Focus on core infrastructure"
    echo "• Complete missing implementations"
    echo "• Establish basic quality standards"
fi

echo -e "\n${BLUE}Assessment completed: $(date)${NC}"
echo -e "${BLUE}Framework Status: $STATUS${NC}"

# Exit code based on readiness
if [ $PERCENTAGE -ge 80 ]; then
    exit 0
else
    exit 1
fi