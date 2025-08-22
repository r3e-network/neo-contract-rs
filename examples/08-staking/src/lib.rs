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
// Simple Staking Implementation
pub struct Staking {
    // Storage keys for metadata
    admin_key: ByteString,
    reward_rate_key: ByteString,
    total_staked_key: ByteString,
    initialized_key: ByteString,
}

#[contract]
impl Staking {
    pub fn init() -> Self {
        Self {
            admin_key: ByteString::from_literal("admin"),
            reward_rate_key: ByteString::from_literal("reward_rate"),
            total_staked_key: ByteString::from_literal("total_staked"),
            initialized_key: ByteString::from_literal("initialized"),
        }
    }

    #[method]
    pub fn initialize(&self, reward_rate: Int256) -> bool {
        let context = Storage::get_context();
        
        // Check if already initialized
        if let Some(_) = Storage::get(context.clone(), self.initialized_key.clone()) {
            Runtime::log(ByteString::from_literal("Already initialized"));
            return false;
        }

        let admin = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(admin) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if reward_rate <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid reward rate"));
            return false;
        }

        // Store staking data in storage
        Storage::put(context.clone(), self.admin_key.clone(), admin.into_byte_string());
        Storage::put(context.clone(), self.reward_rate_key.clone(), reward_rate.into_byte_string());
        Storage::put(context.clone(), self.total_staked_key.clone(), Int256::zero().into_byte_string());
        Storage::put(context, self.initialized_key.clone(), ByteString::from_literal("true"));

