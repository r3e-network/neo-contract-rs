# Testing Neo Smart Contracts

This guide covers strategies and best practices for testing Neo smart contracts developed with the Neo Contract Rust Framework.

## Introduction

Testing smart contracts is critical due to their immutable nature once deployed. Unlike traditional software, you can't easily patch a blockchain application after deployment. A comprehensive testing strategy helps prevent costly bugs and vulnerabilities.

## Testing Levels

### 1. Unit Testing

Unit tests focus on individual components of your contract in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_increment() {
        // Create a new contract instance
        let mut counter = Counter::new();
        
        // Test the initial state
        assert_eq!(counter.get_count(), 0);
        
        // Test the increment method
        counter.increment();
        assert_eq!(counter.get_count(), 1);
        
        // Test multiple increments
        counter.increment();
        counter.increment();
        assert_eq!(counter.get_count(), 3);
    }
}
```

### 2. Integration Testing

Integration tests verify how different parts of your contract work together:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_token_transfers() {
        // Create a token contract
        let mut token = Token::new(
            "TestToken".to_string(),
            "TT".to_string(),
            8,
            1_000_000
        );
        
        // Mock addresses
        let owner = Address::from([1u8; 20]);
        let recipient = Address::from([2u8; 20]);
        
        // Set up test environment
        runtime::set_current_address(owner.clone());
        
        // Test the transfer flow
        assert_eq!(token.balance_of(owner.clone()), 1_000_000);
        assert_eq!(token.balance_of(recipient.clone()), 0);
        
        token.transfer(recipient.clone(), 500_000);
        
        assert_eq!(token.balance_of(owner), 500_000);
        assert_eq!(token.balance_of(recipient), 500_000);
    }
}
```

### 3. Contract Interaction Testing

Test interactions between multiple contracts:

```rust
#[cfg(test)]
mod contract_interaction_tests {
    use super::*;
    
    #[test]
    fn test_contract_calls() {
        // Create token contract
        let mut token = Token::new("TestToken".to_string(), "TT".to_string(), 8, 1_000_000);
        
        // Create marketplace contract with token contract reference
        let mut marketplace = Marketplace::new(token.contract_hash());
        
        // Test interaction between contracts
        // ...
    }
}
```

## Mock Environment

The Neo Contract Rust Framework provides a mock runtime environment for testing:

```rust
// Import mock utilities
use neo_contract::test_utils::{
    setup_mock_runtime,
    MockStorage,
    MockRuntime,
};

#[test]
fn test_with_mock_runtime() {
    // Set up mock environment
    let mut mock_runtime = setup_mock_runtime();
    
    // Mock addresses
    let owner = Address::from([1u8; 20]);
    let spender = Address::from([2u8; 20]);
    
    // Configure mock environment
    mock_runtime.set_current_sender(owner.clone());
    mock_runtime.set_gas_balance(owner.clone(), 1000);
    mock_runtime.set_block_height(100);
    
    // Create contract within mock environment
    let mut token = Token::new("Test".to_string(), "TST".to_string(), 8, 1_000_000);
    
    // Test contract behavior
    assert!(token.transfer(spender.clone(), 100));
    assert_eq!(token.balance_of(spender), 100);
}
```

## Testing Storage Operations

Test how your contract interacts with blockchain storage:

```rust
#[test]
fn test_storage_operations() {
    // Set up mock storage
    let mut mock_storage = MockStorage::new();
    
    // Create contract with mock storage
    let mut counter = Counter::new();
    
    // Test storage operations
    counter.increment();
    counter.increment();
    
    // Verify storage state
    let storage_key = counter.count.key().to_vec();
    let raw_value = mock_storage.get(storage_key);
    let value = u64::deserialize(&raw_value.unwrap()).unwrap();
    
    assert_eq!(value, 2);
}
```

## Event Testing

Verify that your contract emits the expected events:

```rust
#[test]
fn test_events() {
    // Set up mock runtime with event capture
    let mut mock_runtime = setup_mock_runtime();
    mock_runtime.enable_event_capture();
    
    // Mock addresses
    let sender = Address::from([1u8; 20]);
    let receiver = Address::from([2u8; 20]);
    
    // Configure runtime
    mock_runtime.set_current_sender(sender.clone());
    
    // Create and execute contract
    let mut token = Token::new("Test".to_string(), "TST".to_string(), 8, 1000);
    token.transfer(receiver.clone(), 100);
    
    // Capture and verify events
    let events = mock_runtime.get_captured_events();
    
    assert_eq!(events.len(), 2); // Mint event + Transfer event
    
    let transfer_event = &events[1];
    assert_eq!(transfer_event.name, "Transfer");
    
    // Decode event parameters
    let from = Address::deserialize(&transfer_event.params[0]).unwrap();
    let to = Address::deserialize(&transfer_event.params[1]).unwrap();
    let amount = u64::deserialize(&transfer_event.params[2]).unwrap();
    
    assert_eq!(from, sender);
    assert_eq!(to, receiver);
    assert_eq!(amount, 100);
}
```

## Test Organization

