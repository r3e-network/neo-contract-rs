# Neo N3 Ledger API Best Practices

This guide provides best practices for working with the Ledger API in Neo N3 smart contracts. These recommendations will help you build more secure, efficient, and reliable smart contracts that interact with blockchain data.

## Table of Contents

- [Security Considerations](#security-considerations)
- [Gas Optimization](#gas-optimization)
- [Reliability Best Practices](#reliability-best-practices)
- [Time-Based Logic](#time-based-logic)
- [Transaction Validation](#transaction-validation)
- [Block-Based Logic](#block-based-logic)
- [Testing Recommendations](#testing-recommendations)
- [Examples of Best Practices](#examples-of-best-practices)

## Security Considerations

### Transaction Confirmations

**Always require sufficient confirmations for high-value operations**

```rust
// Bad practice: Processing without confirmation checks
fn process_high_value_operation(tx_hash: H256) -> bool {
    // Directly process the transaction without confirmation checks
    true
}

// Good practice: Requiring confirmations for high-value operations
fn process_high_value_operation(tx_hash: H256) -> bool {
    const REQUIRED_CONFIRMATIONS: u32 = 6;
    
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        let current_height = Ledger::current_index();
        if current_height >= tx_height && 
           current_height - tx_height + 1 >= REQUIRED_CONFIRMATIONS {
            // Process the transaction
            return true;
        }
    }
    
    false
}
```

### Timestamp Manipulation Protections

**Use time buffers and bounds checks for time-sensitive operations**

```rust
// Bad practice: Exact timestamp comparisons
fn is_deadline_passed(deadline: u64) -> bool {
    Ledger::current_timestamp() > deadline
}

// Good practice: Adding a buffer for timestamp variance
fn is_deadline_passed(deadline: u64) -> bool {
    const TIME_BUFFER: u64 = 60; // 60 seconds buffer
    Ledger::current_timestamp() > deadline + TIME_BUFFER
}
```

### Prevent Transaction Replay

**Track processed transactions to prevent replay attacks**

```rust
// Good practice: Tracking processed transactions
struct Contract {
    processed_txs: StorageMap<H256, bool>
}

impl Contract {
    fn process_transaction(&mut self, tx_hash: H256) -> bool {
        // Check if already processed
        if self.processed_txs.get(&tx_hash) {
            return false;
        }
        
        // Process transaction...
        
        // Mark as processed
        self.processed_txs.insert(&tx_hash, true);
        true
    }
}
```

## Gas Optimization

### Cache Ledger API Results

**Cache results in storage for frequently accessed data**

```rust
// Bad practice: Repeatedly calling Ledger API
fn get_timestamp_for_block(&self, block_index: u32) -> u64 {
    if let Some(block) = Ledger::get_block(block_index) {
        return block.timestamp;
    }
    0
}

// Good practice: Caching results in storage
struct Contract {
    block_timestamps: StorageMap<u32, u64>
}

impl Contract {
    fn get_timestamp_for_block(&mut self, block_index: u32) -> u64 {
        if let Some(timestamp) = self.block_timestamps.get(&block_index) {
            return timestamp;
        }
        
        if let Some(block) = Ledger::get_block(block_index) {
            let timestamp = block.timestamp;
            self.block_timestamps.insert(&block_index, timestamp);
            return timestamp;
        }
        0
    }
}
```

### Minimize Ledger API Calls

**Batch operations to reduce Ledger API calls**

```rust
// Bad practice: Multiple separate Ledger calls
fn get_multiple_block_timestamps(indexes: &[u32]) -> Vec<u64> {
    let mut results = Vec::new();
    for index in indexes {
        if let Some(block) = Ledger::get_block(*index) {
            results.push(block.timestamp);
        } else {
            results.push(0);
        }
    }
    results
}

// Good practice: Process data from a single call when possible
fn get_sequential_block_timestamps(start_index: u32, count: u32) -> Vec<u64> {
    let mut results = Vec::new();
    let mut current_index = start_index;
    let mut last_timestamp = 0;
    
    // Get first block
    if let Some(block) = Ledger::get_block(current_index) {
        last_timestamp = block.timestamp;
        results.push(last_timestamp);
        current_index += 1;
    } else {
        return results;
    }
    
    // Estimate remaining timestamps using average block time
    // This avoids multiple Ledger calls when exact precision isn't needed
    const AVG_BLOCK_TIME: u64 = 15; // 15 seconds per block on average
    for _ in 1..count {
        last_timestamp += AVG_BLOCK_TIME;
        results.push(last_timestamp);
        current_index += 1;
    }
    
    results
}
```

### Lazy Loading Ledger Data

**Load blockchain data only when needed**

```rust
// Bad practice: Loading all data upfront
fn process_transaction_batch(&mut self, tx_hashes: &[H256]) {
    // Load all transactions immediately
    let mut transactions = Vec::new();
    for hash in tx_hashes {
        if let Some(tx) = Ledger::get_transaction(*hash) {
            transactions.push(tx);
        }
    }
    
    // Process transactions
    for tx in transactions {
        // Process each transaction
    }
}

// Good practice: Loading data only when processing
fn process_transaction_batch(&mut self, tx_hashes: &[H256]) {
    for hash in tx_hashes {
        // Only load transaction if not already processed
        if !self.is_processed(*hash) {
            if let Some(tx) = Ledger::get_transaction(*hash) {
                // Process transaction
                self.mark_processed(*hash);
            }
        }
    }
}
```

## Reliability Best Practices

### Handle Missing Data Gracefully

**Always handle cases where blockchain data might not be available**

```rust
// Bad practice: Assuming data always exists
fn get_transaction_time(tx_hash: H256) -> u64 {
    let tx_height = Ledger::get_transaction_height(tx_hash);
    let block = Ledger::get_block(tx_height).unwrap();
    block.timestamp // Will panic if block is not found
}

// Good practice: Handling possible missing data
fn get_transaction_time(tx_hash: H256) -> Option<u64> {
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        if let Some(block) = Ledger::get_block(tx_height) {
            return Some(block.timestamp);
        }
    }
    None
}
```

### Block Reorganization Awareness

**Be aware of potential block reorganizations for recent blocks**

```rust
// Bad practice: Assuming finality for recent blocks
fn is_transaction_final(tx_hash: H256) -> bool {
    Ledger::get_transaction(tx_hash).is_some()
}

// Good practice: Requiring multiple confirmations
fn is_transaction_final(tx_hash: H256) -> bool {
    const FINALITY_CONFIRMATIONS: u32 = 10;
    
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        let current_height = Ledger::current_index();
        return current_height >= tx_height && 
               current_height - tx_height + 1 >= FINALITY_CONFIRMATIONS;
    }
    false
}
```

## Time-Based Logic

### Flexible Time Comparisons

**Use time windows instead of exact matches**

```rust
// Bad practice: Exact timestamp comparison
fn is_within_timeframe(start_time: u64, end_time: u64) -> bool {
    let current = Ledger::current_timestamp();
    current >= start_time && current <= end_time
}

// Good practice: Using time windows
fn is_within_timeframe(start_time: u64, end_time: u64) -> bool {
    const BUFFER: u64 = 60; // 60 seconds buffer
    let current = Ledger::current_timestamp();
    current >= start_time.saturating_sub(BUFFER) && 
    current <= end_time.saturating_add(BUFFER)
}
```

### Time-Lock Implementation

**Properly implement time-locks with appropriate safeguards**

```rust
// Good practice: Time-lock with safeguards
struct TimeLock {
    unlock_time: StorageItem<u64>,
    owner: StorageItem<H160>,
    executed: StorageItem<bool>
}

impl TimeLock {
    fn new(owner: H160, unlock_time: u64) -> Self {
        Self {
            unlock_time: StorageItem::new(unlock_time),
            owner: StorageItem::new(owner),
            executed: StorageItem::new(false)
        }
    }
    
    fn execute(&mut self) -> bool {
        // Check if already executed
        if self.executed.get() {
            return false;
        }
        
        // Check if sender is owner
        assert!(Runtime::check_witness(&self.owner.get()), "Not authorized");
        
        // Check if unlock time has passed
        let current_time = Ledger::current_timestamp();
        let unlock_time = self.unlock_time.get();
        
        assert!(current_time >= unlock_time, "Time lock not expired");
        
        // Mark as executed
        self.executed.set(true);
        
        // Execute time-locked action
        true
    }
}
```

## Transaction Validation

### Verify Transaction VM State

**Check transaction execution state to ensure successful execution**

```rust
// Good practice: Verifying VM state
fn is_transaction_successful(tx_hash: H256) -> bool {
    const HALT_STATE: i32 = 0; // HALT state indicates successful execution
    
    if let Some(tx) = Ledger::get_transaction(tx_hash) {
        return Ledger::get_transaction_vm_state(tx_hash) == HALT_STATE;
    }
    false
}
```

### Validate Transaction Content

**Verify transaction properties for additional security**

```rust
// Good practice: Validating transaction content
fn validate_transaction(&self, tx_hash: H256, expected_sender: H160) -> bool {
    if let Some(tx) = Ledger::get_transaction(tx_hash) {
        // Verify sender
        if tx.sender != expected_sender {
            return false;
        }
        
        // Verify script is not empty
        if tx.script.is_empty() {
            return false;
        }
        
        // Verify VM state
        if Ledger::get_transaction_vm_state(tx_hash) != 0 {
            return false;
        }
        
        return true;
    }
    false
}
```

## Block-Based Logic

### Reliable Block-Based Intervals

**Use block numbers for reliable interval tracking**

```rust
// Good practice: Block-based intervals for recurring actions
struct BlockBasedSchedule {
    last_execution_block: StorageItem<u32>,
    interval_blocks: StorageItem<u32>
}

impl BlockBasedSchedule {
    fn new(interval: u32) -> Self {
        Self {
            last_execution_block: StorageItem::new(Ledger::current_index()),
            interval_blocks: StorageItem::new(interval)
        }
    }
    
    fn is_execution_due(&self) -> bool {
        let current_block = Ledger::current_index();
        let last_block = self.last_execution_block.get();
        let interval = self.interval_blocks.get();
        
        current_block >= last_block + interval
    }
    
    fn execute_scheduled_action(&mut self) -> bool {
        if self.is_execution_due() {
            // Perform scheduled action
            
            // Update last execution block
            self.last_execution_block.set(Ledger::current_index());
            return true;
        }
        false
    }
}
```

### Block Height Range Validation

**Validate operations within specific block ranges**

```rust
// Good practice: Block range validation
fn is_within_valid_block_range(min_block: u32, max_block: u32) -> bool {
    let current_block = Ledger::current_index();
    current_block >= min_block && current_block <= max_block
}
```

## Testing Recommendations

### Mock Different Block States

**Test contract with different block heights and timestamps**

```rust
#[test]
fn test_time_dependent_logic() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Test with different timestamps
    test.ledger().set_current_timestamp(1000);
    assert!(!contract.is_deadline_reached(1500));
    
    test.ledger().set_current_timestamp(2000);
    assert!(contract.is_deadline_reached(1500));
}
```

### Test Edge Cases

**Test boundary conditions for blockchain data**

```rust
#[test]
fn test_block_reorganization_handling() {
    let mut test = TestBuilder::new()
        .with_mock_ledger()
        .build();
    
    // Setup
    let tx_hash = H256::from_slice(&[1; 32]);
    test.ledger().mock_transaction_height(tx_hash.clone(), 995);
    test.ledger().set_current_index(1000);
    
    // Test with 5 confirmations (should pass)
    assert!(contract.verify_transaction_confirmations(tx_hash.clone(), 5));
    
    // Test with 6 confirmations (should fail)
    assert!(!contract.verify_transaction_confirmations(tx_hash.clone(), 6));
    
    // Simulate a reorg where transaction is now in block 994
    test.ledger().mock_transaction_height(tx_hash.clone(), 994);
    
    // Should now have 6 confirmations
    assert!(contract.verify_transaction_confirmations(tx_hash.clone(), 6));
}
```

## Examples of Best Practices

### Complete Vesting Implementation

```rust
pub struct VestingContract {
    beneficiary: StorageItem<H160>,
    start_time: StorageItem<u64>,
    duration: StorageItem<u64>,
    total_amount: StorageItem<u64>,
    claimed_amount: StorageItem<u64>,
    tx_processed: StorageMap<H256, bool>
}

impl VestingContract {
    pub fn new(beneficiary: H160, total_amount: u64, duration: u64) -> Self {
        Self {
            beneficiary: StorageItem::new(beneficiary),
            start_time: StorageItem::new(Ledger::current_timestamp()),
            duration: StorageItem::new(duration),
            total_amount: StorageItem::new(total_amount),
            claimed_amount: StorageItem::new(0),
            tx_processed: StorageMap::new()
        }
    }
    
    pub fn vested_amount(&self) -> u64 {
        let start = self.start_time.get();
        let duration = self.duration.get();
        let total = self.total_amount.get();
        let current = Ledger::current_timestamp();
        
        if current <= start {
            return 0;
        }
        
        if duration == 0 || current >= start + duration {
            return total;
        }
        
        // Calculate vested amount linearly
        let elapsed = current - start;
        (total * elapsed) / duration
    }
    
    pub fn claimable_amount(&self) -> u64 {
        let vested = self.vested_amount();
        let claimed = self.claimed_amount.get();
        vested.saturating_sub(claimed)
    }
    
    pub fn claim(&mut self) -> u64 {
        // Verify caller is beneficiary
        let beneficiary = self.beneficiary.get();
        assert!(Runtime::check_witness(&beneficiary), "Not authorized");
        
        // Calculate claimable amount
        let claimable = self.claimable_amount();
        if claimable == 0 {
            return 0;
        }
        
        // Update claimed amount
        let new_claimed = self.claimed_amount.get() + claimable;
        self.claimed_amount.set(new_claimed);
        
        // Emit event
        let now = Ledger::current_timestamp();
        Runtime::notify(
            "VestingClaimed",
            &[
                &beneficiary,
                &claimable,
                &now
            ]
        );
        
        claimable
    }
    
    pub fn process_claim_transaction(&mut self, tx_hash: H256) -> bool {
        // Check if transaction already processed
        if self.tx_processed.get(&tx_hash) {
            return false;
        }
        
        const REQUIRED_CONFIRMATIONS: u32 = 3;
        
        // Verify transaction has enough confirmations
        if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
            let current_height = Ledger::current_index();
            
            if current_height >= tx_height && 
               current_height - tx_height + 1 >= REQUIRED_CONFIRMATIONS {
                
                // Verify transaction execution state
                if Ledger::get_transaction_vm_state(tx_hash) != 0 {
                    return false;
                }
                
                // Process claim
                self.tx_processed.insert(&tx_hash, true);
                return true;
            }
        }
        
        false
    }
}
```

### Transaction Validation with Rate Limiting

```rust
pub struct TransactionProcessor {
    processed_txs: StorageMap<H256, bool>,
    last_processed_time: StorageMap<H160, u64>,
    cooldown_period: StorageItem<u64>
}

impl TransactionProcessor {
    pub fn new(cooldown_seconds: u64) -> Self {
        Self {
            processed_txs: StorageMap::new(),
            last_processed_time: StorageMap::new(),
            cooldown_period: StorageItem::new(cooldown_seconds)
        }
    }
    
    pub fn process_transaction(&mut self, tx_hash: H256, sender: H160) -> bool {
        // Check if already processed
        if self.processed_txs.get(&tx_hash) {
            return false;
        }
        
        // Check rate limit
        let current_time = Ledger::current_timestamp();
        let cooldown = self.cooldown_period.get();
        
        if let Some(last_time) = self.last_processed_time.get(&sender) {
            if current_time < last_time + cooldown {
                return false; // Still in cooldown period
            }
        }
        
        // Verify transaction
        if let Some(tx) = Ledger::get_transaction(tx_hash) {
            // Verify sender
            if tx.sender != sender {
                return false;
            }
            
            // Verify execution state
            if Ledger::get_transaction_vm_state(tx_hash) != 0 {
                return false;
            }
            
            // Process transaction
            self.processed_txs.insert(&tx_hash, true);
            self.last_processed_time.insert(&sender, current_time);
            
            return true;
        }
        
        false
    }
    
    pub fn cooldown_remaining(&self, sender: H160) -> u64 {
        let current_time = Ledger::current_timestamp();
        let cooldown = self.cooldown_period.get();
        
        if let Some(last_time) = self.last_processed_time.get(&sender) {
            let expiry_time = last_time + cooldown;
            
            if current_time < expiry_time {
                return expiry_time - current_time;
            }
        }
        
        0 // No cooldown remaining
    }
}
```

## Summary

Following these best practices when working with the Ledger API will help ensure your smart contracts are:

1. **Secure** - Protected against common attacks and manipulation
2. **Efficient** - Optimized for gas consumption
3. **Reliable** - Able to handle edge cases and unexpected scenarios
4. **Maintainable** - Well-structured and easier to update

Remember that blockchain data is inherently variable and can be subject to reorganization, especially for recent blocks. Always implement appropriate safeguards and validation to ensure your contract behaves correctly under all circumstances.

## Related Resources

- [Ledger API Guide](./ledger_api_guide.md) - Comprehensive guide to the Ledger API
- [Ledger API Cheat Sheet](./ledger_api_cheatsheet.md) - Quick reference for Ledger API functions
- [Ledger API Testing](./ledger_api_testing.md) - Testing blockchain data dependent contracts
- [Ledger API Diagrams](./ledger_api_diagram.md) - Visual diagrams of blockchain structures
- [Ledger Example](../examples/ledger_example/) - Complete example implementation 