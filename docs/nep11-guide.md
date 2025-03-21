# NEP-11 Implementation Guide

This guide provides a comprehensive overview of implementing the NEP-11 Non-Fungible Token (NFT) standard using the neo-contract-rs framework.

## Introduction to NEP-11

The NEP-11 standard defines the interface for non-fungible tokens (NFTs) on the Neo blockchain. NEP-11 tokens can be:

1. **Non-divisible NFTs**: Traditional NFTs where each token is a unique, indivisible asset
2. **Divisible NFTs**: Semi-fungible tokens that can be divided into fractions

## NEP-11 Interface Requirements

### Core Methods

Every NEP-11 implementation must support these core methods:

| Method | Parameters | Return Type | Description |
|--------|------------|-------------|-------------|
| `symbol` | | `ByteString` | Returns the token symbol |
| `decimals` | | `u32` | Returns the number of decimals (0 for non-divisible) |
| `totalSupply` | | `Int256` | Returns the total token supply |
| `balanceOf` | `owner: H160` | `Int256` | Returns the token balance of an account |
| `ownerOf` | `tokenId: ByteString` | `H160` | Returns the owner of a specific token |
| `tokens` | | `Iterator<ByteString>` | Returns an iterator of all token IDs |
| `tokensOf` | `owner: H160` | `Iterator<ByteString>` | Returns an iterator of token IDs owned by the account |
| `transfer` | `to: H160, tokenId: ByteString, data: Any` | `bool` | Transfers a token to another account |

### Optional Methods

These methods are optional but recommended:

| Method | Parameters | Return Type | Description |
|--------|------------|-------------|-------------|
| `properties` | `tokenId: ByteString` | `Map<ByteString, ByteString>` | Returns token metadata |
| `tokenURI` | `tokenId: ByteString` | `ByteString` | Returns a URI pointing to token metadata |
| `mint` | `owner: H160, tokenId: ByteString, properties: Any` | `bool` | Creates a new token |
| `burn` | `tokenId: ByteString` | `bool` | Destroys a token |

## Implementing NEP-11 in Rust

Below is a step-by-step guide to implementing NEP-11 tokens using neo-contract-rs.

### 1. Basic Structure for Non-Divisible NFTs

First, let's define the basic structure for a non-divisible NFT contract:

```rust
// src/lib.rs
#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, storage::*, types::*};

pub struct NonDivisibleNFT;

#[neo::contract]
impl Nep11Token for NonDivisibleNFT {
    fn symbol() -> ByteString {
        ByteString::from("NDNFT")
    }
    
    fn decimals() -> u32 {
        0  // Non-divisible NFTs have 0 decimals
    }
    
    fn total_supply() -> Int256 {
        // Implementation to retrieve total supply from storage
    }
    
    fn balance_of(owner: H160) -> Int256 {
        // Implementation to count tokens owned by an account
    }
    
    fn owner_of(token_id: ByteString) -> H160 {
        // Implementation to get token owner
    }
    
    fn tokens() -> Array<ByteString> {
        // Implementation to list all tokens
    }
    
    fn tokens_of(owner: H160) -> Array<ByteString> {
        // Implementation to list tokens of owner
    }
    
    fn transfer(to: H160, token_id: ByteString, data: Any) -> bool {
        // Implementation to transfer token ownership
    }
    
    fn properties(token_id: ByteString) -> Map<ByteString, ByteString> {
        // Implementation to retrieve token properties
    }
    
    // Additional methods...
}
```

### 2. Storage Structure

NFT contracts require several storage structures:

```rust
// Storage prefixes for different data types
const PREFIX_OWNER: u8 = 0x01;      // Maps token ID -> owner
const PREFIX_TOKEN: u8 = 0x02;       // Maps owner -> set of token IDs
const PREFIX_PROPERTIES: u8 = 0x03;  // Maps token ID -> properties
const TOTAL_SUPPLY_KEY: u8 = 0x00;   // Key for total supply
```

