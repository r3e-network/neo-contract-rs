# Advanced Storage Patterns for Neo N3 Smart Contracts

This guide covers advanced storage patterns and techniques for optimizing storage usage in Neo N3 smart contracts using the Neo Contract Rust framework. Efficient storage design is crucial for both gas optimization and contract maintainability.

## Table of Contents

- [Introduction](#introduction)
- [Storage Basics Recap](#storage-basics-recap)
- [Advanced Storage Patterns](#advanced-storage-patterns)
  - [Composite Key Pattern](#composite-key-pattern)
  - [Storage Prefix Pattern](#storage-prefix-pattern)
  - [Lazy Loading Pattern](#lazy-loading-pattern)
  - [Pagination Pattern](#pagination-pattern)
  - [Batch Operation Pattern](#batch-operation-pattern)
  - [Hierarchical Storage Pattern](#hierarchical-storage-pattern)
  - [Sparse Storage Pattern](#sparse-storage-pattern)
- [Storage Optimization Techniques](#storage-optimization-techniques)
- [Practical Examples](#practical-examples)
- [Common Pitfalls](#common-pitfalls)
- [Conclusion](#conclusion)

## Introduction

Storage operations in Neo N3 smart contracts consume gas, with costs varying based on the operation type (read, write, delete) and data size. Implementing efficient storage patterns can significantly reduce gas costs and improve contract performance.

This guide builds upon the basic [Storage Guide](./storage_guide.md) and introduces advanced patterns for complex contract scenarios.

## Storage Basics Recap

Neo N3 provides a key-value storage system where:
- Keys and values are byte arrays
- Storage operations include `get`, `put`, and `delete`
- Storage context determines the contract's storage space

The Neo Contract Rust framework provides abstractions through the `Storage` trait and related utilities.

## Advanced Storage Patterns

### Composite Key Pattern

The composite key pattern combines multiple data elements into a single storage key, creating a logical relationship between data items.

#### Implementation

```rust
// Prefix + User Address + Token ID
fn balance_key(user: &Address, token_id: &[u8]) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(b"BALANCE"); // Prefix
    key.extend_from_slice(user.as_bytes());
    key.extend_from_slice(token_id);
    key
}

// Usage
pub fn get_balance(user: Address, token_id: Vec<u8>) -> u64 {
    let key = balance_key(&user, &token_id);
    Storage::get(&key).unwrap_or_default()
}

pub fn set_balance(user: Address, token_id: Vec<u8>, amount: u64) {
    let key = balance_key(&user, &token_id);
    Storage::put(&key, amount);
}
```

#### Benefits
- Organizes related data logically
- Enables efficient querying of related items
- Reduces key collisions

### Storage Prefix Pattern

The storage prefix pattern uses consistent prefixes to categorize different types of data, preventing key collisions and organizing storage logically.

#### Implementation

```rust
// Define storage prefixes as constants
const PREFIX_TOKEN_BALANCE: &[u8] = b"BALANCE";
const PREFIX_TOKEN_SUPPLY: &[u8] = b"SUPPLY";
const PREFIX_TOKEN_METADATA: &[u8] = b"META";

// Helper functions to create prefixed keys
fn balance_key(user: &Address) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(PREFIX_TOKEN_BALANCE);
    key.extend_from_slice(user.as_bytes());
    key
}

fn supply_key() -> Vec<u8> {
    PREFIX_TOKEN_SUPPLY.to_vec()
}

fn metadata_key(field: &[u8]) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(PREFIX_TOKEN_METADATA);
    key.extend_from_slice(field);
    key
}
```

#### Benefits
- Prevents key collisions between different data types
- Improves code readability and maintainability
- Enables easier debugging and data organization

### Lazy Loading Pattern

The lazy loading pattern defers loading data from storage until it's actually needed, reducing unnecessary storage operations.

#### Implementation

```rust
struct UserAccount {
    address: Address,
    balance: Option<u64>,
    settings: Option<UserSettings>,
}

impl UserAccount {
    fn new(address: Address) -> Self {
        Self {
            address,
            balance: None,
            settings: None,
        }
    }
    
    fn get_balance(&mut self) -> u64 {
        if self.balance.is_none() {
            let key = balance_key(&self.address);
            self.balance = Some(Storage::get(&key).unwrap_or_default());
        }
        self.balance.unwrap()
    }
    
    fn get_settings(&mut self) -> UserSettings {
        if self.settings.is_none() {
            let key = settings_key(&self.address);
            self.settings = Some(Storage::get(&key).unwrap_or_default());
        }
        self.settings.unwrap()
    }
    
    fn save(&self) {
        if let Some(balance) = self.balance {
            let key = balance_key(&self.address);
            Storage::put(&key, balance);
        }
        
        if let Some(settings) = &self.settings {
            let key = settings_key(&self.address);
            Storage::put(&key, settings);
        }
    }
}
```

#### Benefits
- Reduces unnecessary storage reads
- Improves performance for operations that don't need all data
- Enables batching of storage operations

### Pagination Pattern

The pagination pattern handles large collections of data by loading and processing them in smaller chunks.

#### Implementation

```rust
const PAGE_SIZE: u32 = 10;

// Store total count
fn set_total_items(count: u32) {
    Storage::put(b"TOTAL_ITEMS", count);
}

fn get_total_items() -> u32 {
    Storage::get(b"TOTAL_ITEMS").unwrap_or_default()
}

// Store item at index
fn set_item(index: u32, value: &[u8]) {
    let key = item_key(index);
    Storage::put(&key, value);
}

fn get_item(index: u32) -> Option<Vec<u8>> {
    let key = item_key(index);
    Storage::get(&key)
}

// Get items by page
fn get_items_page(page: u32) -> Vec<Vec<u8>> {
    let start_idx = page * PAGE_SIZE;
    let total = get_total_items();
    let mut result = Vec::new();
    
    for i in 0..PAGE_SIZE {
        let idx = start_idx + i;
        if idx >= total {
            break;
        }
        
        if let Some(item) = get_item(idx) {
            result.push(item);
        }
    }
    
    result
}

fn item_key(index: u32) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(b"ITEM");
    key.extend_from_slice(&index.to_le_bytes());
    key
}
```

#### Benefits
- Handles large datasets efficiently
- Reduces gas costs for operations on large collections
- Improves user experience for dApp interactions

### Batch Operation Pattern

The batch operation pattern groups multiple storage operations together to reduce the number of storage calls.

#### Implementation

```rust
struct BatchUpdate {
    puts: HashMap<Vec<u8>, Vec<u8>>,
    deletes: Vec<Vec<u8>>,
}

impl BatchUpdate {
    fn new() -> Self {
        Self {
            puts: HashMap::new(),
            deletes: Vec::new(),
        }
    }
    
    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.puts.insert(key, value);
    }
    
    fn delete(&mut self, key: Vec<u8>) {
        self.deletes.push(key);
    }
    
    fn commit(self) {
        // Apply all puts
        for (key, value) in self.puts {
            Storage::put(&key, &value);
        }
        
        // Apply all deletes
        for key in self.deletes {
            Storage::delete(&key);
        }
    }
}

// Usage
fn transfer_tokens(from: Address, to: Address, amount: u64) {
    let mut batch = BatchUpdate::new();
    
    // Get current balances
    let from_key = balance_key(&from);
    let to_key = balance_key(&to);
    
    let from_balance: u64 = Storage::get(&from_key).unwrap_or_default();
    let to_balance: u64 = Storage::get(&to_key).unwrap_or_default();
    
    // Update balances
    assert!(from_balance >= amount, "Insufficient balance");
    let new_from_balance = from_balance - amount;
    let new_to_balance = to_balance + amount;
    
    // Queue updates
    batch.put(from_key, new_from_balance.to_le_bytes().to_vec());
    batch.put(to_key, new_to_balance.to_le_bytes().to_vec());
    
    // Commit all changes at once
    batch.commit();
}
```

#### Benefits
- Reduces the number of storage operations
- Ensures atomicity of related operations
- Improves gas efficiency

### Hierarchical Storage Pattern

The hierarchical storage pattern organizes data in a tree-like structure, making it easier to manage complex relationships.

#### Implementation

```rust
// Organization -> Department -> Employee structure
fn org_key(org_id: &str) -> Vec<u8> {
    [b"ORG", org_id.as_bytes()].concat()
}

fn dept_key(org_id: &str, dept_id: &str) -> Vec<u8> {
    [b"ORG", org_id.as_bytes(), b"DEPT", dept_id.as_bytes()].concat()
}

fn employee_key(org_id: &str, dept_id: &str, emp_id: &str) -> Vec<u8> {
    [b"ORG", org_id.as_bytes(), b"DEPT", dept_id.as_bytes(), b"EMP", emp_id.as_bytes()].concat()
}

// Get all departments in an organization
fn get_org_departments(org_id: &str) -> Vec<String> {
    let prefix = [b"ORG", org_id.as_bytes(), b"DEPT"].concat();
    // Implementation would use iterator pattern to find all keys with this prefix
    // This is a simplified example
    Vec::new()
}
```

#### Benefits
- Natural representation of hierarchical relationships
- Efficient querying of related items
- Logical organization of complex data structures

### Sparse Storage Pattern

The sparse storage pattern allocates storage only for non-default values, saving storage space for sparse data structures.

#### Implementation

```rust
// For a large matrix or grid where most cells are empty/default
fn set_grid_value(x: u32, y: u32, value: u64) {
    if value == 0 {
        // Default value, remove from storage if exists
        let key = grid_key(x, y);
        Storage::delete(&key);
    } else {
        // Non-default value, store it
        let key = grid_key(x, y);
        Storage::put(&key, value);
    }
}

fn get_grid_value(x: u32, y: u32) -> u64 {
    let key = grid_key(x, y);
    Storage::get(&key).unwrap_or_default()
}

fn grid_key(x: u32, y: u32) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(b"GRID");
    key.extend_from_slice(&x.to_le_bytes());
    key.extend_from_slice(&y.to_le_bytes());
    key
}
```

#### Benefits
- Reduces storage usage for sparse data structures
- Saves gas costs by not storing default values
- Efficient for large, mostly empty data structures

## Storage Optimization Techniques

Beyond the patterns above, consider these optimization techniques:

1. **Use Appropriate Data Types**: Choose the smallest data type that meets your needs.
2. **Minimize Storage Writes**: Writes are more expensive than reads; minimize them when possible.
3. **Compress Data**: For large data, consider compression techniques.
4. **Use Serialization Efficiently**: Choose efficient serialization formats.
5. **Clean Up Unused Storage**: Delete storage entries that are no longer needed.
6. **Avoid Redundant Storage**: Don't store data that can be derived or calculated.

## Practical Examples

For practical implementations of these patterns, refer to:

1. [Secure Vault Example](../examples/defi/secure_vault/) - Demonstrates composite keys, prefixes, and lazy loading
2. [Storage-Optimized NFT](../examples/nft/storage_optimized/) - Shows efficient storage for NFT metadata

## Common Pitfalls

1. **Over-optimization**: Don't sacrifice code readability for minor optimizations.
2. **Inconsistent Key Formats**: Maintain consistent key formatting throughout your contract.
3. **Missing Cleanup**: Forgetting to delete obsolete storage entries.
4. **Key Collisions**: Not using proper prefixes, leading to key collisions.
5. **Excessive Serialization**: Serializing and deserializing too frequently.

## Conclusion

Efficient storage design is crucial for Neo N3 smart contracts. By implementing these advanced patterns, you can create contracts that are more gas-efficient, maintainable, and scalable.

For more information on storage optimization, refer to the [Gas Optimization Guide](./gas_optimization.md) and explore the example contracts that demonstrate these patterns in practice.

Remember that the best storage pattern depends on your specific use case. Consider the access patterns, data relationships, and expected scale of your contract when designing your storage solution. 