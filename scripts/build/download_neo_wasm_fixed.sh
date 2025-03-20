#!/bin/bash

# Exit on any error
set -e

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"
BIN_DIR="$PROJECT_ROOT/bin"

# Create bin directory if it doesn't exist
mkdir -p "$BIN_DIR"

# Version to download - updated to the latest version
VERSION="v0.4.0"

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

# Construct filename - updated to match the actual release asset naming
FILENAME="neo-contract-${VERSION}-${SYSTEM}-${ARCH}.tar.gz"

# GitHub repository - updated to the correct organization
REPO="neo-project/neo-contract-rs"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"

echo "Downloading neo-wasm ${VERSION} for ${SYSTEM}-${ARCH}..."
echo "URL: ${DOWNLOAD_URL}"

# Create temporary directory
TEMP_DIR=$(mktemp -d)
TEMP_FILE="${TEMP_DIR}/${FILENAME}"

# Download the file
if command -v curl &> /dev/null; then
    curl -L -o "${TEMP_FILE}" "${DOWNLOAD_URL}" || {
        echo "Failed to download using curl. Checking available releases..."
        curl -s "https://api.github.com/repos/${REPO}/releases" | grep "browser_download_url.*${SYSTEM}-${ARCH}" | head -n 1
        exit 1
    }
elif command -v wget &> /dev/null; then
    wget -O "${TEMP_FILE}" "${DOWNLOAD_URL}" || {
        echo "Failed to download using wget. Checking available releases..."
        curl -s "https://api.github.com/repos/${REPO}/releases" | grep "browser_download_url.*${SYSTEM}-${ARCH}" | head -n 1
        exit 1
    }
else
    echo "Error: Neither curl nor wget is available. Please install one of them."
    exit 1
fi

# Extract to bin directory
echo "Extracting..."
tar -xzf "${TEMP_FILE}" -C "${TEMP_DIR}" || {
    echo "Extraction failed. Checking file contents:"
    file "${TEMP_FILE}"
    exit 1
}

# Find the executable - look for both neo-wasm and neo-contract executables
NEO_WASM_EXEC=$(find "${TEMP_DIR}" -name "neo-wasm" -o -name "neo-contract" -type f | head -n 1)

if [ -z "$NEO_WASM_EXEC" ]; then
    echo "Error: Could not find neo-wasm or neo-contract executable in the downloaded package."
    echo "Listing package contents:"
    find "${TEMP_DIR}" -type f
    exit 1
fi

# Get the base name of the executable
EXEC_BASENAME=$(basename "${NEO_WASM_EXEC}")

# Copy to bin directory
cp "${NEO_WASM_EXEC}" "${BIN_DIR}/${EXEC_BASENAME}"
chmod +x "${BIN_DIR}/${EXEC_BASENAME}"

# Create symlink from neo-wasm to neo-contract if needed
if [ "${EXEC_BASENAME}" = "neo-contract" ] && [ ! -f "${BIN_DIR}/neo-wasm" ]; then
    ln -sf "${BIN_DIR}/neo-contract" "${BIN_DIR}/neo-wasm"
    echo "Created symlink from neo-wasm to neo-contract"
fi

# Clean up
rm -rf "${TEMP_DIR}"

echo "${EXEC_BASENAME} has been downloaded and placed in ${BIN_DIR}/${EXEC_BASENAME}"
echo "Version information:"
"${BIN_DIR}/${EXEC_BASENAME}" --version || "${BIN_DIR}/${EXEC_BASENAME}" -version 