### 3. Implementation Details

#### Total Supply

```rust
fn total_supply() -> Int256 {
    let storage = StorageMap::new();
    let total_supply_key = ByteString::from_bytes(&[TOTAL_SUPPLY_KEY]);
    
    let value = storage.get(total_supply_key);
    if value.is_null() {
        return Int256::zero();
    }
    
    Int256::from_byte_string(value.unwrap())
}
```

#### Balance Of

```rust
fn balance_of(owner: H160) -> Int256 {
    let storage = StorageMap::new();
    let prefix_key = ByteString::from_bytes(&[PREFIX_TOKEN]);
    let owner_key = prefix_key.concat(&ByteString::from_bytes(&owner.to_bytes()));
    
    let value = storage.get(owner_key);
    if value.is_null() {
        return Int256::zero();
    }
    
    // Deserialize the set of token IDs and count them
    let tokens_array = deserialize_tokens(value.unwrap());
    Int256::from_i32(tokens_array.len() as i32)
}
```

#### Owner Of

```rust
fn owner_of(token_id: ByteString) -> H160 {
    let storage = StorageMap::new();
    let prefix_key = ByteString::from_bytes(&[PREFIX_OWNER]);
    let token_key = prefix_key.concat(&token_id);
    
    let value = storage.get(token_key);
    if value.is_null() {
        // Token does not exist
        return H160::zero();
    }
    
    // Deserialize owner address
    let owner_bytes = value.unwrap().to_bytes();
    H160::from_bytes(&owner_bytes)
}
```

#### Tokens

```rust
fn tokens() -> Array<ByteString> {
    // This is a simplified implementation
    // In a real contract, you would need pagination
    let storage = StorageMap::new();
    let mut result = Array::<ByteString>::new();
    
    // Iterate through all tokens (pseudocode, as direct iteration is not available)
    // In practice, you would maintain a separate list of all token IDs
    
    // Return the list of tokens
    result
}
```

#### Transfer

```rust
fn transfer(to: H160, token_id: ByteString, data: Any) -> bool {
    // Check if token exists
    let owner = NonDivisibleNFT::owner_of(token_id.clone());
    if owner == H160::zero() {
        return false; // Token doesn't exist
    }
    
    // Check authorization
    if !runtime::check_witness(owner) {
        return false; // Not authorized
    }
    
    // Update token ownership
    let mut storage = StorageMap::new();
    
    // 1. Update owner mapping
    let prefix_owner = ByteString::from_bytes(&[PREFIX_OWNER]);
    let token_key = prefix_owner.concat(&token_id);
    storage.put(token_key, ByteString::from_bytes(&to.to_bytes()));
    
    // 2. Remove from previous owner's tokens
    let prefix_token = ByteString::from_bytes(&[PREFIX_TOKEN]);
    let prev_owner_key = prefix_token.concat(&ByteString::from_bytes(&owner.to_bytes()));
    let prev_owner_tokens = storage.get(prev_owner_key.clone());
    
    if !prev_owner_tokens.is_null() {
        let mut tokens_array = deserialize_tokens(prev_owner_tokens.unwrap());
        // Remove token from array
        // ...
        storage.put(prev_owner_key, serialize_tokens(tokens_array));
    }
    
    // 3. Add to new owner's tokens
    let new_owner_key = prefix_token.concat(&ByteString::from_bytes(&to.to_bytes()));
    let new_owner_tokens = storage.get(new_owner_key.clone());
    
    let mut tokens_array = if new_owner_tokens.is_null() {
        Array::<ByteString>::new()
    } else {
        deserialize_tokens(new_owner_tokens.unwrap())
    };
    
    tokens_array.push(token_id.clone());
    storage.put(new_owner_key, serialize_tokens(tokens_array));
    
    // 4. Emit transfer event
    emit_transfer_event(owner, to, token_id, data);
    
    true
}
```

