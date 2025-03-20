# Neo Macros Core

This crate provides declarative macros (`macro_rules!`) for Neo N3 smart contract development in Rust.

## Overview

The `neo-macros-core` crate contains all declarative macros for Neo N3 smart contract development, separated from procedural macros to respect Rust's technical limitations on proc-macro crates.

In the Neo smart contract framework, macros are split between two crates:
- `neo-macros`: Contains procedural macros (`#[contract]`, `#[event]`, etc.)
- `neo-macros-core`: Contains declarative macros (`emit_event!`, `profile!`, etc.)

## Macros

### Event Macros

- **`emit_event!`** - Emit an event with specified name and arguments
  ```rust
  emit_event!("Transfer", from_address, to_address, amount);
  ```

- **`implement_event!`** - Implement EventEmitter trait for a struct
  ```rust
  struct TransferEvent { from: H160, to: H160, amount: Int256 }
  implement_event!(TransferEvent, "Transfer");
  ```

- **`extend_event!`** - Extend an event type with StandardEventEmitter trait
  ```rust
  struct TransferEvent { from: H160, to: H160, amount: Int256 }
  extend_event!(TransferEvent);
  ```

### Profiling Macros

- **`profile!`** - Profile a function and return its result
  ```rust
  let result = profile!("my_function", {
      // Code to profile
      calculate_result()
  });
  ```

- **`profile_scope!`** - Create a profiling scope
  ```rust
  {
      profile_scope!("critical_section");
      // Code in this scope will be profiled
  } // Profile stops here automatically
  ```

- **`benchmark!`** - Benchmark a function execution
  ```rust
  let result = benchmark!("my_benchmark", 100, {
      // Code to benchmark
      calculate_result()
  });
  ```

## Usage

In most cases, you don't need to use this crate directly. The `neo-contract` crate re-exports all these macros for convenience:

```rust
use neo_contract::emit_event;

// Use the macro
emit_event!("Transfer", from, to, amount);
```

## Implementation Details

This crate is a pure macro crate without the `proc_macro = true` setting, allowing it to define `macro_rules!` macros which cannot be defined in proc-macro crates.

It has minimal dependencies and is specifically designed to complement the `neo-macros` crate (which contains the procedural macros).

## License

This project is licensed under the MIT License or Apache License 2.0. 