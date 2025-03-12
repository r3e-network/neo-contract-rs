# Events and Notifications in Neo Smart Contracts

Events and notifications are crucial mechanisms for smart contracts to communicate with external systems and clients. This guide explains how to implement and use events in Neo smart contracts with the Neo Contract Rust Framework.

## Introduction to Neo Events

In Neo N3, smart contracts can emit notifications that external applications can subscribe to and process. These notifications are often referred to as "events" in the context of smart contract development, similar to Ethereum's events or Substrate's events.

Events serve several important purposes:
- Signal state changes to off-chain applications
- Provide searchable history of important contract actions
- Enable real-time monitoring of contract activities
- Support for decentralized applications (dApps) frontends

## Events in Neo N3

In Neo N3, events are properly emitted using the `Runtime::notify` method rather than using an attribute-based struct approach. This direct approach aligns with Neo N3's architecture and provides better compatibility with the Neo VM.

### Event Structure

Events in Neo N3 consist of:

1. An event name (as a ByteString)
2. An array of parameters (as Array<Any>)

Each parameter can be of any type that can be converted to the `Any` type.

### Parameter Types

Event parameters can include various types of data:

- Addresses (`H160`)
- Numbers (integers, etc.)
- Strings (as `ByteString`)
- Boolean values
- Byte arrays
- Null/None values (represented as empty `Any`)

### Searchability

In Neo N3, the first argument in the event data array is typically used for filtering and searching. For better searchability:

- Place identifiable fields like addresses or IDs first in the event data array
- Use consistent event names across your contract
- Limit the number of parameters to what's essential

## Emitting Events

To emit an event from your contract, use the `Runtime::notify` method with a properly formatted event name and data array:

```rust
// Inside a contract method
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: u64) {
    // Create event name as ByteString
    let event_name = ByteString::from("Transfer");
    
    // Create an Array to hold parameters
    let mut event_data = Array::<Any>::new();
    
    // Add parameters as Any values
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // For null values
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

You can also emit events conditionally:

```rust
if amount > 0 {
    let event_name = ByteString::from("Transfer");
    let mut event_data = Array::<Any>::new();
    
    // Add parameters
    event_data.push(Any::from(from));
    event_data.push(Any::from(to));
    event_data.push(Any::from(amount));
    
    Runtime::notify(&event_name, &event_data);
}
```

## Standard Events

Neo N3 has several standardized events that should be used for compatibility:

### NEP-17 Token Events

For fungible token contracts following the NEP-17 standard, the `Transfer` event is essential:

```rust
// Function to emit a NEP-17 compliant Transfer event
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
    
    Runtime::notify(&event_name, &event_data);
}
```

### NEP-11A Token Events (Divisible NFTs)

For divisible non-fungible token contracts:

```rust
#[event]
pub struct Transfer {
    #[indexed]
    from: Option<Address>,
    #[indexed]
    to: Option<Address>,
    #[indexed]
    token_id: ByteArray,
    amount: u64,
}
```

### NEP-11B Token Events (Non-divisible NFTs)

For non-divisible non-fungible token contracts:

```rust
#[event]
pub struct Transfer {
    #[indexed]
    from: Option<Address>,
    #[indexed]
    to: Option<Address>,
    #[indexed]
    token_id: ByteArray,
}
```

## Custom Events

You can define custom events for specific contract needs:

```rust
#[event]
pub struct UserRegistered {
    #[indexed]
    address: Address,
    username: String,
    registration_date: u64,
}

#[event]
pub struct ItemListed {
    #[indexed]
    item_id: ByteArray,
    #[indexed]
    seller: Address,
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
pub struct TransferV1 {
    #[indexed]
    from: Option<Address>,
    #[indexed]
    to: Option<Address>,
    amount: u64,
}

#[event]
pub struct TransferV2 {
    #[indexed]
    from: Option<Address>,
    #[indexed]
    to: Option<Address>,
    amount: u64,
    memo: String,
}
```

### Event Aggregation

Aggregate related events for efficiency:

```rust
#[event]
pub struct BatchTransfer {
    #[indexed]
    operator: Address,
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