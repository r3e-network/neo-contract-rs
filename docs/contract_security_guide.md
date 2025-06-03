# Neo Contract Security Guide

This guide outlines security best practices for developing Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Smart contract security is critical for protecting user funds and maintaining system integrity. This guide covers common vulnerabilities, mitigation strategies, and secure coding patterns specific to Neo N3 contracts.

## Common Security Vulnerabilities

### 1. Reentrancy Attacks

Reentrancy occurs when a contract function can be called recursively before the first execution completes.

#### Problem:
```rust
// Vulnerable to reentrancy
impl MyContract {
    #[method]
    pub fn withdraw(&mut self, amount: u64) -> bool {
        let caller = Runtime::calling_script_hash();
        let balance = self.balances.get(&caller).unwrap_or(0);
        
        if balance >= amount {
            // Vulnerable: External call before state update
            if Runtime::call_contract(caller, "receive", &[Any::from(amount)]) {
                self.balances.put(&caller, balance - amount);
                return true;
            }
        }
        false
    }
}
```

#### Solution:
```rust
impl MyContract {
    #[method]
    #[no_reentrant] // Use the framework's reentrancy protection
    pub fn withdraw(&mut self, amount: u64) -> bool {
        let caller = Runtime::calling_script_hash();
        let balance = self.balances.get(&caller).unwrap_or(0);
        
        if balance >= amount {
            // Secure: Update state before external call
            self.balances.put(&caller, balance - amount);
            
            // External call after state update
            Runtime::call_contract(caller, "receive", &[Any::from(amount)]);
            return true;
        }
        false
    }
}
```

### 2. Integer Overflow/Underflow

#### Problem:
```rust
// Vulnerable to overflow
#[method]
pub fn unsafe_mint(&mut self, to: H160, amount: u64) {
    let current_balance = self.balances.get(&to).unwrap_or(0);
    self.balances.put(&to, current_balance + amount); // Potential overflow
}
```

#### Solution:
```rust
#[method]
pub fn safe_mint(&mut self, to: H160, amount: u64) -> bool {
    let current_balance = self.balances.get(&to).unwrap_or(0);
    
    // Check for overflow
    if let Some(new_balance) = current_balance.checked_add(amount) {
        self.balances.put(&to, new_balance);
        true
    } else {
        false // Reject on overflow
    }
}
```

### 3. Access Control Violations

#### Problem:
```rust
// Missing access control
#[method]
pub fn admin_function(&mut self) {
    // Anyone can call this!
    self.critical_parameter.put(new_value);
}
```

#### Solution:
```rust
impl MyContract {
    fn require_owner(&self) -> bool {
        Runtime::calling_script_hash() == self.owner.get().unwrap_or(H160::zero())
    }
    
    #[method]
    pub fn admin_function(&mut self) -> bool {
        if !self.require_owner() {
            return false;
        }
        
        self.critical_parameter.put(new_value);
        true
    }
}
```

## Security Patterns

### 1. Checks-Effects-Interactions Pattern

Always follow this order:
1. **Checks**: Validate inputs and conditions
2. **Effects**: Update contract state
3. **Interactions**: Call external contracts

```rust
#[method]
pub fn secure_transfer(&mut self, to: H160, amount: u64) -> bool {
    let caller = Runtime::calling_script_hash();
    
    // 1. CHECKS
    if amount == 0 {
        return false;
    }
    
    let from_balance = self.balances.get(&caller).unwrap_or(0);
    if from_balance < amount {
        return false;
    }
    
    // 2. EFFECTS
    let to_balance = self.balances.get(&to).unwrap_or(0);
    self.balances.put(&caller, from_balance - amount);
    self.balances.put(&to, to_balance + amount);
    
    // 3. INTERACTIONS (if any)
    self.emit_transfer_event(caller, to, amount);
    
    true
}
```

### 2. Input Validation

```rust
impl MyContract {
    fn validate_address(&self, addr: H160) -> bool {
        addr != H160::zero() && Runtime::check_witness(&addr)
    }
    
    fn validate_amount(&self, amount: u64) -> bool {
        amount > 0 && amount <= MAX_TRANSFER_AMOUNT
    }
    
    #[method]
    pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
        // Validate inputs
        if !self.validate_address(to) {
            return false;
        }
        
        if !self.validate_amount(amount) {
            return false;
        }
        
        // Continue with transfer logic...
        true
    }
}
```

### 3. Safe Math Operations

