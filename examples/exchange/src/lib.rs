#![cfg_attr(not(test), no_std)]
#![allow(unused_imports)]

extern crate alloc;

// Import standard types
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// Import everything from prelude
use neo_contract::prelude::*;

// Import core types directly - using paths that match the actual structure
use neo_contract::types::H160;
use neo_contract::types::ByteString;
use neo_contract::types::Array;
use neo_contract::types::Any;
use neo_contract::Runtime;
use neo_contract::storage::Item;
use neo_contract::storage::Map as StorageMap;

/// A simple exchange contract for trading NEP-17 tokens
/// 
/// This demonstrates how to structure an interoperable contract that
/// interacts with other contracts on the Neo blockchain.

// Define call flags for interoperability
const CALL_FLAG_NONE: u8 = 0;
const CALL_FLAG_READ_ONLY: u8 = 1;
const CALL_FLAG_ALLOW_CALL: u8 = 2;
const CALL_FLAG_ALLOW_NOTIFY: u8 = 4;
const CALL_FLAG_ALLOW_ALL: u8 = 7;  // READ_ONLY + ALLOW_CALL + ALLOW_NOTIFY

// Contract module with metadata
// #[contract]
// #[contract_author("NEO Rust Team")]
// #[contract_description("Simple Token Exchange")]
// #[contract_version("0.1.0")]
mod exchange_contract {
    use super::*;
    
    // Event definitions
    // #[event]
    pub struct LiquidityAdded {
        // #[index]
        pub provider: H160,
        pub token_a: H160,
        pub token_b: H160,
        pub amount_a: u64,
        pub amount_b: u64,
    }
    
    // #[event]
    pub struct Swap {
        // #[index]
        pub trader: H160,
        pub token_in: H160,
        pub token_out: H160,
        pub amount_in: u64,
        pub amount_out: u64,
    }
    
    // Manual event implementations
    impl LiquidityAdded {
        pub fn emit(provider: H160, token_a: H160, token_b: H160, amount_a: u64, amount_b: u64) {
            let mut event_args = Array::new();
            event_args.push(Any::from(provider));
            event_args.push(Any::from(token_a));
            event_args.push(Any::from(token_b));
            event_args.push(Any::integer(amount_a as i64));
            event_args.push(Any::integer(amount_b as i64));
            
            Runtime::notify(&ByteString::from("LiquidityAdded"), &event_args);
        }
    }
    
    impl Swap {
        pub fn emit(trader: H160, token_in: H160, token_out: H160, amount_in: u64, amount_out: u64) {
            let mut event_args = Array::new();
            event_args.push(Any::from(trader));
            event_args.push(Any::from(token_in));
            event_args.push(Any::from(token_out));
            event_args.push(Any::integer(amount_in as i64));
            event_args.push(Any::integer(amount_out as i64));
            
            Runtime::notify(&ByteString::from("Swap"), &event_args);
        }
    }
    
    // Contract storage definition
    // #[storage]
    pub struct ExchangeContract {
        pub owner: Item<H160>,
        pub token_pairs: StorageMap<(H160, H160), bool>,
        pub token_reserves: StorageMap<H160, u64>,
        pub fee_percentage: Item<u64>,
        pub liquidity_providers: StorageMap<H160, bool>,
        pub whitelist: StorageMap<H160, bool>,
        pub exchange_rates: StorageMap<(H160, H160), u64>,
        pub in_swap: Item<bool>, // Reentrancy guard
    }
    
    impl ExchangeContract {
        // Constructor for initializing the contract
        // #[constructor]
        pub fn new(owner: H160) -> Self {
            let mut contract = Self {
                owner: Item::new(b"owner"),
                token_pairs: StorageMap::new(b"token_pairs"),
                token_reserves: StorageMap::new(b"token_reserves"),
                fee_percentage: Item::new(b"fee_percentage"),
                liquidity_providers: StorageMap::new(b"liquidity_providers"),
                whitelist: StorageMap::new(b"whitelist"),
                exchange_rates: StorageMap::new(b"exchange_rates"),
                in_swap: Item::new(b"in_swap"),
            };
            
            // Initialize state
            contract.owner.set(&owner).unwrap_or(());
            contract.fee_percentage.set(&30).unwrap_or(());  // 0.3% fee
            contract.in_swap.set(&false).unwrap_or(());
            
            contract
        }
        
