#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// # Secure Vault Contract for Neo N3
/// 
/// This contract implements a secure token storage vault with advanced security features:
/// 1. Emergency freeze functionality
/// 2. Withdrawal limits and fees
/// 3. Configurable user vaults
/// 4. Comprehensive audit trail
///
/// ## Event Handling
/// This contract uses the standardized Neo N3 event pattern:
/// - Events are defined as structs with the `#[event]` attribute
/// - Event parameters that need to be indexed for efficient filtering use the `#[index]` attribute
/// - Events are emitted using the `EventName::emit(params)` method
///
/// This approach is automatically provided by the neo-contract framework and is the
/// recommended way to handle events in Neo N3 smart contracts.

/// A secure token vault implementation that demonstrates advanced storage patterns
/// and security best practices for Neo N3 smart contracts
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Secure Token Vault System for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod secure_vault {
    use neo_contract::prelude::*;
    use neo_contract::error::{Error, Result};
    use alloc::vec::Vec;
    
    // Events
    struct Deposit {
        #[index]
        user: H160,
        #[index]
        token: H160,
        amount: u64,
        timestamp: u64,
    }
    
    struct Withdrawal {
        #[index]
        user: H160,
        #[index]
        token: H160,
        amount: u64,
        fee: u64,
        timestamp: u64,
    }
    
    struct VaultLocked {
        #[index]
        user: H160,
        until: u64,
    }
    
    struct VaultUnlocked {
        #[index]
        user: H160,
        timestamp: u64,
    }
    
    struct EmergencyFreeze {
        admin: H160,
        reason: u8,
        timestamp: u64,
    }
    
    // Enums for typesafe state tracking
    enum WithdrawalStatus {
        Pending = 0,
        Approved = 1,
        Rejected = 2,
        Executed = 3,
        Expired = 4,
    }
    
    enum FreezeReason {
        None = 0,
        SecurityBreach = 1,
        RegulatoryAction = 2,
        MaintenanceRequired = 3,
        AdminInitiated = 4,
    }
    
    // Data structures for complex storage
    #[derive(Encode, Decode, Debug, Clone)]
    struct WithdrawalRequest {
        id: u64,
        user: H160,
        token: H160,
        amount: u64,
        requested_at: u64,
        status: u8, // WithdrawalStatus
        approvals: Vec<H160>,
        executed_at: u64,
        expiration: u64,
    }
    
    #[derive(Encode, Decode, Debug, Clone)]
    struct UserConfig {
        // Security settings
        has_time_lock: bool,
        time_lock_duration: u64,
        require_approvals: bool,
        min_approvals: u8,
        approved_addresses: Vec<H160>,
        max_daily_withdrawal: u64,
        withdrawal_delay: u64,
        
        // User vault state
        is_locked: bool,
        locked_until: u64,
        daily_withdrawal_count: Map<u64, u64>, // day -> amount
        last_activity: u64,
    }
    
    #[derive(Encode, Decode, Debug, Clone)]
    struct TokenInfo {
        name: String,
        symbol: String,
        decimals: u8,
        total_deposits: u64,
        withdrawal_fee: u64, // Basis points (1/100 of a percent)
    }
    
    // Storage layout
    #[storage]
    struct SecureVault {
        // Contract admin management - using storage prefixing
        admin: Item<H160>,
        pending_admin: Item<H160>,
        admin_transfer_deadline: Item<u64>,
        
        // Global contract config
        paused: Item<bool>,
        freeze_status: Item<u8>, // FreezeReason
        frozen_until: Item<u64>,
        is_in_maintenance: Item<bool>,
        
        // Fee settings
        treasury: Item<H160>,
        withdrawal_fee_basis_points: Item<u64>,
        
        // Supported tokens - using storage maps with clear prefixes
        supported_tokens: Map<H160, TokenInfo>,
        token_list: Map<u64, H160>, // index -> token address
        token_count: Item<u64>,
        
        // Balances - using composite keys for efficient storage
        // (user, token) -> balance
        balances: Map<(H160, H160), u64>,
        
        // Deposits for a token - enables token-specific queries
        // token -> total deposits
        token_deposits: Map<H160, u64>,
        
        // User account configurations
        user_configs: Map<H160, UserConfig>,
        
        // Withdrawal requests - complex data with clear organization
        withdrawal_requests: Map<u64, WithdrawalRequest>,
        next_withdrawal_id: Item<u64>,
        user_withdrawal_requests: Map<H160, Vec<u64>>, // user -> withdrawal request IDs
        
        // Security rate limiting
        failed_attempts: Map<H160, u64>,
        last_failed_attempt: Map<H160, u64>,
        
        // Events and transaction tracking (optional, for easier client consumption)
        event_counter: Item<u64>,
        transaction_log: Map<u64, (H160, H160, u64, u64, u8)>, // ID -> (user, token, amount, timestamp, action)
    }
    
    // Event definitions
    /// Event emitted when the vault is frozen for security reasons
    #[event]
    struct EmergencyFreeze {
        #[index]
        admin: H160,
        reason: u8,
        timestamp: u64,
    }
    
    /// Event emitted when a vault is locked
    #[event]
    struct VaultLocked {
        #[index]
        user: H160,
        until: u64,
    }
    
    /// Event emitted when a vault is unlocked
    #[event]
    struct VaultUnlocked {
        #[index]
        user: H160,
        timestamp: u64,
    }
    
    /// Event emitted when tokens are deposited
    #[event]
    struct Deposit {
        #[index]
        user: H160,
        #[index]
        token: H160,
        amount: u64,
        timestamp: u64,
    }
    
    /// Event emitted when tokens are withdrawn
    #[event]
    struct Withdrawal {
        #[index]
        user: H160,
        #[index]
        token: H160,
        amount: u64,
        fee: u64,
        timestamp: u64,
    }
    
    #[neo_contract::manifest]
    impl SecureVault {
        /// Contract constructor
        #[constructor]
        fn new(admin: H160, treasury: H160) -> Self {
            // Validate inputs
            assert!(admin != H160::zero(), "Invalid admin address");
            assert!(treasury != H160::zero(), "Invalid treasury address");
            
            // Create contract instance with storage prefixes
            let mut contract = Self {
                // Admin
                admin: Item::new(b"admin"),
                pending_admin: Item::new(b"admin.pending"),
                admin_transfer_deadline: Item::new(b"admin.transfer.deadline"),
                
                // Contract config
                paused: Item::new(b"state.paused"),
                freeze_status: Item::new(b"state.freeze"),
                frozen_until: Item::new(b"state.freeze.until"),
                is_in_maintenance: Item::new(b"state.maintenance"),
                
                // Fee settings
                treasury: Item::new(b"fee.treasury"),
                withdrawal_fee_basis_points: Item::new(b"fee.withdrawal"),
                
                // Tokens
                supported_tokens: Map::new(b"tokens"),
                token_list: Map::new(b"tokens.list"),
                token_count: Item::new(b"tokens.count"),
                
                // User data with composite keys
                balances: Map::new(b"balances"),
                token_deposits: Map::new(b"deposits"),
                user_configs: Map::new(b"users.config"),
                
                // Withdrawal requests - organized by prefix
                withdrawal_requests: Map::new(b"withdrawals"),
                next_withdrawal_id: Item::new(b"withdrawals.next_id"),
                user_withdrawal_requests: Map::new(b"withdrawals.by_user"),
                
                // Security
                failed_attempts: Map::new(b"security.failed"),
                last_failed_attempt: Map::new(b"security.failed.last"),
                
                // Transaction logging (optional)
                event_counter: Item::new(b"events.counter"),
                transaction_log: Map::new(b"transactions.log"),
            };
            
            // Initialize state
            contract.admin.set(admin);
            contract.treasury.set(treasury);
            contract.paused.set(false);
            contract.freeze_status.set(0); // No freeze
            contract.frozen_until.set(0);
            contract.is_in_maintenance.set(false);
            contract.withdrawal_fee_basis_points.set(25); // 0.25% default fee
            contract.token_count.set(0);
            contract.next_withdrawal_id.set(1);
            contract.event_counter.set(1);
            
            contract
        }
        
        //
        // Admin functions
        //
        
        /// Add a supported token
        #[method]
        fn add_token(&mut self, token: H160, name: String, symbol: String, decimals: u8) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Check token doesn't already exist
            assert!(!self.supported_tokens.contains_key(&token), "Token already supported");
            assert!(token != H160::zero(), "Invalid token address");
            
            // Create token info
            let token_info = TokenInfo {
                name,
                symbol,
                decimals,
                total_deposits: 0,
                withdrawal_fee: self.withdrawal_fee_basis_points.get(), // Default to contract fee
            };
            
            // Store token info
            self.supported_tokens.insert(&token, &token_info);
            
            // Update token list (for enumeration)
            let count = self.token_count.get();
            self.token_list.insert(&count, &token);
            self.token_count.set(count + 1);
            
            // Emit event
            EmergencyFreeze::emit(
                self.admin.get(),
                0,
                Ledger::current_timestamp()
            );
            
            true
        }
        
        /// Set token-specific withdrawal fee
        #[method]
        fn set_token_fee(&mut self, token: H160, fee_basis_points: u64) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Verify token exists
            assert!(self.supported_tokens.contains_key(&token), "Token not supported");
            
            // Verify fee is reasonable (max 5%)
            assert!(fee_basis_points <= 500, "Fee too high");
            
            // Update token info
            if let Some(mut token_info) = self.supported_tokens.get(&token) {
                token_info.withdrawal_fee = fee_basis_points;
                self.supported_tokens.insert(&token, &token_info);
                
                // Log event
                self.log_transaction(self.admin.get(), token, fee_basis_points, Ledger::current_timestamp(), 2); // 2 = set fee
                
                return true;
            }
            
            false
        }
        
        /// Set global withdrawal fee (affects new tokens)
        #[method]
        fn set_global_fee(&mut self, fee_basis_points: u64) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Verify fee is reasonable (max 5%)
            assert!(fee_basis_points <= 500, "Fee too high");
            
            // Update fee
            self.withdrawal_fee_basis_points.set(fee_basis_points);
            
            // Log event
            self.log_transaction(self.admin.get(), H160::zero(), fee_basis_points, Ledger::current_timestamp(), 3); // 3 = set global fee
            
            true
        }
        
        /// Emergency freeze
        #[method]
        fn emergency_freeze(&mut self, reason: u8, duration: u64) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Set freeze status
            self.freeze_status.set(reason);
            self.frozen_until.set(Ledger::current_timestamp() + duration);
            
            // Emit event
            EmergencyFreeze::emit(
                self.admin.get(),
                reason,
                Ledger::current_timestamp()
            );
            
            // Log transaction
            self.log_transaction(self.admin.get(), H160::zero(), duration, Ledger::current_timestamp(), 4); // 4 = freeze
            
            true
        }
        
        /// Lift emergency freeze
        #[method]
        fn lift_freeze(&mut self) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Check if frozen
            assert!(self.freeze_status.get() > 0, "Not frozen");
            
            // Lift freeze
            self.freeze_status.set(0);
            self.frozen_until.set(0);
            
            // Log transaction
            self.log_transaction(self.admin.get(), H160::zero(), 0, Ledger::current_timestamp(), 5); // 5 = lift freeze
            
            true
        }
        
        /// Enable/disable maintenance mode
        #[method]
        fn set_maintenance(&mut self, maintenance: bool) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Set maintenance status
            self.is_in_maintenance.set(maintenance);
            
            // Log transaction
            self.log_transaction(self.admin.get(), H160::zero(), if maintenance { 1 } else { 0 }, Ledger::current_timestamp(), 6); // 6 = set maintenance
            
            true
        }
        
        /// Start admin transfer process (two-phase for security)
        #[method]
        fn transfer_admin(&mut self, new_admin: H160) -> bool {
            // Verify admin
            self.ensure_admin();
            
            // Validate new admin
            assert!(new_admin != H160::zero(), "Invalid admin address");
            
            // Set pending admin and deadline (24 hours to accept)
            self.pending_admin.set(new_admin);
            self.admin_transfer_deadline.set(Ledger::current_timestamp() + 86400);
            
            // Log transaction
            self.log_transaction(self.admin.get(), new_admin, 0, Ledger::current_timestamp(), 7); // 7 = admin transfer started
            
            true
        }
        
        /// Accept admin transfer (must be called by pending admin)
        #[method]
        fn accept_admin(&mut self) -> bool {
            // Get sender
            let sender = Runtime::current_sender();
            
            // Verify sender is pending admin
            assert!(sender == self.pending_admin.get(), "Not pending admin");
            
            // Verify deadline hasn't passed
            assert!(Ledger::current_timestamp() < self.admin_transfer_deadline.get(), "Transfer expired");
            
            // Transfer admin
            let old_admin = self.admin.get();
            self.admin.set(sender);
            self.pending_admin.set(H160::zero());
            self.admin_transfer_deadline.set(0);
            
            // Log transaction
            self.log_transaction(old_admin, sender, 0, Ledger::current_timestamp(), 8); // 8 = admin transfer completed
            
            true
        }
        
        //
        // User vault setup
        //
        
        /// Initialize user vault with default configuration
        #[method]
        fn initialize_vault(&mut self) -> bool {
            // Check not frozen
            self.ensure_not_frozen();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Check vault not already initialized
            assert!(!self.user_configs.contains_key(&sender), "Vault already initialized");
            
            // Create default config
            let config = UserConfig {
                // Default security settings
                has_time_lock: false,
                time_lock_duration: 0,
                require_approvals: false,
                min_approvals: 0,
                approved_addresses: Vec::new(),
                max_daily_withdrawal: u64::MAX, // No limit by default
                withdrawal_delay: 0,
                
                // Initial state
                is_locked: false,
                locked_until: 0,
                daily_withdrawal_count: Map::new(),
                last_activity: Ledger::current_timestamp(),
            };
            
            // Store config
            self.user_configs.insert(&sender, &config);
            
            // Emit event
            VaultLocked::emit(
                sender,
                0
            );
            
            // Log transaction
            self.log_transaction(sender, H160::zero(), 0, Ledger::current_timestamp(), 9); // 9 = vault initialized
            
            true
        }
        
        /// Configure vault security settings
        #[method]
        fn configure_vault(&mut self, 
            has_time_lock: bool, 
            time_lock_duration: u64,
            require_approvals: bool,
            min_approvals: u8,
            approved_addresses: Vec<H160>,
            max_daily_withdrawal: u64,
            withdrawal_delay: u64
        ) -> bool {
            // Check not frozen
            self.ensure_not_frozen();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Get current config
            let mut config = self.user_configs.get(&sender).expect("Vault not initialized");
            
            // Validate config
            if require_approvals {
                assert!(!approved_addresses.is_empty(), "Must specify approvers");
                assert!(min_approvals > 0 && min_approvals as usize <= approved_addresses.len(), "Invalid approval threshold");
            }
            
            // Update config
            config.has_time_lock = has_time_lock;
            config.time_lock_duration = time_lock_duration;
            config.require_approvals = require_approvals;
            config.min_approvals = min_approvals;
            config.approved_addresses = approved_addresses;
            config.max_daily_withdrawal = max_daily_withdrawal;
            config.withdrawal_delay = withdrawal_delay;
            config.last_activity = Ledger::current_timestamp();
            
            // Store updated config
            self.user_configs.insert(&sender, &config);
            
            // Emit event
            VaultUnlocked::emit(
                sender,
                Ledger::current_timestamp()
            );
            
            // Log transaction
            self.log_transaction(sender, H160::zero(), 0, Ledger::current_timestamp(), 10); // 10 = vault configured
            
            true
        }
        
        /// Lock vault (time lock)
        #[method]
        fn lock_vault(&mut self, duration: u64) -> bool {
            // Check not frozen
            self.ensure_not_frozen();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Get current config
            let mut config = self.user_configs.get(&sender).expect("Vault not initialized");
            
            // Calculate lock duration
            let lock_duration = if config.has_time_lock {
                config.time_lock_duration.max(duration)
            } else {
                duration
            };
            
            // Update config
            config.is_locked = true;
            config.locked_until = Ledger::current_timestamp() + lock_duration;
            config.last_activity = Ledger::current_timestamp();
            
            // Store updated config
            self.user_configs.insert(&sender, &config);
            
            // Emit event
            VaultLocked::emit(
                sender,
                config.locked_until
            );
            
            // Log transaction
            self.log_transaction(sender, H160::zero(), lock_duration, Ledger::current_timestamp(), 11); // 11 = vault locked
            
            true
        }
        
        /// Unlock vault (if time lock expired)
        #[method]
        fn unlock_vault(&mut self) -> bool {
            // Check not frozen
            self.ensure_not_frozen();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Get current config
            let mut config = self.user_configs.get(&sender).expect("Vault not initialized");
            
            // Check if vault is locked
            assert!(config.is_locked, "Vault not locked");
            
            // Check if lock period has passed
            assert!(Ledger::current_timestamp() >= config.locked_until, "Vault still locked");
            
            // Update config
            config.is_locked = false;
            config.locked_until = 0;
            config.last_activity = Ledger::current_timestamp();
            
            // Store updated config
            self.user_configs.insert(&sender, &config);
            
            // Emit event
            VaultUnlocked::emit(
                sender,
                Ledger::current_timestamp()
            );
            
            // Log transaction
            self.log_transaction(sender, H160::zero(), 0, Ledger::current_timestamp(), 12); // 12 = vault unlocked
            
            true
        }
        
        //
        // Token deposit/withdrawal
        //
        
        /// Deposit tokens to the vault
        #[method]
        fn deposit(&mut self, token: H160, amount: u64) -> bool {
            // Check not frozen or paused
            self.ensure_operational();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Check token is supported
            assert!(self.supported_tokens.contains_key(&token), "Token not supported");
            assert!(amount > 0, "Amount must be positive");
            
            // Initialize vault if needed
            if !self.user_configs.contains_key(&sender) {
                self.initialize_vault();
            }
            
            // Transfer tokens from sender to contract
            // This is a simplified version - real implementation would use contract calls
            let success = self.transfer_from_sender(token, amount);
            assert!(success, "Token transfer failed");
            
            // Update balances - using composite key for efficient storage
            let key = (sender, token);
            let balance = self.balances.get(&key).unwrap_or(0);
            self.balances.insert(&key, &(balance + amount));
            
            // Update token deposits
            let total_deposits = self.token_deposits.get(&token).unwrap_or(0);
            self.token_deposits.insert(&token, &(total_deposits + amount));
            
            // Update token info
            if let Some(mut token_info) = self.supported_tokens.get(&token) {
                token_info.total_deposits += amount;
                self.supported_tokens.insert(&token, &token_info);
            }
            
            // Update user activity timestamp
            if let Some(mut config) = self.user_configs.get(&sender) {
                config.last_activity = Ledger::current_timestamp();
                self.user_configs.insert(&sender, &config);
            }
            
            // Emit event
            let timestamp = Ledger::current_timestamp();
            Deposit::emit(
                sender,
                token,
                amount,
                timestamp
            );
            
            // Log transaction
            self.log_transaction(sender, token, amount, timestamp, 13); // 13 = deposit
            
            true
        }
        
        /// Request withdrawal (might require approvals or have time delay)
        #[method]
        fn request_withdrawal(&mut self, token: H160, amount: u64) -> u64 {
            // Check not frozen or paused
            self.ensure_operational();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Verify token and amount
            assert!(self.supported_tokens.contains_key(&token), "Token not supported");
            assert!(amount > 0, "Amount must be positive");
            
            // Check vault is initialized
            let config = self.user_configs.get(&sender).expect("Vault not initialized");
            
            // Check vault is not locked
            assert!(!config.is_locked, "Vault is locked");
            
            // Check user has sufficient balance
            let key = (sender, token);
            let balance = self.balances.get(&key).unwrap_or(0);
            assert!(balance >= amount, "Insufficient balance");
            
            // Check daily withdrawal limit
            let current_day = Ledger::current_timestamp() / 86400; // Days since epoch
            let daily_withdrawal = config.daily_withdrawal_count.get(&current_day).unwrap_or(0);
            assert!(daily_withdrawal + amount <= config.max_daily_withdrawal, "Daily withdrawal limit exceeded");
            
            // Create withdrawal request
            let request_id = self.next_withdrawal_id.get();
            self.next_withdrawal_id.set(request_id + 1);
            
            let now = Ledger::current_timestamp();
            let request = WithdrawalRequest {
                id: request_id,
                user: sender,
                token: token,
                amount: amount,
                requested_at: now,
                status: WithdrawalStatus::Pending as u8,
                approvals: Vec::new(),
                executed_at: 0,
                expiration: now + 86400, // 24 hour expiration by default
            };
            
            // Store withdrawal request
            self.withdrawal_requests.insert(&request_id, &request);
            
            // Add to user's withdrawal requests
            let mut user_requests = self.user_withdrawal_requests.get(&sender).unwrap_or_default();
            user_requests.push(request_id);
            self.user_withdrawal_requests.insert(&sender, &user_requests);
            
            // If no approvals required and no delay, execute immediately
            if !config.require_approvals && config.withdrawal_delay == 0 {
                self.execute_withdrawal(request_id);
            }
            
            // Log transaction
            self.log_transaction(sender, token, amount, now, 14); // 14 = withdrawal requested
            
            request_id
        }
        
        /// Approve a withdrawal request (for multi-signature)
        #[method]
        fn approve_withdrawal(&mut self, request_id: u64) -> bool {
            // Check not frozen or paused
            self.ensure_operational();
            
            // Get sender
            let sender = Runtime::current_sender();
            
            // Get withdrawal request
            let mut request = self.withdrawal_requests.get(&request_id).expect("Request not found");
            
            // Get user config
            let config = self.user_configs.get(&request.user).expect("Vault not initialized");
            
            // Check sender is an approved address
            assert!(config.approved_addresses.contains(&sender), "Not an approved address");
            
            // Check request is pending
            assert!(request.status == WithdrawalStatus::Pending as u8, "Request not pending");
            
            // Check not already approved by this address
            assert!(!request.approvals.contains(&sender), "Already approved");
            
            // Add approval
            request.approvals.push(sender);
            
            // Update request
            self.withdrawal_requests.insert(&request_id, &request);
            
            // Check if enough approvals to execute
            if config.require_approvals && request.approvals.len() >= config.min_approvals as usize {
                // Mark as approved
                request.status = WithdrawalStatus::Approved as u8;
                self.withdrawal_requests.insert(&request_id, &request);
                
                // Check if withdrawal delay has passed
                if config.withdrawal_delay == 0 || 
                   request.requested_at + config.withdrawal_delay <= Ledger::current_timestamp() {
                    // Execute withdrawal
                    self.execute_withdrawal(request_id);
                }
            }
            
            // Log transaction
            self.log_transaction(sender, request.token, request.amount, Ledger::current_timestamp(), 15); // 15 = withdrawal approved
            
            true
        }
        
        /// Execute a delayed withdrawal that has passed its delay period
        #[method]
        fn execute_delayed_withdrawal(&mut self, request_id: u64) -> bool {
            // Check not frozen or paused
            self.ensure_operational();
            
            // Get withdrawal request
            let request = self.withdrawal_requests.get(&request_id).expect("Request not found");
            
            // Check request status
            let status = request.status;
            assert!(status == WithdrawalStatus::Pending as u8 || status == WithdrawalStatus::Approved as u8, 
                   "Invalid request status");
            
            // Get user config
            let config = self.user_configs.get(&request.user).expect("Vault not initialized");
            
            // Check sender
            let sender = Runtime::current_sender();
            assert!(sender == request.user || config.approved_addresses.contains(&sender), 
                   "Not authorized");
            
            // If approvals required, check if approved
            if config.require_approvals && status == WithdrawalStatus::Pending as u8 {
                assert!(request.approvals.len() >= config.min_approvals as usize, "Insufficient approvals");
            }
            
            // Check if delay period has passed
            assert!(request.requested_at + config.withdrawal_delay <= Ledger::current_timestamp(), 
                   "Withdrawal delay not passed");
            
            // Execute withdrawal
            self.execute_withdrawal(request_id);
            
            true
        }
        
        /// Internal method to execute withdrawal
        fn execute_withdrawal(&mut self, request_id: u64) -> bool {
            // Get withdrawal request
            let mut request = self.withdrawal_requests.get(&request_id).expect("Request not found");
            
            // Check request is pending or approved
            assert!(request.status == WithdrawalStatus::Pending as u8 || 
                   request.status == WithdrawalStatus::Approved as u8, 
                   "Invalid request status");
            
            // Check if request is expired
            if request.expiration < Ledger::current_timestamp() {
                request.status = WithdrawalStatus::Expired as u8;
                self.withdrawal_requests.insert(&request_id, &request);
                return false;
            }
            
            // Get user balance
            let key = (request.user, request.token);
            let balance = self.balances.get(&key).unwrap_or(0);
            
            // Verify sufficient balance
            if balance < request.amount {
                request.status = WithdrawalStatus::Rejected as u8;
                self.withdrawal_requests.insert(&request_id, &request);
                return false;
            }
            
            // Get token info for fee calculation
            let token_info = self.supported_tokens.get(&request.token).expect("Token not found");
            
            // Calculate fee
            let fee = (request.amount * token_info.withdrawal_fee) / 10000;
            let amount_after_fee = request.amount - fee;
            
            // Update user balance
            self.balances.insert(&key, &(balance - request.amount));
            
            // Update token deposits
            let total_deposits = self.token_deposits.get(&request.token).unwrap_or(0);
            self.token_deposits.insert(&request.token, &(total_deposits - request.amount));
            
            // Update token info
            if let Some(mut token_info) = self.supported_tokens.get(&request.token) {
                token_info.total_deposits -= request.amount;
                self.supported_tokens.insert(&request.token, &token_info);
            }
            
            // Update user's withdrawal count for the day
            let current_day = Ledger::current_timestamp() / 86400;
            if let Some(mut config) = self.user_configs.get(&request.user) {
                let daily_withdrawal = config.daily_withdrawal_count.get(&current_day).unwrap_or(0);
                config.daily_withdrawal_count.insert(&current_day, &(daily_withdrawal + request.amount));
                config.last_activity = Ledger::current_timestamp();
                self.user_configs.insert(&request.user, &config);
            }
            
            // Transfer tokens to user
            // This is a simplified version - real implementation would use contract calls
            let success = self.transfer_to_user(request.token, request.user, amount_after_fee);
            
            // Transfer fee to treasury
            if fee > 0 {
                let treasury = self.treasury.get();
                self.transfer_to_user(request.token, treasury, fee);
            }
            
            // Update request status
            request.status = WithdrawalStatus::Executed as u8;
            request.executed_at = Ledger::current_timestamp();
            self.withdrawal_requests.insert(&request_id, &request);
            
            // Emit withdrawal event
            let timestamp = Ledger::current_timestamp();
            Withdrawal::emit(
                request.user,
                request.token,
                amount_after_fee,
                fee,
                timestamp
            );
            
            // Log transaction
            self.log_transaction(request.user, request.token, amount_after_fee, timestamp, 16); // 16 = withdrawal executed
            
            true
        }
        
        //
        // Query methods (read-only)
        //
        
        /// Get supported tokens
        #[safe]
        pub fn get_supported_tokens(&self) -> Vec<H160> {
            let mut tokens = Vec::new();
            let count = self.token_count.get();
            
            for i in 0..count {
                if let Some(token) = self.token_list.get(&i) {
                    tokens.push(token);
                }
            }
            
            tokens
        }
        
        /// Get token info
        #[safe]
        pub fn get_token_info(&self, token: H160) -> Option<TokenInfo> {
            self.supported_tokens.get(&token)
        }
        
        /// Get user balance
        #[safe]
        pub fn get_balance(&self, user: H160, token: H160) -> u64 {
            let key = (user, token);
            self.balances.get(&key).unwrap_or(0)
        }
        
        /// Get user config
        #[safe]
        pub fn get_user_config(&self, user: H160) -> Option<UserConfig> {
            self.user_configs.get(&user)
        }
        
        /// Get withdrawal request
        #[safe]
        pub fn get_withdrawal_request(&self, request_id: u64) -> Option<WithdrawalRequest> {
            self.withdrawal_requests.get(&request_id)
        }
        
        /// Get user's withdrawal requests
        #[safe]
        pub fn get_user_withdrawal_requests(&self, user: H160) -> Vec<u64> {
            self.user_withdrawal_requests.get(&user).unwrap_or_default()
        }
        
        /// Get contract status
        #[safe]
        pub fn get_contract_status(&self) -> (bool, u8, u64, bool) {
            (
                self.paused.get(),
                self.freeze_status.get(),
                self.frozen_until.get(),
                self.is_in_maintenance.get()
            )
        }
        
        //
        // Helper methods
        //
        
        /// Get next event ID
        #[safe]
        fn next_event_id(&mut self) -> u64 {
            let id = self.event_counter.get();
            self.event_counter.set(id + 1);
            id
        }
        
        /// Log a transaction to the transaction log
        #[safe]
        fn log_transaction(&mut self, user: H160, token: H160, amount: u64, timestamp: u64, action: u8) {
            let id = self.next_event_id();
            self.transaction_log.insert(&id, &(user, token, amount, timestamp, action));
        }
        
        /// Ensure caller is admin
        #[safe]
        fn ensure_admin(&self) {
            let sender = Runtime::current_sender();
            assert!(sender == self.admin.get(), "Not admin");
        }
        
        /// Ensure contract is operational
        #[safe]
        fn ensure_operational(&self) {
            assert!(!self.paused.get(), "Contract is paused");
            self.ensure_not_frozen();
            assert!(!self.is_in_maintenance.get(), "Contract is in maintenance");
        }
        
        /// Ensure contract is not frozen
        #[safe]
        fn ensure_not_frozen(&self) {
            let status = self.freeze_status.get();
            if status > 0 {
                // Check if freeze period has expired
                if Ledger::current_timestamp() > self.frozen_until.get() {
                    // Auto-unfreeze
                    // In a real contract, you might want admin to explicitly unfreeze
                    return;
                }
                assert!(false, "Contract is frozen");
            }
        }
        
        /// Transfer token from sender to contract (simplified)
        #[safe]
        fn transfer_from_sender(&self, token: H160, amount: u64) -> bool {
            // In a real implementation, this would call the token contract
            // Here we just return true for demonstration
            true
        }
        
        /// Transfer token from contract to user (simplified)
        #[safe]
        fn transfer_to_user(&self, token: H160, user: H160, amount: u64) -> bool {
            // In a real implementation, this would call the token contract
            // Here we just return true for demonstration
            true
        }
    }
} 