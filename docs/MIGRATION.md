# Migration from neo-contract-proc-macros to neo-macros

This document outlines the process of migrating from `neo-contract-proc-macros` to `neo-macros` for Neo N3 smart contract development.

## Background

The Neo Contract Rust Framework originally used two separate crates for procedural macros:

1. `neo-contract-proc-macros` - For core contract macros
2. `neo-macros` - For auxiliary macros

To simplify the framework architecture and maintenance, we have consolidated all macro functionality into a single crate: `neo-macros`.

## Current Status

The migration is a work in progress:

1. All procedural macros from `neo-contract-proc-macros` have been moved to `neo-macros`
2. The neo-contract/src/exports.rs file has been updated to re-export these macros
3. However, there are still integration issues with the macros that need to be resolved

## How to Use the Framework During Migration

During this migration period, we recommend:

1. **For new contracts:**
   - Use the patterns shown in `examples/hello_world` which don't rely on the attribute macros
   - Use the manual implementations as shown in `examples/hello_with_macros`

2. **For existing contracts using neo-contract-proc-macros:**
   - Update your Cargo.toml to depend on `neo-macros` instead of `neo-contract-proc-macros`
   - Update your imports to use macros from `neo_contract::exports::*` instead of directly from `neo_contract_proc_macros`
   - If you encounter issues with the macros, temporarily comment them out and use the manual implementation patterns

## Macro Status

| Macro | Status |
|-------|--------|
| `#[contract]` | Migrated but needs integration fixes |
| `#[event]` | Migrated but needs integration fixes |
| `#[index]` | Migrated but needs integration fixes |
| `#[storage]` | Migrated but needs integration fixes |
| `#[constructor]` | Migrated but needs integration fixes |
| `#[method]` | Migrated but needs integration fixes |
| `#[safe]` | Migrated but needs integration fixes |
| `#[no_reentrant]` | Migrated but needs integration fixes |
| `#[manifest_extra]` | Migrated but needs integration fixes |
| `#[supported_standards]` | Migrated but needs integration fixes |
| `#[contract_permission]` | Migrated but needs integration fixes |
| `#[contract_trust]` | Migrated but needs integration fixes |
| `#[contract_author]` | Migrated but needs integration fixes |
| `#[contract_description]` | Migrated but needs integration fixes |
| `#[contract_version]` | Migrated but needs integration fixes |

## Manual Implementation Patterns

For each macro, you can use the following manual patterns until the macros are fully integrated:

### Contract Module

```rust
// Instead of:
// #[contract]
// #[contract_author("Author Name")]
// #[contract_description("Description")]
// #[contract_version("1.0.0")]
mod my_contract {
    // Contract implementation
}

// Manual entry points:
#[no_mangle]
pub fn deploying() -> bool {
    true
}

#[no_mangle]
pub fn invoke(operation: String, args: Vec<Any>) -> Any {
    // Handle operations
}
```

### Events

```rust
// Instead of:
// #[event]
pub struct Transfer {
    // #[index]
    pub from: H160,
    // #[index]
    pub to: H160,
    pub amount: u64,
}

// Manual implementation:
impl Transfer {
    pub fn emit(from: H160, to: H160, amount: u64) {
        let mut event_args = Array::new();
        event_args.push(Any::from(from));
        event_args.push(Any::from(to));
        event_args.push(Any::from(amount));
        
        Runtime::notify(&ByteString::from("Transfer"), &event_args);
    }
}
```

### Storage

```rust
// Instead of:
// #[storage]
pub struct MyContract {
    pub counter: Item<u32>,
    pub owner: Item<H160>,
}

// Manual implementation:
pub struct MyContract {
    pub counter: Item<u32>,
    pub owner: Item<H160>,
}

impl MyContract {
    pub fn new() -> Self {
        Self {
            counter: Item::new(b"counter"),
            owner: Item::new(b"owner"),
        }
    }
    
    // Implement methods manually
}
```

## Future Work

We are working on the following improvements:

1. Fixing integration issues with the procedural macros
2. Improving the testing framework to validate macro behavior
3. Enhancing documentation with more examples
4. Creating a streamlined workflow for contract development

## Contributing

If you encounter issues with the macros or have suggestions for improvements, please:

1. Check the existing [issues](https://github.com/R3E-Network/neo-contract-rs/issues)
2. Create a new issue with detailed reproduction steps
3. Consider contributing a fix via a pull request 