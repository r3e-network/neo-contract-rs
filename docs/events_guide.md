# Neo N3 Events and Notifications Guide

This guide covers how to effectively use events and notifications in Neo N3 smart contracts developed with the Neo Contract Rust framework.

## Introduction

Events and notifications are essential mechanisms for smart contracts to communicate with external applications. They allow contracts to signal important state changes and provide data that can be indexed and monitored by off-chain applications.

In Neo N3, events are implemented through notifications that contracts emit during execution. These notifications can be observed by clients monitoring the blockchain, enabling responsive and event-driven decentralized applications.

## Standardized Event Pattern

The Neo Contract Rust Framework provides a standardized pattern for defining and emitting events which improves code readability and type safety.

### Event Definition with #[event] Attribute

The recommended way to define events is using Rust structs with the `#[event]` attribute:

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    amount: u64,
}
```

Benefits of this approach:
- **Type Safety**: The compiler ensures your event data has the correct types
- **Code Readability**: Event structure is clearly defined
- **Maintainability**: Changes to event structure are easier to track
- **Consistency**: Standardized approach across all Neo N3 contracts

### Indexing Event Parameters

Use the `#[index]` attribute to mark event parameters that should be indexed for efficient filtering:

```rust
#[event]
struct Approval {
    #[index]  // Indexed for filtering - equivalent to Solidity's "indexed"
    owner: H160,
    #[index]  // Indexed for filtering
    spender: H160,
    amount: u64,  // Not indexed
}
```

Indexed fields allow clients to efficiently filter and query events without processing all event data.

### Emitting Events

To emit an event using the standardized pattern:

```rust
// Create and emit a Transfer event
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount: 100,
}.emit();
```

## Legacy Event Emission Method

While the standardized event pattern is recommended, you may encounter the direct `Runtime::notify` approach in older code:

```rust
// Legacy way to emit events
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: u64) {
    let event_name = ByteString::from("Transfer");
    let mut event_data = Array::<Any>::new();
    
    // Add parameters as Any values
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    event_data.push(Any::from(amount));
    
    // Emit the event
    Runtime::notify(&event_name, &event_data);
}
```

When migrating to the standardized pattern, you should replace such implementations with proper event structs.

## Standard Token Events

### NEP-17 (Fungible Token) Events

NEP-17 compliant contracts should include a standard Transfer event:

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,  // None for minting
    #[index]
    to: Option<H160>,    // None for burning
    amount: u64,
}
```

### NEP-11 (Non-Fungible Token) Events

NEP-11 compliant contracts should include Transfer events with token ID:

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,  // None for minting
    #[index]
    to: Option<H160>,    // None for burning
    #[index]
    token_id: ByteString,
    amount: u64,         // Usually Int256::one() for NFTs
}
```

## Helper Methods for Event Emission

For frequently emitted events, consider creating helper methods to reduce code duplication and improve readability:

```rust
impl MyContract {
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: u64) {
        Transfer {
            from,
            to,
            amount,
        }.emit();
    }
}
```

Then you can call this helper method from your contract functions:

```rust
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Business logic
    
    // Emit event
    self.emit_transfer(Some(from), Some(to), amount);
    
    true
}
```

## Advanced Event Usage

### Events with Structured Data

For complex events, structure your data logically:

```rust
#[event]
struct SwapExecuted {
    #[index]
    user: H160,
    #[index]
    pool_id: u32,
    token_in: H160,
    token_out: H160,
    amount_in: u64,
    amount_out: u64,
    timestamp: u64,
}
```

### Events with Collection Types

You can use collection types in your events for batch operations:

```rust
#[event]
struct BatchOperation {
    #[index]
    operator: H160,
    addresses: Vec<H160>,
    values: Vec<u64>,
    timestamp: u64,
}
```

## Best Practices

### 1. Follow Standardized Pattern

Use the `#[event]` attribute for all event definitions and the `.emit()` method for emission.

### 2. Be Consistent with Event Names

Use consistent naming conventions for events. Common patterns:
- Use past tense verbs for state changes: `Deposited`, `Withdrawn`, `Transferred`
- Use descriptive nouns for entity creation: `PoolCreated`, `AccountRegistered`

### 3. Index Important Fields

Mark fields that users would likely filter on with the `#[index]` attribute:
- Addresses (users, accounts)
- IDs (token_id, pool_id)
- Action types (when represented as enums or integers)

### 4. Document Your Events

Add documentation comments to your event definitions explaining their purpose:

```rust
/// Event emitted when a user stakes tokens in the platform
#[event]
struct Staked {
    #[index]
    user: H160,
    amount: u64,
    timestamp: u64,
}
```

### 5. Consider Client-Side Needs

Design your events with the needs of client applications in mind:
- Include enough information for clients to update their state
- Avoid excessive data that could increase gas costs

### 6. Create Event Helper Methods

For commonly used events, create helper methods to promote code reuse and consistency.

## Example Contract with Standardized Events

Here's a complete example showing the standardized event pattern in a simple token contract:

```rust
#![no_std]
extern crate alloc;

use neo_contract::prelude::*;

#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    amount: u64,
}

#[event]
struct Approval {
    #[index]
    owner: H160,
    #[index]
    spender: H160,
    amount: u64,
}

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Simple Token with Standardized Events")]
mod token_contract {
    use super::*;
    
    #[storage]
    pub struct TokenContract {
        balances: StorageMap<H160, u64>,
        total_supply: StorageItem<u64>,
        allowances: StorageMap<(H160, H160), u64>,
    }
    
    #[contractimpl]
    impl TokenContract {
        #[method]
        pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            let from_balance = self.balances.get(&from).unwrap_or(0);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            self.balances.set(&from, &(from_balance - amount)).unwrap();
            let to_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.set(&to, &(to_balance + amount)).unwrap();
            
            // Emit event using the standardized pattern
            Transfer {
                from: Some(from),
                to: Some(to),
                amount,
            }.emit();
            
            true
        }
        
        #[method]
        pub fn approve(&mut self, owner: H160, spender: H160, amount: u64) -> bool {
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Update allowance
            self.allowances.set(&(owner, spender), &amount).unwrap();
            
            // Emit approval event
            Approval {
                owner,
                spender,
                amount,
            }.emit();
            
            true
        }
    }
}
```

## Conclusion

The standardized event pattern with `#[event]` attribute and `EventName::emit()` method is the recommended approach for Neo N3 smart contracts. This pattern improves code readability, ensures type safety, and provides a more maintainable way to handle events in your contracts.

For examples of this pattern in action, refer to the example contracts in the repository, such as the event_demo, secure_vault, and dex contracts.