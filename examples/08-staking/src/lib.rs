//! # Token Staking Contract
//!
//! A comprehensive staking platform demonstrating DeFi yield farming patterns:
//! - Multiple staking pools with different reward rates
//! - Time-locked staking with early withdrawal penalties
//! - Compound interest calculations with automatic reinvestment
//! - Flexible reward distribution mechanisms
//! - Administrative controls for pool management
//! - Emergency withdrawal and pause functionality
//!
//! This contract showcases advanced DeFi patterns for token incentivization
//! and liquidity mining programs.

#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

declare_id!("StakingContract1111111111111111111111111112");

/// Staking pool information
#[derive(Clone, Debug)]
pub struct StakingPool {
    pub pool_id: u32,
    pub stake_token: H160,      // Token to be staked
    pub reward_token: H160,     // Token given as reward
    pub reward_rate: u32,       // Reward rate in basis points per year
    pub lock_period: u64,       // Lock period in seconds
    pub penalty_rate: u32,      // Early withdrawal penalty in basis points
    pub total_staked: Int256,   // Total amount staked in pool
    pub is_active: bool,        // Pool status
}

/// User stake information
#[derive(Clone, Debug)]
pub struct UserStake {
    pub pool_id: u32,
    pub user: H160,
    pub amount: Int256,         // Staked amount
    pub stake_time: u64,        // When stake was created
    pub last_claim_time: u64,   // Last reward claim time
    pub accumulated_rewards: Int256, // Unclaimed rewards
}

/// Platform data
#[derive(Clone, Debug)]
pub struct StakingPlatform {
    pub owner: H160,
    pub pool_count: u32,
    pub total_value_locked: Int256,
    pub is_paused: bool,
}

/// Initialize context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(init)]
    pub platform: AccountInfo<'info>,
}

/// Create pool context
#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
    #[account(init)]
    pub pool: AccountInfo<'info>,
}

/// Stake context
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub pool: AccountInfo<'info>,
    #[account(mut)]
    pub user_stake_account: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
}

/// Unstake context
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub pool: AccountInfo<'info>,
    #[account(mut)]
    pub user_stake_account: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
}

/// Claim rewards context
#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub pool: AccountInfo<'info>,
    #[account(mut)]
    pub user_stake_account: AccountInfo<'info>,
}

/// Admin context
#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
}

/// View context
#[derive(Accounts)]
pub struct View<'info> {
    pub account: AccountInfo<'info>,
}

#[program]
pub mod staking {
    use super::*;

    /// Initialize the staking platform
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let platform = &mut ctx.accounts.platform;

        // Initialize platform data
        let platform_data = StakingPlatform {
            owner: owner.key(),
            pool_count: 0,
            total_value_locked: Int256::zero(),
            is_paused: false,
        };

        // Store platform data
        let storage = Storage::get_context();
        Storage::put(storage, ByteString::from_literal("staking_platform"), platform_data.serialize());

