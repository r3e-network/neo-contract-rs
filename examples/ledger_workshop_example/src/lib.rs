use neo_contract::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Active,
    Completed,
    Refunded,
    Disputed,
    DisputeResolved
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActionType {
    CreateEscrow,
    ReleaseEscrow,
    RefundEscrow,
    OpenDispute
}

// Define event structures for better client interaction
#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    amount: u64,
    release_time: u64
)]
struct EscrowCreated { }

#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    amount: u64,
    release_time: u64
)]
struct EscrowReleased { }

#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    amount: u64
)]
struct EscrowRefunded { }

#[neo_contract::event(
    escrow_id: u64,
    current_block: u32,
    resolution_block: u32
)]
struct DisputeOpened { }

#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    to_sender: bool
)]
struct DisputeResolved { }

#[neo_contract::event(
    escrow_id: u64,
    tx_hash: H256
)]
struct PaymentProcessed { }

#[neo_contract::contract]
pub struct Escrow {
    // Contract owner
    owner: StorageItem<H160>,
    
    // Escrow details storage
    deposits: StorageMap<u64, u64>,  // escrow_id => amount
    senders: StorageMap<u64, H160>,  // escrow_id => sender
    recipients: StorageMap<u64, H160>,  // escrow_id => recipient
    statuses: StorageMap<u64, EscrowStatus>,  // escrow_id => status
    release_times: StorageMap<u64, u64>,  // escrow_id => release timestamp
    
    // Dispute resolution
    dispute_blocks: StorageMap<u64, u32>,  // escrow_id => dispute resolution block height
    
    // Counter for escrow IDs
    next_escrow_id: StorageItem<u64>,
    
    // Processed transactions to prevent replay
    processed_txs: StorageMap<H256, bool>,
    
    // Rate limiting
    last_action_times: StorageMap<H160, u64>,  // address => last action timestamp
    action_cooldown: StorageItem<u64>,  // general cooldown period in seconds
    action_cooldowns: StorageMap<ActionType, u64>, // specific cooldown periods by action type
}

impl Escrow {
    #[constructor]
    pub fn new(owner: H160) -> Self {
        let mut contract = Self {
            owner: StorageItem::new(owner),
            deposits: StorageMap::new(),
            senders: StorageMap::new(),
            recipients: StorageMap::new(),
            statuses: StorageMap::new(),
            release_times: StorageMap::new(),
            dispute_blocks: StorageMap::new(),
            next_escrow_id: StorageItem::new(1),
            processed_txs: StorageMap::new(),
            last_action_times: StorageMap::new(),
            action_cooldown: StorageItem::new(300), // 5 minutes default cooldown
            action_cooldowns: StorageMap::new(),
        };
        
        // Set default cooldowns for specific action types
        contract.action_cooldowns.insert(&ActionType::CreateEscrow, 300u64);  // 5 min
        contract.action_cooldowns.insert(&ActionType::ReleaseEscrow, 60u64);  // 1 min
        contract.action_cooldowns.insert(&ActionType::RefundEscrow, 60u64);   // 1 min
        contract.action_cooldowns.insert(&ActionType::OpenDispute, 3600u64);  // 1 hour
        
        contract
    }
    
    #[method]
    #[no_reentry]
    pub fn create_escrow(&mut self, sender: H160, recipient: H160, amount: u64, lock_duration: u64) -> u64 {
        // Ensure sender is the one calling this method
        assert!(Runtime::check_witness(&sender), "Sender must initiate escrow");
        
        // Check rate limit
        assert!(self.check_rate_limit(&sender, ActionType::CreateEscrow), "Rate limit exceeded");
        
        // Get and increment the escrow ID
        let escrow_id = self.next_escrow_id.get();
        self.next_escrow_id.set(escrow_id + 1);
        
        // Store escrow details
        self.deposits.insert(&escrow_id, amount);
        self.senders.insert(&escrow_id, sender);
        self.recipients.insert(&escrow_id, recipient);
        self.statuses.insert(&escrow_id, EscrowStatus::Active);
        
        // Set release time to current time + lock duration
        let current_time = Ledger::current_timestamp();
        let release_time = current_time + lock_duration;
        self.release_times.insert(&escrow_id, release_time);
        
        // Emit escrow creation event
        EscrowCreated {}.fire(
            &escrow_id,
            &sender,
            &recipient,
            &amount,
            &release_time
        );
        
        escrow_id
    }
    
