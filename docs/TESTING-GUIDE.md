# Testing NEO Smart Contracts

This guide explains how to test your NEO N3 smart contracts using the documentation-first approach with Rust.

## Overview

Testing is an essential part of smart contract development. Because blockchain deployments are immutable, thorough testing is required before deployment to ensure your contract behaves as expected under all conditions.

## Documentation-First Testing Approach

Following our documentation-first philosophy, we recommend defining tests that validate both the documented intent and the actual implementation:

1. **Document Expected Behavior First**: Write test specifications before implementation
2. **Test Structure Mirrors Documentation**: Structure tests to match documented features
3. **Validate Annotation Compliance**: Test that implementations follow documented annotation patterns

## Test Types for NEO Contracts

### 1. Unit Tests

Unit tests validate individual functions in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transfer_success() {
        // Arrange
        let mut contract = token_contract::TokenContract::new(
            H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap(),
            1000
        );
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Mock Runtime functions
        // (would use a test framework to handle this in a real implementation)
        
        // Act
        let result = contract.transfer(from, to, 100, vec![]);
        
        // Assert
        assert!(result);
        assert_eq!(contract.balance_of(from), 900);
        assert_eq!(contract.balance_of(to), 100);
    }
    
    #[test]
    fn test_transfer_insufficient_balance() {
        // Arrange
        let mut contract = token_contract::TokenContract::new(
            H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap(),
            100
        );
        let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        
        // Act
        let result = contract.transfer(from, to, 200, vec![]);
        
        // Assert
        assert!(!result);
        assert_eq!(contract.balance_of(from), 100);
        assert_eq!(contract.balance_of(to), 0);
    }
}
```

### 2. Blockchain Simulation Tests

These tests simulate blockchain operations using a testing framework.

```rust
#[cfg(test)]
mod blockchain_tests {
    use super::*;
    use neo_test_framework::{TestBuilder, Account, TestTransaction};
    
    #[test]
    fn test_deploy_and_transfer() {
        // Arrange
        let test = TestBuilder::new()
            .add_account("owner", 10_000_000)
            .add_account("user", 10_000_000)
            .build();
            
        // Deploy contract
        let owner = test.get_account("owner");
        let contract_hash = test.deploy_contract("token_contract.nef", owner);
        
        // Act: Initialize with 1000 tokens
        let tx1 = TestTransaction::new()
            .contract(contract_hash)
            .method("deploy")
            .signer(owner)
            .build();
        let result1 = test.execute_transaction(tx1);
        
        // Assert initialization succeeded
        assert!(result1.success());
        
        // Act: Transfer tokens
        let user = test.get_account("user");
        let tx2 = TestTransaction::new()
            .contract(contract_hash)
            .method("transfer")
            .args(&[
                owner.address().into(),
                user.address().into(),
                100_i64.into()
            ])
            .signer(owner)
            .build();
        let result2 = test.execute_transaction(tx2);
        
        // Assert transfer succeeded
        assert!(result2.success());
        
        // Check balances
        let owner_balance = test.call_contract(
            contract_hash, 
            "balanceOf",
            &[owner.address().into()]
        ).as_int().unwrap();
        
        let user_balance = test.call_contract(
            contract_hash,
            "balanceOf",
            &[user.address().into()]
        ).as_int().unwrap();
        
        assert_eq!(owner_balance, 900);
        assert_eq!(user_balance, 100);
    }
}
```

### 3. Annotation Compliance Tests

These tests verify that your manual implementation behaves according to the annotation-defined contract.

```rust
#[cfg(test)]
mod annotation_tests {
    use super::*;
    
    #[test]
    fn test_event_emission_matches_annotation() {
        // This test checks that event emissions match what would be expected
        // from the #[event] annotation
        
        // Arrange
        let mut events = Vec::new();
        // Mock Runtime::notify to capture events
        // (would use a test framework to handle this in reality)
        
        // Act
        token_contract::Transfer::emit(
            Some(H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap()),
            Some(H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap()),
            100
        );
        
        // Assert
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.name, "Transfer");
        assert_eq!(event.args.len(), 3);
        // Further assertions about event format
    }
    
    #[test]
    fn test_storage_accessors_match_annotation() {
        // This test checks that storage behaviors match what would be expected
        // from the #[storage] annotation
        
        // Arrange
        let contract = token_contract::TokenContract::new(
            H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap(),
            1000
        );
        
        // Act & Assert
        // Verify that storage access methods work as expected
        assert_eq!(contract.balance_of(
            H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap()
        ), 1000);
    }
}
```

## Setting Up a Testing Environment

### Local Testing Using Mock Runtime

Since Neo contracts rely on the blockchain runtime, you'll need to mock these interactions for unit tests:

```rust
// Mock runtime for testing
#[cfg(test)]
mod test_runtime {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    
    thread_local! {
        static STORAGE: RefCell<HashMap<Vec<u8>, Vec<u8>>> = RefCell::new(HashMap::new());
        static EVENTS: RefCell<Vec<(String, Vec<Any>)>> = RefCell::new(Vec::new());
        static WITNESSES: RefCell<Vec<H160>> = RefCell::new(Vec::new());
    }
    
    pub fn mock_storage_put(key: &[u8], value: &[u8]) {
        STORAGE.with(|s| {
            s.borrow_mut().insert(key.to_vec(), value.to_vec());
        });
    }
    
    pub fn mock_storage_get(key: &[u8]) -> Option<Vec<u8>> {
        STORAGE.with(|s| {
            s.borrow().get(key).cloned()
        })
    }
    
