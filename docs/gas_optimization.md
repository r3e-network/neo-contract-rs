# Neo N3 Gas Optimization Guide

This guide explores techniques and best practices for optimizing gas usage in Neo N3 smart contracts developed with the Neo Contract Rust framework.

## Introduction

Gas is the computational cost unit in Neo N3 that's charged for executing operations on the blockchain. Optimizing your contract for gas efficiency has several benefits:

- **Lower Cost**: More efficient contracts cost less to execute
- **User Experience**: Lower fees lead to better user experience
- **Throughput**: Efficient contracts enable more transactions per block
- **Inclusivity**: Lower costs enable a broader range of users to interact with your contract

## Understanding Neo N3 Gas Costs

### Basic Gas Principles

In Neo N3, there are two types of gas fees:

1. **System Fee**: The cost of executing the contract operations
2. **Network Fee**: The cost of including the transaction in the blockchain

Gas costs in Neo N3 are determined by:

- Operation complexity
- Memory/storage usage
- Data size in transactions
- Current network conditions

### Common Gas-Intensive Operations

The following operations typically consume more gas:

1. **Storage Operations**: Writing to and reading from blockchain storage
2. **Contract Calls**: Invoking other contracts
3. **Complex Calculations**: Mathematical operations, especially in loops
4. **Large Data Processing**: Working with large arrays or strings
5. **Events**: Emitting events with many parameters

## Optimization Strategies

### 1. Storage Optimization

Storage operations are among the most expensive in terms of gas consumption.

#### Minimize Storage Writes

```rust
// Inefficient: Multiple writes
fn update_values(&mut self, a: u64, b: u64, c: u64) {
    self.value_a.insert(&a);
    self.value_b.insert(&b);
    self.value_c.insert(&c);
}

// More efficient: Batch related data
fn update_values(&mut self, a: u64, b: u64, c: u64) {
    self.values.insert(&(a, b, c));
}
```

#### Use Appropriate Data Structures

```rust
// Inefficient: Storing each token balance individually
// self.balances: Map<(H160, H160), u64> // (token, user) -> balance

// More efficient: Group by token
// self.balances: Map<H160, Map<H160, u64>> // token -> (user -> balance)
```

#### Delete Unused Storage

```rust
// Free up storage when no longer needed
fn close_account(&mut self, account: H160) {
    // Remove all user data
    self.balances.remove(&account);
    self.allowances.remove(&account);
    self.user_data.remove(&account);
}
```

### 2. Computation Optimization

#### Avoid Redundant Calculations

```rust
// Inefficient: Recalculating in a loop
fn get_total_value(&self, token_ids: Vec<u64>) -> u64 {
    let mut total = 0;
    for id in token_ids {
        let token_price = self.calculate_complex_price(id);
        total += token_price;
    }
    total
}

// More efficient: Calculate once and cache
fn get_total_value(&self, token_ids: Vec<u64>) -> u64 {
    let mut total = 0;
    let mut price_cache = Map::new();
    
    for id in token_ids {
        if !price_cache.contains_key(&id) {
            price_cache.insert(id, self.calculate_complex_price(id));
        }
        total += price_cache.get(&id).unwrap();
    }
    total
}
```

#### Optimize Loops

```rust
// Inefficient: Unnecessary operations in loop
fn process_large_array(&mut self, data: Vec<u64>) {
    let len = data.len(); // Calculate once outside loop
    for i in 0..len {
        // Process item
        self.do_something(data[i]);
    }
}
```

#### Use Efficient Algorithms

```rust
// Inefficient: O(n²) lookup
fn find_matches(&self, items: Vec<u64>, target: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for item in items {
        if item == target {
            result.push(item);
        }
    }
    result
}

// More efficient: O(1) lookup with a set/map
fn find_matches(&self, items: Vec<u64>, target: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut item_set = HashSet::new();
    
    // Convert to set first (O(n))
    for item in &items {
        item_set.insert(*item);
    }
    
    // O(1) lookup
    if item_set.contains(&target) {
        result.push(target);
    }
    
    result
}
```

### 3. Contract Interactions

#### Minimize Cross-Contract Calls

