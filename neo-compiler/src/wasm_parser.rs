use anyhow::Result;
use wasmparser::Operator;
use std::collections::HashMap;

use crate::opcodes::OpCode;
use crate::memory_model::{MemoryModelTranslator, MemoryOperation, MemoryOperationType};

/// WASM instruction parser and translator
pub struct WasmInstructionParser {
    /// Current instruction offset
    pub offset: usize,
    /// Local variable types
    pub locals: Vec<wasmparser::ValType>,
    /// Label stack for control flow
    pub label_stack: Vec<ControlLabel>,
    /// Instruction mapping to NEO opcodes
    pub instruction_map: HashMap<u8, InstructionMapping>,
    /// Memory model translator
    pub memory_translator: MemoryModelTranslator,
}

#[derive(Debug, Clone)]
pub struct ControlLabel {
    pub label_type: ControlType,
    pub start_offset: usize,
    pub end_offset: Option<usize>,
    pub continuation_offset: Option<usize>,
    pub jump_positions: Vec<usize>, // Positions that need to be patched
}

#[derive(Debug, Clone)]
pub enum ControlType {
    Block,
    Loop,
    If,
    Else,
    Try,
    Catch,
}

#[derive(Debug, Clone)]
pub struct InstructionMapping {
    pub wasm_opcode: u8,
    pub neo_opcodes: Vec<OpCode>,
    pub immediate_handler: Option<fn(&[u8]) -> Vec<u8>>,
    pub stack_effect: i32, // Net stack change
}

impl WasmInstructionParser {
    pub fn new() -> Self {
        let mut parser = Self {
            offset: 0,
            locals: Vec::new(),
            label_stack: Vec::new(),
            instruction_map: HashMap::new(),
            memory_translator: MemoryModelTranslator::new(),
        };
        
        parser.initialize_instruction_mappings();
        parser
    }

    /// Initialize WASM to NEO instruction mappings
    fn initialize_instruction_mappings(&mut self) {
        // Arithmetic Operations
        self.add_mapping(0x6A, vec![OpCode::Add], 0); // i32.add
        self.add_mapping(0x6B, vec![OpCode::Sub], 0); // i32.sub
        self.add_mapping(0x6C, vec![OpCode::Mul], 0); // i32.mul
        self.add_mapping(0x6D, vec![OpCode::Div], 0); // i32.div_s
        self.add_mapping(0x6E, vec![OpCode::Div], 0); // i32.div_u
        self.add_mapping(0x6F, vec![OpCode::Mod], 0); // i32.rem_s
        self.add_mapping(0x70, vec![OpCode::Mod], 0); // i32.rem_u

        // Bitwise Operations
        self.add_mapping(0x71, vec![OpCode::And], 0); // i32.and
        self.add_mapping(0x72, vec![OpCode::Or], 0);  // i32.or
        self.add_mapping(0x73, vec![OpCode::Xor], 0); // i32.xor
        self.add_mapping(0x74, vec![OpCode::Shl], 0); // i32.shl
        self.add_mapping(0x75, vec![OpCode::Shr], 0); // i32.shr_s
        self.add_mapping(0x76, vec![OpCode::Shr], 0); // i32.shr_u

        // Comparison Operations
        self.add_mapping(0x46, vec![OpCode::Equal], 0);     // i32.eq
        self.add_mapping(0x47, vec![OpCode::NotEqual], 0);  // i32.ne
        self.add_mapping(0x48, vec![OpCode::Lt], 0);        // i32.lt_s
        self.add_mapping(0x49, vec![OpCode::Lt], 0);        // i32.lt_u
        self.add_mapping(0x4A, vec![OpCode::Gt], 0);        // i32.gt_s
        self.add_mapping(0x4B, vec![OpCode::Gt], 0);        // i32.gt_u
        self.add_mapping(0x4C, vec![OpCode::Le], 0);        // i32.le_s
        self.add_mapping(0x4D, vec![OpCode::Le], 0);        // i32.le_u
        self.add_mapping(0x4E, vec![OpCode::Ge], 0);        // i32.ge_s
        self.add_mapping(0x4F, vec![OpCode::Ge], 0);        // i32.ge_u

        // Memory Operations
        self.add_mapping(0x28, vec![OpCode::LdArg], 1);     // i32.load
        self.add_mapping(0x36, vec![OpCode::StArg], -1);    // i32.store

        // Local Variable Operations
        self.add_mapping(0x20, vec![OpCode::LdLocal], 1);   // local.get
        self.add_mapping(0x21, vec![OpCode::StLocal], -1);  // local.set
        self.add_mapping(0x22, vec![OpCode::Dup, OpCode::StLocal], 0); // local.tee

        // Control Flow
        self.add_mapping(0x0F, vec![OpCode::Ret], 0);       // return
        self.add_mapping(0x10, vec![OpCode::Call], 0);      // call

        // Constants
        self.add_mapping(0x41, vec![], 1); // i32.const (handled specially)
        self.add_mapping(0x42, vec![], 1); // i64.const (handled specially)

        // Stack Operations
        self.add_mapping(0x1A, vec![OpCode::Drop], -1);     // drop
        self.add_mapping(0x1B, vec![OpCode::Dup], 1);       // select (simplified)
    }

