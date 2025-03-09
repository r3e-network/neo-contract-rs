#!/bin/bash
# Script to make all build and test scripts executable

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."

echo "Making scripts executable..."

# Make all scripts in scripts directory executable
find "$SCRIPT_DIR" -name "*.sh" -type f -exec chmod +x {} \;

# Make test fixture generation scripts executable
find "$PROJECT_ROOT/neo-compiler/tests/fixtures" -name "*.sh" -type f -exec chmod +x {} \;
find "$PROJECT_ROOT/neo-compiler/tests/fixtures" -name "*.js" -type f -exec chmod +x {} \;

# Make the compiler binary executable if it exists
if [ -f "$PROJECT_ROOT/target/release/neo-compiler" ]; then
    chmod +x "$PROJECT_ROOT/target/release/neo-compiler"
    echo "Made neo-compiler executable"
fi

echo "All scripts are now executable!"
echo "You can now run build and test scripts directly."
echo "For example:"
echo "  ./scripts/build_project.sh"
echo "  ./scripts/run_tests.sh"
echo "  ./scripts/build_examples.sh"