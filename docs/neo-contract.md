# Neo Contract Documentation

The `neo-contract` crate is the core library of the Neo Contract Rust Framework, providing the fundamental types and functionality needed to develop Neo N3 smart contracts in Rust.

## Overview

The neo-contract crate provides:

1. Neo-specific types (Hash160, ByteString, etc.)
2. Storage operations and context
3. Event emission and handling
4. Runtime context and utilities
5. Interoperability with the Neo N3 blockchain

## Core Components

### Types

The `types` module provides Neo N3-specific data types:

#### ByteString

`ByteString` represents a sequence of bytes, similar to a string but with binary data capabilities:

```rust
pub struct ByteString {
    // Representation of bytes
    bytes: Vec<u8>,
}
```

Key methods:
- `new`: Creates a new ByteString
- `from_bytes`: Creates a ByteString from a byte slice
- `len`: Gets the length of the ByteString
- `as_bytes`: Gets the underlying bytes
- `concat`: Concatenates two ByteStrings

#### Hash160

`Hash160` represents a 20-byte hash, typically used for addresses and script hashes:

```rust
pub struct Hash160 {
    // 20-byte hash
    hash: [u8; 20],
}
```

Key methods:
- `new`: Creates a new Hash160
- `from_bytes`: Creates a Hash160 from a 20-byte slice
- `from_address`: Creates a Hash160 from a Neo address string
- `to_address`: Converts the Hash160 to a Neo address string
- `equals`: Compares with another Hash160

#### Hash256

`Hash256` represents a 32-byte hash, typically used for transaction and block hashes:

```rust
pub struct Hash256 {
    // 32-byte hash
    hash: [u8; 32],
}
```

Key methods:
- `new`: Creates a new Hash256
- `from_bytes`: Creates a Hash256 from a 32-byte slice
- `equals`: Compares with another Hash256

#### Other Types

- `ECPoint`: Represents an elliptic curve point (public key)
- `UInt160`: Unsigned 160-bit integer
- `UInt256`: Unsigned 256-bit integer
- `Address`: Neo N3 address

### Storage

The `storage` module provides functionality for persisting data on the blockchain:

#### StorageContext

`StorageContext` represents a storage area for a contract:

```rust
pub struct StorageContext {
    // Internal representation
    context_id: u32,
}
```

Key methods:
- `current`: Gets the current contract's storage context
- `for_contract`: Gets a storage context for a specific contract
- `create_storage_map`: Creates a storage map within the context

#### StorageMap

`StorageMap` provides a map-like interface to storage:

```rust
pub struct StorageMap<K, V> {
    // Internal representation
    context: StorageContext,
    prefix: ByteString,
    _phantom_key: PhantomData<K>,
    _phantom_value: PhantomData<V>,
}
```

Key methods:
- `new`: Creates a new StorageMap
- `get`: Gets a value from storage by key
- `put`: Puts a value into storage by key
- `delete`: Deletes a value from storage by key
- `has`: Checks if a key exists in storage

#### Storage Operations

Direct storage operations:

```rust
// Get a value from storage
pub fn get(context: &StorageContext, key: &[u8]) -> Option<ByteString>;

// Put a value into storage
pub fn put(context: &StorageContext, key: &[u8], value: &[u8]);

// Delete a value from storage
pub fn delete(context: &StorageContext, key: &[u8]);
```

### Events

The `events` module provides functionality for emitting events from contracts:

#### Event Emission

```rust
// Emit an event with parameters
pub fn emit<T: EventParams>(event_name: &str, params: T);
```

#### Event Trait

```rust
// Trait for event parameters
pub trait EventParams {
    // Converts the event parameters to a vector of values
    fn to_array(&self) -> Vec<Value>;
}
```

### Runtime

The `runtime` module provides access to the Neo N3 runtime environment:

#### Transaction Context

```rust
// Get the current transaction hash
pub fn get_tx_hash() -> Hash256;

// Get the current block time
pub fn get_time() -> u64;

// Get the current block height
pub fn get_block_height() -> u32;

// Check if the transaction is valid
pub fn check_witness(hash: &Hash160) -> bool;
```

#### Contract Operations

```rust
// Call another contract
pub fn call_contract<T>(
    script_hash: &Hash160,
    method: &str,
    args: &[Value],
) -> Result<T, ContractError>;

// Create a new contract
pub fn create_contract(
    script: &[u8],
    manifest: &str,
) -> Result<Hash160, ContractError>;

// Update a contract
pub fn update_contract(
    script: &[u8],
    manifest: &str,
) -> Result<(), ContractError>;
```

## Usage Examples

### Basic Contract

```rust
use neo_contract::prelude::*;

#[no_mangle]
pub extern "C" fn main() {
    // Initialize the contract
    initialize();
}

#[no_mangle]
pub extern "C" fn _deploy(data: &[u8], update: bool) -> bool {
    // Deployment logic
    true
}

#[no_mangle]
pub extern "C" fn hello_world() -> ByteString {
    // Return a greeting
    ByteString::from_bytes(b"Hello, Neo!")
}
```

### Storage Example

