//! Example of manually creating Neo scripts using the neo-compiler API.
//!
//! This example demonstrates how to use the API to:
//! 1. Create a Neo VM script directly without WebAssembly
//! 2. Write the script to a file
//! 3. Create a NEF file from the script

use neo_compiler::{nef::NefFile, script::Script, neo::OpCode};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;
    
    println!("Creating Neo script examples...");
    
    // Example 1: Simple add function
    let add_script = create_add_script();
    let add_path = output_dir.join("add.neo");
    add_script.write_to_file(&add_path)?;
    println!("  Created add script: {}", add_path.display());
    
    // Example 2: Storage example
    let storage_script = create_storage_script();
    let storage_path = output_dir.join("storage.neo");
    storage_script.write_to_file(&storage_path)?;
    println!("  Created storage script: {}", storage_path.display());
    
    // Example 3: Create NEF file from script
    let nef = NefFile::with_script(storage_script.to_bytes()?);
    let nef_path = output_dir.join("storage.nef");
    nef.save_to(&nef_path)?;
    println!("  Created NEF file: {}", nef_path.display());
    
    println!("All examples created successfully!");
    println!("Script files are available in: {}", output_dir.display());
    
    Ok(())
}

/// Creates a simple add function script.
///
/// This script:
/// 1. Takes two integers from the stack
/// 2. Adds them together
/// 3. Returns the result
fn create_add_script() -> Script {
    println!("\nCreating add script...");
    println!("  This script adds two integers and returns the result.");
    
    let mut script = Script::new();
    
    // Add comments for clarity in the script
    script.emit_with_operand(OpCode::COMMENT, b"Function: add(a, b)".to_vec());
    
    // Get the two parameters from the stack (already there)
    script.emit_with_operand(OpCode::COMMENT, b"Add the two parameters".to_vec());
    script.emit_opcode(OpCode::ADD);
    
    // Return the result
    script.emit_with_operand(OpCode::COMMENT, b"Return the result".to_vec());
    script.emit_opcode(OpCode::RET);
    
    script
}

/// Creates a script that demonstrates storage operations.
///
/// This script:
/// 1. Stores a value in storage
/// 2. Retrieves the value from storage
/// 3. Returns the retrieved value
fn create_storage_script() -> Script {
    println!("\nCreating storage script...");
    println!("  This script demonstrates storage operations in Neo.");
    
    let mut script = Script::new();
    
    // Initialize the script
    script.emit_with_operand(OpCode::COMMENT, b"Storage example".to_vec());
    
    // Store key-value: storage.put("name", "Neo")
    script.emit_with_operand(OpCode::COMMENT, b"Store key-value pair".to_vec());
    script.emit_push_data(b"name").unwrap();       // Key
    script.emit_push_data(b"Neo").unwrap();        // Value
    script.emit_opcode(OpCode::PACKMAP);           // Create map with one key-value pair
    script.emit_opcode(OpCode::PUTSTATIC0);        // Store in static slot 0
    
    // Get value from storage: storage.get("name")
    script.emit_with_operand(OpCode::COMMENT, b"Retrieve value from storage".to_vec());
    script.emit_push_data(b"name").unwrap();       // Key
    script.emit_opcode(OpCode::GETSTATIC0);        // Get from static slot 0
    
    // Return the retrieved value
    script.emit_with_operand(OpCode::COMMENT, b"Return the retrieved value".to_vec());
    script.emit_opcode(OpCode::RET);
    
    script
}

/// Creates a more complex contract with multiple functions.
fn create_contract_script() -> Script {
    println!("\nCreating contract script...");
    println!("  This script demonstrates a contract with multiple functions.");
    
    let mut script = Script::new();
    
    // Contract entrypoint - dispatch based on method name
    script.emit_with_operand(OpCode::COMMENT, b"Contract entrypoint".to_vec());
    script.emit_opcode(OpCode::LDARG0);            // Load the first argument (method name)
    
    // Check for "name" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'name' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"name").unwrap();
    script.emit_opcode(OpCode::EQUAL);
    
    // Jump to name implementation if matched
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_integer(100);                 // Jump to offset 100 (placeholder)
    
    // Check for "balanceOf" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'balanceOf' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"balanceOf").unwrap();
    script.emit_opcode(OpCode::EQUAL);
    
    // Jump to balanceOf implementation if matched
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_integer(200);                 // Jump to offset 200 (placeholder)
    
    // If no method matched, throw error
    script.emit_with_operand(OpCode::COMMENT, b"Method not found".to_vec());
    script.emit_push_data(b"Method not found").unwrap();
    script.emit_opcode(OpCode::THROW);
    
    // 'name' method implementation (would be at offset 100)
    script.emit_with_operand(OpCode::COMMENT, b"'name' method implementation".to_vec());
    script.emit_push_data(b"ExampleToken").unwrap();
    script.emit_opcode(OpCode::RET);
    
    // 'balanceOf' method implementation (would be at offset 200)
    script.emit_with_operand(OpCode::COMMENT, b"'balanceOf' method implementation".to_vec());
    script.emit_opcode(OpCode::LDARG1);            // Load account argument
    script.emit_push_integer(1000);                // Return fixed balance of 1000 for testing
    script.emit_opcode(OpCode::RET);
    
    script
} 