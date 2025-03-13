# Neo N3 Contract Deployment Guide

This guide provides step-by-step instructions for deploying smart contracts written with the neo-contract-rs framework to the Neo N3 blockchain.

## Overview

Deploying a contract to Neo N3 involves several key steps:

1. Building the Rust contract to WebAssembly
2. Compiling the WebAssembly to Neo N3 bytecode and generating contract metadata
3. Deploying the compiled contract to a Neo N3 blockchain
4. Invoking and testing the deployed contract

## Prerequisites

Before deploying your contract, ensure you have:

- Installed the neo-contract-rs framework and neo-compiler
- Set up a Neo N3 node or have access to a Neo N3 network (MainNet, TestNet, or PrivateNet)
- Have sufficient GAS tokens for deployment (if deploying to MainNet or TestNet)
- Installed Neo CLI or Neo GUI for contract deployment

## Step 1: Building Your Rust Contract

First, build your Rust contract to WebAssembly:

```bash
# Navigate to your contract project
cd path/to/your-contract

# Build the contract targeting WebAssembly
cargo build --target wasm32-unknown-unknown --release
```

This will generate a WebAssembly (.wasm) file in the `target/wasm32-unknown-unknown/release/` directory.

## Step 2: Compiling to Neo N3 Format

Use the neo-compiler to convert the WebAssembly file to Neo N3 format:

```bash
# Create an output directory for the compiled files
mkdir -p build

# Compile the WebAssembly to Neo N3 format
neo-compiler compile target/wasm32-unknown-unknown/release/your_contract.wasm --output ./build
```

This command will generate two essential files:
- `your_contract.nef`: The Neo Executable Format file containing the contract bytecode
- `your_contract.manifest.json`: The contract manifest defining the contract's interface, permissions, and other metadata

### Understanding the Manifest

The manifest file contains crucial information about your contract:

```json
{
  "name": "YourContract",
  "groups": [],
  "abi": {
    "methods": [
      {
        "name": "method_name",
        "parameters": [
          {
            "name": "param_name",
            "type": "param_type"
          }
        ],
        "returntype": "return_type",
        "offset": 0,
        "safe": true
      }
    ],
    "events": [
      {
        "name": "event_name",
        "parameters": [
          {
            "name": "param_name",
            "type": "param_type"
          }
        ]
      }
    ]
  },
  "permissions": [
    {
      "contract": "*",
      "methods": "*"
    }
  ],
  "supportedstandards": ["NEP-17"],
  "trusts": [],
  "features": {},
  "extra": {
    "Description": "Your contract description",
    "Author": "Your Name"
  }
}
```

### Customizing the Manifest

For specialized deployment needs, you can modify the manifest:

```bash
# Edit the manifest before deployment
nano ./build/your_contract.manifest.json
```

Key fields to consider customizing:
- `permissions`: Define which contracts and methods your contract can invoke
- `supportedstandards`: Declare compliance with standards like NEP-17 or NEP-11
- `extra`: Add metadata like description, author, and version information

## Step 3: Deploying to Neo N3

### Using Neo CLI

Deploy your contract using Neo CLI:

```bash
# Start Neo CLI
./neo-cli

# Enter the Neo CLI console and open your wallet
open wallet path/to/your/wallet.json

# Deploy the contract
deploy ./build/your_contract.nef ./build/your_contract.manifest.json
```

The deploy command will:
1. Calculate the deployment gas cost
2. Ask for confirmation
3. Execute the deployment transaction
4. Return the contract hash if successful

Sample output:
```
Enter your password to continue...
Password: ********
Script hash: 0x0000000000000000000000000000000000000000
Gas consumed: 10.0
Deployment successful
```

### Using Neo GUI

If you prefer a graphical interface:

1. Open Neo GUI
2. Connect to your desired Neo N3 network
3. Open your wallet
4. Navigate to "Contract" > "Deploy Contract"
5. Select the .nef and manifest.json files
6. Review the deployment details and confirm

## Step 4: Verifying Deployment

Verify your contract deployment by invoking a simple method:

```bash
# In Neo CLI
invoke 0x0000000000000000000000000000000000000000 method_name []
```

Replace `0x0000000000000000000000000000000000000000` with your actual contract hash.

## Step 5: Advanced Deployment Options

### Multi-node Deployment

For production deployments, consider deploying to multiple consensus nodes:

1. Export your deployment transaction without sending:
```bash
deploy ./build/your_contract.nef ./build/your_contract.manifest.json --output deployment.tx
```

2. Sign the transaction with all required signers
3. Broadcast the signed transaction to the network

### Contract Upgrade

To upgrade an existing contract:

1. Prepare the new contract files (.nef and manifest.json)
2. Call the update method on your contract:
```bash
invoke 0x0000000000000000000000000000000000000000 update [bytearray:path/to/new.nef,bytearray:path/to/new.manifest.json]
```

This requires your contract to have an `update` method that calls the system `Contract.Update` method.

## Contract Deployment for Specific NEP Standards

### NEP-17 Token Deployment

For NEP-17 fungible tokens, ensure:

1. Your manifest includes: `"supportedstandards": ["NEP-17"]`
2. Your contract implements all required NEP-17 methods
3. Your deployment transaction has sufficient GAS

### NEP-11 NFT Deployment

For NEP-11 non-fungible tokens:

1. Include `"supportedstandards": ["NEP-11"]` in your manifest
2. Properly implement all required NEP-11 methods
3. Consider storage requirements for token metadata

## Best Practices for Deployment

### Security Measures

1. **Test thoroughly before deployment**: Use a private net or test net before main net
2. **Minimize permissions**: Only grant permissions your contract actually needs
3. **Use static analysis**: Scan your contract for vulnerabilities before deployment
4. **Audit critical contracts**: Have important contracts professionally audited

### Gas Optimization

1. **Optimize storage operations**: Storage operations are expensive in terms of GAS
2. **Mark read-only methods as safe**: Use the `#[safe]` attribute for gas optimization and to indicate read-only methods (note: `#[safe]` automatically makes the method available in the contract interface without needing an additional `#[method]` annotation)
3. **Batch operations**: Combine multiple operations where possible
4. **Remove debug code**: Remove any debug or test code before final deployment

### Deployment Checklist

Before deploying to MainNet:

- [ ] Code has been thoroughly tested
- [ ] Contract has been deployed and verified on TestNet
- [ ] Storage requirements have been analyzed
- [ ] Manifest permissions are correctly configured
- [ ] Supported standards are properly implemented
- [ ] Contract update mechanism is in place (if required)
- [ ] Sufficient GAS is available for deployment

## Troubleshooting Common Issues

### Deployment Failures

If deployment fails, check:

1. **Gas limit**: Ensure you have sufficient GAS for deployment
2. **Contract size**: Very large contracts may exceed size limits
3. **Manifest validity**: Ensure your manifest.json is properly formatted

### Method Invocation Issues

If contract methods fail:

1. **Check parameters**: Ensure parameters match the expected types
2. **Verify witness**: Authorization issues are common with witness-checking methods
3. **Storage access**: Ensure the contract has proper storage permissions

## Conclusion

Proper deployment of Neo N3 contracts is crucial for their successful operation on the blockchain. By following this guide, you can ensure that your contracts are deployed correctly, securely, and efficiently.

For more information on Neo N3 contract development, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Runtime Guide](./neo_n3_runtime_guide.md)
- [Neo N3 NEP-17 Guide](./neo_n3_nep17_guide.md)
- [Neo N3 NEP-11 Guide](./neo_n3_nep11_guide.md)
- [Neo N3 Security Guide](./neo_n3_security_guide.md)
