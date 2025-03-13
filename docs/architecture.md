# Neo N3 Contract Framework Architecture

This document outlines the architecture of the Neo N3 contract development framework for Rust, explaining how the components work together to enable smart contract development for the Neo N3 blockchain.

## Overview

The framework consists of three main components that work together to provide a comprehensive solution for Neo N3 smart contract development in Rust:

1. **neo-contract**: The core library providing the contract development API
2. **neo-compiler**: The compiler that converts WebAssembly to Neo VM bytecode
3. **neo-macros**: Procedural macros for the attribute-based interface

## Component Details

### neo-contract

The `neo-contract` crate provides the core API for writing Neo N3 smart contracts in Rust. It offers:

- Type definitions for Neo N3 primitives (Address, ByteString, etc.)
- Storage abstractions for managing contract state
- Runtime interactions with the Neo N3 blockchain
- Event emission utilities
- Security features and standard implementations

Key modules include:

- **prelude**: Common imports for contract development
- **storage**: Storage primitives (StorageItem, StorageMap)
- **types**: Neo N3-specific type definitions
- **env**: Low-level interaction with the Neo VM
- **contracts**: Standard contract implementations and helpers

### neo-compiler

The `neo-compiler` is responsible for converting Rust/WebAssembly code into Neo VM bytecode. It performs:

1. WebAssembly parsing and analysis
2. WASM to Neo VM bytecode conversion
3. Intermediate representation optimization
4. NEF file and manifest generation

The compiler ensures that Rust contracts are correctly translated to run on the Neo N3 blockchain, handling:

- Memory management differences
- Call conventions
- Stack operations
- Contract ABIs
- Syscall mappings

### neo-macros

The `neo-macros` crate provides procedural macros that simplify contract development:

- `#[contract]`: Marks a module as a smart contract
- `#[storage]`: Defines the contract storage structure
- `#[method]`: Exposes a function as a contract method
- `#[safe]`: Marks methods as read-only for optimized execution
- `#[initialize]`: Designates the contract initialization method

## Compilation Flow

The compilation process follows these steps:

1. **Rust Compilation**: Rust code is compiled to WebAssembly using the standard Rust toolchain
2. **WebAssembly Processing**: The WebAssembly binary is optimized and prepared for Neo VM
3. **Neo VM Translation**: The WebAssembly is converted to Neo VM bytecode
4. **NEF Generation**: A NEF file and contract manifest are generated for deployment

```
Rust Source → WASM Binary → Neo VM Bytecode → NEF File + Manifest
```

## Contract Lifecycle

### Development

1. Define contract structure using the `#[contract]` and `#[storage]` attributes
2. Implement contract methods with appropriate attributes (`#[method]`, `#[safe]`)
3. Use storage abstractions to manage contract state
4. Emit events using the `Runtime::notify` method

### Compilation

1. Compile the contract to WebAssembly
2. Use `neo-compiler` to convert WebAssembly to Neo VM bytecode
3. Generate NEF file and manifest

### Deployment

1. Deploy the NEF file and manifest to the Neo N3 blockchain
2. Initialize the contract (calls the method marked with `#[initialize]`)

### Execution

1. Users interact with the contract through method calls
2. Safe methods (marked with `#[safe]`) can be called without modifying state
3. State-modifying methods update the contract storage
4. Events are emitted to notify external systems

## Key Design Decisions

### Safe Methods

The framework properly distinguishes between safe (read-only) and non-safe (state-modifying) methods. Safe methods are marked with the `#[safe]` attribute and are represented in the contract manifest with `"safe": true`. This is important for optimizing contract execution and ensuring proper access control.

### Event Emission

Events in Neo N3 contracts are emitted using the `Runtime::notify` method rather than event macros. This follows the Neo N3 specifications and ensures compatibility with the Neo N3 ecosystem.

### Storage Model

The framework uses a key-value storage model that maps directly to the Neo N3 storage system, with type-safe abstractions to simplify development.

### WebAssembly Approach

Using WebAssembly as an intermediate format allows the framework to leverage Rust's existing toolchain while targeting the Neo VM, providing the best of both worlds.

## System Requirements

- **Rust**: 1.60 or higher
- **WebAssembly Support**: wasm32-unknown-unknown target
- **Neo N3 Compatibility**: The framework targets Neo N3 and is not compatible with Neo Legacy
