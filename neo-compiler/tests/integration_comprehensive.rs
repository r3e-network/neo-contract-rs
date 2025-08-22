//! Comprehensive Integration Tests for Neo Compiler
//! 
//! End-to-end testing of the complete compilation pipeline from
//! WASM input to deployable NEF output with manifest generation.

#![cfg(test)]

use neo_compiler::*;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

/// Complete compilation pipeline tests
mod pipeline_integration_tests {
    use super::*;

    #[test]
    fn test_hello_world_compilation_pipeline() {
        // Test complete pipeline with a simple contract
        let temp_dir = TempDir::new().unwrap();
        
        // Create minimal hello world WASM
        let hello_world_wasm = create_hello_world_wasm();
        let wasm_path = temp_dir.path().join("hello_world.wasm");
        fs::write(&wasm_path, hello_world_wasm).unwrap();
        
        let output_dir = temp_dir.path().join("output");
        
        // Compile through full pipeline
        let compiler = NeoCompiler::new()
            .with_source("hello_world_test".to_string())
            .with_output_dir(output_dir.clone());
        
        let result = compiler.compile(&wasm_path);
        assert!(result.is_ok(), "Hello world compilation should succeed");
        
        let compilation_result = result.unwrap();
        
        // Validate outputs exist
        assert!(compilation_result.nef_path.exists(), "NEF file should be created");
        assert!(compilation_result.manifest_path.exists(), "Manifest file should be created");
        
        // Validate NEF content
        let nef_content = fs::read(&compilation_result.nef_path).unwrap();
        assert!(nef_content.len() > 320, "NEF should have header + script"); // Standard NEF header is ~320 bytes
        assert_eq!(&nef_content[0..4], b"NEF3", "Should have NEF3 magic");
        
        // Validate manifest content
        let manifest_content = fs::read_to_string(&compilation_result.manifest_path).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
        
        assert_eq!(manifest["name"], "Contract");
        assert!(manifest["features"]["storage"].as_bool().unwrap_or(false));
        assert!(manifest["abi"]["methods"].is_array());
    }

    #[test]
    fn test_nep17_token_compilation_pipeline() {
        // Test compilation of a more complex NEP-17 token contract
        let temp_dir = TempDir::new().unwrap();
        
        let nep17_wasm = create_nep17_wasm();
        let wasm_path = temp_dir.path().join("nep17_token.wasm");
        fs::write(&wasm_path, nep17_wasm).unwrap();
        
        let output_dir = temp_dir.path().join("nep17_output");
        
        let compiler = NeoCompiler::new()
            .with_source("nep17_test".to_string())
            .with_output_dir(output_dir);
        
        let result = compiler.compile(&wasm_path);
        assert!(result.is_ok(), "NEP-17 compilation should succeed");
        
        let compilation_result = result.unwrap();
        
        // Validate manifest has NEP-17 methods
        let manifest_content = fs::read_to_string(&compilation_result.manifest_path).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
        
        let methods = manifest["abi"]["methods"].as_array().unwrap();
        
        // Check for NEP-17 required methods
        let method_names: Vec<String> = methods.iter()
            .map(|m| m["name"].as_str().unwrap().to_string())
            .collect();
        
        let nep17_methods = vec!["symbol", "decimals", "totalSupply", "balanceOf", "transfer"];
        for required_method in nep17_methods {
            assert!(method_names.contains(&required_method.to_string()), 
                   "Should detect {} method", required_method);
        }
    }

