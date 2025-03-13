# Neo N3 Storage Guide

This guide covers how to effectively work with on-chain storage in Neo N3 smart contracts developed with the Neo Contract Rust framework.

## Introduction

On-chain storage is a fundamental component of blockchain smart contracts, allowing contracts to maintain state across transactions and blocks. The Neo Contract Rust framework provides type-safe abstractions for working with the Neo N3 storage system.

## Storage Basics

### Storage Scope

In Neo N3, storage is:

- **Persistent**: Data remains until explicitly deleted
- **Contract-scoped**: Each contract has its own isolated storage space
- **Key-value based**: Data is stored and retrieved using keys
- **Byte-oriented**: All data is ultimately stored as bytes

### Storage Costs

Storage operations consume GAS:

- **Writing** to storage is more expensive than reading
- **Creating** new storage entries costs more than updating existing ones
- **Deleting** storage frees up space and returns some GAS

## Storage Abstractions

The Neo Contract Rust framework provides several abstractions for working with storage:

### 1. Storage Items

`StorageItem<T>` represents a single value stored under a key:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct MyContract {
    // Single value storage items
    counter: StorageItem<u64>,
    name: StorageItem<String>,
    owner: StorageItem<H160>,
    is_paused: StorageItem<bool>,
}

#[contractimpl]
impl MyContract {
    #[constructor]
    pub fn new(owner: H160) -> Self {
        Self {
            counter: StorageItem::new(0),
            name: StorageItem::new("MyContract".to_string()),
            owner: StorageItem::new(owner),
            is_paused: StorageItem::new(false),
        }
    }
    
    #[method]
    pub fn increment(&mut self) -> u64 {
        let current = self.counter.get();
        let new_value = current + 1;
        self.counter.set(new_value);
        new_value
    }
    
    #[method]
    pub fn get_counter(&self) -> u64 {
        self.counter.get()
    }
}
```

### 2. Storage Maps

`StorageMap<K, V>` represents a key-value mapping:

```rust
#[contract]
pub struct TokenContract {
    // Map from address to balance
    balances: StorageMap<H160, u64>,
    
    // Map from (owner, spender) to allowance
    allowances: StorageMap<(H160, H160), u64>,
    
    // Map from token ID to token data
    tokens: StorageMap<u64, TokenData>,
}

#[contractimpl]
impl TokenContract {
    #[method]
    pub fn get_balance(&self, address: H160) -> u64 {
        self.balances.get(&address).unwrap_or_default()
    }
    
    #[method]
    pub fn set_balance(&mut self, address: H160, amount: u64) {
        self.balances.insert(&address, &amount);
    }
    
    #[method]
    pub fn get_allowance(&self, owner: H160, spender: H160) -> u64 {
        self.allowances.get(&(owner, spender)).unwrap_or_default()
    }
    
    #[method]
    pub fn get_token(&self, token_id: u64) -> Option<TokenData> {
        self.tokens.get(&token_id)
    }
}
```

### 3. Storage Iterators

For collections that need to be iterated:

```rust
#[contract]
pub struct Registry {
    // Vector of registered addresses
    addresses: StorageList<H160>,
    
    // Set of active members
    active_members: StorageSet<H160>,
}

#[contractimpl]
impl Registry {
    #[method]
    pub fn add_address(&mut self, address: H160) {
        self.addresses.push(&address);
    }
    
    #[method]
    pub fn get_all_addresses(&self) -> Vec<H160> {
        let mut result = Vec::new();
        for address in self.addresses.iter() {
            result.push(address);
        }
        result
    }
    
    #[method]
    pub fn add_member(&mut self, member: H160) {
        self.active_members.insert(&member);
    }
    
    #[method]
    pub fn is_member(&self, address: H160) -> bool {
        self.active_members.contains(&address)
    }
}
```

## Working with Complex Data Types

### Custom Structs

To store custom data structures, implement the necessary traits:

```rust
use neo_contract::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct UserProfile {
    username: String,
    email: String,
    reputation: u64,
    is_verified: bool,
    created_at: u64,
}

