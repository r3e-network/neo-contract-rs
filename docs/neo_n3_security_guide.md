# Neo N3 Security Best Practices

This guide outlines essential security best practices for developing smart contracts on the Neo N3 blockchain using the neo-contract-rs framework.

## Overview

Security is paramount when developing smart contracts as they often manage valuable assets and execute critical operations on the blockchain. Following proper security practices helps prevent vulnerabilities that could lead to financial loss or other adverse outcomes.

## Authorization and Authentication

### Check Transaction Signers

Always verify that the caller is authorized to perform sensitive operations:

```rust
#[method]
fn transfer_ownership(&mut self, new_owner: H160) -> bool {
    let current_owner = self.owner.get().unwrap();
    
    // Check that current owner has authorized this transaction
    assert!(Runtime::check_witness(&current_owner), "Not authorized");
    
    // Update owner
    self.owner.set(new_owner);
    
    // Emit event
    OwnershipTransferred::emit(current_owner, new_owner);
    
    true
}
```

### Multi-signature Authorization

For critical operations requiring multiple approvals:

```rust
#[method]
fn execute_multi_sig_operation(&mut self, operation_id: u64) -> bool {
    // Get required signers
    let signers = self.required_signers.get().unwrap();
    
    // Check that all required signers have signed
    for signer in signers.iter() {
        assert!(Runtime::check_witness(signer), "Missing required signature");
    }
    
    // Perform the operation
    // ...
    
    true
}
```

## Reentrancy Protection

### Use No-Reentry Attribute

Prevent reentrancy attacks with the `#[no_reentry]` attribute:

```rust
#[method]
#[no_reentry]
fn withdraw(&mut self, amount: u64) -> bool {
    let caller = Runtime::calling_script_hash();
    
    // Verify authorization
    assert!(Runtime::check_witness(&caller), "Not authorized");
    
    // Check balance
    let balance = self.balances.get(&caller).unwrap_or_default();
    assert!(balance >= amount, "Insufficient balance");
    
    // Update state BEFORE external calls
    let new_balance = balance - amount;
    if new_balance > 0 {
        self.balances.insert(caller, new_balance);
    } else {
        self.balances.remove(&caller);
    }
    
    // Make external call to transfer assets
    // ...
    
    true
}
```

### Checks-Effects-Interactions Pattern

Always follow this pattern for external calls:

1. Perform all checks (validations)
2. Apply all effects (state changes)
3. Interact with external contracts (last step)

```rust
#[method]
fn safe_external_call(&mut self, target_contract: H160, amount: u64) -> bool {
    // 1. CHECKS: Verify conditions
    let caller = Runtime::calling_script_hash();
    assert!(Runtime::check_witness(&caller), "Not authorized");
    let balance = self.balances.get(&caller).unwrap_or_default();
    assert!(balance >= amount, "Insufficient balance");
    
    // 2. EFFECTS: Update internal state
    let new_balance = balance - amount;
    if new_balance > 0 {
        self.balances.insert(caller, new_balance);
    } else {
        self.balances.remove(&caller);
    }
    
    // 3. INTERACTIONS: Call external contract (last)
    let method = "receive";
    let args = vec![
        StackItem::from(caller),
        StackItem::from(amount),
    ];
    
    contract::call(
        &target_contract,
        method,
        &args,
        CallFlags::All
    );
    
    true
}
```

## Integer Arithmetic

### Overflow and Underflow Protection

Always check for integer overflow/underflow:

```rust
#[method]
fn add_balance(&mut self, address: H160, amount: u64) -> bool {
    let balance = self.balances.get(&address).unwrap_or_default();
    
    // Check for overflow
    assert!(u64::MAX - balance >= amount, "Balance overflow");
    
    // Safe to add
    self.balances.insert(address, balance + amount);
    true
}
```

### Safe Subtraction

```rust
#[method]
fn subtract_balance(&mut self, address: H160, amount: u64) -> bool {
    let balance = self.balances.get(&address).unwrap_or_default();
    
    // Check for underflow
    assert!(balance >= amount, "Insufficient balance");
    
    // Safe to subtract
    self.balances.insert(address, balance - amount);
    true
}
```

## Input Validation

### Validate All Inputs

Always validate inputs to prevent unexpected behavior:

```rust
#[method]
fn process_payment(&mut self, recipient: H160, amount: u64) -> bool {
    // Validate inputs
    assert!(!recipient.is_zero(), "Invalid recipient address");
    assert!(amount > 0, "Amount must be greater than zero");
    assert!(amount <= self.max_transfer_limit.get().unwrap(), "Exceeds transfer limit");
    
    // Process the payment
    // ...
    
    true
}
```

### Safe Array Operations

When working with arrays or vectors, always validate their length:

