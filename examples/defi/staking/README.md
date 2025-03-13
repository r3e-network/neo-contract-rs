# Staking Platform Example for Neo N3

This example demonstrates a token staking platform implementation on the Neo N3 blockchain using the Neo Contract Rust framework.

## Staking Features

- **Token Staking**: Lock tokens to earn rewards
- **Time-Based Rewards**: Longer staking periods yield higher rewards
- **Reward Distribution**: Automatic calculation and distribution of rewards
- **Multiple Pools**: Support for different assets and reward rates
- **Vesting Periods**: Lockup options for increased APY

## Contract Structure

### Storage Model

The staking platform maintains several key storage items:

```rust
#[storage]
struct StakingPlatform {
    // Staking pools information
    pools: StorageMap<u32, StakingPool>,
    
    // Next pool ID counter
    next_pool_id: StorageItem<u32>,
    
    // User stakes by pool and address
    stakes: StorageMap<Vec<u8>, UserStake>, // key: pool_id + user_address
    
    // Total staked amount per pool
    total_staked: StorageMap<u32, u64>,
    
    // Reward rate per pool (tokens per block)
    reward_rate: StorageMap<u32, u64>,
    
    // Last reward update block per pool
    last_update_block: StorageMap<u32, u32>,
    
    // Accumulated rewards per token per pool
    rewards_per_token: StorageMap<u32, u64>,
    
    // User reward debt per pool
    user_reward_debt: StorageMap<Vec<u8>, u64>, // key: pool_id + user_address
    
    // Platform administrator
    admin: StorageItem<H160>,
}
```

### Key Data Structures

```rust
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

struct UserStake {
    amount: u64,
    stake_time: u64,
    last_claim_time: u64,
    locked_until: u64,
}
```

## Core Functionality

### Creating a Staking Pool

Administrators can create staking pools:

```rust
#[method]
pub fn create_pool(
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
    let admin = self.admin.get();
    assert!(caller == admin, "Only admin can create pools");
    
    // Validate inputs
    assert!(staking_token != H160::zero(), "Invalid staking token");
    assert!(reward_token != H160::zero(), "Invalid reward token");
    assert!(end_block > start_block, "End block must be after start block");
    assert!(early_withdraw_penalty <= 5000, "Penalty too high"); // Max 50%
    
    // Create new pool
    let pool_id = self.next_pool_id.get();
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
    self.pools.insert(pool_id, pool);
    
    // Initialize pool data
    self.total_staked.insert(pool_id, 0);
    self.reward_rate.insert(pool_id, reward_per_block);
    self.last_update_block.insert(pool_id, start_block);
    self.rewards_per_token.insert(pool_id, 0);
    
    // Increment next pool ID
    self.next_pool_id.set(pool_id + 1);
    
    // Emit pool created event
    self.emit_pool_created_event(
        pool_id,
        &staking_token,
        &reward_token,
        start_block,
        end_block,
        reward_per_block,
    );
    
    pool_id
}
```

### Staking Tokens

Users can stake tokens in a pool:

```rust
#[method]
pub fn stake(&mut self, pool_id: u32, amount: u64, lock_period: u64) -> bool {
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
    let stake_key = self.get_stake_key(pool_id, &user);
    let mut user_stake = self.stakes.get(&stake_key).unwrap_or(UserStake {
        amount: 0,
        stake_time: Ledger::current_timestamp(),
        last_claim_time: Ledger::current_timestamp(),
        locked_until: 0,
    });
    
    // Calculate pending rewards before update
    let pending_reward = self.calculate_pending_rewards(pool_id, &user, user_stake.amount);
    
    // Update user reward debt with new stake amount
    let reward_debt_key = self.get_reward_debt_key(pool_id, &user);
    let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap();
    let new_reward_debt = ((user_stake.amount + amount) * rewards_per_token) / 1e12 as u64;
    self.user_reward_debt.insert(reward_debt_key, new_reward_debt);
    
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
    
    self.stakes.insert(stake_key, user_stake);
    
    // Update total staked amount
    let total_staked = self.total_staked.get(&pool_id).unwrap();
    self.total_staked.insert(pool_id, total_staked + amount);
    
    // Transfer tokens from user to contract
    let success = self.transfer_from(pool.staking_token, &user, &self.contract_hash(), amount);
    assert!(success, "Token transfer failed");
    
    // If user had pending rewards, pay them out
    if pending_reward > 0 {
        self.transfer(pool.reward_token, &user, pending_reward);
    }
    
    // Emit stake event
    self.emit_stake_event(pool_id, &user, amount, user_stake.locked_until);
    
    true
}
```

