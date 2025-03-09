# WASM to Neo VM Conversion

This document describes the process of converting WebAssembly (WASM) modules to Neo VM bytecode, which is a key part of the Neo Contract Rust framework.

## Overview

The Neo Contract Rust framework allows developers to write smart contracts in Rust, compile them to WebAssembly, and then convert the WASM bytecode to Neo VM bytecode. This enables the deployment of Rust-written smart contracts to the Neo N3 blockchain.

The conversion process consists of several steps:

1. Compiling Rust code to WASM using standard Rust tools
2. Parsing the WASM module to extract its structure
3. Converting WASM instructions to equivalent Neo VM instructions
4. Generating a Neo Executable Format (NEF) file
5. Creating a contract manifest

## Key Components

### WASM Module Parser

The WASM module parser reads a WASM binary file and extracts its structure, including:

- Function types (signatures)
- Functions (code)
- Imports and exports
- Global variables
- Memory and table definitions
- Data and element segments

### WASM to Neo VM Converter

The converter translates WASM instructions to Neo VM instructions. This is a complex process because the two virtual machines have different architectures:

- WASM is a stack-based VM with typed instructions
- Neo VM is also stack-based but has fewer primitives
- Memory models differ significantly
- Function calling conventions differ

The converter handles these differences by mapping WASM instructions to equivalent Neo VM instruction sequences.

### NEF Generator

The NEF generator creates a Neo Executable Format file, which is the binary format used by Neo N3 blockchain for deploying smart contracts. The NEF file includes:

- Compiled Neo VM script
- Script tokens (for gas calculations)
- Checksum for validation

### Manifest Generator

The manifest generator creates a contract manifest, which describes the contract's interface, permissions, and other metadata. The manifest includes:

- Contract name
- Supported standards
- ABI (methods and events)
- Permissions and trusts
- Additional metadata

## Instruction Mapping

The core of the conversion process is mapping WASM instructions to Neo VM instructions. Here are some examples:

| WASM Instruction | Neo VM Equivalent |
|------------------|-------------------|
| i32.const n      | PUSH n           |
| i32.add          | ADD              |
| i32.sub          | SUB              |
| i32.mul          | MUL              |
| i32.div_s        | DIV              |
| local.get i      | LDLOC i          |
| local.set i      | STLOC i          |
| call i           | CALL offset      |
| br i             | JMP offset       |
| br_if i          | JMPIF offset     |

## Memory Model

WASM and Neo VM have different memory models:

- WASM has a linear memory model with byte addressing
- Neo VM uses storage for persistent data

The converter bridges these differences by:

1. Converting WASM memory operations to Neo VM storage operations
2. Implementing a memory abstraction layer in Neo VM
3. Managing memory access permissions appropriately

## Function Calling Convention

Function calls in WASM and Neo VM work differently:

- WASM uses indices to reference functions
- Neo VM uses absolute or relative offsets

The converter resolves function references and generates appropriate CALL instructions with correct offsets.

## Handling Imports and Exports

WASM modules can import and export functions:

- Imports from the "neo" namespace are mapped to Neo VM syscalls
- Exports define the public interface of the contract
- The converter generates entry point dispatchers for exports

## Type Conversions

WASM and Neo VM have different type systems:

- WASM has explicit types (i32, i64, f32, f64)
- Neo VM has a more flexible type system

The converter handles type conversions, ensuring that:

1. Integer operations work correctly
2. Floating-point operations are emulated if necessary
3. Type constraints are respected

## Limitations

Current limitations in the WASM to Neo VM conversion:

1. Limited floating-point support
2. Some WASM instructions may not have direct equivalents
3. Memory size is constrained by Neo VM restrictions
4. Some complex WebAssembly features may not be fully supported

## Usage

To convert a Rust smart contract to Neo VM:

1. Compile the Rust contract to WASM:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. Convert the WASM file to NEF:
   ```bash
   neo-wasm --input target/wasm32-unknown-unknown/release/contract.wasm --output ./build
   ```

3. Deploy the NEF file and manifest to the Neo blockchain using the Neo CLI or SDK.

## Advanced Features

### Optimizations

The converter applies various optimizations to the generated Neo VM code:

1. Constant folding and propagation
2. Instruction combining
3. Dead code elimination
4. Stack operation optimization

### Debugging Support

The converter can generate debug information to help troubleshoot issues:

1. Source maps linking Neo VM instructions to WASM instructions
2. Function name tables
3. Stack traces for error reporting

## Future Work

Planned improvements to the WASM to Neo VM converter:

1. Better floating-point support
2. More comprehensive instruction mappings
3. Advanced memory management techniques
4. Improved error reporting and diagnostics
5. Integration with Rust debugging tools