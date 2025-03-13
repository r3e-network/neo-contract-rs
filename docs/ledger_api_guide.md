# Ledger API Guide for Neo N3

This guide provides a comprehensive overview of the Ledger API in Neo Contract Rust, which allows smart contracts to interact with blockchain data such as blocks, transactions, timestamps, and more.

## Table of Contents

- [Introduction](#introduction)
- [Block Information](#block-information)
  - [Current Block](#current-block)
  - [Historical Blocks](#historical-blocks)
  - [Block Structure](#block-structure)
- [Transaction Information](#transaction-information)
  - [Retrieving Transactions](#retrieving-transactions)
  - [Transaction Validation](#transaction-validation)
  - [Transaction Heights](#transaction-heights)
  - [Transaction Signers](#transaction-signers)
  - [Transaction VM State](#transaction-vm-state)
- [Blockchain Time](#blockchain-time)
  - [Timestamps](#timestamps)
  - [Time-based Logic](#time-based-logic)
- [Validator Information](#validator-information)
- [Block Data Retrieval Patterns](#block-data-retrieval-patterns)
  - [Index-Based Retrieval](#index-based-retrieval)
  - [Hash-Based Retrieval](#hash-based-retrieval)
- [Transaction Data Retrieval Patterns](#transaction-data-retrieval-patterns)
  - [By Transaction Hash](#by-transaction-hash)
  - [By Block and Index](#by-block-and-index)
- [Security Considerations](#security-considerations)
- [Practical Examples](#practical-examples)
- [API Reference](#api-reference)

## Introduction

The Ledger API provides access to blockchain data within Neo N3 smart contracts. It enables contracts to retrieve information about blocks, transactions, and the current state of the blockchain. This API is essential for implementing time-dependent logic, validating transactions, and accessing historical blockchain data.

The `Ledger` struct in Neo Contract Rust encapsulates this functionality, providing methods to interact with the Neo N3 blockchain ledger.

## Block Information

### Current Block

To retrieve information about the current block:

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_current_block_info() -> (u32, H256, u64) {
    let current_index = Ledger::current_index();
    let current_hash = Ledger::current_hash();
    let current_timestamp = Ledger::current_timestamp();
    
    (current_index, current_hash, current_timestamp)
}
```

### Historical Blocks

To access historical blocks by index or hash:

```rust
#[method]
pub fn get_historical_block(block_index: u32) -> Option<Block> {
    Ledger::get_block(block_index)
}

#[method]
pub fn get_block_by_hash(block_hash: H256) -> Option<Block> {
    Ledger::get_block(block_hash)
}

#[method]
pub fn get_hash_at_height(block_index: u32) -> H256 {
    Ledger::hash_at(block_index)
}
```

### Block Structure

The Block structure contains comprehensive information about a block:

```rust
pub struct Block {
    /// The hash of the block
    pub hash: H256,
    /// The version of the block
    pub version: u32,
    /// The previous block hash
    pub prev_hash: H256,
    /// The merkle root of the transactions
    pub merkle_root: H256,
    /// The timestamp of the block
    pub timestamp: u64,
    /// The index of the block
    pub index: u32,
    /// The primary index of the consensus node that generated this block
    pub primary: u8,
    /// The next consensus node that will be given priority to generate a block
    pub next_consensus: H160,
    /// The transactions in the block
    pub transactions: Array,
}
```

## Transaction Information

### Retrieving Transactions

To retrieve a transaction by its hash:

```rust
#[method]
pub fn get_tx_info(tx_hash: H256) -> Option<Transaction> {
    Ledger::get_transaction(tx_hash)
}
```

To get a transaction from a specific block:

```rust
#[method]
pub fn get_tx_from_block_by_hash(block_hash: H256, tx_index: i32) -> Option<Transaction> {
    Ledger::get_transaction_from_block_by_hash(block_hash, tx_index)
}

#[method]
pub fn get_tx_from_block_by_height(block_height: u32, tx_index: i32) -> Option<Transaction> {
    Ledger::get_transaction_from_block_by_height(block_height, tx_index)
}
```

### Transaction Validation

Validating that a transaction has been confirmed with a certain number of blocks:

```rust
#[method]
pub fn is_tx_confirmed(tx_hash: H256, min_confirmations: u32) -> bool {
    let tx_height = Ledger::get_transaction_height(tx_hash);
    
    // If tx_height is 0, the transaction doesn't exist or isn't confirmed
    if tx_height == 0 {
        return false;
    }
    
    let current_height = Ledger::current_index();
    
    // Check if we have enough confirmations
    current_height >= tx_height && current_height - tx_height >= min_confirmations
}
```

### Transaction Heights

Getting the height at which a transaction was included in a block:

```rust
#[method]
pub fn get_tx_height(tx_hash: H256) -> u32 {
    Ledger::get_transaction_height(tx_hash)
}
```

### Transaction Signers

Retrieving the signers of a transaction:

```rust
#[method]
pub fn get_tx_signers(tx_hash: H256) -> Vec<H160> {
    let signers_array = Ledger::get_transaction_signers(tx_hash);
    
    // Convert Array to Vec<H160>
    let mut signers = Vec::new();
    for i in 0..signers_array.len() {
        if let Some(signer) = signers_array.get::<H160>(i) {
            signers.push(signer);
        }
    }
    
    signers
}
```

### Transaction VM State

Checking the execution state of a transaction:

```rust
#[method]
pub fn get_tx_execution_state(tx_hash: H256) -> i32 {
    Ledger::get_transaction_vm_state(tx_hash)
}
```

## Blockchain Time

### Timestamps

Getting the current blockchain timestamp:

```rust
#[method]
pub fn get_current_time() -> u64 {
    Ledger::current_timestamp()
}
```

### Time-based Logic

Implementing time-based logic for a vesting contract:

```rust
#[contract]
pub struct VestingContract {
    beneficiary: StorageItem<H160>,
    start_time: StorageItem<u64>,
    end_time: StorageItem<u64>,
    total_amount: StorageItem<u64>,
    claimed_amount: StorageItem<u64>,
}

#[contractimpl]
impl VestingContract {
    #[constructor]
    pub fn new(beneficiary: H160, vesting_duration_seconds: u64, total_amount: u64) -> Self {
        let start_time = Ledger::current_timestamp();
        let end_time = start_time + vesting_duration_seconds;
        
        Self {
            beneficiary: StorageItem::new(beneficiary),
            start_time: StorageItem::new(start_time),
            end_time: StorageItem::new(end_time),
            total_amount: StorageItem::new(total_amount),
            claimed_amount: StorageItem::new(0),
        }
    }
    
    #[method]
    pub fn vested_amount(&self) -> u64 {
        let current_time = Ledger::current_timestamp();
        let start_time = self.start_time.get();
        let end_time = self.end_time.get();
        let total_amount = self.total_amount.get();
        
        if current_time < start_time {
            return 0;
        } else if current_time >= end_time {
            return total_amount;
        } else {
            // Linear vesting calculation
            let time_vested = current_time - start_time;
            let total_vesting_time = end_time - start_time;
            
            // Calculate vested amount based on time elapsed
            // Using integer division, so result is rounded down
            return total_amount * time_vested / total_vesting_time;
        }
    }
    
    #[method]
    pub fn claimable_amount(&self) -> u64 {
        let vested = self.vested_amount();
        let claimed = self.claimed_amount.get();
        
        if vested > claimed {
            return vested - claimed;
        } else {
            return 0;
        }
    }
    
    #[method]
    pub fn claim(&mut self) -> bool {
        let sender = Runtime::current_sender();
        let beneficiary = self.beneficiary.get();
        
        // Only the beneficiary can claim
        assert!(sender == beneficiary, "Only beneficiary can claim");
        
        let claimable = self.claimable_amount();
        assert!(claimable > 0, "No tokens available to claim");
        
        // Update claimed amount
        let current_claimed = self.claimed_amount.get();
        self.claimed_amount.set(current_claimed + claimable);
        
        // Transfer tokens logic would go here
        // ...
        
        true
    }
}
```

## Validator Information

Retrieving the current validator count:

```rust
#[method]
pub fn get_validator_count() -> u32 {
    Ledger::current_validator_count()
}
```

## Block Data Retrieval Patterns

### Index-Based Retrieval

Retrieving block information using block indices:

```rust
#[method]
pub fn get_blocks_in_range(start_index: u32, count: u32) -> Vec<Block> {
    let mut blocks = Vec::new();
    let end_index = start_index + count;
    
    for i in start_index..end_index {
        if let Some(block) = Ledger::get_block(i) {
            blocks.push(block);
        }
    }
    
    blocks
}
```

### Hash-Based Retrieval

Traversing blockchain history using block hashes:

```rust
#[method]
pub fn traverse_blockchain_history(start_hash: H256, count: u32) -> Vec<H256> {
    let mut block_hashes = Vec::new();
    let mut current_hash = start_hash;
    
    for _ in 0..count {
        block_hashes.push(current_hash);
        
        // Get the block with the current hash
        if let Some(block) = Ledger::get_block(current_hash) {
            // Get the previous block hash
            current_hash = block.prev_hash;
        } else {
            // Block not found, break the loop
            break;
        }
    }
    
    block_hashes
}
```

## Transaction Data Retrieval Patterns

### By Transaction Hash

Using transaction hashes to analyze related transactions:

```rust
#[method]
pub fn analyze_transaction(tx_hash: H256) -> (u32, Vec<H160>, i32) {
    let height = Ledger::get_transaction_height(tx_hash);
    
    // Get signers
    let signers_array = Ledger::get_transaction_signers(tx_hash);
    let mut signers = Vec::new();
    for i in 0..signers_array.len() {
        if let Some(signer) = signers_array.get::<H160>(i) {
            signers.push(signer);
        }
    }
    
    // Get VM state
    let vm_state = Ledger::get_transaction_vm_state(tx_hash);
    
    (height, signers, vm_state)
}
```

### By Block and Index

Retrieving transactions from specific blocks:

```rust
#[method]
pub fn get_first_transaction_in_blocks(start_block: u32, block_count: u32) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    let end_block = start_block + block_count;
    
    for block_index in start_block..end_block {
        if let Some(tx) = Ledger::get_transaction_from_block_by_height(block_index, 0) {
            transactions.push(tx);
        }
    }
    
    transactions
}
```

## Security Considerations

When working with the Ledger API, keep these security considerations in mind:

### Trusting Blockchain Time

Blockchain time (timestamps) can vary slightly from real-world time and can be manipulated by miners within certain bounds:

```rust
#[method]
pub fn is_safe_time_reference(reference_time: u64, buffer_seconds: u64) -> bool {
    let current_time = Ledger::current_timestamp();
    
    // Add a buffer to account for potential blockchain time variations
    // Only consider something valid if it's older than current time - buffer
    reference_time <= current_time.saturating_sub(buffer_seconds)
}
```

### Transaction Confirmation Safety

Ensure critical operations require sufficient confirmations:

```rust
#[method]
pub fn process_high_value_transaction(tx_hash: H256, amount: u64) -> bool {
    // For high-value transactions, require more confirmations
    let required_confirmations = if amount > 1000 { 6 } else { 3 };
    
    let tx_height = Ledger::get_transaction_height(tx_hash);
    if tx_height == 0 {
        return false; // Transaction doesn't exist
    }
    
    let current_height = Ledger::current_index();
    let confirmations = current_height - tx_height;
    
    confirmations >= required_confirmations
}
```

### Handling Block Reorganizations

Be aware that block reorganizations can occur, potentially affecting the outcome of previous operations:

```rust
#[contract]
pub struct SafeOperation {
    processed_txs: StorageMap<H256, bool>,
    processing_heights: StorageMap<H256, u32>,
    required_confirmations: StorageItem<u32>,
}

#[contractimpl]
impl SafeOperation {
    #[constructor]
    pub fn new(min_confirmations: u32) -> Self {
        Self {
            processed_txs: StorageMap::new(),
            processing_heights: StorageMap::new(),
            required_confirmations: StorageItem::new(min_confirmations),
        }
    }
    
    #[method]
    pub fn process_tx(&mut self, tx_hash: H256) -> bool {
        // Check if already fully processed
        if let Some(processed) = self.processed_txs.get(&tx_hash) {
            if processed {
                return false; // Already processed
            }
        }
        
        let tx_height = Ledger::get_transaction_height(tx_hash);
        if tx_height == 0 {
            return false; // Transaction doesn't exist
        }
        
        let current_height = Ledger::current_index();
        let confirmations = current_height - tx_height;
        let required_confirmations = self.required_confirmations.get();
        
        if confirmations >= required_confirmations {
            // Safe to process fully
            // Process the transaction...
            
            // Mark as processed
            self.processed_txs.insert(&tx_hash, &true);
            return true;
        } else {
            // Store the processing height for later verification
            self.processing_heights.insert(&tx_hash, &tx_height);
            return false; // Not enough confirmations yet
        }
    }
    
    #[method]
    pub fn verify_processing(&mut self, tx_hash: H256) -> bool {
        // Check if transaction was previously in processing
        if let Some(original_height) = self.processing_heights.get(&tx_hash) {
            // Get current height of the transaction
            let current_tx_height = Ledger::get_transaction_height(tx_hash);
            
            // If height changed, the block might have been reorganized
            if current_tx_height != original_height {
                // Transaction has moved or no longer exists in the blockchain
                // Handle this case (could be reorg)
                self.processing_heights.remove(&tx_hash);
                return false;
            }
            
            // Verify confirmations again
            let current_height = Ledger::current_index();
            let confirmations = current_height - current_tx_height;
            let required_confirmations = self.required_confirmations.get();
            
            if confirmations >= required_confirmations {
                // Now safe to finalize processing
                // Process the transaction...
                
                // Mark as fully processed
                self.processed_txs.insert(&tx_hash, &true);
                self.processing_heights.remove(&tx_hash);
                return true;
            }
        }
        
        false
    }
}
```

## Practical Examples

### Block-Based Rewards

Implementing a reward system based on blocks:

```rust
#[contract]
pub struct BlockRewards {
    reward_per_block: StorageItem<u64>,
    last_claimed_block: StorageMap<H160, u32>,
    rewards_balance: StorageMap<H160, u64>,
}

#[contractimpl]
impl BlockRewards {
    #[constructor]
    pub fn new(reward_amount: u64) -> Self {
        Self {
            reward_per_block: StorageItem::new(reward_amount),
            last_claimed_block: StorageMap::new(),
            rewards_balance: StorageMap::new(),
        }
    }
    
    #[method]
    pub fn claim_rewards(&mut self) -> u64 {
        let sender = Runtime::current_sender();
        let current_block = Ledger::current_index();
        
        // Get the last block this user claimed rewards for
        let last_claimed = self.last_claimed_block.get(&sender).unwrap_or(current_block);
        
        // Calculate blocks passed since last claim
        let blocks_passed = if current_block > last_claimed {
            current_block - last_claimed
        } else {
            0
        };
        
        if blocks_passed == 0 {
            return 0;
        }
        
        // Calculate rewards
        let reward_per_block = self.reward_per_block.get();
        let reward_amount = reward_per_block * blocks_passed;
        
        // Update last claimed block
        self.last_claimed_block.insert(&sender, &current_block);
        
        // Update rewards balance
        let current_balance = self.rewards_balance.get(&sender).unwrap_or(0);
        let new_balance = current_balance + reward_amount;
        self.rewards_balance.insert(&sender, &new_balance);
        
        reward_amount
    }
    
    #[method]
    pub fn get_claimable_rewards(&self, user: H160) -> u64 {
        let current_block = Ledger::current_index();
        let last_claimed = self.last_claimed_block.get(&user).unwrap_or(current_block);
        
        // Calculate blocks passed since last claim
        let blocks_passed = if current_block > last_claimed {
            current_block - last_claimed
        } else {
            0
        };
        
        // Calculate rewards
        let reward_per_block = self.reward_per_block.get();
        reward_per_block * blocks_passed
    }
    
    #[method]
    pub fn get_rewards_balance(&self, user: H160) -> u64 {
        self.rewards_balance.get(&user).unwrap_or(0)
    }
}
```

### Time-locked Fund Release

Creating a time-locked fund release system:

```rust
#[contract]
pub struct TimeLock {
    funds: StorageMap<H160, u64>,
    release_times: StorageMap<H160, u64>,
}

#[contractimpl]
impl TimeLock {
    #[constructor]
    pub fn new() -> Self {
        Self {
            funds: StorageMap::new(),
            release_times: StorageMap::new(),
        }
    }
    
    #[method]
    pub fn lock_funds(&mut self, beneficiary: H160, amount: u64, lock_duration_seconds: u64) -> bool {
        let sender = Runtime::current_sender();
        
        // Calculate release time
        let current_time = Ledger::current_timestamp();
        let release_time = current_time + lock_duration_seconds;
        
        // Store the funds and release time
        self.funds.insert(&beneficiary, &amount);
        self.release_times.insert(&beneficiary, &release_time);
        
        // Implement token transfer from sender to contract here
        // ...
        
        true
    }
    
    #[method]
    pub fn withdraw_funds(&mut self) -> bool {
        let beneficiary = Runtime::current_sender();
        
        // Get the locked amount
        let locked_amount = match self.funds.get(&beneficiary) {
            Some(amount) => amount,
            None => return false, // No funds locked
        };
        
        // Get the release time
        let release_time = match self.release_times.get(&beneficiary) {
            Some(time) => time,
            None => return false, // No release time set
        };
        
        // Check if the release time has passed
        let current_time = Ledger::current_timestamp();
        if current_time < release_time {
            return false; // Funds still locked
        }
        
        // Clear the locked funds
        self.funds.remove(&beneficiary);
        self.release_times.remove(&beneficiary);
        
        // Implement token transfer from contract to beneficiary here
        // ...
        
        true
    }
    
    #[method]
    pub fn get_lock_info(&self, user: H160) -> (u64, u64, u64) {
        let amount = self.funds.get(&user).unwrap_or(0);
        let release_time = self.release_times.get(&user).unwrap_or(0);
        let current_time = Ledger::current_timestamp();
        
        // Calculate remaining time in seconds
        let remaining_time = if release_time > current_time {
            release_time - current_time
        } else {
            0
        };
        
        (amount, release_time, remaining_time)
    }
}
```

### Transaction Rate Limiting

Implementing rate limiting based on transactions:

```rust
#[contract]
pub struct RateLimiter {
    user_last_tx_time: StorageMap<H160, u64>,
    min_interval_seconds: StorageItem<u64>,
}

#[contractimpl]
impl RateLimiter {
    #[constructor]
    pub fn new(interval_seconds: u64) -> Self {
        Self {
            user_last_tx_time: StorageMap::new(),
            min_interval_seconds: StorageItem::new(interval_seconds),
        }
    }
    
    #[method]
    pub fn perform_action(&mut self) -> bool {
        let sender = Runtime::current_sender();
        let current_time = Ledger::current_timestamp();
        let min_interval = self.min_interval_seconds.get();
        
        // Get the last transaction time for this user
        if let Some(last_tx_time) = self.user_last_tx_time.get(&sender) {
            // Check if enough time has passed
            if current_time < last_tx_time + min_interval {
                return false; // Rate limit exceeded
            }
        }
        
        // Update the last transaction time
        self.user_last_tx_time.insert(&sender, &current_time);
        
        // Perform the rate-limited action
        // ...
        
        true
    }
    
    #[method]
    pub fn can_perform_action(&self, user: H160) -> bool {
        let current_time = Ledger::current_timestamp();
        let min_interval = self.min_interval_seconds.get();
        
        // Get the last transaction time for this user
        if let Some(last_tx_time) = self.user_last_tx_time.get(&user) {
            // Check if enough time has passed
            return current_time >= last_tx_time + min_interval;
        }
        
        // User hasn't performed any action yet
        true
    }
    
    #[method]
    pub fn get_cooldown_remaining(&self, user: H160) -> u64 {
        let current_time = Ledger::current_timestamp();
        let min_interval = self.min_interval_seconds.get();
        
        // Get the last transaction time for this user
        if let Some(last_tx_time) = self.user_last_tx_time.get(&user) {
            let next_valid_time = last_tx_time + min_interval;
            
            // Calculate remaining cooldown time
            if current_time < next_valid_time {
                return next_valid_time - current_time;
            }
        }
        
        // No cooldown remaining
        0
    }
}
```

## API Reference

### Ledger Methods

| Method | Description | Return Type |
|--------|-------------|-------------|
| `Ledger::current_index()` | Gets the current block index | `u32` |
| `Ledger::current_hash()` | Gets the current block hash | `H256` |
| `Ledger::hash_at(index)` | Gets the block hash at the specified index | `H256` |
| `Ledger::get_block(index_or_hash)` | Gets a block by index or hash | `Option<Block>` |
| `Ledger::get_transaction(hash)` | Gets a transaction by hash | `Option<Transaction>` |
| `Ledger::get_transaction_height(hash)` | Gets the height at which a transaction was included | `u32` |
| `Ledger::current_timestamp()` | Gets the current block timestamp | `u64` |
| `Ledger::current_validator_count()` | Gets the current validator count | `u32` |
| `Ledger::block_version()` | Gets the current block version | `u32` |
| `Ledger::get_transaction_from_block_by_hash(block_hash, tx_index)` | Gets a transaction from a block by block hash | `Option<Transaction>` |
| `Ledger::get_transaction_from_block_by_height(block_height, tx_index)` | Gets a transaction from a block by block height | `Option<Transaction>` |
| `Ledger::get_transaction_signers(hash)` | Gets the signers of a transaction | `Array` |
| `Ledger::get_transaction_vm_state(hash)` | Gets the VM state of a transaction | `i32` |

### Data Structures

#### Block

```rust
pub struct Block {
    pub hash: H256,
    pub version: u32,
    pub prev_hash: H256,
    pub merkle_root: H256,
    pub timestamp: u64,
    pub index: u32,
    pub primary: u8,
    pub next_consensus: H160,
    pub transactions: Array,
}
```

#### Transaction

```rust
pub struct Transaction {
    pub hash: H256,
    pub version: u8,
    pub nonce: u32,
    pub sender: H160,
    pub system_fee: Int256,
    pub network_fee: Int256,
    pub valid_until_block: u32,
    pub script: ByteString,
}
```

## Conclusion

The Ledger API provides smart contracts with access to the Neo N3 blockchain's state, enabling complex logic based on block data, timestamps, and transaction information. By properly utilizing this API, contracts can implement time-locked functionality, block rewards, rate limiting, and transaction validation.

When working with the Ledger API, always consider the security implications of blockchain time, transaction confirmations, and potential block reorganizations. Implement appropriate safeguards to ensure your contract behaves correctly in all scenarios.

For more information on related topics, see:
- [Contract Security Guide](./contract_security_guide.md)
- [Transaction Patterns Guide](./transaction_patterns.md)
- [Cross-Contract Communication Guide](./cross_contract_guide.md)
- [Ledger Example](../examples/ledger_example/) - A practical implementation of Ledger API concepts 