# neo-contract-rs Documentation Summary

## Overview Documents

- [Architecture Overview](architecture.md) - High-level description of the framework's architecture
- [API Reference](api-reference.md) - Detailed reference for the main API components
- [Getting Started Guide](getting-started.md) - Guide for setting up and creating first contracts
- [Testing Guide](testing-guide.md) - Guide for testing Neo contracts
- [Oracle Framework](oracle-framework.md) - Guide for using the Oracle framework to access external data

## Contract Manifests

- [Manifest Generation](manifest-generation.md) - How the framework generates contract manifests
- [Documentation Best Practices](documentation-best-practices.md) - How to write documentation for optimal manifest generation
- [Understanding NEO Manifests](understanding-neo-manifests.md) - Structure and customization of NEO contract manifests
- [Code Documentation Style](code-documentation-style.md) - Style guide for documenting code and enabling optimal manifest generation

## Best Practices

- [Efficient Smart Contracts](efficient-contracts.md) - Guidelines for writing efficient and optimized NEO contracts

## Development Plans

- [Roadmap](roadmap.md) - Future development plans for the framework
- [Implementation Plan](implementation-plan.md) - Detailed plan for completing the framework
- [Examples Plan](examples-plan.md) - Plan for developing comprehensive examples
- [Testing Plan](testing-plan.md) - Comprehensive testing strategy

## Current Status

The neo-contract-rs framework is in early development with:

- Core Neo N3 types and operations
- NEP-17 token standard implementation
- Basic contract macro functionality
- Storage and runtime operations

## Next Steps

The immediate next steps for the project are:

1. **Documentation**: Complete inline documentation for all existing code
2. **Testing**: Implement basic testing infrastructure and unit tests
3. **Examples**: Develop additional example contracts
4. **NEP Standards**: Complete the NEP-17 implementation and start on NEP-11
5. **Oracle Framework**: Implement the Oracle framework for external data access

## Example Contracts

Current examples:
- [NEP-17 Token](../examples/nep17/) - Basic fungible token implementation
- [Transfer](../examples/transfer/) - Simple value transfer functionality
- [Hello World](../examples/hello-world/) - Basic contract with greeting functionality
- [Simple Storage](../examples/simple-storage/) - Key-value storage operations example
- [NEP-11 NFT](../examples/nep11-nft/) - Non-fungible token implementation
- [Oracle Price Feed](../examples/oracle-price-feed/) - Example using the Oracle framework to access price data

Planned examples (see [Examples Plan](examples-plan.md) for details):
- Advanced NEP-17 Token
- NEP-11 NFT (Divisible)
- Upgradable Contract
- Multi-Signature Wallet
- Voting System
- Various application examples

## Framework Components

The framework consists of:

1. **Core Library** (`neo-contract/`)
   - Contract interfaces
   - Neo types
   - Runtime operations
   - Storage abstractions
   - Event handling
   - Cryptographic operations
   - Oracle interface

2. **Procedural Macros** (`neo-contract-proc-macros/`)
   - Contract macro
   - Structs serialization macro

## Guides

- [Getting Started Guide](getting-started.md) - Guide for setting up and creating first contracts
- [NEP-11 Implementation Guide](nep11-guide.md) - Comprehensive guide for implementing NFTs
- [Serialization Guide](serialization-guide.md) - Guide for implementing custom serialization
- [Testing Guide](testing-guide.md) - Guide for testing Neo contracts
- [Oracle Framework](oracle-framework.md) - Guide for using the Oracle framework to access external data

## Resources

- [Neo Documentation](https://docs.neo.org/)
- [Neo N3 Github](https://github.com/neo-project/neo)
- [Neo Enhancement Proposals (NEPs)](https://github.com/neo-project/proposals) 