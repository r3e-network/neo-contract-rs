#![cfg_attr(not(test), no_std)]
#![allow(unused_imports)]

extern crate alloc;

// Import standard types
use alloc::string::String;
use alloc::vec::Vec;

// Import everything from prelude
use neo_contract::prelude::*;

// Import core types directly
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::string::ByteString;
use neo_contract::types::builtin::array::Array;
use neo_contract::types::builtin::any::Any;
use neo_contract::Runtime;
use neo_contract::storage::item::Item;
use neo_contract::storage::map::Map as StorageMap;

/// A NEP-17 fungible token implementation for Neo N3
/// 
/// This demonstrates how to structure a token contract using the annotation syntax
/// while working around current limitations in the framework.

// Contract module with metadata
// #[contract]
// #[contract_author("NEO Rust Team")]
// #[contract_description("NEP-17 Token Example")]
// #[contract_version("0.1.0")]
// #[supported_standards("NEP-17")]
mod token_contract {
    use super::*;
    
    // Event definitions
    // #[event]
    pub struct Transfer {
        // #[index]
        pub from: Option<H160>,
        // #[index]
        pub to: Option<H160>,
        pub amount: u64,
    }

    // Event implementation
    impl Transfer {
        pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
            let mut event_args = Array::new();
            
            // Add from parameter (handle Option type)
            match from {
                Some(addr) => event_args.push(Any::from(addr)),
                None => event_args.push(Any::null()),
            }
            
            // Add to parameter (handle Option type)
            match to {
                Some(addr) => event_args.push(Any::from(addr)),
                None => event_args.push(Any::null()),
            }
            
            // Add amount
            event_args.push(Any::integer(amount as i64));
            
            // Emit event
            Runtime::notify(&ByteString::from("Transfer"), &event_args);
        }
    }
    
    // Contract storage definition
    // #[storage]
    pub struct TokenContract {
        pub balances: StorageMap<H160, u64>,
        pub total_supply: Item<u64>,
        pub token_name: Item<ByteString>,
        pub token_symbol: Item<ByteString>,
        pub token_decimals: Item<u8>,
        pub owner: Item<H160>,
    }
    
    impl TokenContract {
        // Constructor for initializing the contract
        // #[constructor]
        pub fn new(owner: H160, initial_supply: u64) -> Self {
            let mut contract = Self {
                balances: StorageMap::new(b"balances"),
                total_supply: Item::new(b"total_supply"),
                token_name: Item::new(b"token_name"),
                token_symbol: Item::new(b"token_symbol"),
                token_decimals: Item::new(b"token_decimals"),
                owner: Item::new(b"owner"),
            };
            
            // Set token metadata
            contract.token_name.set(&ByteString::from("NEO Sample Token")).unwrap_or(());
            contract.token_symbol.set(&ByteString::from("NST")).unwrap_or(());
            contract.token_decimals.set(&8).unwrap_or(());
            
            // Set total supply
            contract.total_supply.set(&initial_supply).unwrap_or(());
            
            // Set owner
            contract.owner.set(&owner).unwrap_or(());
            
            // Initialize owner balance with initial supply
            contract.balances.set(&owner, &initial_supply).unwrap_or(());
            
            // Emit transfer event for minting
            Transfer::emit(None, Some(owner), initial_supply);
            
            contract
        }
        
        // NEP-17 Methods
        
        // #[safe]
        pub fn symbol(&self) -> ByteString {
            self.token_symbol.get().unwrap_or(None).unwrap_or_default()
        }
        
        // #[safe]
        pub fn decimals(&self) -> u8 {
            self.token_decimals.get().unwrap_or(None).unwrap_or(8)
        }
        
        // #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or(None).unwrap_or(0)
        }
        
        // #[safe]
        pub fn balance_of(&self, account: H160) -> u64 {
            self.balances.get(&account).unwrap_or(None).unwrap_or(0)
        }
        
        // #[method]
        pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
            // Check that amount is positive
            if amount == 0 {
                return false;
            }
            
            // Check from has authorized the transfer
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check from has sufficient balance
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            if from == to {
                // No need to actually transfer if from and to are the same
                return true;
            }
            
            // Calculate new balances
            let new_from_balance = from_balance - amount;
            
            // Only store non-zero balances
            if new_from_balance > 0 {
                self.balances.set(&from, &new_from_balance).unwrap_or(());
            } else {
                self.balances.delete(&from).unwrap_or(());
            }
            
            // Update or add recipient balance
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.set(&to, &new_to_balance).unwrap_or(());
            
            // Emit the transfer event
            Transfer::emit(Some(from), Some(to), amount);
            
            // If recipient is a contract, call onNEP17Payment
            if is_contract(&to) {
                // Stub implementation - would call the contract
                // In real implementation, would use CallFlags and call onNEP17Payment
            }
            
            true
        }
        
        // Owner methods
        
        // #[method]
        pub fn mint(&mut self, to: H160, amount: u64) -> bool {
            // Only owner can mint tokens
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Update recipient balance
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.set(&to, &new_to_balance).unwrap_or(());
            
            // Update total supply
            let supply = self.total_supply();
            let new_supply = supply + amount;
            self.total_supply.set(&new_supply).unwrap_or(());
            
            // Emit the transfer event (mint)
            Transfer::emit(None, Some(to), amount);
            
            true
        }
        
        // #[method]
        pub fn burn(&mut self, from: H160, amount: u64) -> bool {
            // Check that from has authorized the burn
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check from has sufficient balance
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            // Update balance
            let new_from_balance = from_balance - amount;
            
            // Only store non-zero balances
            if new_from_balance > 0 {
                self.balances.set(&from, &new_from_balance).unwrap_or(());
            } else {
                self.balances.delete(&from).unwrap_or(());
            }
            
            // Update total supply
            let supply = self.total_supply();
            let new_supply = supply - amount;
            self.total_supply.set(&new_supply).unwrap_or(());
            
            // Emit the transfer event (burn)
            Transfer::emit(Some(from), None, amount);
            
            true
        }
        
        // #[method]
        pub fn update_owner(&mut self, new_owner: H160) -> bool {
            // Only current owner can update owner
            let current_owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&current_owner) {
                return false;
            }
            
            // Update owner
            self.owner.set(&new_owner).unwrap_or(());
            
            true
        }
    }
}

