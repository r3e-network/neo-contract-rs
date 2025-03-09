#!/bin/bash

# Exit on any error
set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"
BIN_DIR="$PROJECT_ROOT/bin"

# Create bin directory if it doesn't exist
mkdir -p "$BIN_DIR"

# Version to download
VERSION="v0.1.0"

# Determine system and architecture
SYSTEM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture to expected format
if [ "$ARCH" == "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" == "aarch64" ]; then
    ARCH="arm64"
elif [ "$ARCH" == "armv7l" ]; then
    ARCH="arm"
fi

# Construct filename
FILENAME="neo-wasm-${VERSION}-${SYSTEM}-${ARCH}.tar.gz"

# GitHub repository
REPO="neo-project/neo-contract-rs"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"

echo "Downloading neo-wasm ${VERSION} for ${SYSTEM}-${ARCH}..."
echo "URL: ${DOWNLOAD_URL}"

# Create temporary directory
TEMP_DIR=$(mktemp -d)
TEMP_FILE="${TEMP_DIR}/${FILENAME}"

# Download the file
if command -v curl &> /dev/null; then
    curl -L -o "${TEMP_FILE}" "${DOWNLOAD_URL}"
elif command -v wget &> /dev/null; then
    wget -O "${TEMP_FILE}" "${DOWNLOAD_URL}"
else
    echo "Error: Neither curl nor wget is available. Please install one of them."
    exit 1
fi

# Extract to bin directory
echo "Extracting..."
tar -xzf "${TEMP_FILE}" -C "${TEMP_DIR}"

# Find the executable
NEO_WASM_EXEC=$(find "${TEMP_DIR}" -name "neo-wasm" -type f)

if [ -z "$NEO_WASM_EXEC" ]; then
    echo "Error: Could not find neo-wasm executable in the downloaded package."
    exit 1
fi

# Copy to bin directory
cp "${NEO_WASM_EXEC}" "${BIN_DIR}/neo-wasm"
chmod +x "${BIN_DIR}/neo-wasm"

# Clean up
rm -rf "${TEMP_DIR}"

echo "neo-wasm has been downloaded and placed in ${BIN_DIR}/neo-wasm"
echo "Version information:"
"${BIN_DIR}/neo-wasm" -version
