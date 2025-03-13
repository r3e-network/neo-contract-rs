# Neo N3 NEP-11 Non-Fungible Token Standard Implementation Guide

This guide provides a comprehensive overview of implementing the NEP-11 non-fungible token standard for Neo N3 using the neo-contract-rs framework.

## Overview

NEP-11 is the non-fungible token (NFT) standard for Neo N3. NFTs represent unique digital assets on the blockchain, with each token having distinct properties and identifiers. Unlike fungible tokens (NEP-17), NFTs are not interchangeable with each other.

## Required Methods

A NEP-11 compliant token must implement the following methods:

### 1. Symbol

Returns the token's symbol.

```rust
#[safe]
fn symbol(&self) -> ByteString {
    self.symbol.get().unwrap()
}
```

### 2. Decimals

For NEP-11 tokens, this should always return 0 since NFTs are indivisible.

```rust
#[safe]
fn decimals(&self) -> u8 {
    0
}
```

### 3. TotalSupply

Returns the total count of NFTs in the contract.

```rust
#[safe]
fn total_supply(&self) -> u64 {
    self.token_count.get().unwrap()
}
```

### 4. BalanceOf

Returns the number of NFTs owned by the specified address.

```rust
#[safe]
fn balance_of(&self, owner: H160) -> u64 {
    self.balances.get(&owner).unwrap_or_default()
}
```

### 5. OwnerOf

Returns the owner of the specified token ID.

```rust
#[safe]
fn owner_of(&self, token_id: ByteArray) -> H160 {
    let owner = self.owners.get(&token_id).unwrap();
    assert!(!owner.is_zero(), "Token does not exist");
    owner
}
```

### 6. Properties

Returns the properties of the specified token.

```rust
#[safe]
fn properties(&self, token_id: ByteArray) -> Map<ByteString, ByteString> {
    self.token_properties.get(&token_id).unwrap_or_default()
}
```

### 7. Tokens

Returns all token IDs owned by the specified address.

```rust
#[safe]
fn tokens(&self, owner: H160) -> Array<ByteArray> {
    self.owned_tokens.get(&owner).unwrap_or_default()
}
```

### 8. Transfer

Transfers a specific NFT from one address to another.

```rust
#[method]
fn transfer(&mut self, to: H160, token_id: ByteArray, data: Option<Any>) -> bool {
    let token_owner = self.owners.get(&token_id).unwrap_or_default();
    assert!(!token_owner.is_zero(), "Token does not exist");
    
    // Verify the transaction sender is the token owner
    assert!(Runtime::check_witness(&token_owner), "No authorization");
    
    // Check for zero address
    assert!(!to.is_zero(), "Cannot transfer to zero address");
    
    // Update token ownership
    self.owners.insert(token_id.clone(), to.clone());
    
    // Update balances
    let from_balance = self.balances.get(&token_owner).unwrap_or_default();
    if from_balance == 1 {
        self.balances.remove(&token_owner);
    } else {
        self.balances.insert(token_owner, from_balance - 1);
    }
    
    let to_balance = self.balances.get(&to).unwrap_or_default();
    self.balances.insert(to, to_balance + 1);
    
    // Update owned tokens lists
    self.remove_token_from_owner(token_owner, token_id.clone());
    self.add_token_to_owner(to, token_id.clone());
    
    // Emit the transfer event
    Transfer::emit(Some(token_owner), Some(to), 1, Some(token_id));
    
    true
}
```

## Required Events

### Transfer Event

A compliant NEP-11 token must emit a `Transfer` event when NFTs are transferred, including when NFTs are created (from is null) or destroyed (to is null).

```rust
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64, token_id: Option<ByteArray>) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters, handling None values correctly
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        event_data.push(Any::from(amount));
        
        // Add token_id for NEP-11 (not required in NEP-17)
        match token_id {
            Some(id) => event_data.push(Any::from(id)),
            None => event_data.push(Any::new()),
        }
        
        // Emit event using Neo N3 Runtime::notify
        Runtime::notify(&event_name, &event_data);
    }
}
```

