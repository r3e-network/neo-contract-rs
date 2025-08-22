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
// Simple Crowdfunding Implementation
pub struct Crowdfunding {
    // Storage keys for metadata
    creator_key: ByteString,
    target_amount_key: ByteString,
    current_amount_key: ByteString,
    deadline_key: ByteString,
    active_key: ByteString,
    initialized_key: ByteString,
}

#[contract]
impl Crowdfunding {
    pub fn init() -> Self {
        Self {
            creator_key: ByteString::from_literal("creator"),
            target_amount_key: ByteString::from_literal("target_amount"),
            current_amount_key: ByteString::from_literal("current_amount"),
            deadline_key: ByteString::from_literal("deadline"),
            active_key: ByteString::from_literal("active"),
            initialized_key: ByteString::from_literal("initialized"),
        }
    }

    #[method]
    pub fn initialize(&self, target_amount: Int256, duration_blocks: Int256) -> bool {
        let context = Storage::get_context();
        
        // Check if already initialized
        if let Some(_) = Storage::get(context.clone(), self.initialized_key.clone()) {
            Runtime::log(ByteString::from_literal("Already initialized"));
            return false;
        }

        let creator = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(creator) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if target_amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid target amount"));
            return false;
        }

        // Use current timestamp instead of block height
        let current_time = Int256::from(Runtime::get_time() as i64);
        let deadline = current_time + duration_blocks;

        // Store campaign data in storage
        Storage::put(context.clone(), self.creator_key.clone(), creator.into_byte_string());
        Storage::put(context.clone(), self.target_amount_key.clone(), target_amount.into_byte_string());
        Storage::put(context.clone(), self.current_amount_key.clone(), Int256::zero().into_byte_string());
        Storage::put(context.clone(), self.deadline_key.clone(), deadline.into_byte_string());
        Storage::put(context.clone(), self.active_key.clone(), ByteString::from_literal("true"));
        Storage::put(context, self.initialized_key.clone(), ByteString::from_literal("true"));

        Runtime::log(ByteString::from_literal("Crowdfunding campaign initialized"));
        true
    }

    #[method]
    #[safe]
    pub fn get_creator(&self) -> H160 {
        let context = Storage::get_context();
        if let Some(creator_bytes) = Storage::get(context, self.creator_key.clone()) {
            H160::from_byte_string(creator_bytes)
        } else {
            H160::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_target_amount(&self) -> Int256 {
        let context = Storage::get_context();
        if let Some(amount_bytes) = Storage::get(context, self.target_amount_key.clone()) {
            Int256::from_byte_string(amount_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_current_amount(&self) -> Int256 {
        let context = Storage::get_context();
        if let Some(amount_bytes) = Storage::get(context, self.current_amount_key.clone()) {
            Int256::from_byte_string(amount_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_deadline(&self) -> Int256 {
        let context = Storage::get_context();
        if let Some(deadline_bytes) = Storage::get(context, self.deadline_key.clone()) {
            Int256::from_byte_string(deadline_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn is_active(&self) -> bool {
        let context = Storage::get_context();
        if let Some(_) = Storage::get(context.clone(), self.active_key.clone()) {
            let current_time = Int256::from(Runtime::get_time() as i64);
            let deadline = self.get_deadline();
            current_time <= deadline
        } else {
            false
        }
    }

    #[method]
    pub fn contribute(&self, contributor: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(contributor) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        if !self.is_active() {
            Runtime::log(ByteString::from_literal("Campaign not active"));
            return false;
        }

        let context = Storage::get_context();
        let current_amount = self.get_current_amount();
        let new_amount = current_amount + amount;

        // Update current amount
        Storage::put(context.clone(), self.current_amount_key.clone(), new_amount.into_byte_string());

        // Store contributor's contribution
        let contribution_key = self.get_contribution_key(contributor);
        let existing_contribution = if let Some(existing_bytes) = Storage::get(context.clone(), contribution_key.clone()) {
            Int256::from_byte_string(existing_bytes)
        } else {
            Int256::zero()
        };
        let total_contribution = existing_contribution + amount;
        Storage::put(context, contribution_key, total_contribution.into_byte_string());

        // Emit contribution event
        let mut args = Array::new();
        args.push(contributor.into_any());
        args.push(amount.into_any());
        args.push(new_amount.into_any());
        Runtime::notify(ByteString::from_literal("Contribution"), args);

        Runtime::log(ByteString::from_literal("Contribution received"));
        true
    }

    #[method]
    #[safe]
    pub fn get_contribution(&self, contributor: H160) -> Int256 {
        let context = Storage::get_context();
        let contribution_key = self.get_contribution_key(contributor);
        
        if let Some(contribution_bytes) = Storage::get(context, contribution_key) {
            Int256::from_byte_string(contribution_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    pub fn withdraw(&self) -> bool {
        let creator = self.get_creator();
        if !Runtime::check_witness(creator) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let current_amount = self.get_current_amount();
        let target_amount = self.get_target_amount();

        if current_amount < target_amount {
            Runtime::log(ByteString::from_literal("Target not reached"));
            return false;
        }

        let context = Storage::get_context();
        
        // Mark as withdrawn
        Storage::put(context.clone(), self.active_key.clone(), ByteString::from_literal("false"));

        // Reset current amount
        Storage::put(context, self.current_amount_key.clone(), Int256::zero().into_byte_string());

        // Emit withdraw event
        let mut args = Array::new();
        args.push(creator.into_any());
        args.push(current_amount.into_any());
        Runtime::notify(ByteString::from_literal("Withdraw"), args);

        Runtime::log(ByteString::from_literal("Funds withdrawn"));
        true
    }

    #[method]
    pub fn refund(&self, contributor: H160) -> bool {
        if !Runtime::check_witness(contributor) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let current_time = Int256::from(Runtime::get_time() as i64);
        let deadline = self.get_deadline();
        let target_amount = self.get_target_amount();
        let current_amount = self.get_current_amount();

        // Can only refund if deadline passed and target not reached
        if current_time <= deadline || current_amount >= target_amount {
            Runtime::log(ByteString::from_literal("Refund not available"));
            return false;
        }

        let context = Storage::get_context();
        let contribution_key = self.get_contribution_key(contributor);
        let contribution = self.get_contribution(contributor);

        if contribution <= Int256::zero() {
            Runtime::log(ByteString::from_literal("No contribution found"));
            return false;
        }

        // Remove contribution
        Storage::delete(context.clone(), contribution_key);

        // Update current amount
        let new_current_amount = current_amount - contribution;
        Storage::put(context, self.current_amount_key.clone(), new_current_amount.into_byte_string());

        // Emit refund event
        let mut args = Array::new();
        args.push(contributor.into_any());
        args.push(contribution.into_any());
        Runtime::notify(ByteString::from_literal("Refund"), args);

        Runtime::log(ByteString::from_literal("Refund processed"));
        true
    }

    // Helper methods
    fn get_contribution_key(&self, contributor: H160) -> ByteString {
        let key = ByteString::from_literal("contribution");
        key.concat(&contributor.into_byte_string())
    }
}