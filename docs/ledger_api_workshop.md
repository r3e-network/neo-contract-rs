# Neo N3 Ledger API Workshop

This workshop guides you through building a practical smart contract that interacts with blockchain data using the Neo N3 Ledger API. By completing this workshop, you'll gain hands-on experience implementing common blockchain-dependent patterns.

## Workshop Overview

In this workshop, we'll build a **Blockchain-Powered Escrow Contract** that:

1. Creates time-locked escrow agreements
2. Validates transaction confirmations
3. Implements rate limiting
4. Uses block-based dispute resolution windows
5. Tracks and processes external transactions

Each section builds on the previous one, gradually introducing more advanced Ledger API concepts.

## Prerequisites

- Basic familiarity with Rust and Neo Contract development
- Development environment set up according to the [Installation Guide](./installation.md)
- Understanding of basic contract concepts as covered in [Contract Basics](./contract_basics.md)

## Table of Contents

- [Workshop Setup](#workshop-setup)
- [Part 1: Creating the Base Escrow Contract](#part-1-creating-the-base-escrow-contract)
- [Part 2: Implementing Time-Locked Releases](#part-2-implementing-time-locked-releases)
- [Part 3: Adding Transaction Validation](#part-3-adding-transaction-validation)
- [Part 4: Implementing Rate Limiting](#part-4-implementing-rate-limiting)
- [Part 5: Block-Based Dispute Resolution](#part-5-block-based-dispute-resolution)
- [Part 6: Testing the Contract](#part-6-testing-the-contract)
- [Workshop Exercises](#workshop-exercises)
- [Next Steps](#next-steps)

## Workshop Setup

Create a new contract project:

```bash
# Create a new directory for the workshop
mkdir ledger-workshop
cd ledger-workshop

# Create a new library crate
cargo new --lib escrow
cd escrow
```

Update your `Cargo.toml`:

```toml
[package]
name = "escrow"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { path = "path/to/neo-contract" }

[profile.release]
opt-level = 3
debug = false
lto = true
codegen-units = 1
panic = "abort"
overflow-checks = true
```

## Part 1: Creating the Base Escrow Contract

Let's start by creating a basic escrow contract structure that allows two parties to enter into an escrow agreement.

Create the file `src/lib.rs`:

```rust
use neo_contract::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Active,
    Completed,
    Refunded,
    Disputed
}

pub struct Escrow {
    // Contract owner
    owner: StorageItem<H160>,
    
    // Escrow details storage
    deposits: StorageMap<u64, u64>,  // escrow_id => amount
    senders: StorageMap<u64, H160>,  // escrow_id => sender
    recipients: StorageMap<u64, H160>,  // escrow_id => recipient
    statuses: StorageMap<u64, EscrowStatus>,  // escrow_id => status
    release_times: StorageMap<u64, u64>,  // escrow_id => release timestamp
    
    // Counter for escrow IDs
    next_escrow_id: StorageItem<u64>,
    
    // Processed transactions to prevent replay
    processed_txs: StorageMap<H256, bool>
}

impl Escrow {
    pub fn new(owner: H160) -> Self {
        Self {
            owner: StorageItem::new(owner),
            deposits: StorageMap::new(),
            senders: StorageMap::new(),
            recipients: StorageMap::new(),
            statuses: StorageMap::new(),
            release_times: StorageMap::new(),
            next_escrow_id: StorageItem::new(1),
            processed_txs: StorageMap::new()
        }
    }
    
    pub fn create_escrow(&mut self, sender: H160, recipient: H160, amount: u64) -> u64 {
        // Ensure sender is the one calling this method
        assert!(Runtime::check_witness(&sender), "Sender must initiate escrow");
        
        // Get and increment the escrow ID
        let escrow_id = self.next_escrow_id.get();
        self.next_escrow_id.set(escrow_id + 1);
        
        // Store escrow details
        self.deposits.insert(&escrow_id, amount);
        self.senders.insert(&escrow_id, sender);
        self.recipients.insert(&escrow_id, recipient);
        self.statuses.insert(&escrow_id, EscrowStatus::Active);
        
        // By default, set release time to current time (immediate release)
        // We'll modify this in Part 2
        let current_time = Ledger::current_timestamp();
        self.release_times.insert(&escrow_id, current_time);
        
        // Emit escrow creation event
        Runtime::notify(
            "EscrowCreated",
            &[
                &escrow_id,
                &sender,
                &recipient,
                &amount
            ]
        );
        
        escrow_id
    }
    
    pub fn release_escrow(&mut self, escrow_id: u64) -> bool {
        // Get escrow details
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is still active
            if status != EscrowStatus::Active {
                return false;
            }
            
            // Get sender
            let sender = self.senders.get(&escrow_id).expect("Sender not found");
            
            // Ensure sender is the one calling this method
            assert!(Runtime::check_witness(&sender), "Only sender can release escrow");
            
            // Update status
            self.statuses.insert(&escrow_id, EscrowStatus::Completed);
            
            // Emit escrow released event
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            let amount = self.deposits.get(&escrow_id).expect("Amount not found");
            
            Runtime::notify(
                "EscrowReleased",
                &[
                    &escrow_id,
                    &sender,
                    &recipient,
                    &amount
                ]
            );
            
            return true;
        }
        
        false
    }
    
    pub fn refund_escrow(&mut self, escrow_id: u64) -> bool {
        // Get escrow details
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is still active
            if status != EscrowStatus::Active {
                return false;
            }
            
            // Get recipient
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            
            // Ensure recipient is the one calling this method
            assert!(Runtime::check_witness(&recipient), "Only recipient can refund escrow");
            
            // Update status
            self.statuses.insert(&escrow_id, EscrowStatus::Refunded);
            
            // Emit escrow refunded event
            let sender = self.senders.get(&escrow_id).expect("Sender not found");
            let amount = self.deposits.get(&escrow_id).expect("Amount not found");
            
            Runtime::notify(
                "EscrowRefunded",
                &[
                    &escrow_id,
                    &sender,
                    &recipient,
                    &amount
                ]
            );
            
            return true;
        }
        
        false
    }
    
    pub fn get_escrow_details(&self, escrow_id: u64) -> Option<(H160, H160, u64, EscrowStatus, u64)> {
        // Check if escrow exists
        if let Some(status) = self.statuses.get(&escrow_id) {
            let sender = self.senders.get(&escrow_id)?;
            let recipient = self.recipients.get(&escrow_id)?;
            let amount = self.deposits.get(&escrow_id)?;
            let release_time = self.release_times.get(&escrow_id)?;
            
            return Some((sender, recipient, amount, status, release_time));
        }
        
        None
    }
}
```

### Explanation:

This initial contract sets up:

1. Storage structures for escrow agreements
2. Methods to create, release, and refund escrows
3. A basic query method to retrieve escrow details

We're using the `Ledger::current_timestamp()` method to get the current blockchain time, which is our first interaction with the Ledger API.

## Part 2: Implementing Time-Locked Releases

Now, let's enhance the contract by adding time-locked releases using the Ledger API's timestamp functionality.

Update the `create_escrow` method and add a new helper:

```rust
pub fn create_escrow(&mut self, sender: H160, recipient: H160, amount: u64, lock_duration: u64) -> u64 {
    // Ensure sender is the one calling this method
    assert!(Runtime::check_witness(&sender), "Sender must initiate escrow");
    
    // Get and increment the escrow ID
    let escrow_id = self.next_escrow_id.get();
    self.next_escrow_id.set(escrow_id + 1);
    
    // Store escrow details
    self.deposits.insert(&escrow_id, amount);
    self.senders.insert(&escrow_id, sender);
    self.recipients.insert(&escrow_id, recipient);
    self.statuses.insert(&escrow_id, EscrowStatus::Active);
    
    // Set release time to current time + lock duration
    let current_time = Ledger::current_timestamp();
    let release_time = current_time + lock_duration;
    self.release_times.insert(&escrow_id, release_time);
    
    // Emit escrow creation event
    Runtime::notify(
        "EscrowCreated",
        &[
            &escrow_id,
            &sender,
            &recipient,
            &amount,
            &release_time
        ]
    );
    
    escrow_id
}

// Add this helper method to check if an escrow is releasable
fn is_escrow_releasable(&self, escrow_id: u64) -> bool {
    if let Some(status) = self.statuses.get(&escrow_id) {
        // Check if escrow is still active
        if status != EscrowStatus::Active {
            return false;
        }
        
        // Check if release time has passed
        if let Some(release_time) = self.release_times.get(&escrow_id) {
            let current_time = Ledger::current_timestamp();
            return current_time >= release_time;
        }
    }
    
    false
}
```

Now, update the `release_escrow` method to use the time lock:

```rust
pub fn release_escrow(&mut self, escrow_id: u64) -> bool {
    // Get escrow details
    if let Some(status) = self.statuses.get(&escrow_id) {
        // Check if escrow is still active
        if status != EscrowStatus::Active {
            return false;
        }
        
        // Get sender
        let sender = self.senders.get(&escrow_id).expect("Sender not found");
        
        // Check if time lock has expired
        let release_time = self.release_times.get(&escrow_id).expect("Release time not found");
        let current_time = Ledger::current_timestamp();
        
        if current_time < release_time {
            // If sender is releasing early, require sender's witness
            assert!(Runtime::check_witness(&sender), "Time lock not expired, sender authorization required");
        } else {
            // If time lock has expired, either party can release
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            assert!(
                Runtime::check_witness(&sender) || Runtime::check_witness(&recipient),
                "Authorization required"
            );
        }
        
        // Update status
        self.statuses.insert(&escrow_id, EscrowStatus::Completed);
        
        // Emit escrow released event
        let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
        let amount = self.deposits.get(&escrow_id).expect("Amount not found");
        
        Runtime::notify(
            "EscrowReleased",
            &[
                &escrow_id,
                &sender,
                &recipient,
                &amount,
                &current_time
            ]
        );
        
        return true;
    }
    
    false
}
```

### Explanation:

We've enhanced the contract with:

1. Time-locked escrows by adding a `lock_duration` parameter
2. Release time validation using `Ledger::current_timestamp()`
3. Different authorization rules before and after the time lock expires

This demonstrates a common pattern in blockchain development: using blockchain time for automatic state transitions.

## Part 3: Adding Transaction Validation

Now let's add transaction validation to verify external transactions using the Ledger API.

Add the following method:

```rust
pub fn validate_and_process_payment_tx(&mut self, tx_hash: H256, escrow_id: u64) -> bool {
    // Check if transaction has already been processed
    if self.processed_txs.get(&tx_hash) {
        return false;
    }
    
    // Verify transaction exists and has sufficient confirmations
    const REQUIRED_CONFIRMATIONS: u32 = 3;
    
    if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
        let current_height = Ledger::current_index();
        
        // Check if transaction has enough confirmations
        if current_height < tx_height || current_height - tx_height + 1 < REQUIRED_CONFIRMATIONS {
            return false;
        }
        
        // Verify transaction execution succeeded
        if Ledger::get_transaction_vm_state(tx_hash) != 0 {
            return false;
        }
        
        // Get transaction details
        if let Some(tx) = Ledger::get_transaction(tx_hash) {
            // Verify the transaction is related to the escrow
            // In a real contract, you would verify the transaction content more thoroughly
            
            // Mark as processed to prevent replay
            self.processed_txs.insert(&tx_hash, true);
            
            // Update escrow status
            self.statuses.insert(&escrow_id, EscrowStatus::Completed);
            
            // Emit event
            Runtime::notify(
                "PaymentProcessed",
                &[
                    &escrow_id,
                    &tx_hash
                ]
            );
            
            return true;
        }
    }
    
    false
}
```

### Explanation:

This method:

1. Checks if a transaction has sufficient confirmations using `Ledger::get_transaction_height()`
2. Verifies the transaction execution state with `Ledger::get_transaction_vm_state()`
3. Retrieves transaction details using `Ledger::get_transaction()`
4. Prevents transaction replay by tracking processed transactions

This demonstrates how to securely validate external transactions before taking action in your contract.

## Part 4: Implementing Rate Limiting

Let's add rate limiting to prevent abuse of the contract using the Ledger API's timestamp functionality.

Add the following to your contract:

```rust
// Add these storage items to your Escrow struct
last_action_times: StorageMap<H160, u64>,  // address => last action timestamp
action_cooldown: StorageItem<u64>,  // cooldown period in seconds

// Initialize in new()
action_cooldown: StorageItem::new(300), // 5 minutes default cooldown

// Add this method to set/update the cooldown period
pub fn set_action_cooldown(&mut self, cooldown_seconds: u64) -> bool {
    // Only owner can set cooldown
    let owner = self.owner.get();
    assert!(Runtime::check_witness(&owner), "Only owner can set cooldown");
    
    self.action_cooldown.set(cooldown_seconds);
    return true;
}

// Add this helper method to check rate limits
fn check_rate_limit(&mut self, address: &H160) -> bool {
    let current_time = Ledger::current_timestamp();
    let cooldown = self.action_cooldown.get();
    
    if let Some(last_time) = self.last_action_times.get(address) {
        if current_time < last_time + cooldown {
            return false; // Still in cooldown period
        }
    }
    
    // Update last action time
    self.last_action_times.insert(address, current_time);
    true
}
```

Now update the `create_escrow` method to enforce rate limiting:

```rust
pub fn create_escrow(&mut self, sender: H160, recipient: H160, amount: u64, lock_duration: u64) -> u64 {
    // Ensure sender is the one calling this method
    assert!(Runtime::check_witness(&sender), "Sender must initiate escrow");
    
    // Check rate limit
    assert!(self.check_rate_limit(&sender), "Rate limit exceeded");
    
    // Rest of the method remains the same...
}
```

### Explanation:

We've implemented:

1. Rate limiting using blockchain timestamps
2. Configurable cooldown periods
3. Per-address action tracking

This shows how to use the Ledger API to implement common protection mechanisms against contract abuse.

## Part 5: Block-Based Dispute Resolution

Let's add a dispute resolution window based on block heights.

Add the following to your contract:

```rust
// Add these storage items to your Escrow struct
dispute_blocks: StorageMap<u64, u32>,  // escrow_id => dispute resolution block height

// Update the EscrowStatus enum to add a resolution phase
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Active,
    Completed,
    Refunded,
    Disputed,
    DisputeResolved
}

// Add this method to open a dispute
pub fn open_dispute(&mut self, escrow_id: u64) -> bool {
    // Get escrow details
    if let Some(status) = self.statuses.get(&escrow_id) {
        // Check if escrow is still active
        if status != EscrowStatus::Active {
            return false;
        }
        
        // Get escrow parties
        let sender = self.senders.get(&escrow_id).expect("Sender not found");
        let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
        
        // Ensure either party is opening the dispute
        assert!(
            Runtime::check_witness(&sender) || Runtime::check_witness(&recipient),
            "Only escrow parties can open disputes"
        );
        
        // Set dispute resolution deadline (current block + 100 blocks)
        let current_block = Ledger::current_index();
        let resolution_block = current_block + 100;
        self.dispute_blocks.insert(&escrow_id, resolution_block);
        
        // Update status
        self.statuses.insert(&escrow_id, EscrowStatus::Disputed);
        
        // Emit dispute event
        Runtime::notify(
            "DisputeOpened",
            &[
                &escrow_id,
                &current_block,
                &resolution_block
            ]
        );
        
        return true;
    }
    
    false
}

// Add this method for the owner to resolve a dispute
pub fn resolve_dispute(&mut self, escrow_id: u64, to_sender: bool) -> bool {
    // Ensure owner is resolving
    let owner = self.owner.get();
    assert!(Runtime::check_witness(&owner), "Only owner can resolve disputes");
    
    // Get escrow details
    if let Some(status) = self.statuses.get(&escrow_id) {
        // Check if escrow is in dispute
        if status != EscrowStatus::Disputed {
            return false;
        }
        
        // Check if dispute resolution period has begun
        if let Some(resolution_block) = self.dispute_blocks.get(&escrow_id) {
            let current_block = Ledger::current_index();
            
            // Cannot resolve before resolution block
            assert!(current_block >= resolution_block, "Dispute resolution period not reached");
            
            // Resolve dispute
            if to_sender {
                self.statuses.insert(&escrow_id, EscrowStatus::Refunded);
            } else {
                self.statuses.insert(&escrow_id, EscrowStatus::Completed);
            }
            
            // Emit resolution event
            let sender = self.senders.get(&escrow_id).expect("Sender not found");
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            
            Runtime::notify(
                "DisputeResolved",
                &[
                    &escrow_id,
                    &sender,
                    &recipient,
                    &to_sender
                ]
            );
            
            return true;
        }
    }
    
    false
}
```

### Explanation:

We've implemented:

1. Block-based dispute resolution windows using `Ledger::current_index()`
2. A waiting period measured in blocks rather than time
3. Resolution logic that only activates after the dispute period

This demonstrates using block heights for timing mechanisms, which can be more reliable than timestamps in some blockchain use cases.

## Part 6: Testing the Contract

Let's create a simple test that uses mocked Ledger API functionality to test our time and block-dependent logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::{TestBuilder, TestingContext};
    
    #[test]
    fn test_time_locked_escrow() {
        // Create test environment
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set initial timestamp
        test.ledger().set_current_timestamp(1000);
        
        // Create contract
        let owner = H160::from_slice(&[1; 20]);
        let sender = H160::from_slice(&[2; 20]);
        let recipient = H160::from_slice(&[3; 20]);
        
        let mut contract = Escrow::new(owner);
        
        // Create escrow with 1 hour lock
        test.as_signer(sender);
        let escrow_id = contract.create_escrow(sender, recipient, 100, 3600);
        
        // Try to release before lock expiration (should fail)
        let result = contract.release_escrow(escrow_id);
        assert!(!result);
        
        // Advance time by 2 hours
        test.ledger().advance_time(7200);
        
        // Now release should succeed
        let result = contract.release_escrow(escrow_id);
        assert!(result);
        
        // Check escrow status
        let (_, _, _, status, _) = contract.get_escrow_details(escrow_id).unwrap();
        assert_eq!(status, EscrowStatus::Completed);
    }
    
    #[test]
    fn test_block_based_dispute() {
        // Create test environment
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set initial block
        test.ledger().set_current_index(1000);
        
        // Create contract
        let owner = H160::from_slice(&[1; 20]);
        let sender = H160::from_slice(&[2; 20]);
        let recipient = H160::from_slice(&[3; 20]);
        
        let mut contract = Escrow::new(owner);
        
        // Create escrow with no time lock
        test.as_signer(sender);
        let escrow_id = contract.create_escrow(sender, recipient, 100, 0);
        
        // Open a dispute
        test.as_signer(recipient);
        let result = contract.open_dispute(escrow_id);
        assert!(result);
        
        // Try to resolve dispute immediately (should fail)
        test.as_signer(owner);
        let result = contract.resolve_dispute(escrow_id, false);
        assert!(!result);
        
        // Advance blocks
        test.ledger().advance_blocks(100);
        
        // Now resolution should succeed
        let result = contract.resolve_dispute(escrow_id, false);
        assert!(result);
        
        // Check escrow status
        let (_, _, _, status, _) = contract.get_escrow_details(escrow_id).unwrap();
        assert_eq!(status, EscrowStatus::Completed);
    }
}
```

### Explanation:

These tests:

1. Create a mocked blockchain environment
2. Test time-locked functionality by advancing the simulated timestamp
3. Test block-based functionality by advancing the block height
4. Verify that our contract's behavior matches expectations

This demonstrates how to test Ledger API functionality without needing a real blockchain, using the testing utilities from the Neo Contract framework.

## Workshop Exercises

Now that you've built a smart contract that uses the Ledger API, try these exercises to deepen your understanding:

1. **Transaction Tracking**: Add functionality to track a specific transaction's confirmations over time.

   ```rust
   // Hint: Create a method that checks if a transaction has reached a target confirmation count
   pub fn check_confirmation_status(&self, tx_hash: H256, target_confirmations: u32) -> (bool, u32) {
       // Your code here
   }
   ```

2. **Block Time Estimation**: Create a function to estimate when an escrow will be releasable in terms of blocks, based on average block time.

   ```rust
   // Hint: Implement this helper method
   pub fn estimate_release_block(&self, escrow_id: u64) -> Option<u32> {
       // Your code here
   }
   ```

3. **Advanced Rate Limiting**: Implement a tiered rate limiting system that allows different actions to have different cooldown periods.

   ```rust
   // Hint: Create an enum for action types and a mapping from action type to cooldown
   #[derive(Debug, Copy, Clone, PartialEq, Eq)]
   pub enum ActionType {
       CreateEscrow,
       OpenDispute,
       ResolveDispute
   }
   
   // Add this to your storage
   action_cooldowns: StorageMap<ActionType, u64>
   ```

## Next Steps

Congratulations on completing the workshop! Here are some next steps to further enhance your understanding:

1. **Explore the Full Ledger API**: Review the [Ledger API Cheat Sheet](./ledger_api_cheatsheet.md) to discover more functions.

2. **Study Best Practices**: Check out [Ledger API Best Practices](./ledger_api_best_practices.md) for tips on making your contracts more secure and efficient.

3. **Visual Learning**: Review the [Ledger API Diagrams](./ledger_api_diagram.md) to understand blockchain structures better.

4. **Complete Example**: Examine the [Ledger Example](../examples/ledger_example/) for a full implementation of a contract using the Ledger API.

5. **Advanced Testing**: Learn more about testing contracts that use the Ledger API in [Ledger API Testing](./ledger_api_testing.md).

By completing this workshop, you've gained practical experience implementing common blockchain-dependent patterns using the Ledger API. These patterns are foundational for building sophisticated, secure smart contracts on the Neo N3 blockchain. 