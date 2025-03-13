# Testing Neo N3 Smart Contracts

This document provides guidance on testing smart contracts developed for the Neo N3 blockchain using the Rust framework.

## Overview

Testing is a critical aspect of smart contract development. Due to the immutable nature of blockchain deployments, it's essential to thoroughly test contracts before deployment to ensure they work as expected and are free from vulnerabilities.

## Types of Tests

### 1. Unit Tests

Unit tests focus on testing individual functions or components of your contract in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialize() {
        // Create a new instance of the contract
        let mut contract = MyContract::new();
        
        // Call initialize with test values
        contract.initialize(H160::from_str("0x1234567890123456789012345678901234567890").unwrap());
        
        // Verify the state after initialization
        assert_eq!(contract.owner.get(), H160::from_str("0x1234567890123456789012345678901234567890").unwrap());
    }
    
    #[test]
    fn test_transfer() {
        let mut contract = MyContract::new();
        
        // Setup initial state
        let owner = H160::from_str("0x1234567890123456789012345678901234567890").unwrap();
        let recipient = H160::from_str("0x0987654321098765432109876543210987654321").unwrap();
        contract.initialize(owner);
        contract.mint(owner, 1000);
        
        // Mock check_witness to return true for the owner
        // This requires a testing framework that supports mocking
        mock_runtime_check_witness(owner, true);
        
        // Perform the transfer
        let result = contract.transfer(owner, recipient, 500);
        
        // Verify the result and new state
        assert!(result);
        assert_eq!(contract.balance_of(owner), 500);
        assert_eq!(contract.balance_of(recipient), 500);
    }
}
```

### 2. Integration Tests

Integration tests focus on testing the interaction between different parts of your contract or between multiple contracts.

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_token_swap() {
        // Create instances of both contracts
        let mut token_a = TokenContract::new();
        let mut token_b = TokenContract::new();
        let mut swap_contract = SwapContract::new();
        
        // Initialize contracts
        let owner = H160::from_str("0x1234567890123456789012345678901234567890").unwrap();
        token_a.initialize(owner, ByteString::from("TokenA"), ByteString::from("TA"), 8, 1000000);
        token_b.initialize(owner, ByteString::from("TokenB"), ByteString::from("TB"), 8, 1000000);
        
        // Configure swap contract with token addresses
        let token_a_address = H160::from_str("0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1").unwrap();
        let token_b_address = H160::from_str("0xb2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2").unwrap();
        swap_contract.initialize(owner, token_a_address, token_b_address);
        
        // Setup test user
        let user = H160::from_str("0xc3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3").unwrap();
        token_a.transfer(owner, user, 10000);
        
        // Setup mock for contract calls
        mock_contract_call(token_a_address, "transfer", true);
        mock_contract_call(token_b_address, "transfer", true);
        
        // Mock check_witness
        mock_runtime_check_witness(user, true);
        
        // Perform swap
        let result = swap_contract.swap(user, 1000, 2000);
        
        // Verify results
        assert!(result);
        // Additional verification of contract state
        // ...
    }
}
```

### 3. Scenario Tests

Scenario tests simulate real-world usage patterns and complex interactions.

```rust
#[cfg(test)]
mod scenario_tests {
    use super::*;
    
    #[test]
    fn test_defi_workflow() {
        // Setup contracts
        let mut token = TokenContract::new();
        let mut lending = LendingContract::new();
        let mut oracle = OracleContract::new();
        
        // Initialize with test values
        // ...
        
        // Scenario: User deposits tokens, takes a loan, and then repays
        
        // Step 1: User deposits tokens
        let result_deposit = lending.deposit(user, amount);
        assert!(result_deposit);
        
        // Step 2: Oracle updates price
        oracle.update_price(token_address, new_price);
        
        // Step 3: User takes a loan
        let result_borrow = lending.borrow(user, loan_amount);
        assert!(result_borrow);
        
        // Step 4: Time passes (simulate with test helper)
        advance_time(30 * 24 * 60 * 60); // 30 days
        
        // Step 5: User repays the loan
        let result_repay = lending.repay(user, repay_amount);
        assert!(result_repay);
        
        // Verify final state
        // ...
    }
}
```

## Setting Up a Testing Environment

### Mock Framework

Create a mock framework to simulate the Neo N3 runtime environment:

```rust
// test_utils.rs
use std::collections::HashMap;
use std::sync::Mutex;
use neo_contract::prelude::*;

// Global storage for mocks
lazy_static! {
    static ref WITNESS_MOCKS: Mutex<HashMap<H160, bool>> = Mutex::new(HashMap::new());
    static ref CONTRACT_CALL_MOCKS: Mutex<HashMap<(H160, String), Vec<u8>>> = Mutex::new(HashMap::new());
    static ref STORAGE_MOCKS: Mutex<HashMap<Vec<u8>, Vec<u8>>> = Mutex::new(HashMap::new());
    static ref EVENT_LOGS: Mutex<Vec<(ByteString, Vec<Any>)>> = Mutex::new(Vec::new());
}

// Mock Runtime functions
pub fn mock_runtime_check_witness(address: H160, result: bool) {
    WITNESS_MOCKS.lock().unwrap().insert(address, result);
}

pub fn mock_contract_call(address: H160, method: &str, result: bool) {
    let serialized = if result {
        // Serialize a "true" result
        [1u8].to_vec()
    } else {
        // Serialize a "false" result
        [0u8].to_vec()
    };
    
    CONTRACT_CALL_MOCKS.lock().unwrap().insert((address, method.to_string()), serialized);
}

pub fn mock_storage_put(key: &[u8], value: &[u8]) {
    STORAGE_MOCKS.lock().unwrap().insert(key.to_vec(), value.to_vec());
}

pub fn mock_storage_get(key: &[u8]) -> Option<Vec<u8>> {
    STORAGE_MOCKS.lock().unwrap().get(&key.to_vec()).cloned()
}

pub fn get_emitted_events() -> Vec<(ByteString, Vec<Any>)> {
    EVENT_LOGS.lock().unwrap().clone()
}

pub fn clear_mocks() {
    WITNESS_MOCKS.lock().unwrap().clear();
    CONTRACT_CALL_MOCKS.lock().unwrap().clear();
    STORAGE_MOCKS.lock().unwrap().clear();
    EVENT_LOGS.lock().unwrap().clear();
}

// Override Neo runtime functions for testing
#[cfg(test)]
mod runtime_overrides {
    use super::*;
    
    // Override check_witness
    pub fn check_witness(address: &H160) -> bool {
        WITNESS_MOCKS.lock().unwrap().get(address).cloned().unwrap_or(false)
    }
    
    // Override contract_call
    pub fn contract_call(hash: &[u8], method: &[u8], args: &[Any], call_flags: u32) -> Vec<u8> {
        let address = H160::from_slice(hash);
        let method_str = String::from_utf8(method.to_vec()).unwrap_or_default();
        
        CONTRACT_CALL_MOCKS.lock().unwrap()
            .get(&(address, method_str))
            .cloned()
            .unwrap_or_default()
    }
    
    // Override notify
    pub fn notify(event_name: &ByteString, data: &[Any]) {
        EVENT_LOGS.lock().unwrap().push((event_name.clone(), data.to_vec()));
    }
}
```

### Test Helpers

Create helpers to simplify common testing tasks:

```rust
// test_helpers.rs
use neo_contract::prelude::*;

// Helper to create a new contract instance with initialized storage
pub fn create_contract<T: Default>(storage_values: &[(&[u8], &[u8])]) -> T {
    let mut contract = T::default();
    
    // Initialize mock storage
    for (key, value) in storage_values {
        mock_storage_put(key, value);
    }
    
    contract
}

// Helper to advance block time for testing time-dependent features
pub fn advance_time(seconds: u64) {
    // Update the mock time
    // Implementation depends on how time is mocked in your test framework
}

// Helper to create a valid H160 address
pub fn create_address(seed: u8) -> H160 {
    let mut bytes = [0u8; 20];
    bytes.iter_mut().for_each(|b| *b = seed);
    H160::from(bytes)
}

// Helper to create test tokens
pub fn setup_token(owner: H160, total_supply: u64) -> TokenContract {
    let mut token = TokenContract::new();
    token.initialize(
        owner,
        ByteString::from("TestToken"),
        ByteString::from("TT"),
        8,
        total_supply
    );
    token
}
```

## Testing Neo N3 Specific Features

### 1. Testing Safe Methods

Verify that safe methods don't modify state:

```rust
#[test]
fn test_safe_methods_do_not_modify_state() {
    let mut contract = TokenContract::new();
    
    // Initialize contract
    let owner = create_address(1);
    contract.initialize(owner, ByteString::from("Token"), ByteString::from("TKN"), 8, 1000000);
    
    // Save initial state
    let initial_total_supply = contract.total_supply();
    let initial_owner_balance = contract.balance_of(owner);
    
    // Call multiple safe methods
    let symbol = contract.symbol();
    let decimals = contract.decimals();
    let total = contract.total_supply();
    let balance = contract.balance_of(owner);
    
    // Verify state hasn't changed
    assert_eq!(contract.total_supply(), initial_total_supply);
    assert_eq!(contract.balance_of(owner), initial_owner_balance);
}
```

### 2. Testing Event Emissions

Verify that events are emitted correctly:

```rust
#[test]
fn test_transfer_emits_event() {
    let mut contract = TokenContract::new();
    
    // Initialize contract
    let owner = create_address(1);
    let recipient = create_address(2);
    contract.initialize(owner, ByteString::from("Token"), ByteString::from("TKN"), 8, 1000000);
    
    // Clear event logs before the operation
    clear_mocks();
    
    // Mock check_witness to return true for the owner
    mock_runtime_check_witness(owner, true);
    
    // Perform transfer
    let result = contract.transfer(owner, recipient, 100);
    assert!(result);
    
    // Get emitted events
    let events = get_emitted_events();
    
    // Verify Transfer event was emitted with correct parameters
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].0, ByteString::from("Transfer"));
    assert_eq!(events[0].1.len(), 3);
    
    // Verify event parameters (from, to, amount)
    let from_param = events[0].1[0].clone();
    let to_param = events[0].1[1].clone();
    let amount_param = events[0].1[2].clone();
    
    // Convert Any type to specific types for assertions
    let from_address: H160 = from_param.into();
    let to_address: H160 = to_param.into();
    let amount: u64 = amount_param.into();
    
    assert_eq!(from_address, owner);
    assert_eq!(to_address, recipient);
    assert_eq!(amount, 100);
}
```

