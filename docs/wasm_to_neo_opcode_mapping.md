# WASM to Neo VM Opcode Mapping Guide

This technical guide details how WebAssembly (WASM) instructions are mapped to Neo VM opcodes in the neo-compiler component of the Neo Contract Rust Framework.

## Introduction

Converting WASM code to Neo VM bytecode requires mapping between two different virtual machine instruction sets. This document explains the mapping strategies, challenges, and implementation details of this conversion process.

## Virtual Machine Comparison

### WebAssembly (WASM)

- **Type System**: Strongly typed (i32, i64, f32, f64)
- **Memory Model**: Linear byte-addressable memory
- **Stack**: Typed evaluation stack
- **Locals**: Typed local variables
- **Control Flow**: Structured control flow (blocks, loops, if statements)
- **Function Calls**: Direct and indirect calls via function tables

### Neo VM

- **Type System**: Dynamic typing (Integer, ByteArray, Boolean, Map, Array)
- **Memory Model**: Key-value storage
- **Stack**: Single untyped evaluation stack
- **Locals**: Untyped local variables
- **Control Flow**: Jump-based control flow
- **Function Calls**: Direct calls and syscalls

## Mapping Strategy

The mapping from WASM to Neo VM follows these general principles:

1. **Instruction Mapping**: Each WASM instruction is mapped to one or more Neo VM opcodes
2. **Stack Management**: Handle differences in stack behavior
3. **Type Conversion**: Convert between type systems
4. **Memory Adaptation**: Adapt linear memory to key-value storage
5. **Control Flow Translation**: Convert structured control flow to jumps

## Instruction Mapping Table

Below is a reference table for mapping common WASM instructions to Neo VM opcodes:

| WASM Instruction | Neo VM Opcode(s) | Notes |
|------------------|------------------|-------|
| **Constants** |  |  |
| `i32.const` | `PUSHINT` | Push integer constant |
| `i64.const` | `PUSHINT` | Push integer constant (Big integers may require special handling) |
| `f32.const`, `f64.const` | - | Floating point not directly supported, need workaround |
| **Variables** |  |  |
| `local.get` | `LDLOC` | Load local variable |
| `local.set` | `STLOC` | Store to local variable |
| `local.tee` | `DUP`, `STLOC` | Store and keep value on stack |
| `global.get` | Custom storage operations | Load from storage |
| `global.set` | Custom storage operations | Store to storage |
| **Memory Operations** |  |  |
| `i32.load` | Custom storage operations | Load from storage via key |
| `i32.store` | Custom storage operations | Store to storage via key |
| `memory.size` | Custom storage operations | Get storage size |
| `memory.grow` | - | Not directly supported |
| **Numeric Operations** |  |  |
| `i32.add`, `i64.add` | `ADD` | Addition |
| `i32.sub`, `i64.sub` | `SUB` | Subtraction |
| `i32.mul`, `i64.mul` | `MUL` | Multiplication |
| `i32.div_s`, `i64.div_s` | `DIV` | Signed division |
| `i32.div_u`, `i64.div_u` | Custom implementation | Unsigned division |
| `i32.rem_s`, `i64.rem_s` | `MOD` | Signed remainder |
| `i32.rem_u`, `i64.rem_u` | Custom implementation | Unsigned remainder |
| **Bitwise Operations** |  |  |
| `i32.and`, `i64.and` | `AND` | Bitwise AND |
| `i32.or`, `i64.or` | `OR` | Bitwise OR |
| `i32.xor`, `i64.xor` | `XOR` | Bitwise XOR |
| `i32.shl`, `i64.shl` | `SHL` | Shift left |
| `i32.shr_s`, `i64.shr_s` | `SHR` | Shift right (sign extended) |
| `i32.shr_u`, `i64.shr_u` | Custom implementation | Unsigned shift right |
| `i32.rotl`, `i64.rotl` | Custom implementation | Rotate left |
| `i32.rotr`, `i64.rotr` | Custom implementation | Rotate right |
| **Comparison Operations** |  |  |
| `i32.eq`, `i64.eq` | `EQUAL` | Equality |
| `i32.ne`, `i64.ne` | `EQUAL`, `NOT` | Inequality |
| `i32.lt_s`, `i64.lt_s` | `LT` | Signed less than |
| `i32.lt_u`, `i64.lt_u` | Custom implementation | Unsigned less than |
| `i32.gt_s`, `i64.gt_s` | `GT` | Signed greater than |
| `i32.gt_u`, `i64.gt_u` | Custom implementation | Unsigned greater than |
| `i32.le_s`, `i64.le_s` | `LE` | Signed less than or equal |
| `i32.le_u`, `i64.le_u` | Custom implementation | Unsigned less than or equal |
| `i32.ge_s`, `i64.ge_s` | `GE` | Signed greater than or equal |
| `i32.ge_u`, `i64.ge_u` | Custom implementation | Unsigned greater than or equal |
| **Conversion Operations** |  |  |
| `i32.wrap_i64` | - | Value truncation |
| `i64.extend_i32_s` | - | Sign extension |
| `i64.extend_i32_u` | - | Zero extension |
| **Control Flow** |  |  |
| `block` | - | Begin a block (prepare jump targets) |
| `loop` | - | Begin a loop (prepare jump targets) |
| `if` | `JMPIF` | Conditional branch |
| `else` | `JMP` | Jump to end of if block |
| `br` | `JMP` | Unconditional branch |
| `br_if` | `JMPIF` | Conditional branch |
| `br_table` | Multiple `JMPIF` | Jump table |
| `return` | `RET` | Return from function |
| **Function Calls** |  |  |
| `call` | `CALL` | Direct function call |
| `call_indirect` | `CALL_I` | Indirect function call |

