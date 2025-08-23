#!/bin/bash

# Local Performance Benchmark Script
# Replicates the Performance Benchmark GitHub Actions workflow locally

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                LOCAL PERFORMANCE BENCHMARK                    ║${NC}"
echo -e "${BLUE}║           Neo N3 Framework Performance Validation             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Environment setup
export CARGO_TERM_COLOR=always

echo -e "\n${YELLOW}⚡ PERFORMANCE BENCHMARKS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Core framework build time
echo -e "${BLUE}Measuring core framework build time...${NC}"

START_TIME=$(date +%s.%N)
cargo build --workspace --release >/dev/null 2>&1 || {
    echo -e "${RED}❌ Build failed - measuring partial build time${NC}"
    cargo build -p neo-contract -p neo-compiler -p neo-contract-proc-macros --release >/dev/null 2>&1
}
END_TIME=$(date +%s.%N)

BUILD_TIME=$(echo "$END_TIME - $START_TIME" | bc -l)
BUILD_TIME_FORMATTED=$(printf "%.2f" $BUILD_TIME)

echo -e "Core framework build time: ${BLUE}${BUILD_TIME_FORMATTED}s${NC}"

# Individual component benchmarks
echo -e "\n${BLUE}Component-specific build times:${NC}"

components=("neo-contract" "neo-compiler" "neo-contract-proc-macros")

for component in "${components[@]}"; do
    echo -n "  $component... "
    
    START_COMP=$(date +%s.%N)
    if cargo build -p "$component" --release >/dev/null 2>&1; then
        END_COMP=$(date +%s.%N)
        COMP_TIME=$(echo "$END_COMP - $START_COMP" | bc -l)
        COMP_TIME_FORMATTED=$(printf "%.2f" $COMP_TIME)
        echo -e "${GREEN}${COMP_TIME_FORMATTED}s ✅${NC}"
    else
        echo -e "${RED}❌ Failed${NC}"
    fi
done

# Example compilation benchmarks
echo -e "\n${BLUE}Example compilation benchmarks:${NC}"

examples=("01-hello-world" "02-simple-storage" "03-counter")
total_example_time=0
successful_examples=0

for example in "${examples[@]}"; do
    if [ -d "examples/$example" ]; then
        echo -n "  $example... "
        
        START_EX=$(date +%s.%N)
        if (cd "examples/$example" && cargo build --release >/dev/null 2>&1); then
            END_EX=$(date +%s.%N)
            EX_TIME=$(echo "$END_EX - $START_EX" | bc -l)
            EX_TIME_FORMATTED=$(printf "%.2f" $EX_TIME)
            echo -e "${GREEN}${EX_TIME_FORMATTED}s ✅${NC}"
            total_example_time=$(echo "$total_example_time + $EX_TIME" | bc -l)
            successful_examples=$((successful_examples + 1))
        else
            echo -e "${RED}❌ Failed${NC}"
        fi
    fi
done

# Performance assessment
echo -e "\n${BLUE}Performance assessment:${NC}"

if (( $(echo "$BUILD_TIME < 15" | bc -l) )); then
    echo -e "  Build performance: ${GREEN}✅ Excellent (${BUILD_TIME_FORMATTED}s < 15s target)${NC}"
    PERF_SCORE=100
elif (( $(echo "$BUILD_TIME < 30" | bc -l) )); then
    echo -e "  Build performance: ${GREEN}✅ Good (${BUILD_TIME_FORMATTED}s < 30s target)${NC}"
    PERF_SCORE=80
elif (( $(echo "$BUILD_TIME < 60" | bc -l) )); then
    echo -e "  Build performance: ${YELLOW}⚠️ Acceptable (${BUILD_TIME_FORMATTED}s < 60s)${NC}"
    PERF_SCORE=60
else
    echo -e "  Build performance: ${RED}❌ Needs optimization (${BUILD_TIME_FORMATTED}s > 60s)${NC}"
    PERF_SCORE=40
fi

if [ "$successful_examples" -gt 2 ]; then
    echo -e "  Example builds: ${GREEN}✅ $successful_examples/$((${#examples[@]})) examples building${NC}"
else
    echo -e "  Example builds: ${YELLOW}⚠️ $successful_examples/$((${#examples[@]})) examples building${NC}"
fi

# Generate performance report
cat > LOCAL_PERFORMANCE_REPORT.md << EOF
# Local Performance Benchmark Report - $(date)

## Build Time Metrics

- **Total build time**: ${BUILD_TIME_FORMATTED}s
- **Performance score**: ${PERF_SCORE}/100
- **Successful examples**: $successful_examples/${#examples[@]}

## Component Build Times

$(for component in "${components[@]}"; do
    echo "- **$component**: Individual build time measured above"
done)

## Performance Assessment

$(if [ $PERF_SCORE -ge 80 ]; then
    echo "✅ **EXCELLENT** - Build performance meets enterprise standards"
elif [ $PERF_SCORE -ge 60 ]; then
    echo "✅ **GOOD** - Build performance is acceptable for development"
else
    echo "⚠️ **NEEDS IMPROVEMENT** - Consider build optimization"
fi)

## Recommendations

$(if [ $PERF_SCORE -ge 80 ]; then
    echo "- Maintain current build optimization"
    echo "- Monitor for performance regressions"
else
    echo "- Consider dependency optimization"
    echo "- Review incremental compilation settings"
    echo "- Optimize critical path compilation"
fi)
EOF

echo -e "${GREEN}✅ Performance report saved to LOCAL_PERFORMANCE_REPORT.md${NC}"

# Summary
echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║               LOCAL PERFORMANCE BENCHMARK COMPLETE            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${GREEN}⚡ PERFORMANCE SUMMARY${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "Build time: ${BLUE}${BUILD_TIME_FORMATTED}s${NC}"
echo -e "Performance score: ${BLUE}${PERF_SCORE}/100${NC}"
echo -e "Examples: ${BLUE}${successful_examples}/${#examples[@]}${NC} successful"

if [ $PERF_SCORE -ge 80 ]; then
    echo -e "\n✅ ${GREEN}Framework performance is excellent for enterprise deployment${NC}"
elif [ $PERF_SCORE -ge 60 ]; then
    echo -e "\n✅ ${GREEN}Framework performance is suitable for production use${NC}"
else
    echo -e "\n⚠️ ${YELLOW}Framework performance may benefit from optimization${NC}"
fi

echo -e "\n${BLUE}Local performance benchmark completed at $(date)${NC}"