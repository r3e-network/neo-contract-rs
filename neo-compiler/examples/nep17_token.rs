use neo_compiler::script::Script;
use neo_compiler::manifest::{
    ContractAbi, ContractEventDefinition, Manifest as ContractManifest, ContractMethodDefinition,
    ContractParameterDefinition,
};
use neo_compiler::neo::opcodes::OpCode;
use neo_compiler::error::Error;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let script = create_nep17_script()?;
    let manifest = create_nep17_manifest()?;

    // Save script and manifest to files
    // (Implementation would depend on neo_compiler's file output functions)

    Ok(())
}

fn create_nep17_script() -> Result<Script> {
    let mut script = Script::new();

    // Method dispatch logic - would check method name and jump to implementation
    script.emit_comment("Method selector");
    // Method dispatch implementation...

    // 'name' method implementation
    script.emit_comment("'name' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"name_method".to_vec());
    script.emit_push_data(b"Example Token").expect("Failed to emit token name");
    script.emit_opcode(OpCode::RET);

    // 'symbol' method implementation
    script.emit_comment("'symbol' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"symbol_method".to_vec());
    script.emit_push_data(b"EXT").expect("Failed to emit token symbol");
    script.emit_opcode(OpCode::RET);

    // 'decimals' method implementation
    script.emit_comment("'decimals' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"decimals_method".to_vec());
    script.emit_push_data(&[8]).expect("Failed to emit decimals");
    script.emit_opcode(OpCode::RET);

    // 'totalSupply' method implementation
    script.emit_comment("'totalSupply' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_method".to_vec());

    // Get total supply from storage
    let total_supply_key_bytes = b"totalSupply";
    script.emit_push_data(total_supply_key_bytes).expect("Failed to emit storage key");
    script.emit_with_operand(OpCode::SYSCALL, b"System.Storage.Get".to_vec());

    // Default total supply if not found
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"totalSupply_default").expect("Failed to emit jump label");

    // Return the stored total supply
    script.emit_opcode(OpCode::RET);

    // Default total supply
    script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_default".to_vec());
    let total_supply: i64 = 100_000_000 * 100_000_000; // 100M tokens with 8 decimal places
    let total_supply_bytes = total_supply.to_le_bytes();
    script.emit_push_data(&total_supply_bytes).expect("Failed to emit total supply");
    script.emit_opcode(OpCode::RET);

    // 'balanceOf' method implementation
    script.emit_comment("'balanceOf' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"balanceOf_method".to_vec());

    // Get account parameter
    script.emit_opcode(OpCode::LDARG1);

    // Create storage key: balance:{account}
    script.emit_push_data(b"balance:").expect("Failed to emit storage prefix");
    script.emit_opcode(OpCode::SWAP);
    script.emit_opcode(OpCode::CAT);

    // Get balance from storage
    script.emit_with_operand(OpCode::SYSCALL, b"System.Storage.Get".to_vec());

    // Return zero if nothing found
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_data(b"balance_zero").expect("Failed to emit jump label");

    // Convert stored value to integer
    script.emit_with_operand(OpCode::SYSCALL, b"System.Storage.GetInt".to_vec());
    script.emit_opcode(OpCode::RET);

    // Return zero balance
    script.emit_with_operand(OpCode::PUSHDATA1, b"balance_zero".to_vec());
    script.emit_push_data(&[0]).expect("Failed to emit zero");
    script.emit_opcode(OpCode::RET);

    // 'transfer' method implementation (simplified)
    script.emit_comment("'transfer' method implementation");
    script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_method".to_vec());

    // Get parameters
    script.emit_opcode(OpCode::LDARG1); // from
    script.emit_opcode(OpCode::LDARG2); // to
    script.emit_opcode(OpCode::LDARG3); // amount

    // Check amount > 0
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(&[0]).expect("Failed to emit zero for comparison");
    script.emit_opcode(OpCode::GT);
    script.emit_opcode(OpCode::JMPIFNOT);
    script.emit_push_data(b"transfer_fail").expect("Failed to emit jump label");

    // Verify witness
    script.emit_opcode(OpCode::PICK); // Equivalent to OVER2
    script.emit_push_data(&[2]).expect("Failed to emit pickup index");
    script.emit_with_operand(OpCode::SYSCALL, b"System.Runtime.CheckWitness".to_vec());
    script.emit_opcode(OpCode::JMPIFNOT);
    script.emit_push_data(b"transfer_fail").expect("Failed to emit jump label");

    // Transfer logic here... (simplified)
    // This would include checking balances, updating balances, etc.

    // Emit Transfer event
    script.emit_opcode(OpCode::OVER); // from
    script.emit_opcode(OpCode::SWAP);
    script.emit_opcode(OpCode::OVER); // to
    script.emit_opcode(OpCode::SWAP);
    script.emit_opcode(OpCode::OVER); // amount
    script.emit_with_operand(OpCode::SYSCALL, b"System.Runtime.Notify".to_vec());

    // Return success
    script.emit_push_data(&[1]).expect("Failed to emit true");
    script.emit_opcode(OpCode::RET);

    // Transfer fail label
    script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_fail".to_vec());
    script.emit_push_data(&[0]).expect("Failed to emit false");
    script.emit_opcode(OpCode::RET);

    Ok(script)
}

fn create_nep17_manifest() -> Result<ContractManifest> {
    let mut abi = ContractAbi::new();

    // Add methods
    let name_method = ContractMethodDefinition::new(
        "name".to_string(),
        vec![],
        "String".to_string(),
        true, // safe (read-only)
    );
    abi.add_method(name_method);

    let symbol_method = ContractMethodDefinition::new(
        "symbol".to_string(),
        vec![],
        "String".to_string(),
        true,
    );
    abi.add_method(symbol_method);

    let decimals_method = ContractMethodDefinition::new(
        "decimals".to_string(),
        vec![],
        "Integer".to_string(),
        true,
    );
    abi.add_method(decimals_method);

    let total_supply_method = ContractMethodDefinition::new(
        "totalSupply".to_string(),
        vec![],
        "Integer".to_string(),
        true,
    );
    abi.add_method(total_supply_method);

    let balance_of_method = ContractMethodDefinition::new(
        "balanceOf".to_string(),
        vec![ContractParameterDefinition::new(
            "account".to_string(),
            "Hash160".to_string(),
        )],
        "Integer".to_string(),
        true,
    );
    abi.add_method(balance_of_method);

    let transfer_method = ContractMethodDefinition::new(
        "transfer".to_string(),
        vec![
            ContractParameterDefinition::new("from".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("to".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("amount".to_string(), "Integer".to_string()),
        ],
        "Boolean".to_string(),
        false, // not safe (writes to storage)
    );
    abi.add_method(transfer_method);

    // Add Transfer event
    let transfer_event = ContractEventDefinition::new(
        "Transfer".to_string(),
        vec![
            ContractParameterDefinition::new("from".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("to".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("amount".to_string(), "Integer".to_string()),
        ],
    );
    abi.add_event(transfer_event);

    // Create manifest
    let mut manifest = ContractManifest::new("Neo Example Token");
    
    // Set features
    manifest.features.insert("storage".to_string(), true);
    manifest.features.insert("payable".to_string(), false);
    
    // Add supported standards
    manifest.supported_standards.push("NEP-17".to_string());
    
    // Set ABI
    let manifest = manifest.with_abi(abi);
    
    Ok(manifest)
}