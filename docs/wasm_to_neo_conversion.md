# WASM to Neo Conversion Process

This document explains how the Neo Contract Rust Framework converts WebAssembly (WASM) modules to Neo N3 smart contracts, detailing the compilation pipeline, transformation processes, and output formats.

## Overview

The Neo Contract Rust Framework allows developers to write Neo N3 smart contracts using Rust with an ink!-inspired syntax. The compilation process involves several steps:

1. Rust code is compiled to WebAssembly (WASM) using the standard Rust toolchain
2. The WASM binary is parsed and analyzed
3. WASM instructions are mapped to Neo VM opcodes
4. A Neo Executable Format (NEF) file is generated
5. A contract manifest is created or customized
6. Both files are written to disk, ready for deployment

This document focuses on steps 2-6, the process of converting WASM to a Neo smart contract.

## Compilation Pipeline

The compilation process is orchestrated by the `Compiler` class in the `neo-compiler` crate. Here's the high-level flow:

```
┌───────────┐      ┌──────────┐      ┌──────────┐      ┌──────────┐
│           │      │          │      │          │      │          │
│  Rust     │──────► WASM     │──────► Neo VM   │──────► NEF File │
│  Source   │      │ Binary   │      │ Bytecode │      │          │
│           │      │          │      │          │      │          │
└───────────┘      └──────────┘      └──────────┘      └──────────┘
                                                             │
                                                             │
                                                             ▼
                                                       ┌──────────┐
                                                       │          │
                                                       │ Manifest │
                                                       │ JSON     │
                                                       │          │
                                                       └──────────┘
```

## WASM Parsing and Analysis

The first step in the conversion process is parsing the WASM binary and extracting its components. WASM modules have a well-defined structure:

- A magic number and version identifier
- Sections containing module metadata, functions, types, etc.
- Code sections with function bodies
- Custom sections with additional metadata

The `wasm` module parses these sections and constructs an in-memory representation of the WASM module, which includes:

- Function definitions and signatures
- The function table
- Global variables
- Memory definitions
- Import and export declarations
- Custom sections containing contract metadata

During this analysis, the compiler extracts information needed for the Neo contract, such as:

- Contract entry points
- Events and their structures
- Storage layout
- Method definitions and their parameters

## WASM to Neo VM Instruction Mapping

The core of the conversion process is mapping WASM instructions to Neo VM opcodes. This is handled by the `neo` module, which implements a custom WASM interpreter that emits Neo VM bytecode.

### Handling the Stack-Based Model

Both WASM and Neo VM are stack-based virtual machines, but they have different instruction sets and semantics. The compiler must bridge these differences:

- WASM uses a typed stack, while Neo VM has a single, untyped stack
- WASM has more primitive types than Neo VM
- WASM has direct memory access, while Neo VM uses storage operations
- WASM has control flow instructions that don't directly map to Neo VM

### Instruction Mapping Examples

Here are some examples of how WASM instructions map to Neo VM opcodes:

| WASM Instruction | Neo VM Opcode(s) | Description |
|------------------|------------------|-------------|
| `local.get` | `LDLOC` | Load a local variable onto the stack |
| `local.set` | `STLOC` | Store a value from the stack to a local variable |
| `i32.add` | `ADD` | Add two integers |
| `i32.const` | `PUSHINT` | Push an integer constant onto the stack |
| `call` | `CALL` | Call a function |
| `br_if` | `JMPIF` | Conditional branch |
| `memory.store` | Custom storage operations | Store a value to memory |

For more complex WASM instructions, the compiler may generate multiple Neo VM opcodes to achieve the same behavior.

### Handling Memory

WASM has a linear memory model, while Neo uses a key-value storage system. The compiler maps WASM memory operations to Neo storage operations:

- WASM memory loads become Neo storage reads
- WASM memory stores become Neo storage writes
- Memory allocations are tracked and managed

### Function Calls and Entry Points

The compiler identifies entry points in the WASM module and creates corresponding Neo VM entry points. It also:

- Maps imported functions to Neo syscalls or contract calls
- Handles function parameter passing and return values
- Manages the call stack and local variables

## NEF File Creation

Once the Neo VM bytecode is generated, the compiler creates a Neo Executable Format (NEF) file. The NEF file contains:

- A magic number and version identifier
- Compiler information
- Script hash calculation
- The Neo VM bytecode for the contract
- Checksum for integrity verification

The `nef` module handles the creation and serialization of the NEF file.

## Manifest Generation

In addition to the NEF file, Neo contracts require a manifest file in JSON format. The manifest describes the contract's interface and permissions:

- Basic information (name, groups, etc.)
- ABI (Application Binary Interface) defining methods and their parameters
- Permissions for contract calls
- Supported standards
- Extra metadata

The manifest can be:
- Generated automatically based on the contract code
- Loaded from a template file and customized
- Modified with overrides specified in compiler options

## Optimization

The compiler can perform various optimizations on the generated Neo VM bytecode:

- Dead code elimination
- Constant folding
- Instruction simplification
- Stack manipulation optimizations
- Call pattern optimizations

These optimizations help reduce the size of the contract and the gas cost of execution.

## Output Files

The final output of the compilation process consists of two files:

1. **NEF File** (`.nef`): Contains the Neo VM bytecode for the contract
2. **Manifest File** (`.manifest.json`): Describes the contract's interface and permissions

These files can be deployed to the Neo N3 blockchain using the Neo CLI or other deployment tools.

## Example Compilation

Let's trace through a simplified example of compiling a basic Rust contract:

```rust
#[contract]
pub mod token {
    #[storage]
    struct Token {
        total_supply: StorageItem<u64>,
    }
    
    impl Token {
        #[constructor]
        pub fn new(supply: u64) -> Self {
            let mut this = Self {
                total_supply: StorageItem::new(),
            };
            this.total_supply.set(supply);
            this
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
    }
}
```

1. **Rust to WASM**: The Rust compiler (rustc) compiles this to WASM, generating functions for the constructor and methods

2. **WASM Parsing**: The Neo compiler parses the WASM binary, identifying the constructor and `total_supply` method

3. **Neo VM Code Generation**: The compiler generates Neo VM bytecode for each function:
   - Constructor: Sets up storage, stores the initial supply
   - `total_supply`: Reads from storage, handles the default case

4. **NEF File Creation**: The compiler creates a NEF file with the bytecode

5. **Manifest Generation**: The compiler generates a manifest with:
   - ABI listing the `constructor` and `total_supply` methods
   - Marks `total_supply` as safe (read-only)
   - Sets up appropriate permissions

6. **Output**: The compiler writes the NEF and manifest files to disk

## Advanced Topics

### Custom Sections

WASM allows for custom sections which the Neo compiler uses to store additional metadata about the contract, such as:

- Event definitions
- Storage layout
- Method attributes (safe, payable, etc.)

### Contract Upgradeability

The Neo compiler can include upgrade functionality in the contract, allowing it to be upgraded later while preserving its storage.

### Gas Estimation

While converting WASM to Neo VM bytecode, the compiler can estimate the gas cost of various operations, helping developers optimize their contracts.

## Conclusion

The WASM to Neo conversion process is a sophisticated pipeline that enables writing Neo N3 smart contracts in Rust. By leveraging the Rust ecosystem and toolchain, developers can write safe, efficient smart contracts with modern language features, which are then compiled to the Neo VM format for deployment on the Neo blockchain.

The Neo Contract Rust Framework handles the complexities of this conversion, allowing developers to focus on their contract logic rather than the details of the Neo VM instruction set or the NEF file format.