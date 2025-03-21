// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, runtime, storage::*, types::*};

/// Basic Non-Divisible NFT implementation following the NEP-11 standard
pub struct BasicNFT;

// Storage prefixes for different data types
const PREFIX_OWNER: u8 = 0x01;      // Maps token ID -> owner
const PREFIX_TOKEN: u8 = 0x02;       // Maps owner -> set of token IDs
const PREFIX_PROPERTIES: u8 = 0x03;  // Maps token ID -> properties
const PREFIX_TOKEN_LIST: u8 = 0x04;  // List of all tokens
const TOTAL_SUPPLY_KEY: u8 = 0x00;   // Key for total supply

/// Define the NEP-11 Token trait implementation
#[neo::contract]
impl BasicNFT {
    /// Returns the token symbol
    pub fn symbol() -> ByteString {
        ByteString::from("BNFT")
    }
    
    /// Returns the number of decimals (0 for non-divisible)
    pub fn decimals() -> u32 {
        0  // Non-divisible NFTs have 0 decimals
    }
    
    /// Returns the total token supply
    pub fn total_supply() -> Int256 {
        let storage = StorageMap::new();
        let total_supply_key = ByteString::from_bytes(&[TOTAL_SUPPLY_KEY]);
        
        let value = storage.get(total_supply_key);
        if value.is_null() {
            return Int256::zero();
        }
        
        Int256::from_byte_string(value.unwrap())
    }
    
