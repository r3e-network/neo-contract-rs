# Security Best Practices for Neo Smart Contracts

This guide covers security considerations, best practices, and available security features in the Neo Contract Rust Framework for building secure Neo N3 smart contracts.

## Introduction

Security is a critical concern for blockchain applications, where vulnerabilities can lead to significant financial losses and damage to user trust. The Neo Contract Rust Framework provides several security features to help developers create safer smart contracts, but understanding these features and following security best practices remains essential.

## Reentrancy Protection

Reentrancy attacks occur when a contract's function is called again before the first execution completes. The Neo Contract Rust Framework provides built-in protection against these attacks.

### Using the `#[no_reentrant]` Attribute

The `#[no_reentrant]` attribute adds reentrancy protection to a method:

```rust
#[method]
#[no_reentrant]
pub fn withdraw(&mut self, amount: u64) -> bool {
    // This method is protected from reentrancy attacks
    // ...
}
```

This attribute prevents the method from being called again while it's still executing, blocking potential reentrancy attacks.

### How It Works

The framework implements reentrancy protection by:

1. Adding a reentrancy guard in the contract's storage
2. Setting the guard before the method execution
3. Checking the guard at the beginning of the method
4. Clearing the guard after execution completes

### When to Use Reentrancy Protection

Apply the `#[no_reentrant]` attribute to methods that:

- Transfer tokens or assets
- Modify critical state that could be exploited if called multiple times
- Call external contracts (especially if they transfer control flow)
- Update balances or ownership records

Example of vulnerable code pattern:

```rust
#[method]
pub fn vulnerable_withdraw(&mut self, amount: u64) -> bool {
    let balance = self.balances.get(&runtime::calling_script_hash()).unwrap_or_default();
    if balance < amount {
        return false;
    }
    
    // Vulnerable: External call before state update
    let result = contract::call_contract(
        Address::from_script_hash(GAS_SCRIPT_HASH),
        "transfer",
        CallFlags::ALL,
        &[
            runtime::calling_script_hash().into_value(),
            amount.into_value(),
            ByteArray::from("withdrawal").into_value()
        ]
    );
    
    // If the external call triggers a reentrant call to this method,
    // the balance hasn't been updated yet
    
    // Update balance after the call
    self.balances.insert(&runtime::calling_script_hash(), balance - amount);
    
    true
}
```

Secure version with reentrancy protection:

```rust
#[method]
#[no_reentrant]
pub fn secure_withdraw(&mut self, amount: u64) -> bool {
    let balance = self.balances.get(&runtime::calling_script_hash()).unwrap_or_default();
    if balance < amount {
        return false;
    }
    
    // Update state before external call
    self.balances.insert(&runtime::calling_script_hash(), balance - amount);
    
    // External call after state update
    let result = contract::call_contract(
        Address::from_script_hash(GAS_SCRIPT_HASH),
        "transfer",
        CallFlags::ALL,
        &[
            runtime::calling_script_hash().into_value(),
            amount.into_value(),
            ByteArray::from("withdrawal").into_value()
        ]
    );
    
    true
}
```

## Access Control

Proper access control is essential for contract security. The Neo Contract Rust Framework provides several ways to implement access control.

### Owner-Based Access Control

```rust
#[storage]
pub struct ContractWithOwner {
    owner: StorageItem<Address>,
    // other storage fields
}

impl ContractWithOwner {
    #[constructor]
    pub fn new(owner: Address) -> Self {
        let mut this = Self {
            owner: StorageItem::new(),
            // initialize other fields
        };
        this.owner.set(owner);
        this
    }
    
    // Modifier pattern for access control
    fn assert_owner(&self) {
        let caller = runtime::get_calling_scripthashs().first().unwrap().clone();
        assert!(
            caller == self.owner.get().unwrap(),
            "Only the owner can call this method"
        );
    }
    
    #[method]
    pub fn restricted_method(&mut self, value: u64) {
        self.assert_owner();
        // Method implementation
    }
}
```

### Role-Based Access Control

For more complex access control, implement role-based permissions:

```rust
#[storage]
pub struct ContractWithRoles {
    roles: StorageMap<Address, Vec<String>>,
    // other storage fields
}

impl ContractWithRoles {
    #[constructor]
    pub fn new(admin: Address) -> Self {
        let mut this = Self {
            roles: StorageMap::new(),
            // initialize other fields
        };
        
        // Assign admin role to the deployer
        this.roles.insert(&admin, vec!["ADMIN".to_string()]);
        
        this
    }
    
    fn has_role(&self, address: &Address, role: &str) -> bool {
        if let Some(roles) = self.roles.get(address) {
            return roles.contains(&role.to_string());
        }
        false
    }
    
    fn assert_role(&self, role: &str) {
        let caller = runtime::get_calling_scripthashs().first().unwrap().clone();
        assert!(
            self.has_role(&caller, role),
            format!("Caller doesn't have the required role: {}", role)
        );
    }
    
    #[method]
    pub fn add_role(&mut self, address: Address, role: String) {
        self.assert_role("ADMIN");
        
        let mut roles = self.roles.get(&address).unwrap_or_default();
        if !roles.contains(&role) {
            roles.push(role);
            self.roles.insert(&address, roles);
        }
    }
    
    #[method]
    pub fn admin_only_method(&mut self, value: u64) {
        self.assert_role("ADMIN");
        // Method implementation
    }
    
    #[method]
    pub fn operator_method(&mut self, value: u64) {
        self.assert_role("OPERATOR");
        // Method implementation
    }
}
```

## Safe Methods and State Separation

The `#[safe]` attribute marks methods as read-only, which helps maintain a clear separation between methods that modify state and those that don't.

```rust
#[safe]
pub fn view_balance(&self, account: Address) -> u64 {
    self.balances.get(&account).unwrap_or_default()
}

#[method]
pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
    // Implementation that modifies state
    true
}
```

Benefits of using `#[safe]`:
- Clear documentation of state-modifying vs. read-only methods
- Optimization by the Neo VM for read-only operations
- Better user experience with methods that don't require a full transaction

## Check-Effects-Interactions Pattern

Follow the check-effects-interactions pattern to prevent reentrancy and other attacks:

1. **Check**: Validate all preconditions
2. **Effects**: Update the contract's state
3. **Interactions**: Interact with external contracts

```rust
#[method]
pub fn secure_method(&mut self, recipient: Address, amount: u64) -> bool {
    // 1. Checks
    let sender = runtime::get_calling_scripthashs().first().unwrap().clone();
    let sender_balance = self.balances.get(&sender).unwrap_or_default();
    assert!(sender_balance >= amount, "Insufficient balance");
    
    // 2. Effects
    self.balances.insert(&sender, sender_balance - amount);
    let recipient_balance = self.balances.get(&recipient).unwrap_or_default();
    self.balances.insert(&recipient, recipient_balance + amount);
    
    // 3. Interactions (external calls)
    if runtime::is_contract(recipient) {
        // Now it's safe to call external contracts
        contract::call_contract(
            recipient,
            "onReceived",
            CallFlags::ALL,
            &[sender.into_value(), amount.into_value()]
        ).ok();
    }
    
    true
}
```

## Integer Overflow and Underflow Protection

Rust provides checked arithmetic operations to prevent integer overflow/underflow:

```rust
#[method]
pub fn secure_addition(&mut self, a: u64, b: u64) -> u64 {
    // This will panic if a + b overflows, preventing an attack
    let result = a.checked_add(b).expect("Integer overflow");
    
    // Alternatively, handle the error gracefully
    let result = match a.checked_add(b) {
        Some(sum) => sum,
        None => {
            runtime::log("Addition would overflow");
            return 0;
        }
    };
    
    result
}
```

## Input Validation

Always validate user inputs before processing:

```rust
#[method]
pub fn process_payment(&mut self, recipient: Address, amount: u64) -> bool {
    // Validate inputs
    assert!(!recipient.is_zero(), "Invalid recipient address");
    assert!(amount > 0, "Amount must be greater than zero");
    assert!(amount <= 1_000_000, "Amount exceeds maximum allowed");
    
    // Process the payment
    // ...
    
    true
}
```

## Contract Permissions and Trust

Use the `#[contract_permission]` and `#[contract_trust]` attributes to restrict which contracts your contract can call and which contracts are trusted.