## Implementation Details

### Control Flow Translation

Converting WASM's structured control flow to Neo VM's jump-based control flow requires careful tracking of block boundaries and destinations:

```rust
// Example pseudocode for implementing WASM blocks
fn process_block(&mut self, block: &WasmBlock) -> Result<()> {
    // Generate unique labels for block boundaries
    let end_label = self.generate_label();
    
    // Register this block in our control flow stack
    self.control_stack.push(ControlFrame {
        label: end_label,
        block_type: BlockType::Block,
    });
    
    // Process block body
    self.process_instructions(&block.instructions)?;
    
    // Generate end label
    self.emit_label(end_label);
    
    // Pop control frame
    self.control_stack.pop();
    
    Ok(())
}

// Handling a br (branch) instruction
fn process_br(&mut self, depth: u32) -> Result<()> {
    // Find the target block in the control stack
    let target = self.control_stack.get_target(depth)?;
    
    // Emit jump to the block's end label
    self.emit(Opcode::JMP, &[target.label.into()])?;
    
    Ok(())
}
```

### Memory Model Adaptation

WASM's linear memory model must be adapted to Neo's key-value storage:

```rust
// Example pseudocode for implementing memory operations
fn process_i32_load(&mut self, offset: u32, align: u32) -> Result<()> {
    // Calculate storage key from address on stack
    self.emit(Opcode::PUSHINT, &[offset.into()])?;
    self.emit(Opcode::ADD)?;  // Add offset to address
    
    // Key is now on stack, perform storage lookup
    self.emit_storage_get()?;
    
    Ok(())
}

fn process_i32_store(&mut self, offset: u32, align: u32) -> Result<()> {
    // Stack has: value, address
    self.emit(Opcode::SWAP)?;  // Swap to: address, value
    
    // Calculate storage key from address
    self.emit(Opcode::PUSHINT, &[offset.into()])?;
    self.emit(Opcode::ADD)?;  // Add offset to address
    
    // Stack now has: key, value
    self.emit_storage_put()?;
    
    Ok(())
}

// Helper for storage operations
fn emit_storage_get(&mut self) -> Result<()> {
    // Neo VM storage get operation
    self.emit(Opcode::SYSCALL, &["System.Storage.Get".into()])?;
    Ok(())
}

fn emit_storage_put(&mut self) -> Result<()> {
    // Neo VM storage put operation
    self.emit(Opcode::SYSCALL, &["System.Storage.Put".into()])?;
    Ok(())
}
```

### Type Handling

Handling type differences between WASM and Neo VM:

```rust
// Example pseudocode for type conversions
fn handle_i64_operations(&mut self, op: WasmOp) -> Result<()> {
    match op {
        WasmOp::I64Add => self.emit(Opcode::ADD)?,
        WasmOp::I64Sub => self.emit(Opcode::SUB)?,
        WasmOp::I64Mul => self.emit(Opcode::MUL)?,
        // For 64-bit specific operations, we may need special handling
        WasmOp::I64DivU => {
            // Implement unsigned 64-bit division using Neo VM operations
            self.emit_unsigned_div()?;
        },
        // Other operations...
    }
    Ok(())
}

// Custom implementation for operations not directly supported
fn emit_unsigned_div(&mut self) -> Result<()> {
    // Implementation for unsigned division using available Neo VM opcodes
    // This may involve a series of operations or calling a helper function
    
    // Example: Convert to signed div with appropriate checks
    // ...
    
    Ok(())
}
```

## Function Table and Indirect Calls

WASM supports indirect calls via a function table. This needs special handling in Neo VM:

```rust
// Example pseudocode for handling function tables
fn initialize_function_table(&mut self, module: &WasmModule) -> Result<()> {
    // Generate a dispatch table in the Neo contract
    self.emit_section_start("function_table")?;
    
    // For each function in the table
    for (i, func_idx) in module.function_table.iter().enumerate() {
        // Emit a label for this table entry
        self.emit_label(format!("table_entry_{}", i))?;
        
        // Emit a jump to the actual function
        let func_label = self.function_labels[*func_idx as usize];
        self.emit(Opcode::JMP, &[func_label.into()])?;
    }
    
    self.emit_section_end()?;
    
    Ok(())
}

// Handling call_indirect instruction
fn process_call_indirect(&mut self, type_idx: u32) -> Result<()> {
    // Check bounds
    self.emit(Opcode::DUP)?;  // Duplicate the table index
    self.emit(Opcode::PUSHINT, &[self.module.function_table.len().into()])?;
    self.emit(Opcode::GE)?;
    
    // If index >= table size, throw error
    let error_label = self.generate_label();
    let continue_label = self.generate_label();
    self.emit(Opcode::JMPIF, &[error_label.into()])?;
    
    // Index is valid, calculate jump address
    self.emit(Opcode::PUSHINT, &[4.into()])?;  // Assuming each table entry is 4 bytes
    self.emit(Opcode::MUL)?;  // Multiply index by entry size
    self.emit(Opcode::PUSHINT, &[self.get_table_base_address().into()])?;
    self.emit(Opcode::ADD)?;  // Add to table base address
    
    // Jump to the calculated address
    self.emit(Opcode::JMP_I)?;
    
    // Error handling
    self.emit_label(error_label)?;
    self.emit_runtime_error("function table index out of bounds")?;
    
    // Continue label
    self.emit_label(continue_label)?;
    
    Ok(())
}
```

## Handling WASM Binary Format

The WASM binary format must be parsed to extract instructions and other module information:

```rust
// Example pseudocode for WASM parsing
fn parse_wasm_module(wasm_bytes: &[u8]) -> Result<WasmModule> {
    // Check magic number and version
    let (magic, rest) = wasm_bytes.split_at(4);
    assert_eq!(magic, [0x00, 0x61, 0x73, 0x6D], "Invalid WASM magic number");
    
    let (version, rest) = rest.split_at(4);
    assert_eq!(version, [0x01, 0x00, 0x00, 0x00], "Unsupported WASM version");
    
    // Parse sections
    let mut module = WasmModule::new();
    let mut parser = WasmParser::new(rest);
    
    while !parser.is_empty() {
        let section = parser.parse_section()?;
        match section {
            WasmSection::Type(types) => module.types = types,
            WasmSection::Function(funcs) => module.functions = funcs,
            WasmSection::Memory(mems) => module.memories = mems,
            WasmSection::Export(exports) => module.exports = exports,
            WasmSection::Code(code) => module.code = code,
            // Handle other sections...
        }
    }
    
    Ok(module)
}
```

## Neo VM Syscalls

Neo VM provides syscalls for interacting with the blockchain. These must be mapped from WASM imported functions:

