//! Compilation Example
//! 
//! This is a simple counter contract that demonstrates the compilation process
//! from Rust to Neo VM bytecode.

use neo_contract::prelude::*;

#[contract]
pub struct Counter {
    // Storage for the counter value
    count: StorageItem<u64>,
    // Storage for the owner of the contract
    owner: StorageItem<Address>,
}

#[contractimpl]
impl Counter {
    /// Constructor that initializes the counter and sets the owner
    #[constructor]
    pub fn new(initial_value: u64) -> Self {
        let sender = Runtime::current_sender();
        
        emit!(Initialized {
            owner: sender,
            initial_value,
        });
        
        Self {
            count: StorageItem::new(initial_value),
            owner: StorageItem::new(sender),
        }
    }
    
    /// Alternative constructor that starts from zero
    #[constructor]
    pub fn new_zero() -> Self {
        Self::new(0)
    }
    
    /// Get the current counter value
    #[method(safe)]
    pub fn get_count(&self) -> u64 {
        self.count.get()
    }
    
    /// Increment the counter by one
    #[method]
    pub fn increment(&mut self) -> u64 {
        let current = self.count.get();
        let new_value = current + 1;
        self.count.set(new_value);
        
        emit!(CountIncremented {
            from: current,
            to: new_value,
        });
        
        new_value
    }
    
    /// Decrement the counter by one
    #[method]
    pub fn decrement(&mut self) -> u64 {
        let current = self.count.get();
        if current > 0 {
            let new_value = current - 1;
            self.count.set(new_value);
            
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
        let sender = Runtime::current_sender();
        if sender != self.owner.get() {
            return false;
        }
        
        let old_value = self.count.get();
        self.count.set(new_value);
        
        emit!(CountReset {
            from: old_value,
            to: new_value,
            reset_by: sender,
        });
        
        true
    }
    
    /// Transfer ownership of the contract (owner only)
    #[method]
    pub fn transfer_ownership(&mut self, new_owner: Address) -> bool {
        let sender = Runtime::current_sender();
        if sender != self.owner.get() {
            return false;
        }
        
        self.owner.set(new_owner);
        
        emit!(OwnershipTransferred {
            from: sender,
            to: new_owner,
        });
        
        true
    }
    
    /// Get the current owner
    #[method(safe)]
    pub fn get_owner(&self) -> Address {
        self.owner.get()
    }
    
    /// Check if the caller is the owner
    #[method(safe)]
    pub fn is_owner(&self) -> bool {
        Runtime::current_sender() == self.owner.get()
    }

    /// Event emitted when the contract is initialized
    #[event]
    pub struct Initialized {
        #[indexed]
        owner: Address,
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
        #[indexed]
        reset_by: Address,
    }
    
    /// Event emitted when ownership is transferred
    #[event]
    pub struct OwnershipTransferred {
        #[indexed]
        from: Address,
        #[indexed]
        to: Address,
    }
}