//! Compilation Example
//! 
//! This is a simple counter contract that demonstrates the compilation process
//! from Rust to Neo VM bytecode.

use neo_contract::prelude::*;

#[neo_contract::contract]
pub struct Counter {
    // Storage for the counter value
    count: StorageMap<String, u64>,
    // Storage for the owner of the contract
    owner: StorageMap<String, H160>,
}

impl Counter {
    /// Constructor that initializes the counter and sets the owner
    #[constructor]
    pub fn new(initial_value: u64) -> Self {
        let sender = Runtime::calling_script_hash();
        
        emit!(Initialized {
            owner: sender,
            initial_value,
        });
        
        Self {
            count: StorageMap::new(b"count"),
            owner: StorageMap::new(b"owner"),
        }.setup(initial_value, sender)
    }
    
    /// Alternative constructor that starts from zero
    #[constructor]
    pub fn new_zero() -> Self {
        Self::new(0)
    }
    
    /// Get the current counter value
    #[safe]
    pub fn get_count(&self) -> u64 {
        self.count.get("value").unwrap_or_default()
    }
    
    /// Increment the counter by one
    #[method]
    pub fn increment(&mut self) -> u64 {
        let current = self.get_count();
        let new_value = current + 1;
        self.count.put("value", new_value);
        
        emit!(CountIncremented {
            from: current,
            to: new_value,
        });
        
        new_value
    }
    
    /// Decrement the counter by one
    #[method]
    pub fn decrement(&mut self) -> u64 {
        let current = self.get_count();
        if current > 0 {
            let new_value = current - 1;
            self.count.put("value", new_value);
            
            emit!(CountDecremented {
                from: current,
                to: new_value,
            });
            
            new_value
        } else {
            // Can't go below zero
            current
        }
    }
    
    /// Reset the counter to a specific value (owner only)
    #[method]
    pub fn reset(&mut self, new_value: u64) -> bool {
        let sender = Runtime::calling_script_hash();
        if sender != self.owner.get("owner").unwrap_or_default() {
            return false;
        }
        
        let old_value = self.get_count();
        self.count.put("value", new_value);
        
        emit!(CountReset {
            from: old_value,
            to: new_value,
            reset_by: sender,
        });
        
        true
    }
    
    /// Transfer ownership of the contract (owner only)
    #[method]
    pub fn transfer_ownership(&mut self, new_owner: H160) -> bool {
        let sender = Runtime::calling_script_hash();
        if sender != self.owner.get("owner").unwrap_or_default() {
            return false;
        }
        
        self.owner.put("owner", new_owner);
        
        emit!(OwnershipTransferred {
            from: sender,
            to: new_owner,
        });
        
        true
    }
    
    /// Get the current owner
    #[safe]
    pub fn get_owner(&self) -> H160 {
        self.owner.get("owner").unwrap_or_default()
    }
    
    /// Check if the caller is the owner
    #[safe]
    pub fn is_owner(&self) -> bool {
        Runtime::calling_script_hash() == self.owner.get("owner").unwrap_or_default()
    }

    /// Event emitted when the contract is initialized
    #[event]
    pub struct Initialized {
        #[index]
        owner: H160,
        initial_value: u64,
    }
    
    /// Event emitted when the counter is incremented
    #[event]
    pub struct CountIncremented {
        from: u64,
        to: u64,
    }
    
    /// Event emitted when the counter is decremented
    #[event]
    pub struct CountDecremented {
        from: u64,
        to: u64,
    }
    
    /// Event emitted when the counter is reset
    #[event]
    pub struct CountReset {
        from: u64,
        to: u64,
        #[index]
        reset_by: H160,
    }
    
    /// Event emitted when ownership is transferred
    #[event]
    pub struct OwnershipTransferred {
        #[index]
        from: H160,
        #[index]
        to: H160,
    }

    fn setup(&mut self, initial_value: u64, owner: H160) -> Self {
        self.count.put("value", initial_value);
        self.owner.put("owner", owner);
        self
    }
}