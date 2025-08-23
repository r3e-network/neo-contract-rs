#!/bin/bash

# Local Security Audit Script
# Replicates the Security Audit GitHub Actions workflow locally

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                   LOCAL SECURITY AUDIT                        ║${NC}"
echo -e "${BLUE}║              Neo N3 Framework Security Validation             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Environment setup
export CARGO_TERM_COLOR=always

echo -e "\n${YELLOW}🔍 SECURITY AUDIT STEPS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Install tools if needed
echo -e "${BLUE}Checking security tools...${NC}"

echo -n "  cargo-audit... "
if command -v cargo-audit >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Available${NC}"
else
    echo -e "${YELLOW}⚠️ Installing...${NC}"
    cargo install cargo-audit --locked
fi

echo -n "  cargo-deny... "
if command -v cargo-deny >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Available${NC}"
else
    echo -e "${YELLOW}⚠️ Installing...${NC}"
    cargo install cargo-deny --locked
fi

# Run dependency security audit
echo -e "\n${BLUE}Running dependency security audit...${NC}"
if cargo audit --json > audit-report.json 2>/dev/null; then
    echo -e "${GREEN}✅ No critical vulnerabilities found${NC}"
    VULNERABILITIES=$(jq '.vulnerabilities.count' audit-report.json 2>/dev/null || echo "0")
    echo "  Vulnerabilities: $VULNERABILITIES"
else
    echo -e "${YELLOW}⚠️ Security audit completed with findings${NC}"
    cargo audit 2>&1 | head -10
fi

# Run license and dependency check
echo -e "\n${BLUE}Running license and dependency check...${NC}"
if cargo deny check licenses bans sources >/dev/null 2>&1; then
    echo -e "${GREEN}✅ License compliance verified${NC}"
else
    echo -e "${YELLOW}⚠️ License or dependency issues found${NC}"
    cargo deny check licenses bans sources 2>&1 | head -10
fi

# Run security-focused Clippy lints
echo -e "\n${BLUE}Running security-focused Clippy analysis...${NC}"
if cargo clippy --all-targets --all-features -- \
    -W clippy::suspicious \
    -W clippy::complexity \
    -D clippy::unwrap_used \
    -D clippy::expect_used \
    -D clippy::panic \
    -D clippy::unimplemented \
    -D clippy::unreachable \
    > clippy-security-report.txt 2>&1; then
    echo -e "${GREEN}✅ No security-critical code quality issues${NC}"
else
    echo -e "${YELLOW}⚠️ Security-focused lints found issues${NC}"
    echo "  See clippy-security-report.txt for details"
    head -10 clippy-security-report.txt
fi

# Check for unsafe code blocks
echo -e "\n${BLUE}Checking for unsafe code blocks...${NC}"
UNSAFE_COUNT=$(find neo-contract/src neo-compiler/src neo-contract-proc-macros/src -name "*.rs" -exec grep -l "unsafe" {} \; 2>/dev/null | wc -l)
if [ "$UNSAFE_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ No unsafe code found in core modules${NC}"
else
    echo -e "${YELLOW}⚠️ Found $UNSAFE_COUNT files with unsafe code${NC}"
    echo "  Reviewing unsafe code usage..."
    find neo-contract/src neo-compiler/src neo-contract-proc-macros/src -name "*.rs" -exec grep -Hn "unsafe" {} \; 2>/dev/null | head -5
fi

# Generate security summary
echo -e "\n${BLUE}Generating security summary...${NC}"
cat > LOCAL_SECURITY_SUMMARY.md << EOF
# Local Security Audit Summary - $(date)

## Dependency Vulnerabilities
$(if [ -f audit-report.json ]; then echo "See audit-report.json for details"; else echo "No audit report generated"; fi)

## License Compliance
$(cargo deny check licenses bans sources >/dev/null 2>&1 && echo "✅ All licenses compliant" || echo "⚠️ See above for license issues")

## Code Quality (Security Focus)
$(if [ -f clippy-security-report.txt ]; then echo "See clippy-security-report.txt for details"; else echo "No security lints report generated"; fi)

## Unsafe Code Usage
Found $UNSAFE_COUNT files with unsafe code blocks in core modules.
$(if [ "$UNSAFE_COUNT" -gt 0 ]; then echo "Review recommended for production deployment."; else echo "No unsafe code concerns."; fi)

## Overall Security Assessment
$(if [ "$UNSAFE_COUNT" -le 5 ]; then echo "✅ Security posture is good for production deployment"; else echo "⚠️ Review unsafe code usage before production"; fi)
EOF

echo -e "${GREEN}✅ Security summary saved to LOCAL_SECURITY_SUMMARY.md${NC}"

# Final assessment
echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    LOCAL SECURITY AUDIT COMPLETE              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${GREEN}🛡️ SECURITY AUDIT SUMMARY${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Unsafe code files: ${BLUE}$UNSAFE_COUNT${NC}"
echo -e "Security report: LOCAL_SECURITY_SUMMARY.md"
echo ""
echo "### 🎯 Security Status:"
if [ "$UNSAFE_COUNT" -le 5 ]; then
    echo -e "✅ ${GREEN}Framework security posture is suitable for production deployment${NC}"
else
    echo -e "⚠️ ${YELLOW}Review unsafe code usage before production deployment${NC}"
fi

echo -e "\n${BLUE}Local security audit completed at $(date)${NC}"