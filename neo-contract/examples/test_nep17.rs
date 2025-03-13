#![no_std]
#![cfg_attr(target_arch = "wasm32", feature(core_intrinsics, lang_items))]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::ToString;
use neo_contract::prelude::*;
use neo_contract::storage::test_utils::*;

/// Test file for NEP-17 token implementation
/// This demonstrates how to test a Neo N3 fungible token contract
#[cfg(test)]
mod tests {
    use super::*;

    // Import the module to test (from nep17_token.rs)
    // In a real test, you would import your actual contract code
    mod nep17_token {
        use super::*;

        // Define events for NEP-17 standard following ink! style
        pub struct Transfer {
            #[index]
            from: Option<H160>,
            #[index]
            to: Option<H160>,
            amount: Int256,
        }

        // Storage keys
        const OWNER_KEY: &[u8] = b"owner";
        const NAME_KEY: &[u8] = b"name";
        const SYMBOL_KEY: &[u8] = b"symbol";
        const DECIMALS_KEY: &[u8] = b"decimals";
        const TOTAL_SUPPLY_KEY: &[u8] = b"total_supply";
        const BALANCE_PREFIX: &[u8] = b"balance:";

        #[storage]
        struct TokenContract {
            // Contract metadata
            owner: StorageItem<H160>,
            name: StorageItem<String>,
            symbol: StorageItem<String>,
            decimals: StorageItem<u8>,

            // Token data
            total_supply: StorageItem<Int256>,
            balances: StorageMap<H160, Int256>,
        }

        impl TokenContract {
            /// Initialize a new token contract
            #[constructor]
            pub fn new(
                owner: H160,
                name: String,
                symbol: String,
                decimals: u8,
                initial_supply: Int256,
                initial_holder: H160,
            ) -> Self {
                let mut contract = Self {
                    owner: StorageItem::new(OWNER_KEY).with_data(owner),
                    name: StorageItem::new(NAME_KEY).with_data(name),
                    symbol: StorageItem::new(SYMBOL_KEY).with_data(symbol),
                    decimals: StorageItem::new(DECIMALS_KEY).with_data(decimals),
                    total_supply: StorageItem::new(TOTAL_SUPPLY_KEY).with_data(Int256::zero()),
                    balances: StorageMap::new(BALANCE_PREFIX),
                };

                // Mint initial supply to initial holder if specified
                if initial_supply > Int256::zero() && initial_holder != H160::zero() {
                    contract.mint(initial_holder, initial_supply);
                }

                contract
            }

            /// Get contract owner
            #[safe]
            pub fn get_owner(&self) -> H160 { self.owner.get().unwrap_or_default() }

            /// Update contract owner
            #[method]
            pub fn update_owner(&mut self, new_owner: H160) -> bool {
                // Check if the caller is the current owner
                let current_owner = self.get_owner();
                if !Runtime::check_witness(&current_owner) {
                    return false;
                }

                self.owner.set(new_owner);
                true
            }

            /// Get token name
            #[safe]
            pub fn name(&self) -> String { self.name.get().unwrap_or_else(|| "NEP17Token".to_string()) }

            /// Get token symbol
            #[safe]
            pub fn symbol(&self) -> String { self.symbol.get().unwrap_or_else(|| "NEP17".to_string()) }

            /// Get token decimals
            #[safe]
            pub fn decimals(&self) -> u8 { self.decimals.get().unwrap_or(8) }

            /// Get total supply
            #[safe]
            pub fn total_supply(&self) -> Int256 { self.total_supply.get().unwrap_or_else(Int256::zero) }

            /// Get balance of address
            #[safe]
            pub fn balance_of(&self, owner: H160) -> Int256 { self.balances.get(&owner).unwrap_or_else(Int256::zero) }

            /// Transfer tokens
            #[method]
            pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: Option<ByteString>) -> bool {
                // Validate parameters
                if amount <= Int256::zero() {
                    return false;
                }

                if to == H160::zero() {
                    return false;
                }

                // Check if the sender is authorized
                if !Runtime::check_witness(&from) {
                    return false;
                }

                // Check if the sender has enough balance
                let from_balance = self.balance_of(from);
                if from_balance < amount {
                    return false;
                }

                // Update balances
                let new_from_balance = from_balance - amount;
                if new_from_balance.is_zero() {
                    self.balances.delete(&from);
                } else {
                    self.balances.insert(&from, &new_from_balance);
                }

                let to_balance = self.balance_of(to);
                let new_to_balance = to_balance + amount;
                self.balances.insert(&to, &new_to_balance);

                // Emit transfer event
                self.emit_transfer(Some(from), Some(to), amount);

                // Call onNEP17Payment for contracts if data provided
                if data.is_some() && Storage::is_contract(&to) {
                    // In a real implementation, would call onNEP17Payment
                }

                true
            }

