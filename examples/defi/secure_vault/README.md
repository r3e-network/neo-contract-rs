# Secure Token Vault Example

A comprehensive example of a secure token vault smart contract for Neo N3, demonstrating advanced storage patterns, security best practices, and gas optimization techniques.

## Overview

This smart contract implements a secure multi-user, multi-token vault with configurable security settings, time locks, and withdrawal approvals. It showcases how to structure storage efficiently while implementing robust security features.

## Key Features

- **Multi-token Support**: Store and manage multiple token types simultaneously
- **Configurable Security**: Users can set their own security preferences
- **Time-lock Mechanisms**: Optional time locks to prevent immediate withdrawals
- **Multi-signature Approvals**: Optional required approvals for withdrawals
- **Rate Limiting**: Daily withdrawal limits to prevent large-scale theft
- **Admin Controls**: Emergency freeze, maintenance mode, fee management
- **Detailed Event Logging**: Comprehensive event system for tracking activities

## Advanced Storage Patterns Demonstrated

### 1. Storage Prefixing

The contract uses clear, consistent prefixes for all storage keys to ensure organization and prevent collisions:

```rust
// Admin management
admin: Item::new(b"admin"),
pending_admin: Item::new(b"admin.pending"),
admin_transfer_deadline: Item::new(b"admin.transfer.deadline"),

// Contract state
paused: Item::new(b"state.paused"),
freeze_status: Item::new(b"state.freeze"),
frozen_until: Item::new(b"state.freeze.until"),

// And so on...
```

This pattern ensures storage keys don't collide, makes debugging easier, and allows for more organized storage queries.

### 2. Composite Keys

For efficient storage of related data, the contract uses composite keys:

```rust
// (user, token) -> balance mapping for efficient querying
balances: Map<(H160, H160), u64>,
```

This allows retrieving a user's balance for a specific token in a single lookup, rather than requiring nested maps or multiple queries.

### 3. Lazy Loading

The contract only loads data when needed, avoiding unnecessary storage operations:

```rust
// Only load user config when needed
fn deposit(&mut self, token: H160, amount: u64) -> bool {
    // ...
    // Initialize vault only if needed
    if !self.user_configs.contains_key(&sender) {
        self.initialize_vault();
    }
    // ...
}
```

### 4. Batch Operations

The contract batches related operations together to optimize gas usage:

```rust
// In the deposit method, we batch operations:
// 1. Update user balance
// 2. Update total token deposits
// 3. Update token info
// 4. Update user activity timestamp
// All in the same transaction
```

### 5. Data Segregation

Clear separation of contract state, user configuration, and transaction data:

```rust
// Contract-level settings
paused: Item<bool>,
freeze_status: Item<u8>,

// Token-specific information
supported_tokens: Map<H160, TokenInfo>,

// User-specific configurations
user_configs: Map<H160, UserConfig>,

// Transaction-specific data
withdrawal_requests: Map<u64, WithdrawalRequest>,
```

### 6. Type-safe Storage

Using proper types and enums for storage items:

```rust
enum WithdrawalStatus {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
    Executed = 3,
    Expired = 4,
}

// Store as u8 but use enum for type safety
status: WithdrawalStatus::Pending as u8,
```

## Security Features

### 1. Two-phase Admin Transfers

Admin privileges can only be transferred via a two-phase process:
1. Current admin initiates transfer with a deadline
2. New admin must accept within the deadline

```rust
fn transfer_admin(&mut self, new_admin: H160) -> bool {
    // Verify admin
    self.ensure_admin();
    
    // Set pending admin and deadline
    self.pending_admin.set(new_admin);
    self.admin_transfer_deadline.set(Ledger::current_timestamp() + 86400);
    
    // ...
}

fn accept_admin(&mut self) -> bool {
    // Must be called by pending admin
    let sender = Runtime::current_sender();
    assert!(sender == self.pending_admin.get(), "Not pending admin");
    
    // Verify deadline hasn't passed
    assert!(Ledger::current_timestamp() < self.admin_transfer_deadline.get(), "Transfer expired");
    
    // Transfer admin
    self.admin.set(sender);
    // ...
}
```

### 2. Time-lock Mechanisms

Users can set time locks to prevent immediate withdrawals:

```rust
fn lock_vault(&mut self, duration: u64) -> bool {
    // ...
    config.is_locked = true;
    config.locked_until = Ledger::current_timestamp() + lock_duration;
    // ...
}

fn unlock_vault(&mut self) -> bool {
    // ...
    // Check if lock period has passed
    assert!(Ledger::current_timestamp() >= config.locked_until, "Vault still locked");
    // ...
}
```

### 3. Multi-signature Approvals

Optional multi-signature approvals for withdrawals:

```rust
fn approve_withdrawal(&mut self, request_id: u64) -> bool {
    // ...
    // Check if enough approvals to execute
    if config.require_approvals && request.approvals.len() >= config.min_approvals as usize {
        // Execute withdrawal
        // ...
    }
    // ...
}
```

