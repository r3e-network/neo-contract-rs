# Neo N3 Events and Notifications 

This guide explains how to use events and notifications in Neo N3 smart contracts using the Neo Contract Rust Framework.

## Introduction

Events and notifications are critical mechanisms for smart contracts to communicate with external applications. When a contract executes a transaction that changes state, it can emit events to notify interested parties about what happened.

## Event Structure

Events in Neo N3 are implemented as notifications containing:

1. An event name (as a ByteString)
2. An array of parameters (as Neo VM stack items)

## Event Definition

Events are defined as structs with the `#[neo_contract::event]` attribute:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}
```

### Event Emission

To emit an event with the proper structure:

```rust
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount: 100
}.notify();
```

This will automatically:

1. Convert the event name to a ByteString
2. Serialize the parameters correctly
3. Call the Neo VM notification system

## Event Parameters

Events can have any number of parameters with Neo-compatible types:

- Basic types: `bool`, `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `Int256`
- Container types: `H160`, `H256`, `ByteString`
- Optional types: `Option<T>` where T is any of the above
- Collections: `Vec<T>` for array parameters

### Indexed Parameters

Parameters marked with `#[index]` are indexed by the blockchain, allowing for efficient filtering:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]  // Indexed parameter - can be filtered efficiently
    pub from: Option<H160>,
    // Other parameters...
}
```

## Best Practices

### 1. Use the #[neo_contract::event] Attribute

Define events as structs with the `#[neo_contract::event]` attribute:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}
```

### 2. Use Descriptive Names

Event names should clearly describe what happened:

```rust
#[neo_contract::event]
pub struct Deposit {
    #[index]
    pub user: H160,
    pub amount: u64,
}

#[neo_contract::event]
pub struct Withdrawal {
    #[index]
    pub user: H160,
    pub amount: u64,
}
```

### 3. Index Important Parameters

Mark parameters that clients will filter by with `#[index]`:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]  // Commonly filtered by sender
    pub from: Option<H160>,
    #[index]  // Commonly filtered by receiver
    pub to: Option<H160>,
    pub amount: u64,  // Amount usually doesn't need indexing
}
```

### 4. Use Option for Nullable Parameters

For parameters that might be null (like in token minting/burning):

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,  // None when minting
    #[index]
    pub to: Option<H160>,    // None when burning
    pub amount: u64,
}
```

### 5. Create Helper Methods for Common Events

For frequently emitted events, create helper methods:

```rust
impl MyContract {
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: u64) {
        Transfer {
            from,
            to,
            amount,
        }.notify();
    }
    
    fn emit_deposit(&self, user: H160, amount: u64) {
        Deposit {
            user,
            amount,
        }.notify();
    }
}
```

## Standard Event Patterns

### NEP-17 Transfer Event

For token transfers in NEP-17 compliant contracts:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,  // None for minting
    #[index]
    pub to: Option<H160>,    // None for burning
    pub amount: u64,
}

// Usage examples:

// Regular transfer
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount,
}.notify();

// Minting tokens
Transfer {
    from: None,
    to: Some(recipient),
    amount,
}.notify();

// Burning tokens
Transfer {
    from: Some(owner),
    to: None,
    amount,
}.notify();
```

### NEP-11 Transfer Event

For NFT transfers in NEP-11 compliant contracts:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,  // None for minting
    #[index]
    pub to: Option<H160>,    // None for burning
    #[index]
    pub token_id: ByteString,
    pub amount: u64,         // Usually 1 for NFTs
}
```

## Complex Event Examples

### Multi-Party Events

For events involving multiple parties:

```rust
#[neo_contract::event]
pub struct Swap {
    #[index]
    pub user: H160,
    #[index]
    pub pool_id: ByteString,
    pub token_in: H160,
    pub amount_in: u64,
    pub token_out: H160,
    pub amount_out: u64,
}
```

### State Change Events

For capturing state changes:

```rust
#[neo_contract::event]
pub struct PriceUpdated {
    #[index]
    pub token: H160,
    pub old_price: u64,
    pub new_price: u64,
    pub timestamp: u64,
}
```

### Governance Events

For governance actions:

```rust
#[neo_contract::event]
pub struct ProposalCreated {
    #[index]
    pub proposer: H160,
    #[index]
    pub proposal_id: u64,
    pub description: ByteString,
    pub voting_ends: u64,
}

#[neo_contract::event]
pub struct VoteCast {
    #[index]
    pub voter: H160,
    #[index]
    pub proposal_id: u64,
    pub support: bool,
    pub votes: u64,
}
```

## Low-Level Event API

While the high-level event API is recommended, you can also use the low-level API directly:

```rust
pub fn emit_transfer_manual(from: Option<H160>, to: Option<H160>, amount: u64) {
    // Create event name as ByteString
    let event_name = ByteString::from("Transfer");
    
    // Create array for event parameters
    let mut args = Array::<Any>::new();
    
    // Add parameters
    match from {
        Some(addr) => args.push(Any::from(addr)),
        None => args.push(Any::new()),  // Push null value
    }
    
    match to {
        Some(addr) => args.push(Any::from(addr)),
        None => args.push(Any::new()),  // Push null value
    }
    
    args.push(Any::from(amount));
    
    // Emit event
    Runtime::notify(&event_name, &args);
}
```

## Monitoring Events Off-Chain

To listen for events from your Neo N3 contract:

1. Use the Neo RPC client's `getapplicationlog` method
2. Filter for the specific events you need
3. Process the notification data based on your event's structure

Example RPC call (after a transaction):

```javascript
{
    "jsonrpc": "2.0",
    "method": "getapplicationlog",
    "params": ["0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"],
    "id": 1
}
```

## Conclusion

Events are a critical component of Neo N3 smart contracts. Using the `#[neo_contract::event]` attribute and proper event emission patterns will make your contracts more maintainable and easier to integrate with external applications.