#[contract]
pub struct UserRegistry {
    // Map from address to user profile
    profiles: StorageMap<H160, UserProfile>,
}

#[contractimpl]
impl UserRegistry {
    #[method]
    pub fn register_user(&mut self, address: H160, username: String, email: String) {
        let profile = UserProfile {
            username,
            email,
            reputation: 0,
            is_verified: false,
            created_at: Ledger::current_timestamp(),
        };
        
        self.profiles.insert(&address, &profile);
    }
    
    #[method]
    pub fn get_profile(&self, address: H160) -> Option<UserProfile> {
        self.profiles.get(&address)
    }
    
    #[method]
    pub fn update_reputation(&mut self, address: H160, new_reputation: u64) -> bool {
        if let Some(mut profile) = self.profiles.get(&address) {
            profile.reputation = new_reputation;
            self.profiles.insert(&address, &profile);
            true
        } else {
            false
        }
    }
}
```

### Nested Collections

You can create complex data structures with nested collections:

```rust
#[contract]
pub struct Marketplace {
    // Map from category ID to set of product IDs in that category
    category_products: StorageMap<u64, StorageSet<u64>>,
    
    // Map from product ID to product data
    products: StorageMap<u64, ProductData>,
}

#[contractimpl]
impl Marketplace {
    #[method]
    pub fn add_product_to_category(&mut self, category_id: u64, product_id: u64) {
        // Ensure the category exists
        if !self.category_products.contains_key(&category_id) {
            let products = StorageSet::new();
            self.category_products.insert(&category_id, &products);
        }
        
        // Add product to category
        let mut products = self.category_products.get(&category_id).unwrap();
        products.insert(&product_id);
        self.category_products.insert(&category_id, &products);
    }
    
    #[method]
    pub fn get_products_in_category(&self, category_id: u64) -> Vec<u64> {
        let mut result = Vec::new();
        
        if let Some(products) = self.category_products.get(&category_id) {
            for product_id in products.iter() {
                result.push(product_id);
            }
        }
        
        result
    }
}
```

## Storage Patterns

### 1. Storage Prefix Pattern

Use prefixes to organize storage keys and prevent collisions:

```rust
#[contract]
pub struct MultiTokenContract {
    // Prefixed storage maps for different token types
    fungible_balances: StorageMap<(u64, H160), u64>,  // (token_id, address) -> balance
    non_fungible_owners: StorageMap<u64, H160>,  // token_id -> owner
}

#[contractimpl]
impl MultiTokenContract {
    #[method]
    pub fn get_fungible_balance(&self, token_id: u64, address: H160) -> u64 {
        self.fungible_balances.get(&(token_id, address)).unwrap_or_default()
    }
    
    #[method]
    pub fn get_nft_owner(&self, token_id: u64) -> Option<H160> {
        self.non_fungible_owners.get(&token_id)
    }
}
```

### 2. Lazy Loading Pattern

Load data only when needed:

```rust
#[contract]
pub struct LazyContract {
    // Map from user to profile
    profiles: StorageMap<H160, UserProfile>,
    
    // Map from user to activity data (potentially large)
    activity_data: StorageMap<H160, ActivityData>,
}

#[contractimpl]
impl LazyContract {
    #[method]
    pub fn get_user_summary(&self, user: H160) -> UserSummary {
        // Always load the basic profile (small)
        let profile = self.profiles.get(&user).unwrap_or_default();
        
        // Only load activity data if needed for premium users
        let activity = if profile.is_premium {
            self.activity_data.get(&user)
        } else {
            None
        };
        
        UserSummary {
            username: profile.username,
            reputation: profile.reputation,
            activity_count: activity.map_or(0, |a| a.count),
        }
    }
}
```

### 3. Data Migration Pattern

Upgrade storage schemas while maintaining backward compatibility:

```rust
#[contract]
pub struct UpgradeableContract {
    // Storage version
    version: StorageItem<u32>,
    
    // Old schema (v1)
    old_users: StorageMap<H160, OldUserData>,
    
    // New schema (v2)
    new_users: StorageMap<H160, NewUserData>,
}

