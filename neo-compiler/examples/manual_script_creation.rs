//! Example of manually creating Neo scripts using the neo-compiler API.
//!
//! This example demonstrates how to use the API to:
//! 1. Create a Neo VM script directly without WebAssembly
//! 2. Write the script to a file
//! 3. Create a NEF file from the script

use neo_compiler::{
    nef::NefFile,
    neo::OpCode,
    script::{save_script, Script},
};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    println!("Creating Neo script examples...");

    // Example 1: Simple add function
    let add_script = create_add_script();
    let add_path = output_dir.join("add.neo");
    save_script(&add_script, &add_path)?;
    println!("  Created add script: {}", add_path.display());

    // Example 2: Storage example
    let storage_script = create_storage_script();
    let storage_path = output_dir.join("storage.neo");
    save_script(&storage_script, &storage_path)?;
    println!("  Created storage script: {}", storage_path.display());

    // Example 3: Create NEF file from script
    let nef = NefFile::with_script(storage_script.bytes().to_vec());
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

    // Add comments for clarity in the script - won't be included in Neo N3 bytecode
    script.emit_comment("Function: add(a, b)");

    // Get the two parameters from the stack (already there)
    script.emit_comment("Add the two parameters");
    script.emit_opcode(OpCode::ADD);

    // Return the result
    script.emit_comment("Return the result");
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

    // Initialize the script - comments won't be included in Neo N3 bytecode
    script.emit_comment("Storage example");

    // Store key-value: storage.put("name", "Neo")
    script.emit_comment("Store key-value pair");
    script.emit_push_data(b"name").unwrap(); // Key
    script.emit_push_data(b"Neo").unwrap(); // Value
    script.emit_opcode(OpCode::PACKMAP); // Create map with one key-value pair
    script.emit_opcode(OpCode::STSFLD0); // Store in static slot 0

    // Get value from storage: storage.get("name")
    script.emit_comment("Retrieve value from storage");
    script.emit_push_data(b"name").unwrap(); // Key
    script.emit_opcode(OpCode::LDSFLD0); // Get from static slot 0

    // Return the retrieved value
    script.emit_comment("Return the retrieved value");
    script.emit_opcode(OpCode::RET);

    script
}

/// Creates a more complex contract with multiple functions.
fn create_contract_script() -> Result<Script, neo_compiler::error::Error> {
    println!("\nCreating contract script...");
    println!("  This script demonstrates a contract with multiple functions.");

    let mut script = Script::new();

    // Contract entrypoint - dispatch based on method name
    script.emit_comment("Contract entrypoint");
    script.emit_opcode(OpCode::LDARG0); // Load the first argument (method name)

    // Check for "name" method
    script.emit_comment("Check for 'name' method");
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"name")?;
    script.emit_opcode(OpCode::EQUAL);

    // Jump to name implementation if matched
    script.emit_opcode(OpCode::JMPIF);
    // Convert integer to byte array for offset
    script.emit_push_data(&100u32.to_le_bytes())?; // Jump to offset 100 (placeholder)

    // Check for "balanceOf" method
    script.emit_comment("Check for 'balanceOf' method");
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"balanceOf")?;
    script.emit_opcode(OpCode::EQUAL);

    // Jump to balanceOf implementation if matched
    script.emit_opcode(OpCode::JMPIF);
    // Convert integer to byte array for offset
    script.emit_push_data(&200u32.to_le_bytes())?; // Jump to offset 200 (placeholder)

    // If no method matched, throw error
    script.emit_comment("Method not found");
    script.emit_push_data(b"Method not found")?;
    script.emit_opcode(OpCode::THROW);

    // 'name' method implementation (would be at offset 100)
    script.emit_comment("'name' method implementation");
    script.emit_push_data(b"ExampleToken")?;
    script.emit_opcode(OpCode::RET);

    // 'balanceOf' method implementation (would be at offset 200)
    script.emit_comment("'balanceOf' method implementation");
    script.emit_opcode(OpCode::LDARG1); // Load account argument
    // Convert integer to byte array for balance
    script.emit_push_data(&1000u32.to_le_bytes())?; // Return fixed balance of 1000 for testing
    script.emit_opcode(OpCode::RET);

    Ok(script)
}
