//! Main Neo compiler implementation.
//!
//! This module provides the main compiler implementation for converting
//! WebAssembly to Neo VM bytecode.

use crate::converter::WasmConverter;
use crate::error::Error;
use crate::manifest::{Manifest, ContractMethodDefinition, ContractParameterDefinition, ContractEventDefinition, ContractAbi};
use crate::nef::NefFile;
use crate::script::Script;
use crate::wasm::WasmModule;
use std::fs;
use std::path::{Path, PathBuf};

/// Struct for overriding manifest settings
#[derive(Debug, Clone)]
pub struct ManifestOverride {
    /// The key to override in the manifest
    pub key: String,
    /// The value to use for the override
    pub value: String,
}

/// Options for configuring the compiler.
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Whether to include debug information in the output.
    pub debug: bool,
    /// Path to a manifest template file.
    pub manifest_template: Option<PathBuf>,
    /// Optional contract name override.
    pub contract_name: Option<String>,
    /// Whether to optimize the generated code.
    pub optimize: bool,
    /// Output directory for compiled files.
    pub output_dir: Option<PathBuf>,
    /// Manifest override options.
    pub manifest_overrides: Option<Vec<ManifestOverride>>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            debug: false,
            manifest_template: None,
            contract_name: None,
            optimize: true,
            output_dir: None,
            manifest_overrides: None,
        }
    }
}

/// Neo compiler for converting WebAssembly to Neo VM bytecode.
pub struct Compiler {
    /// Options for the compiler.
    options: CompilerOptions,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    /// Creates a new compiler with default options.
    pub fn new() -> Self {
        Self {
            options: CompilerOptions::default(),
        }
    }

    /// Creates a new compiler with the specified options.
    pub fn with_options(options: CompilerOptions) -> Self {
        Self { options }
    }
    
    /// Get a reference to the compiler options.
    pub fn get_options(&self) -> &CompilerOptions {
        &self.options
    }