#[contractimpl]
impl UpgradeableContract {
    #[method]
    pub fn get_user(&self, address: H160) -> Option<NewUserData> {
        let version = self.version.get();
        
        if version >= 2 {
            // Try to get from new schema
            if let Some(user) = self.new_users.get(&address) {
                return Some(user);
            }
        }
        
        // Fall back to old schema and convert
        if let Some(old_user) = self.old_users.get(&address) {
            return Some(self.convert_user_data(old_user));
        }
        
        None
    }
    
    #[method]
    pub fn migrate_user(&mut self, address: H160) -> bool {
        if let Some(old_user) = self.old_users.get(&address) {
            let new_user = self.convert_user_data(old_user);
            self.new_users.insert(&address, &new_user);
            self.old_users.remove(&address);
            return true;
        }
        
        false
    }
    
    fn convert_user_data(&self, old: OldUserData) -> NewUserData {
        // Convert old format to new format
        NewUserData {
            name: old.name,
            email: old.email,
            created_at: old.joined_date,
            preferences: Default::default(), // New field
        }
    }
}
```

### 4. Pagination Pattern

Handle large collections efficiently:

```rust
#[contract]
pub struct PaginatedRegistry {
    // Total count of items
    item_count: StorageItem<u64>,
    
    // Map from index to item
    items: StorageMap<u64, ItemData>,
}

#[contractimpl]
impl PaginatedRegistry {
    #[method]
    pub fn get_items(&self, start_index: u64, count: u64) -> Vec<ItemData> {
        let mut result = Vec::new();
        let total = self.item_count.get();
        
        // Validate bounds
        let start = start_index.min(total);
        let end = (start + count).min(total);
        
        // Return requested page
        for i in start..end {
            if let Some(item) = self.items.get(&i) {
                result.push(item);
            }
        }
        
        result
    }
    
    #[method]
    pub fn get_total_items(&self) -> u64 {
        self.item_count.get()
    }
}
```

## Storage Optimization

### 1. Minimize Storage Usage

Reduce the amount of data stored on-chain:

```rust
// Inefficient: Storing full data
struct UserProfile {
    address: H160,              // 20 bytes
    name: String,               // Variable length, often large
    email: String,              // Variable length, often large
    profile_image_url: String,  // Very large, often over 100 bytes
    bio: String,                // Variable length, potentially large
}

// More efficient: Store minimal on-chain data
struct OptimizedUserProfile {
    address: H160,              // 20 bytes
    name_hash: H256,            // 32 bytes hash of name
    contact_info_hash: H256,    // 32 bytes hash of contact information
    metadata_url: String,       // URL to off-chain metadata (IPFS, etc.)
}
```

### 2. Delete Unused Storage

Remove data when no longer needed to reclaim space and gas:

```rust
#[method]
pub fn close_account(&mut self, address: H160) {
    // Verify authorization
    assert!(Runtime::check_witness(&address), "Not authorized");
    
    // Remove all user data
    self.balances.remove(&address);
    self.allowances_given.remove(&address);
    self.user_profiles.remove(&address);
    self.user_history.remove(&address);
    
    // Emit account closed event
    Runtime::notify(b"AccountClosed", &[address.into()]);
}
```

### 3. Batch Operations

Group related storage operations to reduce gas costs:

```rust
// Inefficient: Multiple separate operations
fn update_user_data(&mut self, user: H160, name: String, email: String, reputation: u64) {
    self.user_names.insert(&user, &name);
    self.user_emails.insert(&user, &email);
    self.user_reputations.insert(&user, &reputation);
}

// More efficient: Batch related data
fn update_user_data(&mut self, user: H160, name: String, email: String, reputation: u64) {
    let profile = UserProfile {
        name,
        email,
        reputation,
        updated_at: Ledger::current_timestamp(),
    };
    
    self.user_profiles.insert(&user, &profile);
}
```

### 4. Use Appropriate Data Types

Choose the most efficient types for your data:

```rust
// Inefficient: Using string for address
struct Payment {
    recipient_address: String,  // "0x1234567890abcdef1234567890abcdef12345678"
    amount: u64,
}