// Helper function to check if address is a contract
fn is_contract(_address: &H160) -> bool {
    // In a real implementation, would check if the address is a contract
    // For simplicity, always return false in this example
    false
}

// Entry points for Neo VM
#[no_mangle]
pub fn deploying() -> bool {
    // Initialize token contract during deployment
    let owner = Runtime::calling_script_hash();
    let initial_supply = 1_000_000 * 100_000_000; // 1 million tokens with 8 decimals
    
    // Create token contract
    let _contract = token_contract::TokenContract::new(owner, initial_supply);
    
    true
}

#[no_mangle]
pub fn invoke(operation: String, args: Vec<Any>) -> Any {
    // Create contract instance
    let mut contract = token_contract::TokenContract::new(H160::default(), 0);
    
    match operation.as_str() {
        // NEP-17 standard methods
        "symbol" => {
            Any::byte_string(contract.symbol())
        },
        "decimals" => {
            Any::integer(contract.decimals() as i64)
        },
        "totalSupply" => {
            Any::integer(contract.total_supply() as i64)
        },
        "balanceOf" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let account = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::integer(contract.balance_of(account) as i64)
        },
        "transfer" => {
            if args.len() < 3 {
                return Any::boolean(false);
            }
            
            let from = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let to = if let Some(addr) = args[1].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let amount = if let Some(val) = args[2].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            // Try to parse memo from data if provided
            if args.len() > 3 {
                // Use as_bytestring method which is available on Any
                if let Some(bs) = args[3].as_bytestring() {
                    // Process the memo (nothing to do in this example)
                    let _ = bs.as_bytes();
                }
            }
            
            Any::boolean(contract.transfer(from, to, amount, Vec::new()))
        },
        
        // Owner methods
        "mint" => {
            if args.len() != 2 {
                return Any::boolean(false);
            }
            
            let to = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let amount = if let Some(val) = args[1].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.mint(to, amount))
        },
        "burn" => {
            if args.len() != 2 {
                return Any::boolean(false);
            }
            
            let from = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let amount = if let Some(val) = args[1].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.burn(from, amount))
        },
        "updateOwner" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let new_owner = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.update_owner(new_owner))
        },
        
        // Fallback
        _ => {
            Any::null()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock runtime for testing
    mod test_runtime {
        use super::*;
        use std::cell::RefCell;
        use std::collections::HashMap;
        
        thread_local! {
            static STORAGE: RefCell<HashMap<Vec<u8>, Vec<u8>>> = RefCell::new(HashMap::new());
            static EVENTS: RefCell<Vec<(String, Vec<Any>)>> = RefCell::new(Vec::new());
            static WITNESSES: RefCell<Vec<H160>> = RefCell::new(Vec::new());
        }
        
        pub fn mock_storage_put(key: &[u8], value: &[u8]) {
            STORAGE.with(|s| {
                s.borrow_mut().insert(key.to_vec(), value.to_vec());
            });
        }
        
        pub fn mock_storage_get(key: &[u8]) -> Option<Vec<u8>> {
            STORAGE.with(|s| {
                s.borrow().get(key).cloned()
            })
        }
        
        pub fn mock_check_witness(address: &H160) -> bool {
            WITNESSES.with(|w| {
                w.borrow().contains(address)
            })
        }
        
        pub fn mock_notify(name: &str, args: &[Any]) {
            EVENTS.with(|e| {
                e.borrow_mut().push((name.to_string(), args.to_vec()));
            });
        }
        
        pub fn add_witness(address: H160) {
            WITNESSES.with(|w| {
                w.borrow_mut().push(address);
            });
        }
        
        pub fn get_events() -> Vec<(String, Vec<Any>)> {
            EVENTS.with(|e| {
                e.borrow().clone()
            })
        }
        
        pub fn reset() {
            STORAGE.with(|s| s.borrow_mut().clear());
            EVENTS.with(|e| e.borrow_mut().clear());
            WITNESSES.with(|w| w.borrow_mut().clear());
        }
    }
    
    // Mocks for Runtime global functions
    impl Runtime {
        #[allow(unused_variables)]
        pub fn check_witness(address: &H160) -> bool {
            test_runtime::mock_check_witness(address)
        }
        
        #[allow(unused_variables)]
        pub fn notify(name: &ByteString, args: &Array) -> i32 {
            // Convert Array to Vec<Any> for our mock
            let args_vec: Vec<Any> = args.iter().map(|a| a.clone()).collect();
            test_runtime::mock_notify(name.as_string(), &args_vec);
            0
        }
        
        pub fn calling_script_hash() -> H160 {
            // Default test account
            H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap()
        }
    }
    
    // Helper to create a test contract
    fn setup_test_contract() -> token_contract::TokenContract {
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        token_contract::TokenContract::new(owner, 1000)
    }
    
    // NEP-17 Standard Unit Tests
    
    #[test]
    fn test_symbol() {
        let contract = setup_test_contract();
        assert_eq!(contract.symbol().as_string(), "NST");
    }
    
    #[test]
    fn test_decimals() {
        let contract = setup_test_contract();
        assert_eq!(contract.decimals(), 8);
    }
    
    #[test]
    fn test_total_supply() {
        let contract = setup_test_contract();
        assert_eq!(contract.total_supply(), 1000);
    }
    
    #[test]
    fn test_balance_of() {
        let contract = setup_test_contract();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let user = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        assert_eq!(contract.balance_of(owner), 1000);
        assert_eq!(contract.balance_of(user), 0);
    }
    
    #[test]
    fn test_transfer_success() {
        // Arrange
        let mut contract = setup_test_contract();
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset mock state and add witness
        test_runtime::reset();
        test_runtime::add_witness(from);
        
        // Act
        let result = contract.transfer(from, to, 100, vec![]);
        
        // Assert
        assert!(result);
        assert_eq!(contract.balance_of(from), 900);
        assert_eq!(contract.balance_of(to), 100);
    }
    
    #[test]
    fn test_transfer_insufficient_balance() {
        // Arrange
        let mut contract = setup_test_contract();
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset mock state and add witness
        test_runtime::reset();
        test_runtime::add_witness(from);
        
        // Act
        let result = contract.transfer(from, to, 2000, vec![]);
        
        // Assert
        assert!(!result);
        assert_eq!(contract.balance_of(from), 1000); // Unchanged
        assert_eq!(contract.balance_of(to), 0);      // Unchanged
    }
    
    #[test]
    fn test_transfer_unauthorized() {
        // Arrange
        let mut contract = setup_test_contract();
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset mock state - do NOT add from as a witness
        test_runtime::reset();
        
        // Act
        let result = contract.transfer(from, to, 100, vec![]);
        
        // Assert
        assert!(!result);
        assert_eq!(contract.balance_of(from), 1000); // Unchanged
        assert_eq!(contract.balance_of(to), 0);      // Unchanged
    }
    
    #[test]
    fn test_transfer_emits_event() {
        // Arrange
        let mut contract = setup_test_contract();
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset mock state and add witness
        test_runtime::reset();
        test_runtime::add_witness(from);
        
        // Act
        contract.transfer(from, to, 100, vec![]);
        
        // Assert
        let events = test_runtime::get_events();
        assert!(!events.is_empty());
        
        // Check event name
        let (event_name, _) = &events[events.len() - 1];
        assert_eq!(event_name, "Transfer");
    }
    
    // Owner Operation Tests
    
    #[test]
    fn test_mint() {
        // Arrange
        let mut contract = setup_test_contract();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let recipient = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset mock state and add owner witness
        test_runtime::reset();
        test_runtime::add_witness(owner);
        
        // Initial supply
        let initial_supply = contract.total_supply();
        
        // Act
        let result = contract.mint(recipient, 500);
        
        // Assert
        assert!(result);
        assert_eq!(contract.total_supply(), initial_supply + 500);
        assert_eq!(contract.balance_of(recipient), 500);
    }
    
    #[test]
    fn test_burn() {
        // Arrange
        let mut contract = setup_test_contract();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        
        // Reset mock state and add owner witness
        test_runtime::reset();
        test_runtime::add_witness(owner);
        
        // Initial supply and balance
        let initial_supply = contract.total_supply();
        let initial_balance = contract.balance_of(owner);
        
        // Act
        let result = contract.burn(owner, 300);
        
        // Assert
        assert!(result);
        assert_eq!(contract.total_supply(), initial_supply - 300);
        assert_eq!(contract.balance_of(owner), initial_balance - 300);
    }
    
    #[test]
    fn test_update_owner() {
        // Arrange
        let mut contract = setup_test_contract();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let new_owner = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset mock state and add owner witness
        test_runtime::reset();
        test_runtime::add_witness(owner);
        
        // Act
        let result = contract.update_owner(new_owner);
        
        // Assert
        assert!(result);
        
        // Verify the owner was updated - must be indirectly tested by trying mint
        test_runtime::reset();
        test_runtime::add_witness(new_owner);
        let mint_result = contract.mint(new_owner, 500);
        assert!(mint_result);
    }
    
    // Edge Case Tests
    
    #[test]
    fn test_transfer_edge_cases() {
        // Arrange
        let mut contract = setup_test_contract();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let user = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Zero amount transfer
        test_runtime::reset();
        test_runtime::add_witness(owner);
        assert!(!contract.transfer(owner, user, 0, vec![]));
        
        // Transfer to self
        test_runtime::reset();
        test_runtime::add_witness(owner);
        assert!(contract.transfer(owner, owner, 100, vec![]));
        assert_eq!(contract.balance_of(owner), 1000); // Unchanged
        
        // Transfer entire balance
        test_runtime::reset();
        test_runtime::add_witness(owner);
        assert!(contract.transfer(owner, user, 1000, vec![]));
        assert_eq!(contract.balance_of(owner), 0);
        assert_eq!(contract.balance_of(user), 1000);
    }
    
    // Annotation Compliance Tests
    
    #[test]
    fn test_event_structure_matches_annotation() {
        // Test that the event structure matches what would be expected from the #[event] annotation
        
        // Arrange
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Reset events
        test_runtime::reset();
        
        // Act
        token_contract::Transfer::emit(Some(from), Some(to), 100);
        
        // Assert
        let events = test_runtime::get_events();
        assert_eq!(events.len(), 1);
        
        let (event_name, args) = &events[0];
        assert_eq!(event_name, "Transfer");
        assert_eq!(args.len(), 3); // Should have 3 parameters
    }
}

// Extension trait for H160 for testing
trait H160Extensions {
    fn from_hex_string(hex: &str) -> Option<H160>;
}

impl H160Extensions for H160 {
    fn from_hex_string(hex: &str) -> Option<H160> {
        let hex = hex.trim_start_matches("0x");
        if hex.len() != 40 {
            return None;
        }
        
        let mut bytes = [0u8; 20];
        for i in 0..20 {
            let byte_str = &hex[i*2..(i+1)*2];
            match u8::from_str_radix(byte_str, 16) {
                Ok(byte) => bytes[i] = byte,
                Err(_) => return None,
            }
        }
        
        Some(H160(bytes))
    }
}

// Extension trait for ByteString for testing
trait ByteStringExtensions {
    fn as_string(&self) -> String;
}

impl ByteStringExtensions for ByteString {
    fn as_string(&self) -> String {
        String::from_utf8_lossy(self.as_bytes()).into_owned()
    }
} 