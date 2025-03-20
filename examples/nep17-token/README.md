# NEP-17 Token Example

This example demonstrates how to implement a standard-compliant NEP-17 token using the Neo Contract Rust Framework.

## Structure

- `src/lib.rs`: The Rust implementation of the NEP-17 token contract using the `neo-contract` API
- `Cargo.toml`: The cargo manifest file for building the contract

## Features

- Implementation of all NEP-17 standard methods:
  - `name`
  - `symbol`
  - `decimals`
  - `totalSupply`
  - `balanceOf`
  - `transfer`
- Emitting the required `Transfer` event
- Initial token supply allocation

## Building

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Convert to NEF
neo-compiler -i target/wasm32-unknown-unknown/release/nep17_token.wasm \
             -o nep17_token.nef \
             -m nep17_token.manifest.json \
             --name "NEP17 Example Token"
```

## Usage

See the full documentation in `/docs/examples/nep17-token.md` for detailed information on:

- The NEP-17 standard requirements
- Implementation details
- Building and deploying the token
- Interacting with the deployed contract
- Best practices
- Extensions and customizations