## Complete Implementation

Here's a complete implementation of a NEP-11 token contract with all required methods and events:

```rust
use neo_contract::prelude::*;

#[storage]
pub struct NFTContract {
    name: StorageItem<ByteString>,
    symbol: StorageItem<ByteString>,
    token_count: StorageItem<u64>,
    
    // Token ownership
    owners: StorageMap<ByteArray, H160>,
    
    // Token balances (number of NFTs owned by each address)
    balances: StorageMap<H160, u64>,
    
    // Token properties
    token_properties: StorageMap<ByteArray, Map<ByteString, ByteString>>,
    
    // Tokens owned by each address
    owned_tokens: StorageMap<H160, Array<ByteArray>>,
    
    // Contract owner
    owner: StorageItem<H160>,
}

// Transfer event structure
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64, token_id: Option<ByteArray>) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        event_data.push(Any::from(amount));
        
        match token_id {
            Some(id) => event_data.push(Any::from(id)),
            None => event_data.push(Any::new()),
        }
        
        Runtime::notify(&event_name, &event_data);
    }
}

impl NFTContract {
    #[constructor]
    pub fn new(owner: H160) -> Self {
        Self {
            name: StorageItem::new(ByteString::from("Example NFT")),
            symbol: StorageItem::new(ByteString::from("ENFT")),
            token_count: StorageItem::new(0),
            owners: StorageMap::new(),
            balances: StorageMap::new(),
            token_properties: StorageMap::new(),
            owned_tokens: StorageMap::new(),
            owner: StorageItem::new(owner),
        }
    }
    
    // Helper methods
    
    fn add_token_to_owner(&mut self, owner: H160, token_id: ByteArray) {
        let mut tokens = self.owned_tokens.get(&owner).unwrap_or_else(|| Array::new());
        tokens.push(token_id);
        self.owned_tokens.insert(owner, tokens);
    }
    
    fn remove_token_from_owner(&mut self, owner: H160, token_id: ByteArray) {
        let mut tokens = self.owned_tokens.get(&owner).unwrap_or_else(|| Array::new());
        let mut index_to_remove = None;
        
        for (i, token) in tokens.iter().enumerate() {
            if token == &token_id {
                index_to_remove = Some(i);
                break;
            }
        }
        
        if let Some(index) = index_to_remove {
            // Remove token at the specified index
            let length = tokens.len();
            if length > 1 && index < length - 1 {
                // Swap with last element then pop
                tokens.swap(index, length - 1);
            }
            tokens.pop();
            
            if tokens.is_empty() {
                self.owned_tokens.remove(&owner);
            } else {
                self.owned_tokens.insert(owner, tokens);
            }
        }
    }
    
    // NEP-11 Methods
    
    #[safe]
    pub fn symbol(&self) -> ByteString {
        self.symbol.get().unwrap()
    }
    
    #[safe]
    pub fn name(&self) -> ByteString {
        self.name.get().unwrap()
    }
    
    #[safe]
    pub fn decimals(&self) -> u8 {
        0 // NFTs are indivisible
    }
    
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.token_count.get().unwrap()
    }
    
    #[safe]
    pub fn balance_of(&self, owner: H160) -> u64 {
        self.balances.get(&owner).unwrap_or_default()
    }
    
    #[safe]
    pub fn owner_of(&self, token_id: ByteArray) -> H160 {
        let owner = self.owners.get(&token_id).unwrap_or_default();
        assert!(!owner.is_zero(), "Token does not exist");
        owner
    }
    
    #[safe]
    pub fn tokens(&self, owner: H160) -> Array<ByteArray> {
        self.owned_tokens.get(&owner).unwrap_or_else(|| Array::new())
    }
    
    #[safe]
    pub fn properties(&self, token_id: ByteArray) -> Map<ByteString, ByteString> {
        self.token_properties.get(&token_id).unwrap_or_else(|| Map::new())
    }
    
    #[method]
    pub fn transfer(&mut self, to: H160, token_id: ByteArray, data: Option<Any>) -> bool {
        let token_owner = self.owners.get(&token_id).unwrap_or_default();
        assert!(!token_owner.is_zero(), "Token does not exist");
        
        // Verify the transaction sender is the token owner
        assert!(Runtime::check_witness(&token_owner), "No authorization");
        
        // Check for zero address
        assert!(!to.is_zero(), "Cannot transfer to zero address");
        
        // Update token ownership
        self.owners.insert(token_id.clone(), to.clone());
        
        // Update balances
        let from_balance = self.balances.get(&token_owner).unwrap_or_default();
        if from_balance == 1 {
            self.balances.remove(&token_owner);
        } else {
            self.balances.insert(token_owner, from_balance - 1);
        }
        
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(to, to_balance + 1);
        
        // Update owned tokens lists
        self.remove_token_from_owner(token_owner, token_id.clone());
        self.add_token_to_owner(to, token_id.clone());
        
        // Emit the transfer event
        Transfer::emit(Some(token_owner), Some(to), 1, Some(token_id));
        
        // Call onNEP11Payment if the recipient is a contract
        if let Some(data_value) = data {
            if Runtime::calling_script_hash() != to {
                let args = vec![
                    Any::from(token_owner),
                    Any::from(1),
                    Any::from(token_id),
                    data_value,
                ];
                
                // Try to call onNEP11Payment, ignore failure
                let _ = contract::call(
                    &to,
                    "onNEP11Payment",
                    &args,
                    CallFlags::All
                );
            }
        }
        
        true
    }
    
    // Additional Methods
    
    #[method]
    pub fn mint(&mut self, to: H160, token_id: ByteArray, properties: Map<ByteString, ByteString>) -> bool {
        // Only contract owner can mint
        let owner = self.owner.get().unwrap();
        assert!(Runtime::check_witness(&owner), "Only owner can mint");
        
        // Check for zero address
        assert!(!to.is_zero(), "Cannot mint to zero address");
        
        // Check if token already exists
        assert!(self.owners.get(&token_id).unwrap_or_default().is_zero(), "Token already exists");
        
        // Update state
        self.owners.insert(token_id.clone(), to.clone());
        self.token_properties.insert(token_id.clone(), properties);
        
        // Update token count
        let current_count = self.token_count.get().unwrap();
        self.token_count.set(current_count + 1);
        
        // Update owner's balance
        let balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(to, balance + 1);
        
        // Add token to owner's list
        self.add_token_to_owner(to, token_id.clone());
        
        // Emit transfer event (from = null indicates token was minted)
        Transfer::emit(None, Some(to), 1, Some(token_id));
        
        true
    }
    
    #[method]
    pub fn burn(&mut self, token_id: ByteArray) -> bool {
        let token_owner = self.owners.get(&token_id).unwrap_or_default();
        assert!(!token_owner.is_zero(), "Token does not exist");
        
        // Verify the transaction sender is the token owner
        assert!(Runtime::check_witness(&token_owner), "No authorization");
        
        // Update state
        self.owners.remove(&token_id);
        self.token_properties.remove(&token_id);
        
        // Update token count
        let current_count = self.token_count.get().unwrap();
        self.token_count.set(current_count - 1);
        
        // Update owner's balance
        let balance = self.balances.get(&token_owner).unwrap_or_default();
        if balance == 1 {
            self.balances.remove(&token_owner);
        } else {
            self.balances.insert(token_owner, balance - 1);
        }
        
        // Remove token from owner's list
        self.remove_token_from_owner(token_owner, token_id.clone());
        
        // Emit transfer event (to = null indicates token was burned)
        Transfer::emit(Some(token_owner), None, 1, Some(token_id));
        
        true
    }
    
    #[method]
    #[safe]
    pub fn tokens_of(&self, owner: H160) -> Array<Map<ByteString, ByteString>> {
        let token_ids = self.owned_tokens.get(&owner).unwrap_or_else(|| Array::new());
        let mut result = Array::<Map<ByteString, ByteString>>::new();
        
        for token_id in token_ids.iter() {
            if let Some(props) = self.token_properties.get(token_id) {
                result.push(props);
            }
        }
        
        result
    }
}
```

