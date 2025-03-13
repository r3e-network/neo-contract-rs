# Security Best Practices for Neo N3 Smart Contracts

This document outlines security best practices for developing secure smart contracts on the Neo N3 blockchain using the Rust framework.

## Overview

Security is paramount when developing smart contracts that manage digital assets and execute critical business logic. Neo N3 provides several security mechanisms that developers should leverage to ensure their contracts are robust against attacks and vulnerabilities.

## Authentication and Authorization

### Check Witness Pattern

Always verify that the caller has authority to perform sensitive operations using the `Runtime::check_witness` function:

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod token_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct TokenStorage {
        owner: Item<H160>,
    }
    
    #[method]
    fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Verify that the caller is authorized to transfer from this address
        if !Runtime::check_witness(&from) {
            return false;
        }
        
        // Proceed with transfer logic
        // ...
        
        true
    }
}
```

### Multiple Authentication Checks

For operations that require multiple parties' authorization, ensure all required signatures are verified:

```rust
#[method]
fn multi_signature_operation(&mut self, accounts: Array<H160>) -> bool {
    // Verify all required signers have signed
    for account in accounts.iter() {
        if !Runtime::check_witness(&account) {
            return false;
        }
    }
    
    // Proceed with operation
    // ...
    
    true
}
```

### Owner Management

Implement proper owner management with secure ownership transfer:

```rust
#[storage]
struct OwnershipStorage {
    owner: Item<H160>,
    pending_owner: Item<H160>,
}

#[method]
fn transfer_ownership(&mut self, new_owner: H160) -> bool {
    // Verify current owner
    let current_owner = self.owner.get();
    if !Runtime::check_witness(&current_owner) {
        return false;
    }
    
    // Set pending owner instead of immediate transfer
    self.pending_owner.put(new_owner);
    
    true
}

#[method]
fn accept_ownership(&mut self) -> bool {
    // New owner must claim ownership
    let pending = self.pending_owner.get();
    if pending.is_zero() {
        return false;
    }
    
    if !Runtime::check_witness(&pending) {
        return false;
    }
    
    // Transfer ownership
    self.owner.put(pending);
    self.pending_owner.put(H160::zero());
    
    true
}
```

## State Management

### Reentrancy Protection

Prevent reentrancy attacks by using a mutex pattern:

```rust
#[storage]
struct SecureStorage {
    locked: Item<bool>,
    // Other storage items
}

#[method]
fn secure_operation(&mut self) -> bool {
    // Check if already locked
    if self.locked.get() {
        return false;
    }
    
    // Set lock
    self.locked.put(true);
    
    // Perform operations that might call external contracts
    // ...
    
    // Release lock
    self.locked.put(false);
    
    true
}
```

### Checks-Effects-Interactions Pattern

Always follow the Checks-Effects-Interactions pattern:

1. Perform all checks first
2. Update contract state
3. Interact with external contracts last

```rust
#[method]
fn withdraw(&mut self, account: H160, amount: u64) -> bool {
    // 1. Checks
    if !Runtime::check_witness(&account) {
        return false;
    }
    
    let balance = self.balances.get(&account);
    if balance < amount {
        return false;
    }
    
    // 2. Effects (update state)
    self.balances.put(&account, balance - amount);
    
    // 3. Interactions (external calls)
    let gas_token = gas_contract_hash();
    let method = ByteString::from("transfer");
    let mut args = Array::<Any>::new();
    args.push(Any::from(self_address()));
    args.push(Any::from(account));
    args.push(Any::from(amount));
    
    let call_flags = CallFlags::ALL as u32;
    let result = contract_call(&gas_token, &method, &args, call_flags);
    
    true
}
```

### Overflow Protection

Protect against integer overflows and underflows by using safe mathematical operations:

```rust
#[method]
fn safe_add(&self, a: u64, b: u64) -> u64 {
    // Check for overflow
    if b > u64::MAX - a {
        panic!("Integer overflow");
    }
    a + b
}

#[method]
fn safe_sub(&self, a: u64, b: u64) -> u64 {
    // Check for underflow
    if b > a {
        panic!("Integer underflow");
    }
    a - b
}
```

## Safe Intercontract Communication

### Validate Contract Hashes

Always validate contract hashes before making calls:

```rust
#[storage]
struct TrustedContractsStorage {
    token_contract: Item<H160>,
}

#[method]
fn call_trusted_contract(&self, method: ByteString, args: Array<Any>) -> Any {
    // Get the trusted contract hash from storage
    let trusted_hash = self.token_contract.get();
    
    // Ensure it's not zero
    if trusted_hash.is_zero() {
        panic!("Trusted contract not set");
    }
    
    // Make the call with appropriate flags
    let call_flags = CallFlags::ALL as u32;
    let result = contract_call(&trusted_hash, &method, &args, call_flags);
    
    // Process result
    // ...
}
```

### Use Appropriate Call Flags

Limit permissions with appropriate call flags:

```rust
// For read-only operations
let read_only_flags = CallFlags::READ_ONLY as u32;

// For operations that need to modify state
let modify_flags = (CallFlags::ALLOW_MODIFY_STATES | CallFlags::ALLOW_STATES) as u32;