### 3. Testing Contract Calls

Test contract interactions:

```rust
#[test]
fn test_contract_interaction() {
    let mut dex = DexContract::new();
    
    // Setup
    let owner = create_address(1);
    let user = create_address(2);
    let token_a = create_address(3);
    let token_b = create_address(4);
    
    dex.initialize(owner, token_a, token_b);
    
    // Mock contract calls
    mock_contract_call(token_a, "transfer", true);
    mock_contract_call(token_b, "transfer", true);
    mock_runtime_check_witness(user, true);
    
    // Execute swap
    let result = dex.swap(user, token_a, token_b, 100);
    
    // Verify result
    assert!(result);
    
    // Additional verification can be done by inspecting what contract calls were made
    // This would require enhancing the mock framework to record calls
}
```

### 4. Testing Storage Operations

Test storage operations:

```rust
#[test]
fn test_storage_operations() {
    let mut contract = StorageTestContract::new();
    
    // Test putting a value
    let key = b"test_key";
    let value = b"test_value";
    contract.put(key, value);
    
    // Test getting the value
    let retrieved = contract.get(key);
    assert_eq!(retrieved, value);
    
    // Test deleting the value
    contract.delete(key);
    let after_delete = contract.get(key);
    assert!(after_delete.is_empty());
}
```

## Testing for Security Vulnerabilities

### Reentrancy Testing

Test that contracts are protected against reentrancy:

```rust
#[test]
fn test_reentrancy_protection() {
    let mut vulnerable_contract = VulnerableContract::new();
    let mut attack_contract = AttackContract::new();
    
    // Setup
    let owner = create_address(1);
    let attacker = create_address(2);
    
    vulnerable_contract.initialize(owner);
    attack_contract.initialize(attacker);
    
    // Fund the vulnerable contract
    vulnerable_contract.deposit(owner, 1000);
    
    // Setup the attack
    attack_contract.set_target(vulnerable_contract_address());
    
    // Mock check_witness
    mock_runtime_check_witness(attacker, true);
    
    // Attempt the attack
    let initial_balance = vulnerable_contract.balance_of(attacker);
    let result = attack_contract.execute_attack();
    let final_balance = vulnerable_contract.balance_of(attacker);
    
    // In a secure contract, the attack should fail or only withdraw once
    assert!(final_balance <= initial_balance + 100);
}
```

### Access Control Testing

Test that access controls are properly enforced:

```rust
#[test]
fn test_access_control() {
    let mut contract = AdminContract::new();
    
    // Setup
    let admin = create_address(1);
    let user = create_address(2);
    contract.initialize(admin);
    
    // Test admin function with admin
    mock_runtime_check_witness(admin, true);
    let admin_result = contract.admin_function();
    assert!(admin_result);
    
    // Test admin function with non-admin
    mock_runtime_check_witness(admin, false);
    mock_runtime_check_witness(user, true);
    let user_result = contract.admin_function();
    assert!(!user_result);
}
```

## Test Coverage

Aim for high test coverage to ensure all contract code paths are tested:

1. **Function Coverage**: Test all functions in your contract
2. **Branch Coverage**: Test all conditional branches
3. **Edge Cases**: Test boundary conditions and extreme values
4. **Error Paths**: Test error handling and exception scenarios

## Continuous Integration

Set up a CI pipeline to run tests automatically:

```yaml
# .github/workflows/test.yml
name: Run Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Run tests
        run: cargo test --verbose
```

## Testing in a Local Neo N3 Environment

For end-to-end testing, consider setting up a local Neo N3 private network:

1. Install Neo N3 CLI
2. Configure a private network
3. Deploy contracts to the private network
4. Interact with contracts via RPC or SDK
5. Verify contract behavior in a realistic environment

## Best Practices

1. **Start with unit tests**: Begin with comprehensive unit tests before moving to integration tests
2. **Test negative scenarios**: Don't just test the happy path, also test failure conditions
3. **Test all modifiers**: Ensure access control and other modifiers work correctly
4. **Mock external dependencies**: Use the mock framework to simulate external contracts
5. **Reset state between tests**: Clear mocks and state between test cases
6. **Test gas consumption**: For critical functions, consider testing gas usage optimization
7. **Security focused testing**: Prioritize testing for security vulnerabilities
8. **Keep tests up to date**: Update tests when contract functionality changes

## Conclusion

Thorough testing is essential for developing reliable and secure Neo N3 smart contracts. By implementing a comprehensive testing strategy that includes unit tests, integration tests, and scenario tests, you can identify and fix issues before deploying contracts to mainnet.

Remember that testing is an ongoing process, and it's important to continue testing as your contracts evolve and as new vulnerabilities and best practices emerge in the Neo ecosystem.
