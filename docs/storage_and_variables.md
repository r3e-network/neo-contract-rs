# Storage and Variables in Neo Smart Contracts

This guide explains how to manage on-chain state in Neo smart contracts using the Neo Contract Rust Framework.

## Neo Storage Basics

Neo N3 provides a key-value storage system for smart contracts with several important characteristics:

- **Contract Isolation**: Each contract has its own storage space
- **Key-Value Model**: Data is stored as key-value pairs
- **Gas Costs**: Reading and writing storage costs gas
- **Permanence**: Data persists between contract invocations

## Storage Primitives

### StorageItem

`StorageItem<T>` stores a single value of type T at a specific key:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct Counter {
    count: StorageItem<u64>,
}

#[contractimpl]
impl Counter {
    #[constructor]
    pub fn new() -> Self {
        Self {
            count: StorageItem::new(0),
        }
    }
    
    #[method]
    pub fn increment(&mut self) {
        let current = self.count.get();
        self.count.set(current + 1);
    }
    
    #[method]
    pub fn get_count(&self) -> u64 {
        self.count.get()
    }
}
```

### StorageMap

`StorageMap<K, V>` provides a key-value mapping:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct Balances {
    balances: StorageMap<Address, u64>,
}

#[contractimpl]
impl Balances {
    #[constructor]
    pub fn new() -> Self {
        Self {
            balances: StorageMap::new(),
        }
    }
    
    #[method]
    pub fn set_balance(&mut self, address: Address, amount: u64) {
        self.balances.insert(&address, amount);
    }
    
    #[method]
    pub fn get_balance(&self, address: Address) -> u64 {
        self.balances.get(&address).unwrap_or(0)
    }
}
```

## Best Practices

1. **Minimize Storage Operations**: Storage operations are expensive
2. **Use Appropriate Types**: Choose storage primitives that match your data model
3. **Batch Operations**: Group related storage operations when possible
4. **Validate Before Writing**: Always validate data before storing it

## Conclusion

Effective storage management is crucial for developing efficient and secure Neo smart contracts. By using the storage primitives provided by the Neo Contract Rust Framework, developers can create contracts with clean, type-safe state management.