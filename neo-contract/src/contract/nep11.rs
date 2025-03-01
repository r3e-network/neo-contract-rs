// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{contract::*, runtime, storage::{StorageMap, Iter}, types::*};

pub trait TokenState {
    fn name(&self) -> ByteString;
    fn symbol(&self) -> ByteString;
    fn owner(&self) -> H160;
}

pub trait Nep11Token<T: TokenState + Clone>: TokenContract {
    fn owner_of(token_id: ByteString) -> H160 {
        let storage = StorageMap::new();
        let mut key = Vec::with_capacity(token_id.len() + 6);
        key.extend_from_slice(b"owner:");
        key.extend_from_slice(token_id.as_bytes());
        storage.get::<H160>(&key).unwrap_or_default()
    }

    fn properties(token_id: ByteString) -> Map<ByteString, Any> {
        let storage = StorageMap::new();
        let mut key = Vec::with_capacity(token_id.len() + 6);
        key.extend_from_slice(b"props:");
        key.extend_from_slice(token_id.as_bytes());
        storage.get::<Map<ByteString, Any>>(&key).unwrap_or_default()
    }

    fn tokens() -> Iter<T> {
        let storage = StorageMap::new();
        storage.find(b"token:")
    }
    
    fn tokens_of(owner: H160) -> Iter<T> {
        let storage = StorageMap::new();
        let mut prefix = Vec::with_capacity(owner.as_bytes().len() + 13);
        prefix.extend_from_slice(b"owner_tokens:");
        prefix.extend_from_slice(owner.as_bytes());
        storage.find(&prefix)
    }

    fn mint(token_id: ByteString, token_state: T) {
        let storage = StorageMap::new();
        
        // Store token owner
        let owner = token_state.owner();
        let mut owner_key = Vec::with_capacity(token_id.len() + 6);
        owner_key.extend_from_slice(b"owner:");
        owner_key.extend_from_slice(token_id.as_bytes());
        storage.put(&owner_key, &owner);
        
        // Store token properties
        let mut props_key = Vec::with_capacity(token_id.len() + 6);
        props_key.extend_from_slice(b"props:");
        props_key.extend_from_slice(token_id.as_bytes());
        let mut props = Map::<ByteString, Any>::new();
        // Create Any objects directly
        #[cfg(not(target_family = "wasm"))]
        {
            let name_any = Any::new(token_state.name());
            props.insert("name".into(), name_any);
        }
        #[cfg(target_family = "wasm")]
        {
            props.insert("name".into(), token_state.name().into());
        }
        storage.put(&props_key, &props);
        
        // Add to owner's tokens
        let mut owner_tokens_key = Vec::with_capacity(owner.as_bytes().len() + 13);
        owner_tokens_key.extend_from_slice(b"owner_tokens:");
        owner_tokens_key.extend_from_slice(owner.as_bytes());
        let mut owner_tokens = storage.get::<Array<ByteString>>(&owner_tokens_key).unwrap_or_default();
        owner_tokens.push(token_id.clone());
        storage.put(&owner_tokens_key, &owner_tokens);
        
        // Add to all tokens
        let mut token_key = Vec::with_capacity(token_id.len() + 6);
        token_key.extend_from_slice(b"token:");
        token_key.extend_from_slice(token_id.as_bytes());
        storage.put(&token_key, &token_id);
    }

