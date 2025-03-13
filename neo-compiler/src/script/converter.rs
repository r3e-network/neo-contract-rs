//! WebAssembly to Neo script converter
//!
//! This module provides functionality for converting WebAssembly modules to Neo VM scripts.

use crate::neo::OpCode;
use crate::script::Script;
use crate::wasm::WasmFunction;
use crate::wasm::WasmModule;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// WebAssembly to Neo script converter
pub struct Converter {
    /// The WebAssembly module to convert
    module: WasmModule,

    /// Function index to offset mapping
    function_offsets: HashMap<u32, u64>,

    /// Exported functions
    _exported_functions: HashMap<String, u32>,

    /// Current locals mapping
    locals: Vec<Local>,

    /// Current function index
    current_function: Option<u32>,

    /// Generated script
    script: Script,
}

/// Local variable information
#[derive(Debug, Clone)]
struct Local {
    /// Local index
    _index: u32,
    /// Local type
    _ty: String,
}

impl Converter {
    /// Create a new WebAssembly to Neo script converter
    pub fn new(module: WasmModule) -> Self {
        let mut exported_functions = HashMap::new();

        // Collect exported functions
        for export in module.exports() {
            if export.kind == "function" {
                exported_functions.insert(export.name.clone(), export.index);
            }
        }

        Self {
            module,
            function_offsets: HashMap::new(),
            _exported_functions: exported_functions,
            locals: Vec::new(),
            current_function: None,
            script: Script::new(),
        }
    }

    /// Convert the WebAssembly module to a Neo script
    pub fn convert(&mut self) -> Result<Script> {
        // Generate a jump table at the beginning of the script
        self.generate_jump_table()?;

        // Process and convert each function
        self.convert_functions()?;

        // Return the completed script
        Ok(self.script.clone())
    }

    /// Generate a jump table for function dispatch
    fn generate_jump_table(&mut self) -> Result<()> {
        // Add a comment indicating the jump table using PUSHDATA
        self.script.emit_push_data("Jump Table".as_bytes())?;

        // Skip the jump table if the input is empty
        self.script.emit_opcode(OpCode::DUP);
        self.script.emit_opcode(OpCode::ISNULL);
        self.script.emit_opcode(OpCode::JMPIF);
        self.script.emit_with_operand(OpCode::PUSHBYTES2, vec![0x10, 0x00]); // Placeholder offset

        // Get the method name (first argument)
        self.script.emit_opcode(OpCode::DUP);
        self.script.emit_opcode(OpCode::ARRAYSIZE);
        self.script.emit_opcode(OpCode::PUSH0);
        self.script.emit_opcode(OpCode::GT);
        self.script.emit_opcode(OpCode::JMPIFNOT);
        self.script.emit_with_operand(OpCode::PUSHBYTES2, vec![0x10, 0x00]); // Placeholder offset

        self.script.emit_opcode(OpCode::PUSH0);
        self.script.emit_opcode(OpCode::PICKITEM);

        // Check each exported function
        let mut function_count = 0;
        for export in self.module.exports() {
            if export.kind == "function" {
                // Skip internal functions (starting with _)
                if export.name.starts_with('_') {
                    continue;
                }

                function_count += 1;

                // Add a comment for this function entry
                self.script.emit_push_data(format!("Jump entry for {}", export.name).as_bytes())?;

                // Compare the method name with this export
                self.script.emit_opcode(OpCode::DUP);
                self.script.emit_with_operand(OpCode::PUSHDATA1, export.name.as_bytes().to_vec());
                self.script.emit_opcode(OpCode::EQUAL);

                // If it matches, jump to the function implementation
                self.script.emit_opcode(OpCode::JMPIF);
                // For now, use a placeholder jump target that will be filled in later
                // Convert usize to u64 for function_offsets
                self.function_offsets.insert(export.index, (self.script.size() + 2) as u64);
                self.script.emit_with_operand(OpCode::PUSHBYTES2, vec![0xFF, 0xFF]);
            }
        }

        if function_count == 0 {
            return Err(anyhow!("No exported functions found in the module"));
        }

        // Add default case - method not found
        self.script.emit_push_data("Method not found".as_bytes())?;
        self.script.emit_with_operand(OpCode::PUSHDATA1, "Method not found".as_bytes().to_vec());
        self.script.emit_opcode(OpCode::THROW);

        Ok(())
    }