    #[test]
    fn test_complex_defi_compilation() {
        // Test compilation of complex DeFi contract with multiple methods
        let temp_dir = TempDir::new().unwrap();
        
        let defi_wasm = create_complex_defi_wasm();
        let wasm_path = temp_dir.path().join("defi_contract.wasm");
        fs::write(&wasm_path, defi_wasm).unwrap();
        
        let output_dir = temp_dir.path().join("defi_output");
        
        let compiler = NeoCompiler::new()
            .with_debug(true)
            .with_source("defi_test".to_string())
            .with_output_dir(output_dir);
        
        let result = compiler.compile(&wasm_path);
        assert!(result.is_ok(), "Complex DeFi compilation should succeed");
        
        let compilation_result = result.unwrap();
        
        // Validate complex contract output
        let nef_content = fs::read(&compilation_result.nef_path).unwrap();
        assert!(nef_content.len() > 500, "Complex contract should have substantial script");
        
        let manifest_content = fs::read_to_string(&compilation_result.manifest_path).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
        
        let methods = manifest["abi"]["methods"].as_array().unwrap();
        assert!(methods.len() >= 5, "Complex contract should have multiple methods");
        
        // Validate debug information
        assert_eq!(manifest["extra"]["compiler"], "neo-compiler");
    }

    fn create_hello_world_wasm() -> Vec<u8> {
        // Create a WASM module that represents a hello world contract
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]; // Magic + version
        
        // Type section: function type () -> ()
        wasm.extend_from_slice(&[0x01, 0x04, 0x01, 0x60, 0x00, 0x00]);
        
