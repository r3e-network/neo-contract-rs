#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for WASM no_std builds
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// Simple NEP-11 NFT Implementation
pub struct NEP11NFT {
    // Storage keys for metadata
    name_key: ByteString,
    symbol_key: ByteString,
    total_supply_key: ByteString,
    owner_key: ByteString,
    initialized_key: ByteString,
}

#[contract_impl]
impl NEP11NFT {
    pub fn init() -> Self {
        Self {
            name_key: ByteString::from_literal("name"),
            symbol_key: ByteString::from_literal("symbol"),
            total_supply_key: ByteString::from_literal("total_supply"),
            owner_key: ByteString::from_literal("owner"),
            initialized_key: ByteString::from_literal("initialized"),
        }
    }

    #[method]
    pub fn initialize(&self, name: ByteString, symbol: ByteString) -> bool {
        let context = Storage::get_context();
        
        // Check if already initialized
        if let Some(_) = Storage::get(context.clone(), self.initialized_key.clone()) {
            Runtime::log(ByteString::from_literal("Already initialized"));
            return false;
        }

        let owner = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        // Store metadata in storage
        Storage::put(context.clone(), self.name_key.clone(), name.clone());
        Storage::put(context.clone(), self.symbol_key.clone(), symbol.clone());
        Storage::put(context.clone(), self.total_supply_key.clone(), Int256::zero().into_byte_string());
        Storage::put(context.clone(), self.owner_key.clone(), owner.into_byte_string());
        Storage::put(context, self.initialized_key.clone(), ByteString::from_literal("true"));

        Runtime::log(ByteString::from_literal("NFT initialized"));
        true
    }

    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        let context = Storage::get_context();
        Storage::get(context, self.symbol_key.clone()).unwrap_or(ByteString::from_literal("UNKNOWN"))
    }

    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        0  // NFTs have 0 decimals
    }

    #[method]
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        let context = Storage::get_context();
        if let Some(supply_bytes) = Storage::get(context, self.total_supply_key.clone()) {
            Int256::from_byte_string(supply_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn balance_of(&self, owner: H160) -> Int256 {
        let context = Storage::get_context();
        let balance_key = self.get_balance_key(owner);
        
        match Storage::get(context, balance_key) {
            Some(balance) => Int256::from_byte_string(balance),
            None => Int256::zero(),
        }
    }

    #[method]
    #[safe]
    pub fn tokens_of(&self, owner: H160) -> Array<ByteString> {
        let mut tokens = Array::new();
        let context = Storage::get_context();
        let prefix = self.get_owner_token_prefix(owner);
        
        // Simple implementation - just return empty array for now
        // In a full implementation, this would iterate through storage
        tokens
    }

    #[method]
    #[safe]
    pub fn owner_of(&self, token_id: ByteString) -> H160 {
        let context = Storage::get_context();
        let owner_key = self.get_token_owner_key(token_id);
        
        match Storage::get(context, owner_key) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    #[method]
    pub fn mint(&self, to: H160, token_id: ByteString) -> bool {
        let context = Storage::get_context();
        let contract_owner = if let Some(owner_bytes) = Storage::get(context.clone(), self.owner_key.clone()) {
            H160::from_byte_string(owner_bytes)
        } else {
            H160::zero()
        };
        
        if !Runtime::check_witness(contract_owner) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        // Check if token already exists
        let owner_key = self.get_token_owner_key(token_id.clone());
        if let Some(_) = Storage::get(context.clone(), owner_key.clone()) {
            Runtime::log(ByteString::from_literal("Token already exists"));
            return false;
        }

        // Set token owner
        Storage::put(context.clone(), owner_key, to.into_byte_string());

        // Update balance
        let balance_key = self.get_balance_key(to);
        let balance = self.balance_of(to);
        Storage::put(context.clone(), balance_key, (balance + Int256::from(1)).into_byte_string());

        // Update total supply
        let total_supply = self.total_supply();
        Storage::put(context.clone(), self.total_supply_key.clone(), (total_supply + Int256::from(1)).into_byte_string());

        // Emit transfer event
        let mut args = Array::new();
        args.push(H160::zero().into_any());
        args.push(to.into_any());
        args.push(Int256::from(1).into_any());
        args.push(token_id.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        Runtime::log(ByteString::from_literal("NFT minted"));
        true
    }

    #[method]
    pub fn burn(&self, token_id: ByteString) -> bool {
        let owner = self.owner_of(token_id.clone());
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let context = Storage::get_context();
        
        // Remove token owner
        let owner_key = self.get_token_owner_key(token_id.clone());
        Storage::delete(context.clone(), owner_key);

        // Update balance
        let balance_key = self.get_balance_key(owner);
        let balance = self.balance_of(owner);
        if balance > Int256::zero() {
            Storage::put(context.clone(), balance_key, (balance - Int256::from(1)).into_byte_string());
        }

        // Update total supply
        let total_supply = self.total_supply();
        if total_supply > Int256::zero() {
            Storage::put(context.clone(), self.total_supply_key.clone(), (total_supply - Int256::from(1)).into_byte_string());
        }

        // Emit transfer event
        let mut args = Array::new();
        args.push(owner.into_any());
        args.push(H160::zero().into_any());
        args.push(Int256::from(1).into_any());
        args.push(token_id.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        Runtime::log(ByteString::from_literal("NFT burned"));
        true
    }

    #[method]
    pub fn transfer(&self, to: H160, token_id: ByteString, data: Any) -> bool {
        let owner = self.owner_of(token_id.clone());
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if owner == to {
            Runtime::log(ByteString::from_literal("Same owner"));
            return true;
        }

        let context = Storage::get_context();
        
        // Update token owner
        let owner_key = self.get_token_owner_key(token_id.clone());
        Storage::put(context.clone(), owner_key, to.into_byte_string());

        // Update old owner balance
        let old_balance_key = self.get_balance_key(owner);
        let old_balance = self.balance_of(owner);
        if old_balance > Int256::zero() {
            Storage::put(context.clone(), old_balance_key, (old_balance - Int256::from(1)).into_byte_string());
        }

        // Update new owner balance
        let new_balance_key = self.get_balance_key(to);
        let new_balance = self.balance_of(to);
        Storage::put(context, new_balance_key, (new_balance + Int256::from(1)).into_byte_string());

        // Emit transfer event
        let mut args = Array::new();
        args.push(owner.into_any());
        args.push(to.into_any());
        args.push(Int256::from(1).into_any());
        args.push(token_id.clone().into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        // Call onNEP11Payment if needed
        self.post_transfer(owner, to, Int256::from(1), token_id, data);

        Runtime::log(ByteString::from_literal("NFT transferred"));
        true
    }

    #[method]
    #[safe]
    pub fn properties(&self, token_id: ByteString) -> Map<ByteString, ByteString> {
        // Simple implementation - return empty map
        // In a full implementation, this would return token properties
        Map::new()
    }

    // Helper methods
    fn get_balance_key(&self, account: H160) -> ByteString {
        let key = ByteString::from_literal("balance");
        key.concat(&account.into_byte_string())
    }

    fn get_token_owner_key(&self, token_id: ByteString) -> ByteString {
        let key = ByteString::from_literal("owner");
        key.concat(&token_id)
    }

    fn get_owner_token_prefix(&self, owner: H160) -> ByteString {
        let prefix = ByteString::from_literal("tokens");
        prefix.concat(&owner.into_byte_string())
    }

    fn post_transfer(&self, from: H160, to: H160, amount: Int256, token_id: ByteString, data: Any) {
        // Try to call onNEP11Payment on the 'to' contract
        let mut call_args = Array::new();
        call_args.push(from.into_any());
        call_args.push(amount.into_any());
        call_args.push(token_id.into_any());
        call_args.push(data);
        
        let _ = Contract::call(
            to,
            ByteString::from_literal("onNEP11Payment"),
            CallFlags::All,
            call_args,
        );
    }
}