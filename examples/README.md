# neo-contract-rs Examples

This directory contains example smart contracts built with the neo-contract-rs framework.

## Example Contracts

| Example | Description |
|---------|-------------|
| [hello-world](hello-world/) | A simple "Hello World" contract demonstrating basic functionality |
| [simple-storage](simple-storage/) | Basic key-value storage operations |
| [nep17](nep17/) | Implementation of the NEP-17 fungible token standard |
| [nep11-nft](nep11-nft/) | Implementation of the NEP-11 non-fungible token standard |
| [transfer](transfer/) | Simple value transfer functionality |
| [oracle-price-feed](oracle-price-feed/) | Example using the Oracle framework for external data |
| [documented-token](documented-token/) | Well-documented NEP-17 token demonstrating documentation best practices |

## Building Examples

Each example can be built using:

```bash
cd examples/[example-name]
cargo build --release --target wasm32-unknown-unknown
```

The resulting WASM file will be in `target/wasm32-unknown-unknown/release/[example_name].wasm`.

## Generating NEO Contract Files

After building, you can generate the NEO contract files using:

```bash
neo-wasm target/wasm32-unknown-unknown/release/[example_name].wasm
```

This will produce:
- `[example_name].nef` - The NEO Executable Format file
- `[example_name].manifest.json` - The contract manifest

## Learning Path

If you're new to neo-contract-rs, we recommend exploring the examples in this order:

1. **hello-world** - Learn the basic contract structure
2. **simple-storage** - Understand storage operations
3. **transfer** - See how to handle token transfers
4. **nep17** - Study fungible token implementation
5. **nep11-nft** - Explore non-fungible token implementation
6. **documented-token** - Learn documentation best practices
7. **oracle-price-feed** - Advanced usage with external data