// More efficient: Using native types
struct Payment {
    recipient_address: H160,    // Native 20-byte address
    amount: u64,
}
```

## Advanced Storage Techniques

### 1. Storage Context

Access the raw storage context for advanced operations:

```rust
use neo_contract::prelude::*;
use neo_contract::storage::Storage;

#[method]
pub fn low_level_storage_operation(&mut self) {
    // Get raw storage context
    let storage_context = Storage::context();
    
    // Read raw value from storage
    let key = b"my_special_key";
    let value = Storage::get(&storage_context, key);
    
    // Write raw value to storage
    let new_value = b"new_value";
    Storage::put(&storage_context, key, new_value);
}
```

### 2. Serialized Data Structures

Store complex data structures efficiently:

```rust
use neo_contract::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct ComplexData {
    values: Vec<u64>,
    mapping: HashMap<String, u64>,
    nested: Option<Box<NestedStruct>>,
}

#[contract]
pub struct SerializationContract {
    // Store serialized complex data
    complex_data: StorageItem<Vec<u8>>,
}

#[contractimpl]
impl SerializationContract {
    #[method]
    pub fn store_complex_data(&mut self, data: ComplexData) {
        // Serialize the complex data to bytes
        let serialized = data.try_to_vec().expect("Serialization failed");
        
        // Store the serialized bytes
        self.complex_data.set(serialized);
    }
    
    #[method]
    pub fn get_complex_data(&self) -> Option<ComplexData> {
        let serialized = self.complex_data.get();
        
        // Deserialize from bytes
        ComplexData::try_from_slice(&serialized).ok()
    }
}
```

### 3. Encrypted Storage

Implement basic encryption for sensitive data:

```rust
#[contract]
pub struct EncryptedStorage {
    // Store encrypted data
    encrypted_data: StorageMap<H160, Vec<u8>>,
    
    // Store encryption keys
    encryption_keys: StorageMap<H160, H256>,
}

#[contractimpl]
impl EncryptedStorage {
    #[method]
    pub fn store_encrypted(&mut self, user: H160, data: String) {
        // Verify user
        assert!(Runtime::check_witness(&user), "Not authorized");
        
        // Generate or retrieve encryption key
        let key = self.get_or_create_key(user);
        
        // Encrypt data (simplified example)
        let encrypted = self.encrypt(data.as_bytes(), key);
        
        // Store encrypted data
        self.encrypted_data.insert(&user, &encrypted);
    }
    
    #[method]
    pub fn get_encrypted(&self, user: H160) -> Option<String> {
        // Verify user
        assert!(Runtime::check_witness(&user), "Not authorized");
        
        // Get encryption key
        let key = self.encryption_keys.get(&user)?;
        
        // Get encrypted data
        let encrypted = self.encrypted_data.get(&user)?;
        
        // Decrypt data (simplified example)
        let decrypted = self.decrypt(&encrypted, key);
        
        // Convert back to string
        String::from_utf8(decrypted).ok()
    }
    
    // Simplified encryption (in practice, use proper cryptographic functions)
    fn encrypt(&self, data: &[u8], key: H256) -> Vec<u8> {
        // Implementation would use proper encryption
        data.to_vec()
    }
    
    // Simplified decryption (in practice, use proper cryptographic functions)
    fn decrypt(&self, data: &[u8], key: H256) -> Vec<u8> {
        // Implementation would use proper decryption
        data.to_vec()
    }
    