## Best Practices for NEP-11 Implementation

### 1. Token IDs

Use a consistent and unique system for token IDs. Options include:

```rust
// Sequential numeric IDs
let token_id = ByteArray::from(next_id.to_string());

// UUIDs or hash-based IDs
let token_id = Runtime::get_random() + ByteArray::from(timestamp.to_string());

// Content-based IDs (hash of content)
let token_id = hash::sha256(content);
```

### 2. Metadata Standards

Store token metadata consistently:

```rust
let mut properties = Map::<ByteString, ByteString>::new();
properties.insert(ByteString::from("name"), ByteString::from("Artwork #1"));
properties.insert(ByteString::from("description"), ByteString::from("Digital artwork"));
properties.insert(ByteString::from("image"), ByteString::from("ipfs://QmXYZ..."));
properties.insert(ByteString::from("artist"), ByteString::from("Artist Name"));
properties.insert(ByteString::from("created_at"), ByteString::from(timestamp.to_string()));
```

### 3. Efficient Token Ownership Tracking

For contracts with many tokens, efficient owner tracking is essential:

```rust
// For O(1) lookup of token ownership
owners: StorageMap<ByteArray, H160>,

// For O(1) lookup of token balance
balances: StorageMap<H160, u64>,

// For reasonably efficient token enumeration
owned_tokens: StorageMap<H160, Array<ByteArray>>,
```