        // Admin Methods
        
        // #[method]
        pub fn add_token_pair(&mut self, token_a: H160, token_b: H160) -> bool {
            // Only owner can add token pairs
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Add token pair (both directions)
            self.token_pairs.set(&(token_a, token_b), &true).unwrap_or(());
            self.token_pairs.set(&(token_b, token_a), &true).unwrap_or(());
            
            // Set initial exchange rate 1:1 (10000 = 1.0 in fixed point)
            self.exchange_rates.set(&(token_a, token_b), &10000).unwrap_or(());
            self.exchange_rates.set(&(token_b, token_a), &10000).unwrap_or(());
            
            true
        }
        
        // #[method]
        pub fn add_to_whitelist(&mut self, token: H160) -> bool {
            // Only owner can modify whitelist
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            self.whitelist.set(&token, &true).unwrap_or(());
            true
        }
        
        // #[method]
        pub fn remove_from_whitelist(&mut self, token: H160) -> bool {
            // Only owner can modify whitelist
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            self.whitelist.delete(&token).unwrap_or(());
            true
        }
        
        // #[method]
        pub fn update_fee(&mut self, new_fee: u64) -> bool {
            // Only owner can update fee
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Fee can't be more than 5%
            if new_fee > 500 {
                return false;
            }
            
            self.fee_percentage.set(&new_fee).unwrap_or(());
            true
        }
        
        // Liquidity Methods
        
        // #[method]
        pub fn add_liquidity(
            &mut self, 
            token_a: H160,
            token_b: H160, 
            amount_a: u64,
            amount_b: u64
        ) -> bool {
            // Verify pair is supported
            if !self.is_valid_pair(&token_a, &token_b) {
                return false;
            }
            
            // Check caller authorization
            let provider = Runtime::calling_script_hash();
            if !Runtime::check_witness(&provider) {
                return false;
            }
            
            // Transfer tokens from provider to contract
            if !self.transfer_tokens_from_sender(&token_a, &provider, amount_a) {
                return false;
            }
            
            if !self.transfer_tokens_from_sender(&token_b, &provider, amount_b) {
                // Refund token A if token B transfer fails
                self.transfer_tokens(&token_a, &provider, amount_a);
                return false;
            }
            
            // Update reserves
            self.update_reserve(&token_a, amount_a, true);
            self.update_reserve(&token_b, amount_b, true);
            
            // Update exchange rate based on new liquidity
            self.update_exchange_rate(&token_a, &token_b);
            
            // Add provider to liquidity providers list
            self.liquidity_providers.set(&provider, &true).unwrap_or(());
            
            // Emit event
            LiquidityAdded::emit(provider, token_a, token_b, amount_a, amount_b);
            
            true
        }
        
        // Swap Methods
        
