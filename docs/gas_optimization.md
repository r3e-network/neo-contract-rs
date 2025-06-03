# Neo Contract Gas Optimization Guide

This guide provides strategies and techniques for optimizing gas consumption in Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Gas optimization is crucial for reducing transaction costs and improving user experience. Neo N3 charges gas for computation, storage operations, and network usage. Understanding these costs helps you write more efficient contracts.

## Gas Cost Model

### Storage Operations
- **Storage read**: ~200 GAS per read
- **Storage write**: ~1000 GAS per write (first time), ~300 GAS (update)
- **Storage delete**: ~100 GAS per deletion

### Computation
- **Basic operations**: 1-10 GAS
- **Complex operations**: 10-100+ GAS
- **Cryptographic operations**: 100-1000+ GAS

### Network
- **Contract deployment**: Based on contract size
- **Method invocation**: Base cost + computation + storage

## Storage Optimization

### 1. Minimize Storage Reads/Writes

```rust
// Bad: Multiple reads of the same key
#[method]
pub fn inefficient_transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    if self.balances.get(&from).unwrap_or(0) < amount { // Read 1
        return false;
    }
    
    let from_balance = self.balances.get(&from).unwrap_or(0); // Read 2 (duplicate!)
    let to_balance = self.balances.get(&to).unwrap_or(0);     // Read 3
    
    self.balances.put(&from, from_balance - amount);          // Write 1
    self.balances.put(&to, to_balance + amount);              // Write 2
    true
}

// Good: Single read per key
#[method]
pub fn efficient_transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    let from_balance = self.balances.get(&from).unwrap_or(0); // Read 1
    
    if from_balance < amount {
        return false;
    }
    
    let to_balance = self.balances.get(&to).unwrap_or(0);     // Read 2
    
    self.balances.put(&from, from_balance - amount);          // Write 1
    self.balances.put(&to, to_balance + amount);              // Write 2
    true
}
```

### 2. Batch Operations

```rust
// Bad: Individual operations
#[method]
pub fn distribute_tokens_inefficient(&mut self, recipients: Vec<(H160, u64)>) {
    for (recipient, amount) in recipients {
        let balance = self.balances.get(&recipient).unwrap_or(0);
        self.balances.put(&recipient, balance + amount);
    }
}

// Good: Batch with validation
#[method]
pub fn distribute_tokens_efficient(&mut self, recipients: Vec<(H160, u64)>) -> bool {
    let total_amount: u64 = recipients.iter().map(|(_, amount)| amount).sum();
    let contract_balance = self.total_supply.get().unwrap_or(0);
    
    if contract_balance < total_amount {
        return false;
    }
    
    // Single validation, then batch updates
    for (recipient, amount) in recipients {
        let current_balance = self.balances.get(&recipient).unwrap_or(0);
        self.balances.put(&recipient, current_balance + amount);
    }
    
    true
}
```

### 3. Use Compact Data Types

```rust
// Bad: Unnecessarily large types
#[contract]
pub struct InefficientContract {
    #[storage]
    small_counter: StorageItem<u128>, // u32 would suffice
    
    #[storage]
    flags: StorageMap<H160, String>, // bool would be better for flags
}

// Good: Right-sized types
#[contract]
pub struct EfficientContract {
    #[storage]
    small_counter: StorageItem<u32>, // Appropriate size
    
    #[storage]
    flags: StorageMap<H160, bool>, // Compact boolean flags
    
    #[storage]
    balances: StorageMap<H160, u64>, // Standard token amount size
}
```

### 4. Smart Storage Key Management

```rust
// Good: Use composite keys efficiently
impl MyContract {
    // Pack multiple values into single storage key
    fn pack_user_data(&self, account: H160, data: UserStats) {
        let packed = PackedUserData {
            last_activity: data.last_activity,
            transaction_count: data.transaction_count,
            reputation: data.reputation,
        };
        self.user_data.put(&account, packed);
    }
    
    // Use hash for complex composite keys
    fn approval_key(&self, owner: H160, spender: H160) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(owner.as_bytes());
        hasher.update(spender.as_bytes());
        H256::from(hasher.finalize())
    }
}
```

## Computation Optimization

### 1. Early Returns

```rust
// Good: Exit early to save gas
#[method]
pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
    // Cheapest checks first
    if amount == 0 {
        return false; // Exit before expensive operations
    }
    
    if to == H160::zero() {
        return false;
    }
    
    let caller = Runtime::calling_script_hash();
    let from_balance = self.balances.get(&caller).unwrap_or(0);
    
    if from_balance < amount {
        return false; // Exit before storage writes
    }
    
    // Only do expensive operations if all checks pass
    let to_balance = self.balances.get(&to).unwrap_or(0);
    self.balances.put(&caller, from_balance - amount);
    self.balances.put(&to, to_balance + amount);
    
    true
}
```

### 2. Avoid Unnecessary Computations

