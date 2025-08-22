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

// NEP-17 Token Implementation - Solana Style (Converted to Neo N3)
pub struct NEP17TokenSolanaStyle {
    // Storage keys for metadata
    name_key: ByteString,
    symbol_key: ByteString,
    decimals_key: ByteString,
    total_supply_key: ByteString,
    owner_key: ByteString,
    initialized_key: ByteString,
}

#[contract_impl]
impl NEP17TokenSolanaStyle {
    pub fn init() -> Self {
        Self {
            name_key: ByteString::from_literal("name"),
            symbol_key: ByteString::from_literal("symbol"),
            decimals_key: ByteString::from_literal("decimals"),
            total_supply_key: ByteString::from_literal("total_supply"),
            owner_key: ByteString::from_literal("owner"),
            initialized_key: ByteString::from_literal("initialized"),
        }
    }

    #[method]
    pub fn initialize(
        &self,
        name: ByteString,
        symbol: ByteString,
        decimals: u8,
        total_supply: Int256,
    ) -> bool {
        let context = Storage::get_context();
        
        // Check if already initialized
        if let Some(_) = Storage::get(context.clone(), self.initialized_key.clone()) {
            Runtime::log(ByteString::from_literal("Already initialized"));
            return false;
        }

        if total_supply <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid supply"));
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
        Storage::put(context.clone(), self.decimals_key.clone(), ByteString::from_bytes(&[decimals]));
        Storage::put(context.clone(), self.total_supply_key.clone(), total_supply.into_byte_string());
        Storage::put(context.clone(), self.owner_key.clone(), owner.into_byte_string());
        Storage::put(context.clone(), self.initialized_key.clone(), ByteString::from_literal("true"));

        // Set owner balance
        let balance_key = self.get_balance_key(owner);
        Storage::put(context, balance_key, total_supply.into_byte_string());

        // Emit transfer event
        let mut args = Array::new();
        args.push(H160::zero().into_any());
        args.push(owner.into_any());
        args.push(total_supply.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        Runtime::log(ByteString::from_literal("Token initialized"));
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
        let context = Storage::get_context();
        if let Some(decimals_bytes) = Storage::get(context, self.decimals_key.clone()) {
            let bytes = decimals_bytes.to_bytes();
            if !bytes.is_empty() {
                bytes[0]
            } else {
                8
            }
        } else {
            8
        }
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
    pub fn balance_of(&self, account: H160) -> Int256 {
        let context = Storage::get_context();
        let balance_key = self.get_balance_key(account);
        
        match Storage::get(context, balance_key) {
            Some(balance) => Int256::from_byte_string(balance),
            None => Int256::zero(),
        }
    }

    #[method]
    pub fn transfer(&self, from: H160, to: H160, amount: Int256, data: Any) -> bool {
        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        if from == to {
            Runtime::log(ByteString::from_literal("Same from and to"));
            return true;
        }

        if !Runtime::check_witness(from) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let context = Storage::get_context();
        let from_balance = self.balance_of(from);
        
        if from_balance < amount {
            Runtime::log(ByteString::from_literal("Insufficient balance"));
            return false;
        }

        let to_balance = self.balance_of(to);

        // Update balances
        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance + amount;

        let from_key = self.get_balance_key(from);
        let to_key = self.get_balance_key(to);

        if new_from_balance == Int256::zero() {
            Storage::delete(context.clone(), from_key);
        } else {
            Storage::put(context.clone(), from_key, new_from_balance.into_byte_string());
        }

        Storage::put(context, to_key, new_to_balance.into_byte_string());

        // Emit transfer event
        let mut args = Array::new();
        args.push(from.into_any());
        args.push(to.into_any());
        args.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        // Post transfer
        self.post_transfer(from, to, amount, data, true);
        true
    }

    #[method]
    #[safe]
    pub fn allowance(&self, owner: H160, spender: H160) -> Int256 {
        let context = Storage::get_context();
        let allowance_key = self.get_allowance_key(owner, spender);
        
        match Storage::get(context, allowance_key) {
            Some(allowance) => Int256::from_byte_string(allowance),
            None => Int256::zero(),
        }
    }

    #[method]
    pub fn approve(&self, owner: H160, spender: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let context = Storage::get_context();
        let allowance_key = self.get_allowance_key(owner, spender);

        if amount == Int256::zero() {
            Storage::delete(context, allowance_key);
        } else {
            Storage::put(context, allowance_key, amount.into_byte_string());
        }

        // Emit approval event
        let mut args = Array::new();
        args.push(owner.into_any());
        args.push(spender.into_any());
        args.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Approval"), args);

        true
    }

    #[method]
    pub fn transfer_from(&self, spender: H160, from: H160, to: H160, amount: Int256, data: Any) -> bool {
        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        if !Runtime::check_witness(spender) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let allowance = self.allowance(from, spender);
        if allowance < amount {
            Runtime::log(ByteString::from_literal("Insufficient allowance"));
            return false;
        }

        let from_balance = self.balance_of(from);
        if from_balance < amount {
            Runtime::log(ByteString::from_literal("Insufficient balance"));
            return false;
        }

        let context = Storage::get_context();
        let to_balance = self.balance_of(to);

        // Update balances
        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance + amount;
        let new_allowance = allowance - amount;

        let from_key = self.get_balance_key(from);
        let to_key = self.get_balance_key(to);
        let allowance_key = self.get_allowance_key(from, spender);

        if new_from_balance == Int256::zero() {
            Storage::delete(context.clone(), from_key);
        } else {
            Storage::put(context.clone(), from_key, new_from_balance.into_byte_string());
        }

        Storage::put(context.clone(), to_key, new_to_balance.into_byte_string());

        if new_allowance == Int256::zero() {
            Storage::delete(context.clone(), allowance_key);
        } else {
            Storage::put(context, allowance_key, new_allowance.into_byte_string());
        }

        // Emit transfer event
        let mut args = Array::new();
        args.push(from.into_any());
        args.push(to.into_any());
        args.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        // Post transfer
        self.post_transfer(from, to, amount, data, true);
        true
    }

    #[method]
    pub fn mint(&self, to: H160, amount: Int256) -> bool {
        let context = Storage::get_context();
        let owner = if let Some(owner_bytes) = Storage::get(context.clone(), self.owner_key.clone()) {
            H160::from_byte_string(owner_bytes)
        } else {
            H160::zero()
        };
        
        if !Runtime::check_witness(owner) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        let to_balance = self.balance_of(to);
        let new_balance = to_balance + amount;
        let current_total_supply = self.total_supply();
        let new_total_supply = current_total_supply + amount;

        let balance_key = self.get_balance_key(to);
        Storage::put(context.clone(), balance_key, new_balance.into_byte_string());

        // Update total supply
        Storage::put(context.clone(), self.total_supply_key.clone(), new_total_supply.into_byte_string());

        // Emit transfer event
        let mut args = Array::new();
        args.push(H160::zero().into_any());
        args.push(to.into_any());
        args.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        true
    }

    #[method]
    pub fn burn(&self, from: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(from) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        let from_balance = self.balance_of(from);
        if from_balance < amount {
            Runtime::log(ByteString::from_literal("Insufficient balance"));
            return false;
        }

        let context = Storage::get_context();
        let new_balance = from_balance - amount;
        let current_total_supply = self.total_supply();
        let new_total_supply = current_total_supply - amount;

        let balance_key = self.get_balance_key(from);
        
        if new_balance == Int256::zero() {
            Storage::delete(context.clone(), balance_key);
        } else {
            Storage::put(context.clone(), balance_key, new_balance.into_byte_string());
        }

        // Update total supply
        Storage::put(context, self.total_supply_key.clone(), new_total_supply.into_byte_string());

        // Emit transfer event
        let mut args = Array::new();
        args.push(from.into_any());
        args.push(H160::zero().into_any());
        args.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), args);

        true
    }

    // Helper methods
    fn get_balance_key(&self, account: H160) -> ByteString {
        let key = ByteString::from_literal("balance");
        key.concat(&account.into_byte_string())
    }

    fn get_allowance_key(&self, owner: H160, spender: H160) -> ByteString {
        let key = ByteString::from_literal("allowance");
        key.concat(&owner.into_byte_string()).concat(&spender.into_byte_string())
    }

    fn post_transfer(&self, from: H160, to: H160, amount: Int256, data: Any, call_onpayment: bool) {
        if call_onpayment {
            // Try to call onNEP17Payment on the 'to' contract
            let mut call_args = Array::new();
            call_args.push(from.into_any());
            call_args.push(amount.into_any());
            call_args.push(data);
            
            let _ = Contract::call(
                to,
                ByteString::from_literal("onNEP17Payment"),
                CallFlags::All,
                call_args,
            );
        }
    }
}