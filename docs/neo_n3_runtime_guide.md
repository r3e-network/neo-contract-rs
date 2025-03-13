# Neo N3 Runtime Guide

This guide explains how to properly use the Neo N3 Runtime API for Neo smart contract development in Rust. The Runtime provides essential functionality for interacting with the Neo N3 blockchain environment.

## Runtime Overview

The Neo N3 Runtime module provides access to the blockchain execution environment, allowing smart contracts to:

1. Emit events (notifications)
2. Verify transaction signatures
3. Get execution context information 
4. Access time and blockchain data
5. Log information for debugging

## Event Emission

### Proper Event Pattern for Neo N3

Events in Neo N3 must be emitted using the `Runtime::notify` method, following this pattern:

```rust
pub fn emit_event(param1: H160, param2: u64) {
    // 1. Create event name as ByteString
    let event_name = ByteString::from("EventName");
    
    // 2. Create an Array to hold parameters
    let mut event_data = Array::<Any>::new();
    
    // 3. Add parameters, converting to Any type
    event_data.push(Any::from(param1));
    event_data.push(Any::from(param2));
    
    // 4. Emit the event using Runtime::notify
    Runtime::notify(&event_name, &event_data);
}
```

### Handling Null Values in Events

For optional parameters (nullable values), use this approach:

```rust
match optional_param {
    Some(value) => event_data.push(Any::from(value)),
    None => event_data.push(Any::new()), // Proper Neo N3 null representation
}
```

### Event Structure Recommendation

For better code organization, define an event structure and implement an `emit` method:

```rust
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
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
}
```

## Transaction Authentication

### Verifying Transaction Signers

Always verify that the transaction sender has authorized operations that modify state:

```rust
// Check if address has authorized this transaction
if !Runtime::check_witness(&sender) {
    panic!("No authorization");
}
```

### Multi-signature Verification

For operations requiring multiple signatures:

```rust
// Check that all required signatures are present
if !Runtime::check_witness(&owner1) || !Runtime::check_witness(&owner2) {
    panic!("Multi-signature verification failed");
}
```

## Execution Context

### Getting the Calling Contract

To identify which contract or account is calling your contract:

```rust
let caller = Runtime::calling_script_hash();
```

### Getting the Current Contract

To get the current contract's script hash:

```rust
let current = Runtime::executing_script_hash();
```

## Time and Random Numbers

### Getting Current Timestamp

```rust
let timestamp = Runtime::time();
```

### CAUTION: Random Number Generation

Neo N3 does not provide a secure random number generator. For random-like behavior:

```rust
// WARNING: This is NOT cryptographically secure
let pseudo_random = Runtime::time() % 100;
```

For applications requiring true randomness, consider using commit-reveal schemes or off-chain oracles.

## Blockchain Interaction

### Checking Current Blockchain Height

```rust
let height = Runtime::current_block_index();
```

### Getting Transaction Information

```rust
let tx_height = Runtime::transaction_height(tx_hash);
```

## Logging and Debugging

### Logging for Development

```rust
Runtime::log("Debug message");
```

Note: Logs are only visible during development and testing, not in production blockchain environments.

## Triggering Contract Execution

### Determining Trigger Type

```rust
let trigger = Runtime::trigger();
if trigger == TriggerType::Application {
    // Contract is being called directly
} else if trigger == TriggerType::Verification {
    // Contract is being used to verify a transaction
}
```

## Gas Management

### Getting Remaining Gas

```rust
let gas_left = Runtime::gas_left();
```

### Gas Optimization Tips

1. Mark read-only methods with `#[safe]`
2. Minimize storage operations
3. Use appropriate data structures
4. Pre-compute values when possible
5. Avoid recursive functions or complex loops

## Contract Upgrades and Management

### Contract Update/Upgrade

```rust
// Only contract owner should be able to upgrade
assert!(Runtime::check_witness(&owner), "Not authorized");
Contract::update(nef, manifest);
```

## Best Practices

1. **Always use event emission pattern**: Follow the Neo N3 standard for notifying the blockchain about important state changes.
2. **Check authorization**: Call `check_witness` before any state-changing operations.
3. **Properly handle null parameters**: Use `Any::new()` for null values in events.
4. **Mark methods as safe**: Use the `#[safe]` attribute for read-only methods.
5. **Limit storage operations**: Minimize storage reads/writes to reduce gas costs.
6. **Document event structures**: Clearly document the expected parameters and types for each event.
7. **Use appropriate error messages**: Include descriptive error messages in assertions.

## Safe Methods Optimization

Neo N3 allows methods to be marked as `safe`, meaning they are read-only and do not modify the contract state. Safe methods have the following characteristics:

1. Use the `#[safe]` attribute for read-only methods
2. `#[safe]` implicitly makes the method available in the contract interface without needing an additional `#[method]` annotation
3. Safe methods appear in the contract manifest with `"safe": true`
4. Neo N3 nodes can optimize execution of safe methods

Example of a properly annotated safe method:

```rust
// Correct - only #[safe] is needed
#[safe]
pub fn balance_of(&self, account: H160) -> u64 {
    self.balances.get(&account).unwrap_or_default()
}

// INCORRECT - redundant annotation, use only #[safe]
// #[method]
// #[safe]
// pub fn balance_of(&self, account: H160) -> u64 {
//    self.balances.get(&account).unwrap_or_default()
// }
```

Note that the `#[safe]` attribute should only be used for methods that:
- Don't modify contract storage
- Don't call other state-changing methods
- Don't emit events (as event emission is considered a side effect)

## Common Issues and Solutions

### Issue: Events not visible in blockchain explorers

**Solution**: Ensure you're using `Runtime::notify` with properly formatted ByteString event names and Array<Any> parameters.

### Issue: Transaction verification failures

**Solution**: Verify that `check_witness` is properly implemented and that the signer has the correct permissions.

### Issue: High gas costs

**Solution**: Mark read-only methods as `#[safe]`, minimize storage operations, and optimize your code.

### Issue: Null parameters not properly handled

**Solution**: Use `Any::new()` for null values, not empty strings or zero values.

## Example: Standard NEP-17 Events

The following code demonstrates the standard events required for NEP-17 token implementation:

```rust
// Transfer event implementation
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
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
}
```

## Conclusion

The Neo N3 Runtime provides essential functionality for Neo smart contracts. By following the patterns and best practices outlined in this guide, you can ensure your contracts are properly integrated with the Neo N3 blockchain environment.

For more detailed information, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Official Neo N3 Documentation](https://docs.neo.org/docs/en-us/index.html)
