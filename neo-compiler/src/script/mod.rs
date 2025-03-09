//! Neo VM Script
//!
//! This module provides functionality for working with Neo VM scripts.

use crate::neo::OpCode;
use crate::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

/// Represents a Neo VM instruction with its opcode and operand.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    /// The operation code.
    pub opcode: OpCode,
    /// The operand data, if any.
    pub operand: Vec<u8>,
}

impl Instruction {
    /// Creates a new instruction with the given opcode and no operand.
    pub fn new(opcode: OpCode) -> Self {
        Self {
            opcode,
            operand: Vec::new(),
        }
    }

    /// Creates a new instruction with the given opcode and operand.
    pub fn with_operand(opcode: OpCode, operand: Vec<u8>) -> Self {
        Self { opcode, operand }
    }

    /// Encodes the instruction to bytes.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut writer = Vec::new();
        writer.write_all(&[self.opcode as u8])?;
        
        if let Some(size_prefix) = self.opcode.size_prefix() {
            let size = self.operand.len();
            
            match size_prefix {
                1 => writer.write_all(&[size as u8])?,
                2 => writer.write_all(&(size as u16).to_le_bytes())?,
                4 => writer.write_all(&(size as u32).to_le_bytes())?,
                _ => return Err(Error::general(format!("Invalid size prefix: {}", size_prefix))),
            }
        }
        
        writer.write_all(&self.operand)?;
        Ok(writer)
    }

    /// Returns the size of the instruction in bytes.
    pub fn size(&self) -> usize {
        let mut size = 1; // opcode byte
        
        if let Some(size_prefix) = self.opcode.size_prefix() {
            size += size_prefix; // size prefix bytes
        }
        
        size += self.operand.len();
        size
    }
}

/// Represents a Neo VM script.
#[derive(Debug, Clone, Default)]
pub struct Script {
    /// The instructions in the script.
    pub instructions: Vec<Instruction>,
}