        // Function section: 1 function
        wasm.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]);
        
        // Export section: export "main" function
        wasm.extend_from_slice(&[
            0x07, 0x08, 0x01, 0x04, 0x6D, 0x61, 0x69, 0x6E, 0x00, 0x00
        ]);
        
        // Code section: simple function body
        wasm.extend_from_slice(&[
            0x0A, 0x06, 0x01, 0x04, 0x00, 0x41, 0x00, 0x0B  // (func (i32.const 0) return)
        ]);
        
        wasm
    }

    fn create_nep17_wasm() -> Vec<u8> {
        // Create WASM representing NEP-17 token contract
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        
        // Type section: multiple function types
        wasm.extend_from_slice(&[
            0x01, 0x0C, 0x03,
            0x60, 0x00, 0x00,                    // () -> ()
            0x60, 0x00, 0x01, 0x7F,              // () -> i32
            0x60, 0x03, 0x7F, 0x7F, 0x7F, 0x01, 0x7F  // (i32, i32, i32) -> i32
        ]);
        
        // Function section: 6 functions (symbol, decimals, totalSupply, balanceOf, transfer, approve)
        wasm.extend_from_slice(&[0x03, 0x07, 0x06, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02]);
        
        // Export section: export NEP-17 methods
        wasm.extend_from_slice(&[
            0x07, 0x2E, 0x06,
            0x06, 0x73, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x00, 0x00,           // symbol
            0x08, 0x64, 0x65, 0x63, 0x69, 0x6D, 0x61, 0x6C, 0x73, 0x00, 0x01, // decimals
            0x0B, 0x74, 0x6F, 0x74, 0x61, 0x6C, 0x53, 0x75, 0x70, 0x70, 0x6C, 0x79, 0x00, 0x02, // totalSupply
            0x09, 0x62, 0x61, 0x6C, 0x61, 0x6E, 0x63, 0x65, 0x4F, 0x66, 0x00, 0x03, // balanceOf
            0x08, 0x74, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x65, 0x72, 0x00, 0x04,     // transfer
            0x07, 0x61, 0x70, 0x70, 0x72, 0x6F, 0x76, 0x65, 0x00, 0x05           // approve
        ]);
        
        // Code section: function implementations
        wasm.extend_from_slice(&[
            0x0A, 0x20, 0x06,
            0x04, 0x00, 0x41, 0x42, 0x0B,        // symbol: return 66
            0x04, 0x00, 0x41, 0x08, 0x0B,        // decimals: return 8
            0x04, 0x00, 0x41, 0x80, 0x80, 0x40, 0x0B, // totalSupply: return 1000000
            0x04, 0x00, 0x41, 0x64, 0x0B,        // balanceOf: return 100
            0x04, 0x00, 0x41, 0x01, 0x0B,        // transfer: return 1 (true)
            0x04, 0x00, 0x41, 0x01, 0x0B         // approve: return 1 (true)
        ]);
        
        wasm
    }

    fn create_complex_defi_wasm() -> Vec<u8> {
        // Create WASM for complex DeFi contract (simplified)
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        
        // Type section: various function signatures
        wasm.extend_from_slice(&[
            0x01, 0x15, 0x05,
            0x60, 0x00, 0x00,                              // () -> ()
            0x60, 0x01, 0x7F, 0x01, 0x7F,                  // (i32) -> i32
            0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F,            // (i32, i32) -> i32
            0x60, 0x03, 0x7F, 0x7F, 0x7F, 0x01, 0x7F,      // (i32, i32, i32) -> i32
            0x60, 0x04, 0x7F, 0x7F, 0x7F, 0x7F, 0x01, 0x7F // (i32, i32, i32, i32) -> i32
        ]);
        
        // Function section: 8 functions
        wasm.extend_from_slice(&[0x03, 0x09, 0x08, 0x00, 0x01, 0x02, 0x3, 0x04, 0x01, 0x02, 0x03]);
        
        // Export section: DeFi methods
        wasm.extend_from_slice(&[
            0x07, 0x48, 0x08,
            0x0A, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x61, 0x6C, 0x69, 0x7A, 0x65, 0x00, 0x00, // initialize
            0x0D, 0x61, 0x64, 0x64, 0x5F, 0x6C, 0x69, 0x71, 0x75, 0x69, 0x64, 0x69, 0x74, 0x79, 0x00, 0x01, // add_liquidity
            0x10, 0x72, 0x65, 0x6D, 0x6F, 0x76, 0x65, 0x5F, 0x6C, 0x69, 0x71, 0x75, 0x69, 0x64, 0x69, 0x74, 0x79, 0x00, 0x02, // remove_liquidity
            0x04, 0x73, 0x77, 0x61, 0x70, 0x00, 0x03,      // swap
            0x09, 0x67, 0x65, 0x74, 0x5F, 0x70, 0x72, 0x69, 0x63, 0x65, 0x00, 0x04, // get_price
            0x0B, 0x67, 0x65, 0x74, 0x5F, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x73, 0x00, 0x05, // get_reserves
            0x05, 0x70, 0x61, 0x75, 0x73, 0x65, 0x00, 0x06, // pause
            0x07, 0x75, 0x6E, 0x70, 0x61, 0x75, 0x73, 0x65, 0x00, 0x07 // unpause
        ]);
        
        // Code section: function implementations (simplified)
        wasm.extend_from_slice(&[
            0x0A, 0x28, 0x08,
            0x02, 0x00, 0x0B,           // initialize: empty
            0x04, 0x00, 0x41, 0x01, 0x0B, // add_liquidity: return 1
            0x04, 0x00, 0x41, 0x01, 0x0B, // remove_liquidity: return 1
            0x04, 0x00, 0x41, 0x64, 0x0B, // swap: return 100
            0x04, 0x00, 0x41, 0x80, 0x80, 0x40, 0x0B, // get_price: return 1000000
            0x06, 0x00, 0x41, 0x80, 0x40, 0x41, 0x80, 0x40, 0x0B, // get_reserves: return (1000, 1000)
            0x02, 0x00, 0x0B,           // pause: empty
            0x02, 0x00, 0x0B            // unpause: empty
        ]);
        
        wasm
    }

    #[test]
    fn test_batch_compilation() {
        // Test compiling multiple contracts in sequence
        let temp_dir = TempDir::new().unwrap();
        
        let contracts = vec![
            ("hello_world", create_hello_world_wasm()),
            ("nep17_token", create_nep17_wasm()),
            ("simple_storage", create_storage_wasm()),
        ];
        
        let mut compilation_results = Vec::new();
        
        for (name, wasm_content) in contracts {
            let wasm_path = temp_dir.path().join(format!("{}.wasm", name));
            fs::write(&wasm_path, wasm_content).unwrap();
            
            let output_dir = temp_dir.path().join(format!("{}_output", name));
            
            let compiler = NeoCompiler::new()
                .with_source(format!("{}_test", name))
                .with_output_dir(output_dir);
            
            let result = compiler.compile(&wasm_path);
            assert!(result.is_ok(), "Compilation of {} should succeed", name);
            
            compilation_results.push((name, result.unwrap()));
        }
        
        // Validate all compilations succeeded
        assert_eq!(compilation_results.len(), 3);
        
        for (name, result) in compilation_results {
            assert!(result.nef_path.exists(), "{} NEF should exist", name);
            assert!(result.manifest_path.exists(), "{} manifest should exist", name);
        }
    }

    fn create_storage_wasm() -> Vec<u8> {
        // Create WASM for storage contract
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        
        // Type section
        wasm.extend_from_slice(&[
            0x01, 0x0C, 0x03,
            0x60, 0x00, 0x00,                    // () -> ()
            0x60, 0x01, 0x7F, 0x00,              // (i32) -> ()
            0x60, 0x01, 0x7F, 0x01, 0x7F         // (i32) -> i32
        ]);
        
        // Function section: 4 functions
        wasm.extend_from_slice(&[0x03, 0x05, 0x04, 0x00, 0x01, 0x2, 0x02]);
        
        // Export section
        wasm.extend_from_slice(&[
            0x07, 0x20, 0x04,
            0x0A, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x61, 0x6C, 0x69, 0x7A, 0x65, 0x00, 0x00, // initialize
            0x05, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x00, 0x01,    // store
            0x04, 0x6C, 0x6F, 0x61, 0x64, 0x00, 0x02,          // load
            0x06, 0x64, 0x65, 0x6C, 0x65, 0x74, 0x65, 0x00, 0x03 // delete
        ]);
        
        // Code section
        wasm.extend_from_slice(&[
            0x0A, 0x14, 0x04,
            0x02, 0x00, 0x0B,           // initialize: empty
            0x02, 0x00, 0x0B,           // store: empty
            0x04, 0x00, 0x41, 0x2A, 0x0B, // load: return 42
            0x02, 0x00, 0x0B            // delete: empty
        ]);
        
        wasm
    }

    fn create_complex_defi_wasm() -> Vec<u8> {
        // Create WASM for complex DeFi contract (AMM + lending)
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        
        // More complex type section
        wasm.extend_from_slice(&[
            0x01, 0x18, 0x06,
            0x60, 0x00, 0x00,                              // () -> ()
            0x60, 0x01, 0x7F, 0x01, 0x7F,                  // (i32) -> i32
            0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F,            // (i32, i32) -> i32
            0x60, 0x03, 0x7F, 0x7F, 0x7F, 0x01, 0x7F,      // (i32, i32, i32) -> i32
            0x60, 0x04, 0x7F, 0x7F, 0x7F, 0x7F, 0x01, 0x7F, // (i32, i32, i32, i32) -> i32
            0x60, 0x02, 0x7F, 0x7F, 0x02, 0x7F, 0x7F       // (i32, i32) -> (i32, i32)
        ]);
        
        // Function section: 10 functions
        wasm.extend_from_slice(&[0x03, 0x0B, 0x0A, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04]);
        
        // Export section: DeFi methods
        wasm.extend_from_slice(&[
            0x07, 0x5A, 0x0A,
            0x0A, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x61, 0x6C, 0x69, 0x7A, 0x65, 0x00, 0x00,
            0x0D, 0x61, 0x64, 0x64, 0x5F, 0x6C, 0x69, 0x71, 0x75, 0x69, 0x64, 0x69, 0x74, 0x79, 0x00, 0x01,
            0x10, 0x72, 0x65, 0x6D, 0x6F, 0x76, 0x65, 0x5F, 0x6C, 0x69, 0x71, 0x75, 0x69, 0x64, 0x69, 0x74, 0x79, 0x00, 0x02,
            0x04, 0x73, 0x77, 0x61, 0x70, 0x00, 0x03,
            0x09, 0x67, 0x65, 0x74, 0x5F, 0x70, 0x72, 0x69, 0x63, 0x65, 0x00, 0x04,
            0x0B, 0x67, 0x65, 0x74, 0x5F, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x73, 0x00, 0x05,
            0x07, 0x64, 0x65, 0x70, 0x6F, 0x73, 0x69, 0x74, 0x00, 0x06,
            0x08, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x00, 0x07,
            0x06, 0x62, 0x6F, 0x72, 0x72, 0x6F, 0x77, 0x00, 0x08,
            0x05, 0x72, 0x65, 0x70, 0x61, 0x79, 0x00, 0x09
        ]);
        
        // Code section: function implementations
        wasm.extend_from_slice(&[
            0x0A, 0x38, 0x0A,
            0x02, 0x00, 0x0B,                    // initialize
            0x04, 0x00, 0x41, 0x01, 0x0B,        // add_liquidity
            0x04, 0x00, 0x41, 0x01, 0x0B,        // remove_liquidity
            0x04, 0x00, 0x41, 0x64, 0x0B,        // swap
            0x04, 0x00, 0x41, 0x80, 0x80, 0x40, 0x0B, // get_price
            0x06, 0x00, 0x41, 0x80, 0x40, 0x41, 0x80, 0x40, 0x0B, // get_reserves
            0x04, 0x00, 0x41, 0x01, 0x0B,        // deposit
            0x04, 0x00, 0x41, 0x01, 0x0B,        // withdraw
            0x04, 0x00, 0x41, 0x64, 0x0B,        // borrow
            0x04, 0x00, 0x41, 0x01, 0x0B         // repay
        ]);
        
        wasm
    }
}

