# Neo Contract Storage Guide

This guide explains how to use storage in Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Storage in Neo N3 smart contracts is persistent data that survives between contract invocations. The neo-contract-rs framework provides type-safe storage primitives with automatic serialization and gas optimization.

## Storage Types

### StorageItem<T>

Used for storing single values:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct MyContract {
    #[storage]
    total_supply: StorageItem<u64>,
    
    #[storage]
    owner: StorageItem<H160>,
    
    #[storage]
    name: StorageItem<String>,
}
```

### StorageMap<K, V>

Used for key-value mappings:

```rust
#[contract]
pub struct TokenContract {
    #[storage]
    balances: StorageMap<H160, u64>,
    
    #[storage]
    allowances: StorageMap<(H160, H160), u64>, // Nested key
    
    #[storage]
    user_data: StorageMap<H160, UserInfo>,
}
```

### Complex Storage Structures

```rust
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub registration_time: u64,
    pub last_activity: u64,
    pub reputation_score: u32,
}

#[contract]
pub struct UserContract {
    #[storage]
    users: StorageMap<H160, UserInfo>,
    
    #[storage]
    user_groups: StorageMap<String, Vec<H160>>,
}
```

## Storage Operations

### Reading from Storage

```rust
impl MyContract {
    #[safe]
    pub fn get_total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or(0)
    }
    
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    #[safe]
    pub fn get_user_info(&self, account: H160) -> Option<UserInfo> {
        self.user_data.get(&account)
    }
}
```

### Writing to Storage

```rust
impl MyContract {
    #[method]
    pub fn mint(&mut self, to: H160, amount: u64) {
        // Update balance
        let current_balance = self.balances.get(&to).unwrap_or(0);
        self.balances.put(&to, current_balance + amount);
        
        // Update total supply
        let current_supply = self.total_supply.get().unwrap_or(0);
        self.total_supply.put(current_supply + amount);
    }
    
    #[method]
    pub fn update_user_info(&mut self, account: H160, info: UserInfo) {
        self.user_data.put(&account, info);
    }
}
```

### Deleting from Storage

```rust
impl MyContract {
    #[method]
    pub fn burn(&mut self, from: H160, amount: u64) -> bool {
        let current_balance = self.balances.get(&from).unwrap_or(0);
        
        if current_balance < amount {
            return false;
        }
        
        if current_balance == amount {
            // Remove entry entirely to save gas
            self.balances.delete(&from);
        } else {
            self.balances.put(&from, current_balance - amount);
        }
        
        // Update total supply
        let current_supply = self.total_supply.get().unwrap_or(0);
        self.total_supply.put(current_supply - amount);
        
        true
    }
}
```

## Storage Optimization Techniques

### 1. Use Appropriate Data Types

```rust
// Good: Use smallest suitable integer type
#[storage]
balance: StorageMap<H160, u64>, // u64 for token amounts

// Good: Use Option for nullable fields
#[storage]
optional_data: StorageMap<H160, Option<String>>,

// Avoid: Unnecessarily large types
#[storage]
counter: StorageItem<u128>, // u64 would be sufficient for most counters
```

### 2. Batch Operations

```rust
impl MyContract {
    #[method]
    pub fn batch_transfer(&mut self, transfers: Vec<(H160, H160, u64)>) -> bool {
        // Validate all transfers first
        for (from, to, amount) in &transfers {
            if self.balances.get(from).unwrap_or(0) < *amount {
                return false;
            }
        }
        
        // Execute all transfers
        for (from, to, amount) in transfers {
            let from_balance = self.balances.get(&from).unwrap_or(0);
            let to_balance = self.balances.get(&to).unwrap_or(0);
            
            self.balances.put(&from, from_balance - amount);
            self.balances.put(&to, to_balance + amount);
        }
        
        true
    }
}
```

### 3. Storage Key Optimization

```rust
// Use compact keys for frequently accessed data
#[contract]
pub struct OptimizedContract {
    #[storage]
    balances: StorageMap<H160, u64>, // H160 is compact
    
    // For composite keys, consider using a hash
    #[storage]
    approvals: StorageMap<H256, u64>, // Hash of (owner, spender)
}

impl OptimizedContract {
    fn approval_key(&self, owner: H160, spender: H160) -> H256 {
        // Create deterministic hash for composite key
        let mut hasher = Sha256::new();
        hasher.update(owner.as_bytes());
        hasher.update(spender.as_bytes());
        H256::from(hasher.finalize())
    }
    
    #[method]
    pub fn approve(&mut self, spender: H160, amount: u64) {
        let owner = Runtime::calling_script_hash();
        let key = self.approval_key(owner, spender);
        self.approvals.put(&key, amount);
    }
}
```

### 4. Lazy Deletion

```rust
impl MyContract {
    #[method]
    pub fn close_account(&mut self, account: H160) {
        // Mark account as closed instead of deleting immediately
        self.user_data.put(&account, UserInfo {
            registration_time: 0, // Special value indicating closed
            last_activity: Runtime::time(),
            reputation_score: 0,
        });
        
        // Actual cleanup can be done later in batch
    }
    
