//! NEP-17 Token Script Example
//!
//! This example demonstrates how to manually create a Neo VM script for a
//! simplified NEP-17 token contract using the neo-compiler API.
//!
//! The NEP-17 standard is a token standard for the Neo blockchain, similar to ERC-20 for Ethereum.
//! This example implements a subset of the NEP-17 standard including:
//! - name, symbol, decimals, and totalSupply methods
//! - balanceOf method
//! - transfer method (simplified)

use neo_compiler::{
    script::Script,
    neo::OpCode,
    nef::NefFile,
    manifest::{Manifest, ContractParameterDefinition, ContractMethodDefinition, ContractAbi, ContractEventDefinition},
    error::Result,
};
use std::path::PathBuf;
use std::fs;

fn main() -> Result<()> {
    println!("Creating NEP-17 Token Contract Example");
    
    // Set up the output directory
    let output_dir = PathBuf::from("output");
    fs::create_dir_all(&output_dir)?;
    
    // Step 1: Create the token script
    println!("\nStep 1: Creating Neo VM script for NEP-17 token...");
    let script = create_nep17_script()?;
    let script_path = output_dir.join("nep17_token.neo");
    script.write_to_file(&script_path)?;
    println!("  Created script: {}", script_path.display());
    
    // Step 2: Create the NEF file
    println!("\nStep 2: Creating NEF file from script...");
    let mut nef = NefFile::with_script(script.to_bytes()?);
    nef.finalize()?;
    let nef_path = output_dir.join("nep17_token.nef");
    nef.save_to(&nef_path)?;
    println!("  Created NEF file: {}", nef_path.display());
    
    // Step 3: Create the contract manifest
    println!("\nStep 3: Creating contract manifest...");
    let manifest = create_nep17_manifest()?;
    let manifest_path = output_dir.join("nep17_token.manifest.json");
    manifest.save_to_file(&manifest_path)?;
    println!("  Created manifest: {}", manifest_path.display());
    
    println!("\nNEP-17 token contract files successfully created!");
    println!("Files are available in: {}", output_dir.display());
    
    Ok(())
}

/// Creates a Neo VM script for a simplified NEP-17 token contract.
fn create_nep17_script() -> Result<Script> {
    let mut script = Script::new();
    
    // Initialize contract (this would be called during deployment)
    script.emit_with_operand(OpCode::COMMENT, b"Contract initialization".to_vec());
    
    // Main contract entrypoint
    // Dispatches to the appropriate method based on the first argument
    script.emit_with_operand(OpCode::COMMENT, b"Main contract entrypoint".to_vec());
    script.emit_opcode(OpCode::LDARG0);  // Load method name
    
    // ------ Method dispatch logic ------
    
    // Check for "name" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'name' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"name")?;
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"name_method")?;  // Jump label
    
    // Check for "symbol" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'symbol' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"symbol")?;
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"symbol_method")?;  // Jump label
    
    // Check for "decimals" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'decimals' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"decimals")?;
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"decimals_method")?;  // Jump label
    
    // Check for "totalSupply" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'totalSupply' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"totalSupply")?;
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"totalSupply_method")?;  // Jump label
    
    // Check for "balanceOf" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'balanceOf' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"balanceOf")?;
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"balanceOf_method")?;  // Jump label
    
    // Check for "transfer" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'transfer' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"transfer")?;
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"transfer_method")?;  // Jump label
    
    // If no method matched, throw exception
    script.emit_with_operand(OpCode::COMMENT, b"Method not found - throw exception".to_vec());
    script.emit_push_data(b"Method not found")?;
    script.emit_opcode(OpCode::THROW);
    
    // ------ Method implementations ------
    
    // 'name' method implementation
    script.emit_with_operand(OpCode::COMMENT, b"'name' method implementation".to_vec());
    script.emit_with_operand(OpCode::PUSHDATA1, b"name_method".to_vec());  // Label
    script.emit_push_data(b"Neo Example Token")?;
    script.emit_opcode(OpCode::RET);
    
    // 'symbol' method implementation
    script.emit_with_operand(OpCode::COMMENT, b"'symbol' method implementation".to_vec());
    script.emit_with_operand(OpCode::PUSHDATA1, b"symbol_method".to_vec());  // Label
    script.emit_push_data(b"NET")?;
    script.emit_opcode(OpCode::RET);
    
    // 'decimals' method implementation
    script.emit_with_operand(OpCode::COMMENT, b"'decimals' method implementation".to_vec());
    script.emit_with_operand(OpCode::PUSHDATA1, b"decimals_method".to_vec());  // Label
    script.emit_push_integer(8);  // 8 decimals
    script.emit_opcode(OpCode::RET);
    
    // 'totalSupply' method implementation
    script.emit_with_operand(OpCode::COMMENT, b"'totalSupply' method implementation".to_vec());
    script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_method".to_vec());  // Label
    
    // Get total supply from storage
    script.emit_push_data(b"totalSupply")?;  // Storage key
    script.emit_with_operand(OpCode::SYSCALL, b"System.Storage.Get".to_vec());
    
    // If nothing in storage, return default total supply
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"totalSupply_default")?;  // Jump label
    
    // Return the stored total supply
    script.emit_opcode(OpCode::RET);
    
    // Default total supply
    script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_default".to_vec());  // Label
    script.emit_push_integer(100000000 * 100000000);  // 100M tokens with 8 decimals
    script.emit_opcode(OpCode::RET);
    
    // 'balanceOf' method implementation
    script.emit_with_operand(OpCode::COMMENT, b"'balanceOf' method implementation".to_vec());
    script.emit_with_operand(OpCode::PUSHDATA1, b"balanceOf_method".to_vec());  // Label
    
    // Get account parameter
    script.emit_opcode(OpCode::LDARG1);  // Load the account argument
    
    // Create storage key: account_balance_{account}
    script.emit_push_data(b"account_balance_")?;
    script.emit_opcode(OpCode::SWAP);
    script.emit_opcode(OpCode::CAT);
    
    // Get balance from storage
    script.emit_with_operand(OpCode::SYSCALL, b"System.Storage.Get".to_vec());
    
    // If nothing in storage, return 0
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"balance_zero")?;  // Jump label
    
    // Convert the stored value to an integer
    script.emit_with_operand(OpCode::SYSCALL, b"System.Storage.GetInt".to_vec());
    script.emit_opcode(OpCode::RET);
    
    // Return zero balance
    script.emit_with_operand(OpCode::PUSHDATA1, b"balance_zero".to_vec());  // Label
    script.emit_push_integer(0);
    script.emit_opcode(OpCode::RET);
    
    // 'transfer' method implementation (simplified)
    script.emit_with_operand(OpCode::COMMENT, b"'transfer' method implementation".to_vec());
    script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_method".to_vec());  // Label
    
    // Get parameters: from, to, amount
    script.emit_opcode(OpCode::LDARG1);  // from
    script.emit_opcode(OpCode::LDARG2);  // to
    script.emit_opcode(OpCode::LDARG3);  // amount
    
    // Check if amount > 0
    script.emit_opcode(OpCode::DUP);     // Duplicate amount
    script.emit_push_integer(0);
    script.emit_opcode(OpCode::GT);      // amount > 0?
    
    // If amount <= 0, return false
    script.emit_opcode(OpCode::NOT);     // Invert condition
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"transfer_fail")?;  // Jump label
    
    // Check if from has enough balance
    script.emit_with_operand(OpCode::COMMENT, b"Check if sender has enough balance".to_vec());
    
    // Skip for simplicity in this example...
    
    // Transfer success
    script.emit_with_operand(OpCode::COMMENT, b"Transfer success".to_vec());
    script.emit_push_integer(1);   // true
    script.emit_opcode(OpCode::RET);
    
    // Transfer fail
    script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_fail".to_vec());  // Label
    script.emit_push_integer(0);   // false
    script.emit_opcode(OpCode::RET);
    
    Ok(script)
}