/// Real-world deployment simulation tests
mod deployment_simulation_tests {
    use super::*;

    #[test]
    fn test_mainnet_deployment_simulation() {
        // Simulate mainnet deployment preparation
        let temp_dir = TempDir::new().unwrap();
        
        // Compile production contract
        let production_wasm = create_production_ready_wasm();
        let wasm_path = temp_dir.path().join("production.wasm");
        fs::write(&wasm_path, production_wasm).unwrap();
        
        let output_dir = temp_dir.path().join("mainnet_ready");
        
        let compiler = NeoCompiler::new()
            .with_source("mainnet_production".to_string())
            .with_output_dir(output_dir.clone());
        
        let result = compiler.compile(&wasm_path);
        assert!(result.is_ok(), "Production compilation should succeed");
        
        let compilation_result = result.unwrap();
        
        // Validate production readiness
        let nef_content = fs::read(&compilation_result.nef_path).unwrap();
        let manifest_content = fs::read_to_string(&compilation_result.manifest_path).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
        
        // Production validation checks
        assert!(nef_content.len() > 320, "Production NEF should have content");
        assert!(nef_content.len() < 1024 * 1024, "NEF should be under 1MB limit");
        
        // Manifest validation
        assert!(manifest["name"].is_string(), "Contract should have name");
        assert!(manifest["abi"]["methods"].is_array(), "Should have method definitions");
        assert!(manifest["permissions"].is_array(), "Should have permission definitions");
        
        // Security checks
        let permissions = manifest["permissions"].as_array().unwrap();
        let has_wildcard_permissions = permissions.iter()
            .any(|p| p["contract"] == "*" && p["methods"].as_array().unwrap().contains(&serde_json::Value::String("*".to_string())));
        
        // For production, might want to restrict permissions
        if has_wildcard_permissions {
            println!("Warning: Contract has wildcard permissions - review for production");
        }
    }