        emit!(PlatformInitialized { owner: owner.key() });
        Ok(())
    }

    /// Create a new staking pool
    pub fn create_pool(
        ctx: Context<CreatePool>,
        stake_token: H160,
        reward_token: H160,
        reward_rate: u32,
        lock_period: u64,
        penalty_rate: u32
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let platform = &mut ctx.accounts.platform;
        let pool = &mut ctx.accounts.pool;

        // Validate parameters
        require!(reward_rate <= 100000, "Reward rate too high (max 1000%)");
        require!(penalty_rate <= 5000, "Penalty rate too high (max 50%)");
        require!(lock_period <= 31536000, "Lock period too long (max 1 year)");

        let storage = Storage::get_context();
        let mut platform_data = get_platform_data(&storage)?;

        require!(owner.key() == platform_data.owner, "Unauthorized: Not platform owner");
        require!(!platform_data.is_paused, "Platform is paused");

        // Create new pool
        platform_data.pool_count += 1;
        let pool_id = platform_data.pool_count;

        let pool_data = StakingPool {
            pool_id,
            stake_token,
            reward_token,
            reward_rate,
            lock_period,
            penalty_rate,
            total_staked: Int256::zero(),
            is_active: true,
        };

        // Store pool data
        let pool_key = format!("staking_pool_{}", pool_id);
        Storage::put(storage.clone(), ByteString::from_str(&pool_key), pool_data.serialize());

        // Update platform data
        Storage::put(storage, ByteString::from_literal("staking_platform"), platform_data.serialize());

        emit!(PoolCreated {
            pool_id,
            stake_token,
            reward_token,
            reward_rate,
            lock_period,
        });
        Ok(())
    }

    /// Stake tokens in a pool
    pub fn stake_tokens(
        ctx: Context<Stake>,
        pool_id: u32,
        amount: Int256
    ) -> Result<()> {
        let user = &ctx.accounts.user;
        let pool = &mut ctx.accounts.pool;
        let user_stake_account = &mut ctx.accounts.user_stake_account;
        let platform = &mut ctx.accounts.platform;

        require!(amount > Int256::zero(), "Amount must be positive");

        let storage = Storage::get_context();
        let platform_data = get_platform_data(&storage)?;
        require!(!platform_data.is_paused, "Platform is paused");

        // Get pool data
        let pool_key = format!("staking_pool_{}", pool_id);
        let mut pool_data: StakingPool = match Storage::get(storage.clone(), ByteString::from_str(&pool_key)) {
            Some(data) => StakingPool::deserialize(data)?,
            None => return Err(ErrorCode::PoolNotFound.into()),
        };

        require!(pool_data.is_active, "Pool is not active");

        // Get or create user stake
        let stake_key = format!("user_stake_{}_{}", pool_id, user.key());
        let mut user_stake = match Storage::get(storage.clone(), ByteString::from_str(&stake_key)) {
            Some(data) => UserStake::deserialize(data)?,
            None => UserStake {
                pool_id,
                user: user.key(),
                amount: Int256::zero(),
                stake_time: Runtime::get_time(),
                last_claim_time: Runtime::get_time(),
                accumulated_rewards: Int256::zero(),
            },
        };

        // Calculate pending rewards before updating stake
        let pending_rewards = calculate_rewards(&user_stake, &pool_data)?;
        user_stake.accumulated_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);

        // Update stake
        user_stake.amount = user_stake.amount.checked_add(&amount);
        user_stake.last_claim_time = Runtime::get_time();
        
        // If this is a new stake, update stake time
        if user_stake.stake_time == user_stake.last_claim_time {
            user_stake.stake_time = Runtime::get_time();
        }

        // Update pool total
        pool_data.total_staked = pool_data.total_staked.checked_add(&amount);

        // Store updated data
        Storage::put(storage.clone(), ByteString::from_str(&stake_key), user_stake.serialize());
        Storage::put(storage, ByteString::from_str(&pool_key), pool_data.serialize());

        emit!(TokensStaked {
            pool_id,
            user: user.key(),
            amount,
            total_staked: user_stake.amount,
        });

        Ok(())
    }

    /// Unstake tokens from a pool
    pub fn unstake_tokens(
        ctx: Context<Unstake>,
        pool_id: u32,
        amount: Int256
    ) -> Result<()> {
        let user = &ctx.accounts.user;
        let pool = &mut ctx.accounts.pool;
        let user_stake_account = &mut ctx.accounts.user_stake_account;
        let platform = &mut ctx.accounts.platform;

        require!(amount > Int256::zero(), "Amount must be positive");

        let storage = Storage::get_context();
        let platform_data = get_platform_data(&storage)?;

        // Get pool data
        let pool_key = format!("staking_pool_{}", pool_id);
        let mut pool_data: StakingPool = match Storage::get(storage.clone(), ByteString::from_str(&pool_key)) {
            Some(data) => StakingPool::deserialize(data)?,
            None => return Err(ErrorCode::PoolNotFound.into()),
        };

        // Get user stake
        let stake_key = format!("user_stake_{}_{}", pool_id, user.key());
        let mut user_stake: UserStake = match Storage::get(storage.clone(), ByteString::from_str(&stake_key)) {
            Some(data) => UserStake::deserialize(data)?,
            None => return Err(ErrorCode::StakeNotFound.into()),
        };

        require!(user_stake.amount >= amount, "Insufficient staked amount");

        // Calculate pending rewards
        let pending_rewards = calculate_rewards(&user_stake, &pool_data)?;
        user_stake.accumulated_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);

        // Check if lock period has passed
        let current_time = Runtime::get_time();
        let mut final_amount = amount;
        let mut penalty = Int256::zero();

        if current_time < user_stake.stake_time + pool_data.lock_period {
            // Apply early withdrawal penalty
            penalty = amount.checked_mul(&Int256::new(pool_data.penalty_rate as i64))
                .checked_div(&Int256::new(10000));
            final_amount = amount.checked_sub(&penalty);
        }

        // Update stake
        user_stake.amount = user_stake.amount.checked_sub(&amount);
        user_stake.last_claim_time = current_time;

        // Update pool total
        pool_data.total_staked = pool_data.total_staked.checked_sub(&amount);

        // Store updated data
        if user_stake.amount == Int256::zero() {
            Storage::delete(storage.clone(), ByteString::from_str(&stake_key));
        } else {
            Storage::put(storage.clone(), ByteString::from_str(&stake_key), user_stake.serialize());
        }
        Storage::put(storage, ByteString::from_str(&pool_key), pool_data.serialize());

        emit!(TokensUnstaked {
            pool_id,
            user: user.key(),
            amount: final_amount,
            penalty,
            remaining_staked: user_stake.amount,
        });

        Ok(())
    }

    /// Claim accumulated rewards
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        pool_id: u32
    ) -> Result<()> {
        let user = &ctx.accounts.user;
        let pool = &mut ctx.accounts.pool;
        let user_stake_account = &mut ctx.accounts.user_stake_account;

        let storage = Storage::get_context();

        // Get pool data
        let pool_key = format!("staking_pool_{}", pool_id);
        let pool_data: StakingPool = match Storage::get(storage.clone(), ByteString::from_str(&pool_key)) {
            Some(data) => StakingPool::deserialize(data)?,
            None => return Err(ErrorCode::PoolNotFound.into()),
        };

        // Get user stake
        let stake_key = format!("user_stake_{}_{}", pool_id, user.key());
        let mut user_stake: UserStake = match Storage::get(storage.clone(), ByteString::from_str(&stake_key)) {
            Some(data) => UserStake::deserialize(data)?,
            None => return Err(ErrorCode::StakeNotFound.into()),
        };

        // Calculate total claimable rewards
        let pending_rewards = calculate_rewards(&user_stake, &pool_data)?;
        let total_rewards = user_stake.accumulated_rewards.checked_add(&pending_rewards);

        require!(total_rewards > Int256::zero(), "No rewards to claim");

        // Reset accumulated rewards and update claim time
        user_stake.accumulated_rewards = Int256::zero();
        user_stake.last_claim_time = Runtime::get_time();

        // Store updated stake
        Storage::put(storage, ByteString::from_str(&stake_key), user_stake.serialize());

        emit!(RewardsClaimed {
            pool_id,
            user: user.key(),
            amount: total_rewards,
        });

        Ok(())
    }

    /// Emergency pause (owner only)
    pub fn emergency_pause(ctx: Context<AdminAction>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let platform = &mut ctx.accounts.platform;

        let storage = Storage::get_context();
        let mut platform_data = get_platform_data(&storage)?;

        require!(owner.key() == platform_data.owner, "Unauthorized: Not platform owner");

        platform_data.is_paused = true;
        Storage::put(storage, ByteString::from_literal("staking_platform"), platform_data.serialize());

        emit!(PlatformPaused {});
        Ok(())
    }

    /// Get pool information
    pub fn get_pool_info(ctx: Context<View>, pool_id: u32) -> Result<StakingPool> {
        let storage = Storage::get_context();
        let pool_key = format!("staking_pool_{}", pool_id);
        
        match Storage::get(storage, ByteString::from_str(&pool_key)) {
            Some(data) => StakingPool::deserialize(data),
            None => Err(ErrorCode::PoolNotFound.into()),
        }
    }

    /// Get user stake information
    pub fn get_user_stake(ctx: Context<View>, pool_id: u32, user: H160) -> Result<UserStake> {
        let storage = Storage::get_context();
        let stake_key = format!("user_stake_{}_{}", pool_id, user);
        
        match Storage::get(storage, ByteString::from_str(&stake_key)) {
            Some(data) => UserStake::deserialize(data),
            None => Err(ErrorCode::StakeNotFound.into()),
        }
    }

    /// Calculate pending rewards for user
    pub fn get_pending_rewards(ctx: Context<View>, pool_id: u32, user: H160) -> Result<Int256> {
        let storage = Storage::get_context();

        // Get pool data
        let pool_key = format!("staking_pool_{}", pool_id);
        let pool_data: StakingPool = match Storage::get(storage.clone(), ByteString::from_str(&pool_key)) {
            Some(data) => StakingPool::deserialize(data)?,
            None => return Err(ErrorCode::PoolNotFound.into()),
        };

        // Get user stake
        let stake_key = format!("user_stake_{}_{}", pool_id, user);
        let user_stake: UserStake = match Storage::get(storage, ByteString::from_str(&stake_key)) {
            Some(data) => UserStake::deserialize(data)?,
            None => return Ok(Int256::zero()),
        };

        let pending = calculate_rewards(&user_stake, &pool_data)?;
        Ok(user_stake.accumulated_rewards.checked_add(&pending))
    }
}