            /// Mint new tokens
            #[method]
            pub fn mint(&mut self, to: H160, amount: Int256) -> bool {
                // Check if caller is the contract owner
                let owner = self.get_owner();
                if !Runtime::check_witness(&owner) {
                    return false;
                }

                // Validate parameters
                if amount <= Int256::zero() {
                    return false;
                }

                if to == H160::zero() {
                    return false;
                }

                // Update balance
                let to_balance = self.balance_of(to);
                let new_to_balance = to_balance + amount;
                self.balances.insert(&to, &new_to_balance);

                // Update total supply
                let current_supply = self.total_supply();
                let new_supply = current_supply + amount;
                self.total_supply.set(new_supply);

                // Emit transfer event
                self.emit_transfer(None, Some(to), amount);

                true
            }

            /// Burn tokens
            #[method]
            pub fn burn(&mut self, from: H160, amount: Int256) -> bool {
                // Validate parameters
                if amount <= Int256::zero() {
                    return false;
                }

                // Check if the caller is authorized
                if !Runtime::check_witness(&from) {
                    return false;
                }

                // Check if the caller has enough balance
                let from_balance = self.balance_of(from);
                if from_balance < amount {
                    return false;
                }

                // Update balance
                let new_from_balance = from_balance - amount;
                if new_from_balance.is_zero() {
                    self.balances.delete(&from);
                } else {
                    self.balances.insert(&from, &new_from_balance);
                }

                // Update total supply
                let current_supply = self.total_supply();
                let new_supply = current_supply - amount;
                self.total_supply.set(new_supply);

                // Emit transfer event
                self.emit_transfer(Some(from), None, amount);

                true
            }

