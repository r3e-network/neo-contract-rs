use crate::nef::Nef3;
use crate::opcodes::OpCode;
use std::fmt::Write;

/// Debug utilities for NEO smart contracts
pub struct Debugger {
    script: Vec<u8>,
    breakpoints: Vec<usize>,
    current_position: usize,
}

impl Debugger {
    /// Create a new debugger for a NEF file
    pub fn new(nef: &Nef3) -> Self {
        Self {
            script: nef.script.clone(),
            breakpoints: Vec::new(),
            current_position: 0,
        }
    }

    /// Add a breakpoint at the given position
    pub fn add_breakpoint(&mut self, position: usize) {
        if !self.breakpoints.contains(&position) {
            self.breakpoints.push(position);
            self.breakpoints.sort();
        }
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, position: usize) {
        self.breakpoints.retain(|&bp| bp != position);
    }

    /// Disassemble the entire script
    pub fn disassemble(&self) -> String {
        let mut output = String::new();
        let mut position = 0;

        writeln!(&mut output, "Position | OpCode           | Hex  | Description").unwrap();
        writeln!(&mut output, "---------|------------------|------|-------------").unwrap();

        while position < self.script.len() {
            let opcode_byte = self.script[position];
            let opcode = OpCode::from_byte(opcode_byte);

            let bp_marker = if self.breakpoints.contains(&position) {
                "●"
            } else {
                " "
            };

            match opcode {
                Some(op) => {
                    let (operands, description) = self.decode_operands(op, position + 1);
                    writeln!(
                        &mut output,
                        "{} {:06X} | {:16} | 0x{:02X} | {}",
                        bp_marker,
                        position,
                        format!("{:?}", op),
                        opcode_byte,
                        description
                    ).unwrap();

                    if !operands.is_empty() {
                        for (i, byte) in operands.iter().enumerate() {
                            writeln!(
                                &mut output,
                                "  {:06X} |                  | 0x{:02X} | [operand {}]",
                                position + 1 + i,
                                byte,
                                i
                            ).unwrap();
                        }
                    }

                    position += 1 + operands.len();
                }
                None => {
                    writeln!(
                        &mut output,
                        "{} {:06X} | UNKNOWN          | 0x{:02X} | Unknown opcode",
                        bp_marker,
                        position,
                        opcode_byte
                    ).unwrap();
                    position += 1;
                }
            }
        }

        output
    }

