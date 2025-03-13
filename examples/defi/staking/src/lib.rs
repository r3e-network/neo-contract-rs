#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

/// A token staking platform implementation for Neo N3
/// Allows users to stake NEP-17 tokens in various pools to earn rewards.
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Token Staking Platform for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod neo_staking {
    use neo_contract::prelude::*;
    use neo_contract::error::{Error, Result};
    
    // Events
    #[event]
    struct PoolCreated {
        #[index]
        pool_id: u32,
        staking_token: H160,
        reward_token: H160,
        start_block: u32,
        end_block: u32,
        reward_per_block: u64,
    }
    
    impl PoolCreated {
        fn emit(pool_id: u32, staking_token: H160, reward_token: H160, start_block: u32, end_block: u32, reward_per_block: u64) {
            Self {
                pool_id,
                staking_token,
                reward_token,
                start_block,
                end_block,
                reward_per_block,
            }.emit();
        }
    }
    
    #[event]
    struct Staked {
        #[index]
        pool_id: u32,
        #[index]
        user: H160,
        amount: u64,
        locked_until: u64,
    }
    
    impl Staked {
        fn emit(pool_id: u32, user: H160, amount: u64, locked_until: u64) {
            Self {
                pool_id,
                user,
                amount,
                locked_until,
            }.emit();
        }
    }
    
    #[event]
    struct Unstaked {
        #[index]
        pool_id: u32,
        #[index]
        user: H160,
        amount: u64,
        penalty: u64,
    }
    
    impl Unstaked {
        fn emit(pool_id: u32, user: H160, amount: u64, penalty: u64) {
            Self {
                pool_id,
                user,
                amount,
                penalty,
            }.emit();
        }
    }
    
    #[event]
    struct RewardClaimed {
        #[index]
        pool_id: u32,
        #[index]
        user: H160,
        amount: u64,
    }
    
    impl RewardClaimed {
        fn emit(pool_id: u32, user: H160, amount: u64) {
            Self {
                pool_id,
                user,
                amount,
            }.emit();
        }
    }
    
    // Data structures
    #[derive(Encode, Decode, Debug)]
    struct StakingPool {
        id: u32,
        staking_token: H160,
        reward_token: H160,
        start_block: u32,
        end_block: u32,
        min_stake_amount: u64,
        lockup_duration: u64, // seconds
        early_withdraw_penalty: u64, // basis points
        name: ByteString,
    }
    
    #[derive(Encode, Decode, Debug)]
    struct UserStake {
        amount: u64,
        stake_time: u64,
        last_claim_time: u64,
        locked_until: u64,
    }
    
    // Storage layout
    #[storage]
    struct StakingPlatform {
        // Staking pools information
        pools: Map<u32, StakingPool>,
        
        // Next pool ID counter
        next_pool_id: Item<u32>,
        
        // User stakes by pool and address
        stakes: Map<(u32, H160), UserStake>,
        
        // Total staked amount per pool
        total_staked: Map<u32, u64>,
        
        // Reward rate per pool (tokens per block)
        reward_rate: Map<u32, u64>,
        
        // Last reward update block per pool
        last_update_block: Map<u32, u32>,
        
        // Accumulated rewards per token per pool
        rewards_per_token: Map<u32, u64>,
        
        // User reward debt per pool
        user_reward_debt: Map<(u32, H160), u64>,
        
        // Platform administrator
        admin: Item<H160>,
    }
    
    impl StakingPlatform {
        /// Contract constructor - called once when the contract is deployed
        #[constructor]
        fn new(admin: H160) -> Self {
            // Create contract instance
            let mut contract = Self {
                pools: Map::new(b"pools"),
                next_pool_id: Item::new(b"next_pool_id"),
                stakes: Map::new(b"stakes"),
                total_staked: Map::new(b"total_staked"),
                reward_rate: Map::new(b"reward_rate"),
                last_update_block: Map::new(b"last_update_block"),
                rewards_per_token: Map::new(b"rewards_per_token"),
                user_reward_debt: Map::new(b"user_reward_debt"),
                admin: Item::new(b"admin"),
            };
            
            // Initialize contract state
            contract.next_pool_id.set(1).expect("Failed to set initial pool ID");
            contract.admin.set(admin).expect("Failed to set admin");
            
            contract
        }
        
        /// Creates a new staking pool
        #[method]
        fn create_pool(
            &mut self,
            staking_token: H160,
            reward_token: H160,
            start_block: u32,
            end_block: u32,
            reward_per_block: u64,
            min_stake_amount: u64,
            lockup_duration: u64,
            early_withdraw_penalty: u64,
            name: ByteString,
        ) -> u32 {
            // Verify caller is admin
            let caller = Runtime::check_witness(&Runtime::current_sender())
                .expect("Authentication failed");
            let admin = self.admin.get().expect("Failed to get admin");
            assert!(caller == admin, "Only admin can create pools");
            
            // Validate inputs
            assert!(staking_token != H160::zero(), "Invalid staking token");
            assert!(reward_token != H160::zero(), "Invalid reward token");
            assert!(end_block > start_block, "End block must be after start block");
            assert!(early_withdraw_penalty <= 5000, "Penalty too high"); // Max 50%
            
            // Create new pool
            let pool_id = self.next_pool_id.get().expect("Failed to get next pool ID");
            let pool = StakingPool {
                id: pool_id,
                staking_token,
                reward_token,
                start_block,
                end_block,
                min_stake_amount,
                lockup_duration,
                early_withdraw_penalty,
                name,
            };
            
            // Store pool info
            self.pools.insert(pool_id, &pool).expect("Failed to insert pool");
            
            // Initialize pool data
            self.total_staked.insert(pool_id, &0).expect("Failed to set total staked");
            self.reward_rate.insert(pool_id, &reward_per_block).expect("Failed to set reward rate");
            self.last_update_block.insert(pool_id, &start_block).expect("Failed to set last update block");
            self.rewards_per_token.insert(pool_id, &0).expect("Failed to set rewards per token");
            
            // Increment next pool ID
            self.next_pool_id.set(pool_id + 1).expect("Failed to update next pool ID");
            
            // Emit pool created event
            PoolCreated::emit(pool_id, staking_token, reward_token, start_block, end_block, reward_per_block);
            
            pool_id
        }
        
        /// Stake tokens in a pool
        #[method]
        fn stake(&mut self, pool_id: u32, amount: u64, lock_period: u64) -> bool {
            // Validate inputs
            assert!(amount > 0, "Amount must be greater than zero");
            let pool = self.pools.get(&pool_id).expect("Pool not found");
            assert!(amount >= pool.min_stake_amount, "Amount below minimum");
            
            // Get user address
            let user = Runtime::check_witness(&Runtime::current_sender())
                .expect("Authentication failed");
            
            // Update pool rewards
            self.update_pool(pool_id);
            
            // Get existing stake or create new one
            let stake_key = (pool_id, user);
            let mut user_stake = self.stakes.get(&stake_key).unwrap_or(UserStake {
                amount: 0,
                stake_time: Ledger::current_timestamp(),
                last_claim_time: Ledger::current_timestamp(),
                locked_until: 0,
            });
            
            // Calculate pending rewards before update
            let pending_reward = self.calculate_pending_rewards(pool_id, &user, user_stake.amount);
            
            // Update user reward debt with new stake amount
            let reward_debt_key = (pool_id, user);
            let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap_or(0);
            let new_reward_debt = ((user_stake.amount + amount) * rewards_per_token) / 10u64.pow(12);
            self.user_reward_debt.insert(reward_debt_key, &new_reward_debt).expect("Failed to update reward debt");
            
            // Update user stake
            user_stake.amount += amount;
            
            // Update lockup if requested
            if lock_period > 0 {
                let current_time = Ledger::current_timestamp();
                let new_locked_until = current_time + lock_period;
                
                // Only extend lock, never reduce it
                if new_locked_until > user_stake.locked_until {
                    user_stake.locked_until = new_locked_until;
                }
            }
            
            self.stakes.insert(stake_key, &user_stake).expect("Failed to update stake");
            
            // Update total staked amount
            let total_staked = self.total_staked.get(&pool_id).unwrap_or(0);
            self.total_staked.insert(pool_id, &(total_staked + amount)).expect("Failed to update total staked");
            
            // Transfer tokens from user to contract
            let success = self.transfer_from(pool.staking_token, &user, &Runtime::contract_hash(), amount);
            assert!(success, "Token transfer failed");
            
            // If user had pending rewards, pay them out
            if pending_reward > 0 {
                self.transfer(pool.reward_token, &user, pending_reward);
            }
            
            // Emit stake event
            Staked::emit(pool_id, user, amount, user_stake.locked_until);
            
            true
        }
        
        /// Claim rewards from staking
        #[method]
        fn claim_rewards(&mut self, pool_id: u32) -> u64 {
            // Validate pool
            let pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get user address
            let user = Runtime::check_witness(&Runtime::current_sender())
                .expect("Authentication failed");
            
            // Update pool rewards
            self.update_pool(pool_id);
            
            // Get user stake
            let stake_key = (pool_id, user);
            let user_stake_option = self.stakes.get(&stake_key);
            
            // If user has no stake, return zero
            if user_stake_option.is_none() {
                return 0;
            }
            
            let mut user_stake = user_stake_option.unwrap();
            
            // Calculate pending rewards
            let pending_reward = self.calculate_pending_rewards(pool_id, &user, user_stake.amount);
            
            if pending_reward > 0 {
                // Update user stake claim time
                user_stake.last_claim_time = Ledger::current_timestamp();
                self.stakes.insert(stake_key, &user_stake).expect("Failed to update stake");
                
                // Update user reward debt
                let reward_debt_key = (pool_id, user);
                let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap_or(0);
                let new_reward_debt = (user_stake.amount * rewards_per_token) / 10u64.pow(12);
                self.user_reward_debt.insert(reward_debt_key, &new_reward_debt).expect("Failed to update reward debt");
                
                // Transfer reward tokens to user
                let success = self.transfer(pool.reward_token, &user, pending_reward);
                assert!(success, "Reward transfer failed");
                
                // Emit claim event
                RewardClaimed::emit(pool_id, user, pending_reward);
            }
            
            pending_reward
        }
        
        /// Unstake tokens from a pool
        #[method]
        fn unstake(&mut self, pool_id: u32, amount: u64) -> bool {
            // Validate inputs
            assert!(amount > 0, "Amount must be greater than zero");
            let pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get user address
            let user = Runtime::check_witness(&Runtime::current_sender())
                .expect("Authentication failed");
            
            // Update pool rewards
            self.update_pool(pool_id);
            
            // Get user stake
            let stake_key = (pool_id, user);
            let user_stake = self.stakes.get(&stake_key).expect("No stake found");
            
            // Verify sufficient stake amount
            assert!(user_stake.amount >= amount, "Insufficient staked amount");
            
            // Calculate pending rewards
            let pending_reward = self.calculate_pending_rewards(pool_id, &user, user_stake.amount);
            
            // Check if early withdrawal (before lockup period ends)
            let current_time = Ledger::current_timestamp();
            let early_withdrawal = user_stake.locked_until > current_time;
            
            // Calculate withdrawal penalty if applicable
            let withdrawal_penalty = if early_withdrawal {
                (amount * pool.early_withdraw_penalty) / 10000
            } else {
                0
            };
            
            let amount_to_withdraw = amount - withdrawal_penalty;
            
            // Update user stake
            let mut updated_stake = user_stake.clone();
            updated_stake.amount -= amount;
            
            // If all tokens withdrawn, remove stake
            if updated_stake.amount == 0 {
                self.stakes.remove(&stake_key).expect("Failed to remove stake");
            } else {
                self.stakes.insert(stake_key, &updated_stake).expect("Failed to update stake");
            }
            
            // Update user reward debt
            let reward_debt_key = (pool_id, user);
            let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap_or(0);
            let new_reward_debt = ((user_stake.amount - amount) * rewards_per_token) / 10u64.pow(12);
            self.user_reward_debt.insert(reward_debt_key, &new_reward_debt).expect("Failed to update reward debt");
            
            // Update total staked amount
            let total_staked = self.total_staked.get(&pool_id).unwrap_or(0);
            self.total_staked.insert(pool_id, &(total_staked - amount)).expect("Failed to update total staked");
            
            // Transfer staked tokens back to user
            let success = self.transfer(pool.staking_token, &user, amount_to_withdraw);
            assert!(success, "Token transfer failed");
            
            // If user had pending rewards, pay them out
            if pending_reward > 0 {
                self.transfer(pool.reward_token, &user, pending_reward);
            }
            
            // Emit unstake event
            Unstaked::emit(pool_id, user, amount, withdrawal_penalty);
            
            true
        }
        
        /// Updates pool rewards based on blocks elapsed
        fn update_pool(&mut self, pool_id: u32) {
            let current_block = Ledger::current_index() as u32;
            let last_update_block = self.last_update_block.get(&pool_id).unwrap_or(0);
            
            // If already updated in this block, do nothing
            if current_block <= last_update_block {
                return;
            }
            
            let pool = self.pools.get(&pool_id).unwrap();
            let total_staked = self.total_staked.get(&pool_id).unwrap_or(0);
            
            // If no stakes or pool hasn't started or has ended, just update the block
            if total_staked == 0 || current_block < pool.start_block || current_block > pool.end_block {
                self.last_update_block.insert(pool_id, &current_block).expect("Failed to update block");
                return;
            }
            
            // Calculate rewards for the elapsed blocks
            let end_block = current_block.min(pool.end_block);
            let blocks_elapsed = end_block - last_update_block;
            
            if blocks_elapsed > 0 {
                let reward_rate = self.reward_rate.get(&pool_id).unwrap_or(0);
                let reward_this_period = blocks_elapsed as u64 * reward_rate;
                
                // Update accumulated rewards per token
                let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap_or(0);
                let new_rewards_per_token = rewards_per_token + 
                    (reward_this_period * 10u64.pow(12) / total_staked);
                
                self.rewards_per_token.insert(pool_id, &new_rewards_per_token).expect("Failed to update rewards per token");
                self.last_update_block.insert(pool_id, &end_block).expect("Failed to update block");
            }
        }
        
        /// Calculates pending rewards for a user
        fn calculate_pending_rewards(&self, pool_id: u32, user: &H160, amount: u64) -> u64 {
            if amount == 0 {
                return 0;
            }
            
            let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap_or(0);
            let reward_debt_key = (pool_id, *user);
            let user_reward_debt = self.user_reward_debt.get(&reward_debt_key).unwrap_or(0);
            
            // Calculate pending rewards: (amount * rewardsPerToken) - userRewardDebt
            ((amount * rewards_per_token) / 10u64.pow(12)).saturating_sub(user_reward_debt)
        }
        
        /// Helper function to transfer tokens from user to contract
        fn transfer_from(&self, token_hash: H160, from: &H160, to: &H160, amount: u64) -> bool {
            let mut args = Array::new();
            args.push(Any::from(*from));
            args.push(Any::from(*to));
            args.push(Any::from(amount));
            
            let result = Runtime::call(
                token_hash,
                &ByteString::from("transfer"),
                args,
                CallFlags::empty()
            );
            
            match result {
                Some(res) => res.as_bool().unwrap_or(false),
                None => false,
            }
        }
        
        /// Helper function to transfer tokens from contract to user
        fn transfer(&self, token_hash: H160, to: &H160, amount: u64) -> bool {
            let mut args = Array::new();
            args.push(Any::from(Runtime::contract_hash()));
            args.push(Any::from(*to));
            args.push(Any::from(amount));
            
            let result = Runtime::call(
                token_hash,
                &ByteString::from("transfer"),
                args,
                CallFlags::empty()
            );
            
            match result {
                Some(res) => res.as_bool().unwrap_or(false),
                None => false,
            }
        }
        
        // View methods
        
        /// Gets information about a staking pool
        #[safe]
        fn get_pool_info(&self, pool_id: u32) -> Option<(H160, H160, u32, u32, u64, u64, u64)> {
            let pool = self.pools.get(&pool_id)?;
            
            Some((
                pool.staking_token,
                pool.reward_token,
                pool.start_block,
                pool.end_block,
                pool.min_stake_amount,
                pool.lockup_duration,
                pool.early_withdraw_penalty
            ))
        }
        
        /// Gets the user's stake in a pool
        #[safe]
        fn get_user_stake(&self, pool_id: u32, user: H160) -> Option<(u64, u64, u64)> {
            let stake_key = (pool_id, user);
            let stake = self.stakes.get(&stake_key)?;
            
            Some((
                stake.amount,
                stake.stake_time,
                stake.locked_until
            ))
        }
        
        /// Gets the total staked amount in a pool
        #[safe]
        fn get_total_staked(&self, pool_id: u32) -> u64 {
            self.total_staked.get(&pool_id).unwrap_or(0)
        }
        
        /// Gets the current reward rate for a pool
        #[safe]
        fn get_reward_rate(&self, pool_id: u32) -> u64 {
            self.reward_rate.get(&pool_id).unwrap_or(0)
        }
        
        /// Gets the pending rewards for a user in a pool
        #[safe]
        fn get_pending_rewards(&self, pool_id: u32, user: H160) -> u64 {
            let stake_key = (pool_id, user);
            let stake = match self.stakes.get(&stake_key) {
                Some(s) => s,
                None => return 0,
            };
            
            self.calculate_pending_rewards(pool_id, &user, stake.amount)
        }
        
        /// Gets the contract administrator
        #[safe]
        fn get_admin(&self) -> H160 {
            self.admin.get().unwrap_or(H160::zero())
        }
    }
} 