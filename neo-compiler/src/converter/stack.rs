//! WebAssembly stack analyzer.
//!
//! This module provides functionality for analyzing the stack size requirements
//! of WebAssembly functions.

use crate::error::Error;
use crate::wasm::WasmFunction;
use std::cmp::max;
use wasmparser::Operator;
use wasmparser::FunctionBody;

/// Emulator for WebAssembly stack operations.
pub struct StackEmulator {
    /// Stack items
    items: Vec<String>,
}

impl Default for StackEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl StackEmulator {
    /// Creates a new stack emulator.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
    
    /// Pushes a value onto the stack.
    pub fn push(&mut self, value: String) {
        self.items.push(value);
    }
    
    /// Pops a value from the stack.
    pub fn pop(&mut self) -> Option<String> {
        self.items.pop()
    }
    
    /// Returns the current stack depth.
    pub fn depth(&self) -> usize {
        self.items.len()
    }
    
    /// Peeks at the top value on the stack without removing it.
    pub fn peek(&self) -> Option<&String> {
        self.items.last()
    }
}

/// Analyzer for WebAssembly function stack requirements.
pub struct StackAnalyzer {
    /// Maximum stack size observed during analysis.
    max_stack_size: usize,
    /// Current stack size.
    current_stack_size: usize,
}

impl Default for StackAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl StackAnalyzer {
    /// Creates a new stack analyzer.
    pub fn new() -> Self {
        Self {
            max_stack_size: 0,
            current_stack_size: 0,
        }
    }

    /// Analyzes a WebAssembly function to determine its maximum stack size.
    pub fn analyze_function(&self, function: &WasmFunction) -> Result<usize, Error> {
        if function.body.is_empty() {
            return Ok(0); // Imported functions have no body
        }
        
        let mut analyzer = Self::new();
        
        // Parse function body - create a reader for the function body
        let body = FunctionBody::new(0, function.body.as_slice());

        
        // Get local count
        let mut reader = body.get_binary_reader();
        let locals_count = reader.read_var_u32()
            .map_err(|e| Error::invalid_wasm(format!("Failed to read locals count: {}", e)))?;
        
        // Skip locals
        for _ in 0..locals_count {
            let _ = reader.read_var_u32()
                .map_err(|e| Error::invalid_wasm(format!("Failed to read local count: {}", e)))?;
            let _ = reader.read_u8()
                .map_err(|e| Error::invalid_wasm(format!("Failed to read local type: {}", e)))?;
        }
        
        // Analyze operators
        // Read operators one by one until the end
        let mut operators = Vec::new();
        while !reader.eof() {
            let operator = reader.read_operator()
                .map_err(|e| Error::invalid_wasm(format!("Failed to read operator: {}", e)))?;
            operators.push(operator);
        }
        
        for op in operators.into_iter() {
            
            analyzer.analyze_operator(op)?;
        }
        
        Ok(analyzer.max_stack_size)
    }

    /// Analyzes a WebAssembly operator to update stack size tracking.
    fn analyze_operator(&mut self, op: Operator) -> Result<(), Error> {
        // Update the current stack size based on the operator
        match op {
            // Stack-neutral operations (pop and push same amount)
            Operator::I32Add | Operator::I64Add |
            Operator::I32Sub | Operator::I64Sub |
            Operator::I32Mul | Operator::I64Mul |
            Operator::I32DivS | Operator::I64DivS |
            Operator::I32RemS | Operator::I64RemS |
            Operator::I32And | Operator::I64And |
            Operator::I32Or | Operator::I64Or |
            Operator::I32Xor | Operator::I64Xor |
            Operator::I32Shl | Operator::I64Shl |
            Operator::I32ShrS | Operator::I64ShrS |
            Operator::I32Eq | Operator::I64Eq |
            Operator::I32Ne | Operator::I64Ne |
            Operator::I32LtS | Operator::I64LtS |
            Operator::I32LeS | Operator::I64LeS |
            Operator::I32GtS | Operator::I64GtS |
            Operator::I32GeS | Operator::I64GeS => {
                // Pop 2, push 1
                self.current_stack_size -= 1;
            }
            
            // Operations that push a value
            Operator::I32Const { .. } | 
            Operator::I64Const { .. } |
            Operator::LocalGet { .. } |
            Operator::GlobalGet { .. } => {
                self.current_stack_size += 1;
                self.max_stack_size = max(self.max_stack_size, self.current_stack_size);
            }
            
            // Operations that pop a value
            Operator::Drop |
            Operator::LocalSet { .. } |
            Operator::GlobalSet { .. } => {
                self.current_stack_size -= 1;
            }
            
            // Operations that keep stack size the same
            Operator::LocalTee { .. } => {
                // No change (pushes and pops same number)
            }
            
            // Select pops 3, pushes 1
            Operator::Select => {
                self.current_stack_size -= 2;
            }
            
            // Memory operations generally pop address, maybe value, push maybe result
            Operator::I32Load { .. } |
            Operator::I64Load { .. } => {
                // Pop address, push result (no net change)
            }
            
            Operator::I32Store { .. } |
            Operator::I64Store { .. } => {
                // Pop address and value
                self.current_stack_size -= 2;
            }
            
            // Control flow
            Operator::Block { .. } |
            Operator::Loop { .. } |
            Operator::If { .. } |
            Operator::Else |
            Operator::End => {
                // These don't directly affect stack size in our simplified model
            }
            
            Operator::Br { .. } |
            Operator::BrIf { .. } |
            Operator::BrTable { .. } => {
                // These have complex effects on stack size
                // For simplicity, we won't model them precisely
            }
            
            Operator::Return => {
                // Reset stack size on return
                self.current_stack_size = 0;
            }
            
            Operator::Call { .. } => {
                // Call would need to know the function signature
                // For simplicity, we'll assume it uses at most 5 stack slots
                self.current_stack_size += 5;
                self.max_stack_size = max(self.max_stack_size, self.current_stack_size);
                self.current_stack_size -= 5;
            }
            
            // Conversions typically don't change stack size
            Operator::I32WrapI64 |
            Operator::I64ExtendI32S => {
                // No change
            }
            
            // For any other operation, we'll conservatively add to stack size
            _ => {
                self.current_stack_size += 1;
                self.max_stack_size = max(self.max_stack_size, self.current_stack_size);
            }
        }
        
        // Reset stack size if it reaches zero
        if self.current_stack_size == 0 {
            // Stack size is already zero, nothing to do
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasmparser::{FuncType, ValType};

    #[test]
    fn test_stack_analysis_simple() {
        // Create a simple function: i32.const 1, i32.const 2, i32.add
        let wasm_code = vec![
            0x00, 0x00, // Empty locals
            0x41, 0x01, // i32.const 1
            0x41, 0x02, // i32.const 2
            0x6A,       // i32.add
            0x0B,       // end
        ];
        
        let func_type = FuncType::new(vec![], vec![ValType::I32]);
        // Create a WasmFunction with its fields directly instead of using new()
        let func = crate::wasm::WasmFunction {
            name: None,
            index: 0,
            signature: "() -> (I32)".to_string(), // Simple signature for test
            params: vec![],
            returns: vec!["I32".to_string()],
            body: wasm_code,
        };
        
        let analyzer = StackAnalyzer::new();
        let max_stack = analyzer.analyze_function(&func).unwrap();
        
        // Max should be 2 (during the second i32.const)
        assert_eq!(max_stack, 2);
    }
}