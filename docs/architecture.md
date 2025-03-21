# neo-contract-rs Architecture

This document describes the architecture of the neo-contract-rs framework for developing Neo N3 smart contracts in Rust.

## Overview

neo-contract-rs enables Rust developers to write Neo N3 smart contracts by providing a framework that:

1. Exposes Neo N3 blockchain features to Rust code
2. Compiles Rust code to WebAssembly (WASM) for execution in the Neo VM
3. Provides convenient abstractions and macros for contract development

## Components

The framework is structured into the following main components:

### 1. Core Library (`neo-contract`)

The core library provides all the essential building blocks for Neo contract development:

- **Contract Interface**: Traits and implementations for Neo contract standards
- **Types**: Rust implementations of Neo types (ByteString, H160, etc.)
- **Runtime**: Interface to Neo runtime functions and operations
- **Storage**: Abstractions for Neo's key-value storage
- **Events**: Mechanism for emitting and handling contract events
- **Crypto**: Cryptographic operations support
- **Serialization**: Data serialization/deserialization utilities
- **Environment**: Low-level syscalls and environment interfaces

### 2. Procedural Macros (`neo-contract-proc-macros`)

Procedural macros that simplify contract development by:

- **Contract Macro**: Generating necessary boilerplate for contract implementation
- **Struct Macros**: Providing serialization/deserialization for custom types

### 3. Examples

Example implementations demonstrating framework usage:

- **NEP-17 Token**: Basic fungible token implementation
- **Transfer**: Simple value transfer functionality

## Execution Flow

1. A Rust smart contract implementing the Neo contract interfaces is written
2. The contract is compiled to WebAssembly using the Rust toolchain
3. The WASM bytecode is deployed to the Neo N3 blockchain
4. When executed, the Neo VM interacts with the contract through syscalls exposed in the neo-contract library

## Key Interfaces

### Contract Declaration

Contracts are declared using procedural macros:

```rust
#[neo::contract]
impl Nep17Token for MyToken {
    // Implementation of NEP-17 interface
}
```

### Storage Operations

```rust
// Define storage context
let mut storage = StorageMap::new();

// Store value
storage.put(key, value);

// Retrieve value
let value = storage.get(key);
```

### Event Emission

```rust
// Emit an event
runtime::notify(event_name, event_data);
```

## Serialization Strategy

The framework uses a specialized serialization approach for Neo VM compatibility:

1. Rust types are converted to Neo-compatible types
2. Neo types are serialized to binary format for storage/transfer
3. Custom structs use the `#[neo::structs]` macro for automatic serialization

## WASM Compilation Targets

All contracts target the `wasm32-unknown-unknown` platform to produce WebAssembly output compatible with the Neo VM.

## Security Considerations

- Memory management is handled by WebAssembly
- Contract execution has gas limits enforced by Neo VM
- Type safety is enforced by Rust's type system
- Storage access is mediated through the storage abstraction layer 