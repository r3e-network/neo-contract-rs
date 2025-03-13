#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::{TestBuilder, MockLedger, MockRuntime};

    // Helper function to set up a test environment with deployed contract
    fn setup_test() -> (TestBuilder, H160) {
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set owner account
        let owner = test.accounts()[0];
        
        // Deploy contract with owner parameter
        let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(owner)]);
        
        (test, contract_hash)
    }

    #[test]
    fn test_vested_amount_calculation() {
        let (mut test, contract_hash) = setup_test();
        
        // Set initial timestamp for reproducibility
        test.ledger().set_current_timestamp(1000);
        
        // Set up test accounts
        let owner = test.accounts()[0];
        let beneficiary = test.accounts()[1];
        
        // Create a vesting schedule with 1000 tokens over 100 seconds
        test.as_signer(owner).invoke(
            contract_hash,
            "create_vesting_schedule",
            &[
                RuntimeValue::from(beneficiary),
                RuntimeValue::from(1000u64),
                RuntimeValue::from(100u64)
            ]
        );
        
        // Test initial vested amount (should be 0)
        let vested: u64 = test.invoke(
            contract_hash,
            "vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(vested, 0, "Initial vested amount should be 0");
        
        // Advance time by 25 seconds (25% of vesting period)
        test.ledger().advance_time(25);
        
        // Test vested amount (should be 25% = 250 tokens)
        let vested: u64 = test.invoke(
            contract_hash,
            "vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(vested, 250, "Vested amount should be 25% after 25% of time");
        
        // Advance time by another 25 seconds (50% of vesting period)
        test.ledger().advance_time(25);
        
        // Test vested amount (should be 50% = 500 tokens)
        let vested: u64 = test.invoke(
            contract_hash,
            "vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(vested, 500, "Vested amount should be 50% after 50% of time");
        
        // Advance time to the end of vesting period
        test.ledger().advance_time(50);
        
        // Test vested amount (should be 100% = 1000 tokens)
        let vested: u64 = test.invoke(
            contract_hash,
            "vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(vested, 1000, "Vested amount should be 100% after full vesting period");
        
        // Advance time beyond vesting period
        test.ledger().advance_time(100);
        
        // Test vested amount (should still be 100% = 1000 tokens, not more)
        let vested: u64 = test.invoke(
            contract_hash,
            "vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(vested, 1000, "Vested amount should not exceed 100%");
    }
    
    #[test]
    fn test_claim_vested_tokens() {
        let (mut test, contract_hash) = setup_test();
        
        // Set initial timestamp
        test.ledger().set_current_timestamp(1000);
        
        // Set up test accounts
        let owner = test.accounts()[0];
        let beneficiary = test.accounts()[1];
        
        // Create a vesting schedule with 1000 tokens over 100 seconds
        test.as_signer(owner).invoke(
            contract_hash,
            "create_vesting_schedule",
            &[
                RuntimeValue::from(beneficiary),
                RuntimeValue::from(1000u64),
                RuntimeValue::from(100u64)
            ]
        );
        
        // Advance time by 50 seconds (50% of vesting period)
        test.ledger().advance_time(50);
        
        // Claim tokens as beneficiary
        let claimed: u64 = test.as_signer(beneficiary).invoke(
            contract_hash,
            "claim_vested_tokens",
            &[]
        );
        assert_eq!(claimed, 500, "Should claim 50% of tokens");
        
        // Verify that the claimable amount is now 0
        let claimable: u64 = test.invoke(
            contract_hash,
            "claimable_vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(claimable, 0, "Claimable amount should be 0 after claiming");
        
        // Advance time to the end of vesting period
        test.ledger().advance_time(50);
        
        // Check claimable amount again
        let claimable: u64 = test.invoke(
            contract_hash,
            "claimable_vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(claimable, 500, "Claimable amount should be remaining 50%");
        
        // Claim the remaining tokens
        let claimed: u64 = test.as_signer(beneficiary).invoke(
            contract_hash,
            "claim_vested_tokens",
            &[]
        );
        assert_eq!(claimed, 500, "Should claim remaining 50% of tokens");
        
        // Verify that no more tokens are claimable
        let claimable: u64 = test.invoke(
            contract_hash,
            "claimable_vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(claimable, 0, "No more tokens should be claimable");
    }
    
    #[test]
    fn test_block_rewards() {
        let (mut test, contract_hash) = setup_test();
        
        // Set initial block
        test.ledger().set_current_index(1000);
        
        // Set up test accounts
        let owner = test.accounts()[0];
        let user = test.accounts()[1];
        
        // Set reward per block to 10 tokens
        test.as_signer(owner).invoke(
            contract_hash,
            "set_reward_per_block",
            &[RuntimeValue::from(10u64)]
        );
        
        // Claim rewards initially (should be 0)
        let claimed: u64 = test.as_signer(user).invoke(
            contract_hash,
            "claim_block_rewards",
            &[]
        );
        assert_eq!(claimed, 0, "Initial claim should be 0");
        
        // Advance blocks
        test.ledger().advance_blocks(5);
        
        // Check claimable rewards
        let claimable: u64 = test.invoke(
            contract_hash,
            "get_claimable_block_rewards",
            &[RuntimeValue::from(user)]
        );
        assert_eq!(claimable, 50, "Claimable rewards should be 5 blocks * 10 tokens");
        
        // Claim rewards
        let claimed: u64 = test.as_signer(user).invoke(
            contract_hash,
            "claim_block_rewards",
            &[]
        );
        assert_eq!(claimed, 50, "Should claim 50 tokens");
        
        // Check rewards balance
        let balance: u64 = test.invoke(
            contract_hash,
            "get_rewards_balance",
            &[RuntimeValue::from(user)]
        );
        assert_eq!(balance, 50, "Balance should be 50 after claiming");
        
        // Advance more blocks
        test.ledger().advance_blocks(10);
        
        // Claim again
        let claimed: u64 = test.as_signer(user).invoke(
            contract_hash,
            "claim_block_rewards",
            &[]
        );
        assert_eq!(claimed, 100, "Should claim 10 blocks * 10 tokens");
        
        // Check updated balance
        let balance: u64 = test.invoke(
            contract_hash,
            "get_rewards_balance",
            &[RuntimeValue::from(user)]
        );
        assert_eq!(balance, 150, "Balance should be 150 after second claim");
    }
    
    #[test]
    fn test_transaction_processing() {
        let (mut test, contract_hash) = setup_test();
        
        // Set initial block
        test.ledger().set_current_index(1000);
        
        // Create mock transaction hash
        let tx_hash = H256::from_slice(&[1; 32]);
        
        // Mock the transaction height (5 blocks ago)
        test.ledger().mock_transaction_height(tx_hash.clone(), 995);
        
        // Check if the transaction has enough confirmations (require 6)
        let confirmed: bool = test.invoke(
            contract_hash,
            "verify_transaction_confirmations",
            &[
                RuntimeValue::from(tx_hash.clone()),
                RuntimeValue::from(6u32)
            ]
        );
        assert!(!confirmed, "Should not have enough confirmations yet");
        
        // Try to process transaction (should fail due to insufficient confirmations)
        let processed: bool = test.invoke(
            contract_hash,
            "process_transaction",
            &[
                RuntimeValue::from(tx_hash.clone()),
                RuntimeValue::from(6u32)
            ]
        );
        assert!(!processed, "Should not process due to insufficient confirmations");
        
        // Advance block to get 6 confirmations
        test.ledger().advance_blocks(1);
        
        // Check confirmation status again
        let confirmed: bool = test.invoke(
            contract_hash,
            "verify_transaction_confirmations",
            &[
                RuntimeValue::from(tx_hash.clone()),
                RuntimeValue::from(6u32)
            ]
        );
        assert!(confirmed, "Should have enough confirmations now");
        
        // Process transaction (should succeed)
        let processed: bool = test.invoke(
            contract_hash,
            "process_transaction",
            &[
                RuntimeValue::from(tx_hash.clone()),
                RuntimeValue::from(6u32)
            ]
        );
        assert!(processed, "Should process successfully");
        
        // Check that transaction is marked as processed
        let is_processed: bool = test.invoke(
            contract_hash,
            "is_transaction_processed",
            &[RuntimeValue::from(tx_hash.clone())]
        );
        assert!(is_processed, "Transaction should be marked as processed");
        
        // Try to process again (should fail since already processed)
        let processed: bool = test.invoke(
            contract_hash,
            "process_transaction",
            &[
                RuntimeValue::from(tx_hash.clone()),
                RuntimeValue::from(6u32)
            ]
        );
        assert!(!processed, "Should not process already processed transaction");
    }
    
    #[test]
    fn test_rate_limiting() {
        let (mut test, contract_hash) = setup_test();
        
        // Set initial timestamp
        test.ledger().set_current_timestamp(1000);
        
        // Set up test account
        let owner = test.accounts()[0];
        let user = test.accounts()[1];
        
        // Set cooldown to 1 hour (3600 seconds)
        test.as_signer(owner).invoke(
            contract_hash,
            "set_action_cooldown",
            &[RuntimeValue::from(3600u64)]
        );
        
        // Perform action as user (should succeed first time)
        let result: bool = test.as_signer(user).invoke(
            contract_hash,
            "perform_rate_limited_action",
            &[]
        );
        assert!(result, "First action should succeed");
        
        // Try to perform action again immediately (should fail due to cooldown)
        let result: bool = test.as_signer(user).invoke(
            contract_hash,
            "perform_rate_limited_action",
            &[]
        );
        assert!(!result, "Action should fail during cooldown period");
        
        // Check cooldown remaining (should be close to 3600 seconds)
        let cooldown: u64 = test.invoke(
            contract_hash,
            "get_cooldown_remaining",
            &[RuntimeValue::from(user)]
        );
        assert!(cooldown > 3500, "Cooldown should be close to 3600 seconds");
        
        // Advance time by 30 minutes (1800 seconds)
        test.ledger().advance_time(1800);
        
        // Check cooldown remaining (should be close to 1800 seconds)
        let cooldown: u64 = test.invoke(
            contract_hash,
            "get_cooldown_remaining",
            &[RuntimeValue::from(user)]
        );
        assert!(cooldown > 1700 && cooldown < 1900, "Cooldown should be close to 1800 seconds");
        
        // Try to perform action again (should still fail)
        let result: bool = test.as_signer(user).invoke(
            contract_hash,
            "perform_rate_limited_action",
            &[]
        );
        assert!(!result, "Action should still fail with half cooldown");
        
        // Advance time by another 30 minutes (1800 seconds)
        test.ledger().advance_time(1800);
        
        // Check cooldown remaining (should be 0)
        let cooldown: u64 = test.invoke(
            contract_hash,
            "get_cooldown_remaining",
            &[RuntimeValue::from(user)]
        );
        assert_eq!(cooldown, 0, "Cooldown should be 0 after full period");
        
        // Perform action again (should succeed now)
        let result: bool = test.as_signer(user).invoke(
            contract_hash,
            "perform_rate_limited_action",
            &[]
        );
        assert!(result, "Action should succeed after cooldown period");
    }
    
    #[test]
    fn test_blockchain_info_access() {
        let (mut test, contract_hash) = setup_test();
        
        // Set specific blockchain state
        test.ledger().set_current_index(12345);
        test.ledger().set_current_timestamp(1609459200); // Jan 1, 2021
        
        // Get blockchain info
        let (index, hash, timestamp): (u32, H256, u64) = test.invoke(
            contract_hash,
            "get_current_blockchain_info",
            &[]
        );
        
        // Verify the values
        assert_eq!(index, 12345, "Block index should match what was set");
        assert_eq!(timestamp, 1609459200, "Timestamp should match what was set");
        
        // We can't easily assert on the hash since it's implementation-dependent
        // But we can check that it's not empty
        assert!(hash != H256::from_slice(&[0; 32]), "Block hash should not be empty");
        
        // Test getting block info for a specific block
        // First, mock a block
        test.ledger().mock_block(10000, H256::from_slice(&[1; 32]), 1600000000);
        
        // Get the block info
        let block_info: Option<(u32, H256, u64)> = test.invoke(
            contract_hash,
            "get_block_info",
            &[RuntimeValue::from(10000u32)]
        );
        
        // Verify the block info
        assert!(block_info.is_some(), "Block info should be returned for mocked block");
        if let Some((index, hash, timestamp)) = block_info {
            assert_eq!(index, 10000, "Block index should match");
            assert_eq!(hash, H256::from_slice(&[1; 32]), "Block hash should match");
            assert_eq!(timestamp, 1600000000, "Block timestamp should match");
        }
    }
    
    #[test]
    fn test_owner_access_control() {
        let (mut test, contract_hash) = setup_test();
        
        // Set up test accounts
        let owner = test.accounts()[0];
        let non_owner = test.accounts()[1];
        
        // Owner should be able to set reward per block
        let result: bool = test.as_signer(owner).invoke(
            contract_hash,
            "set_reward_per_block",
            &[RuntimeValue::from(20u64)]
        );
        assert!(result, "Owner should be able to set reward per block");
        
        // Non-owner should not be able to set reward per block
        let result = test.as_signer(non_owner).invoke_expecting_error(
            contract_hash,
            "set_reward_per_block",
            &[RuntimeValue::from(30u64)]
        );
        assert!(result.contains("Only the owner can perform this action"), 
                "Non-owner should not be able to set reward per block");
        
        // Owner should be able to set action cooldown
        let result: bool = test.as_signer(owner).invoke(
            contract_hash,
            "set_action_cooldown",
            &[RuntimeValue::from(7200u64)]
        );
        assert!(result, "Owner should be able to set action cooldown");
        
        // Non-owner should not be able to set action cooldown
        let result = test.as_signer(non_owner).invoke_expecting_error(
            contract_hash,
            "set_action_cooldown",
            &[RuntimeValue::from(1800u64)]
        );
        assert!(result.contains("Only the owner can perform this action"), 
                "Non-owner should not be able to set action cooldown");
    }
    
    #[test]
    fn test_edge_case_vesting_with_zero_duration() {
        let (mut test, contract_hash) = setup_test();
        
        // Set initial timestamp
        test.ledger().set_current_timestamp(1000);
        
        // Set up test accounts
        let owner = test.accounts()[0];
        let beneficiary = test.accounts()[1];
        
        // Create a vesting schedule with zero duration (edge case)
        // In a real contract this might be prevented, but useful to test behavior
        test.as_signer(owner).invoke(
            contract_hash,
            "create_vesting_schedule",
            &[
                RuntimeValue::from(beneficiary),
                RuntimeValue::from(1000u64),
                RuntimeValue::from(0u64)
            ]
        );
        
        // Check vested amount (should be fully vested immediately due to zero duration)
        let vested: u64 = test.invoke(
            contract_hash,
            "vested_amount",
            &[RuntimeValue::from(beneficiary)]
        );
        assert_eq!(vested, 1000, "Amount should be fully vested with zero duration");
    }
} 