# Neo Compiler Roadmap

This document outlines the planned development roadmap for the neo-compiler project. It outlines both short-term and long-term goals to guide future development efforts.

## Current Status (v0.1.0)

The neo-compiler has been restructured to provide a clean, consistent API with the following features:

- WebAssembly to Neo VM bytecode conversion
- NEF file generation and manipulation
- Contract manifest generation and manipulation
- Manual Neo VM script creation
- Command-line and programmatic interfaces

## Short-Term Goals (v0.2.0)

### Enhancements

- [ ] **Improved WebAssembly Analysis**: Enhance WebAssembly parsing to extract more detailed information about functions and types
- [ ] **Script Optimization**: Implement more advanced Neo VM bytecode optimizations
- [ ] **Better Error Messages**: Provide more context and suggestions in error messages
- [ ] **Memory Efficiency**: Optimize memory usage for large WebAssembly modules

### Features

- [ ] **Smart Contract Testing Tools**: Provide tools for testing compiled contracts without deployment
- [ ] **Contract Verification**: Add functionality to verify deployed contracts against source code
- [ ] **Contract Upgrade Support**: Add tools to facilitate smart contract upgrades
- [ ] **Gas Estimation**: Estimate gas costs for contract operations

### Documentation & Examples

- [ ] **Advanced Examples**: More advanced contract examples (e.g., DEX contract, NFT contract)
- [ ] **Video Tutorials**: Create video tutorials for common use cases
- [ ] **Integration Guides**: Documents on integrating with Neo blockchain tools

## Mid-Term Goals (v0.3.0)

### Enhancements

- [ ] **Advanced Type System**: Improve conversion of complex WebAssembly types to Neo VM types
- [ ] **Control Flow Analysis**: Advanced analysis of WebAssembly code for better conversion
- [ ] **Debugging Support**: Add debugging information to compiled contracts
- [ ] **Support for More WASM Features**: Support for additional WebAssembly features

### Features

- [ ] **Visual Contract Explorer**: A tool to visualize contract structure and flow
- [ ] **Contract Packaging**: Package and distribute contracts with dependencies
- [ ] **Contract Library Support**: Better support for contract dependencies and linking
- [ ] **IDE Integration**: IDE plugins for common editors (VS Code, JetBrains)

### Ecosystem

- [ ] **Standard Contract Templates**: Provide standard templates for common contract types
- [ ] **Contract Audit Tools**: Built-in security analysis for common vulnerabilities
- [ ] **Community Extensions**: Support for community-developed extensions

## Long-Term Goals

### Vision

- [ ] **WebAssembly Feature Parity**: Support all relevant WebAssembly features
- [ ] **Seamless Developer Experience**: Make Neo development as easy as traditional web development
- [ ] **Performance Optimization**: Continuous performance improvements
- [ ] **Full Language Support**: First-class support for languages beyond Rust

### Research

- [ ] **Advanced Optimization Techniques**: Research on applying compiler optimization techniques to Neo contracts
- [ ] **Formal Verification**: Integrate formal verification tools for critical contracts
- [ ] **AI-Assisted Development**: Leverage AI for contract development and optimization

## Contributing

We welcome contributions to any of the areas mentioned in this roadmap. Please see the [Contributing Guide](./contributing.md) for details on how to contribute.

## Feedback

This roadmap is a living document and will evolve based on community feedback and ecosystem needs. If you have suggestions or feedback, please open an issue in the repository.

---

Last updated: August 2023 