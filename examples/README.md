# NEO Contract Rust Examples

This directory contains example smart contracts for the NEO N3 blockchain using the neo-contract-rs framework.

## Current Examples

1. **Hello World**: A simple example showing basic contract structure and storage concepts.
2. **DAO**: A decentralized autonomous organization example showing more complex contract logic.

## Contract Syntax

The examples use attribute macros to define contract structure, events, and methods. Here's a guide to the syntax used in these examples:

### Contract Attributes

```rust
#[contract]
#[contract_author("Your Name")]
#[contract_description("Description of your contract")]
#[contract_version("1.0.0")]
#[supported_standards("NEP-17")]  // Optional: for token contracts
mod my_contract {
    // Contract code goes here
}
```

### Events

```rust
#[event]
struct MyEvent {
    #[index]  // Optional: indexed fields can be searched for efficiently
    field1: H160,
    field2: String,
}
```

### Storage

```rust
#[storage]
struct MyContract {
    counter: Item<u32>,
    balances: StorageMap<H160, u32>,
}
```

### Methods

```rust
impl MyContract {
    // Constructor - called when contract is deployed
    #[constructor]
    fn new() -> Self {
        // Initialize contract
    }
    
    // Public method - can be called by other contracts and users
    #[method]
    fn some_method(&mut self, param1: H160) -> bool {
        // Method implementation
    }
    
    // Read-only method - doesn't modify state
    #[safe]
    fn get_value(&self, key: H160) -> u32 {
        // Read-only implementation
    }
}
```

## Event Emission

To emit events in your contract, use the `Runtime::notify` method:

```rust
let mut event_args = Array::new();
event_args.push(Any::from(param1));
event_args.push(Any::from(param2));
Runtime::notify(&ByteString::from("EventName"), &event_args);
```

## Building Examples

To build an example contract:

```bash
cd examples/hello_world
cargo build --release --target wasm32-unknown-unknown
```

To compile the WebAssembly to NEO VM bytecode:

```bash
../target/release/neo-compiler compile target/wasm32-unknown-unknown/release/hello_world.wasm
```

## Documentation

For more details about NEO Contract Rust development, see the following documentation:

- [Development Guide](../docs/DEVELOPMENT-GUIDE.md)
- [Fixed Issues](../docs/FIXED-ISSUES.md)
- [Future Improvements](../docs/FUTURE-IMPROVEMENTS.md)