    /// Returns the token balance of an account
    pub fn balance_of(owner: H160) -> Int256 {
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
    
    /// Returns the owner of a specific token
    pub fn owner_of(token_id: ByteString) -> H160 {
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
    
    /// Returns an array of all token IDs
    pub fn tokens() -> Array<ByteString> {
        let storage = StorageMap::new();
        let tokens_key = ByteString::from_bytes(&[PREFIX_TOKEN_LIST]);
        
        let value = storage.get(tokens_key);
        if value.is_null() {
            return Array::<ByteString>::new();
        }
        
        deserialize_tokens(value.unwrap())
    }
    
    /// Returns an array of token IDs owned by the account
    pub fn tokens_of(owner: H160) -> Array<ByteString> {
        let storage = StorageMap::new();
        let prefix_key = ByteString::from_bytes(&[PREFIX_TOKEN]);
        let owner_key = prefix_key.concat(&ByteString::from_bytes(&owner.to_bytes()));
        
        let value = storage.get(owner_key);
        if value.is_null() {
            return Array::<ByteString>::new();
        }
        
        deserialize_tokens(value.unwrap())
    }
    
    /// Transfers a token to another account
    pub fn transfer(to: H160, token_id: ByteString, data: Any) -> bool {
        // Check if token exists
        let owner = BasicNFT::owner_of(token_id.clone());
        if owner == H160::zero() {
            return false; // Token doesn't exist
        }
        
        // Check authorization
        if !runtime::check_witness(owner.clone()) {
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
            remove_token_from_array(&mut tokens_array, &token_id);
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
    
    /// Returns token metadata
    pub fn properties(token_id: ByteString) -> Map<ByteString, ByteString> {
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
    
    /// Creates a new token
    pub fn mint(owner: H160, token_id: ByteString, props: Array<ByteString>) -> bool {
        // Check authorization (contract owner only)
        let contract_hash = runtime::executing_script_hash();
        if !runtime::check_witness(contract_hash) {
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
        if props.len() > 0 && props.len() % 2 == 0 {
            let mut properties = Map::<ByteString, ByteString>::new();
            for i in 0..props.len() / 2 {
                let key = props.get(i * 2);
                let value = props.get(i * 2 + 1);
                properties.insert(key, value);
            }
            
            let prefix_properties = ByteString::from_bytes(&[PREFIX_PROPERTIES]);
            let properties_key = prefix_properties.concat(&token_id);
            storage.put(properties_key, serialize_properties(properties));
        }
        
        // 4. Add to token list
        let tokens_key = ByteString::from_bytes(&[PREFIX_TOKEN_LIST]);
        let tokens_value = storage.get(tokens_key.clone());
        
        let mut all_tokens = if tokens_value.is_null() {
            Array::<ByteString>::new()
        } else {
            deserialize_tokens(tokens_value.unwrap())
        };
        
        all_tokens.push(token_id.clone());
        storage.put(tokens_key, serialize_tokens(all_tokens));
        
        // 5. Update total supply
        let total_supply_key = ByteString::from_bytes(&[TOTAL_SUPPLY_KEY]);
        let current_supply = BasicNFT::total_supply();
        let new_supply = current_supply.checked_add(&Int256::from_i32(1));
        storage.put(total_supply_key, new_supply.into_byte_string());
        
        // 6. Emit transfer event (from zero address for minting)
        emit_transfer_event(H160::zero(), owner, token_id, ByteString::empty());
        
        true
    }
    
    /// Destroys a token
    pub fn burn(token_id: ByteString) -> bool {
        // Get token owner
        let owner = BasicNFT::owner_of(token_id.clone());
        if owner == H160::zero() {
            return false; // Token doesn't exist
        }
        
        // Check authorization
        if !runtime::check_witness(owner.clone()) {
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
            remove_token_from_array(&mut tokens_array, &token_id);
            storage.put(owner_key, serialize_tokens(tokens_array));
        }
        
        // 3. Remove token properties
        let prefix_properties = ByteString::from_bytes(&[PREFIX_PROPERTIES]);
        let properties_key = prefix_properties.concat(&token_id);
        storage.delete(properties_key);
        
        // 4. Remove from token list
        let tokens_key = ByteString::from_bytes(&[PREFIX_TOKEN_LIST]);
        let tokens_value = storage.get(tokens_key.clone());
        
        if !tokens_value.is_null() {
            let mut all_tokens = deserialize_tokens(tokens_value.unwrap());
            remove_token_from_array(&mut all_tokens, &token_id);
            storage.put(tokens_key, serialize_tokens(all_tokens));
        }
        
        // 5. Update total supply
        let total_supply_key = ByteString::from_bytes(&[TOTAL_SUPPLY_KEY]);
        let current_supply = BasicNFT::total_supply();
        let new_supply = current_supply.checked_sub(&Int256::from_i32(1));
        storage.put(total_supply_key, new_supply.into_byte_string());
        
        // 6. Emit transfer event (to zero address for burning)
        emit_transfer_event(owner, H160::zero(), token_id, ByteString::empty());
        
        true
    }
}

/// Helper function to emit transfer events
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

/// Helper function to serialize token arrays
fn serialize_tokens(tokens: Array<ByteString>) -> ByteString {
    // Simple serialization for demonstration
    // In a production environment, you'd want more efficient serialization
    
    let mut result = ByteString::empty();
    
    // Encode length as 4 bytes
    let len = tokens.len();
    let len_bytes = [
        ((len >> 24) & 0xFF) as u8,
        ((len >> 16) & 0xFF) as u8,
        ((len >> 8) & 0xFF) as u8,
        (len & 0xFF) as u8,
    ];
    result = result.concat(&ByteString::from_bytes(&len_bytes));
    
    // Encode each token
    for i in 0..tokens.len() {
        let token = tokens.get(i);
        let token_len = token.len();
        let token_len_bytes = [
            ((token_len >> 8) & 0xFF) as u8,
            (token_len & 0xFF) as u8,
        ];
        
        // Append token length
        result = result.concat(&ByteString::from_bytes(&token_len_bytes));
        
        // Append token bytes
        result = result.concat(&token);
    }
    
    result
}

/// Helper function to deserialize token arrays
fn deserialize_tokens(data: ByteString) -> Array<ByteString> {
    let mut result = Array::<ByteString>::new();
    let bytes = data.to_bytes();
    
    // Need at least 4 bytes for length
    if bytes.len() < 4 {
        return result;
    }
    
    // Decode length
    let len = ((bytes[0] as usize) << 24) |
              ((bytes[1] as usize) << 16) |
              ((bytes[2] as usize) << 8) |
              (bytes[3] as usize);
    
    let mut pos = 4;
    for _ in 0..len {
        // Need at least 2 bytes for token length
        if pos + 2 > bytes.len() {
            break;
        }
        
        // Decode token length
        let token_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        // Need enough bytes for token data
        if pos + token_len > bytes.len() {
            break;
        }
        
        // Extract token bytes
        let token_bytes = &bytes[pos..pos + token_len];
        let token = ByteString::from_bytes(token_bytes);
        result.push(token);
        
        pos += token_len;
    }
    
    result
}

/// Helper function to serialize property maps
fn serialize_properties(properties: Map<ByteString, ByteString>) -> ByteString {
    // This is a simple implementation
    // In a production environment, you'd want more efficient serialization
    
    let mut result = ByteString::empty();
    let keys = properties.keys();
    
    // Encode number of properties
    let len = keys.len();
    let len_bytes = [
        ((len >> 24) & 0xFF) as u8,
        ((len >> 16) & 0xFF) as u8,
        ((len >> 8) & 0xFF) as u8,
        (len & 0xFF) as u8,
    ];
    result = result.concat(&ByteString::from_bytes(&len_bytes));
    
    // Encode each key-value pair
    for i in 0..keys.len() {
        let key = keys.get(i);
        let value = properties.get(key.clone()).unwrap();
        
        // Encode key length and bytes
        let key_len = key.len();
        let key_len_bytes = [
            ((key_len >> 8) & 0xFF) as u8,
            (key_len & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&key_len_bytes));
        result = result.concat(&key);
        
        // Encode value length and bytes
        let value_len = value.len();
        let value_len_bytes = [
            ((value_len >> 8) & 0xFF) as u8,
            (value_len & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&value_len_bytes));
        result = result.concat(&value);
    }
    
    result
}

/// Helper function to deserialize property maps
fn deserialize_properties(data: ByteString) -> Map<ByteString, ByteString> {
    let mut result = Map::<ByteString, ByteString>::new();
    let bytes = data.to_bytes();
    
    // Need at least 4 bytes for length
    if bytes.len() < 4 {
        return result;
    }
    
    // Decode number of properties
    let len = ((bytes[0] as usize) << 24) |
              ((bytes[1] as usize) << 16) |
              ((bytes[2] as usize) << 8) |
              (bytes[3] as usize);
    
    let mut pos = 4;
    for _ in 0..len {
        // Need at least 2 bytes for key length
        if pos + 2 > bytes.len() {
            break;
        }
        
        // Decode key length
        let key_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        // Need enough bytes for key data
        if pos + key_len > bytes.len() {
            break;
        }
        
        // Extract key bytes
        let key_bytes = &bytes[pos..pos + key_len];
        let key = ByteString::from_bytes(key_bytes);
        pos += key_len;
        
        // Need at least 2 bytes for value length
        if pos + 2 > bytes.len() {
            break;
        }
        
        // Decode value length
        let value_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        // Need enough bytes for value data
        if pos + value_len > bytes.len() {
            break;
        }
        
        // Extract value bytes
        let value_bytes = &bytes[pos..pos + value_len];
        let value = ByteString::from_bytes(value_bytes);
        pos += value_len;
        
        // Insert key-value pair
        result.insert(key, value);
    }
    
    result
}

/// Helper function to remove a token from an array
fn remove_token_from_array(tokens: &mut Array<ByteString>, token_id: &ByteString) {
    let mut new_tokens = Array::<ByteString>::new();
    
    for i in 0..tokens.len() {
        let token = tokens.get(i);
        if token != *token_id {
            new_tokens.push(token);
        }
    }
    
    // Replace all elements
    *tokens = new_tokens;
} 