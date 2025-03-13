# Neo N3 Compiler Guide

This guide explains the process of compiling Rust smart contracts to Neo N3-compatible bytecode using the neo-compiler component of the neo-contract-rs framework.

## Overview

The Neo N3 contract compilation process transforms Rust source code into Neo Virtual Machine (NeoVM) bytecode that can run on the Neo N3 blockchain. This conversion happens in multiple stages:

1. Rust source code → WebAssembly (WASM)
2. WebAssembly → Neo N3 bytecode (.nef file)
3. Generation of contract manifest (.manifest.json)

## Compilation Pipeline

### Stage 1: Rust to WebAssembly

The first stage uses the standard Rust compiler with the `wasm32-unknown-unknown` target:

```bash
cargo build --target wasm32-unknown-unknown --release
```

This generates a WebAssembly binary (.wasm) file in the `target/wasm32-unknown-unknown/release/` directory.

Key steps in this stage:
- Rust code is compiled to LLVM IR
- LLVM IR is compiled to WebAssembly
- The resulting WASM file contains the contract's functionality in WebAssembly bytecode

### Stage 2: WebAssembly to Neo N3 Bytecode

The neo-compiler transforms the WebAssembly binary into Neo N3 bytecode:

```bash
neo-compiler compile path/to/your_contract.wasm --output ./build
```

During this stage:
1. The WebAssembly binary is parsed and analyzed
2. The WASM instructions are converted to equivalent NeoVM instructions
3. Optimizations are applied to reduce gas costs
4. The resulting bytecode is packaged into a Neo Executable Format (.nef) file

### Stage 3: Contract Manifest Generation

Along with the .nef file, the compiler generates a contract manifest (.manifest.json) that describes:

- Contract methods and their signatures
- Contract events
- Required permissions
- Supported standards (e.g., NEP-17, NEP-11)
- Additional metadata

## The Neo Compiler in Detail

### Architecture

The neo-compiler works through several layers:

1. **WASM Parser**: Parses and validates the WebAssembly binary
2. **WASM Analyzer**: Analyzes the structure and control flow of the WebAssembly code
3. **Neo Instruction Generator**: Converts WebAssembly instructions to NeoVM instructions
4. **Optimizer**: Applies gas and size optimizations
5. **NEF Generator**: Packages the bytecode into a valid .nef file
6. **Manifest Generator**: Creates the contract manifest based on code analysis

### Compiler Options

The neo-compiler supports various options:

```bash
neo-compiler compile --help

# Key options:
# --output          Specify output directory
# --debug           Include debug information
# --optimize        Set optimization level (default: high)
# --no-inline       Disable function inlining
# --manifest-extra  Add extra fields to manifest
```

## WebAssembly to NeoVM Translation

### Instruction Mapping

The compiler maps WebAssembly instructions to NeoVM instructions:

| WebAssembly | NeoVM |
|-------------|-------|
| i32.add     | ADD   |
| i32.sub     | SUB   |
| local.get   | LDLOC |
| local.set   | STLOC |
| i32.store   | STSFLD |
| call        | CALL  |
| br_if       | JMPIF |

### Memory Management

NeoVM has a different memory model than WebAssembly:

- WebAssembly uses a linear memory model
- NeoVM uses a stack-based execution model with contract storage

The compiler:
1. Maps WebAssembly memory operations to NeoVM operations
2. Handles memory allocation and deallocation
3. Optimizes memory access patterns for Neo N3

### System Calls

Neo N3-specific functionality is accessed via system calls. The compiler:

1. Identifies Rust code that needs to be translated to Neo N3 system calls
2. Inserts appropriate NeoVM `SYSCALL` instructions with the correct identifiers
3. Ensures parameters are properly prepared on the stack

Example system calls:
- Runtime.CheckWitness
- Runtime.Notify
- Storage.Get/Put/Delete
- Contract.Call

## Safe Methods

The compiler analyzes contract methods to determine if they can be marked as "safe" (read-only):

```rust
#[safe]
fn balance_of(&self, address: H160) -> u64 {
    // Read-only implementation
}
```

Methods marked with `#[safe]` are:
1. Automatically exposed as contract methods (no need for additional `#[method]` annotation)
2. Marked as safe in the manifest with `"safe": true`
3. Only valid for read-only methods that don't modify contract storage
4. Optimized for more efficient execution by Neo N3 nodes

