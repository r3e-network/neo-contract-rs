//! Integration tests for Neo N3 Rust Smart Contract Framework
//! 
//! These tests verify that all components work together correctly

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;

    /// Test that all examples compile
    #[test]
    fn test_all_examples_compile() {
        let examples_dir = Path::new("examples");
        
        if !examples_dir.exists() {
            panic!("Examples directory not found");
        }

        let mut failed_examples = Vec::new();

        // Iterate through all example directories
        for entry in std::fs::read_dir(examples_dir).expect("Failed to read examples directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.is_dir() {
                let cargo_toml = path.join("Cargo.toml");
                if cargo_toml.exists() {
                    let example_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    
                    println!("Testing compilation of example: {}", example_name);
                    
                    // Run cargo check for the example
                    let output = Command::new("cargo")
                        .arg("check")
                        .arg("--target")
                        .arg("wasm32-unknown-unknown")
                        .current_dir(&path)
                        .output()
                        .expect("Failed to execute cargo check");
                    
                    if !output.status.success() {
                        failed_examples.push(example_name.to_string());
                        eprintln!("Failed to compile example: {}", example_name);
                        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                    }
                }
            }
        }

        if !failed_examples.is_empty() {
            panic!("The following examples failed to compile: {:?}", failed_examples);
        }
    }

    /// Test that the neo-compiler can generate NEF files
    #[test]
    fn test_nef_generation() {
        // Build a simple test WASM first
        let test_wasm = "target/wasm32-unknown-unknown/release/counter.wasm";
        
        // Check if WASM exists (from previous builds)
        if Path::new(test_wasm).exists() {
            // Try to compile to NEF
            let output = Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg("neo-compiler")
                .arg("--")
                .arg("compile")
                .arg(test_wasm)
                .arg("--output")
                .arg("build/test")
                .output()
                .expect("Failed to execute neo-compiler");
            
            if !output.status.success() {
                eprintln!("Failed to generate NEF");
                eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                panic!("NEF generation failed");
            }
            
            // Check if NEF file was created
            let nef_path = Path::new("build/test/counter.nef");
            assert!(nef_path.exists(), "NEF file was not created");
            
            // Verify the NEF
            let verify_output = Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg("neo-compiler")
                .arg("--")
                .arg("verify")
                .arg("build/test/counter.nef")
                .output()
                .expect("Failed to execute neo-compiler verify");
            
            assert!(verify_output.status.success(), "NEF verification failed");
        }
    }

    /// Test that Solana-style syntax compiles
    #[test]
    fn test_solana_style_syntax() {
        let solana_examples = [
            "examples/01-hello-world-solana-style-simple",
            "examples/01-hello-world-solana-style",
            "examples/04-nep17-token-solana-style",
        ];

        for example_path in &solana_examples {
            let path = Path::new(example_path);
            if path.exists() {
                println!("Testing Solana-style example: {}", example_path);
                
                let output = Command::new("cargo")
                    .arg("check")
                    .arg("--target")
                    .arg("wasm32-unknown-unknown")
                    .current_dir(path)
                    .output()
                    .expect("Failed to execute cargo check");
                
                assert!(
                    output.status.success(),
                    "Failed to compile Solana-style example: {}",
                    example_path
                );
            }
        }
    }

    /// Test that the deployment scripts exist and are executable
    #[test]
    fn test_deployment_infrastructure() {
        // Check deployment script
        let deploy_script = Path::new("scripts/deploy.sh");
        assert!(deploy_script.exists(), "Deployment script not found");
        
        // Check deployment documentation
        let deploy_docs = Path::new("docs/DEPLOYMENT.md");
        assert!(deploy_docs.exists(), "Deployment documentation not found");
        
        // Check Neo Express configuration
        let neo_express_config = Path::new("default.neo-express");
        assert!(neo_express_config.exists(), "Neo Express configuration not found");
        
        // Check Makefile
        let makefile = Path::new("Makefile");
        assert!(makefile.exists(), "Main Makefile not found");
    }

    /// Test that all required dependencies are available
    #[test]
    fn test_dependencies() {
        // Check that WASM target is installed
        let output = Command::new("rustup")
            .arg("target")
            .arg("list")
            .arg("--installed")
            .output()
            .expect("Failed to execute rustup");
        
        let installed_targets = String::from_utf8_lossy(&output.stdout);
        assert!(
            installed_targets.contains("wasm32-unknown-unknown"),
            "WASM target is not installed. Run: rustup target add wasm32-unknown-unknown"
        );
    }
}