    /// Sets the debug mode.
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.options.debug = debug;
        self
    }

    /// Sets the optimization level.
    pub fn with_optimize(mut self, optimize: bool) -> Self {
        self.options.optimize = optimize;
        self
    }

    /// Sets the manifest template path.
    pub fn with_manifest_template<P: AsRef<Path>>(mut self, template_path: P) -> Self {
        self.options.manifest_template = Some(template_path.as_ref().to_path_buf());
        self
    }

    /// Sets the contract name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.options.contract_name = Some(name.into());
        self
    }

    /// Compiles a WebAssembly file to a Neo contract.
    ///
    /// # Arguments
    ///
    /// * `wasm_path` - Path to the WebAssembly file
    /// * `output_dir` - Output directory for the compiled files
    /// * `name` - Name of the contract
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Result of the compilation
    pub fn compile<P: AsRef<Path>>(
        &self,
        wasm_path: P,
        output_dir: P,
        name: &str,
    ) -> Result<(), Error> {
        // Load and parse the WebAssembly module
        let wasm_data = fs::read(&wasm_path).map_err(Error::Io)?;
        let module = WasmModule::parse(&wasm_data)?;
        
        if self.options.debug {
            println!("Loaded WebAssembly module: {} functions, {} exports",
                module.functions().len(),
                module.exports().len());
        }
        
        // Convert WebAssembly to Neo script
        let converter = WasmConverter::new();
        let script_bytes = converter.convert_to_script(&module, self.options.optimize)?;
        let script = Script::from_bytes(&script_bytes);
        
        if self.options.debug {
            println!("Generated Neo script: {} bytes", script.bytes().len());
        }
        
        // Create the NEF file
        let mut nef = NefFile::with_script(script.bytes().to_vec());
        nef.finalize()?;
        
        // Generate the manifest
        let manifest = self.generate_manifest(name, &module)?;
        
        // Create the output directory if it doesn't exist
        fs::create_dir_all(&output_dir).map_err(Error::Io)?;
        
        // Save the NEF file and manifest
        let base_name = name.to_string();
        let nef_path = Path::new(&output_dir.as_ref()).join(format!("{}.nef", base_name));
        let manifest_path = Path::new(&output_dir.as_ref()).join(format!("{}.manifest.json", base_name));
        
        nef.save_to(&nef_path)?;
        manifest.save_to_file(&manifest_path)?;
        
        if self.options.debug {
            println!("Saved NEF file: {}", nef_path.display());
            println!("Saved manifest: {}", manifest_path.display());
        }
        
        Ok(())
    }

    /// Generates a manifest for the contract.
    pub fn generate_manifest(&self, name: &str, module: &WasmModule) -> Result<Manifest, Error> {
        // Create a basic manifest
        let manifest = if let Some(template_path) = &self.options.manifest_template {
            // Load the template and update it with the contract information
            let abi = self.generate_abi(module)?;
            Manifest::from_template(template_path, name, abi)?
        } else {
            // Create a new manifest from scratch
            let mut manifest = Manifest::new(name);
            
            // Add the ABI methods and events
            let abi = self.generate_abi(module)?;
            manifest = manifest.with_abi(abi);
            
            // Set features
            let uses_storage = self.detect_storage_usage(module)?;
            manifest.set_feature("storage", uses_storage)?;
            
            manifest
        };
        
        manifest.validate()?;
        Ok(manifest)
    }

    /// Generates the ABI for the contract from the WebAssembly module.
    fn generate_abi(&self, module: &WasmModule) -> Result<ContractAbi, Error> {
        let mut methods = Vec::new();
        let mut events = Vec::new();
        
        // Find exported functions that should be exposed as methods
        for export in module.exports() {
            if export.kind != "function" {
                continue;
            }
            
            // Skip internal functions (those starting with underscore)
            if export.name.starts_with('_') || export.name == "deploy" {
                continue;
            }
            
            // Get the function signature
            let function = module.get_function_by_index(export.index as usize)
                .ok_or_else(|| Error::general(format!("Function index {} not found", export.index)))?;
            
            // Build parameter list
            let mut parameters = Vec::new();
            for (i, param_type) in function.params.iter().enumerate() {
                let param_name = format!("p{}", i);
                let neo_type = self.wasm_type_to_neo_type(param_type)?;
                parameters.push(ContractParameterDefinition::new(param_name, neo_type));
            }
            
            // Determine return type
            let return_type = if function.returns.is_empty() {
                "Void".to_string()
            } else if function.returns.len() == 1 {
                self.wasm_type_to_neo_type(&function.returns[0])?
            } else {
                return Err(Error::general("Multiple return values not supported"));
            };
            
            // Create the method definition
            let method = ContractMethodDefinition::new(
                export.name.clone(),
                parameters,
                return_type,
                self.is_safe_method(&export.name),
            );
            
            methods.push(method);
        }
        
        // Detect events by analyzing imports for Runtime.notify calls
        // In Neo N3, events are properly emitted using the Runtime::notify method
        // The pattern is:
        // 1. Use ByteString::from() for event name
        // 2. Create an Array<Any> for parameters
        // 3. Convert parameters with Any::from()
        // 4. Use Runtime::notify(event_name, event_params)
        for import in module.imports() {
            // Look for Runtime module imports that represent event notifications
            if (import.module == "runtime" || import.module == "neo_runtime") && import.field == "notify" {
                // Found Runtime.notify import - this is used for event emission in Neo N3
                // Look for functions that create ByteString event names and call notify
                for func in module.functions() {
                    if let Some(name) = &func.name {
                        if name.starts_with("emit_") || name.contains("_event") {
                            // This is an event emitter function following Neo N3 patterns
                            
                            // Extract the event name - typically it follows the emit_ prefix
                            // or is contained in a string literal in the function body
                            let event_name = if name.starts_with("emit_") {
                                name.trim_start_matches("emit_").to_string()
                            } else {
                                // Extract from function name
                                name.split('_')
                                    .find(|s| s == &"event")
                                    .map(|_| name.replace("_event", ""))
                                    .unwrap_or_else(|| name.to_string())
                            };
                            
                            // Build parameter list based on function parameters
                            // In Neo N3, these parameters will be converted to Any type
                            let mut parameters = Vec::new();
                            for (i, param_type) in func.params.iter().enumerate() {
                                let param_name = format!("p{}", i);
                                let neo_type = self.wasm_type_to_neo_type(param_type)?;
                                parameters.push(ContractParameterDefinition::new(param_name, neo_type));
                            }
                            
                            events.push(ContractEventDefinition::new(
                                // Convert to CamelCase for Neo N3 convention
                                event_name.split('_')
                                    .map(|s| {
                                        let mut c = s.chars();
                                        match c.next() {
                                            None => String::new(),
                                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                        }
                                    })
                                    .collect::<String>(),
                                parameters
                            ));
                        }
                    }
                }
            }
        }
        
        // If we couldn't detect events, look for typical naming patterns in the code
        if events.is_empty() {
            // Look for potential event emitter functions in the module
            for func in module.functions() {
                if let Some(name) = &func.name {
                    if name.starts_with("emit_") || name.ends_with("_event") {
                        let event_name = if name.starts_with("emit_") {
                            name.trim_start_matches("emit_").to_string()
                        } else {
                            name.trim_end_matches("_event").to_string()
                        };
                    
                        // Use CamelCase for the event name
                        let event_name = event_name.split('_')
                            .map(|s| {
                                let mut c = s.chars();
                                match c.next() {
                                    None => String::new(),
                                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                }
                            })
                            .collect::<String>();
                        
                        // Build parameter list
                        let mut parameters = Vec::new();
                        for (i, param_type) in func.params.iter().enumerate() {
                            let param_name = format!("p{}", i);
                            let neo_type = self.wasm_type_to_neo_type(param_type)?;
                            parameters.push(ContractParameterDefinition::new(param_name, neo_type));
                        }
                        
                        events.push(ContractEventDefinition::new(event_name, parameters));
                    }
                }
            }
        }
        
        Ok(ContractAbi::new(methods, events))
    }

    /// Converts a WebAssembly type to a Neo type.
    fn wasm_type_to_neo_type(&self, wasm_type: &str) -> Result<String, Error> {
        match wasm_type {
            "i32" | "i64" => Ok("Integer".to_string()),
            "f32" | "f64" => Ok("Integer".to_string()),  // Neo VM doesn't have native floating point, so we map to Integer
            "v128" => Ok("ByteArray".to_string()),        // 128-bit vector maps to ByteArray
            "externref" | "anyref" => Ok("Any".to_string()),
            "funcref" => Ok("InteropInterface".to_string()),
            _ if wasm_type.starts_with("string") => Ok("String".to_string()),
            _ if wasm_type.starts_with("array") => Ok("Array".to_string()),
            _ if wasm_type.starts_with("map") => Ok("Map".to_string()),
            _ if wasm_type.starts_with("h160") || wasm_type.contains("address") => Ok("Hash160".to_string()),
            _ if wasm_type.starts_with("h256") || wasm_type.contains("hash") => Ok("Hash256".to_string()),
            _ if wasm_type.contains("public_key") => Ok("PublicKey".to_string()),
            _ if wasm_type.contains("signature") => Ok("Signature".to_string()),
            _ if wasm_type.contains("bool") => Ok("Boolean".to_string()),
            _ if wasm_type.contains("byte") || wasm_type.contains("buffer") => Ok("ByteArray".to_string()),
            _ => {
                // If this is a debug build, warn about unrecognized type but default to Any
                if self.options.debug {
                    println!("Warning: Unrecognized WASM type: {}. Defaulting to Any.", wasm_type);
                }
                Ok("Any".to_string()) // Default to Any for unknown types in production
            }
        }
    }

    /// Determines if a method is safe (read-only).
    /// 
    /// In Neo N3, safe methods are read-only and don't modify blockchain state.
    /// They are marked with #[safe] attribute in the contract and represented 
    /// with "safe": true in the manifest.
    fn is_safe_method(&self, name: &str) -> bool {
        // Check for explicit safe marker in function metadata (would be added during parsing)
        // This would require additional WASM parsing to detect #[safe] attribute
        
        // For now, use heuristic based on naming conventions
        // Methods that start with "get", "is", "has", "check", "find", "query", "calculate" are typically read-only
        name.starts_with("get") || 
        name.starts_with("is") || 
        name.starts_with("has") || 
        name.starts_with("check") || 
        name.starts_with("find") || 
        name.starts_with("query") || 
        name.starts_with("calculate") || 
        name.starts_with("compute") || 
        name.starts_with("view") || 
        name.starts_with("balance") || 
        name.starts_with("total") || 
        name.starts_with("symbol") || 
        name.starts_with("decimals") || 
        name.starts_with("name") ||
        // Main contract interface methods that are known to be safe
        name == "balanceOf" || 
        name == "totalSupply" || 
        name == "symbol" || 
        name == "decimals" ||
        name == "name" ||
        name == "supportedStandards"
    }

    /// Detects if the contract uses storage.
    fn detect_storage_usage(&self, module: &WasmModule) -> Result<bool, Error> {
        // First, check imports for storage-related functions
        for import in module.imports() {
            // Check for Neo N3 storage-related imports using multiple detection methods
            
            // 1. Check module name patterns - Neo N3 typically uses 'storage' or 'neo_storage' module
            if import.module.contains("storage") || import.module.contains("Store") {
                return Ok(true);
            }
            
            // 2. Check for storage-related function names in any module
            if import.field.contains("storage") ||
               import.field.contains("Storage") ||
               import.field.contains("store") ||
               import.field.contains("Store") ||
               import.field.contains("get_") ||
               import.field.contains("put_") ||
               import.field.contains("delete_") ||
               import.field.contains("remove_") ||
               import.field == "get" ||
               import.field == "put" ||
               import.field == "delete" ||
               import.field == "find" ||
               import.field == "find_by_prefix" ||
               import.field.contains("Item") ||
               import.field.contains("Map") ||
               import.field.contains("Set") ||
               import.field.contains("Collection") {
                return Ok(true);
            }
            
            // 3. Check for specific Neo N3 storage patterns used in newer frameworks
            if (import.module == "neo_contract" || 
                import.module == "neo_sdk" || 
                import.module == "neo_std") && 
               (import.field.contains("Item") || 
                import.field.contains("Map") || 
                import.field.contains("Set") ||
                import.field.contains("Collection")) {
                return Ok(true);
            }
        }
        
        // Look for storage annotations in the code (would require more advanced code analysis)
        // For now, check function names for storage-related operations as a heuristic
        for function in module.functions() {
            if let Some(name) = &function.name {
                if name.contains("storage") ||
                   name.contains("Storage") ||
                   name.contains("store") ||
                   name.contains("Store") ||
                   name.contains("save") ||
                   name.contains("Save") ||
                   name.contains("persist") ||
                   name.contains("Persist") ||
                   name.contains("load") ||
                   name.contains("Load") {
                    return Ok(true);
                }
            }
        }
        
        // Check for common Neo N3 storage attribute patterns in the global data section
        // This would require more advanced bytecode analysis, this is a placeholder
        
        if self.options.debug {
            println!("Notice: No storage usage explicitly detected in the contract.");
        }
        
        // No storage usage detected, but we may have missed something
        // If this is a debug build, default to requiring storage permission for safety
        // In production, default to false unless explicitly detected
        Ok(self.options.debug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compiler_basics() {
        // This is just a template for a test
        // We'd need a sample WebAssembly file to test with
        
        // For now, just check that we can create a compiler
        let compiler = Compiler::new();
        assert!(!compiler.options.debug);
        assert!(compiler.options.optimize);
        
        // Test the builder pattern
        let compiler = Compiler::new()
            .with_debug(true)
            .with_optimize(false);
        
        assert!(compiler.options.debug);
        assert!(!compiler.options.optimize);
    }
}