```rust
// Bad: Redundant calculations
#[method]
pub fn calculate_fees_inefficient(&self, amounts: Vec<u64>) -> Vec<u64> {
    let mut fees = Vec::new();
    let fee_rate = self.fee_rate.get().unwrap_or(0);
    
    for amount in amounts {
        // Repeated division - expensive!
        let fee = (amount * fee_rate) / 10000;
        fees.push(fee);
    }
    
    fees
}

// Good: Pre-calculate when possible
#[method]
pub fn calculate_fees_efficient(&self, amounts: Vec<u64>) -> Vec<u64> {
    let fee_rate = self.fee_rate.get().unwrap_or(0);
    
    // Pre-calculate if fee rate is zero
    if fee_rate == 0 {
        return vec![0; amounts.len()];
    }
    
    amounts.into_iter()
        .map(|amount| (amount * fee_rate) / 10000)
        .collect()
}
```

### 3. Use Appropriate Algorithms

```rust
// Bad: O(n) search for each lookup
#[method]
pub fn find_user_inefficient(&self, users: Vec<H160>, target: H160) -> Option<usize> {
    for (i, user) in users.iter().enumerate() {
        if *user == target {
            return Some(i);
        }
    }
    None
}

// Good: Use efficient data structures
#[contract]
pub struct EfficientContract {
    #[storage]
    user_indices: StorageMap<H160, u32>, // O(1) lookup
    
    #[storage]
    users: StorageMap<u32, H160>, // Indexed access
}

impl EfficientContract {
    #[method]
    pub fn find_user_efficient(&self, target: H160) -> Option<u32> {
        self.user_indices.get(&target)
    }
}
```

## Memory Optimization

### 1. Minimize Vector Allocations

```rust
// Bad: Multiple allocations
#[method]
pub fn process_data_inefficient(&self, data: Vec<u64>) -> Vec<u64> {
    let mut result1 = Vec::new();
    let mut result2 = Vec::new();
    
    for item in data {
        if item > 100 {
            result1.push(item * 2);
        } else {
            result2.push(item);
        }
    }
    
    // Merge vectors (another allocation)
    result1.extend(result2);
    result1
}

// Good: Single allocation with capacity
#[method]
pub fn process_data_efficient(&self, data: Vec<u64>) -> Vec<u64> {
    let mut result = Vec::with_capacity(data.len());
    
    for item in data {
        if item > 100 {
            result.push(item * 2);
        } else {
            result.push(item);
        }
    }
    
    result
}
```

### 2. Reuse Existing Storage

```rust
// Good: Reuse storage slots
impl MyContract {
    #[method]
    pub fn cleanup_and_set(&mut self, old_key: H160, new_key: H160, value: u64) {
        // Remove old entry
        self.data.delete(&old_key);
        
        // Reuse the freed storage slot
        self.data.put(&new_key, value);
    }
    
    #[method]
    pub fn batch_cleanup(&mut self, keys_to_remove: Vec<H160>) {
        // Batch deletions to free up storage
        for key in keys_to_remove {
            self.data.delete(&key);
        }
    }
}
```

## Event Optimization

### 1. Minimize Event Data

```rust
// Bad: Large event data
#[event]
pub struct VerboseTransfer {
    pub transaction_id: String,      // Expensive string
    pub from_address: H160,
    pub to_address: H160,
    pub amount: u64,
    pub timestamp: u64,
    pub gas_used: u64,
    pub description: String,         // Another expensive string
}

// Good: Compact event data
#[event]
pub struct EfficientTransfer {
    #[index]
    pub from: H160,
    #[index]
    pub to: H160,
    pub amount: u64,
}
```

### 2. Selective Event Emission

```rust
impl MyContract {
    #[method]
    pub fn transfer_with_optional_event(&mut self, to: H160, amount: u64, emit_event: bool) -> bool {
        let caller = Runtime::calling_script_hash();
        
        // Core transfer logic
        let from_balance = self.balances.get(&caller).unwrap_or(0);
        if from_balance < amount {
            return false;
        }
        
        let to_balance = self.balances.get(&to).unwrap_or(0);
        self.balances.put(&caller, from_balance - amount);
        self.balances.put(&to, to_balance + amount);
        
        // Only emit event if requested (saves gas)
        if emit_event {
            self.emit_transfer_event(caller, to, amount);
        }
        
        true
    }
}
```

## Cross-Contract Call Optimization

### 1. Minimize External Calls

