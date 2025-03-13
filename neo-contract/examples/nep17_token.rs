#![no_std]
#![cfg_attr(target_arch = "wasm32", feature(core_intrinsics, lang_items))]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::{String, ToString};
use neo_contract::prelude::*;

/// NEP-17 Token Standard Implementation
/// This example demonstrates how to implement a fungible token on Neo N3
/// following the NEP-17 standard with ink! style syntax
#[contract]
mod nep17_token {
    use super::*;

    // Define events for NEP-17 standard
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

        //======== Contract Management ========//

        /// Get contract owner - safe method (read-only)
        #[safe]
        pub fn get_owner(&self) -> H160 { self.owner.get().unwrap_or_default() }

        /// Update contract owner - requires witness check
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

        //======== NEP-17 Standard Implementation ========//

        /// Get token name - safe method (read-only)
        #[safe]
        pub fn name(&self) -> String { self.name.get().unwrap_or_else(|| "NEP17Token".to_string()) }

        /// Get token symbol - safe method (read-only)
        #[safe]
        pub fn symbol(&self) -> String { self.symbol.get().unwrap_or_else(|| "NEP17".to_string()) }

        /// Get token decimals - safe method (read-only)
        #[safe]
        pub fn decimals(&self) -> u8 { self.decimals.get().unwrap_or(8) }

        /// Get total supply - safe method (read-only)
        #[safe]
        pub fn total_supply(&self) -> Int256 { self.total_supply.get().unwrap_or_else(Int256::zero) }

        /// Get balance of address - safe method (read-only)
        #[safe]
        pub fn balance_of(&self, owner: H160) -> Int256 { self.balances.get(&owner).unwrap_or_else(Int256::zero) }

        /// Transfer tokens from one address to another
        /// Implements the NEP-17 transfer method
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

            // If the recipient is a contract, call onNEP17Payment
            if data.is_some() && Storage::is_contract(&to) {
                // In a real implementation, we would call the onNEP17Payment method
                // of the receiving contract
                // This is omitted here for simplicity
            }

            true
        }

        //======== Token Management ========//

        /// Mint new tokens - only contract owner can mint
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

            // Emit transfer event (mint is a transfer from null address)
            self.emit_transfer(None, Some(to), amount);

            true
        }

        /// Burn tokens - tokens must be owned by the caller
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

            // Emit transfer event (burn is a transfer to null address)
            self.emit_transfer(Some(from), None, amount);

            true
        }

        //======== Utility Methods ========//

        /// Emit a NEP-17 Transfer event following Neo N3 standards
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

    // Interface implementation for NEP-17 standard
    pub trait NEP17 {
        fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool;
        fn balance_of(&self, owner: H160) -> Int256;
        fn total_supply(&self) -> Int256;
    }

    impl NEP17 for TokenContract {
        #[method]
        fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool { self.transfer(from, to, amount, None) }

        #[safe]
        fn balance_of(&self, owner: H160) -> Int256 { self.balance_of(owner) }

        #[safe]
        fn total_supply(&self) -> Int256 { self.total_supply() }
    }
}

// Implement the necessary methods for deployment
#[cfg(target_arch = "wasm32")]
mod deployment {
    use super::*;

    // Needed for no_std compatibility with wasm32 target
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! { core::intrinsics::abort() }

    #[alloc_error_handler]
    fn oom(_: core::alloc::Layout) -> ! { core::intrinsics::abort() }

    #[no_mangle]
    pub extern "C" fn _start() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}