// For operations that need to notify but not call other contracts
let notify_flags = (CallFlags::ALLOW_NOTIFY | CallFlags::ALLOW_STATES) as u32;
```

### Handle Call Results Properly

Always check return values from contract calls:

```rust
#[method]
fn transfer_tokens(&mut self, token_hash: H160, to: H160, amount: u64) -> bool {
    // Prepare the call
    let method = ByteString::from("transfer");
    let mut args = Array::<Any>::new();
    args.push(Any::from(self_address()));
    args.push(Any::from(to));
    args.push(Any::from(amount));
    
    let call_flags = CallFlags::ALL as u32;
    let result = contract_call(&token_hash, &method, &args, call_flags);
    
    // Check if result is empty (call failed)
    if result.is_empty() {
        return false;
    }
    
    // Try to deserialize the result as boolean
    let success: bool = serde_json::from_slice(&result).unwrap_or(false);
    
    success
}
```

## Safe Contract Updates

### Staged Updates

Implement a staged update process:

```rust
#[storage]
struct UpdateStorage {
    owner: Item<H160>,
    pending_nef: Item<ByteString>,
    pending_manifest: Item<ByteString>,
    update_available_at: Item<u64>,
}

#[method]
fn stage_update(&mut self, nef_file: ByteString, manifest: ByteString) -> bool {
    // Verify owner
    let owner = self.owner.get();
    if !Runtime::check_witness(&owner) {
        return false;
    }
    
    // Store update details
    self.pending_nef.put(&nef_file);
    self.pending_manifest.put(&manifest);
    
    // Set timelock (48 hours in the future)
    let current_time = Runtime::time();
    let future_time = current_time + 172800000; // 48 hours in milliseconds
    self.update_available_at.put(future_time);
    
    true
}

#[method]
fn apply_update(&mut self) -> bool {
    // Verify owner
    let owner = self.owner.get();
    if !Runtime::check_witness(&owner) {
        return false;
    }
    
    // Check timelock
    let available_time = self.update_available_at.get();
    let current_time = Runtime::time();
    if current_time < available_time {
        return false;
    }
    
    // Get update details
    let nef_file = self.pending_nef.get();
    let manifest = self.pending_manifest.get();
    
    // Apply update
    contract_update(&nef_file, &manifest);
    
    true
}
```

### Version Control

Maintain version control in your contract:

```rust
#[storage]
struct VersionedStorage {
    version: Item<u32>,
    // Other storage items
}

#[method]
pub fn version(&self) -> u32 {
    self.version.get()
}

#[method]
fn after_update(&mut self) {
    // Increment version after update
    let current_version = self.version.get();
    self.version.put(current_version + 1);
    
    // Perform any migration logic needed
    // ...
}
```

## Input Validation

### Validate All Inputs

Always validate inputs before processing:

```rust
#[method]
fn process_payment(&mut self, recipient: H160, amount: u64) -> bool {
    // Validate recipient is not zero address
    if recipient.is_zero() {
        return false;
    }
    
    // Validate amount is greater than zero
    if amount == 0 {
        return false;
    }
    
    // Validate amount is not too large
    if amount > self.max_payment_limit.get() {
        return false;
    }
    
    // Process payment
    // ...
    
    true
}
```

### Limit Array Operations

Be careful with arrays to prevent DOS attacks:

```rust
#[method]
fn process_batch(&mut self, operations: Array<Any>) -> bool {
    // Limit batch size
    let max_batch_size = 20;
    if operations.len() > max_batch_size {
        return false;
    }
    
    // Process batch
    // ...
    
    true
}
```

## Event Emissions

### Emit Appropriate Events

Emit detailed events for important state changes:

```rust
#[method]
fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Transfer logic
    // ...
    
    // Emit event
    let event_name = ByteString::from("Transfer");
    let mut event_data = Array::<Any>::new();
    event_data.push(Any::from(from));
    event_data.push(Any::from(to));
    event_data.push(Any::from(amount));
    
    Runtime::notify(&event_name, &event_data);
    
    true
}
```

## Testing and Security Audits

### Comprehensive Testing

Write comprehensive tests for your contracts:

- Unit tests for individual functions
- Integration tests for contract interactions
- Scenario tests for common workflows
- Edge cases and error conditions
- Reentrancy tests

### Audit and Review

Before deploying to mainnet:

- Conduct internal code reviews
- Consider professional security audits
- Review gas usage and optimization
- Test on testnet thoroughly
- Document known limitations

## Neo N3 Specific Security Considerations

### Native Contracts

Be careful when interacting with native contracts and ensure you understand their behavior:

```rust
// Get native contract hashes using the provided functions
let gas_token = gas_contract_hash();
let contract_management = contract_management_contract_hash();
```

### Gas Optimization

Be mindful of gas usage in your contracts:

- Use safe methods where possible
- Limit storage operations
- Batch operations when appropriate
- Avoid unnecessary contract calls

### Script Hash Verification

Properly verify script hashes for UTXO operations:

```rust
#[method]
fn verify_transaction(&self, tx_hash: H256) -> bool {
    // Get transaction sender
    let sender = Runtime::calling_script_hash();
    
    // Verify sender is authorized
    if !self.authorized_callers.get(&sender) {
        return false;
    }
    
    // Proceed with verification
    // ...
    
    true
}
```

## Conclusion

Building secure Neo N3 smart contracts requires attention to detail, adherence to best practices, and a security-first mindset. By following these guidelines, you can minimize the risk of vulnerabilities and create robust, trustworthy smart contracts.

Remember that security is an ongoing process, and staying updated with the latest security developments in the Neo ecosystem is essential for maintaining secure contracts over time.
