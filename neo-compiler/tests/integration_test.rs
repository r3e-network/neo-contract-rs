//! Integration tests for the Neo Contract Compiler
//!
//! These tests verify that the compiler correctly processes WebAssembly
//! modules and produces valid Neo N3 smart contracts.

use std::fs;
use std::path::Path;
use neo_compiler::{compile_with_options, CompilerOptions, ManifestOverride};

// Add dependency on tempfile for temporary directory management
use tempfile;

// Path to a test fixture WASM file for testing
// In a real implementation, this would point to an actual WASM file
// generated from a test Rust contract
const TEST_WASM_PATH: &str = "tests/fixtures/test_contract.wasm";

#[test]
fn test_basic_compilation() -> anyhow::Result<()> {
    // Skip this test if the test WASM file doesn't exist
    if !Path::new(TEST_WASM_PATH).exists() {
        println!("Skipping test_basic_compilation: test WASM file not found");
        return Ok(());
    }

    // Create a temporary directory for output files
    let temp_dir = tempfile::tempdir()?;
    
    // Set compiler options
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: None, // Using None instead of Some(Vec::new())
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };
    
    // Compile the test WASM file
    let result = compile_with_options(TEST_WASM_PATH, options)?;
    
    // Verify that the output files exist
    assert!(result.nef_path.exists(), "NEF file was not created");
    assert!(result.manifest_path.exists(), "Manifest file was not created");
    
    // Verify that the NEF file contains the expected data
    let nef_data = fs::read(&result.nef_path)?;
    assert!(!nef_data.is_empty(), "NEF file is empty");
    
    // Verify that the manifest file contains the expected JSON
    let manifest_data = fs::read_to_string(&result.manifest_path)?;
    assert!(manifest_data.contains("\"name\""), "Manifest does not contain name field");
    assert!(manifest_data.contains("\"abi\""), "Manifest does not contain ABI field");
    Ok(())
}

#[test]
fn test_manifest_overrides() -> anyhow::Result<()> {
    // Skip this test if the test WASM file doesn't exist
    if !Path::new(TEST_WASM_PATH).exists() {
        println!("Skipping test_manifest_overrides: test WASM file not found");
        return Ok(());
    }

    // Create a temporary directory for output files
    let temp_dir = tempfile::tempdir()?;
    
    // Custom name for testing
    let custom_name = "custom_contract_name";
    
    // Set compiler options with a manifest override
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: Some(vec![
            ManifestOverride {
                key: "name".to_string(),
                value: custom_name.to_string(), // Using String instead of Value::String
            },
        ]),
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };
    
    // Compile the test WASM file
    let result = compile_with_options(TEST_WASM_PATH, options)?;
    
    // Verify that the manifest file contains the overridden name
    let manifest_data = fs::read_to_string(&result.manifest_path)?;
    assert!(manifest_data.contains(&format!("\"name\":\"{custom_name}\"")), 
            "Manifest does not contain the overridden name");
    
    // tempfile::TempDir automatically cleans up when it goes out of scope
    Ok(())
}

// This function could be used to create a minimal valid WASM file for testing
// if the test fixture doesn't exist
#[allow(dead_code)]
fn create_minimal_wasm() -> Vec<u8> {
    // A minimal valid WebAssembly module with a single exported function
    // This is just a binary representation of (module (func (export "main")))
    vec![
        0x00, 0x61, 0x73, 0x6D, // magic number: "\0asm"
        0x01, 0x00, 0x00, 0x00, // version: 1
        
        // Type section
        0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // (type (func))
        
        // Function section
        0x03, 0x02, 0x01, 0x00, // (func (type 0))
        
        // Export section
        0x07, 0x07, 0x01, 0x04, 0x6D, 0x61, 0x69, 0x6E, 0x00, 0x00, // (export "main" (func 0))
        
        // Code section
        0x0A, 0x04, 0x01, 0x02, 0x00, 0x0B, // (code (func (body end)))
    ]
}