### 4. Batch Operations

For better gas efficiency when minting or transferring multiple NFTs:

```rust
#[method]
pub fn batch_mint(&mut self, to: H160, properties_list: Array<Map<ByteString, ByteString>>) -> bool {
    // Only contract owner can mint
    let owner = self.owner.get().unwrap();
    assert!(Runtime::check_witness(&owner), "Only owner can mint");
    
    // Check for zero address
    assert!(!to.is_zero(), "Cannot mint to zero address");
    
    // Get current token count for ID generation
    let mut current_count = self.token_count.get().unwrap();
    
    // Update owner's balance once
    let balance = self.balances.get(&to).unwrap_or_default();
    let new_balance = balance + properties_list.len() as u64;
    self.balances.insert(to, new_balance);
    
    // Create tokens
    for properties in properties_list.iter() {
        // Generate token ID
        let token_id = ByteArray::from(current_count.to_string());
        current_count += 1;
        
        // Store token data
        self.owners.insert(token_id.clone(), to);
        self.token_properties.insert(token_id.clone(), properties.clone());
        
        // Add token to owner's list
        self.add_token_to_owner(to, token_id.clone());
        
        // Emit transfer event
        Transfer::emit(None, Some(to), 1, Some(token_id));
    }
    
    // Update total token count
    self.token_count.set(current_count);
    
    true
}
```

### 5. Supporting Marketplaces

Implement approval mechanisms for marketplace integration:

