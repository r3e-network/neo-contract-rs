#!/bin/bash

# Neo N3 Smart Contract Deployment Example Script
# This script demonstrates the process of compiling and deploying 
# a Neo N3 smart contract written in Rust

# Set variables
CONTRACT_NAME="nep11_nft"
OUTPUT_DIR="./target/neo-contract"
WASM_PATH="./target/wasm32-unknown-unknown/release/examples/${CONTRACT_NAME}.wasm"
NEF_PATH="${OUTPUT_DIR}/${CONTRACT_NAME}.nef"
MANIFEST_PATH="${OUTPUT_DIR}/${CONTRACT_NAME}.manifest.json"

# Optional: Neo RPC URL and wallet path for deployment
NEO_RPC_URL="http://localhost:10332"
WALLET_PATH="./wallet.json"
WALLET_PASS="password"

# Step 1: Ensure output directory exists
mkdir -p ${OUTPUT_DIR}

# Step 2: Build the Rust contract to WebAssembly
echo "Building ${CONTRACT_NAME} to WebAssembly..."
cargo build --release --target wasm32-unknown-unknown --example ${CONTRACT_NAME}

if [ $? -ne 0 ]; then
    echo "Error: Failed to build WebAssembly"
    exit 1
fi

echo "Successfully built WebAssembly: ${WASM_PATH}"

# Step 3: Compile WebAssembly to Neo N3 NEF using neo-compiler
echo "Compiling WebAssembly to Neo N3 NEF..."
neo-compiler compile -i ${WASM_PATH} -o ${OUTPUT_DIR}/${CONTRACT_NAME}

if [ $? -ne 0 ]; then
    echo "Error: Failed to compile to NEF"
    exit 1
fi

echo "Successfully created NEF file: ${NEF_PATH}"
echo "Successfully created manifest file: ${MANIFEST_PATH}"

# Step 4: Display contract hash (helpful for contract invocation)
echo "Computing contract hash..."
CONTRACT_HASH=$(neo-compiler hash -i ${NEF_PATH} -m ${MANIFEST_PATH})

echo "Contract hash: ${CONTRACT_HASH}"

# Step 5: Optional - Deploy to Neo N3 network
read -p "Do you want to deploy the contract to Neo N3 network? (y/n): " DEPLOY

if [ "$DEPLOY" = "y" ]; then
    echo "Deploying contract to Neo N3 network..."
    
    # Uncomment and adjust the command based on your Neo CLI/SDK setup
    # neo-cli deploy ${NEF_PATH} ${MANIFEST_PATH} --rpc-url ${NEO_RPC_URL} --wallet ${WALLET_PATH} --password ${WALLET_PASS}
    
    echo "Note: Update the deploy command with your actual Neo CLI command."
    echo "Deployment command commented out for safety."
    
    echo "For manual deployment using Neo GUI or CLI:"
    echo "1. NEF file: ${NEF_PATH}"
    echo "2. Manifest file: ${MANIFEST_PATH}"
else
    echo "Skipping deployment."
    echo "To deploy manually, use:"
    echo "1. NEF file: ${NEF_PATH}"
    echo "2. Manifest file: ${MANIFEST_PATH}"
fi

echo "Contract processing complete."
echo "-------------------------------------------------------"
echo "Contract: ${CONTRACT_NAME}"
echo "Hash: ${CONTRACT_HASH}"
echo "Files location: ${OUTPUT_DIR}"
echo "-------------------------------------------------------"
