# Neo N3 Event Demo Contract

This example demonstrates the proper way to emit events in Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Event handling in Neo N3 is a critical aspect of smart contract development. Events serve as a way for contracts to communicate with external systems, providing a record of important state changes and actions that occur during contract execution.

## Key Concepts

### 1. Event Structure

In Neo N3, it's recommended to define event structures for better code organization:

```rust
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        // Event emission implementation
    }
}
```

This pattern helps organize your code and makes event emission more maintainable.

### 2. Proper Neo N3 Event Emission

In Neo N3, events are properly emitted using the `Runtime::notify` method with the following pattern:

1. Create the event name as a `ByteString`
2. Create an `Array<Any>` to hold event parameters
3. Convert parameters to `Any` type with `Any::from()`
4. Use `Runtime::notify(event_name, event_params)` to emit the event

For null or None values, use `Any::new()` - this is the correct way to represent null values in Neo N3 events.

Example implementation:

```rust
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: u64) {
    // STEP 1: Create event name as ByteString
    let event_name = ByteString::from("Transfer");
    
    // STEP 2: Create an Array to hold parameters
    let mut event_data = Array::<Any>::new();
    
    // STEP 3: Add parameters as Any values, handling None values properly
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Proper Neo N3 null representation
    }
    
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Proper Neo N3 null representation
    }
    
    // Regular values are converted to Any using From trait
    event_data.push(Any::from(amount));
    
    // STEP 4: Emit the event using Runtime::notify
    Runtime::notify(&event_name, &event_data);
}
```

### 3. Example Events

This demo includes several event types to demonstrate various Neo N3 event patterns:

#### Basic Transfer Event

The standard Transfer event follows the NEP-17 pattern with from, to, and amount parameters:

```rust
// Emit a transfer event
Transfer::emit(Some(sender), Some(recipient), amount);
```

#### Approval Event

Used for authorization scenarios:

```rust
// Emit an approval event
Approval::emit(owner, spender, amount);
```

#### Custom Event

Demonstrates how to emit custom events with different parameter types:

```rust
// Emit a custom event with string data
CustomEvent::emit(user, "deposit".to_string(), amount);
```

#### Batch Operation Event

Shows how to handle array parameters in Neo N3 events:

```rust
// Create arrays of addresses and values
let addresses = vec![addr1, addr2, addr3];
let values = vec![amount1, amount2, amount3];

// Emit batch operation event
BatchOperation::emit(addresses, values);
```

### 4. Best Practices for Neo N3 Events

1. **Use ByteString for event names**: Always use `ByteString::from()` to create event names.
2. **Use Array<Any> for parameters**: Always store parameters in an `Array<Any>`.
3. **Properly handle null values**: Use `Any::new()` for null/None values (not empty strings or ByteArrays).
4. **Document event structures**: Clearly define the expected structure, parameters, and order.
5. **Use consistent naming**: Follow Neo N3 naming conventions (e.g., "Transfer" for NEP-17 tokens).
6. **Index important parameters**: Mark parameters that would benefit from indexing in your documentation.

## Running the Example

### Building

To build the example:

```bash
cd examples/event_demo
cargo build --target wasm32-unknown-unknown --release
```

### Compiling for Neo N3

Convert the WebAssembly to Neo N3 format:

```bash
neo-compiler compile ../../target/wasm32-unknown-unknown/release/event_demo.wasm --output ./build
```

### Deployment

The compiled contract can be deployed to a Neo N3 blockchain using Neo CLI:

```bash
neo-cli deploy ./build/event_demo.nef ./build/event_demo.manifest.json
```

### Testing Events

After deployment, you can verify event emission by:

1. Invoking contract methods that emit events
2. Using a Neo N3 blockchain explorer to view emitted events
3. Using the Neo CLI to retrieve and inspect application logs

For more information on Neo N3 events, refer to the [Neo N3 Runtime Guide](../../docs/neo_n3_runtime_guide.md) and [Neo N3 Implementation Guide](../../docs/neo_n3_implementation_guide.md).