    #[test]
    fn test_testnet_deployment_simulation() {
        // Simulate testnet deployment and testing
        let temp_dir = TempDir::new().unwrap();
        
        let testnet_wasm = create_testnet_wasm();
        let wasm_path = temp_dir.path().join("testnet.wasm");
        fs::write(&wasm_path, testnet_wasm).unwrap();
        
        let output_dir = temp_dir.path().join("testnet_deploy");
        
        let compiler = NeoCompiler::new()
            .with_debug(true) // Enable debug for testnet
            .with_source("testnet_debug".to_string())
            .with_output_dir(output_dir);
        
        let result = compiler.compile(&wasm_path);
        assert!(result.is_ok(), "Testnet compilation should succeed");
        
        let compilation_result = result.unwrap();
        
        // Validate debug features are enabled
        let manifest_content = fs::read_to_string(&compilation_result.manifest_path).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content).unwrap();
        
        // Check for debug metadata
        if let Some(extra) = manifest["extra"].as_object() {
            assert!(extra.contains_key("compiler"), "Should have compiler info");
        }
    }

    fn create_production_ready_wasm() -> Vec<u8> {
        // Create WASM that represents a production-ready contract
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        
        // Complete type section for production contract
        wasm.extend_from_slice(&[
            0x01, 0x10, 0x04,
            0x60, 0x00, 0x00,                    // () -> ()
            0x60, 0x01, 0x7F, 0x01, 0x7F,        // (i32) -> i32
            0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F,  // (i32, i32) -> i32
            0x60, 0x03, 0x7F, 0x7F, 0x7F, 0x01, 0x7F // (i32, i32, i32) -> i32
        ]);
        
        // Function section: 6 production methods
        wasm.extend_from_slice(&[0x03, 0x07, 0x06, 0x00, 0x01, 0x01, 0x2, 0x03, 0x01]);
        
        // Export section: production methods
        wasm.extend_from_slice(&[
            0x07, 0x38, 0x06,
            0x0A, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x61, 0x6C, 0x69, 0x7A, 0x65, 0x00, 0x00,
            0x06, 0x73, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x00, 0x01,
            0x08, 0x64, 0x65, 0x63, 0x69, 0x6D, 0x61, 0x6C, 0x73, 0x00, 0x02,
            0x0B, 0x74, 0x6F, 0x74, 0x61, 0x6C, 0x53, 0x75, 0x70, 0x70, 0x6C, 0x79, 0x00, 0x03,
            0x08, 0x74, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x65, 0x72, 0x00, 0x04,
            0x05, 0x70, 0x61, 0x75, 0x73, 0x65, 0x00, 0x05
        ]);
        
        // Code section: optimized implementations
        wasm.extend_from_slice(&[
            0x0A, 0x24, 0x06,
            0x02, 0x00, 0x0B,                    // initialize
            0x06, 0x00, 0x41, 0x80, 0x80, 0x80, 0x80, 0x00, 0x0B, // symbol (returns large number)
            0x04, 0x00, 0x41, 0x08, 0x0B,        // decimals: 8
            0x06, 0x00, 0x41, 0x80, 0x80, 0x80, 0x80, 0x08, 0x0B, // totalSupply
            0x04, 0x00, 0x41, 0x01, 0x0B,        // transfer: true
            0x02, 0x00, 0x0B                     // pause
        ]);
        
        wasm
    }

    fn create_testnet_wasm() -> Vec<u8> {
        // Create WASM for testnet with debug features
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        
        // Type section
        wasm.extend_from_slice(&[0x01, 0x04, 0x01, 0x60, 0x00, 0x00]);
        
        // Function section
        wasm.extend_from_slice(&[0x03, 0x03, 0x02, 0x00, 0x00]);
        
        // Export section: debug methods
        wasm.extend_from_slice(&[
            0x07, 0x18, 0x02,
            0x08, 0x67, 0x65, 0x74, 0x5F, 0x69, 0x6E, 0x66, 0x6F, 0x00, 0x00,
            0x09, 0x64, 0x65, 0x62, 0x75, 0x67, 0x5F, 0x6C, 0x6F, 0x67, 0x00, 0x01
        ]);
        
        // Code section
        wasm.extend_from_slice(&[
            0x0A, 0x08, 0x02,
            0x02, 0x00, 0x0B,  // get_info
            0x02, 0x00, 0x0B   // debug_log
        ]);
        
        wasm
    }
}

