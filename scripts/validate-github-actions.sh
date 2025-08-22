#!/bin/bash

# GitHub Actions Validation Script
# Validates all workflow files for syntax and best practices

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                GitHub Actions Validation                      ║${NC}"
echo -e "${BLUE}║              Checking Workflow Configuration                  ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

cd "$PROJECT_ROOT"

TOTAL_WORKFLOWS=0
VALID_WORKFLOWS=0
ISSUES_FOUND=0

# Function to validate workflow
validate_workflow() {
    local workflow_file="$1"
    local workflow_name=$(basename "$workflow_file" .yml)
    
    TOTAL_WORKFLOWS=$((TOTAL_WORKFLOWS + 1))
    echo -e "\n${YELLOW}🔍 Validating: $workflow_name${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    # Check if file exists and is readable
    if [ ! -f "$workflow_file" ]; then
        echo -e "  ${RED}❌ File not found: $workflow_file${NC}"
        ISSUES_FOUND=$((ISSUES_FOUND + 1))
        return 1
    fi
    
    # Check YAML syntax
    echo -n "  YAML syntax... "
    if python3 -c "import yaml; yaml.safe_load(open('$workflow_file'))" 2>/dev/null; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${RED}❌ Invalid YAML${NC}"
        ISSUES_FOUND=$((ISSUES_FOUND + 1))
        return 1
    fi
    
    # Check required fields
    echo -n "  Required fields... "
    if grep -q "^name:" "$workflow_file" && grep -q "^on:" "$workflow_file" && grep -q "^jobs:" "$workflow_file"; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${RED}❌ Missing required fields${NC}"
        ISSUES_FOUND=$((ISSUES_FOUND + 1))
    fi
    
    # Check for actions versions
    echo -n "  Action versions... "
    if grep -q "actions/checkout@v4\|actions/upload-artifact@v4\|actions/cache@v4" "$workflow_file"; then
        echo -e "${GREEN}✅ Modern versions${NC}"
    elif grep -q "actions/checkout@v3\|actions/upload-artifact@v3" "$workflow_file"; then
        echo -e "${YELLOW}⚠ Older versions${NC}"
    else
        echo -e "${BLUE}➖ No standard actions${NC}"
    fi
    
    # Check for proper caching
    echo -n "  Rust caching... "
    if grep -q "Swatinem/rust-cache\|actions/cache.*cargo" "$workflow_file"; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${YELLOW}⚠ No Rust caching${NC}"
    fi
    
    # Check for continue-on-error usage
    echo -n "  Error handling... "
    continue_on_error_count=$(grep -c "continue-on-error: true" "$workflow_file" 2>/dev/null || echo "0")
    if [ "$continue_on_error_count" -eq 0 ]; then
        echo -e "${GREEN}✅ Strict${NC}"
    elif [ "$continue_on_error_count" -le 3 ]; then
        echo -e "${YELLOW}⚠ $continue_on_error_count uses${NC}"
    else
        echo -e "${RED}❌ Too many ($continue_on_error_count)${NC}"
        ISSUES_FOUND=$((ISSUES_FOUND + 1))
    fi
    
    # Check for security best practices
    echo -n "  Security practices... "
    security_issues=0
    
    # Check for hardcoded secrets
    if grep -i "password\|token\|secret\|key" "$workflow_file" | grep -v "secrets\." >/dev/null; then
        security_issues=$((security_issues + 1))
    fi
    
    # Check for proper permissions
    if grep -q "permissions:" "$workflow_file"; then
        echo -e "${GREEN}✅ Permissions defined${NC}"
    else
        echo -e "${YELLOW}⚠ No permissions${NC}"
    fi
    
    if [ "$security_issues" -eq 0 ]; then
        echo -n ""
    else
        echo -e "  ${RED}❌ $security_issues security issues${NC}"
        ISSUES_FOUND=$((ISSUES_FOUND + 1))
    fi
    
    VALID_WORKFLOWS=$((VALID_WORKFLOWS + 1))
    echo -e "  ${GREEN}✅ Workflow validation complete${NC}"
}

# Find and validate all workflow files
echo -e "\n${YELLOW}📂 Discovering GitHub Actions workflows...${NC}"

WORKFLOW_DIR=".github/workflows"
if [ ! -d "$WORKFLOW_DIR" ]; then
    echo -e "${RED}❌ No .github/workflows directory found${NC}"
    exit 1
fi

WORKFLOW_FILES=$(find "$WORKFLOW_DIR" -name "*.yml" -o -name "*.yaml" | sort)

if [ -z "$WORKFLOW_FILES" ]; then
    echo -e "${RED}❌ No workflow files found${NC}"
    exit 1
fi

echo -e "Found workflow files:"
for workflow in $WORKFLOW_FILES; do
    echo -e "  • $workflow"
done

# Validate each workflow
for workflow in $WORKFLOW_FILES; do
    validate_workflow "$workflow"
done