        // #[method]
        // Protects against reentrancy
        pub fn swap(
            &mut self,
            token_in: H160,
            token_out: H160,
            amount_in: u64
        ) -> bool {
            // Check for reentrancy - this would use #[no_reentrant] in the future
            let in_swap_already = self.in_swap.get().unwrap_or(None).unwrap_or(false);
            if in_swap_already {
                return false;
            }
            
            // Set reentrancy guard
            self.in_swap.set(&true).unwrap_or(());
            
            // Verify pair is supported
            if !self.is_valid_pair(&token_in, &token_out) {
                self.in_swap.set(&false).unwrap_or(());
                return false;
            }
            
            // Check caller authorization
            let trader = Runtime::calling_script_hash();
            if !Runtime::check_witness(&trader) {
                self.in_swap.set(&false).unwrap_or(());
                return false;
            }
            
            // Calculate output amount
            let amount_out = self.calculate_output_amount(&token_in, &token_out, amount_in);
            if amount_out == 0 {
                self.in_swap.set(&false).unwrap_or(());
                return false;
            }
            
            // Check if contract has enough reserves
            let out_reserve = self.token_reserves.get(&token_out).unwrap_or(None).unwrap_or(0);
            if out_reserve < amount_out {
                self.in_swap.set(&false).unwrap_or(());
                return false;
            }
            
            // Transfer tokens from trader to contract
            if !self.transfer_tokens_from_sender(&token_in, &trader, amount_in) {
                self.in_swap.set(&false).unwrap_or(());
                return false;
            }
            
            // Update reserves BEFORE external call to prevent reentrancy attack
            self.update_reserve(&token_in, amount_in, true);
            self.update_reserve(&token_out, amount_out, false);
            
            // Transfer tokens from contract to trader
            let transfer_result = self.transfer_tokens(&token_out, &trader, amount_out);
            if !transfer_result {
                // Refund if transfer fails (revert reserve changes)
                self.update_reserve(&token_in, amount_in, false);
                self.update_reserve(&token_out, amount_out, true);
                self.transfer_tokens(&token_in, &trader, amount_in);
                self.in_swap.set(&false).unwrap_or(());
                return false;
            }
            
            // Update exchange rate
            self.update_exchange_rate(&token_in, &token_out);
            
            // Emit swap event
            Swap::emit(trader, token_in, token_out, amount_in, amount_out);
            
            // Clear reentrancy guard
            self.in_swap.set(&false).unwrap_or(());
            
            true
        }
        
        // Read Methods
        
        // #[safe]
        pub fn get_reserves(&self, token: H160) -> u64 {
            self.token_reserves.get(&token).unwrap_or(None).unwrap_or(0)
        }
        
        // #[safe]
        pub fn get_exchange_rate(&self, token_in: H160, token_out: H160) -> u64 {
            self.exchange_rates.get(&(token_in, token_out)).unwrap_or(None).unwrap_or(10000)
        }
        
        // #[safe]
        pub fn is_valid_pair(&self, token_a: &H160, token_b: &H160) -> bool {
            self.token_pairs.get(&(*token_a, *token_b)).unwrap_or(None).unwrap_or(false)
        }
        
        // #[safe]
        pub fn is_whitelisted(&self, token: &H160) -> bool {
            self.whitelist.get(token).unwrap_or(None).unwrap_or(false)
        }
        
        // #[safe]
        pub fn calculate_output_amount(&self, token_in: &H160, token_out: &H160, amount_in: u64) -> u64 {
            // Get exchange rate (fixed point with 4 decimals)
            let rate = self.exchange_rates.get(&(*token_in, *token_out)).unwrap_or(None).unwrap_or(10000);
            
            // Calculate output amount
            let fee = self.fee_percentage.get().unwrap_or(None).unwrap_or(30);
            let fee_amount = (amount_in as u128 * fee as u128) / 10000;
            let amount_after_fee = amount_in as u128 - fee_amount;
            
            // Apply exchange rate
            let amount_out = (amount_after_fee * rate as u128) / 10000;
            
            // Ensure we don't return more than u64::MAX
            if amount_out > u64::MAX as u128 {
                return u64::MAX;
            }
            
            amount_out as u64
        }
        
        // Helper Methods for Contract Interoperability
        
        fn transfer_tokens_from_sender(&self, token: &H160, from: &H160, amount: u64) -> bool {
            let mut args = Array::new();
            args.push(Any::from(*from)); // from
            args.push(Any::from(Runtime::executing_script_hash())); // to (this contract)
            args.push(Any::integer(amount as i64)); // amount
            args.push(Any::null()); // data
            
            // Call the NEP-17 transfer method on the token contract
            let result = Runtime::call_contract(
                token,
                "transfer",
                &args,
                CALL_FLAG_ALLOW_CALL | CALL_FLAG_ALLOW_NOTIFY
            );
            
            // Check result
            result.as_bool().unwrap_or(false)
        }
        