    fn get_or_create_key(&mut self, user: H160) -> H256 {
        if let Some(key) = self.encryption_keys.get(&user) {
            return key;
        }
        
        // Generate new key (simplified example)
        let tx_hash = Runtime::current_transaction_hash();
        let key = H256::from(tx_hash);
        
        self.encryption_keys.insert(&user, &key);
        key
    }
}
```

## Testing Storage

### Unit Testing Storage Operations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_counter_storage() {
        // Create contract instance
        let mut contract = MyContract::new(H160::zero());
        
        // Initial value should be zero
        assert_eq!(contract.get_counter(), 0);
        
        // Increment and check new value
        let new_value = contract.increment();
        assert_eq!(new_value, 1);
        assert_eq!(contract.get_counter(), 1);
        
        // Increment again
        contract.increment();
        assert_eq!(contract.get_counter(), 2);
    }
    
    #[test]
    fn test_balance_storage() {
        let mut contract = TokenContract::new();
        let user1 = H160::from([1; 20]);
        let user2 = H160::from([2; 20]);
        
        // Initial balances should be zero
        assert_eq!(contract.get_balance(user1), 0);
        assert_eq!(contract.get_balance(user2), 0);
        
        // Set and verify balance
        contract.set_balance(user1, 100);
        assert_eq!(contract.get_balance(user1), 100);
        
        // Update and verify balance
        contract.set_balance(user1, 150);
        assert_eq!(contract.get_balance(user1), 150);
        
        // Other user's balance should still be zero
        assert_eq!(contract.get_balance(user2), 0);
    }
}
```

### Testing Storage Migrations

```rust
#[test]
fn test_data_migration() {
    let mut contract = UpgradeableContract::new();
    let user = H160::from([1; 20]);
    
    // Setup old data
    let old_user = OldUserData {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        joined_date: 1625097600, // 2021-07-01
    };
    
    contract.old_users.insert(&user, &old_user);
    
    // Migrate user
    assert!(contract.migrate_user(user));
    
    // Verify migration
    let migrated_user = contract.get_user(user).unwrap();
    assert_eq!(migrated_user.name, "Alice");
    assert_eq!(migrated_user.email, "alice@example.com");
    assert_eq!(migrated_user.created_at, 1625097600);
    
    // Old data should be removed
    assert!(contract.old_users.get(&user).is_none());
}
```

## Common Pitfalls

### 1. Not Checking for Existence

Always check if data exists before using it:

```rust
// Unsafe: May panic if record doesn't exist
fn unsafe_get_user(&self, address: H160) -> UserData {
    self.users.get(&address).unwrap() // Will panic if not found
}

// Safe: Handle the case when record doesn't exist
fn safe_get_user(&self, address: H160) -> Option<UserData> {
    self.users.get(&address)
}

// Alternative: Provide default value
fn get_user_or_default(&self, address: H160) -> UserData {
    self.users.get(&address).unwrap_or_default()
}
```

### 2. Forgetting to Update State

Always update storage after modifying values:

```rust
// Incorrect: Modifies local copy but doesn't update storage
fn broken_increment(&mut self) -> u64 {
    let mut counter = self.counter.get();
    counter += 1;
    // Missing storage update!
    counter
}

// Correct: Updates storage after modification
fn correct_increment(&mut self) -> u64 {
    let counter = self.counter.get();
    let new_value = counter + 1;
    self.counter.set(new_value);
    new_value
}
```

### 3. Storage Collisions

Be careful with key structures to avoid collisions:

```rust
// Dangerous: Could have key collisions
struct PoorlyDesignedContract {
    values_by_id: StorageMap<u64, u64>,
    values_by_user: StorageMap<u64, u64>, // Same key type, potential collision
}

// Safer: Use different key types or prefixed keys
struct WellDesignedContract {
    values_by_id: StorageMap<u64, u64>,
    values_by_user: StorageMap<H160, u64>, // Different key type, no collision
}
```

### 4. Wasteful Storage Usage

Don't use storage for temporary values:

```rust
// Wasteful: Using storage for temporary calculation
fn wasteful_calculation(&mut self, a: u64, b: u64) -> u64 {
    // Don't do this!
    self.temp_value_a.set(a);
    self.temp_value_b.set(b);
    
    let result = self.temp_value_a.get() + self.temp_value_b.get();
    
    // Clean up
    self.temp_value_a.remove();
    self.temp_value_b.remove();
    
    result
}

// Efficient: Use local variables for temporary values
fn efficient_calculation(&self, a: u64, b: u64) -> u64 {
    // Simply use function parameters
    a + b
}
```

## Real-World Examples

### NEP-17 Token Contract