```rust
use neo_contract::prelude::*;

// Define storage keys
const TOTAL_SUPPLY_KEY: &[u8] = b"totalSupply";
const BALANCE_PREFIX: &[u8] = b"balance:";

#[no_mangle]
pub extern "C" fn get_total_supply() -> u64 {
    // Get the total supply from storage
    let context = StorageContext::current();
    let value = storage::get(&context, TOTAL_SUPPLY_KEY);
    
    match value {
        Some(bytes) => deserialize::<u64>(&bytes),
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn get_balance(account: Hash160) -> u64 {
    // Get the balance for an account
    let context = StorageContext::current();
    
    // Construct the storage key
    let mut key = Vec::with_capacity(BALANCE_PREFIX.len() + account.len());
    key.extend_from_slice(BALANCE_PREFIX);
    key.extend_from_slice(&account.as_bytes());
    
    let value = storage::get(&context, &key);
    
    match value {
        Some(bytes) => deserialize::<u64>(&bytes),
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn set_balance(account: Hash160, balance: u64) -> bool {
    // Verify caller is authorized
    if !runtime::check_witness(&account) {
        return false;
    }
    
    // Set the balance for an account
    let context = StorageContext::current();
    
    // Construct the storage key
    let mut key = Vec::with_capacity(BALANCE_PREFIX.len() + account.len());
    key.extend_from_slice(BALANCE_PREFIX);
    key.extend_from_slice(&account.as_bytes());
    
    // Serialize and store the balance
    let value = serialize(&balance);
    storage::put(&context, &key, &value);
    
    true
}
```

### Event Example

```rust
use neo_contract::prelude::*;

// Define an event
struct TransferEvent {
    from: Hash160,
    to: Hash160,
    amount: u64,
}

// Implement the EventParams trait
impl EventParams for TransferEvent {
    fn to_array(&self) -> Vec<Value> {
        vec![
            Value::from(&self.from),
            Value::from(&self.to),
            Value::from(self.amount),
        ]
    }
}

#[no_mangle]
pub extern "C" fn transfer(from: Hash160, to: Hash160, amount: u64) -> bool {
    // Verify caller is authorized
    if !runtime::check_witness(&from) {
        return false;
    }
    
    // Transfer logic (simplified)
    // ...
    
    // Emit transfer event
    let event = TransferEvent { from, to, amount };
    events::emit("Transfer", event);
    
    true
}
```

### Contract Call Example

```rust
use neo_contract::prelude::*;

// Define another contract's script hash
const TOKEN_CONTRACT: [u8; 20] = [
    // Script hash bytes
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
    0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14,
];

#[no_mangle]
pub extern "C" fn get_token_balance(account: Hash160) -> u64 {
    // Call the token contract's balanceOf method
    let token_hash = Hash160::from_bytes(&TOKEN_CONTRACT);
    
    let args = vec![
        Value::from(&account),
    ];
    
    match runtime::call_contract::<u64>(&token_hash, "balanceOf", &args) {
        Ok(balance) => balance,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn transfer_tokens(to: Hash160, amount: u64) -> bool {
    // Get the caller's address
    let from = runtime::calling_script_hash();
    
    // Call the token contract's transfer method
    let token_hash = Hash160::from_bytes(&TOKEN_CONTRACT);
    
    let args = vec![
        Value::from(&from),
        Value::from(&to),
        Value::from(amount),
    ];
    
    match runtime::call_contract::<bool>(&token_hash, "transfer", &args) {
        Ok(success) => success,
        Err(_) => false,
    }
}
```

## Advanced Features

### Serialization

The `serialization` module provides utilities for serializing and deserializing data:

```rust
// Serialize a value to bytes
pub fn serialize<T: Serialize>(value: &T) -> Vec<u8>;

// Deserialize bytes to a value
pub fn deserialize<T: Deserialize>(bytes: &[u8]) -> T;
```

### Interoperability

The `interop` module provides low-level interoperability with the Neo N3 blockchain:

```rust
// Call a native contract
pub fn call_native(
    native_hash: u32,
    method: u32,
    args: &[Value],
) -> Result<Value, ContractError>;

// Call a system contract
pub fn call_system(
    method: u32,
    args: &[Value],
) -> Result<Value, ContractError>;
```

### Cryptography

The `crypto` module provides cryptographic utilities:

```rust
// Verify a signature
pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &ECPoint,
) -> bool;

// Calculate a hash
pub fn sha256(data: &[u8]) -> [u8; 32];
pub fn ripemd160(data: &[u8]) -> [u8; 20];
```

## Best Practices

When using the neo-contract crate, follow these best practices:

1. **Minimize Storage Operations**: Storage operations are expensive, so minimize them.

2. **Use Appropriate Types**: Use Neo-specific types when interacting with the blockchain.

3. **Handle Errors**: Always handle potential errors and edge cases.

4. **Validate Inputs**: Always validate inputs to your contract methods.

5. **Check Permissions**: Use `check_witness` to verify caller permissions.

6. **Emit Meaningful Events**: Emit events for important state changes.

7. **Optimize Gas Usage**: Be mindful of gas costs for operations.

## Troubleshooting

Common issues and solutions:

### Storage Issues

If you encounter storage-related errors:
- Ensure you're using the correct `StorageContext`
- Verify that keys and values are properly serialized
- Check for storage size limits

### Type Conversion Errors

If you have type conversion issues:
- Use the appropriate conversion methods
- Verify that byte arrays have the correct length
- Use the proper serialization/deserialization for complex types

### Permission Errors

If operations fail due to permissions:
- Ensure you're calling `check_witness` properly
- Verify that the contract has the necessary permissions in its manifest
- Check that the transaction sender is authorized

### Gas Limitations

If your contract exceeds gas limits:
- Optimize storage operations
- Reduce computational complexity
- Simplify contract logic where possible

## Future Development

The neo-contract crate will continue to evolve with these planned enhancements:

1. **Higher-Level Abstractions**: More convenient APIs for common patterns.

2. **Improved Error Handling**: More detailed error information and recovery.

3. **Advanced Storage Patterns**: Better support for complex storage scenarios.

4. **Standard Implementation Helpers**: Utilities for implementing Neo N3 standards.

5. **Testing Utilities**: Better support for unit and integration testing.