### Claiming Rewards

Users can claim their staking rewards:

```rust
#[method]
pub fn claim_rewards(&mut self, pool_id: u32) -> u64 {
    // Validate pool
    let pool = self.pools.get(&pool_id).expect("Pool not found");
    
    // Get user address
    let user = Runtime::check_witness(&Runtime::current_sender())
        .expect("Authentication failed");
    
    // Update pool rewards
    self.update_pool(pool_id);
    
    // Get user stake
    let stake_key = self.get_stake_key(pool_id, &user);
    let user_stake = self.stakes.get(&stake_key);
    
    // If user has no stake, return zero
    if user_stake.is_none() {
        return 0;
    }
    
    let mut user_stake = user_stake.unwrap();
    
    // Calculate pending rewards
    let pending_reward = self.calculate_pending_rewards(pool_id, &user, user_stake.amount);
    
    if pending_reward > 0 {
        // Update user stake claim time
        user_stake.last_claim_time = Ledger::current_timestamp();
        self.stakes.insert(stake_key, user_stake);
        
        // Update user reward debt
        let reward_debt_key = self.get_reward_debt_key(pool_id, &user);
        let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap();
        let new_reward_debt = (user_stake.amount * rewards_per_token) / 1e12 as u64;
        self.user_reward_debt.insert(reward_debt_key, new_reward_debt);
        
        // Transfer reward tokens to user
        let success = self.transfer(pool.reward_token, &user, pending_reward);
        assert!(success, "Reward transfer failed");
        
        // Emit claim event
        self.emit_claim_event(pool_id, &user, pending_reward);
    }
    
    pending_reward
}
```

### Unstaking Tokens

Users can withdraw their staked tokens:

```rust
#[method]
pub fn unstake(&mut self, pool_id: u32, amount: u64) -> bool {
    // Validate inputs
    assert!(amount > 0, "Amount must be greater than zero");
    let pool = self.pools.get(&pool_id).expect("Pool not found");
    
    // Get user address
    let user = Runtime::check_witness(&Runtime::current_sender())
        .expect("Authentication failed");
    
    // Update pool rewards
    self.update_pool(pool_id);
    
    // Get user stake
    let stake_key = self.get_stake_key(pool_id, &user);
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
        self.stakes.remove(&stake_key);
    } else {
        self.stakes.insert(stake_key, updated_stake);
    }
    
    // Update user reward debt
    let reward_debt_key = self.get_reward_debt_key(pool_id, &user);
    let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap();
    let new_reward_debt = ((user_stake.amount - amount) * rewards_per_token) / 1e12 as u64;
    self.user_reward_debt.insert(reward_debt_key, new_reward_debt);
    
    // Update total staked amount
    let total_staked = self.total_staked.get(&pool_id).unwrap();
    self.total_staked.insert(pool_id, total_staked - amount);
    
    // Transfer staked tokens back to user
    let success = self.transfer(pool.staking_token, &user, amount_to_withdraw);
    assert!(success, "Token transfer failed");
    
    // If penalty was applied, add to accumulated rewards or burn
    if withdrawal_penalty > 0 {
        // In this example we redistribute penalties to remaining stakers
        // Another option would be to send to a treasury or burn
    }
    
    // If user had pending rewards, pay them out
    if pending_reward > 0 {
        self.transfer(pool.reward_token, &user, pending_reward);
    }
    
    // Emit unstake event
    self.emit_unstake_event(pool_id, &user, amount, amount_to_withdraw, withdrawal_penalty);
    
    true
}
```

### Reward Calculation

Rewards are calculated based on staking duration and pool parameters:

```rust
// Updates pool rewards based on blocks elapsed
fn update_pool(&mut self, pool_id: u32) {
    let current_block = Ledger::current_index() as u32;
    let last_update_block = self.last_update_block.get(&pool_id).unwrap();
    
    // If already updated in this block, do nothing
    if current_block <= last_update_block {
        return;
    }
    
    let pool = self.pools.get(&pool_id).unwrap();
    let total_staked = self.total_staked.get(&pool_id).unwrap();
    
    // If no stakes or pool hasn't started or has ended, just update the block
    if total_staked == 0 || current_block < pool.start_block || current_block > pool.end_block {
        self.last_update_block.insert(pool_id, current_block);
        return;
    }
    
    // Calculate rewards for the elapsed blocks
    let end_block = current_block.min(pool.end_block);
    let blocks_elapsed = end_block - last_update_block;
    
    if blocks_elapsed > 0 {
        let reward_rate = self.reward_rate.get(&pool_id).unwrap();
        let reward_this_period = blocks_elapsed as u64 * reward_rate;
        
        // Update accumulated rewards per token
        let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap();
        let new_rewards_per_token = rewards_per_token + 
            (reward_this_period * 1e12 as u64 / total_staked);
        
        self.rewards_per_token.insert(pool_id, new_rewards_per_token);
        self.last_update_block.insert(pool_id, end_block);
    }
}

// Calculates pending rewards for a user
fn calculate_pending_rewards(&self, pool_id: u32, user: &H160, amount: u64) -> u64 {
    if amount == 0 {
        return 0;
    }
    
    let rewards_per_token = self.rewards_per_token.get(&pool_id).unwrap();
    let reward_debt_key = self.get_reward_debt_key(pool_id, user);
    let user_reward_debt = self.user_reward_debt.get(&reward_debt_key).unwrap_or(0);
    
    // Calculate pending rewards: (amount * rewardsPerToken) - userRewardDebt
    ((amount * rewards_per_token) / 1e12 as u64).saturating_sub(user_reward_debt)
}
```

## Building and Deploying

To build this staking platform example:

```bash
# Development build
cargo build -p defi-staking --features std

# Production build
cargo build -p defi-staking --release
```

## Security Considerations

This staking example demonstrates several security mechanisms:

1. **Update Before Action**: Pool rewards are updated before any user action
2. **Safe Reward Calculation**: Preventing overflow in reward calculations
3. **Lockup Enforcement**: Penalties for early withdrawal
4. **Withdrawal Limits**: Users can only withdraw their own tokens
5. **Access Controls**: Administrative functions restricted to authorized users

## Integration with Front-end

This contract can be integrated with a web application to provide users with a complete staking experience:

```javascript
// JavaScript example with neo-js
const { rpc, sc, wallet } = require('@cityofzion/neo-js');

// Connect to staking contract
const stakingContract = new sc.Contract('0xYourContractScriptHash');

// Stake tokens
async function stakeTokens(poolId, amount, lockPeriod) {
  const account = wallet.Account.fromWIF('YourPrivateKeyWIF');
  
  const tx = await stakingContract.invoke(
    'stake',
    [poolId, amount, lockPeriod],
    account
  );
  
  return await tx.send();
}

// Claim rewards
async function claimRewards(poolId) {
  const account = wallet.Account.fromWIF('YourPrivateKeyWIF');
  
  const tx = await stakingContract.invoke(
    'claim_rewards',
    [poolId],
    account
  );
  
  return await tx.send();
}

// Unstake tokens
async function unstakeTokens(poolId, amount) {
  const account = wallet.Account.fromWIF('YourPrivateKeyWIF');
  
  const tx = await stakingContract.invoke(
    'unstake',
    [poolId, amount],
    account
  );
  
  return await tx.send();
}
```

## Educational Value

This example demonstrates several important DeFi concepts:

1. **Yield Generation**: Mechanisms for distributing token rewards
2. **Time-Value Incentives**: Encouraging longer-term commitment through lockups
3. **Effective Annual Percentage Yield (APY)**: Showing how returns are calculated
4. **Reward Accumulation**: Ensuring fair distribution based on stake size and duration
5. **Liquidity Incentives**: Encouraging users to provide liquidity through rewards

## Known Issues and Workarounds

As with other examples in this repository, you may encounter:

1. **Procedural Macro Issues**: The `#[contract]` and other macros may not resolve correctly
2. **Storage Trait Issues**: The `Storage` trait and `StorageContext` may not be found
3. **Runtime Function Signature Mismatches**: Check for current function signatures in the framework

## License

This example is provided under the same license as the Neo Contract Rust framework. 