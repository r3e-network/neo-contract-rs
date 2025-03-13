# Ink-Style NEP-17 Token Example

This example demonstrates how to implement a NEP-17 compatible token using the ink!-style contract definition approach in the Neo N3 smart contract framework.

## Features

- NEP-17 compatibility (transfer, balanceOf, totalSupply, etc.)
- Ink!-style contract definition using macros (`#[contract]`, `#[method]`, etc.)
- Use of `#[safe]` attribute to mark read-only methods
- Event emitting through custom event functions
- Storage handling with the Storage trait

## Safety in Neo N3 Contracts

Neo N3 contracts distinguish between:

- **Safe methods**: Read-only methods that cannot modify contract storage (`&self` methods)
- **Non-safe methods**: Methods that can potentially modify storage (`&mut self` methods)

In this example, all methods that only read from storage are marked with the `#[safe]` attribute, which gets translated into the `"safe": true` flag in the manifest file.

```rust
// Example of a safe (read-only) method
#[method]
#[safe]
pub fn balance_of(&self, account: H160) -> Int256 {
    // Read-only implementation
}

// Example of a non-safe (state-modifying) method
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
    // State-modifying implementation
}
```

Safe methods can be executed without requiring a full verification from the blockchain, making them more efficient to call.

## Building the Contract

To build the contract, run:

```bash
# Debug build
cargo build --target wasm32-unknown-unknown

# Release build (recommended for deployment)
cargo build --target wasm32-unknown-unknown --release
```

The WebAssembly file will be generated at `target/wasm32-unknown-unknown/release/ink_style_complete.wasm`.

## Deployment

### Prerequisites

- [Neo CLI](https://github.com/neo-project/neo-node/releases) installed
- A Neo N3 wallet with sufficient GAS for deployment
- (Optionally) [Neo Express](https://github.com/neo-project/neo-express) for development

### Deployment Steps

1. **Convert WASM to NEF**

   Convert your WASM file to NEF (Neo Executable Format) using the Neo compiler:

   ```bash
   neo-compiler compile -i target/wasm32-unknown-unknown/release/ink_style_complete.wasm -o ink_style_complete.nef
   ```

2. **Create a Manifest File**

   Create a `ink_style_complete.manifest.json` file for your contract. The manifest should include all the methods and events defined in your contract:

   ```json
   {
     "name": "InkStyleToken",
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
             }
           ],
           "returntype": "Boolean",
           "offset": 0,
           "safe": false
         },
         {
           "name": "mint",
           "parameters": [
             {
               "name": "to",
               "type": "Hash160"
             },
             {
               "name": "amount",
               "type": "Integer"
             }
           ],
           "returntype": "Boolean",
           "offset": 0,
           "safe": false
         },
         {
           "name": "burn",
           "parameters": [
             {
               "name": "from",
               "type": "Hash160"
             },
             {
               "name": "amount",
               "type": "Integer"
             }
           ],
           "returntype": "Boolean",
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
       "Author": "Ink Style Example",
       "Email": "example@neo-contract-rs.dev",
       "Description": "NEP-17 token using ink!-style contract definition"
     }
   }
   ```

3. **Deploy using Neo CLI**

   ```bash
   deploy ink_style_complete.nef ink_style_complete.manifest.json
   ```

4. **Deploy using Neo Express (for development)**

   ```bash
   neoxp deploy ink_style_complete.nef ink_style_complete.manifest.json
   ```

## Testing

After deployment, you can test your contract by invoking its methods:

```bash
# Using Neo CLI
invoke <contract-hash> name []
invoke <contract-hash> symbol []
invoke <contract-hash> totalSupply []

# Using Neo Express
neoxp contract invoke InkStyleToken name []
neoxp contract invoke InkStyleToken symbol []
neoxp contract invoke InkStyleToken totalSupply []
```

To transfer tokens:

```bash
# Get your wallet address
wallet list

# Transfer tokens from your wallet to another wallet
invoke <contract-hash> transfer [<your-address-hash>, <recipient-address-hash>, 100]
```

## Troubleshooting

- **Insufficient GAS**: Make sure your wallet has enough GAS to cover deployment costs.
- **Method not found**: Ensure your manifest file correctly lists all methods.
- **Incorrect Parameters**: Double-check your parameter types when invoking methods.

If you encounter issues, check the Neo N3 documentation or join the Neo community for assistance.
