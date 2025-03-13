# Events and Notifications in Neo Smart Contracts

Events and notifications are crucial mechanisms for smart contracts to communicate with external systems and clients. This guide explains how to implement and use events in Neo smart contracts with the Neo Contract Rust Framework.

## Introduction to Neo Events

In Neo N3, smart contracts can emit notifications that external applications can subscribe to and process. These notifications are often referred to as "events" in the context of smart contract development, similar to Ethereum's events or Substrate's events.

Events serve several important purposes:
- Signal state changes to off-chain applications
- Provide searchable history of important contract actions
- Enable real-time monitoring of contract activities
- Support for decentralized applications (dApps) frontends

## Standardized Event Pattern

The Neo Contract Rust Framework provides a standardized approach to define and emit events in smart contracts. This approach simplifies event management and ensures consistent event formats across contracts.

### Event Definition

Events are defined as structs with the `#[event]` attribute:

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

Note the use of the `#[index]` attribute for fields that should be indexed for efficient filtering. This is equivalent to the `indexed` keyword in other blockchain platforms.

### Event Emission

Events are emitted using the `emit()` method on the event struct:

```rust
// Create and emit a Transfer event
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount: 100,
}.emit();
```

This approach is much cleaner and more type-safe than manually constructing event data arrays.

## Legacy Approach: Direct Runtime::notify

While the standardized event pattern above is recommended for new contracts, you may still see the direct `Runtime::notify` approach in older contracts:

```rust
// Legacy approach to emit events
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: u64) {
    // Create event name as ByteString
    let event_name = ByteString::from("Transfer");
    
    // Create an Array to hold parameters
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

## Event Patterns and Best Practices

### 1. Use the #[event] Attribute

Define events as structs with the `#[event]` attribute:

```rust
#[event]
struct CustomEvent {
    #[index]
    user: H160,
    action: String,
    value: u64,
}
```

### 2. Mark Fields for Indexing

Use the `#[index]` attribute to mark fields that should be indexed for efficient filtering:

```rust
#[event]
struct Approval {
    #[index]
    owner: H160,
    #[index]
    spender: H160,
    amount: u64,
}
```

### 3. Create Event Helper Methods (Optional)

For frequently used events, consider creating helper methods:

```rust
#[contractimpl]
impl MyToken {
    // Helper method to emit Transfer events
    fn transfer_event(&self, from: Option<H160>, to: Option<H160>, amount: u64) {
        Transfer {
            from,
            to,
            amount,
        }.emit();
    }
}
```

### 4. Follow NEP Standards for Event Names

For standard token contracts (NEP-17, NEP-11), use the event names specified in the standards:

- **NEP-17 (Fungible Token)**: "Transfer" event
- **NEP-11 (Non-Fungible Token)**: "Transfer" event (with token_id)

### 5. Consistent Structure for Custom Events

For custom events, maintain a consistent structure to simplify client-side handling:

```rust
#[event]
struct PoolCreated {
    #[index]
    pool_id: u32,
    token_a: Hash160, 
    token_b: Hash160,
    fee_rate: u16,
}
```

### 6. Documentation

Document the purpose of each event and its parameters in your contract code:

```rust
/// Event emitted when a user stakes tokens
#[event]
struct Staked {
    #[index]
    user: H160,
    amount: u64,
    timestamp: u64,
}
```

## Emitting Events

To emit an event from your contract, use the `emit()` method on the event struct:

```rust
// Create and emit a Transfer event
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount: 100,
}.emit();
```

You can also emit events conditionally:

```rust
if amount > 0 {
    Transfer {
        from: Some(sender),
        to: Some(receiver),
        amount,
    }.emit();
}
```

## Standard Events

Neo N3 has several standardized events that should be used for compatibility:

### NEP-17 Token Events

For fungible token contracts following the NEP-17 standard, the `Transfer` event is essential:

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

### NEP-11A Token Events (Divisible NFTs)

For divisible non-fungible token contracts:

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    token_id: ByteArray,
    amount: u64,
}
```

### NEP-11B Token Events (Non-divisible NFTs)

For non-divisible non-fungible token contracts:

```rust
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    token_id: ByteArray,
}
```

## Custom Events

You can define custom events for specific contract needs:

```rust
#[event]
struct UserRegistered {
    #[index]
    address: H160,
    username: String,
    registration_date: u64,
}