    fn add_mapping(&mut self, wasm_opcode: u8, neo_opcodes: Vec<OpCode>, stack_effect: i32) {
        self.instruction_map.insert(wasm_opcode, InstructionMapping {
            wasm_opcode,
            neo_opcodes,
            immediate_handler: None,
            stack_effect,
        });
    }

    /// Parse a WASM function body and generate NEO opcodes (simplified version)
    pub fn parse_function_body_simple(
        &mut self, 
        body: &[u8], 
        locals: &[(u32, wasmparser::ValType)]
    ) -> Result<Vec<u8>> {
        self.locals.clear();
        
        // Add parameters and locals
        for (count, val_type) in locals {
            for _ in 0..*count {
                self.locals.push(*val_type);
            }
        }

        let mut neo_bytecode = Vec::new();
        
        // Parse WASM operators directly from binary reader
        let mut binary_reader = wasmparser::BinaryReader::new(body);
        
        while !binary_reader.eof() {
            let op = binary_reader.read_operator()?;
            
            // Validate operator based on current context
            self.validate_operator(&op)?;
            
            self.translate_operator(&op, &mut neo_bytecode)?;
            self.offset = binary_reader.original_position();
        }

        Ok(neo_bytecode)
    }

    /// Translate a single WASM operator to NEO opcodes
    pub fn translate_operator(&mut self, op: &Operator, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        match op {
            // Constants
            Operator::I32Const { value } => {
                self.emit_push_i32(*value, neo_bytecode)?;
            },
            Operator::I64Const { value } => {
                self.emit_push_i64(*value, neo_bytecode)?;
            },

            // Local variables
            Operator::LocalGet { local_index } => {
                self.emit_local_get(*local_index, neo_bytecode)?;
            },
            Operator::LocalSet { local_index } => {
                self.emit_local_set(*local_index, neo_bytecode)?;
            },
            Operator::LocalTee { local_index } => {
                neo_bytecode.push(OpCode::Dup.to_byte());
                self.emit_local_set(*local_index, neo_bytecode)?;
            },

            // Global variables
            Operator::GlobalGet { global_index } => {
                self.emit_global_get(*global_index, neo_bytecode)?;
            },
            Operator::GlobalSet { global_index } => {
                self.emit_global_set(*global_index, neo_bytecode)?;
            },

            // Memory operations
            Operator::I32Load { memarg } => {
                self.emit_memory_load(memarg, 4, neo_bytecode)?;
            },
            Operator::I32Store { memarg } => {
                self.emit_memory_store(memarg, 4, neo_bytecode)?;
            },

            // Arithmetic operations
            Operator::I32Add => neo_bytecode.push(OpCode::Add.to_byte()),
            Operator::I32Sub => neo_bytecode.push(OpCode::Sub.to_byte()),
            Operator::I32Mul => neo_bytecode.push(OpCode::Mul.to_byte()),
            Operator::I32DivS | Operator::I32DivU => neo_bytecode.push(OpCode::Div.to_byte()),
            Operator::I32RemS | Operator::I32RemU => neo_bytecode.push(OpCode::Mod.to_byte()),

            // Bitwise operations
            Operator::I32And => neo_bytecode.push(OpCode::And.to_byte()),
            Operator::I32Or => neo_bytecode.push(OpCode::Or.to_byte()),
            Operator::I32Xor => neo_bytecode.push(OpCode::Xor.to_byte()),
            Operator::I32Shl => neo_bytecode.push(OpCode::Shl.to_byte()),
            Operator::I32ShrS | Operator::I32ShrU => neo_bytecode.push(OpCode::Shr.to_byte()),

            // Comparison operations
            Operator::I32Eq => neo_bytecode.push(OpCode::Equal.to_byte()),
            Operator::I32Ne => neo_bytecode.push(OpCode::NotEqual.to_byte()),
            Operator::I32LtS | Operator::I32LtU => neo_bytecode.push(OpCode::Lt.to_byte()),
            Operator::I32GtS | Operator::I32GtU => neo_bytecode.push(OpCode::Gt.to_byte()),
            Operator::I32LeS | Operator::I32LeU => neo_bytecode.push(OpCode::Le.to_byte()),
            Operator::I32GeS | Operator::I32GeU => neo_bytecode.push(OpCode::Ge.to_byte()),

            // Control flow
            Operator::Block { blockty: _ } => {
                self.push_control_label(ControlType::Block)?;
            },
            Operator::Loop { blockty: _ } => {
                self.push_control_label(ControlType::Loop)?;
            },
            Operator::If { blockty: _ } => {
                self.push_control_label(ControlType::If)?;
                // Emit conditional jump - will be resolved by control flow handler
                neo_bytecode.push(OpCode::JmpIfNot.to_byte());
                let jump_pos = neo_bytecode.len();
                neo_bytecode.push(0); // Jump offset - will be patched
                
                // Record jump position for later resolution
                if let Some(label) = self.label_stack.last_mut() {
                    label.jump_positions.push(jump_pos);
                }
            },
            Operator::Else => {
                self.handle_else(neo_bytecode)?;
            },
            Operator::End => {
                self.handle_end(neo_bytecode)?;
            },
            Operator::Br { relative_depth } => {
                self.emit_branch(*relative_depth, neo_bytecode)?;
            },
            Operator::BrIf { relative_depth } => {
                self.emit_conditional_branch(*relative_depth, neo_bytecode)?;
            },
            Operator::Return => {
                neo_bytecode.push(OpCode::Ret.to_byte());
            },

            // Function calls
            Operator::Call { function_index } => {
                self.emit_call(*function_index, neo_bytecode)?;
            },

            // Stack operations
            Operator::Drop => neo_bytecode.push(OpCode::Drop.to_byte()),
            Operator::Select => {
                // Simplified select implementation
                // Stack: [val1, val2, condition] -> [val1 or val2]
                neo_bytecode.extend_from_slice(&[
                    OpCode::JmpIfNot.to_byte(), 3, // If condition is false, jump 3 bytes
                    OpCode::Swap.to_byte(),        // Swap val1 and val2
                    OpCode::Drop.to_byte(),        // Drop val2, keep val1
                ]);
            },

            // Unimplemented operations
            _ => {
                return Err(anyhow::anyhow!("Unimplemented WASM operator: {:?}", op));
            }
        }

        Ok(())
    }