```rust
// Inefficient: Multiple separate calls
fn process_token_transfer(&mut self, token: H160, from: H160, to: H160, amount: u64) {
    let token_name = self.call_contract(token, "name", Vec::new());
    let token_symbol = self.call_contract(token, "symbol", Vec::new());
    let token_decimals = self.call_contract(token, "decimals", Vec::new());
    
    // Process transfer with the information
    // ...
}

// More efficient: Batch calls or cache results
fn process_token_transfer(&mut self, token: H160, from: H160, to: H160, amount: u64) {
    // Check if token data is already cached
    if !self.token_data.contains_key(&token) {
        let name = self.call_contract(token, "name", Vec::new());
        let symbol = self.call_contract(token, "symbol", Vec::new());
        let decimals = self.call_contract(token, "decimals", Vec::new());
        
        self.token_data.insert(&token, &(name, symbol, decimals));
    }
    
    let (name, symbol, decimals) = self.token_data.get(&token).unwrap();
    
    // Process transfer with the cached information
    // ...
}
```

#### Implement View Functions

Using read-only (view) functions when no state changes are needed avoids unnecessary gas costs:

```rust
#[method]
#[safe]
pub fn get_token_info(&self, token_id: u64) -> TokenInfo {
    // Read-only function that doesn't modify state
    self.token_info.get(&token_id).unwrap_or_default()
}
```

### 4. Data Size Optimization

#### Minimize Input and Output Data

```rust
// Inefficient: Returning a large struct with many fields
#[method]
pub fn get_full_account_data(&self, account: H160) -> AccountData {
    // Return large struct with all account data
    // ...
}

// More efficient: Split into smaller, focused methods
#[method]
pub fn get_account_balance(&self, account: H160) -> u64 {
    // Return just the balance
    // ...
}

#[method]
pub fn get_account_status(&self, account: H160) -> u8 {
    // Return just the status
    // ...
}
```

#### Use Compact Data Representations

```rust
// Inefficient: Storing address as string
struct UserData {
    address_string: String, // E.g. "0x1234567890abcdef1234567890abcdef12345678"
    name: String,
    balance: u64,
}

// More efficient: Use native types
struct UserData {
    address: H160, // Compact binary representation
    name: String,
    balance: u64,
}
```

#### Batch Operations When Possible

```rust
// Inefficient: Processing items one by one
for item in items {
    self.process_item(item);
}

// More efficient: Process in batches
self.process_items(items);
```

### 5. Event Optimization

#### Emit Necessary Events Only

```rust
// Inefficient: Emitting events for intermediate steps
fn multi_step_process(&mut self) {
    // Step 1
    self.do_step_one();
    Runtime::notify(b"StepOneCompleted", &[]);
    
    // Step 2
    self.do_step_two();
    Runtime::notify(b"StepTwoCompleted", &[]);
    
    // Step 3
    self.do_step_three();
    Runtime::notify(b"StepThreeCompleted", &[]);
    
    // Final notification
    Runtime::notify(b"ProcessCompleted", &[]);
}

// More efficient: Emit only the final event
fn multi_step_process(&mut self) {
    // Step 1
    self.do_step_one();
    
    // Step 2
    self.do_step_two();
    
    // Step 3
    self.do_step_three();
    
    // Only emit the necessary event
    Runtime::notify(b"ProcessCompleted", &[]);
}
```

#### Optimize Event Parameters

```rust
// Inefficient: Including unnecessary data
fn transfer(&mut self, from: H160, to: H160, amount: u64) {
    // Process transfer
    // ...
    
    // Emit event with all contract data
    Runtime::notify(b"Transfer", &[
        &from, 
        &to, 
        &amount, 
        &self.token_name, 
        &self.token_symbol, 
        &self.total_supply
    ]);
}

// More efficient: Include only relevant parameters
fn transfer(&mut self, from: H160, to: H160, amount: u64) {
    // Process transfer
    // ...
    
    // Emit event with minimum required data
    Runtime::notify(b"Transfer", &[&from, &to, &amount]);
}
```

## Advanced Optimization Techniques

### 1. Use Lazy Loading

Load data only when needed:

