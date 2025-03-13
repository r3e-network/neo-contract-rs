#![no_std]

use neo_contract::prelude::*;

/// # Neo N3 Ledger API Example
///
/// This example smart contract demonstrates how to use the Ledger API in Neo N3
/// to access blockchain data such as blocks, transactions, and timestamps.
/// 
/// The contract implements a token vesting mechanism with multiple features:
/// - Time-based linear vesting
/// - Block-based rewards
/// - Transaction validation
/// - Rate limiting
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Neo N3 Ledger API Example")]
#[contract_version("0.1.0")]
pub struct LedgerExample {
    // Vesting schedule storage
    vesting_beneficiaries: StorageMap<H160, bool>,
    vesting_start_times: StorageMap<H160, u64>,
    vesting_end_times: StorageMap<H160, u64>,
    vesting_total_amounts: StorageMap<H160, u64>,
    vesting_claimed_amounts: StorageMap<H160, u64>,
    
    // Block rewards storage
    reward_per_block: StorageItem<u64>,
    last_claimed_blocks: StorageMap<H160, u32>,
    rewards_balances: StorageMap<H160, u64>,
    
    // Transaction tracking
    processed_transactions: StorageMap<H256, bool>,
    transaction_heights: StorageMap<H256, u32>,
    
    // Rate limiting
    last_action_times: StorageMap<H160, u64>,
    action_cooldown: StorageItem<u64>,
    
    // Admin controls
    owner: StorageItem<H160>,
}

// Events
struct VestingScheduleCreated {}

impl VestingScheduleCreated {
    pub fn emit(beneficiary: H160, total_amount: u64, start_time: u64, end_time: u64) {
        // Create event name as ByteString
        let event_name = ByteString::from("VestingScheduleCreated");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        event_data.push(Any::from(beneficiary));
        event_data.push(Any::from(total_amount));
        event_data.push(Any::from(start_time));
        event_data.push(Any::from(end_time));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
}

struct VestingTokensClaimed {}

impl VestingTokensClaimed {
    pub fn emit(beneficiary: H160, amount: u64, timestamp: u64) {
        // Create event name as ByteString
        let event_name = ByteString::from("VestingTokensClaimed");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        event_data.push(Any::from(beneficiary));
        event_data.push(Any::from(amount));
        event_data.push(Any::from(timestamp));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
}

struct BlockRewardsClaimed {}

impl BlockRewardsClaimed {
    pub fn emit(user: H160, amount: u64, from_block: u32, to_block: u32) {
        // Create event name as ByteString
        let event_name = ByteString::from("BlockRewardsClaimed");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        event_data.push(Any::from(user));
        event_data.push(Any::from(amount));
        event_data.push(Any::from(from_block));
        event_data.push(Any::from(to_block));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
}

struct TransactionProcessed {}

impl TransactionProcessed {
    pub fn emit(tx_hash: H256, sender: H160, amount: u64, timestamp: u64) {
        // Create event name as ByteString
        let event_name = ByteString::from("TransactionProcessed");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        event_data.push(Any::from(tx_hash));
        event_data.push(Any::from(sender));
        event_data.push(Any::from(amount));
        event_data.push(Any::from(timestamp));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
}

impl LedgerExample {
    #[constructor]
    pub fn new(owner_address: H160) -> Self {
        Self {
            // Vesting storage
            vesting_beneficiaries: StorageMap::new(),
            vesting_start_times: StorageMap::new(),
            vesting_end_times: StorageMap::new(),
            vesting_total_amounts: StorageMap::new(),
            vesting_claimed_amounts: StorageMap::new(),
            
            // Block rewards storage
            reward_per_block: StorageItem::new(10), // Default 10 tokens per block
            last_claimed_blocks: StorageMap::new(),
            rewards_balances: StorageMap::new(),
            
            // Transaction tracking
            processed_transactions: StorageMap::new(),
            transaction_heights: StorageMap::new(),
            
            // Rate limiting
            last_action_times: StorageMap::new(),
            action_cooldown: StorageItem::new(3600), // Default 1 hour cooldown
            
            // Admin controls
            owner: StorageItem::new(owner_address),
        }
    }
    
    // --- Admin Functions ---
    
    #[method]
    pub fn set_reward_per_block(&mut self, amount: u64) -> bool {
        self.ensure_owner();
        self.reward_per_block.set(amount);
        true
    }
    
