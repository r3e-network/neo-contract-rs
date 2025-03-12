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
    manifest::{Manifest, ContractParameterDefinition, ContractMethodDefinition, ContractEventDefinition},
    error::Result,
};
use std::path::PathBuf;
use std::fs;

fn main() -> Result<()> {
    println!("Creating NEP-17 Token Contract Example");
    
    // Set up the output directory
    let output_dir = PathBuf::from("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Step 1: Create the token script
    println!("\nStep 1: Creating Neo VM script for NEP-17 token...");
    let script = create_nep17_script().expect("Failed to create NEP-17 script");
    let script_path = output_dir.join("nep17_token.neo");
    fs::write(&script_path, script.to_bytes()).expect("Failed to write script to file");
    println!("  Created script: {}", script_path.display());
    
    // Step 2: Create the NEF file
    println!("\nStep 2: Creating NEF file from script...");
    // Get script bytes - Vec<u8> doesn't implement Try trait, so we use expect() instead of ?
    let script_bytes = script.to_bytes();
    let mut nef = NefFile::with_script(script_bytes);
    nef.finalize().expect("Failed to finalize NEF file");
    let nef_path = output_dir.join("nep17_token.nef");
    nef.save_to(&nef_path).expect("Failed to save NEF file");
    println!("  Created NEF file: {}", nef_path.display());
    
    // Step 3: Create the contract manifest
    println!("\nStep 3: Creating contract manifest...");
    let manifest = create_nep17_manifest().expect("Failed to create NEP-17 manifest");
    let manifest_path = output_dir.join("nep17_token.manifest.json");
    manifest.save_to_file(&manifest_path).expect("Failed to save manifest to file");
    println!("  Created manifest: {}", manifest_path.display());
    
    println!("\nNEP-17 token contract files successfully created!");
    println!("Files are available in: {}", output_dir.display());
    
    Ok(())
}

/// Creates a Neo VM script for a simplified NEP-17 token contract.
fn create_nep17_script() -> Result<Script> {
    let mut script = Script::new();
    
    // Initialize contract (this would be called during deployment)
    script.emit_comment("Contract initialization");
    
    // Main contract entrypoint
    // Dispatches to the appropriate method based on the first argument
    script.emit_comment("Main contract entrypoint");
    script.emit_opcode(OpCode::LDARG0);  // Load method name
    
    // ------ Method dispatch logic ------
    
    // Check for "name" method
    script.emit_comment("Check for 'name' method");
    script.emit_opcode(OpCode::DUP);
    let name_bytes = b"name";
    script.emit_push_data(name_bytes).expect("Failed to emit push data");
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    let name_method_bytes = b"name_method";
    script.emit_push_data(name_method_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Check for "symbol" method
    script.emit_comment("Check for 'symbol' method");
    script.emit_opcode(OpCode::DUP);
    let symbol_bytes = b"symbol";
    script.emit_push_data(symbol_bytes).expect("Failed to emit push data");
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    let symbol_method_bytes = b"symbol_method";
    script.emit_push_data(symbol_method_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Check for "decimals" method
    script.emit_comment("Check for 'decimals' method");
    script.emit_opcode(OpCode::DUP);
    let decimals_bytes = b"decimals";
    script.emit_push_data(decimals_bytes).expect("Failed to emit push data");
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    let decimals_method_bytes = b"decimals_method";
    script.emit_push_data(decimals_method_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Check for "totalSupply" method
    script.emit_comment("Check for 'totalSupply' method");
    script.emit_opcode(OpCode::DUP);
    let total_supply_bytes = b"totalSupply";
    script.emit_push_data(total_supply_bytes).expect("Failed to emit totalSupply");
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    let total_supply_method_bytes = b"totalSupply_method";
    script.emit_push_data(total_supply_method_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Check for "balanceOf" method
    script.emit_comment("Check for 'balanceOf' method");
    script.emit_opcode(OpCode::DUP);
    let balance_of_bytes = b"balanceOf";
    script.emit_push_data(balance_of_bytes).expect("Failed to emit push data");
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    let balance_of_method_bytes = b"balanceOf_method";
    script.emit_push_data(balance_of_method_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Check for "transfer" method
    script.emit_comment("Check for 'transfer' method");
    script.emit_opcode(OpCode::DUP);
    let transfer_bytes = b"transfer";
    script.emit_push_data(transfer_bytes).expect("Failed to emit push data");
    script.emit_opcode(OpCode::EQUAL);
    script.emit_opcode(OpCode::JMPIF);
    let transfer_method_bytes = b"transfer_method";
    script.emit_push_data(transfer_method_bytes).expect("Failed to emit jump label");  // Jump label
    
    // If no method matched, throw exception
    script.emit_comment("Method not found - throw exception");
    let method_not_found_bytes = b"Method not found";
    script.emit_push_data(method_not_found_bytes).expect("Failed to emit error message");
    script.emit_opcode(OpCode::THROW);
    
    // ------ Method implementations ------
    
    // 'name' method implementation
    script.emit_comment("'name' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"name_method".to_vec());  // Label
    let token_name_bytes = b"Neo Example Token";
    script.emit_push_data(token_name_bytes).expect("Failed to emit token name");
    script.emit_opcode(OpCode::RET);
    
    // 'symbol' method implementation
    script.emit_comment("'symbol' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"symbol_method".to_vec());  // Label
    let token_symbol_bytes = b"NET";
    script.emit_push_data(token_symbol_bytes).expect("Failed to emit token symbol");
    script.emit_opcode(OpCode::RET);
    
    // 'decimals' method implementation
    script.emit_comment("'decimals' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"decimals_method".to_vec());  // Label
    // Use a byte array for the 8 decimals value
    // Use a byte array for the 8 decimals value
    script.emit_push_data(&[8]).expect("Failed to emit decimals");  // 8 decimals
    script.emit_opcode(OpCode::RET);
    
    // 'totalSupply' method implementation
    script.emit_comment("'totalSupply' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_method".to_vec());  // Label
    
    // Get total supply from storage
    let total_supply_key_bytes = b"totalSupply";
    script.emit_push_data(total_supply_key_bytes).expect("Failed to emit storage key");  // Storage key
    let storage_get_syscall = b"System.Storage.Get".to_vec();
    script.emit_with_operand(OpCode::SYSCALL, storage_get_syscall).expect("Failed to emit storage get syscall");
    
    // If nothing in storage, return default total supply
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    // Convert integer to byte array for Neo VM compatibility
    let total_supply_default_bytes = b"totalSupply_default";
    script.emit_push_data(total_supply_default_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Return the stored total supply
    script.emit_opcode(OpCode::RET);
    
    // Default total supply
    script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_default".to_vec());  // Label
    // Convert integer to bytes for Neo VM
    let total_supply = 100000000 * 100000000; // 100M tokens with 8 decimals
    let total_supply_bytes = total_supply.to_le_bytes();
    script.emit_push_data(&total_supply_bytes).expect("Failed to emit total supply");
    script.emit_opcode(OpCode::RET);
    
    // 'balanceOf' method implementation
    script.emit_comment("'balanceOf' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"balanceOf_method".to_vec());  // Label
    
    // Get account parameter
    script.emit_opcode(OpCode::LDARG1);  // Load the account argument as byte array
    
    // Create storage key: account_balance_{account}
    let account_balance_prefix = b"account_balance_";
    script.emit_push_data(account_balance_prefix).expect("Failed to emit storage prefix");
    script.emit_opcode(OpCode::SWAP);
    script.emit_opcode(OpCode::CAT);
    
    // Get balance from storage
    let storage_get_syscall = b"System.Storage.Get".to_vec();
    script.emit_with_operand(OpCode::SYSCALL, storage_get_syscall).expect("Failed to emit storage get syscall");
    
    // If nothing in storage, return 0
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    // Convert integer to byte array for Neo VM compatibility
    // This is important as Neo VM expects byte arrays for storage operations
    let balance_zero_bytes = b"balance_zero";
    script.emit_push_data(balance_zero_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Convert the stored value to an integer
    let storage_get_int_syscall = b"System.Storage.GetInt".to_vec();
    script.emit_with_operand(OpCode::SYSCALL, storage_get_int_syscall).expect("Failed to emit storage get int syscall");
    script.emit_opcode(OpCode::RET);
    
    // Return zero balance
    script.emit_with_operand(OpCode::PUSHDATA1, b"balance_zero".to_vec());  // Label
    // Use a byte array for numeric values
    script.emit_push_data(&[0]).expect("Failed to emit zero");
    script.emit_opcode(OpCode::RET);
    
    // 'transfer' method implementation (simplified)
    script.emit_comment("'transfer' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_method".to_vec());  // Label
    
    // Get parameters: from, to, amount
    script.emit_opcode(OpCode::LDARG1);  // from
    script.emit_opcode(OpCode::LDARG2);  // to
    script.emit_opcode(OpCode::LDARG3);  // amount
    
    // Check if amount > 0
    script.emit_opcode(OpCode::DUP);     // Duplicate amount
    // Use a byte array for numeric values in comparison
    script.emit_push_data(&[0]).expect("Failed to emit zero for comparison");
    script.emit_opcode(OpCode::GT);      // amount > 0?
    
    // If amount <= 0, return false
    script.emit_opcode(OpCode::NOT);     // Invert condition
    script.emit_opcode(OpCode::JMPIF);
    let transfer_fail_bytes = b"transfer_fail";
    script.emit_push_data(transfer_fail_bytes).expect("Failed to emit jump label");  // Jump label
    
    // Check if from has enough balance
    script.emit_comment("Check if sender has enough balance");
    
    // Skip for simplicity in this example...
    
    // Emit Transfer event before returning success
    script.emit_comment("Emit Transfer event before returning success");
    
    // Get a copy of the parameters for the event
    script.emit_opcode(OpCode::LDARG1);  // from address
    script.emit_opcode(OpCode::LDARG2);  // to address
    script.emit_opcode(OpCode::LDARG3);  // amount
    
    // Create a byte array reference for from address
    let from_addr = b"sender_address"; // This would normally be a real address
    let to_addr = b"receiver_address"; // This would normally be a real address
    
    // Call the helper function to emit the Transfer event
    // Use correct byte array for amount
    let amount: i64 = 1000;
    let script_ref = &mut *script; // Create a mutable reference to the script
    emit_transfer_event(script_ref, from_addr, to_addr, amount).expect("Failed to emit transfer event");
    
    // Return success
    // Use a byte array for boolean true (1)
    script.emit_push_data(&[1]).expect("Failed to emit true");   // true
    script.emit_opcode(OpCode::RET);
    
    // Transfer fail
    script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_fail".to_vec());  // Label
    // Use a byte array for numeric values
    script.emit_push_data(&[0]).expect("Failed to emit zero");   // false
    script.emit_opcode(OpCode::RET);
    
    Ok(script)
}

/// Creates a manifest for a NEP-17 token contract.
fn create_nep17_manifest() -> Result<Manifest> {
    let mut manifest = Manifest::new("Neo Example Token")?;
    
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
    manifest.set_feature("storage", true).expect("Failed to set storage feature"); // Uses storage
    manifest.set_feature("payable", false).expect("Failed to set payable feature"); // Not payable
    
    // Add supported standards
    manifest.add_supported_standard("NEP-17");
    
    // Add extra information
    manifest.set_extra("description", "Example NEP-17 token for demonstration purposes").expect("Failed to set description");
    manifest.set_extra("author", "Neo Blockchain").expect("Failed to set author");
    
    // Validate the manifest
    manifest.validate().expect("Failed to validate manifest");
    
    Ok(manifest)
}

/// Helper function to emit a "Transfer" event.
/// This demonstrates proper handling of Neo N3 events within the smart contract.
/// Following the Neo N3 standard pattern for event emission.
fn emit_transfer_event(script: &mut Script, from: &[u8], to: &[u8], amount: i64) -> Result<()> {
    script.emit_comment("Emit Transfer event");
    
    // Create an array of event parameters (from, to, amount)
    // First push all parameters that will go into the array
    script.emit_push_data(from).expect("Failed to emit from address");        // from address
    script.emit_push_data(to).expect("Failed to emit to address");          // to address
    
    // Convert i64 amount to bytes for Neo VM compatibility
    let amount_bytes = amount.to_le_bytes();
    script.emit_push_data(&amount_bytes).expect("Failed to emit amount");    // amount
    
    // Pack the parameters into an array
    // Neo N3 events require array parameters for Runtime.Notify
    script.emit_push_data(&[3]).expect("Failed to emit number of arguments");  // 3 arguments in array
    script.emit_opcode(OpCode::PACK);
    
    // Push event name as ByteString
    let transfer_event_name = b"Transfer";
    script.emit_push_data(transfer_event_name).expect("Failed to emit event name");
    
    // Emit the event using System.Runtime.Notify
    let notify_syscall = b"System.Runtime.Notify".to_vec();
    script.emit_with_operand(OpCode::SYSCALL, notify_syscall).expect("Failed to emit notification syscall");
    
    Ok(())
} 