A complete storage implementation for a NEP-17 token:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct TokenContract {
    // Token metadata
    name: StorageItem<String>,
    symbol: StorageItem<String>,
    decimals: StorageItem<u8>,
    
    // Token supply
    total_supply: StorageItem<u64>,
    
    // Token balances
    balances: StorageMap<H160, u64>,
    
    // Allowances for spenders
    allowances: StorageMap<(H160, H160), u64>, // (owner, spender) -> amount
    
    // Contract owner
    owner: StorageItem<H160>,
}

#[contractimpl]
impl TokenContract {
    #[constructor]
    pub fn new(owner: H160, name: String, symbol: String, decimals: u8, initial_supply: u64) -> Self {
        let mut contract = Self {
            name: StorageItem::new(name),
            symbol: StorageItem::new(symbol),
            decimals: StorageItem::new(decimals),
            total_supply: StorageItem::new(initial_supply),
            balances: StorageMap::new(),
            allowances: StorageMap::new(),
            owner: StorageItem::new(owner),
        };
        
        // Mint initial supply to owner
        contract.balances.insert(&owner, &initial_supply);
        
        contract
    }
    
    // Standard NEP-17 methods
    
    #[method]
    pub fn name(&self) -> String {
        self.name.get()
    }
    
    #[method]
    pub fn symbol(&self) -> String {
        self.symbol.get()
    }
    
    #[method]
    pub fn decimals(&self) -> u8 {
        self.decimals.get()
    }
    
    #[method]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get()
    }
    
    #[method]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or_default()
    }
    
    #[method]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        assert!(Runtime::check_witness(&from), "No authorization");
        assert!(to != H160::zero(), "Invalid recipient");
        
        let from_balance = self.balances.get(&from).unwrap_or_default();
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update sender balance
        let new_from_balance = from_balance - amount;
        if new_from_balance > 0 {
            self.balances.insert(&from, &new_from_balance);
        } else {
            self.balances.remove(&from);
        }
        
        // Update recipient balance
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(&to, &(to_balance + amount));
        
        // Emit transfer event
        Runtime::notify(b"Transfer", &[from.into(), to.into(), amount.into()]);
        
        true
    }
    
    // Additional methods for allowances
    
    #[method]
    pub fn allowance(&self, owner: H160, spender: H160) -> u64 {
        self.allowances.get(&(owner, spender)).unwrap_or_default()
    }
    
    #[method]
    pub fn approve(&mut self, owner: H160, spender: H160, amount: u64) -> bool {
        assert!(Runtime::check_witness(&owner), "No authorization");
        
        self.allowances.insert(&(owner, spender), &amount);
        
        // Emit approval event
        Runtime::notify(b"Approval", &[owner.into(), spender.into(), amount.into()]);
        
        true
    }
    
    #[method]
    pub fn transfer_from(&mut self, spender: H160, owner: H160, to: H160, amount: u64) -> bool {
        assert!(Runtime::check_witness(&spender), "No authorization");
        
        let allowance = self.allowances.get(&(owner, spender)).unwrap_or_default();
        assert!(allowance >= amount, "Insufficient allowance");
        
        // Update allowance
        let new_allowance = allowance - amount;
        if new_allowance > 0 {
            self.allowances.insert(&(owner, spender), &new_allowance);
        } else {
            self.allowances.remove(&(owner, spender));
        }
        
        // Transfer tokens
        let from_balance = self.balances.get(&owner).unwrap_or_default();
        assert!(from_balance >= amount, "Insufficient balance");
        
        self.balances.insert(&owner, &(from_balance - amount));
        
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(&to, &(to_balance + amount));
        
        // Emit transfer event
        Runtime::notify(b"Transfer", &[owner.into(), to.into(), amount.into()]);
        
        true
    }
}
```

### NFT Contract

Storage implementation for a NEP-11 NFT contract:

```rust
use neo_contract::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
struct TokenMetadata {
    name: String,
    description: String,
    image_url: String,
    properties: Map<String, String>,
}

#[contract]
pub struct NFTContract {
    // Contract metadata
    name: StorageItem<String>,
    symbol: StorageItem<String>,
    
