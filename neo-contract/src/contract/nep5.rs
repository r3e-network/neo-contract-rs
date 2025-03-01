// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

/// NEP5 Token Standard Interface
/// This is the legacy token standard for NEO, included for backward compatibility
pub trait Nep5Token {
    /// Gets the name of the token
    fn name() -> ByteString;

    /// Gets the symbol of the token
    fn symbol() -> ByteString;

    /// Gets the number of decimal places
    fn decimals() -> u8;

    /// Gets the total token supply deployed in the system
    fn total_supply() -> Int256;

    /// Gets the token balance of a specific account
    fn balance_of(account: H160) -> Int256;

    /// Transfers tokens from one account to another
    fn transfer(from: H160, to: H160, amount: Int256) -> bool;
}

/// Implementation of the NEP5 token standard
pub trait Nep5TokenImpl: Nep5Token {
    /// Initializes the token with name, symbol, decimals, and total supply
    fn initialize(name: ByteString, symbol: ByteString, decimals: u8, total_supply: Int256, owner: H160) -> bool;

    /// Mints new tokens and assigns them to the specified account
    fn mint(to: H160, amount: Int256) -> bool;

    /// Burns tokens from the specified account
    fn burn(from: H160, amount: Int256) -> bool;
}

/// Helper functions for NEP5 token implementation
pub mod nep5_helpers {
    use super::*;
    use crate::storage::StorageMap;
    use crate::types::consts::DEFAULT_BALANCE_KEY;

    /// Updates the balance of an account
    pub fn update_nep5_balance(account: H160, amount: Int256) -> bool {
        if amount.is_zero() {
            return true;
        }
        
        let storage = StorageMap::new();
        
        // Create key for balance
        let mut key = Vec::with_capacity(account.as_bytes().len() + 1);
        key.push(DEFAULT_BALANCE_KEY);
        key.extend_from_slice(account.as_bytes());
        
        // Get current balance
        let current_balance: Int256 = storage.get(&key).unwrap_or_default();
        
        // Calculate new balance
        let new_balance = current_balance + amount;
        
        // Check for negative balance
        if new_balance.is_negative() {
            return false;
        }
        
        // Update storage
        if new_balance.is_zero() {
            storage.delete(&key);
        } else {
            storage.put(&key, &new_balance);
        }
        
        true
    }
}