        Runtime::log(ByteString::from_literal("Staking pool initialized"));
        true
    }

    #[method]
    #[safe]
    pub fn get_admin(&self) -> H160 {
        let context = Storage::get_context();
        if let Some(admin_bytes) = Storage::get(context, self.admin_key.clone()) {
            H160::from_byte_string(admin_bytes)
        } else {
            H160::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_reward_rate(&self) -> Int256 {
        let context = Storage::get_context();
        if let Some(rate_bytes) = Storage::get(context, self.reward_rate_key.clone()) {
            Int256::from_byte_string(rate_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_total_staked(&self) -> Int256 {
        let context = Storage::get_context();
        if let Some(amount_bytes) = Storage::get(context, self.total_staked_key.clone()) {
            Int256::from_byte_string(amount_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_staked_amount(&self, staker: H160) -> Int256 {
        let context = Storage::get_context();
        let stake_key = self.get_stake_key(staker);
        
        if let Some(amount_bytes) = Storage::get(context, stake_key) {
            Int256::from_byte_string(amount_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    #[safe]
    pub fn get_last_claim_time(&self, staker: H160) -> Int256 {
        let context = Storage::get_context();
        let time_key = self.get_last_claim_key(staker);
        
        if let Some(time_bytes) = Storage::get(context, time_key) {
            Int256::from_byte_string(time_bytes)
        } else {
            Int256::zero()
        }
    }

    #[method]
    pub fn stake(&self, staker: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(staker) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        let context = Storage::get_context();
        let current_stake = self.get_staked_amount(staker);
        let new_stake = current_stake + amount;
        
        let total_staked = self.get_total_staked();
        let new_total = total_staked + amount;

        // Update staked amount
        let stake_key = self.get_stake_key(staker);
        Storage::put(context.clone(), stake_key, new_stake.into_byte_string());

        // Update total staked
        Storage::put(context.clone(), self.total_staked_key.clone(), new_total.into_byte_string());

        // Set last claim time to current time
        let claim_time_key = self.get_last_claim_key(staker);
        let current_time = Int256::from(Runtime::get_time() as i64);
        Storage::put(context, claim_time_key, current_time.into_byte_string());

        // Emit stake event
        let mut args = Array::new();
        args.push(staker.into_any());
        args.push(amount.into_any());
        args.push(new_stake.into_any());
        Runtime::notify(ByteString::from_literal("Stake"), args);

        Runtime::log(ByteString::from_literal("Tokens staked"));
        true
    }

    #[method]
    pub fn unstake(&self, staker: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(staker) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if amount <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid amount"));
            return false;
        }

        let current_stake = self.get_staked_amount(staker);
        if current_stake < amount {
            Runtime::log(ByteString::from_literal("Insufficient stake"));
            return false;
        }

        let context = Storage::get_context();
        let new_stake = current_stake - amount;
        let total_staked = self.get_total_staked();
        let new_total = total_staked - amount;

        // Update staked amount
        let stake_key = self.get_stake_key(staker);
        if new_stake == Int256::zero() {
            Storage::delete(context.clone(), stake_key);
            // Also remove last claim time
            let claim_time_key = self.get_last_claim_key(staker);
            Storage::delete(context.clone(), claim_time_key);
        } else {
            Storage::put(context.clone(), stake_key, new_stake.into_byte_string());
        }

        // Update total staked
        Storage::put(context, self.total_staked_key.clone(), new_total.into_byte_string());

        // Emit unstake event
        let mut args = Array::new();
        args.push(staker.into_any());
        args.push(amount.into_any());
        args.push(new_stake.into_any());
        Runtime::notify(ByteString::from_literal("Unstake"), args);

        Runtime::log(ByteString::from_literal("Tokens unstaked"));
        true
    }

    #[method]
    #[safe]
    pub fn calculate_rewards(&self, staker: H160) -> Int256 {
        let staked_amount = self.get_staked_amount(staker);
        if staked_amount == Int256::zero() {
            return Int256::zero();
        }

        let last_claim_time = self.get_last_claim_time(staker);
        let current_time = Int256::from(Runtime::get_time() as i64);
        let time_diff = current_time - last_claim_time;

        if time_diff <= Int256::zero() {
            return Int256::zero();
        }

        let reward_rate = self.get_reward_rate();
        
        // Calculate rewards: (staked_amount * reward_rate * time_diff) / (100 * 86400)
        // Assuming reward_rate is percentage per day, time_diff is in seconds
        let rewards = (staked_amount * reward_rate * time_diff) / (Int256::from(100) * Int256::from(86400));
        rewards
    }

    #[method]
    pub fn claim_rewards(&self, staker: H160) -> bool {
        if !Runtime::check_witness(staker) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        let rewards = self.calculate_rewards(staker);
        if rewards <= Int256::zero() {
            Runtime::log(ByteString::from_literal("No rewards to claim"));
            return false;
        }

        let context = Storage::get_context();
        
        // Update last claim time
        let claim_time_key = self.get_last_claim_key(staker);
        let current_time = Int256::from(Runtime::get_time() as i64);
        Storage::put(context, claim_time_key, current_time.into_byte_string());

        // Emit claim event
        let mut args = Array::new();
        args.push(staker.into_any());
        args.push(rewards.into_any());
        Runtime::notify(ByteString::from_literal("ClaimRewards"), args);

        Runtime::log(ByteString::from_literal("Rewards claimed"));
        true
    }

    #[method]
    pub fn update_reward_rate(&self, new_rate: Int256) -> bool {
        let admin = self.get_admin();
        if !Runtime::check_witness(admin) {
            Runtime::log(ByteString::from_literal("No authorization"));
            return false;
        }

        if new_rate <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid reward rate"));
            return false;
        }

        let context = Storage::get_context();
        Storage::put(context, self.reward_rate_key.clone(), new_rate.into_byte_string());

        // Emit rate update event
        let mut args = Array::new();
        args.push(admin.into_any());
        args.push(new_rate.into_any());
        Runtime::notify(ByteString::from_literal("RewardRateUpdate"), args);

        Runtime::log(ByteString::from_literal("Reward rate updated"));
        true
    }

    // Helper methods
    fn get_stake_key(&self, staker: H160) -> ByteString {
        let key = ByteString::from_literal("stake");
        key.concat(&staker.into_byte_string())
    }

    fn get_last_claim_key(&self, staker: H160) -> ByteString {
        let key = ByteString::from_literal("last_claim");
        key.concat(&staker.into_byte_string())
    }
}