#### Properties

```rust
fn properties(token_id: ByteString) -> Map<ByteString, ByteString> {
    let storage = StorageMap::new();
    let prefix_key = ByteString::from_bytes(&[PREFIX_PROPERTIES]);
    let token_key = prefix_key.concat(&token_id);
    
    let value = storage.get(token_key);
    if value.is_null() {
        return Map::<ByteString, ByteString>::new();
    }
    
    // Deserialize properties
    deserialize_properties(value.unwrap())
}
```

### 4. Helper Functions

```rust
// Helper to emit transfer events
fn emit_transfer_event(from: H160, to: H160, token_id: ByteString, data: Any) {
    let event_name = ByteString::from("Transfer");
    
    // Create event data
    let mut event_data = Array::<Any>::new();
    event_data.push(ByteString::from_bytes(&from.to_bytes()));
    event_data.push(ByteString::from_bytes(&to.to_bytes()));
    event_data.push(token_id);
    event_data.push(data);
    
    runtime::notify(event_name, event_data);
}

// Helper to serialize token arrays
fn serialize_tokens(tokens: Array<ByteString>) -> ByteString {
    // Implementation depends on serialization approach
}

// Helper to deserialize token arrays
fn deserialize_tokens(data: ByteString) -> Array<ByteString> {
    // Implementation depends on serialization approach
}

// Helper to serialize properties
fn serialize_properties(properties: Map<ByteString, ByteString>) -> ByteString {
    // Implementation depends on serialization approach
}

// Helper to deserialize properties
fn deserialize_properties(data: ByteString) -> Map<ByteString, ByteString> {
    // Implementation depends on serialization approach
}
```

### 5. Minting and Burning

```rust
// Mint a new token
fn mint(owner: H160, token_id: ByteString, properties: Map<ByteString, ByteString>) -> bool {
    // Check authorization
    if !runtime::check_witness(runtime::executing_script_hash()) {
        return false; // Only contract owner can mint
    }
    
    let mut storage = StorageMap::new();
    
    // Check if token already exists
    let prefix_owner = ByteString::from_bytes(&[PREFIX_OWNER]);
    let token_key = prefix_owner.concat(&token_id);
    if !storage.get(token_key.clone()).is_null() {
        return false; // Token already exists
    }
    
    // 1. Set token owner
    storage.put(token_key, ByteString::from_bytes(&owner.to_bytes()));
    
    // 2. Add to owner's tokens
    let prefix_token = ByteString::from_bytes(&[PREFIX_TOKEN]);
    let owner_key = prefix_token.concat(&ByteString::from_bytes(&owner.to_bytes()));
    let owner_tokens = storage.get(owner_key.clone());
    
    let mut tokens_array = if owner_tokens.is_null() {
        Array::<ByteString>::new()
    } else {
        deserialize_tokens(owner_tokens.unwrap())
    };
    
    tokens_array.push(token_id.clone());
    storage.put(owner_key, serialize_tokens(tokens_array));
    
    // 3. Store token properties
    if !properties.is_empty() {
        let prefix_properties = ByteString::from_bytes(&[PREFIX_PROPERTIES]);
        let properties_key = prefix_properties.concat(&token_id);
        storage.put(properties_key, serialize_properties(properties));
    }
    
    // 4. Update total supply
    let total_supply_key = ByteString::from_bytes(&[TOTAL_SUPPLY_KEY]);
    let current_supply = NonDivisibleNFT::total_supply();
    let new_supply = current_supply.checked_add(&Int256::from_i32(1));
    storage.put(total_supply_key, new_supply.into_byte_string());
    
    // 5. Emit transfer event (from zero address for minting)
    emit_transfer_event(H160::zero(), owner, token_id, ByteString::empty());
    
    true
}

// Burn a token
fn burn(token_id: ByteString) -> bool {
    // Get token owner
    let owner = NonDivisibleNFT::owner_of(token_id.clone());
    if owner == H160::zero() {
        return false; // Token doesn't exist
    }
    
    // Check authorization
    if !runtime::check_witness(owner) {
        return false; // Not authorized
    }
    
    let mut storage = StorageMap::new();
    
    // 1. Remove token ownership
    let prefix_owner = ByteString::from_bytes(&[PREFIX_OWNER]);
    let token_key = prefix_owner.concat(&token_id);
    storage.delete(token_key);
    
    // 2. Remove from owner's tokens
    let prefix_token = ByteString::from_bytes(&[PREFIX_TOKEN]);
    let owner_key = prefix_token.concat(&ByteString::from_bytes(&owner.to_bytes()));
    let owner_tokens = storage.get(owner_key.clone());
    
    if !owner_tokens.is_null() {
        let mut tokens_array = deserialize_tokens(owner_tokens.unwrap());
        // Remove token from array
        // ...
        storage.put(owner_key, serialize_tokens(tokens_array));
    }
    
    // 3. Remove token properties
    let prefix_properties = ByteString::from_bytes(&[PREFIX_PROPERTIES]);
    let properties_key = prefix_properties.concat(&token_id);
    storage.delete(properties_key);
    
    // 4. Update total supply
    let total_supply_key = ByteString::from_bytes(&[TOTAL_SUPPLY_KEY]);
    let current_supply = NonDivisibleNFT::total_supply();
    let new_supply = current_supply.checked_sub(&Int256::from_i32(1));
    storage.put(total_supply_key, new_supply.into_byte_string());
    
    // 5. Emit transfer event (to zero address for burning)
    emit_transfer_event(owner, H160::zero(), token_id, ByteString::empty());
    
    true
}
```

