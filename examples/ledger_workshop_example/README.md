# Blockchain-Powered Escrow Contract with Neo Annotations

This is a sample implementation of the escrow contract developed as part of the [Ledger API Workshop](../../docs/ledger_api_workshop.md). It demonstrates practical usage of Neo N3's Ledger API for building blockchain-aware smart contracts with modern Neo Contract annotations.

## Overview

The escrow contract facilitates secure transactions between two parties by implementing:

1. **Time-Locked Agreements**: Escrow funds are locked until a specified time or until both parties agree.
2. **Transaction Validation**: Using blockchain data to verify and track transactions.
3. **Rate Limiting**: Preventing abuse by limiting how frequently actions can be performed.
4. **Block-Based Dispute Resolution**: Utilizing block heights for timing dispute resolution periods.
5. **Neo Contract Annotations**: Using Neo's modern annotation system for improved security and efficiency.

## Neo Contract Annotations

This contract uses Neo's modern annotation system to enhance security, readability, and efficiency:

| Annotation | Purpose | Example in Codebase |
|------------|---------|---------------------|
| `#[neo_contract::contract]` | Marks a struct as a smart contract | `Escrow` struct |
| `#[constructor]` | Identifies initialization method | `fn new()` |
| `#[method]` | Exposes external methods | `fn create_escrow()` |
| `#[safe]` | Marks read-only methods | `fn get_escrow_details()` |
| `#[no_reentry]` | Prevents re-entrancy attacks | `fn release_escrow()` |
| `#[neo_contract::event(...)]` | Defines structured events | `EscrowCreated` event |

```rust
// Example of annotated contract method
#[method]
#[no_reentry]
fn create_escrow(sender: Address, recipient: Address, amount: u64, timeout_seconds: u64) -> u64 {
    // Implementation...
}

// Example of safe read-only method
#[method]
#[safe]
fn get_escrow_details(escrow_id: u64) -> Option<EscrowDetails> {
    // Implementation...
}

// Example of structured event
#[neo_contract::event(sender: Address, recipient: Address, escrow_id: u64, amount: u64)]
struct EscrowCreated {}
```

## Contract Features

### 1. Time-Based Operations with Annotations

The contract uses `Ledger::current_timestamp()` with proper annotations:

```rust
// Create escrow with time lock
#[method]
#[no_reentry]
fn create_escrow(sender: Address, recipient: Address, amount: u64, lock_duration: u64) -> u64 {
    let current_time = Ledger::current_timestamp();
    let release_time = current_time + lock_duration;
    // Implementation...
}

// Check if time lock has expired
#[method]
#[safe]
fn can_release(escrow_id: u64) -> bool {
    let current_time = Ledger::current_timestamp();
    // Implementation...
}
```

### 2. Block-Based Operations with Annotations

The contract uses block height information with proper method annotations:

```rust
// Set dispute resolution deadline with method annotation
#[method]
#[no_reentry]
fn open_dispute(escrow_id: u64, reason: String) -> bool {
    let current_block = Ledger::current_index();
    let resolution_block = current_block + 100;
    // Implementation...
}

// Safe method to check confirmations
#[method]
#[safe]
fn check_confirmation_status(tx_hash: H256) -> u64 {
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        let current_height = Ledger::current_index();
        return current_height - tx_height + 1;
    }
    0
}
```

### 3. Transaction Validation with Annotations

The contract validates transactions with proper protection annotations:

```rust
// Verify transaction with re-entrancy protection
#[method]
#[no_reentry]
fn validate_and_process_payment_tx(tx_hash: H256, escrow_id: u64) -> bool {
    // Verify transaction exists and has sufficient confirmations
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        // Implementation...
    }
    false
}
```

### 4. Structured Contract Events

The contract emits structured events that clients can easily subscribe to:

```rust
// Events are clearly defined using annotation syntax
#[neo_contract::event(sender: Address, recipient: Address, escrow_id: u64, amount: u64)]
struct EscrowCreated {}

#[neo_contract::event(escrow_id: u64, amount: u64)]
struct EscrowReleased {}

#[neo_contract::event(escrow_id: u64, amount: u64)]
struct EscrowRefunded {}

#[neo_contract::event(escrow_id: u64, reason: String, resolution_block: u64)]
struct DisputeOpened {}
```

## Advantages of Annotation-Based Approach

1. **Clear API Definition**: Methods and events are clearly defined and exposed in the contract manifest
2. **Enhanced Security**: Built-in protections like `#[no_reentry]` and `#[safe]`
3. **Gas Optimization**: Proper marking of read-only methods reduces gas costs
4. **Improved Developer Experience**: Modern syntax familiar to Rust developers
5. **Better Client Integration**: Structured events make client applications more robust

## Testing

The contract includes comprehensive tests that demonstrate:
- Testing time-dependent logic using mock timestamps
- Testing block-dependent logic using mock block heights
- Testing transaction validation with mock transaction data

Run tests with:

```bash
cargo test
```

## Building the Contract

```bash
cargo build --release
```

## Documentation Resources

This project includes comprehensive documentation:

- [WORKFLOW.md](./WORKFLOW.md) - Visual explanation of escrow workflow
- [DEPLOYMENT.md](./DEPLOYMENT.md) - Guide for deploying the contract with annotations
- [NEP17_INTEGRATION.md](./NEP17_INTEGRATION.md) - Guide for token integration
- [CLIENT_INTERACTION.md](./CLIENT_INTERACTION.md) - Guide for client application interaction
- [SUMMARY.md](./SUMMARY.md) - Project overview and summary

## Tutorial

For a step-by-step guide on how this contract was built, refer to the [Ledger API Workshop](../../docs/ledger_api_workshop.md) documentation. 