/// CLI and tooling integration tests
mod tooling_integration_tests {
    use super::*;

    #[test]
    fn test_verify_command_integration() {
        // Test the verify command functionality
        let temp_dir = TempDir::new().unwrap();
        
        // Create and compile a contract
        let wasm_content = create_hello_world_wasm();
        let wasm_path = temp_dir.path().join("verify_test.wasm");
        fs::write(&wasm_path, wasm_content).unwrap();
        
        let output_dir = temp_dir.path().join("verify_output");
        
        let compiler = NeoCompiler::new().with_output_dir(output_dir);
        let compile_result = compiler.compile(&wasm_path).unwrap();
        
        // Now test verification
        let verify_result = compiler.verify_nef(&compile_result.nef_path);
        assert!(verify_result.is_ok(), "NEF verification should succeed");
        
        let verification_info = verify_result.unwrap();
        assert!(verification_info.checksum_valid, "Checksum should be valid");
        assert!(verification_info.script_size > 0, "Script should have content");
    }

    #[test]
    fn test_compile_all_command_simulation() {
        // Simulate the compile-all command
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple WASM files
        let contracts = vec![
            ("contract1", create_hello_world_wasm()),
            ("contract2", create_nep17_wasm()),
            ("contract3", create_storage_wasm()),
        ];
        
        for (name, wasm_content) in contracts {
            let wasm_path = temp_dir.path().join(format!("{}.wasm", name));
            fs::write(&wasm_path, wasm_content).unwrap();
        }
        
        // Simulate batch compilation
        let compiler = NeoCompiler::new()
            .with_output_dir(temp_dir.path().join("batch_output"));
        
        let wasm_files: Vec<PathBuf> = fs::read_dir(&temp_dir).unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "wasm" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        
        let mut successful_compilations = 0;
        
        for wasm_file in wasm_files {
            if let Ok(_) = compiler.compile(&wasm_file) {
                successful_compilations += 1;
            }
        }
        
        assert_eq!(successful_compilations, 3, "All contracts should compile successfully");
    }

