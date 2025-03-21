# NEO Contract Manifest Generation

This document describes how the manifest generation works in the neo-contract-rs framework.

## Overview

The NEO blockchain requires each smart contract to have an accompanying manifest file that defines the contract's metadata, permissions, and interface. This manifest is crucial for the NEO virtual machine to properly interact with the contract.

In neo-contract-rs, we automatically generate this manifest from:
1. The compiled WebAssembly (WASM) contract file
2. The Rust source code that defines the contract
3. The doc comments in the Rust code

## Manifest Generation Process

### 1. Internal Manifest Creation

First, an internal manifest is created during the compilation process by analyzing the WASM module:

- Exported functions are identified
- Parameter and return types are extracted
- Function names are processed

### 2. Rust Source Analysis

To enhance the manifest with human-readable documentation:

- The system locates the Rust source file associated with the contract
- Documentation comments are extracted for contract methods
- The contract's overall description is identified
- Safety annotations (`@safe`) are detected

### 3. NEO Manifest Transformation

The internal manifest is then transformed into the standard NEO manifest format:

- Methods are converted with appropriate parameter and return types
- Method safety (read-only status) is determined based on `@safe` annotations and naming conventions
- Standard detection identifies if the contract implements NEP-17 (fungible token) or NEP-11 (non-fungible token) standards
- Documentation is added to the manifest's "Extra" field

## Documentation Extraction

The manifest generator extracts documentation in the following ways:

1. **Contract Description**: Extracted from the doc comment on the contract's impl block
2. **Method Descriptions**: Extracted from doc comments on each exported method
3. **Parameter Names**: Obtained from the parameter names in the Rust function declarations
4. **Safety Information**: Extracted from `@safe` annotations in method doc comments

## Method Safety Determination

Methods are classified as "safe" (read-only) based on the following criteria, in order of precedence:

1. **`@safe` Annotation**: Methods with the `@safe` annotation in their doc comments are marked as safe
2. **Naming Conventions**: If no `@safe` annotation exists, methods are marked as safe based on these naming conventions:
   - Methods starting with `get_`, `query_`, `balance_`, or `total_`
   - Methods named `symbol`, `decimals`, or `contract_info`
3. **Default**: All other methods are considered potentially state-changing and marked as "unsafe"

Example of a safe method with annotation:

```rust
/// Returns the token symbol
/// 
/// @safe
pub fn symbol() -> String {
    "TOKEN".to_string()
}
```

## Standard Detection

The manifest generator automatically detects if a contract implements standard interfaces:

### NEP-17 (Fungible Token)
Detected when at least 3 of these methods are found:
- `transfer`
- `balanceOf`
- `totalSupply`
- `decimals`
- `symbol`

### NEP-11 (Non-Fungible Token)
Detected when any of these methods are found:
- `ownerOf`
- `tokens`
- `tokenURI`
- `transfer` (with specific signature)

## Manual Override

While the automatic generation should work well for most cases, developers can:

1. Add comprehensive documentation in their Rust code to enhance the generated manifest
2. Manually edit the generated manifest file if specific customization is needed
3. Use `@safe` annotations to explicitly mark methods as read-only

## Best Practices

For optimal manifest generation:

1. Add detailed doc comments to your contract impl block and exported methods
2. Follow naming conventions for read-only methods
3. Use `@safe` annotations for all read-only methods
4. Implement standard interfaces completely to ensure proper detection
5. Review the generated manifest file before deployment
6. Follow the [Code Documentation Style Guide](code-documentation-style.md) for consistent documentation

## Related Documentation

- [Documentation Best Practices](documentation-best-practices.md)
- [Understanding NEO Manifests](understanding-neo-manifests.md)
- [Code Documentation Style](code-documentation-style.md)
- [Efficient Smart Contracts](efficient-contracts.md)