//! Example NEP-17 contract test
//! 
//! This example demonstrates how to test a NEP-17 token contract.

use neo_contract_testing::prelude::*;

/// Mock NEP-17 token transfer function
/// 
/// In a real test, this would call your actual contract code.
fn mock_nep17_transfer(args: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
    // Parse arguments
    if args.len() != 3 {
        return Err("Expected 3 arguments: from, to, amount".into());
    }
    
    let from = &args[0];
    let to = &args[1];
    let amount = u64::from_le_bytes(args[2].clone().try_into().unwrap_or([0; 8]));
    
    // Check witness
    if !MockRuntime::check_witness(from) {
        return Err("No authorization".into());
    }
    
    // Construct storage keys
    let from_key = [b"balance:".to_vec(), from.clone()].concat();
    let to_key = [b"balance:".to_vec(), to.clone()].concat();
    
    // Get current balances
    let from_balance = MockStorage::get(&from_key)
        .map(|v| u64::from_le_bytes(v.try_into().unwrap_or([0; 8])))
        .unwrap_or(0);
        
    let to_balance = MockStorage::get(&to_key)
        .map(|v| u64::from_le_bytes(v.try_into().unwrap_or([0; 8])))
        .unwrap_or(0);
    
    // Check sufficient balance
    if from_balance < amount {
        return Err("Insufficient balance".into());
    }
    
    // Update balances
    let new_from_balance = from_balance - amount;
    let new_to_balance = to_balance.checked_add(amount)
        .ok_or("Overflow")?;
    
    // Store new balances
    MockStorage::put(&from_key, &new_from_balance.to_le_bytes());
    MockStorage::put(&to_key, &new_to_balance.to_le_bytes());
    
    // Emit transfer event
    MockRuntime::notify(
        "Transfer",
        vec![
            from.clone(),
            to.clone(),
            amount.to_le_bytes().to_vec(),
        ],
    );
    
    // Return success
    Ok(vec![1]) // Return true
}

fn main() {
    // This is just an example, no need to run anything in main
    println!("Run this example with 'cargo test --example nep17_test'");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to create an account
    fn create_account(id: u8) -> Vec<u8> {
        let mut account = vec![0; 20]; // Hash160 size
        account[0] = id;
        account
    }
    
    // Helper function to set account balance
    fn set_balance(account: &[u8], amount: u64) {
        let key = [b"balance:".to_vec(), account.to_vec()].concat();
        MockStorage::put(&key, &amount.to_le_bytes());
    }
    
    // Helper function to get account balance
    fn get_balance(account: &[u8]) -> u64 {
        let key = [b"balance:".to_vec(), account.to_vec()].concat();
        MockStorage::get(&key)
            .map(|v| u64::from_le_bytes(v.try_into().unwrap_or([0; 8])))
            .unwrap_or(0)
    }
    
    #[test]
    fn test_transfer_success() {
        // Create test accounts
        let account1 = create_account(1);
        let account2 = create_account(2);
        
        // Create a test fixture
        let fixture = TestFixture::new();
        
        // Set up initial balances
        set_balance(&account1, 1000);
        set_balance(&account2, 500);
        
        // Authorize account1 for the transfer
        MockRuntime::add_witness(&account1);
        
        // Transfer 200 tokens from account1 to account2
        let result = fixture.run(|| {
            let args = vec![
                account1.clone(),
                account2.clone(),
                200u64.to_le_bytes().to_vec(),
            ];
            mock_nep17_transfer(args)
        });
        
        // Assert success
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1]);
        
        // Assert balances were updated
        assert_eq!(get_balance(&account1), 800);
        assert_eq!(get_balance(&account2), 700);
        
        // Assert event was emitted
        assert!(MockRuntime::verify_notification("Transfer"));
    }
    
    #[test]
    fn test_transfer_unauthorized() {
        // Create test accounts
        let account1 = create_account(1);
        let account2 = create_account(2);
        
        // Create a test fixture
        let fixture = TestFixture::new();
        
        // Set up initial balances
        set_balance(&account1, 1000);
        set_balance(&account2, 500);
        
        // Do NOT authorize account1 for the transfer
        // MockRuntime::add_witness(&account1);
        
        // Attempt to transfer 200 tokens from account1 to account2
        let result = fixture.run(|| {
            let args = vec![
                account1.clone(),
                account2.clone(),
                200u64.to_le_bytes().to_vec(),
            ];
            mock_nep17_transfer(args)
        });
        
        // Assert failure
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No authorization");
        
        // Assert balances were not updated
        assert_eq!(get_balance(&account1), 1000);
        assert_eq!(get_balance(&account2), 500);
    }
    
    #[test]
    fn test_transfer_insufficient_balance() {
        // Create test accounts
        let account1 = create_account(1);
        let account2 = create_account(2);
        
        // Create a test fixture
        let fixture = TestFixture::new();
        
        // Set up initial balances
        set_balance(&account1, 100); // Only 100 tokens
        set_balance(&account2, 500);
        
        // Authorize account1 for the transfer
        MockRuntime::add_witness(&account1);
        
        // Attempt to transfer 200 tokens from account1 to account2
        let result = fixture.run(|| {
            let args = vec![
                account1.clone(),
                account2.clone(),
                200u64.to_le_bytes().to_vec(), // Trying to transfer 200
            ];
            mock_nep17_transfer(args)
        });
        
        // Assert failure
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance");
        
        // Assert balances were not updated
        assert_eq!(get_balance(&account1), 100);
        assert_eq!(get_balance(&account2), 500);
    }
    
    #[test]
    fn test_transfer_using_simulator() {
        // Create test accounts
        let account1 = create_account(1);
        let account2 = create_account(2);
        
        // Create a simulator with debug enabled
        let simulator = ContractSimulator::new()
            .with_capture_debug(true, TracingMode::Full);
        
        // Set up initial balances
        set_balance(&account1, 1000);
        set_balance(&account2, 500);
        
        // Authorize account1 for the transfer
        MockRuntime::add_witness(&account1);
        
        // Create arguments
        let args = vec![
            account1.clone(),
            account2.clone(),
            200u64.to_le_bytes().to_vec(),
        ];
        
        // Invoke contract
        let result = simulator.invoke(mock_nep17_transfer, args);
        
        // Assert success
        assert!(result.success);
        assert_eq!(result.result, Some(vec![1]));
        
        // Check events
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].name, "Transfer");
        
        // Check storage changes
        assert_eq!(result.storage_changes.len(), 2);
        
        // Verify final balances
        assert_eq!(get_balance(&account1), 800);
        assert_eq!(get_balance(&account2), 700);
        
        // Print debug report
        println!("{}", simulator.debug_report());
    }
}