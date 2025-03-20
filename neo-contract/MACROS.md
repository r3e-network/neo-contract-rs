# Neo Smart Contract Macros Architecture

This document outlines the architecture for organizing macros in the Neo smart contract framework.

## Current Status

The macros in the Neo Smart Contract framework are now properly organized into two distinct crates:

- `neo-macros`: Contains procedural macros (`proc_macro`) like `#[contract]`, `#[event]`, etc.
- `neo-macros-core`: Contains declarative macros (`macro_rules!`) like `emit_event!`, `profile!`, etc.

This separation resolves the technical limitation that declarative macros cannot be defined in proc-macro crates.

## Architecture Details

### 1. `neo-macros-core`

This crate contains all the declarative macros (`macro_rules!`) and has the following characteristics:

- **Not a proc-macro crate**: Does not have `proc-macro = true` in Cargo.toml
- **No direct dependencies on neo-contract**: To avoid circular dependencies
- **Re-exported by neo-contract**: Users don't need to import it directly

The crate provides macros such as:
- Event Macros: `emit_event!`, `implement_event!`, `extend_event!`
- Profiling Macros: `profile!`, `profile_scope!`, `benchmark!`

### 2. `neo-macros`

This crate contains all the procedural macros (attribute macros) and has the following characteristics:

- **Is a proc-macro crate**: Has `proc-macro = true` in Cargo.toml
- **Can depend on neo-contract**: For type information and utilities
- **Re-exported by neo-contract**: Users don't need to import it directly

The crate provides macros such as:
- `#[contract]`
- `#[event]`
- `#[method]`
- `#[constructor]`

### 3. `neo-contract`

The main crate that:
- Re-exports macros from both `neo-macros` and `neo-macros-core`
- Provides the core functionality for Neo Smart Contracts

## Usage Example

For the end-user, the usage remains simple due to re-exports:

```rust
use neo_contract::prelude::*;

#[event]
pub struct Transfer {
    pub from: Option<H160>,
    pub to: Option<H160>,
    pub amount: Int256,
}

// Later in code
emit_event!("Transfer", from, to, amount);
```

## Implementation Notes

1. The declarative macros in `neo-macros-core` reference types from `neo-contract` but the crate itself doesn't depend on `neo-contract` to avoid circular dependencies.

2. To prevent issues with circular imports, `neo-macros-core` uses fully qualified paths:
   ```rust
   ::neo_contract::types::builtin::array::Array::new()
   ```

3. In the `neo-contract` crate, we've removed the direct definitions of macros and replaced them with re-exports:
   ```rust
   pub use neo_macros_core::{
       emit_event, implement_event, extend_event,
       profile, profile_scope, benchmark
   };
   ```

## Benefits

- **Cleaner architecture**: Each macro type is in its appropriate crate type
- **Better maintainability**: Easier to update each macro type separately
- **Follows Rust best practices**: Respects the technical limitations of proc-macro crates
- **Simplified user experience**: All macros are re-exported from the main crate 