# Validate workflow dependencies
echo -e "\n${YELLOW}🔗 Checking workflow dependencies...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check for script dependencies
echo -n "  Validation scripts... "
SCRIPT_COUNT=0
for script in scripts/find-production-blockers.sh scripts/final-production-check.sh scripts/validate-neo-n3-completeness.sh scripts/validate-reference-implementation.sh scripts/run-all-validations.sh; do
    if [ -f "$script" ] && [ -x "$script" ]; then
        SCRIPT_COUNT=$((SCRIPT_COUNT + 1))
    fi
done

if [ "$SCRIPT_COUNT" -ge 4 ]; then
    echo -e "${GREEN}✅ $SCRIPT_COUNT/5 scripts available${NC}"
else
    echo -e "${RED}❌ Only $SCRIPT_COUNT/5 scripts available${NC}"
    ISSUES_FOUND=$((ISSUES_FOUND + 1))
fi

# Check for required tools
echo -n "  Required tools... "
REQUIRED_TOOLS=("cargo" "rustc" "python3")
MISSING_TOOLS=0

for tool in "${REQUIRED_TOOLS[@]}"; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        MISSING_TOOLS=$((MISSING_TOOLS + 1))
    fi
done

if [ "$MISSING_TOOLS" -eq 0 ]; then
    echo -e "${GREEN}✅ All tools available${NC}"
else
    echo -e "${YELLOW}⚠ $MISSING_TOOLS tools missing${NC}"
fi

# Workflow coverage analysis
echo -e "\n${YELLOW}📊 Workflow coverage analysis...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check coverage of different workflow types
coverage_areas=(
    "ci:Basic CI/CD"
    "security:Security auditing"
    "test:Test coverage"
    "release:Release automation"
    "production:Production validation"
)

for area in "${coverage_areas[@]}"; do
    workflow_type=${area%%:*}
    description=${area##*:}
    
    echo -n "  $description... "
    if find "$WORKFLOW_DIR" -name "*$workflow_type*" | grep -q .; then
        echo -e "${GREEN}✅ Covered${NC}"
    else
        echo -e "${YELLOW}⚠ Not covered${NC}"
    fi
done

# Summary
echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    GITHUB ACTIONS SUMMARY                     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${YELLOW}📋 VALIDATION RESULTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Total workflows: ${BLUE}$TOTAL_WORKFLOWS${NC}"
echo -e "Valid workflows: ${GREEN}$VALID_WORKFLOWS${NC}"
echo -e "Issues found: ${RED}$ISSUES_FOUND${NC}"

SUCCESS_RATE=$((VALID_WORKFLOWS * 100 / TOTAL_WORKFLOWS))
echo -e "Success rate: ${BLUE}$SUCCESS_RATE%${NC}"

# Final assessment
echo -e "\n${YELLOW}🎯 OVERALL ASSESSMENT${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $ISSUES_FOUND -eq 0 ] && [ $SUCCESS_RATE -eq 100 ]; then
    echo -e "${GREEN}🏆 EXCELLENT - All workflows are properly configured${NC}"
    echo "The GitHub Actions setup is production-ready with best practices."
    STATUS="EXCELLENT"
elif [ $ISSUES_FOUND -le 2 ] && [ $SUCCESS_RATE -ge 90 ]; then
    echo -e "${GREEN}✅ GOOD - Workflows are well configured with minor issues${NC}"
    echo "The GitHub Actions setup is production-ready with minor improvements needed."
    STATUS="GOOD"
elif [ $ISSUES_FOUND -le 5 ] && [ $SUCCESS_RATE -ge 75 ]; then
    echo -e "${YELLOW}⚠ FAIR - Workflows need some improvements${NC}"
    echo "The GitHub Actions setup works but has issues that should be addressed."
    STATUS="FAIR"
else
    echo -e "${RED}❌ POOR - Workflows have significant issues${NC}"
    echo "The GitHub Actions setup needs substantial improvements."
    STATUS="POOR"
fi

echo -e "\n${YELLOW}📋 RECOMMENDATIONS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$STATUS" = "EXCELLENT" ]; then
    echo "• Monitor workflow performance and update as needed"
    echo "• Consider advanced optimizations like matrix builds"
    echo "• Add more comprehensive integration tests"
elif [ "$STATUS" = "GOOD" ]; then
    echo "• Address the minor issues identified above"
    echo "• Update action versions to latest"
    echo "• Improve caching strategies"
elif [ "$STATUS" = "FAIR" ]; then
    echo "• Fix syntax errors in workflow files"
    echo "• Add proper error handling and security practices"
    echo "• Implement missing validation scripts"
else
    echo "• Complete rewrite of problematic workflows"
    echo "• Implement basic CI/CD best practices"
    echo "• Add proper validation and security checks"
fi

echo -e "\n${BLUE}Validation completed: $(date)${NC}"
echo -e "${BLUE}GitHub Actions Status: $STATUS${NC}"

# Exit code based on status
if [ "$STATUS" = "EXCELLENT" ] || [ "$STATUS" = "GOOD" ]; then
    exit 0
else
    exit 1
fi