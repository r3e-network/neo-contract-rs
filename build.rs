use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=examples/");
    
    // Check if we're building for release
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    
    if profile == "release" {
        // Only compile examples in release mode
        compile_examples();
    }
}

fn compile_examples() {
    let examples_dir = PathBuf::from("examples");
    let build_dir = PathBuf::from("build");
    
    // Create build directory if it doesn't exist
    fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    
    // Find all example directories
    if let Ok(entries) = fs::read_dir(&examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let example_name = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                
                // Skip non-example directories
                if !example_name.starts_with(|c: char| c.is_ascii_digit()) {
                    continue;
                }
                
                println!("Building example: {}", example_name);
                
                // Build the WASM file
                if build_wasm_for_example(&path, example_name).is_ok() {
                    // Compile to NEF if WASM exists
                    let wasm_path = path
                        .join("target/wasm32-unknown-unknown/release")
                        .join(format!("{}.wasm", example_name.replace('-', "_")));
                    
                    if wasm_path.exists() {
                        compile_to_nef(&wasm_path, &build_dir, example_name);
                    }
                }
            }
        }
    }
}

fn build_wasm_for_example(example_path: &Path, example_name: &str) -> Result<(), String> {
    // Set the required RUSTFLAGS
    let rustflags = "-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152";
    
    let output = Command::new("cargo")
        .current_dir(example_path)
        .env("RUSTFLAGS", rustflags)
        .args(&[
            "build",
            "--target", "wasm32-unknown-unknown",
            "--release",
            "--quiet"
        ])
        .output()
        .map_err(|e| format!("Failed to execute cargo: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Failed to build {}: {}", example_name, stderr);
        return Err(format!("Build failed for {}", example_name));
    }
    
    Ok(())
}

fn compile_to_nef(wasm_path: &Path, build_dir: &Path, example_name: &str) {
    // Check if neo-compiler is built
    let compiler_path = PathBuf::from("target/release/neo-compiler");
    if !compiler_path.exists() {
        // Try debug build
        let debug_compiler = PathBuf::from("target/debug/neo-compiler");
        if !debug_compiler.exists() {
            println!("Neo compiler not found, skipping NEF compilation for {}", example_name);
            return;
        }
    }
    
    // Run the neo-compiler
    let output = Command::new(if compiler_path.exists() { &compiler_path } else { &PathBuf::from("target/debug/neo-compiler") })
        .args(&[
            "compile",
            wasm_path.to_str().unwrap(),
            "--output", build_dir.to_str().unwrap(),
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("Successfully compiled {} to NEF", example_name);
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: Failed to compile {} to NEF: {}", example_name, stderr);
        }
        Err(e) => {
            eprintln!("Warning: Failed to run neo-compiler for {}: {}", example_name, e);
        }
    }
}