//! Neo Smart Contract Compiler
//!
//! This crate provides functionality for compiling WebAssembly modules
//! into Neo N3 smart contracts. It converts WASM bytecode to NEO VM script,
//! and generates the necessary NEF files and manifests.

use std::path::{Path, PathBuf};

// Re-export important types for a clean public API
pub use crate::compiler::{Compiler, CompilerOptions};
pub use crate::error::Error;
pub use crate::manifest::Manifest;
pub use crate::compiler::ManifestOverride;
pub use crate::nef::NefFile;
pub use crate::script::Script;
pub use crate::wasm::WasmModule;

// Core modules
pub mod compiler;
pub mod error;
pub mod manifest;
pub mod nef;
pub mod script;
pub mod wasm;
pub mod neo;

// Utility modules
pub mod converter;
pub mod utils;

/// Result of a successful compilation containing paths to the generated files
#[derive(Debug)]
pub struct CompilationResult {
    /// Path to the generated NEF file
    pub nef_path: PathBuf,
    /// Path to the generated manifest file
    pub manifest_path: PathBuf,
    /// Name of the compiled contract
    pub contract_name: String,
}

/// Compile a WebAssembly file to a Neo N3 smart contract
///
/// # Arguments
///
/// * `wasm_path` - Path to the WebAssembly file
/// * `output_dir` - Output directory for the compiled files
/// * `name` - Optional name of the contract (defaults to the filename without extension)
///
/// # Returns
///
/// * `Result<(), Error>` - Result of the compilation
pub fn compile<P: AsRef<std::path::Path>>(
    wasm_path: P,
    output_dir: P,
    name: Option<&str>,
) -> Result<(), error::Error> {
    let compiler = Compiler::new();
    let contract_name = name.map(|s| s.to_string()).unwrap_or_else(|| {
        wasm_path
            .as_ref()
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });
    
    compiler.compile(wasm_path, output_dir, &contract_name)
}

/// Quick compile with default settings
///
/// This is a convenience function that compiles a WebAssembly file with default settings.
///
/// # Arguments
///
/// * `wasm_path` - Path to the WebAssembly file
///
/// # Returns
///
/// * `Result<(), Error>` - Result of the compilation
pub fn quick_compile<P: AsRef<std::path::Path>>(wasm_path: P) -> Result<(), error::Error> {
    // Convert to PathBuf for both parameters since we need to modify output_dir
    let wasm_path_buf = PathBuf::from(wasm_path.as_ref());
    let output_dir = wasm_path_buf.parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    
    let compiler = Compiler::new();
    let contract_name = wasm_path_buf
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    compiler.compile(wasm_path_buf, output_dir, &contract_name)
}

/// Compile a WebAssembly file with specific compiler options
///
/// # Arguments
///
/// * `wasm_path` - Path to the WebAssembly file
/// * `options` - Compiler options
///
/// # Returns
///
/// * `Result<CompilationResult, Error>` - Result of the compilation including output file paths
pub fn compile_with_options<P: AsRef<Path>>(
    wasm_path: P,
    options: CompilerOptions,
) -> Result<CompilationResult, Error> {
    // Load and parse the WebAssembly module
    let wasm_data = std::fs::read(&wasm_path).map_err(Error::Io)?;
    let module = WasmModule::parse(&wasm_data)?;
    
    // Get contract name either from options or from file name
    let contract_name = options.contract_name.clone().unwrap_or_else(|| {
        wasm_path
            .as_ref()
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });
    
    // Get output directory
    let output_dir = std::env::current_dir().map_err(Error::Io)?;
    
    // Create a new compiler and compile the contract
    let compiler = Compiler::with_options(options);
    
    // Convert WebAssembly to Neo script
    let converter = converter::WasmConverter::new();
    let script_bytes = converter.convert_to_script(&module, compiler.get_options().optimize)?;
    let script = Script::from_bytes(&script_bytes);
    
    // Create the NEF file
    let mut nef = NefFile::with_script(script.bytes().to_vec());
    nef.finalize()?;
    
    // Generate the manifest
    let manifest = compiler.generate_manifest(&contract_name, &module)?;
    
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir).map_err(Error::Io)?;
    
    // Save the NEF file and manifest
    let nef_path = output_dir.join(format!("{}.nef", contract_name));
    let manifest_path = output_dir.join(format!("{}.manifest.json", contract_name));
    
    nef.save_to(&nef_path)?;
    manifest.save_to_file(&manifest_path)?;
    
    if compiler.get_options().debug {
        println!("Saved NEF file: {}", nef_path.display());
        println!("Saved manifest: {}", manifest_path.display());
    }
    
    Ok(CompilationResult {
        nef_path,
        manifest_path,
        contract_name,
    })
}