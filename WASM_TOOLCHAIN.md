# Neo WASM Toolchain Setup

This document explains how to set up and use the WebAssembly (WASM) toolchain for Neo smart contract development.

## Overview

Neo smart contracts can be compiled to WebAssembly for deployment on the Neo blockchain. This process requires several tools:

1. **Rust WASM Target**: Allows Rust code to be compiled to WASM
2. **wasm-pack**: A tool for building and packaging Rust-generated WASM
3. **wasm-opt**: Optimizes WASM files to reduce size and improve performance

## Setup Instructions

We've created a script to automatically set up all the necessary tools:

```bash
# Make the script executable
chmod +x scripts/build/setup_wasm_toolchain.sh

# Run the setup script
./scripts/build/setup_wasm_toolchain.sh
```

This script will:
- Install the `wasm32-unknown-unknown` Rust target
- Install `wasm-pack` if not already available
- Install `wasm-opt` (from binaryen) if not already available
- Create a `build_wasm.sh` script for compiling projects

## Building WASM Files

We provide three different approaches for building WASM contracts, each with different levels of integration with the Neo framework:

### Option 1: Using the Neo-Adapted WASM Contract (Recommended)

This approach uses a special adaptation that works with both Neo blockchain and WebAssembly:

```bash
# Build the Neo WASM contract
./scripts/build/build_neo_wasm.sh
```

This builds the `examples/neo_wasm_contract` example, which demonstrates:
- Dual contract implementation (Neo + WASM)
- Token functionality (transfers, balances)
- WASM-specific optimizations

### Option 2: Using the full Neo contract framework (with limitations)

```bash
# Show available crates
./scripts/build/build_wasm.sh

# Build a specific crate
./scripts/build/build_wasm.sh hello_world

# Build with custom output name
./scripts/build/build_wasm.sh hello_world customName.wasm
```

### Option 3: Using the simplified example builder

For simpler WASM builds that don't depend on the full Neo contract framework:

```bash
# Show available examples
./scripts/build/build_example_wasm.sh

# Build a specific example
./scripts/build/build_example_wasm.sh wasm_demo

# Build with custom output name
./scripts/build/build_example_wasm.sh wasm_demo customName.wasm
```

The compiled WASM files will be placed in the `wasm/` directory at the project root.

## Examples

### 1. Available Examples

The project includes various examples that can be compiled to WASM:

- **NEP-17 Token**: Standard fungible token implementation
- **NFT Examples**: Non-fungible token contracts
- **DeFi Examples**: Decentralized finance applications
- **Cross-Contract Examples**: Inter-contract communication patterns

For detailed information about available examples, see the [Examples README](examples/README.md).

### 2. WASM Demo

We've created a simple example WASM contract that demonstrates how to:
- Create a basic contract structure with wasm-bindgen
- Implement methods and state storage
- Simulate blockchain events
- Build to WASM without dependencies on the neo-contract framework

This example serves as a good starting point while we fix the compilation issues in the main framework.

## WASM Syscall Implementation

For the technically inclined, our architecture now includes:

1. **syscall_wasm.rs**: A WASM-specific syscall implementation that provides stubs for all Neo VM functions
2. **build.rs**: Configuration for WASM-specific build settings
3. **Conditional compilation**: Using `#[cfg(target_arch = "wasm32")]` for WASM-specific code

## Current Limitations

> **Note**: There are still some compilation issues with the original Neo contract examples when compiled to WASM. We've implemented workarounds described above, but more work is needed for full WASM support.

The most common issues are:
- Missing type imports in various files
- Undeclared module references
- Function calls to non-existent functions

## Future Improvements

Future work on the WASM toolchain could include:
1. Complete the syscall implementations for WASM targets
2. Add proper Neo VM emulation in WebAssembly
3. Create deployment scripts for compiled WASM files
4. Add WASM verification tooling

## References

- [Rust WebAssembly Documentation](https://rustwasm.github.io/docs/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/book/)
- [Binaryen (wasm-opt) Repository](https://github.com/WebAssembly/binaryen) 