    #[method]
    #[no_reentry]
    pub fn release_escrow(&mut self, escrow_id: u64) -> bool {
        // Get escrow details
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is still active
            if status != EscrowStatus::Active {
                return false;
            }
            
            // Get sender and recipient
            let sender = self.senders.get(&escrow_id).expect("Sender not found");
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            
            // Check who is calling
            let sender_witness = Runtime::check_witness(&sender);
            let recipient_witness = Runtime::check_witness(&recipient);
            
            // Apply rate limiting to the caller
            if sender_witness {
                assert!(self.check_rate_limit(&sender, ActionType::ReleaseEscrow), "Rate limit exceeded");
            } else if recipient_witness {
                assert!(self.check_rate_limit(&recipient, ActionType::ReleaseEscrow), "Rate limit exceeded");
            }
            
            // Check if time lock has expired
            let release_time = self.release_times.get(&escrow_id).expect("Release time not found");
            let current_time = Ledger::current_timestamp();
            
            if current_time < release_time {
                // If time lock has not expired, only sender can release
                assert!(sender_witness, "Time lock not expired, sender authorization required");
            } else {
                // If time lock has expired, either party can release
                assert!(
                    sender_witness || recipient_witness,
                    "Authorization required"
                );
            }
            
            // Update status
            self.statuses.insert(&escrow_id, EscrowStatus::Completed);
            
            // Emit escrow released event
            let amount = self.deposits.get(&escrow_id).expect("Amount not found");
            
            EscrowReleased {}.fire(
                &escrow_id,
                &sender,
                &recipient,
                &amount,
                &current_time
            );
            
            return true;
        }
        