            /// Emit a Transfer event
            #[safe]
            fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: Int256) {
                // Create event name as ByteString
                let event_name = ByteString::from("Transfer");

                // Create an Array to hold parameters
                let mut event_data = Array::<Any>::new();

                // Add parameters as Any values
                match from {
                    Some(addr) => event_data.push(Any::from(addr)),
                    None => event_data.push(Any::new()),
                }

                match to {
                    Some(addr) => event_data.push(Any::from(addr)),
                    None => event_data.push(Any::new()),
                }

                event_data.push(Any::from(amount));

                // Emit the event
                Runtime::notify(&event_name, &event_data);
            }
        }
    }

    // Create mock storage for testing
    fn setup_mock_storage() -> MockStorage {
        let mut storage = MockStorage::new();

        // Add mock Runtime::check_witness to always return true for testing
        MockRuntime::mock_check_witness(&mut storage, true);

        // Optional: Mock other runtime behavior as needed

        storage
    }

    // Test cases

    #[test]
    fn test_initialization() {
        // Setup mock storage
        let storage = setup_mock_storage();

        // Create token contract
        let owner = H160::from([1u8; 20]);
        let name = "Sample Token".to_string();
        let symbol = "SMPL".to_string();
        let decimals = 8u8;
        let initial_supply = Int256::from(1_000_000);
        let initial_holder = owner.clone();

        let contract = nep17_token::TokenContract::new(
            owner,
            name.clone(),
            symbol.clone(),
            decimals,
            initial_supply.clone(),
            initial_holder,
        );

        // Verify contract state
        assert_eq!(contract.name(), name);
        assert_eq!(contract.symbol(), symbol);
        assert_eq!(contract.decimals(), decimals);
        assert_eq!(contract.total_supply(), initial_supply);
        assert_eq!(contract.balance_of(initial_holder), initial_supply);
    }

    #[test]
    fn test_transfer() {
        // Setup mock storage
        let storage = setup_mock_storage();

        // Create token contract with initial supply
        let owner = H160::from([1u8; 20]);
        let recipient = H160::from([2u8; 20]);
        let initial_supply = Int256::from(1_000_000);
        let transfer_amount = Int256::from(100_000);

        let mut contract = nep17_token::TokenContract::new(
            owner.clone(),
            "Test Token".to_string(),
            "TEST".to_string(),
            8u8,
            initial_supply.clone(),
            owner.clone(),
        );

        // Test transfer
        let result = contract.transfer(owner.clone(), recipient.clone(), transfer_amount.clone(), None);
        assert!(result, "Transfer should succeed");

        // Verify balances after transfer
        assert_eq!(contract.balance_of(owner), initial_supply - transfer_amount.clone());
        assert_eq!(contract.balance_of(recipient), transfer_amount);
        assert_eq!(contract.total_supply(), initial_supply);
    }

    #[test]
    fn test_mint() {
        // Setup mock storage
        let storage = setup_mock_storage();

        // Create token contract with no initial supply
        let owner = H160::from([1u8; 20]);
        let recipient = H160::from([2u8; 20]);
        let mint_amount = Int256::from(500_000);

        let mut contract = nep17_token::TokenContract::new(
            owner.clone(),
            "Test Token".to_string(),
            "TEST".to_string(),
            8u8,
            Int256::zero(),
            H160::zero(),
        );

        // Test minting
        let result = contract.mint(recipient.clone(), mint_amount.clone());
        assert!(result, "Minting should succeed");

        // Verify state after minting
        assert_eq!(contract.total_supply(), mint_amount);
        assert_eq!(contract.balance_of(recipient), mint_amount);
    }

    #[test]
    fn test_burn() {
        // Setup mock storage
        let storage = setup_mock_storage();

        // Create token contract with initial supply
        let owner = H160::from([1u8; 20]);
        let initial_supply = Int256::from(1_000_000);
        let burn_amount = Int256::from(250_000);

        let mut contract = nep17_token::TokenContract::new(
            owner.clone(),
            "Test Token".to_string(),
            "TEST".to_string(),
            8u8,
            initial_supply.clone(),
            owner.clone(),
        );

        // Test burning
        let result = contract.burn(owner.clone(), burn_amount.clone());
        assert!(result, "Burning should succeed");

        // Verify state after burning
        assert_eq!(contract.total_supply(), initial_supply - burn_amount);
        assert_eq!(contract.balance_of(owner), initial_supply - burn_amount);
    }

    #[test]
    fn test_update_owner() {
        // Setup mock storage
        let storage = setup_mock_storage();

        // Create token contract
        let original_owner = H160::from([1u8; 20]);
        let new_owner = H160::from([2u8; 20]);

        let mut contract = nep17_token::TokenContract::new(
            original_owner.clone(),
            "Test Token".to_string(),
            "TEST".to_string(),
            8u8,
            Int256::zero(),
            H160::zero(),
        );

        // Test owner update
        assert_eq!(contract.get_owner(), original_owner);

        let result = contract.update_owner(new_owner.clone());
        assert!(result, "Owner update should succeed");

        assert_eq!(contract.get_owner(), new_owner);
    }

    #[test]
    fn test_failed_transfer() {
        // Setup mock storage
        let storage = setup_mock_storage();

        // Create token contract with initial supply
        let owner = H160::from([1u8; 20]);
        let recipient = H160::from([2u8; 20]);
        let initial_supply = Int256::from(1_000_000);
        let excessive_amount = Int256::from(2_000_000); // More than total supply

        let mut contract = nep17_token::TokenContract::new(
            owner.clone(),
            "Test Token".to_string(),
            "TEST".to_string(),
            8u8,
            initial_supply.clone(),
            owner.clone(),
        );

        // Test invalid transfer (insufficient balance)
        let result = contract.transfer(owner.clone(), recipient.clone(), excessive_amount.clone(), None);
        assert!(!result, "Transfer with insufficient balance should fail");

        // Test invalid transfer (zero address recipient)
        let result = contract.transfer(owner.clone(), H160::zero(), initial_supply.clone(), None);
        assert!(!result, "Transfer to zero address should fail");

        // Test invalid transfer (zero amount)
        let result = contract.transfer(owner.clone(), recipient.clone(), Int256::zero(), None);
        assert!(!result, "Transfer of zero amount should fail");

        // Verify state remains unchanged
        assert_eq!(contract.balance_of(owner), initial_supply);
        assert_eq!(contract.balance_of(recipient), Int256::zero());
        assert_eq!(contract.total_supply(), initial_supply);
    }
}