```rust
impl SafeMath {
    pub fn safe_add(a: u64, b: u64) -> Option<u64> {
        a.checked_add(b)
    }
    
    pub fn safe_sub(a: u64, b: u64) -> Option<u64> {
        a.checked_sub(b)
    }
    
    pub fn safe_mul(a: u64, b: u64) -> Option<u64> {
        a.checked_mul(b)
    }
    
    pub fn safe_div(a: u64, b: u64) -> Option<u64> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

impl MyContract {
    #[method]
    pub fn calculate_fee(&self, amount: u64, rate: u64) -> Option<u64> {
        SafeMath::safe_mul(amount, rate)
            .and_then(|result| SafeMath::safe_div(result, 10000))
    }
}
```

### 4. Emergency Controls

```rust
#[contract]
pub struct SecureContract {
    #[storage]
    owner: StorageItem<H160>,
    
    #[storage]
    paused: StorageItem<bool>,
    
    #[storage]
    emergency_stop: StorageItem<bool>,
}

impl SecureContract {
    fn require_not_paused(&self) -> bool {
        !self.paused.get().unwrap_or(false)
    }
    
    fn require_not_emergency(&self) -> bool {
        !self.emergency_stop.get().unwrap_or(false)
    }
    
    #[method]
    pub fn pause(&mut self) -> bool {
        if !self.require_owner() {
            return false;
        }
        
        self.paused.put(true);
        true
    }
    
    #[method]
    pub fn emergency_stop(&mut self) -> bool {
        if !self.require_owner() {
            return false;
        }
        
        self.emergency_stop.put(true);
        true
    }
    
    #[method]
    pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
        // Security checks
        if !self.require_not_paused() || !self.require_not_emergency() {
            return false;
        }
        
        // Transfer logic...
        true
    }
}
```

## Authentication and Authorization

### 1. Witness Verification

```rust
impl MyContract {
    fn require_witness(&self, account: H160) -> bool {
        Runtime::check_witness(&account)
    }
    
    #[method]
    pub fn withdraw(&mut self, amount: u64) -> bool {
        let caller = Runtime::calling_script_hash();
        
        // Ensure the caller has proper authorization
        if !self.require_witness(caller) {
            return false;
        }
        
        // Withdrawal logic...
        true
    }
}
```

### 2. Role-Based Access Control

```rust
#[derive(Clone, Copy, PartialEq)]
pub enum Role {
    Owner,
    Admin,
    User,
}

#[contract]
pub struct RoleBasedContract {
    #[storage]
    roles: StorageMap<H160, Role>,
    
    #[storage]
    owner: StorageItem<H160>,
}

impl RoleBasedContract {
    fn get_role(&self, account: H160) -> Role {
        self.roles.get(&account).unwrap_or(Role::User)
    }
    
    fn require_role(&self, required_role: Role) -> bool {
        let caller = Runtime::calling_script_hash();
        let caller_role = self.get_role(caller);
        
        match required_role {
            Role::Owner => caller == self.owner.get().unwrap_or(H160::zero()),
            Role::Admin => caller_role == Role::Admin || caller_role == Role::Owner,
            Role::User => true, // Anyone can have user role
        }
    }
    
    #[method]
    pub fn admin_function(&mut self) -> bool {
        if !self.require_role(Role::Admin) {
            return false;
        }
        
        // Admin-only logic...
        true
    }
}
```

## Data Validation

### 1. Input Sanitization

```rust
impl MyContract {
    fn sanitize_string(&self, input: &str) -> Option<String> {
        if input.len() > MAX_STRING_LENGTH {
            return None;
        }
        
        // Remove control characters
        let sanitized: String = input.chars()
            .filter(|c| !c.is_control())
            .collect();
            
        if sanitized.is_empty() {
            None
        } else {
            Some(sanitized)
        }
    }
    
    #[method]
    pub fn set_name(&mut self, name: String) -> bool {
        if let Some(clean_name) = self.sanitize_string(&name) {
            self.name.put(clean_name);
            true
        } else {
            false
        }
    }
}
```

### 2. Range Validation

```rust
impl MyContract {
    const MIN_AMOUNT: u64 = 1;
    const MAX_AMOUNT: u64 = 1_000_000_000;
    
    fn validate_amount(&self, amount: u64) -> bool {
        amount >= Self::MIN_AMOUNT && amount <= Self::MAX_AMOUNT
    }
    
    fn validate_percentage(&self, percentage: u64) -> bool {
        percentage <= 10000 // Assuming basis points (0-100%)
    }
    
    #[method]
    pub fn set_fee_rate(&mut self, rate: u64) -> bool {
        if !self.require_owner() || !self.validate_percentage(rate) {
            return false;
        }
        
        self.fee_rate.put(rate);
        true
    }
}
```