```rust
#[method]
fn batch_transfer(&mut self, recipients: Vec<H160>, amounts: Vec<u64>) -> bool {
    // Validate array lengths
    assert!(!recipients.is_empty(), "Recipients list cannot be empty");
    assert!(recipients.len() == amounts.len(), "Arrays must have equal length");
    assert!(recipients.len() <= 20, "Batch size too large");
    
    // Process batch transfer
    // ...
    
    true
}
```

## Access Control

### Role-Based Access Control

Implement proper access controls for different roles:

```rust
#[storage]
struct SecureContract {
    owner: StorageItem<H160>,
    admins: StorageMap<H160, bool>,
    operators: StorageMap<H160, bool>,
}

impl SecureContract {
    fn only_owner(&self) -> bool {
        let caller = Runtime::calling_script_hash();
        caller == self.owner.get().unwrap()
    }
    
    fn is_admin(&self) -> bool {
        let caller = Runtime::calling_script_hash();
        self.admins.get(&caller).unwrap_or_default()
    }
    
    fn is_operator(&self) -> bool {
        let caller = Runtime::calling_script_hash();
        self.operators.get(&caller).unwrap_or_default()
    }
    
    #[method]
    fn admin_operation(&mut self) -> bool {
        assert!(self.only_owner() || self.is_admin(), "Admin access required");
        // Admin operation logic
        true
    }
}
```

### Time-Locked Operations

For critical operations, consider time locks:

```rust
#[storage]
struct TimeLockContract {
    ownership_transfer_time: StorageMap<H160, u64>,
    pending_owner: StorageItem<Option<H160>>,
    owner: StorageItem<H160>,
}

impl TimeLockContract {
    #[method]
    fn initiate_ownership_transfer(&mut self, new_owner: H160) -> bool {
        // Only current owner can initiate
        let caller = Runtime::calling_script_hash();
        assert!(caller == self.owner.get().unwrap(), "Not authorized");
        
        // Set pending owner
        self.pending_owner.set(Some(new_owner));
        
        // Set time lock (48 hours from now)
        let current_time = Runtime::time();
        let unlock_time = current_time + 48 * 60 * 60;
        self.ownership_transfer_time.insert(new_owner, unlock_time);
        
        true
    }
    
    #[method]
    fn complete_ownership_transfer(&mut self) -> bool {
        let caller = Runtime::calling_script_hash();
        
        // Check if caller is the pending owner
        let pending = self.pending_owner.get().unwrap();
        assert!(pending == Some(caller), "Not the pending owner");
        
        // Check if time lock has expired
        let unlock_time = self.ownership_transfer_time.get(&caller).unwrap_or_default();
        let current_time = Runtime::time();
        assert!(current_time >= unlock_time, "Time lock not expired");
        
        // Complete transfer
        let previous_owner = self.owner.get().unwrap();
        self.owner.set(caller);
        self.pending_owner.set(None);
        
        // Emit event
        OwnershipTransferred::emit(previous_owner, caller);
        
        true
    }
}
```

## Contract Storage Security

### Proper Key Management

Use structured storage keys to prevent collisions:

```rust
#[storage]
struct SecureStorage {
    // Using structured storage prevents key collisions
    user_balances: StorageMap<H160, u64>,
    admin_balances: StorageMap<H160, u64>,
}
```

### Storage Cleanup

Always clean up storage when data is no longer needed:

```rust
#[method]
fn close_account(&mut self, user: H160) -> bool {
    let caller = Runtime::calling_script_hash();
    assert!(caller == user || self.is_admin(), "Not authorized");
    
    // Clean up all user data
    self.user_balances.remove(&user);
    self.user_settings.remove(&user);
    self.user_history.remove(&user);
    
    true
}
```

## Safe Event Emission

### Proper Event Structure

Events should follow Neo N3 standards for proper indexing:

```rust
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        // Properly handle null values
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(amount));
        
        // Emit using Neo N3 standard method
        Runtime::notify(&event_name, &event_data);
    }
}
```

### Sensitive Information in Events

Never include sensitive information in events:

```rust
// WRONG: Including sensitive data in event
PasswordChanged::emit(user, old_password, new_password);

// RIGHT: Only emit the fact that a change occurred
PasswordChanged::emit(user, Runtime::time());
```

## Contract Upgrades

### Secure Upgrade Pattern

Implement secure contract upgrade functionality:

```rust
#[method]
fn upgrade(&mut self, nef_file: ByteArray, manifest: ByteArray) -> bool {
    // Only owner can upgrade
    let caller = Runtime::calling_script_hash();
    assert!(caller == self.owner.get().unwrap(), "Not authorized");
    
    // Perform the upgrade
    Contract::update(&nef_file, &manifest);
    
    true
}
```

