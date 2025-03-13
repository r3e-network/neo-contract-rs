# Transaction Patterns for Neo N3 Smart Contracts

This guide covers common transaction patterns and best practices for working with transactions in Neo N3 smart contracts. Understanding how to properly validate, process, and respond to transactions is essential for building secure and reliable smart contracts.

## Table of Contents

- [Introduction](#introduction)
- [Transaction Structure in Neo N3](#transaction-structure-in-neo-n3)
- [Common Transaction Patterns](#common-transaction-patterns)
  - [Transaction Confirmation Validation](#transaction-confirmation-validation)
  - [Rate Limiting](#rate-limiting)
  - [Multi-step Transactions](#multi-step-transactions)
  - [Transaction Batching](#transaction-batching)
  - [Hash-based Transaction Tracking](#hash-based-transaction-tracking)
- [Working with Transaction Signers](#working-with-transaction-signers)
- [Transaction Events and Notifications](#transaction-events-and-notifications)
- [Security Considerations](#security-considerations)
- [Examples](#examples)
- [Conclusion](#conclusion)

## Introduction

Blockchain transactions form the foundation of all smart contract operations. In Neo N3, understanding how to work with transactions is essential for implementing secure business logic, validating operations, and controlling access to contract functions.

This guide covers patterns for transaction validation, tracking, and processing, with practical examples using the Neo Contract Rust framework.

## Transaction Structure in Neo N3

Before diving into patterns, let's understand the key properties of a Neo N3 transaction:

```rust
pub struct Transaction {
    /// Hash of the transaction
    pub hash: H256,
    /// Version of the transaction
    pub version: u8,
    /// Nonce of the transaction
    pub nonce: u32,
    /// Sender of the transaction
    pub sender: H160,
    /// System fee of the transaction
    pub system_fee: Int256,
    /// Network fee of the transaction
    pub network_fee: Int256,
    /// Valid until block of the transaction
    pub valid_until_block: u32,
    /// Script of the transaction
    pub script: ByteString,
}
```

These properties can be accessed using the `Ledger` API in Neo contracts:

```rust
use neo_contract::prelude::*;

#[method]
pub fn process_tx_info(tx_hash: H256) {
    if let Some(tx) = Ledger::get_transaction(tx_hash) {
        let sender = tx.sender;
        let nonce = tx.nonce;
        let valid_until = tx.valid_until_block;
        // Process transaction information
    }
}
```

## Common Transaction Patterns

### Transaction Confirmation Validation

A common requirement is to verify that a transaction has reached a certain number of confirmations:

```rust
#[method]
pub fn is_transaction_confirmed(tx_hash: H256, required_confirmations: u32) -> bool {
    // Get the transaction height
    let tx_height = Ledger::get_transaction_height(tx_hash);
    
    // If transaction height is 0, it means the transaction isn't found or confirmed
    if tx_height == 0 {
        return false;
    }
    
    // Get the current block height
    let current_height = Ledger::current_index();
    
    // Calculate confirmations
    let confirmations = current_height - tx_height + 1;
    
    // Check if we have enough confirmations
    confirmations >= required_confirmations
}
```

This pattern is useful for operations that should only proceed after a transaction has been sufficiently confirmed on the blockchain, reducing the risk of chain reorganizations affecting contract state.

### Rate Limiting

Limiting how frequently a user can perform certain actions is important for preventing abuse. Here's a pattern using transaction timestamps:

```rust
const MIN_TIME_BETWEEN_ACTIONS: u64 = 3600; // 1 hour in seconds

#[method]
pub fn rate_limited_action() -> bool {
    let sender = Runtime::current_sender();
    let key = format!("last_action:{}", sender);
    
    // Get the last action timestamp
    let last_action_time: u64 = Storage::get(&key).unwrap_or(0);
    let current_time = Ledger::current_timestamp();
    
    // Check if enough time has passed
    if current_time - last_action_time < MIN_TIME_BETWEEN_ACTIONS {
        // Too soon, deny the action
        return false;
    }
    
    // Update the last action time
    Storage::put(&key, current_time);
    
    // Perform the action
    true
}
```

You can also implement block-based rate limiting by using `Ledger::current_index()` instead of timestamps.

### Multi-step Transactions

Some operations require multiple steps to complete. This pattern uses transaction hashes to track progress:

```rust
enum TxStatus {
    Started = 1,
    Committed = 2,
    Completed = 3,
}

#[method]
pub fn start_complex_operation() -> H256 {
    let tx_hash = Runtime::get_entry_script_hash();
    let sender = Runtime::current_sender();
    
    // Store the transaction status
    let status_key = format!("tx_status:{}", tx_hash);
    Storage::put(&status_key, TxStatus::Started as u8);
    
    // Store the transaction owner
    let owner_key = format!("tx_owner:{}", tx_hash);
    Storage::put(&owner_key, sender);
    
    // Return the transaction hash for future reference
    tx_hash
}

#[method]
pub fn commit_operation(tx_hash: H256) -> bool {
    let sender = Runtime::current_sender();
    
    // Check ownership
    let owner_key = format!("tx_owner:{}", tx_hash);
    let owner: H160 = Storage::get(&owner_key).expect("Transaction not found");
    assert!(sender == owner, "Not the transaction owner");
    
    // Check status
    let status_key = format!("tx_status:{}", tx_hash);
    let status: u8 = Storage::get(&status_key).expect("Transaction not found");
    assert!(status == TxStatus::Started as u8, "Transaction not in correct state");
    
    // Update status
    Storage::put(&status_key, TxStatus::Committed as u8);
    
    // Perform commit logic
    true
}

#[method]
pub fn complete_operation(tx_hash: H256) -> bool {
    let sender = Runtime::current_sender();
    
    // Check ownership
    let owner_key = format!("tx_owner:{}", tx_hash);
    let owner: H160 = Storage::get(&owner_key).expect("Transaction not found");
    assert!(sender == owner, "Not the transaction owner");
    
    // Check status
    let status_key = format!("tx_status:{}", tx_hash);
    let status: u8 = Storage::get(&status_key).expect("Transaction not found");
    assert!(status == TxStatus::Committed as u8, "Transaction not in correct state");
    
    // Update status
    Storage::put(&status_key, TxStatus::Completed as u8);
    
    // Clean up (optional)
    // Storage::delete(&owner_key);
    
    // Perform completion logic
    true
}
```

This pattern is useful for operations that can't be completed atomically in a single transaction.

### Transaction Batching

Processing multiple operations in a single transaction can improve efficiency:

```rust
struct BatchOperation {
    operation_type: u8,
    target: H160,
    amount: u64,
}

#[method]
pub fn batch_process(operations: Vec<BatchOperation>) -> bool {
    let sender = Runtime::current_sender();
    
    // Check authorization
    assert!(Runtime::check_witness(&sender), "Unauthorized");
    
    // Process each operation
    for op in operations {
        match op.operation_type {
            1 => {
                // Transfer operation
                // ...
            },
            2 => {
                // Approval operation
                // ...
            },
            3 => {
                // Mint operation
                // ...
            },
            _ => {
                panic!("Unknown operation type");
            }
        }
    }
    
    true
}
```

This pattern can reduce gas costs by combining multiple operations into a single transaction, especially for operations that target the same contract.

### Hash-based Transaction Tracking

Using transaction hashes to prevent duplicate processing:

```rust
#[method]
pub fn process_deposit(tx_hash: H256) -> bool {
    // Check if this transaction has already been processed
    let processed_key = format!("processed_tx:{}", tx_hash);
    if Storage::get::<bool>(&processed_key).unwrap_or(false) {
        return false; // Already processed
    }
    
    // Get transaction details
    let tx = Ledger::get_transaction(tx_hash).expect("Transaction not found");
    
    // Process the deposit
    // ...
    
    // Mark as processed
    Storage::put(&processed_key, true);
    
    true
}
```

This pattern is essential for preventing replay attacks or double-processing of the same transaction, especially for cross-contract invocations or oracles.

## Working with Transaction Signers

Neo N3 allows you to check transaction signers to verify authorization:

```rust
#[method]
pub fn multi_sig_operation(required_signers: Vec<H160>) -> bool {
    // Get the transaction hash of the current invocation
    let tx_hash = Runtime::get_entry_script_hash();
    
    // Get all signers of the transaction
    let signers = Ledger::get_transaction_signers(tx_hash);
    
    // Check if all required signers have signed
    for required in required_signers {
        let mut found = false;
        for signer in &signers {
            if signer == required {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    
    // All required signers found
    true
}
```

This pattern is useful for implementing multi-signature requirements or complex authorization schemes.

## Transaction Events and Notifications

Emitting events for important transaction milestones improves transparency and helps with off-chain tracking:

```rust
#[event]
struct TransactionProcessed {
    #[index]
    tx_hash: H256,
    #[index]
    user: H160,
    status: u8,
    timestamp: u64,
}

#[method]
pub fn process_transaction(tx_hash: H256) -> bool {
    let sender = Runtime::current_sender();
    let timestamp = Ledger::current_timestamp();
    
    // Process the transaction
    // ...
    
    // Emit event
    Runtime::notify(
        &TransactionProcessed {
            tx_hash,
            user: sender,
            status: 1, // Success
            timestamp,
        }
    );
    
    true
}
```

For more details on events, see the [Events Guide](./events_guide.md).

## Security Considerations

When working with transactions, consider these security aspects:

1. **Confirmation Requirements**: For high-value operations, require multiple confirmations to avoid issues from chain reorganizations.

2. **Replay Protection**: Always track processed transaction hashes to prevent replay attacks.

3. **Witness Verification**: Use `Runtime::check_witness()` to verify the sender's authorization.

4. **Transaction Expiry**: Be aware of the `valid_until_block` field to handle transaction expiration properly.

5. **Fee Verification**: For certain operations, verify that adequate fees are included in the transaction.

6. **Transaction Sequence**: When operations must happen in sequence, enforce strict ordering through state tracking.

## Examples

For practical examples of transaction patterns, refer to:

1. [Secure Vault Example](../examples/defi/secure_vault/) - Shows advanced transaction validation and multi-step operations
2. [Ledger Example](../examples/ledger_example/) - Demonstrates Ledger API usage with transaction tracking

## Conclusion

Properly handling transactions is crucial for building secure and reliable Neo N3 smart contracts. By implementing these patterns, you can ensure your contracts correctly validate, track, and process blockchain transactions.

For more information on blockchain data access, refer to the [Ledger API Guide](./ledger_api_guide.md) and explore the example contracts that demonstrate these patterns in practice. 