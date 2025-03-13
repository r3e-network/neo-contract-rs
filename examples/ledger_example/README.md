# Neo N3 Ledger API Example

This example demonstrates practical usage of the Neo N3 Ledger API for blockchain data interactions in smart contracts. It shows how to implement time-based and block-based logic, transaction validation, and rate limiting using blockchain state.

## Features

This smart contract demonstrates several key capabilities of the Neo N3 Ledger API:

### Time-Based Vesting

- Linear vesting schedule implementation using blockchain timestamps
- Configurable vesting periods with start and end times
- Proportional release of tokens over time
- Claim mechanism for vested tokens

### Block-Based Rewards

- Reward distribution based on block progression
- Per-block reward configuration
- Tracking of last claimed block for each user
- Claiming accumulated rewards

### Transaction Validation

- Processing transactions based on confirmation count
- Transaction height validation
- Prevention of duplicate processing
- Confirmation threshold configuration

### Rate Limiting

- Time-based action cooldown using blockchain timestamps
- Configurable cooldown periods
- Protection against action spam

### Blockchain Data Access

- Current block information retrieval
- Historical block data access
- Transaction information queries
- Timestamp access and validation

## Implementation Details

The contract is structured with several key components:

### Storage Layout

```
// Vesting-related storage
vesting_beneficiaries: StorageMap<H160, bool>
vesting_start_times: StorageMap<H160, u64>
vesting_end_times: StorageMap<H160, u64>
vesting_total_amounts: StorageMap<H160, u64>
vesting_claimed_amounts: StorageMap<H160, u64>

// Block rewards storage
reward_per_block: StorageItem<u64>
last_claimed_blocks: StorageMap<H160, u32>
rewards_balances: StorageMap<H160, u64>

// Transaction tracking
processed_transactions: StorageMap<H256, bool>
transaction_heights: StorageMap<H256, u32>

// Rate limiting
last_action_times: StorageMap<H160, u64>
action_cooldown: StorageItem<u64>

// Admin controls
owner: StorageItem<H160>
```

### Events

The contract emits the following events:

- `VestingScheduleCreated`: When a new vesting schedule is set up
- `VestingTokensClaimed`: When tokens are claimed from a vesting schedule
- `BlockRewardsClaimed`: When block rewards are claimed
- `TransactionProcessed`: When a transaction is processed with sufficient confirmations

### Key Methods

#### Vesting Methods

- `create_vesting_schedule`: Create a time-locked vesting schedule for a beneficiary
- `vested_amount`: Calculate how many tokens have vested at the current time
- `claimable_vested_amount`: Calculate how many tokens can be claimed
- `claim_vested_tokens`: Claim available vested tokens

#### Block Reward Methods

- `set_reward_per_block`: Set the reward amount per block
- `claim_block_rewards`: Claim rewards for blocks passed since last claim
- `get_claimable_block_rewards`: Calculate claimable rewards without claiming
- `get_rewards_balance`: Get the total claimed rewards balance

#### Transaction Processing Methods

- `process_transaction`: Process a transaction if it has sufficient confirmations
- `is_transaction_processed`: Check if a transaction has been processed
- `verify_transaction_confirmations`: Verify transaction has required confirmations

#### Rate Limiting Methods

- `set_action_cooldown`: Set the cooldown period for rate-limited actions
- `perform_rate_limited_action`: Attempt to perform a rate-limited action
- `get_cooldown_remaining`: Get remaining cooldown time for an action

#### Blockchain Information Methods

- `get_current_blockchain_info`: Get current block index, hash, and timestamp
- `get_block_info`: Get information about a specific block
- `get_transaction_info`: Get information about a transaction

## Usage Examples

### Creating and Claiming from a Vesting Schedule

```rust
// Create a vesting schedule for beneficiary
// 1000 tokens vesting over 30 days (2,592,000 seconds)
let beneficiary = H160::from_str("0x1234567890123456789012345678901234567890").unwrap();
let total_amount = 1000;
let duration = 2592000; // 30 days in seconds

let contract = LedgerExample::new(owner);
contract.create_vesting_schedule(beneficiary, total_amount, duration);

// Later, beneficiary can check vested amount
let vested_amount = contract.vested_amount(beneficiary);

// And claim available tokens
let claimed_amount = contract.claim_vested_tokens();
```

### Block Rewards

```rust
// Set reward per block
let reward_per_block = 10;
contract.set_reward_per_block(reward_per_block);

// Later, user can claim rewards
let claimed_rewards = contract.claim_block_rewards();

// Check balance after claiming
let balance = contract.get_rewards_balance(user_address);
```

### Transaction Processing

```rust
// Process a transaction with 6 confirmations required
let tx_hash = H256::from_str("0x1234567890123456789012345678901234567890123456789012345678901234").unwrap();
let required_confirmations = 6;
let processed = contract.process_transaction(tx_hash, required_confirmations);

// Check if transaction was processed
let is_processed = contract.is_transaction_processed(tx_hash);
```

### Rate-Limited Actions

```rust
// Set cooldown of 1 hour for rate-limited actions
let cooldown = 3600; // 1 hour in seconds
contract.set_action_cooldown(cooldown);

// Attempt to perform the action
let success = contract.perform_rate_limited_action();

// Check remaining cooldown time
let remaining = contract.get_cooldown_remaining(user_address);
```

## Building and Testing

This contract comes with comprehensive unit tests that demonstrate how to test time-based and block-based logic using the Neo testing framework.

To build the contract:

```bash
cargo build --release
```

The tests showcase how to:
1. Test vested amount calculations at different time points
2. Verify block reward accumulation and claiming
3. Validate transaction processing with varying confirmation levels
4. Test rate limiting with time progression
5. Test blockchain data access
6. Test edge cases for blockchain-specific functions

## Limitations and Considerations

When using this contract or the patterns it demonstrates, consider the following:

1. **Block Time Variability**: Neo blocks aren't produced at exact intervals, so time-based logic has some variability
2. **Gas Costs**: Accessing blockchain data consumes gas, so optimize your usage
3. **Reorg Risks**: Very recent blocks can be reorganized, consider higher confirmation counts for critical operations
4. **Storage Costs**: Each storage operation costs gas, so the contract optimizes storage usage

## See Also

- [Ledger API Guide](../../docs/ledger_api_guide.md): Comprehensive documentation on the Neo N3 Ledger API
- [Ledger API Testing Guide](../../docs/ledger_api_testing.md): Guide to testing contracts that use the Ledger API
- [Contract Testing Guide](../../docs/contract_testing_guide.md): General contract testing approaches

## License

This example is part of the Neo Contract Rust framework and is available under the MIT license. 