```rust
#[storage]
pub struct NFTContract {
    // Existing fields...
    
    // Token approvals (address approved to transfer a specific token)
    token_approvals: StorageMap<ByteArray, H160>,
    
    // Operator approvals (address approved to manage all tokens of owner)
    operator_approvals: StorageMap<(H160, H160), bool>,
}

// Additional methods
impl NFTContract {
    #[method]
    pub fn approve(&mut self, approved: H160, token_id: ByteArray) -> bool {
        let owner = self.owners.get(&token_id).unwrap_or_default();
        assert!(!owner.is_zero(), "Token does not exist");
        
        // Verify authorization
        assert!(Runtime::check_witness(&owner), "No authorization");
        
        // Set approval
        self.token_approvals.insert(token_id, approved);
        
        // Emit approval event
        Approval::emit(owner, approved, token_id);
        
        true
    }
    
    #[method]
    pub fn set_approval_for_all(&mut self, operator: H160, approved: bool) -> bool {
        let owner = Runtime::calling_script_hash();
        
        // Verify authorization
        assert!(Runtime::check_witness(&owner), "No authorization");
        
        // Set operator approval
        self.operator_approvals.insert((owner, operator), approved);
        
        // Emit event
        ApprovalForAll::emit(owner, operator, approved);
        
        true
    }
    
    #[method]
    #[safe]
    pub fn get_approved(&self, token_id: ByteArray) -> H160 {
        self.token_approvals.get(&token_id).unwrap_or_default()
    }
    
    #[method]
    #[safe]
    pub fn is_approved_for_all(&self, owner: H160, operator: H160) -> bool {
        self.operator_approvals.get(&(owner, operator)).unwrap_or_default()
    }
}
```

## Supporting NFT Collections

For collections with traits and attributes:

```rust
#[method]
#[safe]
pub fn get_token_trait(&self, token_id: ByteArray, trait_name: ByteString) -> ByteString {
    let properties = self.token_properties.get(&token_id).unwrap_or_default();
    properties.get(&trait_name).unwrap_or_default()
}

#[method]
#[safe]
pub fn tokens_with_trait(&self, trait_name: ByteString, trait_value: ByteString) -> Array<ByteArray> {
    let mut result = Array::<ByteArray>::new();
    let total = self.token_count.get().unwrap();
    
    // Simple linear search (can be optimized with additional indices)
    for i in 0..total {
        let token_id = ByteArray::from(i.to_string());
        if let Some(properties) = self.token_properties.get(&token_id) {
            if let Some(value) = properties.get(&trait_name) {
                if value == &trait_value {
                    result.push(token_id);
                }
            }
        }
    }
    
    result
}
```

## Integration with Neo N3 Ecosystem

### Contract Interaction

Interacting with other Neo N3 contracts:

```rust
#[method]
pub fn list_on_marketplace(&mut self, marketplace: H160, token_id: ByteArray, price: u64) -> bool {
    let owner = self.owners.get(&token_id).unwrap_or_default();
    
    // Verify authorization
    assert!(Runtime::check_witness(&owner), "No authorization");
    
    // Approve marketplace to transfer the token
    self.approve(marketplace, token_id.clone());
    
    // Notify marketplace
    let args = vec![
        Any::from(owner),
        Any::from(token_id),
        Any::from(price),
    ];
    
    contract::call(
        &marketplace,
        "list_item",
        &args,
        CallFlags::All
    );
    
    true
}
```

### NFT Fractional Ownership

Combining NEP-11 and NEP-17 for fractional ownership:

```rust
#[method]
pub fn fractionalize(&mut self, token_id: ByteArray, fractions: u64) -> H160 {
    let owner = self.owners.get(&token_id).unwrap_or_default();
    
    // Verify authorization
    assert!(Runtime::check_witness(&owner), "No authorization");
    
    // Create a new NEP-17 token contract for this NFT
    // (simplified - in reality this would deploy a new contract)
    let fraction_token = create_fraction_token(token_id.clone(), fractions);
    
    // Transfer NFT to the new contract
    self.transfer(fraction_token, token_id, None);
    
    // Mint fractions to the original owner
    fraction_token
}
```

## Conclusion

This guide provides a comprehensive overview of implementing NEP-11 tokens for Neo N3 using the neo-contract-rs framework. By following these standards and best practices, you'll ensure your NFT contract is compatible with wallets, marketplaces, and other smart contracts in the Neo N3 ecosystem.

For more information, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Runtime Guide](./neo_n3_runtime_guide.md)
- [Neo N3 Storage Guide](./neo_n3_storage_guide.md)
- [Neo N3 Security Guide](./neo_n3_security_guide.md)
- [Neo N3 NEP-17 Guide](./neo_n3_nep17_guide.md)
