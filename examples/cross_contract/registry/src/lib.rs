//! # Contract Registry Example
//!
//! This example demonstrates a contract registry pattern for Neo N3 smart contracts,
//! which allows for maintaining a centralized registry of contract addresses that
//! other contracts can query.

use neo_contract::prelude::*;

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Cross Contract Registry Example for Neo N3")]
#[contract_version("0.1.0")]
pub mod contract_registry {
    use super::*;

    // Events
    struct ContractRegistered {}
    
    impl ContractRegistered {
        pub fn emit(name: String, hash: H160, timestamp: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("ContractRegistered");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            event_data.push(Any::from(name));
            event_data.push(Any::from(hash));
            event_data.push(Any::from(timestamp));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }

    struct ContractUpdated {}
    
    impl ContractUpdated {
        pub fn emit(name: String, old_hash: H160, new_hash: H160, timestamp: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("ContractUpdated");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            event_data.push(Any::from(name));
            event_data.push(Any::from(old_hash));
            event_data.push(Any::from(new_hash));
            event_data.push(Any::from(timestamp));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }

    struct OwnershipTransferred {}
    
    impl OwnershipTransferred {
        pub fn emit(from: H160, to: H160, timestamp: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("OwnershipTransferred");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            event_data.push(Any::from(from));
            event_data.push(Any::from(to));
            event_data.push(Any::from(timestamp));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }

    // Contract storage
    pub struct ContractRegistry {
        // Registry entries
        contracts: StorageMap<String, H160>,
        contracts_list: StorageItem<Vec<String>>,
        
        // Ownership
        owner: StorageItem<H160>,
        pending_owner: StorageItem<H160>,
        
        // Access control
        authorized_managers: StorageMap<H160, bool>,
    }

    impl ContractRegistry {
        #[constructor]
        pub fn new() -> Self {
            let owner = Runtime::current_sender();
            Self {
                contracts: StorageMap::new(),
                contracts_list: StorageItem::new(Vec::new()),
                owner: StorageItem::new(owner),
                pending_owner: StorageItem::new(H160::zero()),
                authorized_managers: StorageMap::new(),
            }
        }

        // ===== Registry Methods =====
        
        /// Register a new contract
        #[method]
        pub fn register_contract(&mut self, name: String, contract_hash: H160) -> bool {
            // Ensure caller is authorized
            self.ensure_authorized();
            
            // Check if the contract already exists
            if self.contracts.contains_key(&name) {
                panic!("Contract with this name already exists");
            }
            
            // Register the contract
            self.contracts.insert(&name, &contract_hash);
            
            // Add to the list of contracts
            let mut contracts_list = self.contracts_list.get();
            contracts_list.push(name.clone());
            self.contracts_list.set(contracts_list);
            
            // Emit event
            ContractRegistered::emit(name, contract_hash, Ledger::current_timestamp());
            
            true
        }
        
        /// Update an existing contract address
        #[method]
        pub fn update_contract(&mut self, name: String, contract_hash: H160) -> bool {
            // Ensure caller is authorized
            self.ensure_authorized();
            
            // Check if the contract exists
            if !self.contracts.contains_key(&name) {
                panic!("Contract does not exist");
            }
            
            // Get the old hash for the event
            let old_hash = self.contracts.get(&name).unwrap();
            
            // Update the contract
            self.contracts.insert(&name, &contract_hash);
            
            // Emit event
            ContractUpdated::emit(name, old_hash, contract_hash, Ledger::current_timestamp());
            
            true
        }
        
        /// Get a contract address by name
        #[safe]
        pub fn get_contract(&self, name: String) -> Option<H160> {
            self.contracts.get(&name)
        }
        
        /// List all registered contract names
        #[safe]
        pub fn list_contracts(&self) -> Vec<String> {
            self.contracts_list.get()
        }
        
        /// Count of registered contracts
        #[safe]
        pub fn contract_count(&self) -> usize {
            self.contracts_list.get().len()
        }
        
        /// Check if a contract with the given name exists
        #[safe]
        pub fn contains_contract(&self, name: String) -> bool {
            self.contracts.contains_key(&name)
        }

        // ===== Administration Methods =====
        
        /// Add an authorized manager
        #[method]
        pub fn add_manager(&mut self, manager: H160) -> bool {
            // Only owner can add managers
            self.ensure_owner();
            
            self.authorized_managers.insert(&manager, &true);
            true
        }
        
        /// Remove an authorized manager
        #[method]
        pub fn remove_manager(&mut self, manager: H160) -> bool {
            // Only owner can remove managers
            self.ensure_owner();
            
            self.authorized_managers.insert(&manager, &false);
            true
        }
        
        /// Check if an address is an authorized manager
        #[safe]
        pub fn is_manager(&self, address: H160) -> bool {
            self.authorized_managers.get(&address).unwrap_or(false)
        }
        
        /// Begin ownership transfer
        #[method]
        pub fn transfer_ownership(&mut self, new_owner: H160) -> bool {
            // Only current owner can transfer ownership
            self.ensure_owner();
            
            // Set the pending owner
            self.pending_owner.set(new_owner);
            true
        }
        
        /// Complete ownership transfer
        #[method]
        pub fn accept_ownership(&mut self) -> bool {
            // Get the sender
            let sender = Runtime::current_sender();
            
            // Check if sender is the pending owner
            let pending_owner = self.pending_owner.get();
            assert!(sender == pending_owner, "Not pending owner");
            
            // Transfer ownership
            let old_owner = self.owner.get();
            self.owner.set(sender);
            self.pending_owner.set(H160::zero());
            
            // Emit event
            OwnershipTransferred::emit(old_owner, sender, Ledger::current_timestamp());
            
            true
        }
        
        /// Get the current owner
        #[safe]
        pub fn get_owner(&self) -> H160 {
            self.owner.get()
        }
        
        /// Get the pending owner
        #[safe]
        pub fn get_pending_owner(&self) -> H160 {
            self.pending_owner.get()
        }
        
        // ===== Internal Helper Methods =====
        
        fn ensure_owner(&self) {
            let sender = Runtime::current_sender();
            assert!(sender == self.owner.get(), "Not authorized: not owner");
            assert!(Runtime::check_witness(&sender), "No witness");
        }
        
        fn ensure_authorized(&self) {
            let sender = Runtime::current_sender();
            
            // Check if sender is owner or an authorized manager
            let is_owner = sender == self.owner.get();
            let is_manager = self.authorized_managers.get(&sender).unwrap_or(false);
            
            assert!(is_owner || is_manager, "Not authorized");
            assert!(Runtime::check_witness(&sender), "No witness");
        }
    }
} 