# Neo N3 Smart Contract Security Guide

This guide provides security best practices and patterns for developing secure smart contracts on the Neo N3 blockchain using the Neo Contract Rust framework.

## Introduction

Security is paramount in blockchain development since smart contracts often manage valuable assets and cannot be easily modified after deployment. Following security best practices can help prevent vulnerabilities that could lead to asset loss or contract compromise.

## Common Vulnerabilities and Prevention

### 1. Reentrancy Attacks

**Vulnerability:** An attacker can recursively call back into your contract before the first invocation completes, potentially manipulating state in unexpected ways.

**Prevention:**
- Use the `#[no_reentry]` attribute on sensitive methods
- Follow the checks-effects-interactions pattern (validate first, update state, then interact with external contracts)
- Use a reentrancy guard pattern

**Example:**
```rust
#[method]
#[no_reentry]
fn withdraw(&mut self, amount: u64) -> bool {
    let sender = Runtime::check_witness(&Runtime::current_sender())?;
    let balance = self.balances.get(&sender).unwrap_or_default();
    
    // Checks
    assert!(balance >= amount, "Insufficient balance");
    
    // Effects (update state)
    self.balances.insert(&sender, &(balance - amount));
    
    // Interactions (external calls)
    let gas_token = self.gas_token.get();
    self.transfer_tokens(&gas_token, &sender, amount);
    
    true
}
```

### 2. Integer Overflow/Underflow

**Vulnerability:** Mathematical operations can wrap around if they exceed the boundaries of their type.

**Prevention:**
- Enable overflow checks with `overflow-checks = true` in your Cargo.toml
- Use checked arithmetic methods (checked_add, checked_sub, etc.)
- Consider using a safe math library for complex calculations

**Example:**
```rust
// In Cargo.toml
[profile.release]
overflow-checks = true

// In code
let new_balance = balance.checked_add(amount).expect("Overflow detected");
```

### 3. Access Control Vulnerabilities

**Vulnerability:** Unauthorized users can access privileged functions if access control is missing or improperly implemented.

**Prevention:**
- Consistently use `Runtime::check_witness()` for authentication
- Implement role-based access control for admin functions
- Use access control modifiers or wrappers

**Example:**
```rust
fn only_owner(&self) {
    let caller = Runtime::check_witness(&Runtime::current_sender()).expect("Authentication failed");
    let owner = self.owner.get().expect("Owner not set");
    assert!(caller == owner, "Only owner can call this function");
}

#[method]
fn set_fee_rate(&mut self, new_rate: u16) -> bool {
    self.only_owner();
    assert!(new_rate <= 1000, "Fee rate too high"); // Max 10%
    self.fee_rate.set(new_rate);
    true
}
```

### 4. Transaction Ordering Vulnerabilities (Front-running)

**Vulnerability:** Malicious users can observe pending transactions and insert their own transactions ahead of them to gain an advantage.

**Prevention:**
- Implement commit-reveal schemes for sensitive operations
- Use maximum slippage parameters for trading operations
- Add expiration timestamps to sensitive operations

**Example:**
```rust
#[method]
fn swap_tokens(&mut self, amount_in: u64, min_amount_out: u64, deadline: u64) -> u64 {
    // Check deadline
    let current_time = Ledger::current_timestamp();
    assert!(current_time <= deadline, "Transaction expired");
    
    // Calculate output amount based on current price
    let amount_out = self.calculate_output_amount(amount_in);
    
    // Check minimum output
    assert!(amount_out >= min_amount_out, "Slippage too high");
    
    // Execute swap
    // ...
    
    amount_out
}
```

### 5. Logic Errors in Business Rules

**Vulnerability:** Flaws in business logic can lead to unintended contract behavior and vulnerabilities.

**Prevention:**
- Define clear invariants that should always hold true
- Add assertions to verify invariants are maintained
- Implement comprehensive tests for all edge cases

**Example:**
```rust
fn add_liquidity(&mut self, token_a_amount: u64, token_b_amount: u64) -> u64 {
    // ... processing ...
    
    // Invariant: product after >= product before
    let reserves_a_before = self.reserves_a.get();
    let reserves_b_before = self.reserves_b.get();
    let product_before = reserves_a_before * reserves_b_before;
    
    self.reserves_a.set(reserves_a_before + token_a_amount);
    self.reserves_b.set(reserves_b_before + token_b_amount);
    
    let reserves_a_after = self.reserves_a.get();
    let reserves_b_after = self.reserves_b.get();
    let product_after = reserves_a_after * reserves_b_after;
    
    assert!(product_after >= product_before, "Invariant violated");
    
    // ... continue processing ...
}
```

