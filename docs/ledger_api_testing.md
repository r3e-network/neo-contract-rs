# Testing Smart Contracts with Ledger API

This guide focuses on testing Neo N3 smart contracts that interact with blockchain data through the Ledger API. Testing such contracts requires special considerations since blockchain state (blocks, transactions, timestamps) needs to be properly mocked for deterministic and reliable tests.

## Table of Contents

- [Introduction](#introduction)
- [Challenges in Testing Ledger API](#challenges-in-testing-ledger-api)
- [Test Environment Setup](#test-environment-setup)
- [Mocking Blockchain State](#mocking-blockchain-state)
  - [Mocking Current Block](#mocking-current-block)
  - [Mocking Timestamps](#mocking-timestamps)
  - [Mocking Transactions](#mocking-transactions)
- [Testing Time-Dependent Logic](#testing-time-dependent-logic)
- [Testing Block-Based Logic](#testing-block-based-logic)
- [Testing Transaction Validation](#testing-transaction-validation)
- [Integration with Neo Test Framework](#integration-with-neo-test-framework)
- [Best Practices](#best-practices)
- [Example Test Implementations](#example-test-implementations)

## Introduction

Smart contracts that use the Ledger API depend on blockchain state, which makes testing them challenging. You need to:

1. Mock blockchain data to simulate real-world conditions
2. Test time-dependent logic under various scenarios
3. Verify contracts behave correctly with different block heights and transaction confirmations

This guide demonstrates strategies for effectively testing such contracts.

## Challenges in Testing Ledger API

Smart contracts using the Ledger API present unique testing challenges:

1. **State Dependency**: Contract behavior depends on blockchain state that's difficult to control in tests
2. **Time Simulation**: Testing time-based logic requires advancing timestamps
3. **Transaction Validation**: Testing confirmation-dependent logic requires simulating blockchain confirmations
4. **Determinism**: Tests must be deterministic despite blockchain's inherently changing state

## Test Environment Setup

To test contracts using the Ledger API, you'll need a testing framework that allows mocking blockchain state:

```rust
use neo_contract::prelude::*;
use neo_contract_testing::{TestBuilder, MockLedger, MockRuntime};

#[test]
fn setup_test_environment() {
    // Create a test builder with mock ledger
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Deploy contract with arguments
    let contract_hash = test.deploy("path/to/contract.nef", &[RuntimeValue::from("arg1")]);
    
    // Now you can interact with the contract in a controlled test environment
    let result: bool = test.invoke(contract_hash, "some_method", &[]);
    assert!(result);
}
```

## Mocking Blockchain State

### Mocking Current Block

To test logic that depends on block height:

```rust
#[test]
fn test_block_dependent_logic() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial block
    test.ledger().set_current_index(1000);
    
    // Deploy and invoke contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Check contract behavior at current block
    let result: u64 = test.invoke(
        contract_hash,
        "get_claimable_block_rewards",
        &[RuntimeValue::from(test.accounts()[0])]
    );
    assert_eq!(result, 0); // No rewards initially
    
    // Advance blocks 
    test.ledger().advance_blocks(10);
    
    // Check contract behavior after blocks have advanced
    let result: u64 = test.invoke(
        contract_hash,
        "get_claimable_block_rewards",
        &[RuntimeValue::from(test.accounts()[0])]
    );
    assert_eq!(result, 100); // 10 blocks * 10 tokens per block
}
```

### Mocking Timestamps

To test time-dependent logic:

```rust
#[test]
fn test_time_dependent_logic() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial timestamp (seconds since Unix epoch)
    test.ledger().set_current_timestamp(1609459200); // Jan 1, 2021
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Create a vesting schedule (30 days)
    let beneficiary = test.accounts()[1];
    test.invoke(
        contract_hash,
        "create_vesting_schedule",
        &[
            RuntimeValue::from(beneficiary),
            RuntimeValue::from(1000u64),
            RuntimeValue::from(2592000u64) // 30 days in seconds
        ]
    );
    
    // Check initial vested amount (should be 0)
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 0);
    
    // Advance time by 15 days
    test.ledger().advance_time(1296000); // 15 days in seconds
    
    // Check vested amount (should be ~50%)
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 500); // 50% of 1000
    
    // Advance time to the end of vesting
    test.ledger().advance_time(1296000); // Another 15 days
    
    // Check final vested amount (should be 100%)
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 1000); // 100% of 1000
}
```

### Mocking Transactions

To test transaction confirmation logic:

```rust
#[test]
fn test_transaction_confirmation() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial block
    test.ledger().set_current_index(1000);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Create a mock transaction hash
    let tx_hash = H256::from_slice(&[1; 32]);
    
    // Mock the transaction's block height (included in block 995)
    test.ledger().mock_transaction_height(tx_hash.clone(), 995);
    
    // Check confirmation status (should have 5 confirmations)
    let is_confirmed: bool = test.invoke(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(5u32)
        ]
    );
    assert!(is_confirmed); // 5 >= 5 confirmations
    
    // Check with higher confirmation requirement
    let is_confirmed: bool = test.invoke(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(6u32)
        ]
    );
    assert!(!is_confirmed); // 5 < 6 confirmations
    
    // Advance blocks
    test.ledger().advance_blocks(1);
    
    // Check confirmation status again
    let is_confirmed: bool = test.invoke(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(6u32)
        ]
    );
    assert!(is_confirmed); // 6 >= 6 confirmations
}
```

## Testing Time-Dependent Logic

For more complex time-dependent scenarios:

```rust
#[test]
fn test_time_lock_functionality() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial timestamp
    test.ledger().set_current_timestamp(1000);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Setup a rate limiter for an action
    test.invoke(
        contract_hash,
        "set_action_cooldown",
        &[RuntimeValue::from(3600u64)] // 1 hour cooldown
    );
    
    // Perform the action (should succeed first time)
    let result: bool = test.invoke(
        contract_hash,
        "perform_rate_limited_action",
        &[]
    );
    assert!(result);
    
    // Try to perform the action again immediately (should fail)
    let result: bool = test.invoke(
        contract_hash,
        "perform_rate_limited_action",
        &[]
    );
    assert!(!result);
    
    // Advance time by 30 minutes
    test.ledger().advance_time(1800);
    
    // Try again (should still fail)
    let result: bool = test.invoke(
        contract_hash,
        "perform_rate_limited_action",
        &[]
    );
    assert!(!result);
    
    // Advance time another 30 minutes (total 1 hour)
    test.ledger().advance_time(1800);
    
    // Try again (should succeed now)
    let result: bool = test.invoke(
        contract_hash,
        "perform_rate_limited_action",
        &[]
    );
    assert!(result);
}
```

## Testing Block-Based Logic

For complex block-based scenarios:

```rust
#[test]
fn test_block_rewards_claiming() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial block
    test.ledger().set_current_index(1000);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Set rewards per block
    test.invoke(
        contract_hash,
        "set_reward_per_block",
        &[RuntimeValue::from(10u64)]
    );
    
    // Check initial claimable rewards
    let rewards: u64 = test.invoke(
        contract_hash,
        "get_claimable_block_rewards",
        &[RuntimeValue::from(test.accounts()[0])]
    );
    assert_eq!(rewards, 0);
    
    // Advance blocks
    test.ledger().advance_blocks(10);
    
    // Check claimable rewards after advancing
    let rewards: u64 = test.invoke(
        contract_hash,
        "get_claimable_block_rewards",
        &[RuntimeValue::from(test.accounts()[0])]
    );
    assert_eq!(rewards, 100); // 10 blocks * 10 tokens
    
    // Claim rewards
    let claimed: u64 = test.invoke(
        contract_hash,
        "claim_block_rewards",
        &[]
    );
    assert_eq!(claimed, 100);
    
    // Check balance after claiming
    let balance: u64 = test.invoke(
        contract_hash,
        "get_rewards_balance",
        &[RuntimeValue::from(test.accounts()[0])]
    );
    assert_eq!(balance, 100);
    
    // Check claimable rewards again (should be 0 after claiming)
    let rewards: u64 = test.invoke(
        contract_hash,
        "get_claimable_block_rewards",
        &[RuntimeValue::from(test.accounts()[0])]
    );
    assert_eq!(rewards, 0);
}
```

## Testing Transaction Validation

For complex transaction validation scenarios:

```rust
#[test]
fn test_transaction_processing() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial block
    test.ledger().set_current_index(1000);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Create a mock transaction hash
    let tx_hash = H256::from_slice(&[1; 32]);
    
    // Mock the transaction's block height
    test.ledger().mock_transaction_height(tx_hash.clone(), 995);
    
    // Try to process with 6 confirmations required
    let processed: bool = test.invoke(
        contract_hash,
        "process_transaction",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(6u32)
        ]
    );
    assert!(!processed); // Only 5 confirmations available
    
    // Advance blocks to get more confirmations
    test.ledger().advance_blocks(1);
    
    // Try to process again
    let processed: bool = test.invoke(
        contract_hash,
        "process_transaction",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(6u32)
        ]
    );
    assert!(processed); // Now we have 6 confirmations
    
    // Check if transaction is marked as processed
    let is_processed: bool = test.invoke(
        contract_hash,
        "is_transaction_processed",
        &[RuntimeValue::from(tx_hash.clone())]
    );
    assert!(is_processed);
    
    // Try to process again (should fail because already processed)
    let processed: bool = test.invoke(
        contract_hash,
        "process_transaction",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(6u32)
        ]
    );
    assert!(!processed);
}
```

## Integration with Neo Test Framework

For more complex scenarios, you can integrate with the Neo test framework:

```rust
#[test]
fn test_with_neo_framework() {
    // Configure a private test network
    let mut chain = TestChain::new();
    
    // Create wallets for testing
    let alice = chain.create_wallet();
    let bob = chain.create_wallet();
    
    // Deploy contract
    let contract_hash = chain.deploy_contract("ledger_example.nef", alice.account());
    
    // Set up initial blockchain state
    chain.run_to_block(100);
    
    // Create a real transaction
    let tx_hash = chain.transfer_neo(alice.account(), bob.account(), 5);
    
    // Process the transaction with the contract
    let result = chain.invoke_function(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(tx_hash),
            RuntimeValue::from(1u32)
        ],
        alice.account()
    );
    
    assert!(result.success());
    assert_eq!(result.result::<bool>(), true);
}
```

## Best Practices

When testing contracts that use the Ledger API:

1. **Mock Comprehensively**: Ensure all ledger-related calls are properly mocked
2. **Test Time Boundaries**: Test at the boundaries of time-based logic (just before, exactly at, and just after)
3. **Test Confirmation Thresholds**: Test with confirmations below, at, and above required thresholds
4. **Cover Edge Cases**: Test with zero and maximum values where applicable
5. **Regression Tests**: Create tests for any bugs found to prevent regressions
6. **Simulate Network Conditions**: Test with block reorganizations and varying block times
7. **Isolate Tests**: Each test should run in isolation, with its own mocked blockchain state

## Example Test Implementations

### Testing Vesting Calculation Edge Cases

```rust
#[test]
fn test_vesting_calculation_edge_cases() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial timestamp
    let start_time = 1609459200; // Jan 1, 2021
    test.ledger().set_current_timestamp(start_time);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Create a vesting schedule with very small values to test rounding
    let beneficiary = test.accounts()[1];
    let total_amount = 10;
    let duration = 3; // 3 seconds duration
    
    test.invoke(
        contract_hash,
        "create_vesting_schedule",
        &[
            RuntimeValue::from(beneficiary),
            RuntimeValue::from(total_amount),
            RuntimeValue::from(duration)
        ]
    );
    
    // Test exactly at start time
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 0);
    
    // Test after 1 second (should vest approximately 1/3)
    test.ledger().advance_time(1);
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 3); // Rounded to 3 out of 10
    
    // Test after 2 seconds (should vest approximately 2/3)
    test.ledger().advance_time(1);
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 6); // Rounded to 6 out of 10
    
    // Test at exactly the end time
    test.ledger().advance_time(1);
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 10); // Fully vested
    
    // Test past the end time
    test.ledger().advance_time(100);
    let vested: u64 = test.invoke(
        contract_hash,
        "vested_amount",
        &[RuntimeValue::from(beneficiary)]
    );
    assert_eq!(vested, 10); // Still fully vested, doesn't go beyond
}
```

### Testing Transaction Height Edge Cases

```rust
#[test]
fn test_transaction_height_edge_cases() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial block to a very high number
    let initial_block = u32::MAX - 10;
    test.ledger().set_current_index(initial_block);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Create a mock transaction hash
    let tx_hash = H256::from_slice(&[1; 32]);
    
    // Mock the transaction's block height to be very close to current
    test.ledger().mock_transaction_height(tx_hash.clone(), initial_block - 1);
    
    // Verify transaction confirmations (should have 1)
    let confirmations = test.invoke(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(1u32)
        ]
    );
    assert!(confirmations);
    
    // Test with non-existent transaction
    let nonexistent_tx = H256::from_slice(&[2; 32]);
    let confirmations = test.invoke(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(nonexistent_tx),
            RuntimeValue::from(1u32)
        ]
    );
    assert!(!confirmations);
    
    // Test with same-block transaction (0 confirmations)
    test.ledger().mock_transaction_height(tx_hash.clone(), initial_block);
    let confirmations = test.invoke(
        contract_hash,
        "verify_transaction_confirmations",
        &[
            RuntimeValue::from(tx_hash.clone()),
            RuntimeValue::from(1u32)
        ]
    );
    assert!(!confirmations);
}
```

### Testing Block Progression Limits

```rust
#[test]
fn test_block_progression_limits() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Set initial block to near maximum
    let initial_block = u32::MAX - 5;
    test.ledger().set_current_index(initial_block);
    
    // Deploy contract
    let contract_hash = test.deploy("ledger_example.nef", &[RuntimeValue::from(test.accounts()[0])]);
    
    // Create test account
    let account = test.accounts()[1];
    
    // Initialize last claimed block to current block
    test.as_signer(test.accounts()[0]).invoke(
        contract_hash,
        "set_reward_per_block",
        &[RuntimeValue::from(10u64)]
    );
    
    // Claim rewards at initial block (should be 0)
    let rewards = test.as_signer(account).invoke(
        contract_hash,
        "claim_block_rewards",
        &[]
    );
    assert_eq!(rewards, 0);
    
    // Advance by max possible blocks before overflow
    test.ledger().advance_blocks(5);
    
    // Check rewards
    let rewards = test.invoke(
        contract_hash,
        "get_claimable_block_rewards",
        &[RuntimeValue::from(account)]
    );
    assert_eq!(rewards, 50); // 5 blocks * 10 tokens
    
    // Try to advance beyond u32::MAX (should wrap safely)
    test.ledger().advance_blocks(1);
    
    // Check behavior at block limit
    let current_info = test.invoke(
        contract_hash,
        "get_current_blockchain_info",
        &[]
    );
    // Now we need to check if the implementation properly handles the wraparound
    // The specific assertion would depend on how the test framework handles this edge case
}
```

By implementing these test strategies, you can ensure your contracts that use the Ledger API behave correctly under various blockchain conditions, improving their reliability and security. 