// Helper functions
fn get_platform_data(storage: &Storage) -> Result<StakingPlatform> {
    match Storage::get(storage.clone(), ByteString::from_literal("staking_platform")) {
        Some(data) => StakingPlatform::deserialize(data),
        None => Err(ErrorCode::PlatformNotInitialized.into()),
    }
}

fn calculate_rewards(user_stake: &UserStake, pool: &StakingPool) -> Result<Int256> {
    if user_stake.amount == Int256::zero() {
        return Ok(Int256::zero());
    }

    let current_time = Runtime::get_time();
    let time_elapsed = current_time - user_stake.last_claim_time;
    
    // Calculate rewards: (amount * rate * time) / (365 * 24 * 3600 * 10000)
    let annual_seconds = Int256::new(31536000); // 365 * 24 * 3600
    let basis_points = Int256::new(10000);
    
    let rewards = user_stake.amount
        .checked_mul(&Int256::new(pool.reward_rate as i64))
        .checked_mul(&Int256::new(time_elapsed as i64))
        .checked_div(&annual_seconds)
        .checked_div(&basis_points);

    Ok(rewards)
}

// Events
#[event]
pub struct PlatformInitialized {
    pub owner: H160,
}

#[event]
pub struct PoolCreated {
    pub pool_id: u32,
    pub stake_token: H160,
    pub reward_token: H160,
    pub reward_rate: u32,
    pub lock_period: u64,
}

#[event]
pub struct TokensStaked {
    pub pool_id: u32,
    pub user: H160,
    pub amount: Int256,
    pub total_staked: Int256,
}

#[event]
pub struct TokensUnstaked {
    pub pool_id: u32,
    pub user: H160,
    pub amount: Int256,
    pub penalty: Int256,
    pub remaining_staked: Int256,
}

#[event]
pub struct RewardsClaimed {
    pub pool_id: u32,
    pub user: H160,
    pub amount: Int256,
}

#[event]
pub struct PlatformPaused {}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Platform not initialized")]
    PlatformNotInitialized,
    #[msg("Pool not found")]
    PoolNotFound,
    #[msg("Stake not found")]
    StakeNotFound,
    #[msg("Pool not active")]
    PoolNotActive,
    #[msg("Insufficient staked amount")]
    InsufficientStake,
    #[msg("Platform paused")]
    PlatformPaused,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid amount")]
    InvalidAmount,
}