```rust
// Example pseudocode for mapping WASM imports to Neo syscalls
fn map_import_to_syscall(&mut self, import: &WasmImport) -> Result<()> {
    match (import.module.as_str(), import.name.as_str()) {
        ("env", "timestamp") => {
            // Map to System.Runtime.GetTime syscall
            self.emit(Opcode::SYSCALL, &["System.Runtime.GetTime".into()])?;
        },
        ("env", "storage_read") => {
            // Map to System.Storage.Get syscall
            self.emit(Opcode::SYSCALL, &["System.Storage.Get".into()])?;
        },
        ("env", "storage_write") => {
            // Map to System.Storage.Put syscall
            self.emit(Opcode::SYSCALL, &["System.Storage.Put".into()])?;
        },
        // Handle other imports...
        _ => {
            return Err(Error::UnknownImport(format!(
                "Unknown import: {}.{}", 
                import.module, import.name
            )));
        }
    }
    
    Ok(())
}
```

## Challenges and Solutions

### 1. Integer Size and Overflow

**Challenge**: WASM distinguishes between 32-bit and 64-bit integers, while Neo VM has a single integer type.

**Solution**: Use Neo VM's biginteger support for 64-bit operations, with careful handling of overflow and underflow conditions.

### 2. Floating Point Support

**Challenge**: WASM supports floating-point operations, while Neo VM does not have native floating-point support.

**Solution**: Either:
1. Reject contracts that use floating-point operations
2. Implement floating-point operations using integer math
3. Provide a fixed-point decimal library

### 3. Stack Management

**Challenge**: WASM and Neo VM have different stack behaviors.

**Solution**: Carefully track stack state during compilation and insert stack manipulation opcodes (SWAP, DUP, etc.) as needed.

### 4. Memory Model

**Challenge**: Adapting WASM's linear memory to Neo's key-value storage.

**Solution**: Implement a memory mapper that:
1. Assigns storage keys based on memory addresses
2. Handles alignment and multi-byte operations
3. Optimizes frequently accessed memory regions

### 5. Exception Handling

**Challenge**: WASM has structured exceptions, while Neo VM uses THROW/TRY/CATCH.

**Solution**: Insert appropriate exception handling code and maintain an exception table for proper handling.

## Optimization Techniques

To generate efficient Neo VM bytecode from WASM, several optimization techniques are applied:

### 1. Constant Folding

Evaluate constant expressions at compile time:

```rust
// Before optimization:
// PUSHINT 2
// PUSHINT 3
// ADD

// After optimization:
// PUSHINT 5
```

### 2. Dead Code Elimination

Remove unreachable or unused code:

```rust
// Before optimization:
// JMP label1
// PUSHINT 1  // Unreachable
// ADD        // Unreachable
// label1:
// PUSHINT 2

// After optimization:
// JMP label1
// label1:
// PUSHINT 2
```

### 3. Stack Operation Optimization

Eliminate redundant stack operations:

```rust
// Before optimization:
// PUSHINT 1
// DUP
// SWAP
// ADD

// After optimization:
// PUSHINT 1
// DUP
// ADD
```

### 4. Inlining

For small functions, inline the function body rather than making a call:

```rust
// Before optimization:
// PUSHINT 1
// CALL function_add_one
// ...
// function_add_one:
// PUSHINT 1
// ADD
// RET

// After optimization:
// PUSHINT 1
// PUSHINT 1
// ADD
// ...
// function_add_one:  // might be removed if not called elsewhere
// PUSHINT 1
// ADD
// RET
```

## Testing and Validation

The WASM to Neo VM conversion should be extensively tested:

1. **Unit Tests**: Test individual instruction mappings
2. **Integration Tests**: Test complete functions and modules
3. **Reference Tests**: Compare against known correct Neo VM bytecode
4. **Gas Estimation**: Verify gas costs are reasonable
5. **Contract Testing**: Test converted contracts on Neo blockchain testnet

## Conclusion

Converting WASM to Neo VM opcodes is a complex but solvable challenge. The key is to carefully map between instruction sets while handling differences in type systems, memory models, and control flow mechanisms.

By following the strategies outlined in this document, the Neo Contract Rust Framework can provide an efficient compilation pipeline from Rust source code through WASM to Neo VM bytecode, enabling developers to write smart contracts in Rust for the Neo blockchain.