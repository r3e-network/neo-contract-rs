# Transaction Patterns Example

This example demonstrates common transaction patterns for Neo N3 smart contracts, including transaction validation, multi-step transactions, rate limiting, and transaction tracking.

## Overview

The Transaction Patterns example contract showcases best practices for handling blockchain transactions in Neo N3 smart contracts. It demonstrates various patterns that are essential for secure and efficient smart contract development.

## Key Features

- **Transaction Confirmation Validation**: Ensuring transactions have sufficient confirmations
- **Rate Limiting**: Preventing abuse by limiting the number of actions within a time period
- **Hash-based Transaction Tracking**: Preventing double-processing of transactions
- **Multi-step Transactions**: Managing operations that span multiple transactions
- **Event Emission**: Properly tracking transaction processing through events
- **Security Controls**: Implementing proper authentication and validation

## Patterns Demonstrated

### 1. Transaction Confirmation Validation

The contract includes methods to check if a transaction has reached a sufficient number of confirmations before processing it:

```rust
pub fn check_transaction_confirmations(&self, tx_hash: H256, required_confirmations: u32) -> bool {
    let tx_height = Ledger::get_transaction_height(tx_hash);
    if tx_height == 0 {
        return false;
    }
    
    let current_height = Ledger::current_index();
    let confirmations = current_height - tx_height + 1;
    
    confirmations >= required_confirmations
}
```

This is crucial for operations that should only proceed after a transaction is sufficiently confirmed on the blockchain.

### 2. Rate Limiting

The contract implements time-based rate limiting to prevent abuse:

```rust
pub fn is_action_allowed(&self, sender: H160, action: u8) -> bool {
    let current_time = Ledger::current_timestamp();
    let period = current_time / RATE_LIMIT_DURATION;
    
    let count = self.user_action_count.get(&(sender, period)).unwrap_or(0);
    
    count < MAX_OPERATIONS_PER_PERIOD
}
```

This pattern categorizes actions by time periods (e.g., hours) and limits the number of actions per period.

### 3. Hash-based Transaction Tracking

To prevent double-processing, the contract tracks processed transaction hashes:

```rust
pub fn is_processed(&self, tx_hash: H256) -> bool {
    self.processed_txs.get(&tx_hash).unwrap_or(false)
}

fn mark_as_processed(&mut self, tx_hash: H256) {
    self.processed_txs.insert(&tx_hash, &true);
}
```

This is essential for preventing replay attacks and ensuring idempotence.

### 4. Multi-step Transactions

Some operations require multiple steps to complete. The contract demonstrates a pattern for managing such operations:

```rust
// First transaction: Start the operation
pub fn start_operation(&mut self) -> u64 {
    // ... validation and setup ...
    
    // Get next operation ID
    let operation_id = self.next_operation_id.get();
    self.next_operation_id.set(operation_id + 1);
    
    // Store operation info
    self.operations.insert(&operation_id, &(tx_hash, TxStatus::Pending as u8));
    
    // ... more processing ...
    
    operation_id
}

// Second transaction: Complete the operation
pub fn complete_operation(&mut self, operation_id: u64) -> bool {
    // ... validation ...
    
    // Check if operation exists and is pending
    let (original_tx, status) = self.operations.get(&operation_id)
        .expect("Operation not found");
    
    assert!(status == TxStatus::Pending as u8, "Operation not in pending state");
    
    // ... complete the operation ...
    
    true
}
```

This pattern is useful for operations that can't be completed atomically in a single transaction.

## Storage Structure

The contract uses an efficient storage structure to manage all necessary data:

```rust
pub struct TransactionPatterns {
    // Admin
    owner: StorageItem<H160>,
    
    // Transaction tracking
    processed_txs: StorageMap<H256, bool>,
    tx_status: StorageMap<H256, u8>,
    tx_owner: StorageMap<H256, H160>,
    tx_data: StorageMap<H256, Vec<u8>>,
    
    // Operation tracking
    next_operation_id: StorageItem<u64>,
    operations: StorageMap<u64, (H256, u8)>, // (tx_hash, status)
    user_operations: StorageMap<H160, Vec<u64>>, // user -> operation IDs
    
    // Rate limiting
    user_action_count: StorageMap<(H160, u64), u64>, // (user, period) -> count
    last_action_time: StorageMap<H160, u64>, // user -> timestamp
    
    // Balances for demo
    balances: StorageMap<H160, u64>,
}
```

## Events

The contract emits events to track important milestones in transaction processing:

```rust
#[event]
struct TransactionProcessed {
    #[index]
    tx_hash: H256,
    #[index]
    sender: H160,
    action: u8,
    timestamp: u64,
}

#[event]
struct OperationStarted {
    #[index]
    tx_hash: H256,
    #[index]
    sender: H160,
    operation_id: u64,
    timestamp: u64,
}

#[event]
struct OperationCompleted {
    #[index]
    tx_hash: H256,
    #[index]
    sender: H160,
    operation_id: u64,
    timestamp: u64,
}
```

These events provide a clear audit trail of transaction processing.

## Security Considerations

The contract implements several security checks:

1. **Witness Verification**: Using `Runtime::check_witness()` to verify sender authorization
2. **Transaction Validation**: Checking confirmation count and preventing double-processing
3. **Rate Limiting**: Preventing abuse by limiting the frequency of actions
4. **Operation Ownership**: Ensuring only the initiator can complete an operation

## Usage

To use this contract, deploy it to the Neo N3 blockchain and interact with its methods:

1. **Deposit Processing**: Call `process_deposit` with a transaction hash and amount
2. **Multi-step Operations**: Call `start_operation` to start and `complete_operation` to finish
3. **Status Checking**: Use `is_confirmed` and `is_processed` to check transaction status
4. **Rate Limit Checking**: Use `is_action_allowed` and `get_remaining_actions` to check rate limits

## Building and Deployment

To build this example:

```bash
# Development build
cargo build -p tx-patterns --features std

# Production build
cargo build -p tx-patterns --release
```

## Related Documentation

For more details on transaction patterns, see:

- [Transaction Patterns Guide](../../docs/transaction_patterns.md)
- [Ledger API Guide](../../docs/ledger_api_guide.md)
- [Security Guide](../../docs/contract_security_guide.md)

## License

This example is provided under the same license as the Neo Contract Rust framework.