    fn burn(token_id: ByteString) {
        let storage = StorageMap::new();
        
        // Get owner
        let owner = Self::owner_of(token_id.clone());
        if owner == H160::default() {
            runtime::abort();
            return;
        }
        
        // Remove token owner
        let mut owner_key = Vec::with_capacity(token_id.len() + 6);
        owner_key.extend_from_slice(b"owner:");
        owner_key.extend_from_slice(token_id.as_bytes());
        storage.delete(&owner_key);
        
        // Remove token properties
        let mut props_key = Vec::with_capacity(token_id.len() + 6);
        props_key.extend_from_slice(b"props:");
        props_key.extend_from_slice(token_id.as_bytes());
        storage.delete(&props_key);
        
        // Remove from owner's tokens
        let mut owner_tokens_key = Vec::with_capacity(owner.as_bytes().len() + 13);
        owner_tokens_key.extend_from_slice(b"owner_tokens:");
        owner_tokens_key.extend_from_slice(owner.as_bytes());
        let mut owner_tokens = storage.get::<Array<ByteString>>(&owner_tokens_key).unwrap_or_default();
        let index = owner_tokens.iter().position(|t| *t == token_id).unwrap_or(owner_tokens.len());
        if index < owner_tokens.len() {
            owner_tokens.remove(index);
            storage.put(&owner_tokens_key, &owner_tokens);
        }
        
        // Remove from all tokens
        let mut token_key = Vec::with_capacity(token_id.len() + 6);
        token_key.extend_from_slice(b"token:");
        token_key.extend_from_slice(token_id.as_bytes());
        storage.delete(&token_key);
    }
    
    fn transfer(from: H160, to: H160, token_id: ByteString) -> bool {
        if !runtime::check_witness_with_account(from) {
            return false;
        }
        
        let owner = Self::owner_of(token_id.clone());
        if owner != from {
            return false;
        }
        
        let storage = StorageMap::new();
        
        // Update token owner
        let mut owner_key = Vec::with_capacity(token_id.len() + 6);
        owner_key.extend_from_slice(b"owner:");
        owner_key.extend_from_slice(token_id.as_bytes());
        storage.put(&owner_key, &to);
        
        // Remove from previous owner's tokens
        let mut from_tokens_key = Vec::with_capacity(from.as_bytes().len() + 13);
        from_tokens_key.extend_from_slice(b"owner_tokens:");
        from_tokens_key.extend_from_slice(from.as_bytes());
        let mut from_tokens = storage.get::<Array<ByteString>>(&from_tokens_key).unwrap_or_default();
        let index = from_tokens.iter().position(|t| *t == token_id).unwrap_or(from_tokens.len());
        if index < from_tokens.len() {
            from_tokens.remove(index);
            storage.put(&from_tokens_key, &from_tokens);
        }
        
        // Add to new owner's tokens
        let mut to_tokens_key = Vec::with_capacity(to.as_bytes().len() + 13);
        to_tokens_key.extend_from_slice(b"owner_tokens:");
        to_tokens_key.extend_from_slice(to.as_bytes());
        let mut to_tokens = storage.get::<Array<ByteString>>(&to_tokens_key).unwrap_or_default();
        to_tokens.push(token_id);
        storage.put(&to_tokens_key, &to_tokens);
        
        return true;
    }
}

pub fn update_nep11_balance(owner: H160, token_id: ByteString, increment: Int256) {
    let storage = StorageMap::new();
    
    // Create key directly
    let mut key = Vec::with_capacity(owner.as_bytes().len() + token_id.len() + 8);
    key.extend_from_slice(b"balance:");
    key.extend_from_slice(owner.as_bytes());
    key.extend_from_slice(token_id.as_bytes());
    
    let balance = storage.get::<Int256>(&key).unwrap_or_default();
    let new_balance = balance + increment;
    
    if new_balance.is_zero() {
        storage.delete(&key);
    } else {
        storage.put(&key, &new_balance);
    }
}

pub fn get_nep11_balance(owner: H160, token_id: ByteString) -> Int256 {
    let storage = StorageMap::new();
    
    // Create key directly
    let mut key = Vec::with_capacity(owner.as_bytes().len() + token_id.len() + 8);
    key.extend_from_slice(b"balance:");
    key.extend_from_slice(owner.as_bytes());
    key.extend_from_slice(token_id.as_bytes());
    
    storage.get::<Int256>(&key).unwrap_or_default()
}

pub fn set_nep11_balance(owner: H160, token_id: ByteString, balance: Int256) {
    let storage = StorageMap::new();
    
    // Create key directly
    let mut key = Vec::with_capacity(owner.as_bytes().len() + token_id.len() + 8);
    key.extend_from_slice(b"balance:");
    key.extend_from_slice(owner.as_bytes());
    key.extend_from_slice(token_id.as_bytes());
    
    if balance.is_zero() {
        storage.delete(&key);
    } else {
        storage.put(&key, &balance);
    }
}