### 6. Timestamp Dependence Vulnerabilities

**Vulnerability:** Relying on timestamps that can be slightly manipulated by block producers.

**Prevention:**
- Avoid exact timestamp comparisons for high-value operations
- Use block numbers for higher security guarantees
- Add buffer periods for time-sensitive operations

**Example:**
```rust
fn is_action_allowed(&self) -> bool {
    let action_cooldown = 3600; // 1 hour in seconds
    let buffer = 300; // 5-minute buffer
    let last_action_time = self.last_action_time.get().unwrap_or(0);
    let current_time = Ledger::current_timestamp();
    
    // Add buffer to account for minor timestamp variations
    current_time >= (last_action_time + action_cooldown + buffer)
}
```

### 7. Improper Error Handling

**Vulnerability:** Poor error handling can lead to inconsistent state or unexpected behavior.

**Prevention:**
- Use proper assertions with clear error messages
- Handle all possible error cases
- Consider using a Result type for error propagation

**Example:**
```rust
fn process_transfer(&mut self, from: H160, to: H160, amount: u64) -> Result<bool, &'static str> {
    let balance = self.balances.get(&from).unwrap_or(0);
    
    if balance < amount {
        return Err("Insufficient balance");
    }
    
    if to == H160::zero() {
        return Err("Invalid recipient");
    }
    
    self.balances.insert(&from, &(balance - amount));
    let to_balance = self.balances.get(&to).unwrap_or(0);
    self.balances.insert(&to, &(to_balance + amount));
    
    Ok(true)
}
```

## Security Patterns

### 1. Pause Mechanism

Implement a pause mechanism that allows contract administrators to halt sensitive operations in case of emergencies.

```rust
#[storage]
struct Contract {
    paused: Item<bool>,
    owner: Item<H160>,
    // ... other storage items
}

impl Contract {
    fn ensure_not_paused(&self) {
        assert!(!self.paused.get().unwrap_or(false), "Contract is paused");
    }
    
    #[method]
    fn pause(&mut self) -> bool {
        // Only owner can pause
        let caller = Runtime::check_witness(&Runtime::current_sender())?;
        assert!(caller == self.owner.get(), "Only owner can pause");
        
        self.paused.set(true);
        true
    }
    
    #[method]
    fn unpause(&mut self) -> bool {
        // Only owner can unpause
        let caller = Runtime::check_witness(&Runtime::current_sender())?;
        assert!(caller == self.owner.get(), "Only owner can unpause");
        
        self.paused.set(false);
        true
    }
    
    #[method]
    fn sensitive_operation(&mut self) -> bool {
        self.ensure_not_paused();
        // ... implementation
    }
}
```

### 2. Multi-signature Authorization

For high-value operations, require multiple signatures to proceed:

```rust
#[storage]
struct MultiSigWallet {
    owners: Map<H160, bool>,
    required_confirmations: Item<u32>,
    transaction_count: Item<u32>,
    transactions: Map<u32, Transaction>,
    confirmations: Map<(u32, H160), bool>,
}

impl MultiSigWallet {
    #[method]
    fn submit_transaction(&mut self, destination: H160, value: u64, data: ByteString) -> u32 {
        let sender = Runtime::check_witness(&Runtime::current_sender())?;
        assert!(self.owners.get(&sender).unwrap_or(false), "Not an owner");
        
        let tx_id = self.transaction_count.get().unwrap_or(0);
        self.transactions.insert(&tx_id, &Transaction { 
            destination, 
            value, 
            data, 
            executed: false 
        });
        self.transaction_count.set(tx_id + 1);
        
        // Auto-confirm by submitter
        self.confirmations.insert(&(tx_id, sender), &true);
        
        tx_id
    }
    
    #[method]
    fn confirm_transaction(&mut self, tx_id: u32) -> bool {
        let sender = Runtime::check_witness(&Runtime::current_sender())?;
        assert!(self.owners.get(&sender).unwrap_or(false), "Not an owner");
        assert!(self.transactions.get(&tx_id).is_some(), "Transaction does not exist");
        assert!(!self.confirmations.get(&(tx_id, sender)).unwrap_or(false), "Transaction already confirmed");
        
        self.confirmations.insert(&(tx_id, sender), &true);
        
        // Execute if enough confirmations
        self.execute_transaction_if_confirmed(tx_id)
    }
    
    fn execute_transaction_if_confirmed(&mut self, tx_id: u32) -> bool {
        // ... implementation to check confirmations and execute
    }
}
```

### 3. Rate Limiting

Implement rate limiting to prevent spam and abuse:

```rust
#[storage]
struct RateLimitedContract {
    action_counts: Map<(H160, u64), u32>, // Maps (address, day) to count
    action_limits: Item<u32>,
}

impl RateLimitedContract {
    fn check_rate_limit(&mut self, address: H160) -> bool {
        let current_time = Ledger::current_timestamp();
        let day = current_time / 86400; // Convert to day
        
        let key = (address, day);
        let count = self.action_counts.get(&key).unwrap_or(0);
        let limit = self.action_limits.get().unwrap_or(10);
        
        if count >= limit {
            return false;
        }
        
        self.action_counts.insert(&key, &(count + 1));
        true
    }
    
    #[method]
    fn perform_limited_action(&mut self) -> bool {
        let sender = Runtime::check_witness(&Runtime::current_sender())?;
        assert!(self.check_rate_limit(sender), "Rate limit exceeded");
        
        // ... action implementation
        true
    }
}
```

### 4. Withdrawal Pattern

Use the withdrawal pattern to separate asset transfers from business logic:

```rust
#[storage]
struct SafePayment {
    payments: Map<H160, u64>,
}

impl SafePayment {
    #[method]
    fn deposit(&mut self, recipient: H160) -> bool {
        let sender = Runtime::check_witness(&Runtime::current_sender())?;
        // ... handle deposit and accounting
        true
    }
    
    // Separate withdrawal method that recipients call themselves
    #[method]
    fn withdraw(&mut self) -> bool {
        let recipient = Runtime::check_witness(&Runtime::current_sender())?;
        let amount = self.payments.get(&recipient).unwrap_or(0);
        assert!(amount > 0, "No funds to withdraw");
        
        // Clear state before transfer
        self.payments.insert(&recipient, &0);
        
        // Perform transfer after state update
        let gas_token = self.gas_token.get();
        self.transfer_token(&gas_token, &recipient, amount);
        
        true
    }
}
```

## Testing for Security

### Use Unit Tests

Write comprehensive unit tests for all contract functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deposit_withdraw_flow() {
        let mut contract = Contract::new();
        
        // Test deposit
        assert!(contract.deposit(100));
        assert_eq!(contract.get_balance(), 100);
        
        // Test withdraw
        assert!(contract.withdraw(50));
        assert_eq!(contract.get_balance(), 50);
        
        // Test withdraw too much
        assert!(!contract.withdraw(100));
        assert_eq!(contract.get_balance(), 50);
    }
    
    #[test]
    fn test_authorization() {
        // ... test that unauthorized accounts cannot access restricted methods
    }
    
    #[test]
    fn test_edge_cases() {
        // ... test boundary conditions and edge cases
    }
}
```

### Use Fuzzing Tests

Consider using fuzzing tools to test against unexpected inputs:

```rust
#[cfg(test)]
#[fuzz]
fn fuzz_deposit_withdraw(amount_in: u64, amount_out: u64) {
    let mut contract = Contract::new();
    
    // Ensure we handle all possible input combinations gracefully
    let _ = contract.deposit(amount_in);
    let _ = contract.withdraw(amount_out);
    
    // Verify invariants still hold
    assert!(contract.get_balance() <= amount_in);
}
```

### Perform Security Audits

Before deploying high-value contracts:

1. Conduct a thorough internal code review
2. Consider a professional security audit
3. Use static analysis tools if available
4. Perform penetration testing with simulated attacks

## Pre-deployment Checklist

Before deploying to mainnet, verify:

1. **Contract Logic**: Ensure all contract logic works as expected
2. **Access Controls**: Verify proper access controls on all sensitive functions
3. **Error Handling**: Check all error conditions are properly handled
4. **Gas Efficiency**: Optimize for gas efficiency where possible
5. **External Calls**: Review all external contract calls for security concerns
6. **Arithmetic**: Check for any potential overflows or underflows
7. **Timestamp Usage**: Verify timestamp usage is not vulnerable to manipulation
8. **Upgradeability**: If the contract needs to be upgradable, ensure the upgrade process is secure
9. **Event Emissions**: Confirm all important state changes emit appropriate events
10. **Documentation**: Document all security considerations for users

## Additional Resources

- [Neo Security Best Practices](https://docs.neo.org/docs/en-us/sc/security.html)
- [Smart Contract Weakness Classification (SWC) Registry](https://swcregistry.io/)
- [OWASP Smart Contract Security Verification Standard](https://github.com/OWASP/SCSVS)

## Conclusion

Security should be a primary consideration throughout the development lifecycle of your Neo N3 smart contracts. By following these best practices and patterns, you can mitigate common vulnerabilities and build more secure dApps on the Neo blockchain.

Remember that security is an ongoing process, and staying informed about the latest security developments is essential for maintaining secure contracts. 