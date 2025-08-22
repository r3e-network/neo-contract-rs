use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use wasmparser::{Parser, Payload, ValType};

pub mod debug;
pub mod manifest;
pub mod memory_model;
pub mod nef;
pub mod opcodes;
pub mod optimizer;
pub mod solana_detector;
pub mod translator;
pub mod wasm_parser;

use manifest::Manifest;
use nef::Nef3;
use solana_detector::SolanaStyleDetector;
use translator::WasmTranslator;

/// Main compiler structure for converting WASM to NEF
pub struct NeoCompiler {
    /// Enable debug output
    debug: bool,
    /// Source identifier
    source: String,
    /// Output directory
    output_dir: PathBuf,
}

impl NeoCompiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self {
            debug: false,
            source: "neo-compiler".to_string(),
            output_dir: PathBuf::from("build"),
        }
    }

    /// Enable debug mode
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set the source identifier
    pub fn with_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    /// Set the output directory
    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = dir;
        self
    }

    /// Compile a WASM file to NEF format
    pub fn compile(&self, wasm_path: &Path) -> Result<CompilationResult> {
        // Read WASM file
        let wasm_bytes = fs::read(wasm_path)
            .with_context(|| format!("Failed to read WASM file: {:?}", wasm_path))?;

        // Parse WASM module
        let module = self.parse_wasm(&wasm_bytes)?;
        
        // Detect if it's Solana-style
        let mut detector = SolanaStyleDetector::new(&module);
        let is_solana_style = detector.detect()?;
        
        // Generate or load manifest
        let manifest = if is_solana_style {
            log::info!("Detected Solana-style contract");
            detector.generate_manifest()?
        } else {
            // Try to load existing manifest
            let manifest_path = wasm_path.with_extension("manifest.json");
            if manifest_path.exists() {
                Manifest::from_file(&manifest_path)?
            } else {
                // Generate default manifest
                Manifest::default()
            }
        };

        // Translate to NEF
        let mut translator = WasmTranslator::new(self.debug);
        
        // Pass WASM bytes to translator for enhanced instruction translation
        translator.set_wasm_bytes(wasm_bytes.clone());
        
        let nef = translator.translate(&module, &manifest, &self.source)?;

        // Prepare output paths
        let base_name = wasm_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("contract");
        
        let nef_path = self.output_dir.join(format!("{}.nef", base_name));
        let manifest_path = self.output_dir.join(format!("{}.manifest.json", base_name));
        
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)?;
        
        // Write NEF file
        let nef_bytes = nef.to_bytes()?;
        fs::write(&nef_path, &nef_bytes)?;
        
        // Write manifest file
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_json)?;

        Ok(CompilationResult {
            nef_path,
            manifest_path,
            nef,
            manifest,
            is_solana_style,
        })
    }

    /// Parse WASM bytes into a module representation
    fn parse_wasm(&self, wasm_bytes: &[u8]) -> Result<WasmModule> {
        let mut module = WasmModule::default();
        let parser = Parser::new(0);
        
        for payload in parser.parse_all(wasm_bytes) {
            match payload? {
                Payload::TypeSection(types) => {
                    for (idx, _ty) in types.into_iter().enumerate() {
                        module.types.push(idx as u32);
                    }
                }
                Payload::ImportSection(imports) => {
                    for import in imports {
                        let import = import?;
                        module.imports.push(WasmImport {
                            module: import.module.to_string(),
                            name: import.name.to_string(),
                        });
                    }
                }
                Payload::FunctionSection(functions) => {
                    for func in functions {
                        module.function_types.push(func?);
                    }
                }
                Payload::ExportSection(exports) => {
                    for export in exports {
                        let export = export?;
                        module.exports.push(WasmExport {
                            name: export.name.to_string(),
                            index: export.index,
                        });
                    }
                }
                Payload::CodeSectionEntry(body) => {
                    module.functions.push(WasmFunction {
                        locals: body.get_locals_reader()?.into_iter()
                            .collect::<Result<Vec<_>, _>>()?,
                        body: body.range().clone(),
                    });
                }
                Payload::CustomSection(section) => {
                    // Check for name section
                    if section.name() == "name" {
                        // Parse function names
                        self.parse_name_section(section.data(), &mut module)?;
                    }
                }
                _ => {} // Ignore other sections for now
            }
        }
        
        Ok(module)
    }

    /// Parse the name section to extract function names
    fn parse_name_section(&self, _data: &[u8], module: &mut WasmModule) -> Result<()> {
        // Simple name section parsing (simplified for now)
        // In a full implementation, this would properly parse the name section format
        module.function_names = HashMap::new(); // Placeholder
        Ok(())
    }

    /// Compile all contracts in the examples directory
    pub fn compile_all_examples(&self) -> Result<Vec<CompilationResult>> {
        let examples_dir = PathBuf::from("examples");
        let mut results = Vec::new();
        
        if !examples_dir.exists() {
            return Ok(results);
        }
        
        for entry in fs::read_dir(examples_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Look for WASM files in target directory
                let wasm_dir = path.join("target/wasm32-unknown-unknown/release");
                if wasm_dir.exists() {
                    for wasm_entry in fs::read_dir(wasm_dir)? {
                        let wasm_entry = wasm_entry?;
                        let wasm_path = wasm_entry.path();
                        
                        if wasm_path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                            // Skip .d files and test files
                            let name = wasm_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                            if !name.contains("test") && !name.contains("deps") {
                                log::info!("Compiling: {:?}", wasm_path);
                                match self.compile(&wasm_path) {
                                    Ok(result) => results.push(result),
                                    Err(e) => log::error!("Failed to compile {:?}: {}", wasm_path, e),
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
}

/// Result of compilation
#[derive(Debug)]
pub struct CompilationResult {
    /// Path to generated NEF file
    pub nef_path: PathBuf,
    /// Path to generated manifest file
    pub manifest_path: PathBuf,
    /// The compiled NEF structure
    pub nef: Nef3,
    /// The manifest
    pub manifest: Manifest,
    /// Whether this was detected as Solana-style
    pub is_solana_style: bool,
}

/// Simplified WASM module representation
#[derive(Default)]
pub struct WasmModule {
    pub types: Vec<u32>, // Type indices
    pub imports: Vec<WasmImport>,
    pub function_types: Vec<u32>,
    pub functions: Vec<WasmFunction>,
    pub exports: Vec<WasmExport>,
    pub function_names: HashMap<u32, String>,
}

impl WasmModule {
    /// Get all function names including generated ones
    pub fn get_function_names(&self) -> HashMap<u32, String> {
        let mut names = self.function_names.clone();
        
        // Generate names for unnamed functions
        let import_count = self.imports.len() as u32;
        for (idx, _) in self.functions.iter().enumerate() {
            let func_idx = import_count + idx as u32;
            names.entry(func_idx).or_insert_with(|| format!("func_{}", func_idx));
        }
        
        // Check exports for additional names
        for export in &self.exports {
            if let Some(name) = self.function_names.get(&export.index) {
                names.insert(export.index, name.clone());
            } else {
                names.insert(export.index, export.name.clone());
            }
        }
        
        names
    }
}

#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct WasmExport {
    pub name: String,
    pub index: u32,
}

#[derive(Debug)]
pub struct WasmFunction {
    pub locals: Vec<(u32, ValType)>,
    pub body: std::ops::Range<usize>,
}

impl Default for NeoCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_compiler_creation() {
        let compiler = NeoCompiler::new()
            .with_debug(true)
            .with_source("test".to_string());
        
        assert_eq!(compiler.source, "test");
        assert!(compiler.debug);
    }

    #[test]
    fn test_wasm_parsing() {
        // Test with a minimal WASM module
        let wasm = wat::parse_str(r#"
            (module
                (func $add (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add
                )
                (export "add" (func $add))
            )
        "#).unwrap();
        
        let compiler = NeoCompiler::new();
        let module = compiler.parse_wasm(&wasm).unwrap();
        
        assert_eq!(module.exports.len(), 1);
        assert_eq!(module.exports[0].name, "add");
    }
}