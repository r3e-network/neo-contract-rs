# Neo N3 Contract API Reference

This section provides detailed documentation for the Neo N3 contract development framework API.

## Core Modules

- [Storage](storage.md) - Storage abstractions for managing contract state
- [Runtime](runtime.md) - Blockchain runtime interactions
- [Types](types.md) - Neo N3-specific type definitions
- [Events](events.md) - Event emission utilities
- [Syscalls](syscalls.md) - Low-level Neo VM syscall interfaces

## Attributes

- [`#[contract]`](attributes/contract.md) - Marks a module as a Neo N3 smart contract
- [`#[storage]`](attributes/storage.md) - Defines the contract storage structure
- [`#[method]`](attributes/method.md) - Exposes a function as a contract method
- [`#[safe]`](attributes/safe.md) - Marks methods as read-only for optimized execution
- [`#[initialize]`](attributes/initialize.md) - Designates the contract initialization method

## Standard Implementations

- [NEP-17 Token](standards/nep17.md) - Fungible token standard implementation
- [NEP-11 Token](standards/nep11.md) - Non-fungible token standard implementation

## Utility Modules

- [Crypto](utilities/crypto.md) - Cryptographic utilities
- [Testing](utilities/testing.md) - Contract testing utilities
- [Serialization](utilities/serialization.md) - Data serialization helpers