```rust
fn get_user_profile(&self, user: H160) -> UserProfile {
    // Check if basic profile exists
    if let Some(basic_profile) = self.basic_profiles.get(&user) {
        // Only load extended data if needed for this user
        if basic_profile.has_extended_data {
            let extended_data = self.extended_profiles.get(&user).unwrap_or_default();
            return UserProfile::new(basic_profile, Some(extended_data));
        }
        return UserProfile::new(basic_profile, None);
    }
    
    // Return default profile if not found
    UserProfile::default()
}
```

### 2. Implement Pagination

When dealing with large collections, implement pagination:

```rust
#[method]
pub fn get_token_list(&self, start_idx: u32, count: u32) -> Vec<TokenInfo> {
    let mut result = Vec::new();
    let total = self.token_count.get().unwrap_or(0);
    
    // Validate inputs
    let start = start_idx.min(total);
    let end = (start + count).min(total);
    
    // Return only requested page
    for i in start..end {
        if let Some(token) = self.tokens.get(&i) {
            result.push(token);
        }
    }
    
    result
}
```

### 3. State Channel Pattern

For complex multi-step operations, consider using a state channel pattern:

```rust
enum ProcessState {
    NotStarted,
    Step1Completed,
    Step2Completed,
    Completed,
}

fn start_process(&mut self, user: H160) {
    // Initialize process state
    self.process_states.insert(&user, &ProcessState::NotStarted);
    
    // Execute first step
    self.execute_step_1(user);
}

fn continue_process(&mut self, user: H160) {
    let current_state = self.process_states.get(&user).unwrap_or(ProcessState::NotStarted);
    
    match current_state {
        ProcessState::NotStarted => {
            self.execute_step_1(user);
        },
        ProcessState::Step1Completed => {
            self.execute_step_2(user);
        },
        ProcessState::Step2Completed => {
            self.finalize_process(user);
            self.process_states.insert(&user, &ProcessState::Completed);
        },
        ProcessState::Completed => {
            // Process already completed
        }
    }
}
```

### 4. Use Bitmap for Flags

When you need to store multiple boolean flags, use a bitmap instead of separate storage:

```rust
// Inefficient: Multiple storage items for flags
struct UserFlags {
    is_verified: Item<bool>,
    is_admin: Item<bool>,
    is_active: Item<bool>,
    has_paid: Item<bool>,
    // ...many more flags
}

// More efficient: Single u32/u64 bitmap
struct UserFlags {
    flags: Item<u32>, // Bitmap where each bit represents a flag
}

impl UserFlags {
    const VERIFIED: u32 = 1;      // 0b00000001
    const ADMIN: u32 = 1 << 1;    // 0b00000010
    const ACTIVE: u32 = 1 << 2;   // 0b00000100
    const PAID: u32 = 1 << 3;     // 0b00001000
    
    fn is_verified(&self) -> bool {
        (self.flags.get() & Self::VERIFIED) != 0
    }
    
    fn set_verified(&mut self, value: bool) {
        let mut flags = self.flags.get();
        if value {
            flags |= Self::VERIFIED;
        } else {
            flags &= !Self::VERIFIED;
        }
        self.flags.set(flags);
    }
    
    // Similar methods for other flags
}
```

## Testing Gas Efficiency

### 1. Gas Profiling

Profile your contract to identify gas usage hotspots:

```rust
// Example of a simple gas profiling test
#[test]
fn test_gas_usage() {
    let mut contract = MyContract::new();
    
    // Profile storage operation
    let storage_start = get_gas_used();
    contract.store_value(42);
    let storage_gas = get_gas_used() - storage_start;
    println!("Storage operation gas usage: {}", storage_gas);
    
    // Profile calculation operation
    let calc_start = get_gas_used();
    contract.complex_calculation(100);
    let calc_gas = get_gas_used() - calc_start;
    println!("Calculation operation gas usage: {}", calc_gas);
    
    // Compare and identify the most expensive operations
}
```

### 2. A/B Testing Different Implementations

Test different implementations to compare gas usage:

```rust
#[test]
fn compare_implementation_efficiency() {
    let mut contract = MyContract::new();
    
    // Test implementation A
    let start_a = get_gas_used();
    contract.process_data_implementation_a(test_data.clone());
    let gas_a = get_gas_used() - start_a;
    
    // Reset contract state
    let mut contract = MyContract::new();
    
    // Test implementation B
    let start_b = get_gas_used();
    contract.process_data_implementation_b(test_data.clone());
    let gas_b = get_gas_used() - start_b;
    
    println!("Implementation A: {} gas", gas_a);
    println!("Implementation B: {} gas", gas_b);
    println!("Savings: {} gas ({}%)", 
        gas_a - gas_b, 
        ((gas_a - gas_b) as f64 / gas_a as f64) * 100.0
    );
}
```

## Neo N3-Specific Optimizations

### 1. Use Native Contracts When Possible

Neo N3 provides native contracts that are more gas-efficient than user contracts:

```rust
// Inefficient: Implementing your own GAS token transfer logic
fn transfer_gas(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Custom implementation
    // ...
}

// More efficient: Use the native GAS token contract
fn transfer_gas(&mut self, from: H160, to: H160, amount: u64) -> bool {
    let gas_script_hash = H160::from_str("0xd2a4cff31913016155e38e474a2c06d08be276cf").unwrap();
    let params = [
        RuntimeValue::from(from),
        RuntimeValue::from(to),
        RuntimeValue::from(amount)
    ];
    
    // Call the native contract
    let result = Runtime::call(gas_script_hash, "transfer", params);
    result == true.into()
}
```

### 2. Leverage Native Cryptography

Use native cryptographic functions instead of custom implementations:

```rust
// Use Neo's native cryptography for hashing
fn calculate_hash(&self, data: &[u8]) -> H256 {
    let engine = CryptoLib::sha256();
    engine.compute(data)
}
```

### 3. Optimize for Neo VM

Understand how Neo VM executes code and optimize accordingly:

```rust
// Inefficient: Complex conditions with multiple checks
fn process_transaction(&mut self, tx: Transaction) -> bool {
    if tx.value > 0 && 
       tx.from != H160::zero() && 
       tx.to != H160::zero() && 
       self.is_valid_signature(tx.signature) && 
       !self.blacklist.contains(&tx.from) {
        // Process transaction
        return true;
    }
    false
}

// More efficient: Early returns to avoid unnecessary checks
fn process_transaction(&mut self, tx: Transaction) -> bool {
    // Check each condition separately with early returns
    if tx.value == 0 {
        return false;
    }
    
    if tx.from == H160::zero() || tx.to == H160::zero() {
        return false;
    }
    
    if !self.is_valid_signature(tx.signature) {
        return false;
    }
    
    if self.blacklist.contains(&tx.from) {
        return false;
    }
    
    // Process transaction
    true
}
```

## Gas Optimization Checklist

Before deploying your contract, review this checklist:

1. **Storage**
   - [ ] Minimized number of storage writes
   - [ ] Used appropriate data structures for your use case
   - [ ] Removed unused storage to reclaim space

2. **Computation**
   - [ ] Avoided redundant calculations
   - [ ] Optimized loops and removed unnecessary operations
   - [ ] Used efficient algorithms with better time complexity

3. **Contract Interactions**
   - [ ] Minimized number of cross-contract calls
   - [ ] Used view functions for read-only operations
   - [ ] Cached results of expensive calls

4. **Data Size**
   - [ ] Minimized input and output data sizes
   - [ ] Used compact data representations
   - [ ] Implemented pagination for large data sets

5. **Events**
   - [ ] Emitted only necessary events
   - [ ] Included only relevant data in event parameters

6. **Advanced Techniques**
   - [ ] Implemented lazy loading where appropriate
   - [ ] Used bitmaps for storing multiple flags
   - [ ] Leveraged Neo N3 native contracts when possible

## Conclusion

Gas optimization is a crucial aspect of smart contract development on Neo N3. By implementing the techniques described in this guide, you can significantly reduce the gas costs of your contracts, improving both user experience and contract efficiency.

Remember that optimization should be balanced with readability and maintainability. Only apply optimizations where they provide meaningful benefits, and always test thoroughly to ensure optimized code functions correctly.

For specific gas costs of Neo N3 operations, refer to the [Neo N3 documentation](https://docs.neo.org/docs/en-us/basic/concept/fee.html) for the most up-to-date information. 