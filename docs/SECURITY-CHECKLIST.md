# NEO Smart Contract Security Checklist

This document provides a security checklist for NEO N3 smart contracts, following our documentation-first approach.

## Overview

Smart contract security is critical as contracts on the blockchain are immutable and directly handle digital assets. This checklist helps identify and mitigate common security risks before deployment.

## Documentation-First Security Approach

Following our documentation-first philosophy, we recommend:

1. **Document Security Requirements First**: Define security requirements before implementation
2. **Create Security Tests**: Document and implement tests for all security concerns
3. **Document Audit Procedures**: Define how to audit your contract before deployment
4. **Document Incident Response**: Plan how to respond to security incidents

## Access Control Vulnerabilities

### 1. Improper Authentication
- [ ] All sensitive functions verify the caller's identity with `Runtime::check_witness`
- [ ] Functions that modify contract state verify appropriate permissions
- [ ] Admin/owner functions are properly restricted

**Example of proper authentication:**
```rust
// #[method]
pub fn update_owner(&mut self, new_owner: H160) -> bool {
    // Only current owner can update owner
    let current_owner = self.owner.get().unwrap_or(None).unwrap_or_default();
    if !Runtime::check_witness(&current_owner) {
        return false;
    }
    
    // Update owner
    self.owner.set(&new_owner).unwrap_or(());
    
    true
}
```

### 2. Improper Authorization Checks
- [ ] Contract checks that the transaction signer is authorized to perform the operation
- [ ] Multi-signature operations are properly implemented
- [ ] Role-based access control is implemented for contracts with multiple access levels

## Input Validation Vulnerabilities

### 1. Missing Input Validation
- [ ] All method parameters are validated before use
- [ ] Methods check for invalid inputs, including edge cases
- [ ] Methods validate numerical inputs (zero amounts, overflows, etc.)

**Example of proper input validation:**
```rust
// #[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
    // Check that amount is positive
    if amount == 0 {
        return false;
    }
    
    // Check from has authorized the transfer
    if !Runtime::check_witness(&from) {
        return false;
    }
    
    // Check from has sufficient balance
    let from_balance = self.balance_of(from);
    if from_balance < amount {
        return false;
    }
    
    // Additional validation...
}
```

### 2. Integer Overflow/Underflow
- [ ] All mathematical operations are checked for overflow/underflow
- [ ] Use checked arithmetic operations where appropriate
- [ ] Validate that balances don't underflow or overflow

## Logic Vulnerabilities

### 1. Reentrancy
- [ ] Contract state is updated before calling external contracts
- [ ] Contract uses a reentrancy guard when appropriate
- [ ] Contract is tested for reentrancy vulnerabilities

**Example of avoiding reentrancy:**
```rust
// #[method]
// #[no_reentrant]  // Would use an annotation in the future
pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
    // Validate...
    
    // Update state BEFORE external calls
    self.balances.set(&from, &new_from_balance).unwrap_or(());
    self.balances.set(&to, &new_to_balance).unwrap_or(());
    
    // Emit events
    Transfer::emit(Some(from), Some(to), amount);
    
    // External call comes AFTER state updates
    if is_contract(&to) {
        // Call to external contract
    }
    
    true
}
```

### 2. Logic Errors
- [ ] Contract logic is thoroughly tested with unit tests
- [ ] All execution paths are tested, including failure paths
- [ ] Edge cases are identified and tested

## Storage Vulnerabilities

### 1. Improper Storage Management
- [ ] Storage keys use appropriate namespaces to avoid conflicts
- [ ] Storage operations handle errors properly
- [ ] Contract doesn't store unnecessary data

**Example of proper storage management:**
```rust
// #[storage]
pub struct TokenContract {
    // Proper namespacing with unique keys
    pub balances: StorageMap<H160, u64>,  // Uses "balances" prefix
    pub total_supply: Item<u64>,          // Uses "total_supply" key
    // ...
}

impl TokenContract {
    pub fn new() -> Self {
        Self {
            balances: StorageMap::new(b"balances"),
            total_supply: Item::new(b"total_supply"),
            // ...
        }
    }
}
```

### 2. Storage Manipulation
- [ ] Contract prevents unauthorized storage modification
- [ ] Contract uses appropriate storage patterns (maps, items, etc.)
- [ ] Storage operations are atomic where appropriate