Note that `#[safe]` already includes the functionality of `#[method]`, so there's no need to use both annotations together.

## Contract Metadata Extraction

The compiler extracts metadata from the Rust code to generate the manifest:

### Method Signatures

Methods marked with `#[method]` are included in the ABI section of the manifest:

```json
"methods": [
  {
    "name": "balance_of",
    "parameters": [
      {"name": "address", "type": "Hash160"}
    ],
    "returntype": "Integer",
    "offset": 123,
    "safe": true
  }
]
```

### Events

Events from your Rust code are included in the manifest:

```json
"events": [
  {
    "name": "Transfer",
    "parameters": [
      {"name": "from", "type": "Hash160"},
      {"name": "to", "type": "Hash160"},
      {"name": "amount", "type": "Integer"}
    ]
  }
]
```

## Optimizations

The neo-compiler applies several optimizations:

### Constant Folding

```rust
// Before optimization
let a = 2 + 3;

// After optimization (in NeoVM)
// PUSH5 (directly pushes the value 5)
```

### Dead Code Elimination

Unreachable code is removed to reduce contract size and gas costs.

### Function Inlining

Small functions are inlined to eliminate call overhead.

### Loop Optimizations

Loops are optimized for more efficient execution on NeoVM.

## Debug Information

When compiling with `--debug`, the compiler includes additional information that helps with contract debugging:

```bash
neo-compiler compile your_contract.wasm --debug --output ./build
```

This generates:
- Source mapping information
- Variable names
- Function boundaries

## Advanced Compilation Techniques

### Custom Sections

You can include custom metadata in your WebAssembly that is preserved in the NEF file:

```rust
#[compiler_metadata]
struct ContractInfo {
    author: &'static str,
    website: &'static str,
}

// Accessed via compiler.custom_section("contract_info")
```

### Conditional Compilation

Use Rust's conditional compilation to create different versions:

```rust
#[cfg(feature = "mainnet")]
const OWNER: &str = "NMainNetAddress...";

#[cfg(feature = "testnet")]
const OWNER: &str = "NTestNetAddress...";
```

Build with specific features:
```bash
cargo build --target wasm32-unknown-unknown --release --features testnet
```

## Troubleshooting Common Compiler Issues

### Size Limitations

Neo N3 has limits on contract size. If your contract is too large:

- Break it into multiple contracts
- Remove unnecessary functionality
- Optimize code for size

### Memory Errors

If you encounter memory-related errors:

- Check array bounds and memory access
- Ensure proper handling of dynamic memory
- Avoid excessive memory usage

### System Call Errors

For system call-related errors:

- Verify you're using the correct Neo N3 syscall interface
- Ensure parameters are of the correct type
- Check syscall permissions in the manifest

## Compiler Output Analysis

### Analyzing NEF Files

To analyze a compiled NEF file:

```bash
neo-compiler info your_contract.nef

# Outputs:
# - Contract size
# - Instruction counts
# - System call usage
# - Entry points
```

### Disassembling NeoVM Bytecode

For a deeper understanding:

```bash
neo-compiler disassemble your_contract.nef

# Outputs NeoVM assembly:
# 0000: PUSHDATA1 0x0123...
# 0011: SYSCALL 0x456789...
# ...
```

## Best Practices for Neo N3 Compilation

### 1. Minimal Dependencies

Minimize external crate dependencies to keep contract size small.

### 2. Test Before Compilation

Test your Rust contract with the neo-contract-testing framework before compilation:

```rust
#[cfg(test)]
mod tests {
    use neo_contract_testing::*;
    // Test your contract logic
}
```

### 3. Incremental Compilation

For complex contracts, compile incrementally to identify issues early.

### 4. Use Conditional Compilation

Use `#[cfg]` attributes to exclude test code from production builds.

### 5. Optimize Storage Access

Group storage operations to minimize gas costs.

### 6. Verify Manifest

Always review the generated manifest to ensure it accurately represents your contract.

## Conclusion

The Neo N3 compilation process is a critical step in deploying Rust smart contracts to the Neo N3 blockchain. Understanding this process helps you write more efficient and effective contracts.

For additional information, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Deployment Guide](./neo_n3_deployment_guide.md)
- [Neo N3 Security Guide](./neo_n3_security_guide.md)