```rust
#[neo_contract::contract]
#[contract_permission(
    contract = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5", // GAS contract
    methods = ["transfer", "balanceOf"]
)]
pub mod secure_contract {
    // Contract implementation
}
```

## Avoiding Common Vulnerabilities

### 1. Secure Random Number Generation

Don't rely on block timestamps or hashes for randomness. Use proper sources of randomness or design your contract to not require randomness.

Insecure approach:
```rust
// DON'T DO THIS
fn generate_random(&self) -> u64 {
    let block_hash = runtime::get_current_block_hash();
    let timestamp = runtime::get_time();
    
    // Hash combination to create "random" number
    // This is predictable and manipulable by miners
    let mut hasher = Sha256::new();
    hasher.update(block_hash.as_bytes());
    hasher.update(&timestamp.to_le_bytes());
    let result = hasher.finalize();
    
    u64::from_le_bytes([
        result[0], result[1], result[2], result[3],
        result[4], result[5], result[6], result[7]
    ])
}
```

Better approaches:
- Use off-chain randomness with oracle services
- Commit-reveal schemes
- Multi-party computation

### 2. Front-Running Protection

In public blockchains, transactions can be seen in the mempool before being included in a block, enabling front-running attacks.

Protect against front-running using techniques like:
- Commit-reveal schemes
- Batch processing
- Time-locked transactions

### 3. Proper Error Handling

Handle errors gracefully to prevent unexpected behavior:

```rust
#[method]
pub fn safe_operation(&mut self) -> bool {
    // Use Result and match for error handling
    match self.perform_risky_operation() {
        Ok(value) => {
            // Process the value
            true
        },
        Err(error) => {
            runtime::log(&format!("Operation failed: {}", error));
            false
        }
    }
}
```

## Security Testing

### Unit Tests

Write tests for all security-critical functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract::testing::*;
    
    #[test]
    fn test_reentrancy_protection() {
        let mut env = TestEnvironment::new();
        let owner = env.create_account([1u8; 20]);
        
        // Create contract
        let mut contract = SecureContract::new(owner.address());
        
        // First call should succeed
        env.set_caller(owner.address());
        let result1 = contract.withdraw(100);
        
        // Simulate a reentrant call during execution
        let reentrant_result = contract.withdraw(50);
        
        // Should fail due to reentrancy protection
        assert!(!reentrant_result, "Reentrancy protection failed");
    }
    
    #[test]
    fn test_access_control() {
        let mut env = TestEnvironment::new();
        let owner = env.create_account([1u8; 20]);
        let user = env.create_account([2u8; 20]);
        
        // Create contract
        let mut contract = SecureContract::new(owner.address());
        
        // Owner should succeed
        env.set_caller(owner.address());
        let owner_result = contract.admin_function();
        assert!(owner_result, "Owner should be able to call admin function");
        
        // Non-owner should fail
        env.set_caller(user.address());
        let should_panic = std::panic::catch_unwind(|| {
            contract.admin_function()
        });
        assert!(should_panic.is_err(), "Non-owner should not be able to call admin function");
    }
}
```

### Testnet Deployment

Before mainnet deployment:
1. Deploy to a testnet
2. Perform thorough testing with realistic scenarios
3. Stress test with edge cases
4. Test with the actual front-end application

## Security Checklist

Before deploying to mainnet, review this security checklist:

- [ ] All state-modifying methods are protected against reentrancy
- [ ] Access control is properly implemented and tested
- [ ] The contract follows the check-effects-interactions pattern
- [ ] Integer arithmetic is protected against overflow and underflow
- [ ] All user inputs are validated
- [ ] Contract permissions are properly restricted
- [ ] Error handling is robust and graceful
- [ ] The contract has thorough test coverage
- [ ] The contract has been reviewed by other developers
- [ ] The contract has been deployed and tested on a testnet

## Conclusion

Security in smart contracts is a continuous process that begins with proper design and implementation. The Neo Contract Rust Framework provides several security features to help developers create secure contracts, but it's essential to understand and follow security best practices.

By implementing proper reentrancy protection, access control, and following secure coding patterns, you can minimize the risk of vulnerabilities in your Neo smart contracts. Always remember that security is not an afterthought but a fundamental aspect of smart contract development.