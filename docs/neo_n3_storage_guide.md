# Neo N3 Storage Guide

This guide explains how to properly use storage in Neo N3 smart contracts using the neo-contract-rs framework. Efficient storage management is critical for optimizing gas costs and ensuring proper contract functionality.

## Overview

Neo N3 provides a key-value storage system for smart contracts. The neo-contract-rs framework offers abstractions that make working with the Neo N3 storage system more ergonomic while maintaining efficiency.

## Storage Abstractions

The framework provides several storage abstractions:

### 1. StorageItem

For single-value storage:

```rust
#[storage]
struct MyContract {
    // Simple counter
    counter: StorageItem<u64>,
    
    // Contract owner
    owner: StorageItem<Address>,
    
    // Token name
    name: StorageItem<String>,
}

impl MyContract {
    #[constructor]
    fn new() -> Self {
        Self {
            counter: StorageItem::new(0),
            owner: StorageItem::new(Runtime::calling_script_hash()),
            name: StorageItem::new("My Token".to_string()),
        }
    }
    
    // Increment counter
    fn increment(&mut self) {
        let current = self.counter.get().unwrap_or_default();
        self.counter.set(current + 1);
    }
}
```

### 2. StorageMap

For key-value mappings:

```rust
#[storage]
struct MyContract {
    // Token balances
    balances: StorageMap<Address, u64>,
    
    // Token allowances (nested mapping)
    allowances: StorageMap<(Address, Address), u64>,
}

impl MyContract {
    // Get balance
    #[method]
    #[safe]
    fn balance_of(&self, address: Address) -> u64 {
        self.balances.get(&address).unwrap_or_default()
    }
    
    // Set balance
    fn set_balance(&mut self, address: Address, amount: u64) {
        self.balances.insert(address, amount);
    }
    
    // Get allowance
    #[method]
    #[safe]
    fn allowance(&self, owner: Address, spender: Address) -> u64 {
        self.allowances.get(&(owner, spender)).unwrap_or_default()
    }
}
```

### 3. StorageArray

For array-like structures:

```rust
#[storage]
struct MyContract {
    // List of addresses
    members: StorageArray<Address>,
}

impl MyContract {
    // Add member
    fn add_member(&mut self, address: Address) {
        self.members.push(address);
    }
    
    // Get member at index
    #[method]
    #[safe]
    fn get_member(&self, index: u32) -> Option<Address> {
        if index < self.members.len() {
            Some(self.members.get(index).unwrap())
        } else {
            None
        }
    }
}
```

## Key Prefixes and Serialization

Under the hood, the framework handles key prefixes and serialization:

```rust
// This creates storage with keys like:
// - "counter" for counter
// - "balances:{address}" for balances
// - "allowances:{owner}:{spender}" for allowances
#[storage]
struct MyContract {
    counter: StorageItem<u64>,
    balances: StorageMap<Address, u64>,
    allowances: StorageMap<(Address, Address), u64>,
}
```

## Storage Operations

### Read Operations

```rust
// Read a storage item
let counter = self.counter.get().unwrap_or_default();

// Read from a map
let balance = self.balances.get(&address).unwrap_or_default();

// Check if key exists
if self.balances.contains_key(&address) {
    // Key exists
}
```

### Write Operations

```rust
// Write to a storage item
self.counter.set(new_value);

// Insert into a map
self.balances.insert(address, new_balance);

// Remove from a map
self.balances.remove(&address);
```

## Storage Iteration

Neo N3 allows iteration over storage entries with a common prefix:

```rust
// Iterate over all balances
fn sum_all_balances(&self) -> u64 {
    let mut total = 0;
    
    for (_, balance) in self.balances.iter() {
        total += balance;
    }
    
    total
}
```

## Best Practices

### 1. Minimize Storage Operations

Storage operations (especially writes) are expensive in terms of gas. Minimize them by:

```rust
// BAD: Multiple writes
self.balances.insert(address1, balance1 + 100);
self.balances.insert(address2, balance2 - 100);

// GOOD: Calculate first, then write
let new_balance1 = balance1 + 100;
let new_balance2 = balance2 - 100;
self.balances.insert(address1, new_balance1);
self.balances.insert(address2, new_balance2);
```

### 2. Use Option for Optional Values

```rust
// Define optional storage
token_admin: StorageItem<Option<Address>>,

// Setting optional value
self.token_admin.set(Some(address));

// Clearing optional value
self.token_admin.set(None);
```

### 3. Proper Removal for Zero Values

For token balances, it's often better to remove entries rather than store zeros:

```rust
fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    let from_balance = self.balances.get(&from).unwrap_or_default();
    assert!(from_balance >= amount, "Insufficient balance");
    
    // Update from balance
    let new_from_balance = from_balance - amount;
    if new_from_balance > 0 {
        self.balances.insert(from, new_from_balance);
    } else {
        // Remove entry instead of storing zero
        self.balances.remove(&from);
    }
    
    // Update to balance
    let to_balance = self.balances.get(&to).unwrap_or_default();
    self.balances.insert(to, to_balance + amount);
    
    true
}
```

### 4. Batch Operations

Group related storage operations:

```rust
fn batch_transfer(&mut self, from: Address, recipients: Vec<Address>, amounts: Vec<u64>) -> bool {
    // Calculate total amount first
    let total_amount: u64 = amounts.iter().sum();
    
    // Check balance once
    let from_balance = self.balances.get(&from).unwrap_or_default();
    assert!(from_balance >= total_amount, "Insufficient balance");
    
    // Update from balance once
    let new_from_balance = from_balance - total_amount;
    if new_from_balance > 0 {
        self.balances.insert(from, new_from_balance);
    } else {
        self.balances.remove(&from);
    }
    
    // Update recipient balances
    for i in 0..recipients.len() {
        let recipient = recipients[i];
        let amount = amounts[i];
        
        let recipient_balance = self.balances.get(&recipient).unwrap_or_default();
        self.balances.insert(recipient, recipient_balance + amount);
    }
    
    true
}
```

### 5. Clean Up Unused Storage

When data is no longer needed, remove it to free up storage:

```rust
fn remove_expired_data(&mut self, user: Address) {
    // Clean up all user data
    self.user_data.remove(&user);
    self.user_settings.remove(&user);
    self.user_history.remove(&user);
}
```

## Security Considerations

### 1. Check Authorization Before Storage Writes

Always verify that the transaction sender is authorized before modifying storage:

```rust
fn set_owner(&mut self, new_owner: Address) -> bool {
    let current_owner = self.owner.get().unwrap();
    assert!(Runtime::check_witness(&current_owner), "Not authorized");
    
    self.owner.set(new_owner);
    true
}
```

### 2. Overflow Protection

Always check for integer overflows/underflows:

```rust
fn add_balance(&mut self, address: Address, amount: u64) -> bool {
    let balance = self.balances.get(&address).unwrap_or_default();
    
    // Check for overflow
    assert!(u64::MAX - balance >= amount, "Balance overflow");
    
    self.balances.insert(address, balance + amount);
    true
}
```

### 3. Protect Critical Storage

For critical storage values, consider adding additional access controls:

```rust
#[storage]
struct MyContract {
    // Critical contract configuration
    settings: StorageItem<ContractSettings>,
    
    // Access control list
    admins: StorageMap<Address, bool>,
}

impl MyContract {
    fn only_admin(&self) -> bool {
        let caller = Runtime::calling_script_hash();
        self.admins.get(&caller).unwrap_or_default()
    }
    
    fn update_settings(&mut self, new_settings: ContractSettings) -> bool {
        assert!(self.only_admin(), "Admin only");
        self.settings.set(new_settings);
        true
    }
}
```

## Optimizing Storage for Neo N3

### 1. Use Appropriate Types

Choose the smallest suitable type for your data:

```rust
// BAD: Using u64 when u8 would suffice
decimals: StorageItem<u64>,  // Decimals are typically 0-18

// GOOD: Using appropriate type
decimals: StorageItem<u8>,   // Using u8 saves storage space
```

### 2. Batch Related Data

For data that's often accessed together, consider using structs:

```rust
// Define a struct for related data
struct UserProfile {
    name: String,
    email: String,
    registration_date: u64,
}

#[storage]
struct MyContract {
    // Store entire profile in one storage slot
    profiles: StorageMap<Address, UserProfile>,
}
```

### 3. Lazy Loading

For complex structures, consider lazy loading:

```rust
fn get_user_data(&self, user: Address) -> UserData {
    if let Some(data) = self.user_data.get(&user) {
        // Return existing data
        data
    } else {
        // Return default data without writing to storage
        UserData::default()
    }
}

fn update_user_data(&mut self, user: Address, data: UserData) {
    // Only write to storage when explicitly updated
    self.user_data.insert(user, data);
}
```

## Conclusion

Proper storage management is essential for efficient and secure Neo N3 smart contracts. By following these patterns and best practices, you can create contracts that minimize gas costs, maintain data integrity, and provide optimal performance on the Neo N3 blockchain.

For more information, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Runtime Guide](./neo_n3_runtime_guide.md)
