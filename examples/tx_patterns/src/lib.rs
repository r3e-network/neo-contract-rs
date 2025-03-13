//! # Transaction Patterns Example
//!
//! This example demonstrates common transaction patterns for Neo N3 smart contracts,
//! including transaction validation, multi-step transactions, rate limiting, and more.

use neo_contract::prelude::*;

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Transaction Patterns for Neo N3")]
#[contract_version("0.1.0")]
pub mod tx_patterns {
    use super::*;

    // Events
    struct TransactionProcessed {}
    
    impl TransactionProcessed {
        pub fn emit(tx_hash: H256, sender: H160, action: u8, timestamp: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("TransactionProcessed");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            event_data.push(Any::from(tx_hash));
            event_data.push(Any::from(sender));
            event_data.push(Any::from(action));
            event_data.push(Any::from(timestamp));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }

    struct OperationStarted {}
    
    impl OperationStarted {
        pub fn emit(tx_hash: H256, sender: H160, operation_id: u64, timestamp: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("OperationStarted");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            event_data.push(Any::from(tx_hash));
            event_data.push(Any::from(sender));
            event_data.push(Any::from(operation_id));
            event_data.push(Any::from(timestamp));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }

    struct OperationCompleted {}
    
    impl OperationCompleted {
        pub fn emit(tx_hash: H256, sender: H160, operation_id: u64, timestamp: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("OperationCompleted");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            event_data.push(Any::from(tx_hash));
            event_data.push(Any::from(sender));
            event_data.push(Any::from(operation_id));
            event_data.push(Any::from(timestamp));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }

    // Transaction status enum
    #[repr(u8)]
    enum TxStatus {
        None = 0,
        Pending = 1,
        Completed = 2,
        Failed = 3,
    }

    // Actions
    const ACTION_DEPOSIT: u8 = 1;
    const ACTION_WITHDRAW: u8 = 2;
    const ACTION_TRANSFER: u8 = 3;

    // Security constants
    const MIN_CONFIRMATIONS: u32 = 2;
    const RATE_LIMIT_DURATION: u64 = 3600; // 1 hour in seconds
    const MAX_OPERATIONS_PER_PERIOD: u64 = 10;

    // Contract storage
    pub struct TransactionPatterns {
        // Admin
        owner: StorageItem<H160>,
        
        // Transaction tracking
        processed_txs: StorageMap<H256, bool>,
        tx_status: StorageMap<H256, u8>,
        tx_owner: StorageMap<H256, H160>,
        tx_data: StorageMap<H256, Vec<u8>>,
        
        // Operation tracking
        next_operation_id: StorageItem<u64>,
        operations: StorageMap<u64, (H256, u8)>, // (tx_hash, status)
        user_operations: StorageMap<H160, Vec<u64>>, // user -> operation IDs
        
        // Rate limiting
        user_action_count: StorageMap<(H160, u64), u64>, // (user, period) -> count
        last_action_time: StorageMap<H160, u64>, // user -> timestamp
        
        // Balances for demo
        balances: StorageMap<H160, u64>,
    }

    impl TransactionPatterns {
        #[constructor]
        pub fn new() -> Self {
            let owner = Runtime::current_sender();
            Self {
                owner: StorageItem::new(owner),
                processed_txs: StorageMap::new(),
                tx_status: StorageMap::new(),
                tx_owner: StorageMap::new(),
                tx_data: StorageMap::new(),
                next_operation_id: StorageItem::new(1),
                operations: StorageMap::new(),
                user_operations: StorageMap::new(),
                user_action_count: StorageMap::new(),
                last_action_time: StorageMap::new(),
                balances: StorageMap::new(),
            }
        }

        // ===== Transaction Confirmation Validation =====
        
        /// Checks if a transaction has reached the required number of confirmations
        #[method]
        pub fn is_confirmed(&self, tx_hash: H256) -> bool {
            self.check_transaction_confirmations(tx_hash, MIN_CONFIRMATIONS)
        }
        
        /// Validate transaction confirmations with custom confirmation requirement
        #[method]
        pub fn check_transaction_confirmations(&self, tx_hash: H256, required_confirmations: u32) -> bool {
            // Get the transaction height
            let tx_height = Ledger::get_transaction_height(tx_hash);
            
            // If transaction height is 0, it means the transaction isn't found or confirmed
            if tx_height == 0 {
                return false;
            }
            
            // Get the current block height
            let current_height = Ledger::current_index();
            
            // Calculate confirmations
            let confirmations = current_height - tx_height + 1;
            
            // Check if we have enough confirmations
            confirmations >= required_confirmations
        }

        // ===== Rate Limiting =====
        
        /// Check if an action is allowed under rate limiting rules
        #[method]
        pub fn is_action_allowed(&self, sender: H160, action: u8) -> bool {
            // Get current time
            let current_time = Ledger::current_timestamp();
            
            // Calculate time period (hour bucket)
            let period = current_time / RATE_LIMIT_DURATION;
            
            // Check the action count for this period
            let count = self.user_action_count.get(&(sender, period)).unwrap_or(0);
            
            // Check if under the limit
            count < MAX_OPERATIONS_PER_PERIOD
        }
        
        /// Record an action for rate limiting purposes
        fn record_action(&mut self, sender: H160, action: u8) {
            // Get current time
            let current_time = Ledger::current_timestamp();
            
            // Calculate time period (hour bucket)
            let period = current_time / RATE_LIMIT_DURATION;
            
            // Update the action count
            let count = self.user_action_count.get(&(sender, period)).unwrap_or(0);
            self.user_action_count.insert(&(sender, period), &(count + 1));
            
            // Update last action time
            self.last_action_time.insert(&sender, &current_time);
        }