        fn transfer_tokens(&self, token: &H160, to: &H160, amount: u64) -> bool {
            let mut args = Array::new();
            args.push(Any::from(Runtime::executing_script_hash())); // from (this contract)
            args.push(Any::from(*to)); // to 
            args.push(Any::integer(amount as i64)); // amount
            args.push(Any::null()); // data
            
            // Call the NEP-17 transfer method on the token contract
            let result = Runtime::call_contract(
                token,
                "transfer",
                &args,
                CALL_FLAG_ALLOW_CALL | CALL_FLAG_ALLOW_NOTIFY
            );
            
            // Check result
            result.as_bool().unwrap_or(false)
        }
        
        fn update_reserve(&mut self, token: &H160, amount: u64, is_add: bool) {
            let current = self.token_reserves.get(token).unwrap_or(None).unwrap_or(0);
            let new_amount = if is_add {
                current + amount
            } else {
                if current >= amount {
                    current - amount
                } else {
                    0
                }
            };
            
            self.token_reserves.set(token, &new_amount).unwrap_or(());
        }
        
        fn update_exchange_rate(&mut self, token_a: &H160, token_b: &H160) {
            let reserve_a = self.token_reserves.get(token_a).unwrap_or(None).unwrap_or(1);
            let reserve_b = self.token_reserves.get(token_b).unwrap_or(None).unwrap_or(1);
            
            // Calculate new rate: (reserve_b / reserve_a) * 10000 (fixed point)
            let new_rate_a_to_b = if reserve_a > 0 {
                ((reserve_b as u128) * 10000) / (reserve_a as u128)
            } else {
                10000
            };
            
            let new_rate_b_to_a = if reserve_b > 0 {
                ((reserve_a as u128) * 10000) / (reserve_b as u128)
            } else {
                10000
            };
            
            // Ensure rates are within u64 range
            let rate_a_to_b = if new_rate_a_to_b > u64::MAX as u128 {
                u64::MAX
            } else {
                new_rate_a_to_b as u64
            };
            
            let rate_b_to_a = if new_rate_b_to_a > u64::MAX as u128 {
                u64::MAX
            } else {
                new_rate_b_to_a as u64
            };
            
            // Update rates
            self.exchange_rates.set(&(*token_a, *token_b), &rate_a_to_b).unwrap_or(());
            self.exchange_rates.set(&(*token_b, *token_a), &rate_b_to_a).unwrap_or(());
        }
        
        // NEP-17 Handler for receiving token transfers
        
        // #[method]
        pub fn onNEP17Payment(&mut self, from: H160, amount: i64, data: Vec<u8>) -> bool {
            // Ensure amount is positive
            if amount <= 0 {
                return false;
            }
            
            // Get token contract that is sending tokens
            let token = Runtime::calling_script_hash();
            
            // Check if token is whitelisted
            if !self.is_whitelisted(&token) {
                return false;
            }
            
            // Update reserves for the token
            self.update_reserve(&token, amount as u64, true);
            
            // Mark sender as liquidity provider (could parse data to determine action)
            self.liquidity_providers.set(&from, &true).unwrap_or(());
            
            true
        }
    }
}

// Entry points for Neo VM
#[no_mangle]
pub fn deploying() -> bool {
    // Initialize exchange contract during deployment
    let owner = Runtime::calling_script_hash();
    
    // Create exchange contract
    let _contract = exchange_contract::ExchangeContract::new(owner);
    
    true
}

