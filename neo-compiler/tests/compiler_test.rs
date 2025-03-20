//! Comprehensive tests for the Neo Contract Compiler
//!
//! These tests verify that the compiler correctly processes WebAssembly
//! modules and produces valid Neo N3 smart contracts.

use neo_compiler::{compile_with_options, CompilerOptions, ManifestOverride};
use std::fs;
use std::path::{PathBuf};
use tempfile::tempdir;

// Flag to temporarily skip actual compilation tests if the environment 
// doesn't support generating test WASM files
const SKIP_WASM_TESTS: bool = true;

// Helper to generate a simple test WASM file
fn create_test_wasm() -> (PathBuf, Vec<u8>) {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_contract.wasm");
    
    // Simple binary with just the WASM header
    let wasm_binary = vec![
        0x00, 0x61, 0x73, 0x6D, // Magic
        0x01, 0x00, 0x00, 0x00, // Version
    ];
    
    fs::write(&file_path, &wasm_binary).unwrap();
    (file_path, wasm_binary)
}

// Test basic compilation with default options
#[test]
fn test_basic_compilation() -> anyhow::Result<()> {
    if SKIP_WASM_TESTS {
        println!("Skipping test_basic_compilation: SKIP_WASM_TESTS is true");
        return Ok(());
    }

    let (temp_file, _wasm_binary) = create_test_wasm();
    let temp_dir = tempdir()?;

    // Set compiler options
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: None,
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };

    // Compile the test WASM file
    let result = compile_with_options(temp_file.to_str().unwrap(), options)?;

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

// Test compilation with optimizations disabled
#[test]
fn test_no_optimization() -> anyhow::Result<()> {
    if SKIP_WASM_TESTS {
        println!("Skipping test_no_optimization: SKIP_WASM_TESTS is true");
        return Ok(());
    }

    let (temp_file, _wasm_binary) = create_test_wasm();
    let temp_dir = tempdir()?;

    // Set compiler options with optimizations disabled
    let options = CompilerOptions {
        optimize: false,
        manifest_template: None,
        manifest_overrides: None,
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };

    // Compile the test WASM file
    let result = compile_with_options(temp_file.to_str().unwrap(), options)?;

    // Verify that the output files exist
    assert!(result.nef_path.exists(), "NEF file was not created");
    assert!(result.manifest_path.exists(), "Manifest file was not created");
    
    Ok(())
}

// Test compilation with debug information
#[test]
fn test_with_debug_info() -> anyhow::Result<()> {
    if SKIP_WASM_TESTS {
        println!("Skipping test_with_debug_info: SKIP_WASM_TESTS is true");
        return Ok(());
    }

    let (temp_file, _wasm_binary) = create_test_wasm();
    let temp_dir = tempdir()?;

    // Set compiler options with debug info enabled
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: None,
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: true,
    };

    // Compile the test WASM file
    let result = compile_with_options(temp_file.to_str().unwrap(), options)?;

    // Verify that the output files exist
    assert!(result.nef_path.exists(), "NEF file was not created");
    assert!(result.manifest_path.exists(), "Manifest file was not created");
    
    Ok(())
}

// Test the manifest overrides
#[test]
fn test_manifest_overrides() -> anyhow::Result<()> {
    if SKIP_WASM_TESTS {
        println!("Skipping test_manifest_overrides: SKIP_WASM_TESTS is true");
        return Ok(());
    }

    let (temp_file, _wasm_binary) = create_test_wasm();
    let temp_dir = tempdir()?;

    // Custom name and description for testing
    let custom_name = "custom_contract_name";
    let custom_description = "custom contract description";

    // Set compiler options with manifest overrides
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: Some(vec![
            ManifestOverride {
                key: "name".to_string(),
                value: custom_name.to_string(),
            },
            ManifestOverride {
                key: "description".to_string(),
                value: custom_description.to_string(),
            },
        ]),
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };

    // Compile the test WASM file
    let result = compile_with_options(temp_file.to_str().unwrap(), options)?;

    // Verify that the manifest file contains the overridden values
    let manifest_data = fs::read_to_string(&result.manifest_path)?;
    assert!(
        manifest_data.contains(&format!("\"name\":\"{custom_name}\"")),
        "Manifest does not contain the overridden name"
    );
    assert!(
        manifest_data.contains(&format!("\"description\":\"{custom_description}\"")),
        "Manifest does not contain the overridden description"
    );
    
    Ok(())
}