        // ===== Hash-based Transaction Tracking =====
        
        /// Check if a transaction has been processed
        #[method]
        pub fn is_processed(&self, tx_hash: H256) -> bool {
            self.processed_txs.get(&tx_hash).unwrap_or(false)
        }
        
        /// Mark a transaction as processed
        fn mark_as_processed(&mut self, tx_hash: H256) {
            self.processed_txs.insert(&tx_hash, &true);
        }
        
        /// Process a deposit with transaction tracking
        #[method]
        pub fn process_deposit(&mut self, tx_hash: H256, amount: u64) -> bool {
            // Check that the sender is calling
            let sender = Runtime::current_sender();
            assert!(Runtime::check_witness(&sender), "Unauthorized");
            
            // Check if transaction already processed
            assert!(!self.is_processed(tx_hash), "Transaction already processed");
            
            // Check confirmation count
            assert!(self.check_transaction_confirmations(tx_hash, MIN_CONFIRMATIONS), 
                "Not enough confirmations");
            
            // Check rate limiting
            assert!(self.is_action_allowed(sender, ACTION_DEPOSIT), "Rate limit exceeded");
            
            // Process the deposit
            let balance = self.balances.get(&sender).unwrap_or(0);
            self.balances.insert(&sender, &(balance + amount));
            
            // Record the action for rate limiting
            self.record_action(sender, ACTION_DEPOSIT);
            
            // Mark transaction as processed
            self.mark_as_processed(tx_hash);
            
            // Emit event
            TransactionProcessed::emit(tx_hash, sender, ACTION_DEPOSIT, Ledger::current_timestamp());
            
            true
        }

        // ===== Multi-step Transactions =====
        
        /// Start a complex operation (first step)
        #[method]
        pub fn start_operation(&mut self) -> u64 {
            let sender = Runtime::current_sender();
            assert!(Runtime::check_witness(&sender), "Unauthorized");
            
            // Get current transaction hash
            let tx_hash = Runtime::get_entry_script_hash();
            
            // Check if transaction already processed
            assert!(!self.is_processed(tx_hash), "Transaction already processed");
            
            // Check rate limiting
            assert!(self.is_action_allowed(sender, ACTION_TRANSFER), "Rate limit exceeded");
            
            // Get next operation ID
            let operation_id = self.next_operation_id.get();
            self.next_operation_id.set(operation_id + 1);
            
            // Store operation info
            self.operations.insert(&operation_id, &(tx_hash, TxStatus::Pending as u8));
            
            // Store transaction owner
            self.tx_owner.insert(&tx_hash, &sender);
            
            // Update user operations
            let mut user_ops = self.user_operations.get(&sender).unwrap_or_default();
            user_ops.push(operation_id);
            self.user_operations.insert(&sender, &user_ops);
            
            // Record action for rate limiting
            self.record_action(sender, ACTION_TRANSFER);
            
            // Mark transaction as processed
            self.mark_as_processed(tx_hash);
            
            // Emit event
            OperationStarted::emit(tx_hash, sender, operation_id, Ledger::current_timestamp());
            
            operation_id
        }
        
        /// Complete a complex operation (second step)
        #[method]
        pub fn complete_operation(&mut self, operation_id: u64) -> bool {
            let sender = Runtime::current_sender();
            assert!(Runtime::check_witness(&sender), "Unauthorized");
            
            // Get current transaction hash
            let tx_hash = Runtime::get_entry_script_hash();
            
            // Check if transaction already processed
            assert!(!self.is_processed(tx_hash), "Transaction already processed");
            
            // Check if operation exists
            let (original_tx, status) = self.operations.get(&operation_id)
                .expect("Operation not found");
            
            // Check if operation is pending
            assert!(status == TxStatus::Pending as u8, "Operation not in pending state");
            
            // Check if caller is the operation owner
            let owner = self.tx_owner.get(&original_tx).expect("Transaction not found");
            assert!(sender == owner, "Not the operation owner");
            
            // Update operation status
            self.operations.insert(&operation_id, &(original_tx, TxStatus::Completed as u8));
            
            // Mark transaction as processed
            self.mark_as_processed(tx_hash);
            
            // Emit event
            OperationCompleted::emit(tx_hash, sender, operation_id, Ledger::current_timestamp());
            
            true
        }

        // ===== Utility methods =====
        
        /// Get the operations for a user
        #[method]
        pub fn get_user_operations(&self, user: H160) -> Vec<u64> {
            self.user_operations.get(&user).unwrap_or_default()
        }
        
        /// Get an operation's status
        #[method]
        pub fn get_operation_status(&self, operation_id: u64) -> u8 {
            let (_, status) = self.operations.get(&operation_id)
                .expect("Operation not found");
            status
        }
        
        /// Get user balance
        #[method]
        pub fn get_balance(&self, user: H160) -> u64 {
            self.balances.get(&user).unwrap_or(0)
        }
        
        /// Get action count for the current period
        #[method]
        pub fn get_action_count(&self, user: H160) -> u64 {
            let current_time = Ledger::current_timestamp();
            let period = current_time / RATE_LIMIT_DURATION;
            
            self.user_action_count.get(&(user, period)).unwrap_or(0)
        }
        
        /// Get remaining actions for the current period
        #[method]
        pub fn get_remaining_actions(&self, user: H160) -> u64 {
            let count = self.get_action_count(user);
            if count >= MAX_OPERATIONS_PER_PERIOD {
                0
            } else {
                MAX_OPERATIONS_PER_PERIOD - count
            }
        }
    }
} 