## Event Emission Vulnerabilities

### 1. Missing Events
- [ ] Contract emits events for all state changes
- [ ] Events provide sufficient information for off-chain tracking
- [ ] Events are tested to ensure they're emitted correctly

**Example of proper event emission:**
```rust
// Event emission for a transfer
pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
    let mut event_args = Array::new();
    
    // Add parameters...
    
    // Emit event with all required information
    Runtime::notify(&ByteString::from("Transfer"), &event_args);
}
```

### 2. Incorrect Events
- [ ] Events contain accurate information
- [ ] Events follow NEP standards where applicable
- [ ] Event parameters are in the correct order

## Token-Specific Vulnerabilities

### 1. NEP-17 Token Standard Compliance
- [ ] Implementation follows NEP-17 standard completely
- [ ] `transfer` function validates sender's identity
- [ ] `balanceOf` and other read-only methods work correctly
- [ ] `Transfer` events are emitted with correct parameters

### 2. Token Minting/Burning
- [ ] Only authorized users can mint tokens
- [ ] Burn operations validate ownership
- [ ] Total supply is properly updated

## Contract Upgrade Vulnerabilities

### 1. Unsafe Upgrade Mechanism
- [ ] Upgrade process is properly authenticated
- [ ] Upgrade process preserves contract state
- [ ] Upgrade process is tested thoroughly

### 2. Missing Upgrade Capability
- [ ] Contract includes upgrade capability if needed
- [ ] Upgrade process is properly documented
- [ ] Fallback mechanisms exist if upgrade fails

## Gas Optimization and DoS Protection

### 1. Gas Inefficiency
- [ ] Contract optimizes storage operations
- [ ] Contract avoids unnecessary computations
- [ ] Contract handles large data structures efficiently

### 2. Denial of Service Vulnerabilities
- [ ] Contract limits iteration over unbounded data structures
- [ ] Contract handles out of gas scenarios gracefully
- [ ] Contract avoids operations that could be exploited to consume excessive gas

## Security Testing

### 1. Unit Tests
- [ ] Each security check has corresponding unit tests
- [ ] All methods have tests for success and failure cases
- [ ] Authentication and authorization are thoroughly tested

**Example of security unit test:**
```rust
#[test]
fn test_only_owner_can_mint() {
    // Arrange
    let mut contract = setup_test_contract();
    let owner = H160::from_hex_string("0x1234...").unwrap();
    let user = H160::from_hex_string("0x9876...").unwrap();
    
    // Reset mock state and add user witness (non-owner)
    test_runtime::reset();
    test_runtime::add_witness(user);
    
    // Act
    let result = contract.mint(user, 1000);
    
    // Assert
    assert!(!result); // Should fail because user is not owner
}
```

### 2. Integration Tests
- [ ] Contract interactions are tested in a blockchain environment
- [ ] Multi-step processes are tested end-to-end
- [ ] Contract performs correctly in a realistic environment

## Pre-Deployment Security Checklist

Before deploying to mainnet, verify:

- [ ] All items in this checklist have been addressed
- [ ] Contract has been audited by a third party (if possible)
- [ ] Contract has been deployed and tested on testnet
- [ ] All known vulnerabilities have been mitigated
- [ ] Documentation accurately reflects contract behavior
- [ ] Incident response plan is in place

## Post-Deployment Monitoring

After deployment, monitor for:

- [ ] Unusual transaction patterns
- [ ] Unexpected state changes
- [ ] Gas usage anomalies
- [ ] Failed transactions
- [ ] Contract invocation errors

## Security Resources

- [NEO Smart Contract Best Practices](https://developers.neo.org/docs/n3/develop/write/best-practice)
- [Smart Contract Security Guidelines](https://consensys.github.io/smart-contract-best-practices/)
- [NEO Enhancement Proposals (NEPs)](https://github.com/neo-project/proposals)

## Conclusion

Following a documentation-first approach to security ensures that your Neo contracts are designed with security in mind from the beginning. By documenting security requirements and potential vulnerabilities before implementation, you can build more secure contracts and minimize the risk of security incidents.

Remember that this checklist is not exhaustive, and security is an ongoing process. Regular audits, testing, and monitoring are essential to maintaining contract security over time. 