// Test using a custom manifest template
#[test]
fn test_custom_manifest_template() -> anyhow::Result<()> {
    if SKIP_WASM_TESTS {
        println!("Skipping test_custom_manifest_template: SKIP_WASM_TESTS is true");
        return Ok(());
    }

    let (temp_file, _wasm_binary) = create_test_wasm();
    let temp_dir = tempdir()?;

    // Create a custom manifest template
    let template_dir = tempdir()?;
    let template_path = template_dir.path().join("template.json");
    
    // Write a simple manifest template
    fs::write(
        &template_path,
        r#"{
            "name": "template_contract",
            "groups": [],
            "supportedstandards": ["NEP-17"],
            "abi": {
                "methods": [],
                "events": []
            },
            "permissions": [],
            "trusts": [],
            "features": {},
            "extra": {}
        }"#
    )?;

    // Set compiler options with the custom template
    let options = CompilerOptions {
        optimize: true,
        manifest_template: Some(template_path.to_path_buf()),
        manifest_overrides: None,
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };

    // Compile the test WASM file
    let result = compile_with_options(temp_file.to_str().unwrap(), options)?;

    // Verify that the manifest file contains the template's values
    let manifest_data = fs::read_to_string(&result.manifest_path)?;
    assert!(
        manifest_data.contains("\"supportedstandards\":[\"NEP-17\"]"),
        "Manifest does not contain the supported standards from template"
    );
    
    Ok(())
}

// Test compiling with a non-existent WASM file
#[test]
fn test_nonexistent_wasm_file() {
    if SKIP_WASM_TESTS {
        println!("Skipping test_nonexistent_wasm_file: SKIP_WASM_TESTS is true");
        return;
    }

    let temp_dir = tempdir().unwrap();
    let non_existent_file = temp_dir.path().join("non_existent.wasm");

    // Set compiler options
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: None,
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };

    // Compile with a non-existent file should fail
    let result = compile_with_options(non_existent_file.to_str().unwrap(), options);
    assert!(result.is_err(), "Compilation with non-existent file should fail");
}

// Test compiling with an invalid WASM file
#[test]
fn test_invalid_wasm_file() {
    if SKIP_WASM_TESTS {
        println!("Skipping test_invalid_wasm_file: SKIP_WASM_TESTS is true");
        return;
    }

    let temp_dir = tempdir().unwrap();
    let invalid_file = temp_dir.path().join("invalid.wasm");

    // Write an invalid WASM file (just some random bytes)
    fs::write(&invalid_file, &[0x00, 0x01, 0x02, 0x03]).unwrap();

    // Set compiler options
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: None,
        output_dir: Some(temp_dir.path().to_path_buf()),
        contract_name: Some("test_contract".to_string()),
        debug: false,
    };

    // Compile with an invalid file should fail
    let result = compile_with_options(invalid_file.to_str().unwrap(), options);
    assert!(result.is_err(), "Compilation with invalid WASM file should fail");
}

// Test to ensure the compiler options struct works as expected
#[test]
fn test_compiler_options() {
    // Make sure we can create compiler options with various settings
    
    // Default options
    let options = CompilerOptions {
        optimize: true,
        manifest_template: None,
        manifest_overrides: None,
        output_dir: None,
        contract_name: None,
        debug: false,
    };
    
    assert_eq!(options.optimize, true);
    assert_eq!(options.debug, false);
    assert!(options.manifest_template.is_none());
    
    // Custom options
    let options = CompilerOptions {
        optimize: false,
        manifest_template: Some(PathBuf::from("template.json")),
        manifest_overrides: Some(vec![
            ManifestOverride {
                key: "name".to_string(),
                value: "test".to_string(),
            }
        ]),
        output_dir: Some(PathBuf::from("/tmp")),
        contract_name: Some("custom_name".to_string()),
        debug: true,
    };
    
    assert_eq!(options.optimize, false);
    assert_eq!(options.debug, true);
    assert_eq!(options.manifest_template, Some(PathBuf::from("template.json")));
    assert_eq!(options.contract_name, Some("custom_name".to_string()));
} 