impl Script {
    /// Creates a new empty script.
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    /// Creates a script from a sequence of bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            instructions: vec![Instruction::with_operand(OpCode::PUSHDATA1, bytes.to_vec())],
        }
    }

    /// Returns the size of the script in bytes.
    pub fn size(&self) -> usize {
        self.instructions.iter().map(|i| i.size()).sum()
    }
    
    /// Returns the current position in the script (number of instructions).
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
    
    /// Returns true if the script has no instructions.
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
    
    /// Emits a u32 value to the script.
    pub fn emit_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes().to_vec();
        self.emit_with_operand(OpCode::PUSHDATA1, bytes);
    }

    /// Emits an opcode without an operand.
    pub fn emit_opcode(&mut self, opcode: OpCode) {
        self.add_instruction(Instruction::new(opcode));
    }

    /// Emits an opcode with an operand.
    pub fn emit_with_operand(&mut self, opcode: OpCode, operand: Vec<u8>) {
        self.add_instruction(Instruction::with_operand(opcode, operand));
    }

    /// Adds binary data to the script.
    pub fn emit_push_data(&mut self, data: &[u8]) -> Result<(), Error> {
        let opcode = if data.len() <= 0xFF {
            OpCode::PUSHDATA1
        } else if data.len() <= 0xFFFF {
            OpCode::PUSHDATA2
        } else {
            OpCode::PUSHDATA4
        };
        
        self.emit_with_operand(opcode, data.to_vec());
        Ok(())
    }

    /// Emits an integer value to the script.
    pub fn emit_int(&mut self, value: i64) {
        match value {
            -1 => self.emit_opcode(OpCode::PUSHM1),
            0 => self.emit_opcode(OpCode::PUSH0),
            1 => self.emit_opcode(OpCode::PUSH1),
            2 => self.emit_opcode(OpCode::PUSH2),
            3 => self.emit_opcode(OpCode::PUSH3),
            4 => self.emit_opcode(OpCode::PUSH4),
            5 => self.emit_opcode(OpCode::PUSH5),
            6 => self.emit_opcode(OpCode::PUSH6),
            7 => self.emit_opcode(OpCode::PUSH7),
            8 => self.emit_opcode(OpCode::PUSH8),
            9 => self.emit_opcode(OpCode::PUSH9),
            10 => self.emit_opcode(OpCode::PUSH10),
            11 => self.emit_opcode(OpCode::PUSH11),
            12 => self.emit_opcode(OpCode::PUSH12),
            13 => self.emit_opcode(OpCode::PUSH13),
            14 => self.emit_opcode(OpCode::PUSH14),
            15 => self.emit_opcode(OpCode::PUSH15),
            16 => self.emit_opcode(OpCode::PUSH16),
            _ => {
                // For other values, use the appropriate PUSHINT opcode based on size
                if value >= i8::MIN as i64 && value <= i8::MAX as i64 {
                    let bytes = (value as i8).to_le_bytes().to_vec();
                    self.emit_with_operand(OpCode::PUSHINT8, bytes);
                } else if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    let bytes = (value as i16).to_le_bytes().to_vec();
                    self.emit_with_operand(OpCode::PUSHINT16, bytes);
                } else if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                    let bytes = (value as i32).to_le_bytes().to_vec();
                    self.emit_with_operand(OpCode::PUSHINT32, bytes);
                } else {
                    let bytes = value.to_le_bytes().to_vec();
                    self.emit_with_operand(OpCode::PUSHINT64, bytes);
                }
            }
        }
    }

    /// Adds an instruction to the script.
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Returns the script as bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        for instruction in &self.instructions {
            if let Ok(bytes) = instruction.encode() {
                result.extend_from_slice(&bytes);
            }
        }
        
        result
    }

    /// Returns a reference to the script's bytes.
    pub fn bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }
    
    /// Updates the operand of an instruction at a specific offset.
    /// This is particularly useful for jump targets that need to be updated
    /// after the full script is generated.
    pub fn update_operand_at(&mut self, offset: usize, new_operand: &[u8]) -> bool {
        let mut current_offset = 0;
        
        for instruction in &mut self.instructions {
            let instr_size = instruction.size();
            
            // Check if the target offset is within this instruction
            if offset >= current_offset && offset < current_offset + instr_size {
                // Calculate the offset within the instruction
                let instr_offset = offset - current_offset;
                
                // For most instructions, the operand starts after the opcode
                // and possibly after the size prefix
                let operand_start = match instruction.opcode.size_prefix() {
                    Some(size_prefix) => 1 + size_prefix, // opcode + size prefix
                    None => 1, // just opcode
                };
                
                // If the offset points to the operand part
                if instr_offset == operand_start {
                    // Replace the operand
                    instruction.operand = new_operand.to_vec();
                    return true;
                }
            }
            
            current_offset += instr_size;
        }
        
        false // Offset not found
    }

    /// Disassembles the script into a string representation.
    pub fn disassemble(&self) -> String {
        let mut result = String::new();
        for (i, instruction) in self.instructions.iter().enumerate() {
            result.push_str(&format!("{:04X}: {:?}", i, instruction.opcode));
            if !instruction.operand.is_empty() {
                result.push_str(&format!(" {:?}", instruction.operand));
            }
            result.push('\n');
        }
        result
    }
}

/// Save a script to a file
pub fn save_script<P: AsRef<Path>>(script: &Script, path: P) -> Result<(), Error> {
    let bytes = script.to_bytes();
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Load a script from a file
pub fn load_script<P: AsRef<Path>>(path: P) -> Result<Script, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(Script::from_bytes(&bytes))
}

/// Disassemble a script to a string
pub fn disassemble_script(script: &Script) -> String {
    let mut result = String::new();
    
    for (i, instruction) in script.instructions.iter().enumerate() {
        result.push_str(&format!("{:04X}: {:?}", i, instruction.opcode));
        
        if !instruction.operand.is_empty() {
            if instruction.operand.len() <= 8 {
                // For small operands, show the bytes
                let hex = instruction.operand.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                result.push_str(&format!(" {}", hex));
            } else {
                // For larger operands, just show the length
                result.push_str(&format!(" [{}]", instruction.operand.len()));
            }
        }
        
        result.push('\n');
    }
    
    result
}

pub mod converter;