## Differences for Divisible NFTs

For divisible NFTs, a few key changes are needed:

1. **Decimals**: Return a non-zero value for decimals
2. **Balance Tracking**: Track fractional ownership
3. **Transfer Logic**: Allow partial transfers

## Testing Your NEP-11 Implementation

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol() {
        assert_eq!(NonDivisibleNFT::symbol().to_bytes(), b"NDNFT");
    }
    
    #[test]
    fn test_decimals() {
        assert_eq!(NonDivisibleNFT::decimals(), 0);
    }
    
    // More tests for other functions...
}
```

### Integration Tests

For integration testing, deploy your contract to a Neo test network and interact with it using:

```
neo-express deploy token.nef
neo-express invoke <contract-hash> mint [<params>]
neo-express invoke <contract-hash> transfer [<params>]
```

## Best Practices

1. **Implement the complete interface**: Even optional methods are important for ecosystem compatibility
2. **Use proper error handling**: Never panic in production code
3. **Emit events for all state changes**: This helps external systems track token movements
4. **Validate all inputs**: Never trust input parameters
5. **Implement authorization checks**: Only allow authorized users to perform sensitive operations
6. **Minimize storage operations**: Storage operations consume gas
7. **Consider gas costs**: Optimize your code for gas efficiency

## Advanced Features

### Metadata Standards

Consider implementing standardized metadata with properties like:

- `name`: Token name
- `description`: Token description
- `image`: URL to token image
- `attributes`: Array of trait objects

### Royalties

Implement royalty payments for creators:

```rust
fn royalty_info(token_id: ByteString) -> Map<H160, Int256> {
    // Return map of beneficiary addresses to percentage (basis points)
}
```

### Token URI

Implement token URI for off-chain metadata:

```rust
fn token_uri(token_id: ByteString) -> ByteString {
    // Return URI pointing to off-chain metadata
}
```

## Conclusion

Implementing the NEP-11 standard in Rust with neo-contract-rs provides a powerful way to create NFTs on the Neo blockchain. This guide covered the basic implementation details, but you can extend it with additional features to suit your specific use case. 