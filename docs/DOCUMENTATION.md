# NEO Contract Rust Framework Documentation

## Current State

The NEO Contract Rust Framework is a comprehensive toolkit for developing smart contracts for the NEO N3 blockchain platform. This document outlines the current state of the framework, known issues, and recommended approaches.

## Framework Components

1. **neo-contract**: Core library providing types, runtime functions, and APIs for NEO N3 contracts
2. **neo-compiler**: Compiler for NEO contracts, converts WASM to NEO VM bytecode
3. **neo-macros**: Procedural macros for simplified contract development
4. **neo-contract-testing**: Testing utilities for NEO contracts

## Known Issues and Workarounds

### 1. Macro Attribute Issues

#### Issue
Several attribute macros used for contract metadata are imported but not fully implemented:
- `contract_author`
- `contract_description`
- `contract_version`
- `supported_standards`
- Other contract-related attributes (`event`, `index`, etc.)

#### Workaround
We've added placeholder implementations for these macros, but they're not fully functional yet. The current implementation allows compilation but doesn't perform the full functionality expected from these macros.

### 2. Storage System

#### Issue
The storage system has inconsistencies between naming and implementation.

#### Workaround
Use type aliases to bridge differences:
```rust
use neo_contract::storage::map::Map as StorageMap;
```

### 3. Event Emission System

#### Issue
The event emission system using `EventName::emit()` is not fully implemented.

#### Workaround
Use the lower-level `Runtime::notify()` method:
```rust
let mut event_args = Array::new();
event_args.push(Any::from(address));
Runtime::notify(&ByteString::from("MemberAdded"), &event_args);
```

## Documentation-First Approach

Following the requirement for a documentation-first approach:

1. **Contract Documentation**: Each contract should include comprehensive documentation regarding its purpose, functions, and behavior.

2. **Documentation/Implementation Pairing**: Maintain paired documentation and implementation files for each feature.

3. **Documentation-Based Navigation**: Start with documentation to understand the framework before navigating to implementation details.

4. **Consistent Structure**: The repository maintains a consistent documentation structure in the `docs/` directory.

## Recommended Development Approach

For developing new contracts:

1. **Start with Hello World Example**: Use the simplified Hello World example as a starting point.

2. **Manual Implementation for Now**: Until macros are fully functional, manually implement and manage events and storage.

3. **Explicit Imports**: Be explicit about imports; avoid relying on prelude until it's stabilized.

4. **Documentation First**: Write documentation describing the contract's functionality before implementing.

## Next Steps

To improve the framework:

1. **Complete Macro Implementations**: Fully implement all attribute macros with proper functionality.

2. **Standardize Storage API**: Create a consistent API for storage operations.

3. **Event System**: Implement a type-safe event system.

4. **Testing Utilities**: Enhance testing utilities for contract testing.

5. **Documentation**: Continue improving documentation with examples and best practices.

## Conclusion

The NEO Contract Rust Framework is a promising framework for developing NEO N3 smart contracts. While there are currently some limitations with macros and APIs, workarounds exist to enable contract development. Following the documentation-first approach will help maintain a clean and well-structured project.