        false
    }
    
    #[method]
    #[no_reentry]
    pub fn refund_escrow(&mut self, escrow_id: u64) -> bool {
        // Get escrow details
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is still active
            if status != EscrowStatus::Active {
                return false;
            }
            
            // Get recipient
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            
            // Ensure recipient is the one calling this method
            assert!(Runtime::check_witness(&recipient), "Only recipient can refund escrow");
            
            // Apply rate limiting
            assert!(self.check_rate_limit(&recipient, ActionType::RefundEscrow), "Rate limit exceeded");
            
            // Update status
            self.statuses.insert(&escrow_id, EscrowStatus::Refunded);
            
            // Emit escrow refunded event
            let sender = self.senders.get(&escrow_id).expect("Sender not found");
            let amount = self.deposits.get(&escrow_id).expect("Amount not found");
            
            EscrowRefunded {}.fire(
                &escrow_id,
                &sender,
                &recipient,
                &amount
            );
            
            return true;
        }
        
        false
    }
    
    #[method]
    #[no_reentry]
    pub fn open_dispute(&mut self, escrow_id: u64) -> bool {
        // Get escrow details
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is still active
            if status != EscrowStatus::Active {
                return false;
            }
            
            // Get escrow parties
            let sender = self.senders.get(&escrow_id).expect("Sender not found");
            let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
            
            // Ensure either party is opening the dispute
            let is_sender = Runtime::check_witness(&sender);
            let is_recipient = Runtime::check_witness(&recipient);
            assert!(
                is_sender || is_recipient,
                "Only escrow parties can open disputes"
            );
            
            // Apply rate limiting to the caller
            if is_sender {
                assert!(self.check_rate_limit(&sender, ActionType::OpenDispute), "Rate limit exceeded");
            } else {
                assert!(self.check_rate_limit(&recipient, ActionType::OpenDispute), "Rate limit exceeded");
            }
            
            // Set dispute resolution deadline (current block + 100 blocks)
            let current_block = Ledger::current_index();
            let resolution_block = current_block + 100;
            self.dispute_blocks.insert(&escrow_id, resolution_block);
            
            // Update status
            self.statuses.insert(&escrow_id, EscrowStatus::Disputed);
            
            // Emit dispute event
            DisputeOpened {}.fire(
                &escrow_id,
                &current_block,
                &resolution_block
            );
            
            return true;
        }
        
        false
    }
    
    #[method]
    #[no_reentry]
    pub fn resolve_dispute(&mut self, escrow_id: u64, to_sender: bool) -> bool {
        // Ensure owner is resolving
        let owner = self.owner.get();
        assert!(Runtime::check_witness(&owner), "Only owner can resolve disputes");
        
        // Get escrow details
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is in dispute
            if status != EscrowStatus::Disputed {
                return false;
            }
            
            // Check if dispute resolution period has begun
            if let Some(resolution_block) = self.dispute_blocks.get(&escrow_id) {
                let current_block = Ledger::current_index();
                
                // Cannot resolve before resolution block
                assert!(current_block >= resolution_block, "Dispute resolution period not reached");
                
                // Resolve dispute
                if to_sender {
                    self.statuses.insert(&escrow_id, EscrowStatus::Refunded);
                } else {
                    self.statuses.insert(&escrow_id, EscrowStatus::Completed);
                }
                
                // Emit resolution event
                let sender = self.senders.get(&escrow_id).expect("Sender not found");
                let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
                
                DisputeResolved {}.fire(
                    &escrow_id,
                    &sender,
                    &recipient,
                    &to_sender
                );
                
                return true;
            }
        }
        
        false
    }
    
    #[method]
    #[no_reentry]
    pub fn validate_and_process_payment_tx(&mut self, tx_hash: H256, escrow_id: u64) -> bool {
        // Check if transaction has already been processed
        if self.processed_txs.get(&tx_hash) {
            return false;
        }
        
        // Verify transaction exists and has sufficient confirmations
        const REQUIRED_CONFIRMATIONS: u32 = 3;
        
        if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
            let current_height = Ledger::current_index();
            
            // Check if transaction has enough confirmations
            if current_height < tx_height || current_height - tx_height + 1 < REQUIRED_CONFIRMATIONS {
                return false;
            }
            
            // Verify transaction execution succeeded
            if Ledger::get_transaction_vm_state(tx_hash) != 0 {
                return false;
            }
            
            // Get transaction details
            if let Some(tx) = Ledger::get_transaction(tx_hash) {
                // Verify the transaction is related to the escrow
                // In a real contract, you would verify the transaction content more thoroughly
                
                // Mark as processed to prevent replay
                self.processed_txs.insert(&tx_hash, true);
                
                // Update escrow status
                self.statuses.insert(&escrow_id, EscrowStatus::Completed);
                
                // Emit event
                PaymentProcessed {}.fire(
                    &escrow_id,
                    &tx_hash
                );
                
                return true;
            }
        }
        
        false
    }
    
    #[method]
    pub fn set_action_cooldown(&mut self, cooldown_seconds: u64) -> bool {
        // Only owner can set cooldown
        let owner = self.owner.get();
        assert!(Runtime::check_witness(&owner), "Only owner can set cooldown");
        
        self.action_cooldown.set(cooldown_seconds);
        return true;
    }
    
    #[method]
    pub fn set_action_specific_cooldown(&mut self, action_type: ActionType, cooldown_seconds: u64) -> bool {
        // Only owner can set cooldown
        let owner = self.owner.get();
        assert!(Runtime::check_witness(&owner), "Only owner can set cooldown");
        
        self.action_cooldowns.insert(&action_type, cooldown_seconds);
        return true;
    }
    
    #[method]
    #[safe]
    pub fn get_escrow_details(&self, escrow_id: u64) -> Option<(H160, H160, u64, EscrowStatus, u64)> {
        // Check if escrow exists
        if let Some(status) = self.statuses.get(&escrow_id) {
            let sender = self.senders.get(&escrow_id)?;
            let recipient = self.recipients.get(&escrow_id)?;
            let amount = self.deposits.get(&escrow_id)?;
            let release_time = self.release_times.get(&escrow_id)?;
            
            return Some((sender, recipient, amount, status, release_time));
        }
        
        None
    }
    
    #[method]
    #[safe]
    pub fn check_confirmation_status(&self, tx_hash: H256, target_confirmations: u32) -> (bool, u32) {
        if let Some(tx_height) = Ledger::get_transaction_height(tx_hash) {
            let current_height = Ledger::current_index();
            
            if current_height >= tx_height {
                let confirmations = current_height - tx_height + 1;
                return (confirmations >= target_confirmations, confirmations);
            }
        }
        
        (false, 0)
    }
    
    #[method]
    #[safe]
    pub fn estimate_release_block(&self, escrow_id: u64) -> Option<u32> {
        if let Some(release_time) = self.release_times.get(&escrow_id) {
            let current_time = Ledger::current_timestamp();
            
            // If already releasable, return current block
            if current_time >= release_time {
                return Some(Ledger::current_index());
            }
            
            // Calculate time remaining
            let time_remaining = release_time - current_time;
            
            // Estimate using average block time (15 seconds)
            const AVG_BLOCK_TIME: u64 = 15;
            let blocks_remaining = (time_remaining + AVG_BLOCK_TIME - 1) / AVG_BLOCK_TIME; // Ceiling division
            
            return Some(Ledger::current_index() + blocks_remaining as u32);
        }
        
        None
    }
    
    // Private helper methods
    fn check_rate_limit(&mut self, address: &H160, action_type: ActionType) -> bool {
        let current_time = Ledger::current_timestamp();
        
        // Get action-specific cooldown if available, otherwise use default
        let cooldown = if let Some(specific_cooldown) = self.action_cooldowns.get(&action_type) {
            specific_cooldown
        } else {
            self.action_cooldown.get()
        };
        
        if let Some(last_time) = self.last_action_times.get(address) {
            if current_time < last_time + cooldown {
                return false; // Still in cooldown period
            }
        }
        
        // Update last action time
        self.last_action_times.insert(address, current_time);
        true
    }
    
    fn is_escrow_releasable(&self, escrow_id: u64) -> bool {
        if let Some(status) = self.statuses.get(&escrow_id) {
            // Check if escrow is still active
            if status != EscrowStatus::Active {
                return false;
            }
            
            // Check if release time has passed
            if let Some(release_time) = self.release_times.get(&escrow_id) {
                let current_time = Ledger::current_timestamp();
                return current_time >= release_time;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::{TestBuilder, TestingContext};
    
    #[test]
    fn test_time_locked_escrow() {
        // Create test environment
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set initial timestamp
        test.ledger().set_current_timestamp(1000);
        
        // Create contract
        let owner = H160::from_slice(&[1; 20]);
        let sender = H160::from_slice(&[2; 20]);
        let recipient = H160::from_slice(&[3; 20]);
        
        let mut contract = Escrow::new(owner);
        
        // Create escrow with 1 hour lock
        test.as_signer(sender);
        let escrow_id = contract.create_escrow(sender, recipient, 100, 3600);
        
        // Try to release before lock expiration (should fail)
        let result = contract.release_escrow(escrow_id);
        assert!(!result);
        
        // Advance time by 2 hours
        test.ledger().advance_time(7200);
        
        // Now release should succeed
        let result = contract.release_escrow(escrow_id);
        assert!(result);
        
        // Check escrow status
        let (_, _, _, status, _) = contract.get_escrow_details(escrow_id).unwrap();
        assert_eq!(status, EscrowStatus::Completed);
    }
    
    #[test]
    fn test_block_based_dispute() {
        // Create test environment
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set initial block
        test.ledger().set_current_index(1000);
        
        // Create contract
        let owner = H160::from_slice(&[1; 20]);
        let sender = H160::from_slice(&[2; 20]);
        let recipient = H160::from_slice(&[3; 20]);
        
        let mut contract = Escrow::new(owner);
        
        // Create escrow with no time lock
        test.as_signer(sender);
        let escrow_id = contract.create_escrow(sender, recipient, 100, 0);
        
        // Open a dispute
        test.as_signer(recipient);
        let result = contract.open_dispute(escrow_id);
        assert!(result);
        
        // Try to resolve dispute immediately (should fail)
        test.as_signer(owner);
        let result = contract.resolve_dispute(escrow_id, false);
        assert!(!result);
        
        // Advance blocks
        test.ledger().advance_blocks(100);
        
        // Now resolution should succeed
        let result = contract.resolve_dispute(escrow_id, false);
        assert!(result);
        
        // Check escrow status
        let (_, _, _, status, _) = contract.get_escrow_details(escrow_id).unwrap();
        assert_eq!(status, EscrowStatus::Completed);
    }
    
    #[test]
    fn test_transaction_validation() {
        // Create test environment
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set initial block
        test.ledger().set_current_index(1000);
        
        // Create contract
        let owner = H160::from_slice(&[1; 20]);
        let sender = H160::from_slice(&[2; 20]);
        let recipient = H160::from_slice(&[3; 20]);
        
        let mut contract = Escrow::new(owner);
        
        // Create escrow
        test.as_signer(sender);
        let escrow_id = contract.create_escrow(sender, recipient, 100, 3600);
        
        // Create a mock transaction
        let tx_hash = H256::from_slice(&[1; 32]);
        
        // Mock transaction height (block 990)
        test.ledger().mock_transaction_height(tx_hash.clone(), 990);
        
        // Set VM state to HALT (0)
        test.ledger().mock_transaction_vm_state(tx_hash.clone(), 0);
        
        // Try to process with insufficient confirmations (should fail)
        let result = contract.validate_and_process_payment_tx(tx_hash.clone(), escrow_id);
        assert!(!result);
        
        // Advance blocks to get more confirmations
        test.ledger().advance_blocks(10);
        
        // Now processing should succeed
        let result = contract.validate_and_process_payment_tx(tx_hash.clone(), escrow_id);
        assert!(result);
        
        // Check escrow status
        let (_, _, _, status, _) = contract.get_escrow_details(escrow_id).unwrap();
        assert_eq!(status, EscrowStatus::Completed);
        
        // Try to process again (should fail due to replay protection)
        let result = contract.validate_and_process_payment_tx(tx_hash.clone(), escrow_id);
        assert!(!result);
    }
    
    #[test]
    fn test_rate_limiting() {
        // Create test environment
        let mut test = TestBuilder::new()
            .with_mock_ledger()
            .build();
        
        // Set initial timestamp
        test.ledger().set_current_timestamp(1000);
        
        // Create contract
        let owner = H160::from_slice(&[1; 20]);
        let sender = H160::from_slice(&[2; 20]);
        let recipient = H160::from_slice(&[3; 20]);
        
        let mut contract = Escrow::new(owner);
        
        // Set a shorter cooldown for testing
        test.as_signer(owner);
        contract.set_action_specific_cooldown(ActionType::CreateEscrow, 60); // 1 minute
        
        // Create first escrow
        test.as_signer(sender);
        let escrow_id1 = contract.create_escrow(sender, recipient, 100, 3600);
        assert!(escrow_id1 > 0);
        
        // Try to create another escrow immediately (should fail due to rate limiting)
        let escrow_id2 = contract.create_escrow(sender, recipient, 200, 3600);
        assert_eq!(escrow_id2, 0);
        
        // Advance time by more than the cooldown
        test.ledger().advance_time(61);
        
        // Now should be able to create another escrow
        let escrow_id2 = contract.create_escrow(sender, recipient, 200, 3600);
        assert!(escrow_id2 > 0);
    }
} 