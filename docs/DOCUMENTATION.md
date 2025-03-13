# Neo N3 Contract Development Framework Documentation

## Overview

The Neo N3 contract development framework for Rust enables developers to write smart contracts for the Neo N3 blockchain using the Rust programming language. This framework provides a comprehensive set of tools and libraries for contract development, compilation, and deployment.

## Documentation Structure

- [Getting Started](getting-started.md) - Quick start guide for new developers
- [Architecture](architecture.md) - Overview of the framework architecture
- [API Reference](api/README.md) - Detailed API documentation
- [Examples](examples.md) - Example contracts and use cases
- [Best Practices](best-practices.md) - Recommended patterns and practices
- [Migration Guide](migration-guide.md) - Guide for migrating from other frameworks
- [Troubleshooting](troubleshooting.md) - Common issues and solutions
- [Contributing](contributing.md) - Guidelines for contributing to the project

## Framework Components

The framework consists of three main components:

1. **neo-contract** - Core library providing the Neo N3 contract development API
2. **neo-compiler** - Compiler that converts WebAssembly to Neo VM bytecode
3. **neo-macros** - Procedural macros for the attribute-based interface

## Compilation Flow

```
Rust code → WebAssembly → Neo VM bytecode → NEF file + Manifest
```

## Key Features

- **Rust-First Development**: Write smart contracts in pure Rust
- **Neo N3 Compatibility**: Full support for Neo N3 features and standards
- **Storage Abstractions**: Type-safe storage primitives for contract state
- **Event System**: Strongly-typed event emission system
- **Security Features**: Built-in security patterns and protections
- **Testing Tools**: Utilities for unit and integration testing
- **Standard Implementations**: Helpers for implementing Neo N3 standards (NEP-17, NEP-11, etc.)
