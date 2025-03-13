# Neo N3 Ledger API Cheat Sheet

This cheat sheet provides a quick reference to the Ledger API functions available in Neo N3 smart contracts built with the Neo Contract Rust framework.

## Block Information

| Function | Description | Return Type | Example |
|----------|-------------|-------------|---------|
| `Ledger::current_index()` | Get the current block index (height) | `u32` | `let height = Ledger::current_index();` |
| `Ledger::current_hash()` | Get the current block hash | `H256` | `let hash = Ledger::current_hash();` |
| `Ledger::hash_at(index)` | Get block hash at specific height | `H256` | `let hash = Ledger::hash_at(1000);` |
| `Ledger::get_block(index_or_hash)` | Get detailed block information | `Option<Block>` | `let block = Ledger::get_block(1000);` |
| `Ledger::current_timestamp()` | Get current block timestamp | `u64` | `let timestamp = Ledger::current_timestamp();` |
| `Ledger::block_version()` | Get current block version | `u32` | `let version = Ledger::block_version();` |

## Transaction Information

| Function | Description | Return Type | Example |
|----------|-------------|-------------|---------|
| `Ledger::get_transaction(hash)` | Get transaction by hash | `Option<Transaction>` | `let tx = Ledger::get_transaction(hash);` |
| `Ledger::get_transaction_height(hash)` | Get block height containing tx | `u32` | `let height = Ledger::get_transaction_height(hash);` |
| `Ledger::get_transaction_vm_state(hash)` | Get VM execution state of tx | `i32` | `let state = Ledger::get_transaction_vm_state(hash);` |
| `Ledger::get_transaction_signers(hash)` | Get list of tx signers | `Array` | `let signers = Ledger::get_transaction_signers(hash);` |

## Block-Transaction Relationship

| Function | Description | Return Type | Example |
|----------|-------------|-------------|---------|
| `Ledger::get_transaction_from_block_by_hash(block_hash, tx_index)` | Get tx from block by hash | `Option<Transaction>` | `let tx = Ledger::get_transaction_from_block_by_hash(block_hash, 0);` |
| `Ledger::get_transaction_from_block_by_height(block_height, tx_index)` | Get tx from block by height | `Option<Transaction>` | `let tx = Ledger::get_transaction_from_block_by_height(1000, 0);` |

## System Information

| Function | Description | Return Type | Example |
|----------|-------------|-------------|---------|
| `Ledger::script_hash()` | Get Ledger contract's script hash | `H160` | `let ledger_hash = Ledger::script_hash();` |
| `Ledger::hash()` | Get Ledger contract's hash | `H160` | `let ledger_hash = Ledger::hash();` |
| `Ledger::current_validator_count()` | Get current validator count | `u32` | `let count = Ledger::current_validator_count();` |

## Common Patterns

### Confirming a Transaction

```rust
fn verify_transaction_confirmations(tx_hash: H256, required_confirmations: u32) -> bool {
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        let current_height = Ledger::current_index();
        
        // Check if transaction exists and has enough confirmations
        if current_height >= tx_height && current_height - tx_height + 1 >= required_confirmations {
            return true;
        }
    }
    
    false
}
```

### Time-Based Logic

```rust
fn is_time_elapsed(start_time: u64, duration_seconds: u64) -> bool {
    let current_time = Ledger::current_timestamp();
    current_time >= start_time + duration_seconds
}
```

### Block-Based Schedule

```rust
fn is_action_due(last_action_block: u32, blocks_interval: u32) -> bool {
    let current_block = Ledger::current_index();
    current_block >= last_action_block + blocks_interval
}
```

### Transaction Validation

```rust
fn validate_transaction(tx_hash: H256) -> bool {
    if let Some(tx) = Ledger::get_transaction(tx_hash) {
        // Check transaction properties
        if tx.version == 0 && !tx.script.is_empty() {
            // Check VM execution state (HALT is 0)
            if Ledger::get_transaction_vm_state(tx_hash) == 0 {
                return true;
            }
        }
    }
    
    false
}
```

### Block Information Access

```rust
fn get_block_details(index: u32) -> Option<(H256, u64)> {
    if let Some(block) = Ledger::get_block(index) {
        return Some((block.hash, block.timestamp));
    }
    
    None
}
```

## The Block Object Structure

When using `Ledger::get_block()`, the returned `Block` object has these fields:

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

## The Transaction Object Structure

When using `Ledger::get_transaction()`, the returned `Transaction` object has these fields:

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

## Best Practices

1. **Cache Results**: Store Ledger API results in contract storage if they'll be used repeatedly
2. **Confirmation Thresholds**: Use higher confirmation requirements for high-value operations
3. **Time Buffers**: Account for block time variability when using timestamps
4. **Gas Efficiency**: Minimize Ledger API calls as they consume more gas than regular operations
5. **Error Handling**: Always handle the case where a transaction or block might not exist

## Related Resources

- [Ledger API Guide](./ledger_api_guide.md) - Detailed guide to the Ledger API
- [Ledger API Testing](./ledger_api_testing.md) - Testing Ledger-dependent contracts
- [Ledger Example](../examples/ledger_example/) - Complete example contracts 