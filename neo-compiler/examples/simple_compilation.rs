//! Simple example of using the neo-compiler API to compile a WebAssembly module to Neo bytecode.
//!
//! This example demonstrates how to use the neo-compiler API to:
//! 1. Parse a WebAssembly module
//! 2. Convert it to Neo VM bytecode
//! 3. Generate NEF and manifest files

use neo_compiler::{Compiler, CompilerOptions, WasmModule};
use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <wasm_file> [output_dir]", args[0]);
        return Ok(());
    }

    let wasm_path = PathBuf::from(&args[1]);
    let output_dir = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from(".")
    };

    // Ensure the output directory exists
    fs::create_dir_all(&output_dir)?;

    println!("Compiling WebAssembly module: {}", wasm_path.display());
    println!("Output directory: {}", output_dir.display());

    // 1. Load and parse the WebAssembly module
    println!("\nStep 1: Loading WebAssembly module...");
    let wasm_data = fs::read(&wasm_path)?;
    let module = WasmModule::parse(&wasm_data)?;

    // Print module information
    println!("  Module loaded successfully!");
    println!("  Functions: {}", module.functions().len());
    println!("  Exports: {}", module.exports().len());
    println!("  Imports: {}", module.imports().len());

    // 2. Configure the compiler
    println!("\nStep 2: Configuring compiler...");
    let options = CompilerOptions {
        debug: true,
        optimize: true,
        contract_name: Some(
            wasm_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        ),
        manifest_template: None,
        manifest_overrides: Some(Vec::new()),
        output_dir: Some(std::env::current_dir()?),
    };

    // 3. Compile the module
    println!("\nStep 3: Compiling module...");
    let compiler = Compiler::with_options(options);
    let contract_name = wasm_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    compiler.compile(&wasm_path, &output_dir, &contract_name)?;

    // 4. Verify output files
    println!("\nStep 4: Verifying output files...");
    let nef_path = output_dir.join(format!("{}.nef", contract_name));
    let manifest_path = output_dir.join(format!("{}.manifest.json", contract_name));

    if nef_path.exists() {
        println!("  NEF file created: {}", nef_path.display());
    } else {
        println!("  Error: NEF file not created!");
    }

    if manifest_path.exists() {
        println!("  Manifest file created: {}", manifest_path.display());
    } else {
        println!("  Error: Manifest file not created!");
    }

    println!("\nCompilation completed successfully!");
    Ok(())
} 