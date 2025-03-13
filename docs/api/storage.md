# Storage System in Neo N3 Contracts

This document describes the storage system for Neo N3 smart contracts developed with the Rust framework.

## Overview

The storage system in Neo N3 provides a persistent key-value store for smart contracts. The Neo N3 contract framework for Rust provides type-safe abstractions over this raw storage system to simplify development and reduce errors.

## Storage Implementation Details

### no_std Compatibility

The Neo N3 contract framework's storage system is fully compatible with `no_std` environments, which is essential for blockchain contract development. Key points:

- Uses `alloc` instead of `std` for collections and memory management
- Properly handles static mutable storage in test utilities
- Avoids standard library features that are not available in `no_std` environments

```rust
// Example of no_std compatible storage code
use alloc::vec::Vec;
use alloc::string::String;

// Creating a storage key without std dependencies
pub fn make_key(prefix: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(prefix.len() + 1 + key.len());
    result.extend_from_slice(prefix);
    result.push(b':');
    result.extend_from_slice(key);
    result
}
```

### FindOptions

The `FindOptions` type provides control over how the storage system searches for and returns data:

```rust
// Creating and using FindOptions
let mut options = FindOptions::default(); // Same as FindOptions::NONE

// Add specific options
options.add(FindOptions::REMOVE_PREFIX); // Remove prefix from keys
options.add(FindOptions::KEYS_ONLY);     // Return only keys

// Combining options with bitwise operations
let combined = FindOptions::REMOVE_PREFIX | FindOptions::BACKWARD;

// Available options:
// - NONE: No special options
// - KEYS_ONLY: Returns only keys, not values
// - REMOVE_PREFIX: Removes the prefix from returned keys
// - VALUES_ONLY: Returns only values, not keys
// - DESERIALIZE_VALUES: Deserialize values into specified type
// - BACKWARD: Search in backwards order
```

## Storage Primitives

### StorageItem

`StorageItem<T>` is a wrapper around a single value in storage with a specific key:

```rust
use neo_contract::prelude::*;

#[storage]
pub struct MyContract {
    counter: StorageItem<u64>,
    name: StorageItem<String>,
    owner: StorageItem<Address>,
}

impl MyContract {
    #[initialize]
    pub fn new() -> Self {
        Self {
            counter: StorageItem::new(b"counter"),
            name: StorageItem::new(b"name"),
            owner: StorageItem::new(b"owner"),
        }
    }
    
    pub fn increment(&mut self) {
        let current = self.counter.get().unwrap_or_default();
        self.counter.set(&(current + 1));
    }
    
    #[safe]
    pub fn get_counter(&self) -> u64 {
        self.counter.get().unwrap_or_default()
    }
}
```

Key methods:
- `new(key: &[u8]) -> Self` - Create a new storage item with the given key
- `get(&self) -> Option<T>` - Get the value from storage
- `set(&self, value: &T)` - Set the value in storage
- `delete(&self)` - Delete the value from storage

### StorageMap

`StorageMap<K, V>` provides a map-like interface for storing multiple values with different keys:

```rust
use neo_contract::prelude::*;

#[storage]
pub struct TokenContract {
    balances: StorageMap<Address, u64>,
}

impl TokenContract {
    #[initialize]
    pub fn new() -> Self {
        Self {
            balances: StorageMap::new(b"balances"),
        }
    }
    
    pub fn transfer(&mut self, from: &Address, to: &Address, amount: u64) -> bool {
        let from_balance = self.balances.get(from).unwrap_or_default();
        if from_balance < amount {
            return false;
        }
        
        // Update balances
        self.balances.insert(from, &(from_balance - amount));
        
        let to_balance = self.balances.get(to).unwrap_or_default();
        self.balances.insert(to, &(to_balance + amount));
        
        true
    }
    
    #[safe]
    pub fn balance_of(&self, account: &Address) -> u64 {
        self.balances.get(account).unwrap_or_default()
    }
}
```

