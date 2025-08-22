use neo_compiler::{NeoCompiler, manifest::Manifest};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to compile Rust code to WASM
fn compile_rust_to_wasm(rust_code: &str, contract_name: &str) -> Result<PathBuf, String> {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join(contract_name);
    fs::create_dir_all(&project_dir).unwrap();
    
    // Create Cargo.toml
    let cargo_toml = format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
neo-contract = {{ path = "../../../neo-contract" }}

[lib]
crate-type = ["cdylib"]

[profile.release]
lto = true
opt-level = 3
"#, contract_name);
    
    fs::write(project_dir.join("Cargo.toml"), cargo_toml).unwrap();
    
    // Create src directory and lib.rs
    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    fs::write(src_dir.join("lib.rs"), rust_code).unwrap();
    
    // Build with cargo
    let rustflags = "-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152";
    let output = Command::new("cargo")
        .current_dir(&project_dir)
        .env("RUSTFLAGS", rustflags)
        .args(&[
            "build",
            "--target", "wasm32-unknown-unknown",
            "--release"
        ])
        .output()
        .map_err(|e| format!("Failed to run cargo: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Cargo build failed: {}", stderr));
    }
    
    let wasm_path = project_dir
        .join("target/wasm32-unknown-unknown/release")
        .join(format!("{}.wasm", contract_name.replace('-', "_")));
    
    if !wasm_path.exists() {
        return Err("WASM file not found".to_string());
    }
    
    // Copy to a stable location
    let stable_path = temp_dir.path().join(format!("{}.wasm", contract_name));
    fs::copy(&wasm_path, &stable_path).unwrap();
    
    Ok(stable_path)
}

#[test]
#[ignore] // Requires cargo and neo-contract to be built
fn test_e2e_hello_world_contract() {
    let rust_code = r#"
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use neo_contract::prelude::*;

#[no_mangle]
pub extern "C" fn hello() -> String {
    String::from("Hello, World!")
}

#[no_mangle]
pub extern "C" fn greet(name: String) -> String {
    let mut result = String::from("Hello, ");
    result.push_str(&name);
    result.push('!');
    result
}
"#;

    // Compile Rust to WASM
    let wasm_path = compile_rust_to_wasm(rust_code, "hello_world")
        .expect("Failed to compile Rust to WASM");
    
    // Compile WASM to NEF
    let temp_dir = TempDir::new().unwrap();
    let compiler = NeoCompiler::new()
        .with_output_dir(temp_dir.path().to_path_buf());
    
    let result = compiler.compile(&wasm_path).unwrap();
    
    // Verify NEF
    assert!(result.nef_path.exists());
    assert!(result.nef.verify_checksum());
    assert!(!result.nef.script.is_empty());
    
    // Verify manifest
    assert!(result.manifest.abi.methods.len() >= 2);
    
    let method_names: Vec<String> = result.manifest.abi.methods
        .iter()
        .map(|m| m.name.clone())
        .collect();
    
    assert!(method_names.contains(&"hello".to_string()));
    assert!(method_names.contains(&"greet".to_string()));
}

#[test]
#[ignore] // Requires cargo and neo-contract to be built
fn test_e2e_solana_style_contract() {
    let rust_code = r#"
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use neo_contract::prelude::*;

#[contract_author("Test Contract")]
#[contract_version("1.0.0")]
pub struct TestContract {
    is_initialized: bool,
    value: u64,
}

#[contract_impl]
impl TestContract {
    pub fn init() -> Self {
        Self {
            is_initialized: false,
            value: 0,
        }
    }
    
    #[method]
    pub fn initialize(&mut self) -> Result<()> {
        require!(!self.is_initialized, ContractError::AlreadyInitialized);
        self.is_initialized = true;
        self.value = 42;
        notify!("Initialized", self.value);
        Ok(())
    }
    
    #[method]
    #[safe]
    pub fn get_value(&self) -> u64 {
        self.value
    }
    
    #[method]
    pub fn set_value(&mut self, value: u64) -> Result<()> {
        self.value = value;
        notify!("ValueChanged", value);
        Ok(())
    }
}
"#;

    // Compile Rust to WASM
    let wasm_path = compile_rust_to_wasm(rust_code, "solana_style")
        .expect("Failed to compile Rust to WASM");
    
    // Compile WASM to NEF
    let temp_dir = TempDir::new().unwrap();
    let compiler = NeoCompiler::new()
        .with_output_dir(temp_dir.path().to_path_buf());
    
    let result = compiler.compile(&wasm_path).unwrap();
    
    // Should detect Solana style
    assert!(result.is_solana_style);
    
    // Verify NEF
    assert!(result.nef_path.exists());
    assert!(result.nef.verify_checksum());
    
    // Verify manifest has correct methods
    let method_names: Vec<String> = result.manifest.abi.methods
        .iter()
        .map(|m| m.name.clone())
        .collect();
    
    assert!(method_names.contains(&"initialize".to_string()));
    assert!(method_names.contains(&"get_value".to_string()));
    assert!(method_names.contains(&"set_value".to_string()));
    
    // get_value should be marked as safe (read-only)
    let get_value = result.manifest.abi.methods
        .iter()
        .find(|m| m.name == "get_value")
        .unwrap();
    assert!(get_value.safe);
}

