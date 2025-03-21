# Annotation-Based Manifest Generation

This document describes how to use code annotations to control the generation of NEO contract manifests in the neo-contract-rs framework.

## Overview

Neo N3 contract manifests define the contract's interface, permissions, and metadata. The neo-contract-rs framework now supports a robust annotation system that allows developers to:

1. Explicitly mark which methods should be exposed in the contract interface
2. Define method safety (read-only vs. state-changing)
3. Specify contract metadata, permissions, and supported standards
4. Control parameter and return type information

## Available Annotations

### Contract-Level Annotations

These annotations should be placed above the contract struct definition:

| Annotation | Description | Example |
|------------|-------------|---------|
| `#[contract_author("name")]` | Specifies the contract author | `#[contract_author("Neo Contract Team")]` |
| `#[contract_permission("contract:methods")]` | Defines contract permissions | `#[contract_permission("*:*")]` |
| `#[contract_standards("standard")]` | Declares supported standards | `#[contract_standards("NEP-17")]` |
| `#[contract_meta("key", "value")]` | Adds extra metadata to the manifest | `#[contract_meta("Version", "1.0.0")]` |

### Method-Level Annotations

These annotations should be placed above each method implementation:

| Annotation | Description | Example |
|------------|-------------|---------|
| `#[method]` | Marks a method to be exposed in the contract interface | `#[method]` |
| `#[safe]` | Indicates a method is read-only (doesn't modify state) | `#[safe]` |

## Usage Examples

### Basic Contract with Annotations

```rust
/// TokenContract is a sample NEP-17 token implementation
#[contract_author("Neo Contract Team")]
#[contract_permission("*:*")]
#[contract_standards("NEP-17")]
pub struct TokenContract;

#[neo::contract]
impl Nep17Token for TokenContract {
    #[method]
    #[safe]
    fn symbol() -> ByteString {
        ByteString::from_literal("DEMO")
    }

    #[method]
    #[safe]
    fn decimals() -> u32 {
        8
    }
    
    #[method]
    fn transfer(from: &Address, to: &Address, amount: BigInt) -> bool {
        // Implementation...
    }
}
```

### Custom Methods with Annotations

```rust
impl TokenContract {
    /// Returns information about the contract
    #[method]
    #[safe]
    fn contract_info() -> ByteString {
        ByteString::from_literal("Demo NEP-17 Token Contract")
    }
    
    /// Mints new tokens to the specified address
    #[method]
    fn mint(to: &Address, amount: BigInt) -> bool {
        // Implementation...
    }
}
```

## Annotation Processing

During compilation and manifest generation, the neo-contract-rs framework:

1. Analyzes the Rust source code to find annotated methods
2. Extracts parameter types, return types, and safety information
3. Identifies contract-level metadata from annotations
4. Produces a complete and accurate manifest file

## Parameter and Return Types

The manifest generator automatically analyzes parameter types and return types from method signatures:

- Common Rust types (`bool`, `u32`, etc.) are mapped to appropriate NEO parameter types
- Complex types like `ByteString`, `Address`, and `BigInt` are properly recognized
- Container types like `Vec<T>` are mapped to array types
- References (`&T`) are processed correctly

## Safety Rules

A method is considered safe (read-only) if any of the following conditions are met:

1. It has the `#[safe]` annotation
2. It has `@safe` in its doc comments
3. The method name follows safe naming conventions:
   - Starts with `get_`, `query_`, `balance_`, or `total_`
   - Is named `symbol`, `decimals`, `totalSupply`, `balanceOf`, or `contract_info`

## Permission Format

The `#[contract_permission("")]` annotation uses the format:

```
"contract:method1,method2,..."
```

Where:
- `contract` is the contract hash or `*` for any contract
- `method1,method2,...` are specific methods or `*` for all methods

Examples:
- `#[contract_permission("*:*")]` - Allow calling any method on any contract
- `#[contract_permission("0x1234...:transfer,balanceOf")]` - Allow calling specific methods on a specific contract

## Standards Detection

The framework can automatically detect standard implementations if you implement the corresponding traits:

- **NEP-17 (Fungible Token)**: Detected if you implement the `Nep17Token` trait
- **NEP-11 (Non-Fungible Token)**: Detected if you implement the `Nep11Token` trait

Alternatively, you can explicitly declare supported standards with the `#[contract_standards("NEP-17")]` annotation.

## Preserving Offsets

When updating an existing manifest, method offsets are preserved to maintain compatibility with the compiled NEF file.

## Best Practices

1. **Always use `#[method]`** to explicitly mark which methods should be exposed
2. **Mark read-only methods** with `#[safe]` to ensure they're properly identified
3. **Use descriptive doc comments** to document your contract and methods
4. **Explicitly declare standards** with `#[contract_standards("")]` for clarity
5. **Be precise with permissions** to follow the principle of least privilege

## Example Contract

See the `examples/annotated-contract` directory for a complete example of a contract using the annotation system.

## Compatibility

The annotation-based system is compatible with the existing manifest generation approach and will correctly process methods with or without annotations. However, using the explicit annotations is recommended for clarity and precision.

## Related Documentation

- [NEO Contract Manifest Reference](https://docs.neo.org/docs/en-us/reference/scapi/framework/native/Contract/Manifest.html)
- [NEP-17 Token Standard](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki)
- [NEP-11 NFT Standard](https://github.com/neo-project/proposals/blob/master/nep-11.mediawiki) 