Organize your tests for maintainability:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Shared test utilities
    fn setup() -> (Address, Address, Token) {
        let owner = Address::from([1u8; 20]);
        let user = Address::from([2u8; 20]);
        
        runtime::set_current_sender(owner.clone());
        
        let token = Token::new("Test".to_string(), "TST".to_string(), 8, 1000);
        
        (owner, user, token)
    }
    
    mod basic_functionality {
        use super::*;
        
        #[test]
        fn test_metadata() {
            let (_, _, token) = setup();
            
            assert_eq!(token.name(), "Test");
            assert_eq!(token.symbol(), "TST");
            assert_eq!(token.decimals(), 8);
        }
        
        #[test]
        fn test_initial_supply() {
            let (owner, _, token) = setup();
            
            assert_eq!(token.total_supply(), 1000);
            assert_eq!(token.balance_of(owner), 1000);
        }
    }
    
    mod transfer_tests {
        use super::*;
        
        #[test]
        fn test_successful_transfer() {
            let (owner, user, mut token) = setup();
            
            assert!(token.transfer(user.clone(), 500));
            
            assert_eq!(token.balance_of(owner), 500);
            assert_eq!(token.balance_of(user), 500);
        }
        
        #[test]
        fn test_insufficient_balance() {
            let (_, user, mut token) = setup();
            
            assert!(!token.transfer(user, 1001));
        }
    }
}
```

## Property-Based Testing

For complex contracts, consider property-based testing:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_transfer_preserves_total_supply(amount in 0..1000u64) {
            let (owner, user, mut token) = setup();
            let initial_supply = token.total_supply();
            
            if amount <= token.balance_of(owner) {
                token.transfer(user, amount);
            }
            
            // Property: Total supply should remain constant
            assert_eq!(token.total_supply(), initial_supply);
            
            // Property: Sum of all balances equals total supply
            assert_eq!(
                token.balance_of(owner) + token.balance_of(user),
                initial_supply
            );
        }
    }
}
```

## Security Testing

Test security properties of your contract:

```rust
#[test]
fn test_reentrancy_protection() {
    // Set up a mock attacker contract
    let mut attacker = ReentrancyAttacker::new();
    
    // Set up the target contract
    let mut vault = Vault::new();
    runtime::set_current_sender(owner.clone());
    
    // Fund the vault
    vault.deposit(1000);
    
    // Attempt reentrancy attack
    let result = vault.withdraw(500, attacker.address());
    
    // Verify attack was prevented
    assert!(result.is_err());
    assert_eq!(vault.balance_of(owner), 1000);
}
```

## Gas Optimization Testing

Test gas usage of your contract methods:

```rust
#[test]
fn test_gas_usage() {
    // Set up mock runtime with gas tracking
    let mut mock_runtime = setup_mock_runtime();
    mock_runtime.enable_gas_tracking();
    
    // Create contract
    let mut token = Token::new("Test".to_string(), "TST".to_string(), 8, 1000);
    
    // Reset gas counter
    mock_runtime.reset_gas_counter();
    
    // Execute method
    token.transfer(receiver.clone(), 100);
    
    // Get gas used
    let gas_used = mock_runtime.get_gas_used();
    
    // Ensure gas usage is within acceptable limits
    assert!(gas_used < 100);
}
```

## Testing with Neo Express

For end-to-end testing, use Neo Express:

```bash
# Create a private blockchain
neoxp create

# Start the chain
neoxp run

# Deploy contract
neoxp contract deploy ./path/to/contract.nef ./path/to/contract.manifest.json

# Invoke contract methods for testing
neoxp contract invoke <contract-hash> <method-name> [<parameters>]

# Verify results
neoxp contract get <contract-hash>
```

## Continuous Integration

Integrate testing into your CI/CD pipeline:

```yaml
# .github/workflows/test.yml
name: Test Neo Contract

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          target: wasm32-unknown-unknown
      
      - name: Build
        run: cargo build
      
      - name: Run tests
        run: cargo test
      
      - name: Build WASM
        run: cargo build --target wasm32-unknown-unknown --release
      
      - name: Setup Neo Express
        run: |
          # Install Neo Express
          # ...
      
      - name: E2E Tests
        run: |
          # Run end-to-end tests with Neo Express
          # ...
```

## Best Practices

1. **Test Coverage**: Aim for high test coverage of your contract code
2. **Edge Cases**: Test boundary conditions and edge cases
3. **Negative Testing**: Verify that your contract correctly handles invalid inputs
4. **Regression Tests**: Add tests for any bugs found
5. **Modular Tests**: Keep tests small, focused, and independent
6. **Security-First**: Test for common smart contract vulnerabilities
7. **Gas Optimization**: Monitor gas costs of your contract methods

## Testing Checklist

- [ ] Basic functionality works as expected
- [ ] Contract handles edge cases correctly
- [ ] Invalid inputs are rejected
- [ ] Contract emits expected events
- [ ] Storage operations work correctly
- [ ] Contract is secure against common attacks
- [ ] Gas usage is optimized
- [ ] Contract interactions work correctly
- [ ] Upgrades and migrations work if applicable

## Conclusion

Thorough testing is essential for blockchain development. By incorporating these testing strategies, you can have greater confidence in your Neo smart contracts' reliability, security, and efficiency.

Remember that no amount of testing can guarantee the absence of all bugs, but a comprehensive testing strategy significantly reduces the risk of vulnerabilities and malfunctions in your deployed contracts.