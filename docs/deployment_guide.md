# Deploying Neo N3 Smart Contracts

This guide explains how to deploy Neo N3 smart contracts written in Rust using neo-contract-rs.

## Prerequisites

Before deploying, make sure you have:

- Built your smart contract (`.wasm` file)
- [Neo CLI](https://github.com/neo-project/neo-node/releases) installed
- A Neo N3 wallet with sufficient GAS for deployment

## Deployment Process

### Step 1: Compile your contract to WebAssembly

Build your Rust contract targeting WebAssembly:

```bash
cargo build --target wasm32-unknown-unknown --release
```

This will produce a `.wasm` file in `target/wasm32-unknown-unknown/release/`.

### Step 2: Convert WASM to NEF

Neo N3 uses the NEF (Neo Executable Format) for smart contracts. Convert your WASM file to NEF using the Neo compiler:

```bash
neo-compiler compile -i your_contract.wasm -o your_contract.nef
```

Note: The Neo compiler with WASM support is still under development. We recommend using the Neo CLI's built-in compiler for now.

### Step 3: Generate a Manifest File

Create a contract manifest file (`your_contract.manifest.json`) that defines the contract's properties, methods, and events:

```json
{
  "name": "YourContractName",
  "groups": [],
  "features": {},
  "supportedstandards": ["NEP-17"],
  "abi": {
    "methods": [
      {
        "name": "name",
        "parameters": [],
        "returntype": "String",
        "offset": 0,
        "safe": true
      },
      {
        "name": "symbol",
        "parameters": [],
        "returntype": "String",
        "offset": 0,
        "safe": true
      },
      {
        "name": "decimals",
        "parameters": [],
        "returntype": "Integer",
        "offset": 0,
        "safe": true
      },
      {
        "name": "totalSupply",
        "parameters": [],
        "returntype": "Integer",
        "offset": 0,
        "safe": true
      },
      {
        "name": "balanceOf",
        "parameters": [
          {
            "name": "account",
            "type": "Hash160"
          }
        ],
        "returntype": "Integer",
        "offset": 0,
        "safe": true
      },
      {
        "name": "transfer",
        "parameters": [
          {
            "name": "from",
            "type": "Hash160"
          },
          {
            "name": "to",
            "type": "Hash160"
          },
          {
            "name": "amount",
            "type": "Integer"
          },
          {
            "name": "data",
            "type": "Any"
          }
        ],
        "returntype": "Boolean",
        "offset": 0,
        "safe": false
      },
      {
        "name": "deploy",
        "parameters": [
          {
            "name": "data",
            "type": "Any"
          }
        ],
        "returntype": "Void",
        "offset": 0,
        "safe": false
      },
      {
        "name": "update",
        "parameters": [
          {
            "name": "nefFile",
            "type": "ByteArray"
          },
          {
            "name": "manifest",
            "type": "String"
          }
        ],
        "returntype": "Void",
        "offset": 0,
        "safe": false
      }
    ],
    "events": [
      {
        "name": "Transfer",
        "parameters": [
          {
            "name": "from",
            "type": "Hash160"
          },
          {
            "name": "to",
            "type": "Hash160"
          },
          {
            "name": "amount",
            "type": "Integer"
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
  "trusts": [],
  "extra": {
    "Author": "Your Name",
    "Email": "your.email@example.com",
    "Description": "A description of your contract"
  }
}
```

### Step 4: Deploy using Neo CLI

Open Neo CLI and deploy your contract using the following command:

```
deploy your_contract.nef your_contract.manifest.json
```

You will need to provide a wallet password and confirm the deployment. The deployment will cost GAS, which is the native utility token of the Neo blockchain.

### Step 5: Verify Deployment

After deployment, you can verify that your contract is deployed correctly by invoking its methods:

```
invoke <contract-hash> name []
```

Replace `<contract-hash>` with your contract's script hash.

## Using Neo Express (for Development)

For easier development and testing, you can use [Neo Express](https://github.com/neo-project/neo-express), which provides a simplified environment for Neo N3 development:

1. Create a Neo Express instance:
   ```
   neoxp create
   ```

2. Deploy your contract:
   ```
   neoxp deploy your_contract.nef your_contract.manifest.json
   ```

3. Invoke contract methods:
   ```
   neoxp contract invoke <contract-name> <method-name> [parameters]
   ```

## Conclusion

Deploying Neo N3 smart contracts involves compiling your Rust code to WebAssembly, converting it to the Neo Executable Format (NEF), creating a manifest file, and then deploying using the Neo CLI or Neo Express.

As the Neo N3 ecosystem evolves, more streamlined deployment tools specifically for Rust-based contracts are being developed. This guide will be updated as those tools become available.

## Troubleshooting

### Common Issues

- **Insufficient GAS**: Make sure your wallet has enough GAS to cover deployment costs.
- **Method not found**: Ensure your manifest file correctly lists all methods.
- **Stack overflow**: Your contract might be too complex or have infinite loops.

If you encounter issues, check the Neo N3 documentation or join the Neo community Discord for assistance.