#[event]
struct ItemListed {
    #[index]
    item_id: ByteArray,
    #[index]
    seller: H160,
    price: u64,
    metadata: String,
}
```

## Event Serialization

Event fields are automatically serialized when emitted. The framework supports various types:

- Primitive types (u64, i32, bool, etc.)
- String and ByteArray
- Option<T> where T is a serializable type
- Address and other Neo-specific types
- Complex types that implement proper serialization

## Subscribing to Events

Events can be subscribed to in various ways:

### Using Neo RPC API

```javascript
// JavaScript example
const client = new neo.rpc.RPCClient("https://seed1.neo.org:10332");

// Subscribe to all Transfer events from a specific contract
const subscription = client.subscribe("notification_from_contract", [contractHash], (event) => {
    if (event.eventname === "Transfer") {
        console.log(`Transfer: ${event.state.value[0]} -> ${event.state.value[1]}, Amount: ${event.state.value[2]}`);
    }
});
```

### Using Neo SDK

```typescript
// TypeScript with neo-js example
import { rpc, u, wallet } from '@cityofzion/neo-js';

const rpcClient = new rpc.RPCClient("https://mainnet1.neo.org:10331");

async function getTransferEvents(contractHash, blocksBack = 100) {
    const currentHeight = await rpcClient.getBlockCount();
    const events = await rpcClient.getContractEvents({
        contractHash,
        eventName: 'Transfer',
        startHeight: currentHeight - blocksBack,
        endHeight: currentHeight
    });
    
    return events.map(event => ({
        from: event.state.value[0]?.value, 
        to: event.state.value[1]?.value,
        amount: parseInt(event.state.value[2]?.value || '0')
    }));
}
```

## Event Best Practices

1. **Be Consistent**: Use consistent event structures throughout your contract
2. **Documentation**: Document the purpose and structure of each event
3. **Minimal Data**: Include only necessary data in events to reduce gas costs
4. **Index Wisely**: Only mark fields as indexed when they need to be searched
5. **Standard Compliance**: Follow NEP standards for standard contract types
6. **Avoid Sensitive Data**: Don't include private or sensitive information in events
7. **Consider Gas Costs**: Emitting events consumes gas, so use them judiciously

## Advanced Techniques

### Versioned Events

For contract upgrades, consider versioning your events:

```rust
#[event]
struct TransferV1 {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    amount: u64,
}

#[event]
struct TransferV2 {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    amount: u64,
    memo: String,
}
```

### Event Aggregation

Aggregate related events for efficiency:

```rust
#[event]
struct BatchTransfer {
    #[index]
    operator: H160,
    transfers: Vec<TransferData>,
    timestamp: u64,
}
```

## Testing Events

When testing your contract, verify that events are emitted correctly:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract::prelude::testing::*;
    
    #[test]
    fn test_transfer_emits_event() {
        // Set up test environment
        let mut env = TestEnvironment::new();
        let owner = env.create_account([1u8; 20]);
        let recipient = env.create_account([2u8; 20]);
        
        // Create token contract
        let mut token = Token::new("Test".to_string(), "TST".to_string(), 8, 1000, owner.address());
        
        // Capture events
        env.event_recorder().start();
        
        // Execute transfer
        token.transfer(recipient.address(), 100);
        
        // Verify events
        let events = env.event_recorder().events();
        assert_eq!(events.len(), 1);
        
        let transfer_event = &events[0];
        assert_eq!(transfer_event.name, "Transfer");
        assert_eq!(transfer_event.fields[0].as_address(), Some(owner.address()));
        assert_eq!(transfer_event.fields[1].as_address(), Some(recipient.address()));
        assert_eq!(transfer_event.fields[2].as_u64(), Some(100));
    }
}
```

## Event Limitations

Be aware of these limitations when working with Neo events:

1. **Gas Costs**: Each event emission costs gas
2. **Size Limits**: Very large events may hit gas limits
3. **Indexing Limits**: Neo typically allows up to 3 indexed fields per event
4. **Off-Chain Reliability**: External systems might miss events if not properly configured

## Conclusion

Events are a powerful mechanism for smart contracts to communicate with the outside world. By effectively using events, you can create more transparent and interactive blockchain applications. The Neo Contract Rust Framework makes it easy to define, emit, and test events in your smart contracts.

Remember that events are part of the public blockchain data, so design your event system carefully, considering both usability for dApp developers and gas efficiency for contract users.