```rust
// Bad: Multiple external calls
#[method]
pub fn get_token_info_inefficient(&self, token: H160) -> (String, u8, u64) {
    let symbol = Runtime::call_contract(token, "symbol", &[]).unwrap();
    let decimals = Runtime::call_contract(token, "decimals", &[]).unwrap();
    let total_supply = Runtime::call_contract(token, "totalSupply", &[]).unwrap();
    
    (symbol, decimals, total_supply)
}

// Good: Batch call if possible, or cache results
#[contract]
pub struct OptimizedContract {
    #[storage]
    token_cache: StorageMap<H160, TokenInfo>,
}

impl OptimizedContract {
    #[method]
    pub fn get_token_info_efficient(&mut self, token: H160) -> Option<TokenInfo> {
        // Check cache first
        if let Some(cached) = self.token_cache.get(&token) {
            return Some(cached);
        }
        
        // Make single batch call if possible
        if let Some(info) = self.fetch_token_info_batch(token) {
            self.token_cache.put(&token, info.clone());
            Some(info)
        } else {
            None
        }
    }
}
```

## Contract Structure Optimization

### 1. Efficient Contract Layout

```rust
// Good: Organize storage by access patterns
#[contract]
pub struct WellOrganizedContract {
    // Frequently accessed together
    #[storage]
    balances: StorageMap<H160, u64>,
    
    #[storage]
    total_supply: StorageItem<u64>,
    
    // Administrative data (less frequently accessed)
    #[storage]
    owner: StorageItem<H160>,
    
    #[storage]
    paused: StorageItem<bool>,
    
    // Historical data (rarely accessed)
    #[storage]
    transaction_history: StorageMap<H256, TransactionRecord>,
}
```

### 2. Lazy Loading

```rust
impl MyContract {
    // Load expensive data only when needed
    #[safe]
    pub fn get_user_stats(&self, user: H160) -> UserStats {
        UserStats {
            balance: self.balances.get(&user).unwrap_or(0),
            // Only load transaction count if explicitly requested
            transaction_count: 0, // Placeholder
        }
    }
    
    #[safe]
    pub fn get_detailed_user_stats(&self, user: H160) -> DetailedUserStats {
        let basic_stats = self.get_user_stats(user);
        
        DetailedUserStats {
            balance: basic_stats.balance,
            transaction_count: self.transaction_counts.get(&user).unwrap_or(0),
            last_activity: self.last_activity.get(&user).unwrap_or(0),
            // Load expensive historical data only when needed
        }
    }
}
```

## Testing Gas Usage

### 1. Gas Measurement in Tests

```rust
#[cfg(test)]
mod gas_tests {
    use super::*;
    
    #[test]
    fn test_transfer_gas_usage() {
        let mut contract = MyContract::new();
        
        // Set up test data
        contract.balances.put(&alice, 1000);
        
        // Measure gas for operation
        let gas_before = Runtime::gas_left();
        contract.transfer(bob, 100);
        let gas_used = gas_before - Runtime::gas_left();
        
        // Assert gas usage is within expected range
        assert!(gas_used < 50000, "Transfer used too much gas: {}", gas_used);
    }
    
    #[test]
    fn compare_batch_vs_individual() {
        let mut contract = MyContract::new();
        
        // Test individual operations
        let gas_before = Runtime::gas_left();
        for i in 0..10 {
            contract.mint(H160::from_low_u64_be(i), 100);
        }
        let individual_gas = gas_before - Runtime::gas_left();
        
        // Reset contract
        contract = MyContract::new();
        
        // Test batch operation
        let recipients: Vec<(H160, u64)> = (0..10)
            .map(|i| (H160::from_low_u64_be(i), 100))
            .collect();
        
        let gas_before = Runtime::gas_left();
        contract.batch_mint(recipients);
        let batch_gas = gas_before - Runtime::gas_left();
        
        // Batch should be more efficient
        assert!(batch_gas < individual_gas);
    }
}
```

## Gas Optimization Checklist

### Storage
- [ ] Minimize storage reads/writes
- [ ] Use appropriate data types
- [ ] Batch operations when possible
- [ ] Delete unused storage entries
- [ ] Cache frequently accessed data

### Computation
- [ ] Use early returns
- [ ] Avoid redundant calculations
- [ ] Choose efficient algorithms
- [ ] Pre-calculate constants

### Memory
- [ ] Minimize vector allocations
- [ ] Reuse data structures
- [ ] Use appropriate collection types

### Events
- [ ] Keep event data minimal
- [ ] Use indexed fields wisely
- [ ] Emit events selectively

### External Calls
- [ ] Minimize cross-contract calls
- [ ] Cache external data
- [ ] Use batch calls when available

## Performance Monitoring

```rust
impl MyContract {
    // Add gas profiling to expensive operations
    #[method]
    pub fn expensive_operation(&mut self, data: Vec<u64>) -> Vec<u64> {
        let start_gas = Runtime::gas_left();
        
        let result = self.process_large_dataset(data);
        
        let gas_used = start_gas - Runtime::gas_left();
        
        // Log gas usage for monitoring
        Runtime::log(&format!("Operation used {} gas", gas_used));
        
        result
    }
}
```

## See Also

- [Storage Guide](storage_guide.md)
- [Contract Security Guide](contract_security_guide.md)
- [Events Guide](events_guide.md)
- [Cross-Contract Communication Guide](cross_contract_guide.md) 