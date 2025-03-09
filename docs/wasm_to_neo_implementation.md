# WebAssembly to Neo VM Implementation Details

This document provides technical details about the implementation of the WebAssembly to Neo VM conversion process in the Neo Contract Rust Framework. It's intended for developers who want to understand the internals of the compiler or contribute to its development.

## Compiler Architecture

The WASM to Neo VM compiler is structured in a multi-stage pipeline:

```
┌───────────────┐     ┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│               │     │               │     │               │     │               │
│  WASM Parser  │ --> │ IR Generator  │ --> │ Optimizer    │ --> │ Neo Bytecode  │
│               │     │               │     │               │     │  Generator    │
│               │     │               │     │               │     │               │
└───────────────┘     └───────────────┘     └───────────────┘     └───────────────┘
        │                     │                     │                     │
        ▼                     ▼                     ▼                     ▼
┌───────────────┐     ┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│               │     │               │     │               │     │               │
│  WASM Module  │     │ Intermediate  │     │  Optimized    │     │    NEF File   │
│   Structure   │     │ Representation│     │      IR       │     │ & Manifest    │
│               │     │               │     │               │     │               │
└───────────────┘     └───────────────┘     └───────────────┘     └───────────────┘
```

### 1. WASM Parser

The WebAssembly parser uses the [`walrus`](https://github.com/rustwasm/walrus) crate to parse and analyze WebAssembly modules. It extracts:

- Function definitions and signatures
- Global variables
- Memory sections
- Import/export declarations
- Custom sections (metadata)

Key implementation aspects:

```rust
pub fn parse_wasm_module(wasm_bytes: &[u8]) -> Result<WasmModule, CompilerError> {
    // Parse the WebAssembly binary
    let module = match walrus::Module::from_buffer(wasm_bytes) {
        Ok(module) => module,
        Err(e) => return Err(CompilerError::WasmParsingError(e.to_string())),
    };
    
    // Extract module information
    let functions = extract_functions(&module)?;
    let exports = extract_exports(&module)?;
    let imports = extract_imports(&module)?;
    let globals = extract_globals(&module)?;
    let memory = extract_memory_config(&module)?;
    
    // Create our internal module representation
    Ok(WasmModule {
        functions,
        exports,
        imports,
        globals,
        memory,
    })
}
```

### 2. Intermediate Representation (IR)

The compiler uses a custom intermediate representation to facilitate the translation from WASM to Neo VM:

```rust
pub struct IR {
    /// Basic blocks of instructions
    pub blocks: Vec<BasicBlock>,
    /// Function table
    pub functions: Vec<Function>,
    /// Global variables
    pub globals: Vec<Global>,
    /// Type information
    pub types: TypeInfo,
}

pub struct BasicBlock {
    /// Block ID
    pub id: BlockId,
    /// Instructions in this block
    pub instructions: Vec<IRInstruction>,
    /// Control flow edges
    pub edges: Vec<BlockId>,
}

pub enum IRInstruction {
    // Stack operations
    Push(Value),
    Pop,
    Dup,
    Swap,
    
    // Arithmetic
    Add(ValueType),
    Sub(ValueType),
    Mul(ValueType),
    Div(ValueType),
    Mod(ValueType),
    
    // Memory operations
    Load(ValueType, u32),
    Store(ValueType, u32),
    
    // Control flow
    Jump(BlockId),
    JumpIf(BlockId),
    JumpIfNot(BlockId),
    Call(FunctionId),
    Return,
    
    // System calls
    SysCall(String),
    
    // Other
    Nop,
}
```

The IR generator translates WASM instructions to this intermediate representation:

```rust
pub fn generate_ir(module: &WasmModule) -> Result<IR, CompilerError> {
    let mut ir_builder = IRBuilder::new();
    
    // Process globals
    for global in &module.globals {
        ir_builder.add_global(global)?;
    }
    
    // Process functions
    for function in &module.functions {
        let function_id = ir_builder.start_function(function)?;
        
        for instruction in &function.instructions {
            match instruction {
                WasmInstruction::I32Const(val) => {
                    ir_builder.emit(IRInstruction::Push(Value::I32(*val)));
                },
                WasmInstruction::I32Add => {
                    ir_builder.emit(IRInstruction::Add(ValueType::I32));
                },
                // ... more instruction translations
            }
        }
        
        ir_builder.finish_function(function_id)?;
    }
    
    Ok(ir_builder.build())
}
```

### 3. Optimization

The optimizer performs various transformations on the IR to improve the efficiency of the generated Neo VM code:

```rust
pub fn optimize_ir(ir: &mut IR) -> Result<(), CompilerError> {
    // Constant folding
    constant_folding(ir)?;
    
    // Dead code elimination
    eliminate_dead_code(ir)?;
    
    // Instruction combining
    combine_instructions(ir)?;
    
    // Tail call optimization
    optimize_tail_calls(ir)?;
    
    // Register allocation
    allocate_registers(ir)?;
    
    Ok(())
}
```

Key optimization passes:

1. **Constant Folding**: Evaluates constant expressions at compile time
   ```rust
   fn constant_folding(ir: &mut IR) -> Result<(), CompilerError> {
       for block in &mut ir.blocks {
           let mut i = 0;
           while i < block.instructions.len() - 1 {
               // Check for push followed by operations on constants
               if let IRInstruction::Push(v1) = block.instructions[i] {
                   if let IRInstruction::Push(v2) = block.instructions[i + 1] {
                       if let IRInstruction::Add(vtype) = block.instructions[i + 2] {
                           // Replace with a single push of the computed value
                           let result = compute_binary_op(v1, v2, BinaryOp::Add, vtype)?;
                           block.instructions[i] = IRInstruction::Push(result);
                           block.instructions.remove(i + 1);
                           block.instructions.remove(i + 1);
                           continue;
                       }
                   }
               }
               i += 1;
           }
       }
       Ok(())
   }
   ```

2. **Dead Code Elimination**: Removes unreachable or ineffectual code
   ```rust
   fn eliminate_dead_code(ir: &mut IR) -> Result<(), CompilerError> {
       // Mark reachable blocks
       let mut reachable = vec![false; ir.blocks.len()];
       mark_reachable_blocks(ir, 0, &mut reachable);
       
       // Remove unreachable blocks
       ir.blocks.retain(|block| reachable[block.id.0 as usize]);
       
       // Eliminate other dead code within blocks
       // (e.g., operations whose results are never used)
       for block in &mut ir.blocks {
           eliminate_dead_instructions(block);
       }
       
       Ok(())
   }
   ```

3. **Instruction Combining**: Merges multiple instructions into more efficient equivalents
   ```rust
   fn combine_instructions(ir: &mut IR) -> Result<(), CompilerError> {
       for block in &mut ir.blocks {
           let mut i = 0;
           while i < block.instructions.len() - 1 {
               // Check for push followed by pop (can eliminate both)
               if let IRInstruction::Push(_) = block.instructions[i] {
                   if let IRInstruction::Pop = block.instructions[i + 1] {
                       block.instructions.remove(i);
                       block.instructions.remove(i);
                       continue;
                   }
               }
               
               // Check for dup followed by pop (can eliminate both)
               if let IRInstruction::Dup = block.instructions[i] {
                   if let IRInstruction::Pop = block.instructions[i + 1] {
                       block.instructions.remove(i);
                       block.instructions.remove(i);
                       continue;
                   }
               }
               
               i += 1;
           }
       }
       Ok(())
   }
   ```

### 4. Neo VM Bytecode Generation

The final stage translates the optimized IR to Neo VM bytecode and packages it into a NEF file with a manifest:

```rust
pub fn generate_neo_bytecode(ir: &IR) -> Result<NeoByteCode, CompilerError> {
    let mut bytecode = Vec::new();
    let mut method_token_map = HashMap::new();
    
    // Generate bytecode for each function
    for function in &ir.functions {
        let offset = bytecode.len();
        method_token_map.insert(function.id, offset);
        
        for block_id in &function.blocks {
            let block = &ir.blocks[*block_id as usize];
            
            for instruction in &block.instructions {
                match instruction {
                    IRInstruction::Push(Value::I32(val)) => {
                        emit_push_int(&mut bytecode, *val);
                    },
                    IRInstruction::Add(_) => {
                        bytecode.push(NeoOpcode::Add as u8);
                    },
                    // ... more instruction translations
                    IRInstruction::SysCall(name) => {
                        emit_syscall(&mut bytecode, name);
                    },
                    // ... other instructions
                }
            }
        }
    }
    
    Ok(NeoByteCode {
        bytecode,
        method_token_map,
    })
}
```

## Handling WASM Features

### Memory Management

WebAssembly has a linear memory model which must be mapped to Neo VM's storage system:

```rust
fn translate_memory_access(
    ir_builder: &mut IRBuilder,
    access_type: MemoryAccessType,
    offset: u32,
    value_type: ValueType,
) -> Result<(), CompilerError> {
    // For loads
    if access_type == MemoryAccessType::Load {
        // 1. Generate code to compute storage key from memory offset
        ir_builder.emit(IRInstruction::Push(Value::I32(offset)));
        ir_builder.emit(IRInstruction::SysCall("Storage.GetContext".to_string()));
        
        // 2. Load value from storage
        ir_builder.emit(IRInstruction::SysCall("Storage.Get".to_string()));
        
        // 3. Convert loaded value based on value_type
        match value_type {
            ValueType::I32 => {
                ir_builder.emit(IRInstruction::SysCall("Binary.ToInteger".to_string()));
            },
            ValueType::I64 => {
                ir_builder.emit(IRInstruction::SysCall("Binary.ToInteger".to_string()));
            },
            // ... handle other types
        }
    } 
    // For stores
    else {
        // 1. Convert value based on value_type
        match value_type {
            ValueType::I32 | ValueType::I64 => {
                ir_builder.emit(IRInstruction::SysCall("Integer.ToBinary".to_string()));
            },
            // ... handle other types
        }
        
        // 2. Generate code to compute storage key from memory offset
        ir_builder.emit(IRInstruction::Push(Value::I32(offset)));
        ir_builder.emit(IRInstruction::SysCall("Storage.GetContext".to_string()));
        
        // 3. Store value to storage
        ir_builder.emit(IRInstruction::SysCall("Storage.Put".to_string()));
    }
    
    Ok(())
}
```

### Function Call Translation

WebAssembly function calls are mapped to Neo VM call operations:

```rust
fn translate_call(
    ir_builder: &mut IRBuilder,
    function_idx: u32,
    module: &WasmModule,
) -> Result<(), CompilerError> {
    // Check if this is an internal or imported function
    if let Some(function) = module.get_function_by_idx(function_idx) {
        // Internal function - use simple call
        ir_builder.emit(IRInstruction::Call(function.id));
    } else if let Some(import) = module.get_imported_function(function_idx) {
        // Imported function - use syscall or contract call
        match import.module.as_str() {
            "env" => {
                // Handle host environment functions
                translate_env_function(ir_builder, &import.name)?;
            },
            "neo" => {
                // Handle Neo-specific imports
                translate_neo_function(ir_builder, &import.name)?;
            },
            // Handle contract calls for other modules
            _ => {
                // Assumed to be another contract's hash
                translate_contract_call(ir_builder, &import.module, &import.name)?;
            }
        }
    } else {
        return Err(CompilerError::UnknownFunction(function_idx));
    }
    
    Ok(())
}
```

### Control Flow

WebAssembly's structured control flow (blocks, loops, if/else) is translated to Neo VM's jump instructions:

```rust
fn translate_control_flow(
    ir_builder: &mut IRBuilder,
    instructions: &[WasmInstruction],
    pc: &mut usize,
) -> Result<(), CompilerError> {
    match &instructions[*pc] {
        WasmInstruction::Block(block_type) => {
            // Create a new block for the code after this block
            let end_block = ir_builder.create_block();
            
            // Push the end block to block stack
            ir_builder.push_block(end_block);
            
            // Increase PC to process block contents
            *pc += 1;
            
            // Continue processing instructions until end
            while *pc < instructions.len() && instructions[*pc] != WasmInstruction::End {
                translate_instruction(ir_builder, instructions, pc)?;
            }
            
            // Pop the end block from stack
            ir_builder.pop_block();
            
            // Now we're at the end instruction
            *pc += 1;
        },
        
        WasmInstruction::Loop(block_type) => {
            // Create a block for the loop body
            let loop_block = ir_builder.create_block();
            
            // Jump to the loop block
            ir_builder.emit(IRInstruction::Jump(loop_block));
            
            // Set current block to loop block
            ir_builder.set_current_block(loop_block);
            
            // Push the loop block to block stack (for br instructions)
            ir_builder.push_block(loop_block);
            
            // Increase PC to process loop contents
            *pc += 1;
            
            // Continue processing instructions until end
            while *pc < instructions.len() && instructions[*pc] != WasmInstruction::End {
                translate_instruction(ir_builder, instructions, pc)?;
            }
            
            // Pop the loop block from stack
            ir_builder.pop_block();
            
            // Add jump back to the beginning of the loop
            ir_builder.emit(IRInstruction::Jump(loop_block));
            
            // Create new block for code after the loop
            let after_loop = ir_builder.create_block();
            ir_builder.set_current_block(after_loop);
            
            // Now we're at the end instruction
            *pc += 1;
        },
        
        WasmInstruction::If(block_type) => {
            // Create blocks for then, else, and after
            let then_block = ir_builder.create_block();
            let else_block = ir_builder.create_block();
            let end_block = ir_builder.create_block();
            
            // Pop condition and branch
            ir_builder.emit(IRInstruction::JumpIf(then_block));
            ir_builder.emit(IRInstruction::Jump(else_block));
            
            // Process then block
            ir_builder.set_current_block(then_block);
            
            // Push end block to stack for br instructions
            ir_builder.push_block(end_block);
            
            // Advance PC
            *pc += 1;
            
            // Process instructions until else or end
            while *pc < instructions.len() && 
                  instructions[*pc] != WasmInstruction::Else && 
                  instructions[*pc] != WasmInstruction::End {
                translate_instruction(ir_builder, instructions, pc)?;
            }
            
            // Add jump to end
            ir_builder.emit(IRInstruction::Jump(end_block));
            
            // Check if we have an else block
            if *pc < instructions.len() && instructions[*pc] == WasmInstruction::Else {
                // Process else block
                ir_builder.set_current_block(else_block);
                
                // Advance PC past else
                *pc += 1;
                
                // Process instructions until end
                while *pc < instructions.len() && instructions[*pc] != WasmInstruction::End {
                    translate_instruction(ir_builder, instructions, pc)?;
                }
                
                // Add jump to end
                ir_builder.emit(IRInstruction::Jump(end_block));
            } else {
                // No else block, just jump to end
                ir_builder.set_current_block(else_block);
                ir_builder.emit(IRInstruction::Jump(end_block));
            }
            
            // Pop end block from stack
            ir_builder.pop_block();
            
            // Set current block to end
            ir_builder.set_current_block(end_block);
            
            // Now we're at the end instruction
            *pc += 1;
        },
        
        // ... other control flow instructions (br, br_if, etc.)
    }
    
    Ok(())
}
```

## Type Mapping

WebAssembly has a limited type system that must be mapped to Neo VM's types:

| WASM Type | Neo VM Type | Notes |
|-----------|-------------|-------|
| `i32` | Integer | 32-bit integers |
| `i64` | Integer | 64-bit integers |
| `f32` | Integer | Floating-point is emulated |
| `f64` | Integer | Floating-point is emulated |
| Function reference | Integer | Function pointers via token IDs |
| Reference types | ByteArray | References are serialized |

```rust
fn translate_type(wasm_type: &WasmType) -> Result<NeoType, CompilerError> {
    match wasm_type {
        WasmType::I32 => Ok(NeoType::Integer),
        WasmType::I64 => Ok(NeoType::Integer),
        WasmType::F32 => Ok(NeoType::Integer), // Floating-point emulated
        WasmType::F64 => Ok(NeoType::Integer), // Floating-point emulated
        WasmType::FuncRef => Ok(NeoType::Integer),
        WasmType::ExternRef => Ok(NeoType::ByteArray),
        _ => Err(CompilerError::UnsupportedType(format!("{:?}", wasm_type))),
    }
}
```

## Manifest Generation

The Neo VM requires a contract manifest that describes the contract's interface. The compiler extracts this information from the WASM module's exports:

```rust
pub fn generate_manifest(
    module: &WasmModule,
    name: &str,
    author: &str,
    description: &str,
) -> Result<ContractManifest, CompilerError> {
    // Create methods from exported functions
    let mut methods = Vec::new();
    
    for export in &module.exports {
        if export.kind == ExportKind::Function {
            let function = module.get_function_by_idx(export.index)
                .ok_or_else(|| CompilerError::UnknownFunction(export.index))?;
            
            // Convert function signature to ABI method
            let parameters = function.type_signature.params
                .iter()
                .enumerate()
                .map(|(i, ty)| {
                    let neo_type = translate_type(ty)?;
                    Ok(Parameter {
                        name: format!("param{}", i),
                        type_: neo_type,
                    })
                })
                .collect::<Result<Vec<_>, CompilerError>>()?;
            
            let return_type = if function.type_signature.returns.is_empty() {
                NeoType::Void
            } else {
                translate_type(&function.type_signature.returns[0])?
            };
            
            methods.push(ContractMethod {
                name: export.name.clone(),
                parameters,
                return_type,
                offset: 0, // Will be filled in later
                safe: false, // Default to false, will be updated based on attributes
            });
        }
    }
    
    // Extract events from custom sections
    let events = extract_events_from_custom_sections(module)?;
    
    // Extract supported standards from custom sections
    let supported_standards = extract_standards_from_custom_sections(module)?;
    
    // Create default permissions
    let permissions = vec![
        Permission {
            contract: "*".to_string(),
            methods: vec!["*".to_string()],
        }
    ];
    
    // Assemble the manifest
    Ok(ContractManifest {
        name: name.to_string(),
        groups: Vec::new(),
        supported_standards,
        abi: ContractABI {
            methods,
            events,
        },
        permissions,
        trusts: Vec::new(),
        extra: ContractExtra {
            author: author.to_string(),
            email: "".to_string(),
            description: description.to_string(),
        },
    })
}
```

## NEF File Generation

The Neo Executable Format (NEF) file contains the compiled bytecode and metadata:

```rust
pub fn generate_nef_file(
    bytecode: &[u8],
    compiler_name: &str,
) -> Result<NefFile, CompilerError> {
    // Calculate bytecode checksum
    let checksum = calculate_checksum(bytecode)?;
    
    // Create NEF file
    let nef = NefFile {
        magic: NEF_MAGIC, // "NEF3"
        compiler: compiler_name.to_string(),
        source: "".to_string(), // Optional source info
        tokens: Vec::new(), // Method tokens
        script: bytecode.to_vec(),
        checksum,
    };
    
    Ok(nef)
}
```

## Syscall Integration

Neo provides system calls for blockchain interaction. The compiler maps framework calls to appropriate Neo syscalls:

```rust
fn translate_neo_function(
    ir_builder: &mut IRBuilder,
    function_name: &str,
) -> Result<(), CompilerError> {
    match function_name {
        "Runtime.CheckWitness" => {
            ir_builder.emit(IRInstruction::SysCall("System.Runtime.CheckWitness".to_string()));
        },
        "Storage.Get" => {
            ir_builder.emit(IRInstruction::SysCall("System.Storage.Get".to_string()));
        },
        "Storage.Put" => {
            ir_builder.emit(IRInstruction::SysCall("System.Storage.Put".to_string()));
        },
        // ... more Neo syscalls
        _ => {
            return Err(CompilerError::UnknownNeoFunction(function_name.to_string()));
        }
    }
    
    Ok(())
}
```

## Error Handling and Debugging

The compiler provides detailed error messages and source mapping for debugging:

```rust
pub enum CompilerError {
    WasmParsingError(String),
    InvalidWasmModule(String),
    UnsupportedFeature(String),
    UnknownFunction(u32),
    UnknownNeoFunction(String),
    UnsupportedType(String),
    InvalidMemoryAccess(String),
    StackImbalance(String),
    InternalError(String),
}

struct SourceMapping {
    wasm_offset: u32,
    neo_offset: u32,
    original_source_location: Option<SourceLocation>,
}

struct SourceLocation {
    file: String,
    line: u32,
    column: u32,
}
```

## Performance Optimizations

Several optimizations are applied to improve the performance of generated code:

1. **Instruction Fusion**: Combining common instruction sequences
   ```rust
   // Before optimization:
   //    PUSH1      // Push 1 onto stack
   //    PUSH2      // Push 2 onto stack
   //    ADD        // Add them
   //
   // After optimization:
   //    PUSH3      // Push result directly
   ```

2. **Stack Caching**: Tracking stack values to avoid redundant operations
   ```rust
   // Before optimization:
   //    PUSH "key" // Push key
   //    DUP        // Duplicate for later use
   //    PUSH value // Push value
   //    SETITEM    // Store in map
   //    PUSH "key" // Push key again
   //    GETITEM    // Read from map
   //
   // After optimization:
   //    PUSH "key" // Push key
   //    PUSH value // Push value
   //    SETITEM    // Store in map
   //    PUSH value // Push value directly instead of reading back
   ```

3. **Peephole Optimization**: Examining small windows of instructions for patterns
   ```rust
   // Before optimization:
   //    JMP label1
   //    label1: JMP label2
   //
   // After optimization:
   //    JMP label2
   ```

## Future Work

Areas for further improvement in the compiler:

1. **More Aggressive Optimizations**: Applying more advanced compiler optimizations

2. **Better Stack Management**: Optimizing stack operations to reduce overhead

3. **Advanced Type Handling**: Improving mapping of complex types

4. **Floating-Point Support**: Better handling of floating-point operations

5. **Contract Splitting**: Techniques for handling large contracts by splitting them

6. **Memory Management**: More efficient storage access patterns

7. **Cross-Contract Calls**: Optimized patterns for contract-to-contract interaction

## Conclusion

The WebAssembly to Neo VM compiler is a critical component of the Neo Contract Rust Framework. By translating Rust code (via WASM) to Neo VM bytecode, it enables developers to write smart contracts in Rust while targeting the Neo N3 blockchain. The multi-stage pipeline with parsing, IR generation, optimization, and code generation provides a flexible and extendable architecture for future improvements.

For implementation details of specific components, refer to the source code in the `neo-compiler` crate.