    #[method]
    pub fn set_action_cooldown(&mut self, seconds: u64) -> bool {
        self.ensure_owner();
        self.action_cooldown.set(seconds);
        true
    }
    
    // --- Vesting Functions (Time-Based) ---
    
    #[method]
    pub fn create_vesting_schedule(&mut self, beneficiary: H160, total_amount: u64, duration_seconds: u64) -> bool {
        self.ensure_owner();
        
        // Set up vesting schedule
        let current_time = Ledger::current_timestamp();
        let end_time = current_time + duration_seconds;
        
        self.vesting_beneficiaries.insert(&beneficiary, &true);
        self.vesting_start_times.insert(&beneficiary, &current_time);
        self.vesting_end_times.insert(&beneficiary, &end_time);
        self.vesting_total_amounts.insert(&beneficiary, &total_amount);
        self.vesting_claimed_amounts.insert(&beneficiary, &0);
        
        // Emit event
        VestingScheduleCreated::emit(beneficiary, total_amount, current_time, end_time);
        
        true
    }
    
    #[method]
    pub fn vested_amount(&self, beneficiary: H160) -> u64 {
        // Check if beneficiary has a vesting schedule
        if !self.vesting_beneficiaries.get(&beneficiary).unwrap_or(false) {
            return 0;
        }
        
        let current_time = Ledger::current_timestamp();
        let start_time = self.vesting_start_times.get(&beneficiary).unwrap_or(current_time);
        let end_time = self.vesting_end_times.get(&beneficiary).unwrap_or(current_time);
        let total_amount = self.vesting_total_amounts.get(&beneficiary).unwrap_or(0);
        
        // If vesting hasn't started yet
        if current_time < start_time {
            return 0;
        }
        
        // If vesting is complete
        if current_time >= end_time {
            return total_amount;
        }
        
        // Calculate linear vesting
        let time_vested = current_time - start_time;
        let total_vesting_time = end_time - start_time;
        
        total_amount * time_vested / total_vesting_time
    }
    
    #[method]
    pub fn claimable_vested_amount(&self, beneficiary: H160) -> u64 {
        let vested = self.vested_amount(beneficiary);
        let claimed = self.vesting_claimed_amounts.get(&beneficiary).unwrap_or(0);
        
        if vested > claimed {
            vested - claimed
        } else {
            0
        }
    }
    
    #[method]
    pub fn claim_vested_tokens(&mut self) -> u64 {
        let sender = Runtime::current_sender();
        
        // Check if sender is a beneficiary
        if !self.vesting_beneficiaries.get(&sender).unwrap_or(false) {
            return 0;
        }
        
        let claimable = self.claimable_vested_amount(sender);
        if claimable == 0 {
            return 0;
        }
        
        // Update claimed amount
        let claimed = self.vesting_claimed_amounts.get(&sender).unwrap_or(0);
        self.vesting_claimed_amounts.insert(&sender, &(claimed + claimable));
        
        // Emit event
        VestingTokensClaimed::emit(sender, claimable, Ledger::current_timestamp());
        
        // Transferred tokens would be handled here in a real contract
        
        claimable
    }
    
    // --- Block Rewards Functions (Block-Based) ---
    
    #[method]
    pub fn claim_block_rewards(&mut self) -> u64 {
        let sender = Runtime::current_sender();
        let current_block = Ledger::current_index();
        
        // Get the last claimed block for this user
        let last_claimed_block = self.last_claimed_blocks.get(&sender).unwrap_or(current_block);
        
        // Calculate blocks passed
        let blocks_passed = if current_block > last_claimed_block {
            current_block - last_claimed_block
        } else {
            0
        };
        
        if blocks_passed == 0 {
            return 0;
        }
        
        // Calculate reward
        let reward_per_block = self.reward_per_block.get();
        let reward_amount = reward_per_block * blocks_passed as u64;
        
        // Update state
        self.last_claimed_blocks.insert(&sender, &current_block);
        
        let current_balance = self.rewards_balances.get(&sender).unwrap_or(0);
        self.rewards_balances.insert(&sender, &(current_balance + reward_amount));
        
        // Emit event
        BlockRewardsClaimed::emit(sender, reward_amount, last_claimed_block, current_block);
        
        reward_amount
    }
    