    pub fn mock_check_witness(address: &H160) -> bool {
        WITNESSES.with(|w| {
            w.borrow().contains(address)
        })
    }
    
    pub fn mock_notify(name: &str, args: &[Any]) {
        EVENTS.with(|e| {
            e.borrow_mut().push((name.to_string(), args.to_vec()));
        });
    }
    
    pub fn add_witness(address: H160) {
        WITNESSES.with(|w| {
            w.borrow_mut().push(address);
        });
    }
    
    pub fn get_events() -> Vec<(String, Vec<Any>)> {
        EVENTS.with(|e| {
            e.borrow().clone()
        })
    }
    
    pub fn reset() {
        STORAGE.with(|s| s.borrow_mut().clear());
        EVENTS.with(|e| e.borrow_mut().clear());
        WITNESSES.with(|w| w.borrow_mut().clear());
    }
}
```

### Network Testing with Neo Express

For integration testing with a NEO blockchain:

1. Install Neo Express from the Neo Blockchain Toolkit
2. Create a private network:
   ```bash
   neoxp create
   ```
3. Deploy your contract to the private network:
   ```bash
   neoxp contract deploy path/to/contract.nef owner
   ```
4. Test contract operations:
   ```bash
   neoxp contract invoke $CONTRACT_HASH transfer '["NZNos2WqTbu5oCgyfss9kUJhwU4nyYL39w", "NhxK8rNVnWjfdczokZkKJK6zYHc5h4cXxw", 100]' --account owner
   ```

## Test Patterns for NEO Contracts

### 1. Test Event Emissions

Events are crucial for off-chain applications to understand what's happening in your contract.

```rust
#[test]
fn test_transfer_emits_event() {
    // Arrange
    let mut contract = setup_test_contract();
    let from = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
    let to = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
    
    // Reset mocked events
    test_runtime::reset();
    test_runtime::add_witness(from);
    
    // Act
    contract.transfer(from, to, 100, vec![]);
    
    // Assert
    let events = test_runtime::get_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].0, "Transfer");
    // Check event parameters
    let args = &events[0].1;
    assert_eq!(args.len(), 3);
    assert_eq!(args[0].as_h160(), Some(&from));
    assert_eq!(args[1].as_h160(), Some(&to));
    assert_eq!(args[2].as_i64(), Some(100));
}
```

### 2. Test Access Control

Security is critical for blockchain applications. Test that only authorized users can perform restricted operations.

```rust
#[test]
fn test_mint_requires_owner() {
    // Arrange
    let mut contract = setup_test_contract();
    let owner = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
    let user = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
    
    // Test 1: Non-owner can't mint
    test_runtime::reset();
    test_runtime::add_witness(user); // User tries to mint
    
    // Act & Assert
    let result1 = contract.mint(user, 1000);
    assert!(!result1); // Should fail
    
    // Test 2: Owner can mint
    test_runtime::reset();
    test_runtime::add_witness(owner); // Owner tries to mint
    
    // Act & Assert
    let result2 = contract.mint(user, 1000);
    assert!(result2); // Should succeed
    assert_eq!(contract.balance_of(user), 1000);
}
```

### 3. Test Edge Cases

Always test boundary conditions and edge cases, especially for financial contracts.

```rust
#[test]
fn test_transfer_edge_cases() {
    // Arrange
    let mut contract = setup_test_contract();
    let account = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
    
    // Setup
    test_runtime::reset();
    test_runtime::add_witness(account);
    
    // Test cases
    
    // Zero amount transfer
    assert!(!contract.transfer(account, account, 0, vec![]));
    
    // Transfer to self
    contract.mint(account, 100);
    assert!(contract.transfer(account, account, 50, vec![]));
    assert_eq!(contract.balance_of(account), 100); // Balance unchanged
    
    // Transfer exact balance
    let recipient = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
    assert!(contract.transfer(account, recipient, 100, vec![]));
    assert_eq!(contract.balance_of(account), 0);
    assert_eq!(contract.balance_of(recipient), 100);
}
```

## Contract Feature Testing Matrix

When testing your contract, ensure you cover all features defined in your documentation. Here's a sample test matrix for a NEP-17 token:

| Feature | Test Cases | Validated By |
|---------|------------|-------------|
| `symbol` | Returns correct symbol | Unit test |
| `decimals` | Returns correct decimal count | Unit test |
| `totalSupply` | Reports accurate supply after mint/burn | Unit + Integration |
| `balanceOf` | Returns correct balances for accounts | Unit + Integration |
| `transfer` | Success case, insufficient funds, unauthorized | Unit + Integration |
| | Zero amount, self-transfer, exact balance | Unit test |
| | Event emission | Unit test |
| `mint` | Owner can mint, non-owner cannot | Unit test |
| | Updates total supply | Unit test |
| | Emits Transfer event | Unit test |
| `burn` | User can burn own tokens | Unit test |
| | Updates total supply | Unit test |
| | Emits Transfer event | Unit test |
| `updateOwner` | Owner can change, non-owner cannot | Unit test |
| Entry Points | `deploying` initializes contract | Integration |
| | `invoke` routes to proper methods | Integration |

## Conclusion

Testing is a critical part of Neo contract development. By following the documentation-first approach to testing, you can ensure that your implementation behavior matches the documented contract annotation structure.

As the framework evolves to support full annotation functionality, your tests should continue to validate that the behavior remains consistent with the expected annotation-driven functionality.

For more information on the annotation roadmap, see [Annotation Roadmap](ANNOTATION-ROADMAP.md). 