    #[method]
    pub fn cleanup_closed_accounts(&mut self, accounts: Vec<H160>) {
        for account in accounts {
            if let Some(info) = self.user_data.get(&account) {
                if info.registration_time == 0 { // Closed account
                    self.user_data.delete(&account);
                    self.balances.delete(&account);
                }
            }
        }
    }
}
```

## Storage Patterns

### 1. Registry Pattern

```rust
#[contract]
pub struct Registry {
    #[storage]
    entries: StorageMap<String, RegistryEntry>,
    
    #[storage]
    entry_count: StorageItem<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct RegistryEntry {
    pub owner: H160,
    pub data: String,
    pub created_at: u64,
}
```

### 2. Multi-Index Pattern

```rust
#[contract]
pub struct MultiIndexContract {
    #[storage]
    items_by_id: StorageMap<u64, Item>,
    
    #[storage]
    items_by_owner: StorageMap<H160, Vec<u64>>,
    
    #[storage]
    items_by_category: StorageMap<String, Vec<u64>>,
}
```

### 3. Pagination Pattern

```rust
impl MyContract {
    #[safe]
    pub fn get_users_paginated(&self, start: u64, limit: u64) -> Vec<H160> {
        let mut users = Vec::new();
        let mut current = start;
        let end = start + limit;
        
        // Implementation depends on how you store user lists
        // This is a simplified example
        while current < end && users.len() < limit as usize {
            if let Some(user) = self.get_user_by_index(current) {
                users.push(user);
            }
            current += 1;
        }
        
        users
    }
}
```

## Gas Optimization

### 1. Minimize Storage Reads

```rust
// Bad: Multiple reads
#[method]
pub fn transfer_bad(&mut self, from: H160, to: H160, amount: u64) -> bool {
    if self.balances.get(&from).unwrap_or(0) < amount {
        return false;
    }
    
    let from_balance = self.balances.get(&from).unwrap_or(0); // Duplicate read
    let to_balance = self.balances.get(&to).unwrap_or(0);
    
    self.balances.put(&from, from_balance - amount);
    self.balances.put(&to, to_balance + amount);
    true
}

// Good: Single read per key
#[method]
pub fn transfer_good(&mut self, from: H160, to: H160, amount: u64) -> bool {
    let from_balance = self.balances.get(&from).unwrap_or(0);
    
    if from_balance < amount {
        return false;
    }
    
    let to_balance = self.balances.get(&to).unwrap_or(0);
    
    self.balances.put(&from, from_balance - amount);
    self.balances.put(&to, to_balance + amount);
    true
}
```

### 2. Use Default Values

```rust
// Good: Use unwrap_or with sensible defaults
let balance = self.balances.get(&account).unwrap_or(0);

// Good: Check existence before expensive operations
if self.user_data.contains_key(&account) {
    let user_info = self.user_data.get(&account).unwrap();
    // Process user info
}
```

## Testing Storage

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_operations() {
        let mut contract = MyContract::new();
        
        // Test initial state
        assert_eq!(contract.get_total_supply(), 0);
        assert_eq!(contract.balance_of(H160::zero()), 0);
        
        // Test storage writes
        contract.mint(H160::zero(), 100);
        assert_eq!(contract.balance_of(H160::zero()), 100);
        assert_eq!(contract.get_total_supply(), 100);
        
        // Test storage deletes
        contract.burn(H160::zero(), 100);
        assert_eq!(contract.balance_of(H160::zero()), 0);
        assert_eq!(contract.get_total_supply(), 0);
    }
}
```

## Common Pitfalls

### 1. Storage Key Collisions

```rust
// Bad: Potential key collision
#[storage]
map1: StorageMap<String, u64>,
#[storage] 
map2: StorageMap<String, String>,

// Good: Use different prefixes or types
#[storage]
balances: StorageMap<H160, u64>,
#[storage]
metadata: StorageMap<H160, String>,
```

### 2. Expensive Serialization

```rust
// Avoid storing very large structures
#[derive(Serialize, Deserialize)]
pub struct HugeStruct {
    pub large_array: [u8; 1000000], // This will be expensive to serialize
}

// Better: Break into smaller pieces or use references
#[derive(Serialize, Deserialize)]
pub struct OptimizedStruct {
    pub data_hash: H256, // Reference to off-chain data
    pub size: u64,
    pub created_at: u64,
}
```

## See Also

- [Events Guide](events_guide.md)
- [Contract Security Guide](contract_security_guide.md)
- [Gas Optimization Guide](gas_optimization.md)
- [Examples](../examples/README.md) 