    #[method]
    pub fn get_claimable_block_rewards(&self, user: H160) -> u64 {
        let current_block = Ledger::current_index();
        let last_claimed_block = self.last_claimed_blocks.get(&user).unwrap_or(current_block);
        
        // Calculate blocks passed
        let blocks_passed = if current_block > last_claimed_block {
            current_block - last_claimed_block
        } else {
            0
        };
        
        // Calculate reward
        let reward_per_block = self.reward_per_block.get();
        reward_per_block * blocks_passed as u64
    }
    
    #[method]
    pub fn get_rewards_balance(&self, user: H160) -> u64 {
        self.rewards_balances.get(&user).unwrap_or(0)
    }
    
    // --- Transaction Validation Functions ---
    
    #[method]
    pub fn process_transaction(&mut self, tx_hash: H256, required_confirmations: u32) -> bool {
        // Check if already processed
        if self.processed_transactions.get(&tx_hash).unwrap_or(false) {
            return false;
        }
        
        // Get transaction height
        let tx_height = Ledger::get_transaction_height(tx_hash);
        if tx_height == 0 {
            return false; // Transaction doesn't exist
        }
        
        let current_height = Ledger::current_index();
        
        // Calculate confirmations
        if current_height < tx_height {
            return false; // Something is wrong
        }
        
        let confirmations = current_height - tx_height;
        
        // Check if enough confirmations
        if confirmations < required_confirmations {
            // Store the transaction height for later verification
            self.transaction_heights.insert(&tx_hash, &tx_height);
            return false;
        }
        
        // Process the transaction
        self.processed_transactions.insert(&tx_hash, &true);
        
        // Emit event
        TransactionProcessed::emit(tx_hash, Runtime::current_sender(), 0, Ledger::current_timestamp());
        
        true
    }
    
    #[method]
    pub fn is_transaction_processed(&self, tx_hash: H256) -> bool {
        self.processed_transactions.get(&tx_hash).unwrap_or(false)
    }
    
    #[method]
    pub fn verify_transaction_confirmations(&self, tx_hash: H256, required_confirmations: u32) -> bool {
        let tx_height = Ledger::get_transaction_height(tx_hash);
        if tx_height == 0 {
            return false; // Transaction doesn't exist
        }
        
        let current_height = Ledger::current_index();
        let confirmations = current_height - tx_height;
        
        confirmations >= required_confirmations
    }
    
    // --- Rate Limiting Functions (Time-Based) ---
    
    #[method]
    pub fn perform_rate_limited_action(&mut self) -> bool {
        let sender = Runtime::current_sender();
        let current_time = Ledger::current_timestamp();
        let cooldown = self.action_cooldown.get();
        
        // Check last action time
        if let Some(last_time) = self.last_action_times.get(&sender) {
            if current_time < last_time + cooldown {
                return false; // Still in cooldown
            }
        }
        
        // Update last action time
        self.last_action_times.insert(&sender, &current_time);
        
        // Perform the rate-limited action here
        // ...
        
        true
    }
    
    #[method]
    pub fn get_cooldown_remaining(&self, user: H160) -> u64 {
        let current_time = Ledger::current_timestamp();
        let cooldown = self.action_cooldown.get();
        
        if let Some(last_time) = self.last_action_times.get(&user) {
            let next_allowed_time = last_time + cooldown;
            
            if current_time < next_allowed_time {
                return next_allowed_time - current_time;
            }
        }
        
        0 // No cooldown remaining
    }
    
    // --- Blockchain Data Access Functions ---
    
    #[method]
    pub fn get_current_blockchain_info(&self) -> (u32, H256, u64) {
        (
            Ledger::current_index(),
            Ledger::current_hash(),
            Ledger::current_timestamp()
        )
    }
    
    #[method]
    pub fn get_block_info(&self, block_index: u32) -> Option<(u32, H256, u64)> {
        if let Some(block) = Ledger::get_block(block_index) {
            Some((
                block.index,
                block.hash,
                block.timestamp
            ))
        } else {
            None
        }
    }
    
    #[method]
    pub fn get_transaction_info(&self, tx_hash: H256) -> Option<(u32, i32)> {
        let height = Ledger::get_transaction_height(tx_hash);
        if height == 0 {
            return None;
        }
        
        let vm_state = Ledger::get_transaction_vm_state(tx_hash);
        
        Some((height, vm_state))
    }
    
    // --- Helper Functions ---
    
    fn ensure_owner(&self) {
        let sender = Runtime::current_sender();
        let owner = self.owner.get();
        assert!(sender == owner, "Only the owner can perform this action");
    }
}

#[cfg(test)]
mod tests; 