## Cross-Contract Security

### 1. Safe External Calls

```rust
impl MyContract {
    fn safe_call_contract(&self, contract: H160, method: &str, args: &[Any]) -> bool {
        // Validate contract address
        if contract == H160::zero() {
            return false;
        }
        
        // Use try_call to handle failures gracefully
        match Runtime::try_call_contract(contract, method, args) {
            Ok(result) => {
                // Handle successful call
                true
            }
            Err(_) => {
                // Handle failure without panicking
                false
            }
        }
    }
}
```

### 2. Contract Interface Validation

```rust
impl MyContract {
    fn validate_token_contract(&self, token: H160) -> bool {
        // Check if contract implements required methods
        let methods = ["symbol", "decimals", "totalSupply"];
        
        for method in &methods {
            if !self.contract_has_method(token, method) {
                return false;
            }
        }
        
        true
    }
    
    #[method]
    pub fn add_supported_token(&mut self, token: H160) -> bool {
        if !self.require_owner() {
            return false;
        }
        
        if !self.validate_token_contract(token) {
            return false;
        }
        
        self.supported_tokens.put(&token, true);
        true
    }
}
```

## Testing Security

### 1. Security-Focused Tests

```rust
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_overflow_protection() {
        let mut contract = MyContract::new();
        
        // Test addition overflow
        let max_balance = u64::MAX - 100;
        contract.balances.put(&alice, max_balance);
        
        // This should fail due to overflow protection
        assert!(!contract.safe_mint(alice, 200));
        
        // Balance should remain unchanged
        assert_eq!(contract.balance_of(alice), max_balance);
    }
    
    #[test]
    fn test_access_control() {
        let mut contract = MyContract::new();
        
        // Non-owner should not be able to call admin functions
        assert!(!contract.admin_function());
        
        // Set proper owner
        contract.owner.put(owner_address);
        
        // Now it should work
        assert!(contract.admin_function());
    }
    
    #[test]
    fn test_reentrancy_protection() {
        let mut contract = MyContract::new();
        
        // Set up initial state
        contract.balances.put(&user, 1000);
        
        // Attempt reentrancy attack
        // This should be prevented by #[no_reentrant]
        assert!(contract.withdraw(500));
        
        // Verify state is correct
        assert_eq!(contract.balance_of(user), 500);
    }
}
```

## Security Checklist

Before deploying your contract, verify:

### Access Control
- [ ] All administrative functions have proper access control
- [ ] Witness verification is implemented where needed
- [ ] Role-based permissions are correctly implemented

### Input Validation
- [ ] All inputs are validated for range and format
- [ ] String inputs are sanitized
- [ ] Numeric inputs check for overflow/underflow

### State Management
- [ ] State updates follow checks-effects-interactions pattern
- [ ] Reentrancy protection is in place
- [ ] Emergency stop mechanisms are implemented

### External Interactions
- [ ] External contract calls are handled safely
- [ ] Contract interfaces are validated
- [ ] Failures are handled gracefully

### Testing
- [ ] Security-focused test cases are written
- [ ] Edge cases are tested
- [ ] Attack scenarios are simulated

## Common Anti-Patterns

### 1. Avoid These Patterns

```rust
// DON'T: Direct arithmetic without overflow checks
let new_balance = old_balance + amount;

// DON'T: Missing access control
#[method]
pub fn admin_only_function(&mut self) {
    // Missing authorization check
}

// DON'T: External calls before state updates
if external_call_success {
    self.update_state();
}

// DON'T: Ignoring return values
Runtime::call_contract(target, "method", &args);
```

### 2. Use These Instead

```rust
// DO: Safe arithmetic
if let Some(new_balance) = old_balance.checked_add(amount) {
    // Safe to proceed
}

// DO: Proper access control
#[method]
pub fn admin_only_function(&mut self) -> bool {
    if !self.require_owner() {
        return false;
    }
    // Function logic
}

// DO: State updates before external calls
self.update_state();
let result = external_call();

// DO: Handle return values
if Runtime::call_contract(target, "method", &args) {
    // Handle success
} else {
    // Handle failure
}
```

## See Also

- [Storage Guide](storage_guide.md)
- [Events Guide](events_guide.md)
- [Gas Optimization Guide](gas_optimization.md)
- [Cross-Contract Communication Guide](cross_contract_guide.md) 