    // Helper function from previous module
    fn create_hello_world_wasm() -> Vec<u8> {
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        wasm.extend_from_slice(&[0x01, 0x04, 0x01, 0x60, 0x00, 0x00]);
        wasm.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]);
        wasm.extend_from_slice(&[0x07, 0x08, 0x01, 0x04, 0x6D, 0x61, 0x69, 0x6E, 0x00, 0x00]);
        wasm.extend_from_slice(&[0x0A, 0x06, 0x01, 0x04, 0x00, 0x41, 0x00, 0x0B]);
        wasm
    }

    fn create_nep17_wasm() -> Vec<u8> {
        // Simplified version from previous implementation
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        wasm.extend_from_slice(&[0x01, 0x04, 0x01, 0x60, 0x00, 0x01, 0x7F]);
        wasm.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]);
        wasm.extend_from_slice(&[0x07, 0x0A, 0x01, 0x06, 0x73, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x00, 0x00]);
        wasm.extend_from_slice(&[0x0A, 0x06, 0x01, 0x04, 0x00, 0x41, 0x42, 0x0B]);
        wasm
    }

    fn create_storage_wasm() -> Vec<u8> {
        // Simplified storage contract
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        wasm.extend_from_slice(&[0x01, 0x04, 0x01, 0x60, 0x00, 0x00]);
        wasm.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]);
        wasm.extend_from_slice(&[0x07, 0x09, 0x01, 0x05, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x00, 0x00]);
        wasm.extend_from_slice(&[0x0A, 0x04, 0x01, 0x02, 0x00, 0x0B]);
        wasm
    }
}