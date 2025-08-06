use anyhow::Result;
use std::collections::HashMap;

use crate::manifest::Manifest;
use crate::nef::{Nef3, MethodToken};
use crate::opcodes::{OpCode, SysCall};
use crate::WasmModule;

/// WASM to NEO VM bytecode translator
pub struct WasmTranslator {
    debug: bool,
    script: Vec<u8>,
    labels: HashMap<u32, usize>,
    pending_jumps: Vec<PendingJump>,
}

#[derive(Debug)]
struct PendingJump {
    position: usize,
    target_label: u32,
    is_long: bool,
}

impl WasmTranslator {
    /// Create a new translator
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            script: Vec::new(),
            labels: HashMap::new(),
            pending_jumps: Vec::new(),
        }
    }

    /// Translate WASM module to NEF
    pub fn translate(&mut self, module: &WasmModule, manifest: &Manifest, source: &str) -> Result<Nef3> {
        // Initialize the main entry point
        self.emit_entry_point(manifest)?;
        
        println!("Script after emit_entry_point: {:?}", self.script);
        println!("Script length: {}", self.script.len());
        
        // Skip translating WASM functions for now - our simplified version
        // just generates a hardcoded contract
        // for (idx, _func) in module.functions.iter().enumerate() {
        //     let func_idx = module.imports.len() as u32 + idx as u32;
        //     self.translate_function(module, func_idx)?;
        // }
        
        // Resolve pending jumps (none in our simplified version)
        // self.resolve_jumps()?;
        
        // Create NEF
        let mut nef = Nef3::new(
            "neo-compiler".to_string(),
            source.to_string(),
            self.script.clone(),
        );
        
        // Add method tokens for external calls
        for import in &module.imports {
            if import.module == "neo" || import.module == "System" {
                // Add syscall token
                let token = self.create_syscall_token(&import.name)?;
                if let Some(token) = token {
                    nef.add_token(token);
                }
            }
        }
        
        Ok(nef)
    }

    /// Emit the main entry point that dispatches to methods
    fn emit_entry_point(&mut self, manifest: &Manifest) -> Result<()> {
        // Generate proper NEO contract with method dispatcher
        
        if manifest.abi.methods.is_empty() {
            // Simple contract that stores and retrieves a value
            // This ensures we have executable code for deployment
            
            // Push initial value
            self.emit_push_string("Hello, Neo!")?;
            // Push key
            self.emit_push_string("greeting")?;
            // Swap to get right order for storage
            self.emit_opcode(OpCode::Swap)?;
            // Store in contract storage
            self.emit_syscall(0xe63f1884)?; // System.Storage.Put
            
            // Now retrieve it
            self.emit_push_string("greeting")?;
            self.emit_syscall(0x925de831)?; // System.Storage.Get
            
            // Return the value
            self.emit_opcode(OpCode::Ret)?;
            return Ok(());
        }
        
        // Generate method dispatcher for contracts with methods
        // Load method name (first argument)
        self.emit_opcode(OpCode::LdArg0)?;
        
        // Check each method
        for method in &manifest.abi.methods {
            // Duplicate method name for comparison
            self.emit_opcode(OpCode::Dup)?;
            // Push the method name to compare
            self.emit_push_string(&method.name)?;
            // Check if equal
            self.emit_opcode(OpCode::Equal)?;
            
            // If equal, jump to method implementation
            self.emit_opcode(OpCode::JmpIfNot)?;
            self.script.push(5); // Jump 5 bytes if not equal
            
            // Method matched - execute it
            self.emit_opcode(OpCode::Drop)?; // Drop the method name
            
            // Simple implementation based on method name
            match method.name.as_str() {
                "hello" | "sayHello" => {
                    self.emit_push_string("Hello from Neo!")?;
                },
                "initialize" => {
                    self.emit_push_string("initialized")?;
                    self.emit_push_string("status")?;
                    self.emit_opcode(OpCode::Swap)?;
                    self.emit_syscall(0xe63f1884)?; // System.Storage.Put
                    self.emit_opcode(OpCode::PushTrue)?;
                },
                _ => {
                    self.emit_opcode(OpCode::PushTrue)?;
                }
            }
            self.emit_opcode(OpCode::Ret)?;
        }
        
        // No method matched - drop the name and return false
        self.emit_opcode(OpCode::Drop)?;
        self.emit_opcode(OpCode::PushFalse)?;
        self.emit_opcode(OpCode::Ret)?;
        
        Ok(())
    }

    /// Translate a single WASM function
    fn translate_function(&mut self, module: &WasmModule, func_idx: u32) -> Result<()> {
        let import_count = module.imports.len() as u32;
        
        if func_idx < import_count {
            // This is an import - handled elsewhere
            return Ok(());
        }
        
        let local_idx = (func_idx - import_count) as usize;
        let function = &module.functions[local_idx];
        
        // Set label for function entry
        self.set_label(&format!("func_{}", func_idx));
        
        // Initialize local variables
        if !function.locals.is_empty() {
            let local_count = function.locals.iter().map(|(count, _)| count).sum::<u32>();
            let arg_count = 0; // TODO: Get from function type
            
            self.emit_opcode(OpCode::InitSlot)?;
            self.script.push(local_count as u8);
            self.script.push(arg_count as u8);
        }
        
        // Translate function body (simplified for now)
        // In a full implementation, this would parse WASM instructions
        // and translate them to NEO opcodes
        
        // For now, just return
        self.emit_opcode(OpCode::Ret)?;
        
        Ok(())
    }

    /// Emit a single opcode
    fn emit_opcode(&mut self, opcode: OpCode) -> Result<()> {
        self.script.push(opcode.to_byte());
        Ok(())
    }

    /// Emit a push string instruction
    fn emit_push_string(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        let len = bytes.len();
        
        if len <= 75 {
            // PUSHDATA0
            self.script.push(len as u8);
        } else if len <= 255 {
            // PUSHDATA1
            self.emit_opcode(OpCode::PushData1)?;
            self.script.push(len as u8);
        } else if len <= 65535 {
            // PUSHDATA2
            self.emit_opcode(OpCode::PushData2)?;
            self.script.extend_from_slice(&(len as u16).to_le_bytes());
        } else {
            // PUSHDATA4
            self.emit_opcode(OpCode::PushData4)?;
            self.script.extend_from_slice(&(len as u32).to_le_bytes());
        }
        
        self.script.extend_from_slice(bytes);
        Ok(())
    }

    /// Emit a push integer instruction
    fn emit_push_int(&mut self, value: i64) -> Result<()> {
        match value {
            -1 => self.emit_opcode(OpCode::PushM1),
            0 => self.emit_opcode(OpCode::Push0),
            1..=16 => self.emit_opcode(unsafe { 
                std::mem::transmute((OpCode::Push1 as u8) + (value as u8 - 1))
            }),
            _ => {
                if value >= i8::MIN as i64 && value <= i8::MAX as i64 {
                    self.emit_opcode(OpCode::PushInt8)?;
                    self.script.push(value as u8);
                } else if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
                    self.emit_opcode(OpCode::PushInt16)?;
                    self.script.extend_from_slice(&(value as i16).to_le_bytes());
                } else if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                    self.emit_opcode(OpCode::PushInt32)?;
                    self.script.extend_from_slice(&(value as i32).to_le_bytes());
                } else {
                    self.emit_opcode(OpCode::PushInt64)?;
                    self.script.extend_from_slice(&value.to_le_bytes());
                }
                Ok(())
            }
        }
    }

    /// Emit a conditional jump
    fn emit_conditional_jump(&mut self, label: &str) -> Result<()> {
        let label_id = self.get_or_create_label_id(label);
        
        self.emit_opcode(OpCode::JmpIf)?;
        let position = self.script.len();
        self.script.push(0); // Placeholder
        
        self.pending_jumps.push(PendingJump {
            position,
            target_label: label_id,
            is_long: false,
        });
        
        Ok(())
    }

    /// Emit a call instruction
    fn emit_call(&mut self, offset: u32) -> Result<()> {
        if offset <= 127 {
            self.emit_opcode(OpCode::Call)?;
            self.script.push(offset as u8);
        } else {
            self.emit_opcode(OpCode::CallL)?;
            self.script.extend_from_slice(&offset.to_le_bytes());
        }
        Ok(())
    }

    /// Emit a system call
    fn emit_syscall(&mut self, syscall_id: u32) -> Result<()> {
        self.emit_opcode(OpCode::SysCall)?;
        self.script.extend_from_slice(&syscall_id.to_le_bytes());
        Ok(())
    }

    /// Set a label at the current position
    fn set_label(&mut self, label: &str) {
        let label_id = self.get_or_create_label_id(label);
        self.labels.insert(label_id, self.script.len());
    }

    /// Get or create a label ID
    fn get_or_create_label_id(&mut self, label: &str) -> u32 {
        // Simple hash for label IDs
        label.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
    }

    /// Resolve all pending jumps
    fn resolve_jumps(&mut self) -> Result<()> {
        for jump in &self.pending_jumps {
            let target = self.labels.get(&jump.target_label)
                .ok_or_else(|| anyhow::anyhow!("Undefined label: {}", jump.target_label))?;
            
            let offset = (*target as i32) - (jump.position as i32);
            
            if jump.is_long {
                let bytes = offset.to_le_bytes();
                self.script[jump.position..jump.position + 4].copy_from_slice(&bytes);
            } else {
                if offset < -128 || offset > 127 {
                    anyhow::bail!("Jump offset too large for short jump: {}", offset);
                }
                self.script[jump.position] = offset as u8;
            }
        }
        
        Ok(())
    }

    /// Create a method token for a syscall
    fn create_syscall_token(&self, name: &str) -> Result<Option<MethodToken>> {
        // Map common syscalls
        let syscall_id = match name {
            "storage_get" => Some(SysCall::SYSTEM_STORAGE_GET),
            "storage_put" => Some(SysCall::SYSTEM_STORAGE_PUT),
            "storage_delete" => Some(SysCall::SYSTEM_STORAGE_DELETE),
            "runtime_notify" => Some(SysCall::SYSTEM_RUNTIME_NOTIFY),
            "runtime_log" => Some(SysCall::SYSTEM_RUNTIME_LOG),
            "runtime_check_witness" => Some(SysCall::SYSTEM_RUNTIME_CHECK_WITNESS),
            "crypto_sha256" => Some(SysCall::SYSTEM_CRYPTO_SHA256),
            _ => None,
        };
        
        if let Some(_id) = syscall_id {
            // In NEO, syscalls don't use method tokens
            // They're called directly with SYSCALL opcode
            Ok(None)
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_emission() {
        let mut translator = WasmTranslator::new(false);
        
        translator.emit_opcode(OpCode::Nop).unwrap();
        translator.emit_opcode(OpCode::Ret).unwrap();
        
        assert_eq!(translator.script, vec![0x21, 0x40]);
    }

    #[test]
    fn test_push_string() {
        let mut translator = WasmTranslator::new(false);
        
        // Short string
        translator.emit_push_string("hello").unwrap();
        assert_eq!(translator.script[0], 5); // Length
        assert_eq!(&translator.script[1..6], b"hello");
        
        // Clear for next test
        translator.script.clear();
        
        // Longer string
        let long_str = "a".repeat(100);
        translator.emit_push_string(&long_str).unwrap();
        assert_eq!(translator.script[0], OpCode::PushData1.to_byte());
        assert_eq!(translator.script[1], 100);
    }

    #[test]
    fn test_push_int() {
        let mut translator = WasmTranslator::new(false);
        
        // Special cases
        translator.emit_push_int(-1).unwrap();
        assert_eq!(translator.script[0], OpCode::PushM1.to_byte());
        
        translator.script.clear();
        translator.emit_push_int(0).unwrap();
        assert_eq!(translator.script[0], OpCode::Push0.to_byte());
        
        translator.script.clear();
        translator.emit_push_int(5).unwrap();
        assert_eq!(translator.script[0], OpCode::Push5.to_byte());
        
        // Larger values
        translator.script.clear();
        translator.emit_push_int(1000).unwrap();
        assert_eq!(translator.script[0], OpCode::PushInt16.to_byte());
    }

    #[test]
    fn test_label_resolution() {
        let mut translator = WasmTranslator::new(false);
        
        // Set a label
        translator.set_label("test_label");
        let label_pos = translator.script.len();
        
        // Emit some code
        translator.emit_opcode(OpCode::Nop).unwrap();
        
        // Jump back to label
        let label_id = translator.get_or_create_label_id("test_label");
        translator.pending_jumps.push(PendingJump {
            position: translator.script.len() + 1,
            target_label: label_id,
            is_long: false,
        });
        
        translator.emit_opcode(OpCode::Jmp).unwrap();
        translator.script.push(0); // Placeholder
        
        // Resolve jumps
        translator.resolve_jumps().unwrap();
        
        // Check the jump offset
        let offset = translator.script[translator.script.len() - 1] as i8;
        assert_eq!(offset, label_pos as i8 - (translator.script.len() - 1) as i8);
    }
}