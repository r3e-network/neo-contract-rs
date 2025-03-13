# Neo N3 Contract Implementation Guide

This guide provides comprehensive information on how to properly implement Neo N3 smart contracts using the neo-contract-rs framework.

## Neo N3 Contract Structure

A Neo N3 contract typically consists of the following components:

1. **Contract Storage Definition**: Defines the persistent state stored on the blockchain
2. **Events**: Notifications emitted during contract execution
3. **Contract Methods**: Functions that can be called from outside the contract
4. **Private Helper Functions**: Internal functions for code organization

## Proper Event Implementation for Neo N3

In Neo N3, events are properly emitted using the `Runtime::notify` method rather than using macros. Follow these steps:

1. Create a struct for your event
2. Implement the event emission method following this pattern:

```rust
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Null for minting
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Null for burning
        }
        
        event_data.push(Any::from(amount));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
}
```

### Handling Null Values

For null or None values, use `Any::new()` as there is no `null()` method in the Neo N3 API.

## Safe Methods in Neo N3

Neo N3 contracts should distinguish between safe (read-only) and non-safe (state-modifying) methods. Safe methods are marked with the `#[safe]` attribute and are represented in the contract manifest with `"safe": true`. This is important for optimizing contract execution and ensuring proper access control.

Example:

```rust
#[safe]
pub fn balance_of(&self, address: H160) -> u64 {
    self.balances.get(&address).unwrap_or_default()
}
```

## Method Annotations

In Neo N3 smart contracts, there are several important method annotations:

### `#[method]`

The `#[method]` annotation exposes a function as a callable method in your Neo N3 contract. This makes the method available to be called from outside the contract.

```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Implementation
}
```

### `#[safe]`

The `#[safe]` annotation marks a method as read-only, meaning it doesn't modify the contract's state. Safe methods are optimized for gas efficiency and are marked as `"safe": true` in the contract manifest.

**Important**: `#[safe]` implicitly includes the functionality of `#[method]`, so you don't need to use both annotations together. Simply use `#[safe]` for read-only methods.

```rust
// Correct - #[safe] already includes #[method] functionality
#[safe]
pub fn balance_of(&self, account: H160) -> u64 {
    self.balances.get(&account).unwrap_or_default()
}

// Incorrect - redundant annotation
// #[method]  
// #[safe]
// pub fn total_supply(&self) -> u64 {
//    self.total_supply.get()
// }
```

### `#[constructor]`

The `#[constructor]` annotation marks a method as the contract's constructor, which is called when the contract is deployed.

```rust
#[constructor]
pub fn new(owner: H160) -> Self {
    // Implementation
}
```

## Storage Patterns

Neo N3 contracts use the following storage patterns:

### Simple Value Storage

```rust
#[storage]
struct MyContract {
    counter: StorageItem<u64>,
    owner: StorageItem<Address>,
}
```

### Map Storage

```rust
#[storage]
struct MyContract {
    balances: StorageMap<Address, u64>,
    allowances: StorageMap<(Address, Address), u64>,
}
```

## Constructor Pattern

Use the `#[constructor]` attribute to define contract initialization:

```rust
#[constructor]
pub fn new(owner: Address) -> Self {
    Self {
        counter: StorageItem::new(0),
        owner: StorageItem::new(owner),
    }
}
```

## Security Best Practices

### Authentication

Always verify transaction signers:

```rust
assert!(Runtime::check_witness(&caller), "No authorization");
```

### Reentrancy Protection

Use the `#[no_reentry]` attribute for methods that modify state:

```rust
#[method]
#[no_reentry]
pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
    // Method implementation
}
```

### Integer Overflow Protection

Always check for integer overflows:

```rust
// Ensure addition doesn't overflow
assert!(u64::MAX - current_balance >= amount, "Addition overflow");
let new_balance = current_balance + amount;
```

## Interacting with Other Contracts

To call other contracts on the Neo N3 blockchain:

```rust
let target_contract = H160::from_hex("your_contract_hash").unwrap();
let method = "transfer";
let args = vec![
    StackItem::from(caller),
    StackItem::from(recipient),
    StackItem::from(amount),
];

let result = contract::call(
    &target_contract,
    method,
    &args,
    CallFlags::All
);
```

## Example NEP-17 Implementation

For a complete NEP-17 token implementation following Neo N3 standards, refer to the [NEP-17 example](/examples/nep17/).

## Compilation and Deployment

### Compilation

Compile your Rust contract to WebAssembly:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Convert to Neo N3 format:

```bash
neo-compiler compile \
    target/wasm32-unknown-unknown/release/my_contract.wasm \
    --output ./build
```

### Deployment

Deploy using Neo CLI:

```bash
neo-cli deploy ./build/my_contract.nef ./build/my_contract.manifest.json
```

## Testing Neo N3 Contracts

Use the neo-contract-testing utility to test your contracts:

```rust
#[test]
fn test_transfer() {
    let mut context = TestingContext::new();
    let sender = H160::from_hex("0x01ff00ff00ff00ff00ff00ff00ff00ff00ff00a4").unwrap();
    let recipient = H160::from_hex("0x01ff00ff00ff00ff00ff00ff00ff00ff00ff00a5").unwrap();
    
    // Set up test context
    context.set_caller(sender);
    
    // Deploy contract
    let contract = MyContract::new();
    context.deploy(contract);
    
    // Call contract method
    let result = context.call::<_, bool>("transfer", (recipient, 100u64));
    
    // Assert expectations
    assert!(result);
    assert_eq!(context.get_storage::<u64>("balances", &recipient).unwrap(), 100);
}
```

## Best Practices for Neo N3 Contract Development

1. **Minimize Storage Usage**: Storage operations are expensive and limited
2. **Use Safe Methods**: Mark read-only methods as `#[safe]` to optimize gas costs
3. **Proper Event Emission**: Follow the Neo N3 event pattern with `Runtime::notify`
4. **Access Control**: Implement proper authorization checks with `check_witness`
5. **Comprehensive Testing**: Test all contract paths thoroughly before deployment
6. **Gas Optimization**: Minimize computation to reduce transaction costs
7. **Secure Integer Math**: Always check for overflows and underflows
8. **Documentation**: Document your code thoroughly, especially public methods

## Common Neo N3 APIs

### Runtime APIs

- `Runtime::check_witness(&address)`: Verify transaction signer
- `Runtime::calling_script_hash()`: Get the caller's script hash
- `Runtime::current_time()`: Get current blockchain timestamp
- `Runtime::gas_left()`: Get remaining gas for the transaction
- `Runtime::notify(name, data)`: Emit an event

### Storage APIs

- `Storage::get(key)`: Get a value from storage
- `Storage::put(key, value)`: Store a value in storage
- `Storage::delete(key)`: Remove a value from storage
- `Storage::find(key_prefix)`: Iterate over storage with prefix

## Conclusion

Following these guidelines ensures your Neo N3 contracts will be properly implemented, secure, and compatible with the Neo N3 blockchain. Remember to always test thoroughly before deploying to the mainnet.