    /// Decode operands for an opcode
    fn decode_operands(&self, opcode: OpCode, start: usize) -> (Vec<u8>, String) {
        let mut operands = Vec::new();
        let mut description = String::new();

        match opcode {
            OpCode::PushInt8 => {
                if start < self.script.len() {
                    let value = self.script[start] as i8;
                    operands.push(self.script[start]);
                    write!(&mut description, "Push int8: {}", value).unwrap();
                }
            }
            OpCode::PushInt16 => {
                if start + 1 < self.script.len() {
                    let bytes = [self.script[start], self.script[start + 1]];
                    let value = i16::from_le_bytes(bytes);
                    operands.extend_from_slice(&bytes);
                    write!(&mut description, "Push int16: {}", value).unwrap();
                }
            }
            OpCode::PushInt32 => {
                if start + 3 < self.script.len() {
                    let bytes = [
                        self.script[start],
                        self.script[start + 1],
                        self.script[start + 2],
                        self.script[start + 3],
                    ];
                    let value = i32::from_le_bytes(bytes);
                    operands.extend_from_slice(&bytes);
                    write!(&mut description, "Push int32: {}", value).unwrap();
                }
            }
            OpCode::PushData1 => {
                if start < self.script.len() {
                    let len = self.script[start] as usize;
                    operands.push(self.script[start]);
                    
                    if start + 1 + len <= self.script.len() {
                        let data = &self.script[start + 1..start + 1 + len];
                        operands.extend_from_slice(data);
                        
                        // Try to interpret as string
                        if let Ok(s) = std::str::from_utf8(data) {
                            write!(&mut description, "Push string: \"{}\"", s).unwrap();
                        } else {
                            write!(&mut description, "Push {} bytes", len).unwrap();
                        }
                    }
                }
            }
            OpCode::Jmp | OpCode::JmpIf | OpCode::JmpIfNot |
            OpCode::JmpEq | OpCode::JmpNe | OpCode::JmpGt |
            OpCode::JmpGe | OpCode::JmpLt | OpCode::JmpLe => {
                if start < self.script.len() {
                    let offset = self.script[start] as i8;
                    operands.push(self.script[start]);
                    let target = (start as i32 + offset as i32) as usize;
                    write!(&mut description, "Jump to 0x{:06X} (offset: {})", target, offset).unwrap();
                }
            }
            OpCode::JmpL | OpCode::JmpIfL | OpCode::JmpIfNotL |
            OpCode::JmpEqL | OpCode::JmpNeL | OpCode::JmpGtL |
            OpCode::JmpGeL | OpCode::JmpLtL | OpCode::JmpLeL => {
                if start + 3 < self.script.len() {
                    let bytes = [
                        self.script[start],
                        self.script[start + 1],
                        self.script[start + 2],
                        self.script[start + 3],
                    ];
                    let offset = i32::from_le_bytes(bytes);
                    operands.extend_from_slice(&bytes);
                    let target = (start as i32 + offset) as usize;
                    write!(&mut description, "Jump to 0x{:06X} (offset: {})", target, offset).unwrap();
                }
            }
            OpCode::Call => {
                if start < self.script.len() {
                    let offset = self.script[start] as i8;
                    operands.push(self.script[start]);
                    let target = (start as i32 + offset as i32) as usize;
                    write!(&mut description, "Call 0x{:06X} (offset: {})", target, offset).unwrap();
                }
            }
            OpCode::CallL => {
                if start + 3 < self.script.len() {
                    let bytes = [
                        self.script[start],
                        self.script[start + 1],
                        self.script[start + 2],
                        self.script[start + 3],
                    ];
                    let offset = i32::from_le_bytes(bytes);
                    operands.extend_from_slice(&bytes);
                    let target = (start as i32 + offset) as usize;
                    write!(&mut description, "Call 0x{:06X} (offset: {})", target, offset).unwrap();
                }
            }
            OpCode::SysCall => {
                if start + 3 < self.script.len() {
                    let bytes = [
                        self.script[start],
                        self.script[start + 1],
                        self.script[start + 2],
                        self.script[start + 3],
                    ];
                    let syscall_id = u32::from_le_bytes(bytes);
                    operands.extend_from_slice(&bytes);
                    
                    let name = self.syscall_name(syscall_id);
                    write!(&mut description, "Syscall: {} (0x{:08X})", name, syscall_id).unwrap();
                }
            }
            OpCode::InitSlot => {
                if start + 1 < self.script.len() {
                    let local_count = self.script[start];
                    let arg_count = self.script[start + 1];
                    operands.push(local_count);
                    operands.push(arg_count);
                    write!(&mut description, "Init {} locals, {} args", local_count, arg_count).unwrap();
                }
            }
            _ => {
                // Basic description for opcodes without operands
                description = self.opcode_description(opcode);
            }
        }

        (operands, description)
    }

    /// Get a description for an opcode
    fn opcode_description(&self, opcode: OpCode) -> String {
        match opcode {
            OpCode::Nop => "No operation".to_string(),
            OpCode::Ret => "Return from function".to_string(),
            OpCode::Drop => "Drop top stack item".to_string(),
            OpCode::Dup => "Duplicate top stack item".to_string(),
            OpCode::Swap => "Swap top two stack items".to_string(),
            OpCode::Add => "Add top two values".to_string(),
            OpCode::Sub => "Subtract top two values".to_string(),
            OpCode::Mul => "Multiply top two values".to_string(),
            OpCode::Div => "Divide top two values".to_string(),
            OpCode::Mod => "Modulo of top two values".to_string(),
            OpCode::Equal => "Check equality".to_string(),
            OpCode::NotEqual => "Check inequality".to_string(),
            OpCode::Lt => "Less than comparison".to_string(),
            OpCode::Le => "Less than or equal comparison".to_string(),
            OpCode::Gt => "Greater than comparison".to_string(),
            OpCode::Ge => "Greater than or equal comparison".to_string(),
            OpCode::Not => "Boolean NOT".to_string(),
            OpCode::BoolAnd => "Boolean AND".to_string(),
            OpCode::BoolOr => "Boolean OR".to_string(),
            OpCode::Assert => "Assert condition".to_string(),
            OpCode::Abort => "Abort execution".to_string(),
            OpCode::Throw => "Throw exception".to_string(),
            _ => format!("{:?}", opcode),
        }
    }