/// Creates a manifest for a NEP-17 token contract.
fn create_nep17_manifest() -> Result<Manifest> {
    let mut manifest = Manifest::new("Neo Example Token");
    
    // Add methods to the ABI
    
    // "name" method
    let name_method = ContractMethodDefinition::new(
        "name",
        vec![],
        "String",
        true, // safe (read-only)
    );
    manifest.add_method(name_method);
    
    // "symbol" method
    let symbol_method = ContractMethodDefinition::new(
        "symbol",
        vec![],
        "String",
        true, // safe (read-only)
    );
    manifest.add_method(symbol_method);
    
    // "decimals" method
    let decimals_method = ContractMethodDefinition::new(
        "decimals",
        vec![],
        "Integer",
        true, // safe (read-only)
    );
    manifest.add_method(decimals_method);
    
    // "totalSupply" method
    let total_supply_method = ContractMethodDefinition::new(
        "totalSupply",
        vec![],
        "Integer",
        true, // safe (read-only)
    );
    manifest.add_method(total_supply_method);
    
    // "balanceOf" method
    let balance_of_method = ContractMethodDefinition::new(
        "balanceOf",
        vec![ContractParameterDefinition::new("account", "Hash160")],
        "Integer",
        true, // safe (read-only)
    );
    manifest.add_method(balance_of_method);
    
    // "transfer" method
    let transfer_method = ContractMethodDefinition::new(
        "transfer",
        vec![
            ContractParameterDefinition::new("from", "Hash160"),
            ContractParameterDefinition::new("to", "Hash160"),
            ContractParameterDefinition::new("amount", "Integer"),
        ],
        "Boolean",
        false, // not safe (modifies state)
    );
    manifest.add_method(transfer_method);
    
    // Add events to the ABI
    
    // "Transfer" event
    let transfer_event = ContractEventDefinition::new(
        "Transfer",
        vec![
            ContractParameterDefinition::new("from", "Hash160"),
            ContractParameterDefinition::new("to", "Hash160"),
            ContractParameterDefinition::new("amount", "Integer"),
        ],
    );
    manifest.add_event(transfer_event);
    
    // Set features
    manifest.set_feature("storage", true)?; // Uses storage
    manifest.set_feature("payable", false)?; // Not payable
    
    // Add supported standards
    manifest.add_supported_standard("NEP-17");
    
    // Add extra information
    manifest.set_extra("description", "Example NEP-17 token for demonstration purposes")?;
    manifest.set_extra("author", "Neo Blockchain")?;
    
    // Validate the manifest
    manifest.validate()?;
    
    Ok(manifest)
}

/// Helper function to emit a "Transfer" event.
fn emit_transfer_event(script: &mut Script, from: &[u8], to: &[u8], amount: i64) -> Result<()> {
    script.emit_with_operand(OpCode::COMMENT, b"Emit Transfer event".to_vec());
    
    // Push event arguments
    script.emit_push_data(from)?;        // from
    script.emit_push_data(to)?;          // to
    script.emit_push_integer(amount);    // amount
    
    // Pack arguments into an array
    script.emit_push_integer(3);         // 3 arguments
    script.emit_opcode(OpCode::PACK);
    
    // Push event name
    script.emit_push_data(b"Transfer")?;
    
    // Emit the event
    script.emit_with_operand(OpCode::SYSCALL, b"System.Runtime.Notify".to_vec());
    
    Ok(())
} 