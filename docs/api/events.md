# Event System in Neo N3 Contracts

This document describes the event emission system for Neo N3 smart contracts developed with the Rust framework.

## Overview

Events in Neo N3 smart contracts are notifications emitted during contract execution that can be observed by external applications. They provide a way for dApps to track important state changes without requiring direct blockchain queries.

In the Neo N3 contract framework for Rust, events are emitted using the `Runtime::notify` method, which is now simplified with the `#[event]` attribute.

## Event Structure

Neo N3 events consist of:

1. An event name (as a `ByteString`)
2. An array of parameters (as `Array<Any>`)

## Standardized Event Pattern with #[event] Attribute

The recommended approach for defining and emitting events in Neo N3 contracts is to use the `#[event]` attribute on a struct:

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    amount: u64,
}
```

The `#[event]` attribute automatically generates an `emit` method that follows Neo N3 standards for event emission. The `#[index]` attribute marks fields that should be indexed for efficient querying.

### Emitting Events

With the `#[event]` attribute, you can emit events using the static `emit` method:

```rust
// Emit a transfer event
Transfer::emit(Some(sender), Some(recipient), amount);
```

Under the hood, the generated code:

1. Creates a `ByteString` with the event name (matching the struct name)
2. Creates an `Array<Any>` to hold event parameters
3. Converts parameters to `Any` type with proper null handling for Option types
4. Calls `Runtime::notify(event_name, event_params)` to emit the event

This standardized approach ensures consistency and reduces boilerplate code.

## Manual Event Emission

For cases where you need more control, you can still manually emit events using the underlying Neo N3 API:

```rust
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {
    // Create event name as ByteString
    let event_name = ByteString::from("Transfer");
    
    // Create an Array to hold parameters
    let mut event_data = Array::<Any>::new();
    
    // Add parameters as Any values
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),  // No null() method, use Any::new()
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

## Best Practices for Events

1. **Use the `#[event]` attribute** when defining events to ensure standardized emission.
2. **Mark indexed fields** with the `#[index]` attribute to enable efficient querying.
3. **Be consistent with event names** by following common conventions (e.g., "Transfer", "Approval").
4. **Keep event parameters minimal** to reduce gas costs while ensuring sufficient information.

## Standard Events

### NEP-17 Token Events

NEP-17 token contracts should emit the following events:

1. **Transfer**: When tokens are transferred between addresses
   ```
   Transfer(from: H160|null, to: H160|null, amount: Int256)
   ```

### NEP-11 Token Events

NEP-11 token contracts should emit the following events:

1. **Transfer**: When tokens are transferred between addresses
   ```
   Transfer(from: H160|null, to: H160|null, tokenId: ByteString, amount: Int256)
   ```

## Examples

### NEP-17 Token Transfer Event

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    amount: u64,
}

// In your transfer method:
fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    // ... transfer logic ...
    
    // Emit the event
    Transfer::emit(Some(from), Some(to), amount);
    
    true
}
```

### Auction Contract Events

```rust
#[event]
struct BidPlaced {
    #[index]
    auction_id: u32,
    #[index]
    bidder: Address,
    amount: u64,
    timestamp: u64,
}

#[event]
struct AuctionCompleted {
    #[index]
    auction_id: u32,
    winner: Address,
    final_price: u64,
}

// In your bid method:
fn place_bid(&mut self, auction_id: u32, amount: u64) -> bool {
    // ... bid logic ...
    
    // Emit the event
    BidPlaced::emit(auction_id, bidder, amount, timestamp);
    
    true
}
```

## Low-Level Details

Under the hood, `Runtime::notify` calls the Neo VM syscall `System.Runtime.Notify` with the event name and parameters. The Neo N3 blockchain then logs this event in its execution notifications.

## Subscribing to Events

Client applications can subscribe to events using the Neo N3 RPC method `subscribeToEvents`. They can filter events by:

- Contract hash
- Event name
- Parameter values (for indexed parameters)

## Testing Events

The framework provides utilities for testing event emissions:

```rust
#[test]
fn test_transfer_event() {
    let mut env = TestEnvironment::new();
    let contract = env.deploy_contract::<token::TokenContract>();
    
    // Execute contract method
    let from = Address::from_public_key(&[1; 33]);
    let to = Address::from_public_key(&[2; 33]);
    contract.transfer(from, to, 100);
    
    // Verify event was emitted
    let events = env.get_events();
    assert_eq!(events.len(), 1);
    
    let event = &events[0];
    assert_eq!(event.name, "Transfer");
    assert_eq!(event.params.len(), 3);
    
    // Check parameters
    assert_eq!(event.params[0].as_address(), Some(from));
    assert_eq!(event.params[1].as_address(), Some(to));
    assert_eq!(event.params[2].as_int(), Some(100));
}
```
