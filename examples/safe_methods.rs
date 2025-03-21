// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use neo::prelude::*;

#[neo::contract]
pub struct TokenContract {
    storage: StorageMap,
}

#[neo::contract]
impl TokenContract {
    // Constructor to initialize the contract
    pub fn init(owner: H160, total_supply: Int256) -> Self {
        let mut instance = Self {
            storage: StorageMap::new(),
        };
        
        instance.storage.put("owner", owner);
        instance.storage.put("total_supply", total_supply);
        
        // Emit an event when tokens are created
        instance.emit_transfer(None, Some(owner), total_supply);
        
        instance
    }
    
    // Safe method - doesn't modify contract state, only reads it
    #[safe]
    pub fn total_supply() -> Int256 {
        let storage = StorageMap::new();
        storage.get("total_supply").unwrap_or_default()
    }
    
    // Safe method - only reads the balance of an account
    #[safe]
    pub fn balance_of(account: H160) -> Int256 {
        let storage = StorageMap::new();
        storage.get(&account).unwrap_or_default()
    }
    
    // Unsafe method - modifies contract state
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        // Check transaction sender is authorized
        assert!(Runtime::check_witness(&from), "Not authorized");
        
        let mut storage = StorageMap::new();
        
        // Get current balances
        let from_balance: Int256 = storage.get(&from).unwrap_or_default();
        let to_balance: Int256 = storage.get(&to).unwrap_or_default();
        
        // Check sufficient balance
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        storage.put(&from, from_balance - amount);
        storage.put(&to, to_balance + amount);
        
        // Emit transfer event
        Self::emit_transfer(Some(from), Some(to), amount);
        
        true
    }
    
    // Safe method - only reads contract state to check if an address is the owner
    #[safe]
    pub fn is_owner(address: H160) -> bool {
        let storage = StorageMap::new();
        let owner: H160 = storage.get("owner").unwrap();
        address == owner
    }
    
    // Helper function to emit transfer events
    fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(amount));
        
        Runtime::notify(&event_name, &event_data);
    }
}