    // Token supply
    total_supply: StorageItem<u64>,
    
    // Token owners
    owners: StorageMap<ByteString, H160>,
    
    // Token metadata
    token_metadata: StorageMap<ByteString, TokenMetadata>,
    
    // Owner token lists (for enumeration)
    tokens_by_owner: StorageMap<H160, Vec<ByteString>>,
    
    // Contract owner
    owner: StorageItem<H160>,
}

#[contractimpl]
impl NFTContract {
    #[constructor]
    pub fn new(owner: H160, name: String, symbol: String) -> Self {
        Self {
            name: StorageItem::new(name),
            symbol: StorageItem::new(symbol),
            total_supply: StorageItem::new(0),
            owners: StorageMap::new(),
            token_metadata: StorageMap::new(),
            tokens_by_owner: StorageMap::new(),
            owner: StorageItem::new(owner),
        }
    }
    
    #[method]
    pub fn mint(&mut self, to: H160, token_id: ByteString, metadata: TokenMetadata) -> bool {
        // Verify mint authorization
        let contract_owner = self.owner.get();
        assert!(Runtime::check_witness(&contract_owner), "Only owner can mint");
        
        // Ensure token doesn't already exist
        assert!(!self.owners.contains_key(&token_id), "Token already exists");
        
        // Store token owner
        self.owners.insert(&token_id, &to);
        
        // Store token metadata
        self.token_metadata.insert(&token_id, &metadata);
        
        // Update owner's token list
        let mut tokens = self.tokens_by_owner.get(&to).unwrap_or_default();
        tokens.push(token_id.clone());
        self.tokens_by_owner.insert(&to, &tokens);
        
        // Update total supply
        let supply = self.total_supply.get();
        self.total_supply.set(supply + 1);
        
        // Emit transfer event (mint: null -> to)
        Runtime::notify(b"Transfer", &[H160::zero().into(), to.into(), token_id.into()]);
        
        true
    }
    
    #[method]
    pub fn transfer(&mut self, from: H160, to: H160, token_id: ByteString) -> bool {
        // Verify ownership and authorization
        assert!(Runtime::check_witness(&from), "No authorization");
        let owner = self.owners.get(&token_id).expect("Token does not exist");
        assert!(owner == from, "Not the token owner");
        
        // Update token owner
        self.owners.insert(&token_id, &to);
        
        // Update from's token list
        let mut from_tokens = self.tokens_by_owner.get(&from).unwrap_or_default();
        from_tokens.retain(|id| id != &token_id);
        
        if from_tokens.is_empty() {
            self.tokens_by_owner.remove(&from);
        } else {
            self.tokens_by_owner.insert(&from, &from_tokens);
        }
        
        // Update to's token list
        let mut to_tokens = self.tokens_by_owner.get(&to).unwrap_or_default();
        to_tokens.push(token_id.clone());
        self.tokens_by_owner.insert(&to, &to_tokens);
        
        // Emit transfer event
        Runtime::notify(b"Transfer", &[from.into(), to.into(), token_id.into()]);
        
        true
    }
    
    #[method]
    pub fn owner_of(&self, token_id: ByteString) -> Option<H160> {
        self.owners.get(&token_id)
    }
    
    #[method]
    pub fn tokens_of(&self, owner: H160) -> Vec<ByteString> {
        self.tokens_by_owner.get(&owner).unwrap_or_default()
    }
    
    #[method]
    pub fn metadata(&self, token_id: ByteString) -> Option<TokenMetadata> {
        self.token_metadata.get(&token_id)
    }
    
    #[method]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get()
    }
}
```

## Conclusion

Effective storage management is crucial for building efficient and reliable Neo N3 smart contracts. The Neo Contract Rust framework provides powerful abstractions that make it easier to work with on-chain storage in a type-safe manner.

By understanding the storage mechanisms and following the patterns and best practices covered in this guide, you can create contracts that efficiently utilize blockchain storage while maintaining good performance and gas economy.

Remember that on-chain storage is a valuable resource, and minimizing storage usage not only reduces costs but also contributes to the overall health of the Neo N3 blockchain ecosystem. 