    /// Map syscall ID to name
    fn syscall_name(&self, id: u32) -> &'static str {
        match id {
            0x627D5B52 => "System.Contract.Call",
            0x338312F3 => "System.Contract.CallNative",
            0x40A92E8B => "System.Contract.CreateStandardAccount",
            0xDACE1648 => "System.Contract.CreateMultisigAccount",
            0xB7C7FACC => "System.Contract.GetCallFlags",
            0xFDEA5D4E => "System.Runtime.Platform",
            0xD642A42E => "System.Runtime.GetTrigger",
            0xF157E7EA => "System.Runtime.GetTime",
            0x2D728D86 => "System.Runtime.GetScriptContainer",
            0x632B6E29 => "System.Runtime.GetExecutingScriptHash",
            0x528D0D00 => "System.Runtime.GetCallingScriptHash",
            0x0BEEBEB4 => "System.Runtime.GetEntryScriptHash",
            0x3307B520 => "System.Runtime.CheckWitness",
            0x2F729FF8 => "System.Runtime.GetInvocationCounter",
            0x3A3B9A31 => "System.Runtime.GasLeft",
            0x8166107A => "System.Runtime.GetNotifications",
            0x476DC615 => "System.Runtime.GetNetwork",
            0x8D8B9E42 => "System.Runtime.GetRandom",
            0x8F61E6CE => "System.Runtime.Log",
            0x9BF667CE => "System.Runtime.Notify",
            0x369426FF => "System.Runtime.GetTransaction",
            0xCF561045 => "System.Runtime.BurnGas",
            0x161B7804 => "System.Storage.GetReadOnlyContext",
            0xD4FB8203 => "System.Storage.AsReadOnly",
            0x925DE831 => "System.Storage.Get",
            0xC0695219 => "System.Storage.Find",
            0xE63F1884 => "System.Storage.Put",
            0x6D625B09 => "System.Storage.Delete",
            0x41166107 => "System.Crypto.CheckSig",
            0xD8258E93 => "System.Crypto.CheckMultisig",
            0x0FAAC4E6 => "System.Crypto.SHA256",
            0x7A806A87 => "System.Crypto.RIPEMD160",
            0x95440D7E => "System.Crypto.VerifyWithECDsa",
            0x24C5C88A => "System.Crypto.Murmur32",
            0xD3CE96E7 => "System.Iterator.Create",
            0x932BF322 => "System.Iterator.Next",
            0x61B7C1E5 => "System.Iterator.Value",
            0x7D12289B => "System.Json.Serialize",
            0x8B2E8A15 => "System.Json.Deserialize",
            _ => "Unknown",
        }
    }

    /// Step through one instruction
    pub fn step(&mut self) -> Result<StepResult, String> {
        if self.current_position >= self.script.len() {
            return Ok(StepResult::End);
        }

        let opcode_byte = self.script[self.current_position];
        let opcode = OpCode::from_byte(opcode_byte)
            .ok_or_else(|| format!("Unknown opcode: 0x{:02X}", opcode_byte))?;

        let instruction_size = self.get_instruction_size(opcode, self.current_position + 1);
        let next_position = self.current_position + instruction_size;

        let result = StepResult::Normal {
            position: self.current_position,
            opcode,
            operands: self.script[self.current_position + 1..next_position].to_vec(),
            next_position,
        };

        self.current_position = next_position;

        // Check for breakpoint
        if self.breakpoints.contains(&next_position) {
            Ok(StepResult::Breakpoint(Box::new(result)))
        } else {
            Ok(result)
        }
    }

    /// Get the size of an instruction including operands
    fn get_instruction_size(&self, opcode: OpCode, operand_start: usize) -> usize {
        match opcode.size() {
            1 => 1,
            n if n > 1 => {
                // For variable-length instructions, calculate actual size
                match opcode {
                    OpCode::PushData1 => {
                        if operand_start < self.script.len() {
                            2 + self.script[operand_start] as usize
                        } else {
                            1
                        }
                    }
                    OpCode::PushData2 => {
                        if operand_start + 1 < self.script.len() {
                            let len = u16::from_le_bytes([
                                self.script[operand_start],
                                self.script[operand_start + 1],
                            ]) as usize;
                            3 + len
                        } else {
                            1
                        }
                    }
                    OpCode::PushData4 => {
                        if operand_start + 3 < self.script.len() {
                            let len = u32::from_le_bytes([
                                self.script[operand_start],
                                self.script[operand_start + 1],
                                self.script[operand_start + 2],
                                self.script[operand_start + 3],
                            ]) as usize;
                            5 + len
                        } else {
                            1
                        }
                    }
                    _ => n,
                }
            }
            _ => 1,
        }
    }

    /// Reset the debugger to the beginning
    pub fn reset(&mut self) {
        self.current_position = 0;
    }

    /// Get current position
    pub fn current_position(&self) -> usize {
        self.current_position
    }

    /// Set current position
    pub fn set_position(&mut self, position: usize) {
        self.current_position = position.min(self.script.len());
    }
}

