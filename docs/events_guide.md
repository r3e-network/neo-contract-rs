# Neo Contract Events Guide

This guide explains how to define, emit, and handle events in Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Events in Neo N3 smart contracts provide a way to log information and notify external applications about contract state changes. The neo-contract-rs framework provides an annotation-based approach to define events with type safety and automatic ABI generation.

## Defining Events

Use the `#[event]` annotation to define contract events:

```rust
use neo_contract::prelude::*;

#[event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index] 
    pub to: Option<H160>,
    pub amount: u64,
}

#[event]
pub struct Approval {
    #[index]
    pub owner: H160,
    #[index]
    pub spender: H160,
    pub amount: u64,
}
```

### Event Field Attributes

- **`#[index]`**: Marks a field as indexed, allowing efficient filtering and searching
- **Optional fields**: Use `Option<T>` for fields that might be null (like `from` in minting operations)

## Emitting Events

### Using Runtime::notify (Recommended)

```rust
use neo_contract::prelude::*;

impl MyContract {
    #[method]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // ... transfer logic ...
        
        // Emit Transfer event
        let mut event_args = Array::new();
        event_args.push(Any::from(from));
        event_args.push(Any::from(to));
        event_args.push(Any::from(amount));
        
        Runtime::notify(&ByteString::from("Transfer"), &event_args);
        
        true
    }
}
```

### Using the Event Macro (if available)

```rust
// If using declarative macros from neo-macros-core
emit_event!("Transfer", from, to, amount);
```

## Event Best Practices

### 1. Consistent Naming

Use consistent event names across your contracts:
- `Transfer` for token transfers
- `Approval` for approvals
- `Mint` for token creation
- `Burn` for token destruction

### 2. Indexed Fields

Choose indexed fields carefully:
- Typically index addresses (`H160`) for filtering by user
- Index important identifiers that will be searched
- Limit indexed fields (Neo N3 allows up to 3 indexed fields per event)

### 3. Event Documentation

Document your events clearly:

```rust
/// Emitted when tokens are transferred between accounts
#[event]
pub struct Transfer {
    /// The account sending tokens (None for minting)
    #[index]
    pub from: Option<H160>,
    
    /// The account receiving tokens
    #[index]
    pub to: Option<H160>,
    
    /// The amount of tokens transferred
    pub amount: u64,
}
```

## Event Types and Neo N3 Integration

### Standard Events

For NEP-17 tokens:
```rust
#[event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}
```

For NEP-11 NFTs:
```rust
#[event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    #[index]
    pub tokenId: ByteString,
}
```

### Custom Events

```rust
#[event]
pub struct PriceUpdate {
    #[index]
    pub asset: H160,
    pub old_price: u64,
    pub new_price: u64,
    pub timestamp: u64,
}

#[event]
pub struct GovernanceVote {
    #[index]
    pub voter: H160,
    #[index]
    pub proposal_id: u64,
    pub vote: bool, // true for yes, false for no
    pub voting_power: u64,
}
```

## Event Filtering and Querying

External applications can filter events using RPC calls:

```javascript
// Filter Transfer events by sender
await neo.getApplicationLog(txid, {
    eventName: "Transfer",
    from: "0x1234567890abcdef1234567890abcdef12345678"
});
```

## Contract Manifest Integration

Events defined with `#[event]` are automatically included in the contract manifest, providing:
- Event signatures for ABI generation
- Parameter names and types
- Indexed field information

## Error Handling

Events should not be used for error reporting. Use `panic!` or return error codes for error handling:

```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    if amount == 0 {
        return false; // Don't emit event for failed transfers
    }
    
    // ... successful transfer logic ...
    
    // Emit event only on success
    Runtime::notify(&ByteString::from("Transfer"), &transfer_args);
    true
}
```

## Testing Events

Test event emission in your contract tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transfer_emits_event() {
        let mut contract = MyContract::new();
        
        // Perform transfer
        let result = contract.transfer(alice, bob, 100);
        
        assert!(result);
        // Verify event was emitted (implementation depends on test framework)
    }
}
```

## Performance Considerations

- Events have gas costs in Neo N3
- Minimize event data size where possible
- Use indexed fields judiciously (they cost more gas)
- Consider batching related events when appropriate

## See Also

- [Contract Organization](../neo-contract/CONTRACTS.md)
- [Examples](../examples/README.md)
- [Macro Architecture](../neo-contract/MACROS.md) 