#[no_mangle]
pub fn invoke(operation: String, args: Vec<Any>) -> Any {
    // Create contract instance
    let mut contract = exchange_contract::ExchangeContract::new(H160::default());
    
    match operation.as_str() {
        // Admin methods
        "addTokenPair" => {
            if args.len() != 2 {
                return Any::boolean(false);
            }
            
            let token_a = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let token_b = if let Some(addr) = args[1].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.add_token_pair(token_a, token_b))
        },
        
        "addToWhitelist" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let token = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.add_to_whitelist(token))
        },
        
        "removeFromWhitelist" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let token = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.remove_from_whitelist(token))
        },
        
        "updateFee" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let fee = if let Some(val) = args[0].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.update_fee(fee))
        },
        
        // Liquidity methods
        "addLiquidity" => {
            if args.len() != 4 {
                return Any::boolean(false);
            }
            
            let token_a = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let token_b = if let Some(addr) = args[1].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let amount_a = if let Some(val) = args[2].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            let amount_b = if let Some(val) = args[3].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.add_liquidity(token_a, token_b, amount_a, amount_b))
        },
        
        // Swap methods
        "swap" => {
            if args.len() != 3 {
                return Any::boolean(false);
            }
            
            let token_in = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let token_out = if let Some(addr) = args[1].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let amount_in = if let Some(val) = args[2].as_i64() {
                val as u64
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.swap(token_in, token_out, amount_in))
        },
        
        // Read methods
        "getReserves" => {
            if args.len() != 1 {
                return Any::integer(0);
            }
            
            let token = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::integer(0);
            };
            
            Any::integer(contract.get_reserves(token) as i64)
        },
        
        "getExchangeRate" => {
            if args.len() != 2 {
                return Any::integer(0);
            }
            
            let token_in = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::integer(0);
            };
            
            let token_out = if let Some(addr) = args[1].as_h160() {
                addr.clone()
            } else {
                return Any::integer(0);
            };
            
            Any::integer(contract.get_exchange_rate(token_in, token_out) as i64)
        },
        
        "isValidPair" => {
            if args.len() != 2 {
                return Any::boolean(false);
            }
            
            let token_a = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let token_b = if let Some(addr) = args[1].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.is_valid_pair(&token_a, &token_b))
        },
        
        "isWhitelisted" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let token = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.is_whitelisted(&token))
        },
        
        // NEP-17 Handler
        "onNEP17Payment" => {
            if args.len() != 3 {
                return Any::boolean(false);
            }
            
            let from = if let Some(addr) = args[0].as_h160() {
                addr.clone()
            } else {
                return Any::boolean(false);
            };
            
            let amount = if let Some(val) = args[1].as_i64() {
                val
            } else {
                return Any::boolean(false);
            };
            
            let data = if let Some(bytes) = args[2].as_byte_array() {
                bytes.to_vec()
            } else {
                Vec::new()
            };
            
            Any::boolean(contract.onNEP17Payment(from, amount, data))
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
        use alloc::collections::BTreeMap;
        use std::cell::RefCell;
        
        thread_local! {
            static STORAGE: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new());
            static EVENTS: RefCell<Vec<(String, Vec<Any>)>> = RefCell::new(Vec::new());
            static WITNESSES: RefCell<Vec<H160>> = RefCell::new(Vec::new());
            static CONTRACT_CALLS: RefCell<BTreeMap<(H160, String), Any>> = RefCell::new(BTreeMap::new());
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
            CONTRACT_CALLS.with(|c| c.borrow_mut().clear());
        }
        
        pub fn mock_contract_call_result(contract: &H160, method: &str, result: Any) {
            CONTRACT_CALLS.with(|c| {
                c.borrow_mut().insert((contract.clone(), method.to_string()), result);
            });
        }
        
        pub fn get_mock_call_result(contract: &H160, method: &str) -> Option<Any> {
            CONTRACT_CALLS.with(|c| {
                c.borrow().get(&(contract.clone(), method.to_string())).cloned()
            })
        }
    }
    
    // Mocks for Runtime global functions
    impl Runtime {
        pub fn check_witness(address: &H160) -> bool {
            test_runtime::mock_check_witness(address)
        }
        
        pub fn notify(name: &ByteString, args: &Array) -> i32 {
            // Convert Array to Vec<Any> for our mock
            let args_vec: Vec<Any> = args.iter().collect();
            test_runtime::mock_notify(name.as_string(), &args_vec);
            0
        }
        
        pub fn call_contract(contract: &H160, method: &str, args: &Array, _flags: u8) -> Any {
            // This is our mock for testing contract calls
            if let Some(result) = test_runtime::get_mock_call_result(contract, method) {
                return result;
            }
            
            // Default behavior for transfer method
            if method == "transfer" {
                return Any::boolean(true);
            }
            
            Any::null()
        }
        
        pub fn calling_script_hash() -> H160 {
            // Default test account
            H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap()
        }
        
        pub fn executing_script_hash() -> H160 {
            // Default contract hash
            H160::from_hex_string("0xabcdef1234567890abcdef1234567890abcdef12").unwrap()
        }
    }
    
    // Helper to create a test contract
    fn setup_test_contract() -> exchange_contract::ExchangeContract {
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        exchange_contract::ExchangeContract::new(owner)
    }
    
    // Create test tokens
    fn get_test_tokens() -> (H160, H160) {
        let token_a = H160::from_hex_string("0x1111111111111111111111111111111111111111").unwrap();
        let token_b = H160::from_hex_string("0x2222222222222222222222222222222222222222").unwrap();
        (token_a, token_b)
    }
    
    #[test]
    fn test_add_token_pair() {
        // Arrange
        let mut contract = setup_test_contract();
        let (token_a, token_b) = get_test_tokens();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        
        // Reset mock state and add owner witness
        test_runtime::reset();
        test_runtime::add_witness(owner);
        
        // Act
        let result = contract.add_token_pair(token_a, token_b);
        
        // Assert
        assert!(result);
        assert!(contract.is_valid_pair(&token_a, &token_b));
        assert!(contract.is_valid_pair(&token_b, &token_a));
    }
    
    #[test]
    fn test_swap_tokens() {
        // Arrange
        let mut contract = setup_test_contract();
        let (token_a, token_b) = get_test_tokens();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let trader = H160::from_hex_string("0x3333333333333333333333333333333333333333").unwrap();
        
        // Reset mock state and add witnesses
        test_runtime::reset();
        test_runtime::add_witness(owner);
        
        // Setup: Add token pair and whitelist tokens
        contract.add_token_pair(token_a, token_b);
        contract.add_to_whitelist(token_a);
        contract.add_to_whitelist(token_b);
        
        // Setup: Add initial liquidity (as owner)
        contract.add_liquidity(token_a, token_b, 10000, 10000);
        
        // Reset witnesses for trader
        test_runtime::reset();
        test_runtime::add_witness(trader);
        
        // Act: Perform swap
        // Mock token transfer to return true
        test_runtime::mock_contract_call_result(&token_a, "transfer", Any::boolean(true));
        test_runtime::mock_contract_call_result(&token_b, "transfer", Any::boolean(true));
        
        let result = contract.swap(token_a, token_b, 100);
        
        // Assert
        assert!(result);
        
        // Check events
        let events = test_runtime::get_events();
        assert!(!events.is_empty());
        
        // Get the swap event
        let swap_events: Vec<_> = events.iter()
            .filter(|(name, _)| name == "Swap")
            .collect();
        
        assert!(!swap_events.is_empty());
    }
    
    #[test]
    fn test_onNEP17Payment() {
        // Arrange
        let mut contract = setup_test_contract();
        let (token_a, _) = get_test_tokens();
        let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let user = H160::from_hex_string("0x3333333333333333333333333333333333333333").unwrap();
        
        // Reset mock state and add witnesses
        test_runtime::reset();
        test_runtime::add_witness(owner);
        
        // Setup: Whitelist token A
        contract.add_to_whitelist(token_a);
        
        // Mock Runtime::calling_script_hash to return token A
        impl Runtime {
            pub fn calling_script_hash() -> H160 {
                H160::from_hex_string("0x1111111111111111111111111111111111111111").unwrap()
            }
        }
        
        // Act
        let result = contract.onNEP17Payment(user, 100, Vec::new());
        
        // Assert
        assert!(result);
        
        // Check if reserves were updated
        assert_eq!(contract.get_reserves(token_a), 100);
    }
} 