/// Result of stepping through an instruction
#[derive(Debug)]
pub enum StepResult {
    /// Normal step
    Normal {
        position: usize,
        opcode: OpCode,
        operands: Vec<u8>,
        next_position: usize,
    },
    /// Hit a breakpoint
    Breakpoint(Box<StepResult>),
    /// End of script
    End,
}

impl StepResult {
    #[allow(dead_code)]
    fn new(position: usize, opcode: OpCode, operands: Vec<u8>, next_position: usize) -> Self {
        StepResult::Normal {
            position,
            opcode,
            operands,
            next_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::SysCall;

    #[test]
    fn test_debugger_creation() {
        let nef = Nef3::new(
            "test".to_string(),
            "test.rs".to_string(),
            vec![0x21, 0x40], // NOP, RET
        );
        
        let debugger = Debugger::new(&nef);
        assert_eq!(debugger.script, vec![0x21, 0x40]);
        assert_eq!(debugger.current_position, 0);
    }

    #[test]
    fn test_disassemble() {
        let nef = Nef3::new(
            "test".to_string(),
            "test.rs".to_string(),
            vec![
                0x21, // NOP
                0x11, // PUSH1
                0x12, // PUSH2
                0x9B, // ADD
                0x40, // RET
            ],
        );
        
        let debugger = Debugger::new(&nef);
        let disassembly = debugger.disassemble();
        
        assert!(disassembly.contains("Nop"));
        assert!(disassembly.contains("Push1"));
        assert!(disassembly.contains("Push2"));
        assert!(disassembly.contains("Add"));
        assert!(disassembly.contains("Ret"));
    }

    #[test]
    fn test_breakpoints() {
        let nef = Nef3::new(
            "test".to_string(),
            "test.rs".to_string(),
            vec![0x21, 0x21, 0x21, 0x40],
        );
        
        let mut debugger = Debugger::new(&nef);
        
        debugger.add_breakpoint(2);
        assert_eq!(debugger.breakpoints, vec![2]);
        
        debugger.add_breakpoint(1);
        assert_eq!(debugger.breakpoints, vec![1, 2]); // Should be sorted
        
        debugger.remove_breakpoint(1);
        assert_eq!(debugger.breakpoints, vec![2]);
    }

    #[test]
    fn test_stepping() {
        let nef = Nef3::new(
            "test".to_string(),
            "test.rs".to_string(),
            vec![
                0x21, // NOP
                0x11, // PUSH1
                0x40, // RET
            ],
        );
        
        let mut debugger = Debugger::new(&nef);
        
        // Step through NOP
        let result = debugger.step().unwrap();
        if let StepResult::Normal { opcode, next_position, .. } = result {
            assert_eq!(opcode, OpCode::Nop);
            assert_eq!(next_position, 1);
        } else {
            panic!("Expected normal step");
        }
        
        // Step through PUSH1
        let result = debugger.step().unwrap();
        if let StepResult::Normal { opcode, next_position, .. } = result {
            assert_eq!(opcode, OpCode::Push1);
            assert_eq!(next_position, 2);
        } else {
            panic!("Expected normal step");
        }
        
        // Step through RET
        let result = debugger.step().unwrap();
        if let StepResult::Normal { opcode, next_position, .. } = result {
            assert_eq!(opcode, OpCode::Ret);
            assert_eq!(next_position, 3);
        } else {
            panic!("Expected normal step");
        }
        
        // Should be at end
        let result = debugger.step().unwrap();
        assert!(matches!(result, StepResult::End));
    }

    #[test]
    fn test_syscall_disassembly() {
        let mut script = vec![0x41]; // SYSCALL
        script.extend_from_slice(&SysCall::SYSTEM_RUNTIME_NOTIFY.to_le_bytes());
        
        let nef = Nef3::new(
            "test".to_string(),
            "test.rs".to_string(),
            script,
        );
        
        let debugger = Debugger::new(&nef);
        let disassembly = debugger.disassemble();
        
        assert!(disassembly.contains("SysCall"));
        assert!(disassembly.contains("System.Runtime.Notify"));
    }
}