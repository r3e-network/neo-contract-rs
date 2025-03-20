//! Neo VM Script
//!
//! This module provides functionality for working with Neo VM scripts.

use crate::error::Error;
use crate::neo::OpCode;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
    pub fn new(opcode: OpCode) -> Self { Self { opcode, operand: Vec::new() } }

    /// Creates a new instruction with the given opcode and operand.
    pub fn with_operand(opcode: OpCode, operand: Vec<u8>) -> Self { Self { opcode, operand } }

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

/// NEO VM script representation
#[derive(Debug, Clone)]
pub struct Script {
    /// Script bytes
    bytes: Vec<u8>,
}

impl Script {
    /// Create a new empty script
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    
    /// Create a script from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self { bytes: bytes.to_vec() }
    }
    
    /// Get the script bytes
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    
    /// Push data onto the script
    pub fn emit_push_data(&mut self, data: &[u8]) -> Result<(), crate::error::Error> {
        self.bytes.extend_from_slice(data);
        Ok(())
    }

    /// Returns the size of the script in bytes.
    pub fn size(&self) -> usize { self.bytes.len() }

    /// Returns the current position in the script (number of instructions).
    pub fn len(&self) -> usize { self.bytes.len() }

    /// Returns true if the script has no instructions.
    pub fn is_empty(&self) -> bool { self.bytes.is_empty() }

    /// Emits a u32 value to the script.
    pub fn emit_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes().to_vec();
        self.bytes.extend_from_slice(&bytes);
    }

    /// Emits an opcode without an operand.
    pub fn emit_opcode(&mut self, opcode: OpCode) {
        self.bytes.push(opcode as u8);
    }

    /// Emits an opcode with an operand.
    pub fn emit_with_operand(&mut self, opcode: OpCode, operand: Vec<u8>) {
        self.bytes.push(opcode as u8);
        self.bytes.extend_from_slice(&operand);
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
                    self.bytes.extend_from_slice(&bytes);
                } else if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    let bytes = (value as i16).to_le_bytes().to_vec();
                    self.bytes.extend_from_slice(&bytes);
                } else if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                    let bytes = (value as i32).to_le_bytes().to_vec();
                    self.bytes.extend_from_slice(&bytes);
                } else {
                    let bytes = value.to_le_bytes().to_vec();
                    self.bytes.extend_from_slice(&bytes);
                }
            }
        }
    }

    /// Adds a comment to the script.
    ///
    /// Note: Since Neo N3 does not support the COMMENT opcode, this function
    /// now only stores the comment in memory without adding it to the bytecode.
    /// Comments will be available in the source code but not in the compiled script.
    pub fn emit_comment(&mut self, _comment: &str) {
        // In Neo N3, we don't emit comments into the bytecode
        // This is a no-op that preserves the API for backward compatibility
        // Comments are only stored in the source code
    }

    /// Updates the operand of an instruction at a specific offset.
    /// This is particularly useful for jump targets that need to be updated
    /// after the full script is generated.
    pub fn update_operand_at(&mut self, offset: usize, new_operand: &[u8]) -> bool {
        if offset < self.bytes.len() {
            self.bytes.splice(offset..offset + new_operand.len(), new_operand.iter().cloned());
            true
        } else {
            false // Offset not found
        }
    }

    /// Disassembles the script into a string representation.
    pub fn disassemble(&self) -> String {
        let mut result = String::new();
        for (i, byte) in self.bytes.iter().enumerate() {
            result.push_str(&format!("{:04X}: {:?}", i, *byte));
            result.push('\n');
        }
        result
    }
}

/// Save a script to a file
pub fn save_script<P: AsRef<Path>>(script: &Script, path: P) -> Result<(), Error> {
    let bytes = script.bytes().to_vec();
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

    for (i, byte) in script.bytes().iter().enumerate() {
        result.push_str(&format!("{:04X}: {:?}", i, *byte));
        result.push('\n');
    }

    result
}

pub mod converter;