Key methods:
- `new(prefix: &[u8]) -> Self` - Create a new storage map with the given prefix
- `get(&self, key: &K) -> Option<V>` - Get the value for a key
- `insert(&self, key: &K, value: &V)` - Insert or update a value for a key
- `remove(&self, key: &K)` - Remove a key-value pair
- `contains(&self, key: &K) -> bool` - Check if a key exists

## Storage Context

For more advanced use cases, you can work with the raw storage context:

```rust
use neo_contract::prelude::*;

#[method]
pub fn advanced_storage_operation() {
    let ctx = Storage::current_context();
    
    // Raw storage operations
    Storage::put(&ctx, b"raw_key", b"raw_value");
    let value = Storage::get(&ctx, b"raw_key");
    
    // Delete from storage
    Storage::delete(&ctx, b"raw_key");
}
```

## Testing Storage

### MockStorage for Unit Tests

The framework provides a `MockStorage` utility for testing storage operations without deploying to a blockchain:

```rust
use neo_contract::storage::test_utils::MockStorage;
use neo_contract::prelude::*;

#[test]
fn test_storage_operations() {
    // Clear any previous test data
    MockStorage::clear();
    
    // Set a value
    MockStorage::put(b"test:key", b"value");
    
    // Check if the key exists
    assert!(MockStorage::contains_key(b"test:key"));
    
    // Get the value
    let value = MockStorage::get(b"test:key");
    assert_eq!(value, b"value");
    
    // Find entries with prefix
    MockStorage::put(b"test:key1", b"value1");
    MockStorage::put(b"test:key2", b"value2");
    
    let mut options = FindOptions::default();
    options.add(FindOptions::REMOVE_PREFIX);
    
    let results = MockStorage::find(b"test:", &options);
    assert_eq!(results.len(), 3);
    
    // Clean up
    MockStorage::delete(b"test:key");
    assert!(!MockStorage::contains_key(b"test:key"));
}
```

The `MockStorage` implementation in the Neo N3 contract framework uses a static mutable storage container that properly handles access in tests using unsafe blocks to ensure thread safety.

### Debug Utilities

When testing Neo N3 contracts, you can use the `dump()` method to view the contents of the mock storage:

```rust
// After performing storage operations
let storage_contents = MockStorage::dump();
println!("{}", storage_contents);
```

## Serialization

The storage system automatically handles serialization and deserialization of common types:

- Basic types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `bool`, etc.
- Neo types: `Address`, `H160`, `H256`, `ByteString`, etc.
- Collection types: `Vec<T>`, `HashMap<K, V>` (with limitations)

For custom types, you need to implement the `Serialize` and `Deserialize` traits.

## Storage Keys and Prefixes

Keys and prefixes for storage items should be chosen carefully:

1. Use descriptive names for readability
2. Keep them reasonably short to minimize storage costs
3. Ensure uniqueness to avoid collisions
4. Consider using a hierarchical structure for complex contracts

Example of a hierarchical key structure:
```rust
// Token balances
let balance_prefix = b"balance";
// Token allowances
let allowance_prefix = b"allowance";
// Token metadata
let metadata_prefix = b"metadata";
```

## Storage Limitations

Be aware of these storage limitations:

1. Keys and values are limited to 64KB each
2. Writing to storage consumes GAS
3. Reading from storage is cheaper than writing
4. Storage operations in safe methods are limited to reading

## Best Practices

1. **Use Abstractions**: Always use `StorageItem` and `StorageMap` instead of raw storage when possible
2. **Default Values**: Handle missing values with `unwrap_or_default()` or similar methods
3. **Batched Operations**: Batch storage operations when possible to reduce GAS costs
4. **Clean Up**: Remove unused data to keep storage size minimal
5. **Safe Methods**: Mark read-only methods with `#[safe]` to optimize execution
6. **Consistent Keys**: Use a consistent naming convention for storage keys