    /// Convert all functions in the module
    fn convert_functions(&mut self) -> Result<()> {
        // First, collect all the exports we need to process to avoid borrow conflicts
        let mut exports_to_process = Vec::new();

        for export in self.module.exports() {
            if export.kind == "function" && !export.name.starts_with('_') {
                // For each export, get the function and save it with the export info
                if let Some(function) = self.module.get_function_by_index(export.index as usize) {
                    exports_to_process.push((export.clone(), function.clone()));
                } else {
                    return Err(anyhow!("Function not found at index {}", export.index));
                }
            }
        }

        // Now process each function without conflicting borrows
        for (export, function) in exports_to_process {
            // Add function label
            self.script.emit_push_data(format!("Function: {}", export.name).as_bytes())?;

            // Save the current offset for this function
            let func_offset = self.script.size();
            if let Some(jump_offset) = self.function_offsets.get(&export.index) {
                // Update the placeholder jump target in the jump table
                let jump_bytes = [(func_offset & 0xFF) as u8, ((func_offset >> 8) & 0xFF) as u8];
                // Convert u64 to usize for update_operand_at
                self.script.update_operand_at(*jump_offset as usize, &jump_bytes);
            }

            // Implement the function
            self.convert_function(&function, &export.name)?;
        }

        Ok(())
    }

    /// Convert a single function
    fn convert_function(&mut self, function: &WasmFunction, name: &str) -> Result<()> {
        // Set the current function
        self.current_function = Some(function.index);

        // Setup function parameters and locals
        self.setup_function_locals(function)?;

        // Call the core function logic (simplified for now)
        if name == "main" || name == "Main" {
            // Special handling for entry point
            self.implement_main_function(function)?;
        } else {
            // Standard function implementation
            self.implement_standard_function(function)?;
        }

        // Return from function
        self.script.emit_opcode(OpCode::RET);

        Ok(())
    }

    /// Setup function parameters and local variables
    fn setup_function_locals(&mut self, function: &WasmFunction) -> Result<()> {
        // Clear the locals list
        self.locals.clear();

        // Get the number of parameters for this function
        let param_count = function.params.len();

        // Store parameters as locals
        for (i, param_type) in function.params.iter().enumerate() {
            self.locals.push(Local { _index: i as u32, _ty: param_type.clone() });
        }

        // In Neo N3, we need to extract the parameters from the arguments array
        if param_count > 0 {
            self.script.emit_push_data("Extract function parameters".as_bytes())?;

            // Get arguments array (should be at top of stack)
            for i in 0..param_count {
                // Get parameter at index (i+1) - skip the method name at index 0
                self.script.emit_opcode(OpCode::DUP);
                self.script.emit_with_operand(OpCode::PUSHBYTES1, vec![(i + 1) as u8]);
                self.script.emit_opcode(OpCode::PICKITEM);

                // Convert parameter to appropriate type if needed
                // (For simplicity, we're skipping type conversions for now)
            }

            // Remove the arguments array from the stack
            self.script.emit_opcode(OpCode::DROP);
        }

        Ok(())
    }

    /// Implement the main entry point function
    fn implement_main_function(&mut self, _function: &WasmFunction) -> Result<()> {
        // This is a simplified implementation
        self.script.emit_push_data("Main function implementation".as_bytes())?;

        // For Neo N3, main functions often initialize contract storage or perform other setup
        // For now, just push a success value (true) onto the stack
        self.script.emit_opcode(OpCode::PUSH1);

        Ok(())
    }

    /// Implement a standard (non-main) function
    fn implement_standard_function(&mut self, function: &WasmFunction) -> Result<()> {
        self.script.emit_push_data("Standard function implementation".as_bytes())?;

        // For simplicity, we'll implement a basic function that returns a default value
        match function.returns.first().map(|s| s.as_str()) {
            Some("i32") | Some("i64") => {
                // Return an integer value
                self.script.emit_opcode(OpCode::PUSH0);
            }
            Some("bool") => {
                // Return a boolean value (false)
                self.script.emit_opcode(OpCode::PUSH0);
            }
            Some("string") => {
                // Return an empty string
                self.script.emit_with_operand(OpCode::PUSHDATA1, vec![]);
            }
            Some("void") | None => {
                // Return void/null
                self.script.emit_opcode(OpCode::PUSHNULL);
            }
            Some(_) => {
                // For other types, return null
                self.script.emit_opcode(OpCode::PUSHNULL);
            }
        }

        Ok(())
    }
}