### 4. Rate Limiting & Daily Withdrawal Limits

Configurable daily withdrawal limits:

```rust
// Check daily withdrawal limit
let current_day = Ledger::current_timestamp() / 86400; // Days since epoch
let daily_withdrawal = config.daily_withdrawal_count.get(&current_day).unwrap_or(0);
assert!(daily_withdrawal + amount <= config.max_daily_withdrawal, "Daily withdrawal limit exceeded");
```

### 5. Emergency Controls

Admin can freeze the contract in case of emergencies:

```rust
fn emergency_freeze(&mut self, reason: u8, duration: u64) -> bool {
    // Verify admin
    self.ensure_admin();
    
    // Set freeze status
    self.freeze_status.set(reason);
    self.frozen_until.set(Ledger::current_timestamp() + duration);
    // ...
}
```

### 6. Transaction Logging

Detailed transaction logging for audit purposes:

```rust
fn log_transaction(&mut self, user: H160, token: H160, amount: u64, timestamp: u64, action: u8) {
    let id = self.next_event_id();
    self.transaction_log.insert(&id, &(user, token, amount, timestamp, action));
}
```

## Contract Structure

### Storage Layout

The contract storage is organized into logical sections:

```rust
struct SecureVault {
    // Contract admin management
    admin: Item<H160>,
    pending_admin: Item<H160>,
    admin_transfer_deadline: Item<u64>,
    
    // Global contract config
    paused: Item<bool>,
    freeze_status: Item<u8>,
    frozen_until: Item<u64>,
    is_in_maintenance: Item<bool>,
    
    // Fee settings
    treasury: Item<H160>,
    withdrawal_fee_basis_points: Item<u64>,
    
    // Supported tokens
    supported_tokens: Map<H160, TokenInfo>,
    token_list: Map<u64, H160>,
    token_count: Item<u64>,
    
    // User balances and data
    balances: Map<(H160, H160), u64>,
    token_deposits: Map<H160, u64>,
    user_configs: Map<H160, UserConfig>,
    
    // Withdrawal requests
    withdrawal_requests: Map<u64, WithdrawalRequest>,
    next_withdrawal_id: Item<u64>,
    user_withdrawal_requests: Map<H160, Vec<u64>>,
    
    // Security
    failed_attempts: Map<H160, u64>,
    last_failed_attempt: Map<H160, u64>,
    
    // Transaction logging
    event_counter: Item<u64>,
    transaction_log: Map<u64, (H160, H160, u64, u64, u8)>,
}
```

### Method Categories

The contract methods are organized into logical categories:

1. **Admin Functions**: Token management, fee settings, emergency controls
2. **User Vault Setup**: Initialization and configuration of user vaults
3. **Token Operations**: Deposits and withdrawals
4. **Query Methods**: View-only methods for querying state
5. **Helper Methods**: Internal utility functions

## Use Cases

### 1. Corporate Treasury Management

Companies can use this vault to:
- Store multiple tokens with multi-signature approvals
- Set withdrawal limits to prevent large unauthorized transfers
- Use time locks for scheduled fund releases

### 2. DAO Fund Management

DAOs can use this vault to:
- Configure multi-signature requirements for treasury management
- Implement time locks for proposal execution delays
- Track all transactions for transparency

### 3. Personal Secure Storage

Individuals can use this vault for:
- Enhanced security with configurable protection mechanisms
- Time locks to prevent impulsive withdrawals
- Ability to set trusted approvers for recovery

## Gas Optimization Techniques

1. **Composite Keys**: Reduced storage overhead from using composite keys
2. **Lazy Initialization**: Only creating user configurations when needed
3. **Storage Prefixing**: Optimized key structure for related data
4. **Batch Operations**: Grouping related operations to reduce overall gas cost
5. **Efficient Data Structures**: Using appropriate types and sizes for data

## Building and Testing

To build the secure vault contract:

```bash
# Development build (with debugging)
cargo build -p secure-vault --features std

# Production build
cargo build -p secure-vault --release
```

## Limitations and Further Improvements

1. **Token Integration**: The current example uses simplified token transfer logic
2. **Enhanced Recovery**: Could add more sophisticated account recovery mechanisms
3. **Access Control Lists**: Could implement more granular permissions
4. **Advanced Rate Limiting**: Could add IP-based or blockchain-based rate limiting
5. **Formal Verification**: The contract could benefit from formal verification

## Related Documentation

For more details on the patterns used in this example, see:

- [Storage Guide](../../../docs/storage_guide.md)
- [Gas Optimization Guide](../../../docs/gas_optimization.md)
- [Contract Security Guide](../../../docs/contract_security_guide.md)
- [Events Guide](../../../docs/events_guide.md)

## License

MIT 