### Migration Strategies

For complex data migrations during upgrades:

```rust
#[method]
fn migrate_data(&mut self, old_format_addresses: Vec<H160>) -> bool {
    // Only owner can migrate
    let caller = Runtime::calling_script_hash();
    assert!(caller == self.owner.get().unwrap(), "Not authorized");
    
    // Migrate data from old format to new format
    for address in old_format_addresses {
        let old_data = self.old_format_data.get(&address).unwrap_or_default();
        let new_data = convert_to_new_format(old_data);
        
        // Store in new format
        self.new_format_data.insert(address, new_data);
        
        // Clean up old data
        self.old_format_data.remove(&address);
    }
    
    true
}
```

## Neo N3 Specific Security

### Method Annotations for Security

#### Safe Methods

Neo N3 distinguishes between methods that modify state and those that only read state. Properly marking methods as `#[safe]` has important security implications:

1. **Use `#[safe]` for read-only methods**: Any method that doesn't modify contract storage or invoke state-changing operations should be marked with `#[safe]`.

    ```rust
    // Correct usage - read-only method with #[safe]
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or_default()
    }
    ```

2. **Important**: The `#[safe]` attribute implicitly includes the functionality of `#[method]`, so there's no need to use both annotations together. Simply use `#[safe]` for read-only methods that you want to expose in your contract interface.

3. **Do not mark state-changing methods as safe**: Methods that modify state should never be marked as `#[safe]`, as this could lead to unexpected behavior and security vulnerabilities.

    ```rust
    // Correct - state-changing method with only #[method]
    #[method]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Modify state
        // ...
    }
    
    // Incorrect - state-changing method should not be marked safe
    // #[safe]
    // pub fn modify_balance(&mut self, account: H160, amount: u64) {
    //     // This modifies state but is incorrectly marked as safe
    //     // ...
    // }
    ```

4. **Safe methods in manifest**: Methods marked with `#[safe]` will be represented in the contract manifest with `"safe": true`, allowing Neo N3 nodes to optimize their execution.

### Call Flags

When calling other contracts, use appropriate call flags:

```rust
#[method]
fn safe_contract_call(&self, target: H160) -> bool {
    // For read-only calls
    let read_result = contract::call(
        &target,
        "read_only_method",
        &[],
        CallFlags::ReadOnly
    );
    
    // For state-changing calls
    let write_result = contract::call(
        &target,
        "state_changing_method",
        &[Any::from(123)],
        CallFlags::All
    );
    
    true
}
```

## Testing and Verification

### Comprehensive Testing

Always thoroughly test smart contracts:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::prelude::*;
    
    #[test]
    fn test_transfer() {
        // Set up test environment
        let mut context = TestingContext::new();
        
        // Create test accounts
        let owner = H160::from_hex("0x0000000000000000000000000000000000000001").unwrap();
        let user1 = H160::from_hex("0x0000000000000000000000000000000000000002").unwrap();
        
        // Deploy contract
        context.set_caller(owner);
        let mut contract = TokenContract::new();
        
        // Test transfer functionality
        assert!(contract.mint(user1, 1000));
        
        context.set_caller(user1);
        assert!(contract.transfer(owner, 500));
        
        // Verify balances
        assert_eq!(contract.balance_of(user1), 500);
        assert_eq!(contract.balance_of(owner), 500);
    }
    
    #[test]
    fn test_unauthorized_transfer() {
        // Set up test environment
        let mut context = TestingContext::new();
        
        // Create test accounts
        let owner = H160::from_hex("0x0000000000000000000000000000000000000001").unwrap();
        let user1 = H160::from_hex("0x0000000000000000000000000000000000000002").unwrap();
        let attacker = H160::from_hex("0x0000000000000000000000000000000000000003").unwrap();
        
        // Deploy contract
        context.set_caller(owner);
        let mut contract = TokenContract::new();
        
        // Mint tokens to user1
        assert!(contract.mint(user1, 1000));
        
        // Attacker tries to transfer user1's tokens
        context.set_caller(attacker);
        let result = std::panic::catch_unwind(|| {
            contract.transfer_from(user1, attacker, 500)
        });
        
        // Should panic with authorization error
        assert!(result.is_err());
    }
}
```

### Formal Verification

Consider formal verification for critical contracts to mathematically prove security properties.

## Conclusion

Security is an ongoing process that should be integrated throughout the development lifecycle. By following these best practices, you can significantly reduce the risk of vulnerabilities in your Neo N3 smart contracts.

For more information on Neo N3 contract development, refer to:

- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Runtime Guide](./neo_n3_runtime_guide.md)
- [Neo N3 Storage Guide](./neo_n3_storage_guide.md)
