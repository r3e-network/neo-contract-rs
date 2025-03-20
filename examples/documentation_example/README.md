# Documentation Example Contract

This is an example contract that demonstrates the current recommended syntax and macros for Neo N3 contract development using Rust.

## Features Demonstrated

- Event definition with `#[neo_contract::event]` and field indexing with `#[index]`
- Contract structure with `#[neo_contract::contract]`
- Storage fields with `#[storage]`
- Constructor with `#[constructor]`
- Read-only methods with `#[safe]`
- State-changing methods with `#[method]`
- Reentrancy protection with `#[no_reentry]`
- Event emission with `.notify()`

## Contract Overview

This contract is a simple NEP-17 compatible token with the following features:

- Token creation with initial supply
- Token transfers
- Balance queries
- Minting (admin only)
- Burning

## Building

```bash
cd examples/documentation_example
cargo build --release
```

## Testing

```bash
cd examples/documentation_example
cargo test
```

## Deploying

See the main project documentation for deployment instructions.