#[test]
#[ignore] // Requires cargo and neo-contract to be built
fn test_e2e_nep17_token_contract() {
    let rust_code = r#"
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use neo_contract::prelude::*;

#[no_mangle]
pub extern "C" fn symbol() -> String {
    String::from("TEST")
}

#[no_mangle]
pub extern "C" fn decimals() -> u8 {
    8
}

#[no_mangle]
pub extern "C" fn totalSupply() -> u128 {
    1_000_000_00000000
}

#[no_mangle]
pub extern "C" fn balanceOf(account: H160) -> u128 {
    // Simplified - would use storage in real implementation
    if account == H160::zero() {
        1_000_000_00000000
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn transfer(from: H160, to: H160, amount: u128, data: Any) -> bool {
    // Simplified - would implement full logic
    true
}
"#;

    // Compile Rust to WASM
    let wasm_path = compile_rust_to_wasm(rust_code, "nep17_token")
        .expect("Failed to compile Rust to WASM");
    
    // Compile WASM to NEF
    let temp_dir = TempDir::new().unwrap();
    let compiler = NeoCompiler::new()
        .with_output_dir(temp_dir.path().to_path_buf());
    
    let result = compiler.compile(&wasm_path).unwrap();
    
    // Verify NEF
    assert!(result.nef_path.exists());
    assert!(result.nef.verify_checksum());
    
    // Should have NEP-17 methods
    let method_names: Vec<String> = result.manifest.abi.methods
        .iter()
        .map(|m| m.name.clone())
        .collect();
    
    assert!(method_names.contains(&"symbol".to_string()));
    assert!(method_names.contains(&"decimals".to_string()));
    assert!(method_names.contains(&"totalSupply".to_string()));
    assert!(method_names.contains(&"balanceOf".to_string()));
    assert!(method_names.contains(&"transfer".to_string()));
    
    // Read-only methods should be marked as safe
    for method in &result.manifest.abi.methods {
        match method.name.as_str() {
            "symbol" | "decimals" | "totalSupply" | "balanceOf" => {
                assert!(method.safe, "{} should be safe", method.name);
            }
            "transfer" => {
                assert!(!method.safe, "transfer should not be safe");
            }
            _ => {}
        }
    }
}

#[test]
fn test_compile_all_examples() {
    // This test compiles all example contracts if they exist
    let examples_dir = PathBuf::from("../examples");
    
    if !examples_dir.exists() {
        println!("Examples directory not found, skipping");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let compiler = NeoCompiler::new()
        .with_output_dir(temp_dir.path().to_path_buf());
    
    let results = compiler.compile_all_examples().unwrap();
    
    // We should have compiled at least some examples
    if examples_dir.exists() {
        println!("Compiled {} examples", results.len());
    }
    
    // Verify all compiled contracts
    for result in results {
        assert!(result.nef_path.exists());
        assert!(result.manifest_path.exists());
        assert!(result.nef.verify_checksum());
        assert!(!result.nef.script.is_empty());
    }
}

#[test]
fn test_nef_size_limits() {
    // NEF files have size limits we should respect
    let large_wasm = wat::parse_str(r#"
        (module
            (func $main
                ;; Generate a large function body
                i32.const 0
                i32.const 0
                i32.const 0
                drop
                drop
                drop
            )
            (export "main" (func $main))
        )
    "#).unwrap();
    
    let temp_dir = TempDir::new().unwrap();
    let wasm_path = temp_dir.path().join("large.wasm");
    fs::write(&wasm_path, &large_wasm).unwrap();
    
    let compiler = NeoCompiler::new()
        .with_output_dir(temp_dir.path().to_path_buf());
    
    let result = compiler.compile(&wasm_path).unwrap();
    
    // NEF should be within reasonable size limits
    let nef_size = result.nef.size();
    assert!(nef_size < 1024 * 1024); // Less than 1MB
    
    // Script should be within VM limits
    assert!(result.nef.script.len() < 65536); // Less than 64KB
}