    /// Emit NEO opcodes for pushing i32 constant
    fn emit_push_i32(&self, value: i32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        match value {
            -1 => neo_bytecode.push(OpCode::PushM1.to_byte()),
            0 => neo_bytecode.push(OpCode::Push0.to_byte()),
            1..=16 => {
                let opcode = unsafe { 
                    std::mem::transmute((OpCode::Push1 as u8) + (value as u8 - 1))
                };
                neo_bytecode.push(opcode);
            },
            _ => {
                if value >= i8::MIN as i32 && value <= i8::MAX as i32 {
                    neo_bytecode.push(OpCode::PushInt8.to_byte());
                    neo_bytecode.push(value as u8);
                } else if value >= i16::MIN as i32 && value <= i16::MAX as i32 {
                    neo_bytecode.push(OpCode::PushInt16.to_byte());
                    neo_bytecode.extend_from_slice(&(value as i16).to_le_bytes());
                } else {
                    neo_bytecode.push(OpCode::PushInt32.to_byte());
                    neo_bytecode.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        Ok(())
    }

    /// Emit NEO opcodes for pushing i64 constant
    fn emit_push_i64(&self, value: i64, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            self.emit_push_i32(value as i32, neo_bytecode)
        } else {
            neo_bytecode.push(OpCode::PushInt64.to_byte());
            neo_bytecode.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
    }

    /// Emit local variable get operation
    fn emit_local_get(&self, local_index: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        match local_index {
            0..=6 => {
                let opcode = unsafe { 
                    std::mem::transmute((OpCode::LdLocal0 as u8) + local_index as u8)
                };
                neo_bytecode.push(opcode);
            },
            _ => {
                neo_bytecode.push(OpCode::LdLocal.to_byte());
                neo_bytecode.push(local_index as u8);
            }
        }
        Ok(())
    }

    /// Emit local variable set operation
    fn emit_local_set(&self, local_index: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        match local_index {
            0..=6 => {
                let opcode = unsafe { 
                    std::mem::transmute((OpCode::StLocal0 as u8) + local_index as u8)
                };
                neo_bytecode.push(opcode);
            },
            _ => {
                neo_bytecode.push(OpCode::StLocal.to_byte());
                neo_bytecode.push(local_index as u8);
            }
        }
        Ok(())
    }

    /// Emit global variable operations (mapped to static slots)
    fn emit_global_get(&self, global_index: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        match global_index {
            0..=6 => {
                let opcode = unsafe { 
                    std::mem::transmute((OpCode::LdStatic0 as u8) + global_index as u8)
                };
                neo_bytecode.push(opcode);
            },
            _ => {
                neo_bytecode.push(OpCode::LdStatic.to_byte());
                neo_bytecode.push(global_index as u8);
            }
        }
        Ok(())
    }

    /// Emit global variable set operation
    fn emit_global_set(&self, global_index: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        match global_index {
            0..=6 => {
                let opcode = unsafe { 
                    std::mem::transmute((OpCode::StStatic0 as u8) + global_index as u8)
                };
                neo_bytecode.push(opcode);
            },
            _ => {
                neo_bytecode.push(OpCode::StStatic.to_byte());
                neo_bytecode.push(global_index as u8);
            }
        }
        Ok(())
    }

    /// Emit memory load operation (WASM linear memory -> Neo storage)
    fn emit_memory_load(&mut self, memarg: &wasmparser::MemArg, size: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [address] -> [loaded_value]
        
        // Calculate actual memory address: stack_value + memarg.offset
        let base_address = memarg.offset as u32;
        
        // Add offset to stack address if non-zero
        if base_address > 0 {
            neo_bytecode.push(OpCode::PushInt32.to_byte());
            neo_bytecode.extend_from_slice(&base_address.to_le_bytes());
            neo_bytecode.push(OpCode::Add.to_byte());
        }
        
        // Create memory operation
        let mem_op = MemoryOperation {
            operation_type: MemoryOperationType::Load,
            address: base_address,
            size,
            alignment: memarg.align as u32,
        };
        
        // Use memory model translator
        let memory_bytecode = self.memory_translator.translate_memory_operation(&mem_op)?;
        neo_bytecode.extend_from_slice(&memory_bytecode);
        
        Ok(())
    }

    /// Emit memory store operation (WASM linear memory -> Neo storage)
    fn emit_memory_store(&mut self, memarg: &wasmparser::MemArg, size: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [address, value] -> []
        
        let base_address = memarg.offset as u32;
        
        // Create memory operation
        let mem_op = MemoryOperation {
            operation_type: MemoryOperationType::Store,
            address: base_address,
            size,
            alignment: memarg.align as u32,
        };
        
        // Use memory model translator
        let memory_bytecode = self.memory_translator.translate_memory_operation(&mem_op)?;
        neo_bytecode.extend_from_slice(&memory_bytecode);
        
        Ok(())
    }

    /// Push a control flow label
    fn push_control_label(&mut self, control_type: ControlType) -> Result<()> {
        self.label_stack.push(ControlLabel {
            label_type: control_type,
            start_offset: self.offset,
            end_offset: None,
            continuation_offset: None,
            jump_positions: Vec::new(),
        });
        Ok(())
    }

    /// Handle else branch
    fn handle_else(&mut self, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        if let Some(label) = self.label_stack.last_mut() {
            if matches!(label.label_type, ControlType::If) {
                // Insert unconditional jump to end of if block
                neo_bytecode.push(OpCode::Jmp.to_byte());
                let jump_pos = neo_bytecode.len();
                neo_bytecode.push(0); // Jump offset - will be patched
                
                // Record jump position for resolution
                label.jump_positions.push(jump_pos);
                
                label.continuation_offset = Some(neo_bytecode.len());
                label.label_type = ControlType::Else;
            }
        }
        Ok(())
    }

    /// Handle end of control block
    fn handle_end(&mut self, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        if let Some(label) = self.label_stack.pop() {
            let end_position = neo_bytecode.len();
            
            // Patch all pending jumps for this control block
            for &jump_pos in &label.jump_positions {
                let offset = (end_position as i32) - (jump_pos as i32) - 1;
                if offset >= -128 && offset <= 127 {
                    neo_bytecode[jump_pos] = offset as u8;
                } else {
                    // Production implementation: Handle long jumps with proper opcode conversion
                    // Convert short jump to long jump opcode
                    if jump_pos > 0 {
                        let opcode = neo_bytecode[jump_pos - 1];
                        neo_bytecode[jump_pos - 1] = match opcode {
                            0x22 => 0x25, // JMP -> JMPL
                            0x23 => 0x2A, // JMPIF -> JMPIFL
                            0x24 => 0x2B, // JMPIFNOT -> JMPIFNOTL
                            _ => opcode,   // Keep original if no long version
                        };
                    }
                    // Replace single byte with 4-byte offset
                    let offset_bytes = offset.to_le_bytes();
                    neo_bytecode[jump_pos] = offset_bytes[0];
                    neo_bytecode.insert(jump_pos + 1, offset_bytes[1]);
                    neo_bytecode.insert(jump_pos + 2, offset_bytes[2]);
                    neo_bytecode.insert(jump_pos + 3, offset_bytes[3]);
                }
            }
        }
        Ok(())
    }

    /// Emit branch instruction
    fn emit_branch(&self, relative_depth: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        // Branch to the target control block
        neo_bytecode.push(OpCode::Jmp.to_byte());
        
        // Calculate target from label stack
        if relative_depth as usize >= self.label_stack.len() {
            anyhow::bail!("Invalid branch depth: {}", relative_depth);
        }
        
        let target_idx = self.label_stack.len() - 1 - (relative_depth as usize);
        let target_label = &self.label_stack[target_idx];
        
        // Calculate offset to target
        let current_pos = neo_bytecode.len();
        let target_pos = match target_label.label_type {
            ControlType::Loop => target_label.start_offset,
            _ => target_label.end_offset.unwrap_or(current_pos + 1), // Will be patched
        };
        
        let offset = (target_pos as i32) - (current_pos as i32) - 1;
        if offset >= -128 && offset <= 127 {
            // Short jump fits in i8
            neo_bytecode.push(offset as u8);
        } else {
            // Long jump requires different opcode - use JMPL instead
            // First, replace the short jump opcode with long jump equivalent
            if let Some(last_opcode) = neo_bytecode.last_mut() {
                *last_opcode = match *last_opcode {
                    0x23 => 0x2A, // JMPIF -> JMPIFL  
                    0x24 => 0x2B, // JMPIFNOT -> JMPIFNOTL
                    0x22 => 0x25, // JMP -> JMPL
                    _ => *last_opcode,
                };
            }
            // Emit 4-byte offset for long jump
            let offset_bytes = offset.to_le_bytes();
            neo_bytecode.extend_from_slice(&offset_bytes);
        }
        
        Ok(())
    }

    /// Emit conditional branch instruction
    fn emit_conditional_branch(&self, relative_depth: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        // Conditional branch to the target control block
        neo_bytecode.push(OpCode::JmpIf.to_byte());
        
        // Calculate target from label stack
        if relative_depth as usize >= self.label_stack.len() {
            anyhow::bail!("Invalid conditional branch depth: {}", relative_depth);
        }
        
        let target_idx = self.label_stack.len() - 1 - (relative_depth as usize);
        let target_label = &self.label_stack[target_idx];
        
        // Calculate offset to target
        let current_pos = neo_bytecode.len();
        let target_pos = match target_label.label_type {
            ControlType::Loop => target_label.start_offset,
            _ => target_label.end_offset.unwrap_or(current_pos + 1), // Will be patched
        };
        
        let offset = (target_pos as i32) - (current_pos as i32) - 1;
        if offset >= -128 && offset <= 127 {
            // Short conditional jump fits in i8
            neo_bytecode.push(offset as u8);
        } else {
            // Long conditional jump requires 4-byte offset
            // Replace previous opcode with long version if applicable
            if let Some(last_opcode) = neo_bytecode.last_mut() {
                *last_opcode = match *last_opcode {
                    0x23 => 0x2A, // JMPIF -> JMPIFL
                    0x24 => 0x2B, // JMPIFNOT -> JMPIFNOTL
                    _ => *last_opcode,
                };
            }
            // Emit 4-byte offset for long conditional jump
            let offset_bytes = offset.to_le_bytes();
            neo_bytecode.extend_from_slice(&offset_bytes);
        }
        
        Ok(())
    }

    /// Emit function call
    fn emit_call(&self, function_index: u32, neo_bytecode: &mut Vec<u8>) -> Result<()> {
        // For imported functions, emit syscall
        // For local functions, emit call instruction
        if function_index < 100 { // Assume first 100 are imports
            // This would need proper import resolution
            neo_bytecode.push(OpCode::SysCall.to_byte());
            neo_bytecode.extend_from_slice(&function_index.to_le_bytes());
        } else {
            neo_bytecode.push(OpCode::Call.to_byte());
            neo_bytecode.push(function_index as u8);
        }
        Ok(())
    }
}

impl Default for WasmInstructionParser {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmInstructionParser {
    /// Validate WASM operator in current context
    fn validate_operator(&self, op: &wasmparser::Operator) -> Result<()> {
        use wasmparser::Operator;
        
        match op {
            // Validate stack operations
            Operator::Drop | Operator::Select => {
                // Always valid
                Ok(())
            },
            
            // Validate local variable operations
            Operator::LocalGet { local_index } | 
            Operator::LocalSet { local_index } | 
            Operator::LocalTee { local_index } => {
                if *local_index > 65535 {
                    anyhow::bail!("Local index {} exceeds maximum (65535)", local_index);
                }
                Ok(())
            },
            
            // Validate memory operations
            Operator::I32Load { memarg } | 
            Operator::I64Load { memarg } |
            Operator::I32Store { memarg } |
            Operator::I64Store { memarg } => {
                if memarg.align > 8 {
                    anyhow::bail!("Memory alignment {} exceeds maximum (8)", memarg.align);
                }
                if memarg.offset > 0xFFFFFFFF {
                    anyhow::bail!("Memory offset {} exceeds 32-bit limit", memarg.offset);
                }
                Ok(())
            },
            
            // Validate control flow
            Operator::Br { relative_depth } |
            Operator::BrIf { relative_depth } => {
                if *relative_depth as usize >= self.label_stack.len() {
                    anyhow::bail!("Branch depth {} exceeds label stack depth {}", 
                                relative_depth, self.label_stack.len());
                }
                Ok(())
            },
            
            // Validate function calls
            Operator::Call { function_index } => {
                // Function index validation would require module context
                // For production, validate against actual function table
                if *function_index > 65535 {
                    anyhow::bail!("Function index {} exceeds reasonable limit", function_index);
                }
                Ok(())
            },
            
            // All other operators are considered valid
            _ => Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_mapping() {
        let parser = WasmInstructionParser::new();
        
        // Test arithmetic operations
        assert!(parser.instruction_map.contains_key(&0x6A)); // i32.add
        assert!(parser.instruction_map.contains_key(&0x6B)); // i32.sub
        assert!(parser.instruction_map.contains_key(&0x6C)); // i32.mul
    }

    #[test]
    fn test_push_constants() {
        let parser = WasmInstructionParser::new();
        let mut bytecode = Vec::new();
        
        // Test various constant values
        parser.emit_push_i32(0, &mut bytecode).unwrap();
        assert_eq!(bytecode[0], OpCode::Push0.to_byte());
        
        bytecode.clear();
        parser.emit_push_i32(5, &mut bytecode).unwrap();
        assert_eq!(bytecode[0], OpCode::Push5.to_byte());
        
        bytecode.clear();
        parser.emit_push_i32(1000, &mut bytecode).unwrap();
        assert_eq!(bytecode[0], OpCode::PushInt16.to_byte());
    }

    #[test]
    fn test_local_operations() {
        let parser = WasmInstructionParser::new();
        let mut bytecode = Vec::new();
        
        // Test local get
        parser.emit_local_get(0, &mut bytecode).unwrap();
        assert_eq!(bytecode[0], OpCode::LdLocal0.to_byte());
        
        bytecode.clear();
        parser.emit_local_get(10, &mut bytecode).unwrap();
        assert_eq!(bytecode[0], OpCode::LdLocal.to_byte());
        assert_eq!(bytecode[1], 10);
    }
}