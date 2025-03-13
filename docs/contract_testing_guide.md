# Neo N3 Smart Contract Testing Guide

This guide covers approaches, tools, and best practices for testing Neo N3 smart contracts written using the Neo Contract Rust framework.

## Introduction

Testing is a critical component of smart contract development. Due to the immutable nature of blockchains and the financial value often managed by contracts, thorough testing before deployment is essential to prevent costly bugs and security vulnerabilities.

## Testing Approaches

### 1. Unit Testing

Unit tests focus on testing individual components or functions in isolation.

**Benefits:**
- Fast execution
- Precise identification of issues
- Early detection of bugs
- Clear documentation of expected behavior

**Example unit test structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transfer() {
        // Setup contract
        let mut token = TokenContract::new("TestToken", "TT", 8, 1000000);
        
        // Execute operation
        let result = token.transfer(Address::from([1; 20]), Address::from([2; 20]), 100);
        
        // Verify results
        assert!(result);
        assert_eq!(token.balance_of(Address::from([1; 20])), 999900);
        assert_eq!(token.balance_of(Address::from([2; 20])), 100);
    }
    
    #[test]
    fn test_transfer_insufficient_balance() {
        let mut token = TokenContract::new("TestToken", "TT", 8, 1000);
        
        // Attempt to transfer more than available
        let result = token.transfer(Address::from([1; 20]), Address::from([2; 20]), 2000);
        
        // Verify failure
        assert!(!result);
        assert_eq!(token.balance_of(Address::from([1; 20])), 1000);
        assert_eq!(token.balance_of(Address::from([2; 20])), 0);
    }
}
```

### 2. Integration Testing

Integration tests verify that different components of your contract work together correctly.

**Benefits:**
- Tests interactions between components
- Validates workflow sequences
- Ensures components integrate correctly

**Example integration test:**

```rust
#[test]
fn test_full_token_workflow() {
    // Setup contract
    let mut token = TokenContract::new("TestToken", "TT", 8, 10000);
    
    // Test approval and transfer
    let owner = Address::from([1; 20]);
    let spender = Address::from([2; 20]);
    let recipient = Address::from([3; 20]);
    
    // Approve
    assert!(token.approve(owner, spender, 500));
    assert_eq!(token.allowance(owner, spender), 500);
    
    // Transfer using allowance
    assert!(token.transfer_from(spender, owner, recipient, 300));
    
    // Verify final state
    assert_eq!(token.allowance(owner, spender), 200); // Reduced allowance
    assert_eq!(token.balance_of(owner), 9700);
    assert_eq!(token.balance_of(recipient), 300);
}
```

### 3. Scenario Testing

Scenario tests simulate real-world usage patterns and complex interactions.

**Benefits:**
- Tests contract behavior under realistic conditions
- Validates expected outcomes in complex scenarios
- Identifies edge cases and boundary conditions

**Example scenario test:**

```rust
#[test]
fn test_dex_liquidity_scenario() {
    // Setup contracts
    let mut token_a = TokenContract::new("TokenA", "TA", 8, 1000000);
    let mut token_b = TokenContract::new("TokenB", "TB", 8, 1000000);
    let mut dex = DexContract::new();
    
    let alice = Address::from([1; 20]);
    let bob = Address::from([2; 20]);
    
    // Scenario 1: Alice adds liquidity
    token_a.transfer(Address::zero(), alice, 10000);
    token_b.transfer(Address::zero(), alice, 20000);
    
    token_a.approve(alice, dex.address(), 5000);
    token_b.approve(alice, dex.address(), 10000);
    
    let lp_tokens = dex.add_liquidity(alice, token_a.address(), token_b.address(), 5000, 10000, 4900, 9900);
    assert!(lp_tokens > 0);
    
    // Scenario 2: Bob swaps tokens
    token_a.transfer(Address::zero(), bob, 1000);
    token_a.approve(bob, dex.address(), 500);
    
    let tokens_out = dex.swap(bob, token_a.address(), token_b.address(), 500, 900);
    assert!(tokens_out > 900);
    
    // Verify final balances reflect expected outcome
    // ...
}
```

### 4. Property-Based Testing

Property-based tests verify that certain properties or invariants of your contract always hold true, regardless of the specific inputs.

**Benefits:**
- Discovers edge cases automatically
- Increases test coverage
- Finds unexpected vulnerabilities

**Example property test (using a hypothetical proptest framework):**

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn balance_sum_constant(transfers in vec![(0..100u64, 0..100u64), 1..10]) {
            let mut token = TokenContract::new("Token", "TKN", 8, 1000);
            let initial_supply = token.total_supply();
            
            // Execute random transfers
            for (amount, recipient_id) in transfers {
                let recipient = Address::from([recipient_id as u8; 20]);
                let _ = token.transfer(Address::zero(), recipient, amount);
            }
            
            // Property: total supply remains constant
            prop_assert_eq!(token.total_supply(), initial_supply);
            
            // Property: sum of all balances equals total supply
            let sum_of_balances = /* calculate sum */;
            prop_assert_eq!(sum_of_balances, initial_supply);
        }
    }
}
```

## Testing Tools for Neo N3 Contracts

### 1. Rust Standard Testing Framework

The Neo Contract Rust framework supports Rust's built-in testing framework:

```bash
# Run unit tests
cargo test -p your-contract --features std
```

### 2. Mocking the Blockchain Environment

Create mock implementations of blockchain functions to simulate chain behavior:

```rust
#[cfg(test)]
mod mock {
    use super::*;
    
    // Mock Ledger
    pub struct MockLedger {
        current_height: u32,
        current_time: u64,
    }
    
    impl MockLedger {
        pub fn new(height: u32, time: u64) -> Self {
            Self { current_height: height, current_time: time }
        }
        
        pub fn set_height(&mut self, height: u32) {
            self.current_height = height;
        }
        
        pub fn set_time(&mut self, time: u64) {
            self.current_time = time;
        }
        
        pub fn current_index(&self) -> u32 {
            self.current_height
        }
        
        pub fn current_timestamp(&self) -> u64 {
            self.current_time
        }
    }
}

#[test]
fn test_time_dependent_logic() {
    let mut contract = YourContract::new();
    let mut mock_ledger = mock::MockLedger::new(1000, 1625097600); // 2021-07-01
    
    // Test with initial time
    assert!(!contract.is_expired(&mock_ledger));
    
    // Advance time and test again
    mock_ledger.set_time(1640995200); // 2022-01-01
    assert!(contract.is_expired(&mock_ledger));
}
```

### 3. Neo Blockchain Emulator

For integration tests against a simulated blockchain:

```rust
#[test]
fn test_with_chain_emulator() {
    // Initialize emulator
    let mut emulator = neo_emulator::NeoEmulator::new();
    
    // Deploy contract
    let contract_addr = emulator.deploy_contract("your_contract.nef", &[]);
    
    // Create test accounts
    let alice = emulator.create_account(1000);
    
    // Invoke contract
    let result = emulator.invoke_contract(
        contract_addr,
        "transfer",
        &[alice.address.into(), Address::from([2; 20]).into(), 100.into()],
        alice
    );
    
    // Verify result
    assert!(result.success);
    
    // Check contract state
    let balance = emulator.invoke_view(contract_addr, "balanceOf", &[Address::from([2; 20]).into()]);
    assert_eq!(balance.as_int().unwrap(), 100);
}
```

## Testing Best Practices

### 1. Test Coverage

Aim for high test coverage across your contract code:

- Test all public methods
- Cover all branches in conditional logic
- Test boundary conditions and edge cases
- Include negative tests (cases expected to fail)

### 2. Test Independence

Ensure tests are independent of each other:

- Each test should setup its own environment
- Tests should not depend on the order of execution
- Clean up any shared resources between tests

```rust
#[test]
fn test_a() {
    let mut contract = Contract::new(); // Fresh contract for this test
    // Test logic...
}

#[test]
fn test_b() {
    let mut contract = Contract::new(); // Fresh contract for this test
    // Different test logic...
}
```

### 3. Test Readability

Write clear and descriptive tests:

- Use meaningful test names that describe what's being tested
- Structure tests using the Arrange-Act-Assert pattern
- Add comments explaining complex test scenarios

```rust
#[test]
fn transfer_should_fail_when_sender_has_insufficient_balance() {
    // Arrange
    let mut token = TokenContract::new("Test", "TST", 8, 100);
    let sender = Address::from([1; 20]);
    let recipient = Address::from([2; 20]);
    
    // Act
    let result = token.transfer(sender, recipient, 200); // Try to transfer more than available
    
    // Assert
    assert!(!result);
    assert_eq!(token.balance_of(sender), 100); // Balance unchanged
    assert_eq!(token.balance_of(recipient), 0); // No tokens received
}
```

### 4. Testing Time-Dependent Logic

Smart contracts often include time-dependent logic that can be challenging to test:

```rust
#[test]
fn test_vesting_schedule() {
    let mut contract = VestingContract::new();
    let mut mock_ledger = mock::MockLedger::new(1000, 1625097600); // 2021-07-01
    
    // Setup vesting schedule (1 year)
    contract.create_schedule(
        Address::from([1; 20]),
        1000,
        1625097600, // Start: 2021-07-01
        1656633600  // End: 2022-07-01
    );
    
    // Test at start (0% vested)
    assert_eq!(contract.vested_amount(Address::from([1; 20]), &mock_ledger), 0);
    
    // Test at 6 months (50% vested)
    mock_ledger.set_time(1640995200); // 2022-01-01
    assert_eq!(contract.vested_amount(Address::from([1; 20]), &mock_ledger), 500);
    
    // Test at end (100% vested)
    mock_ledger.set_time(1656633600); // 2022-07-01
    assert_eq!(contract.vested_amount(Address::from([1; 20]), &mock_ledger), 1000);
    
    // Test after end (still 100% vested)
    mock_ledger.set_time(1672531200); // 2023-01-01
    assert_eq!(contract.vested_amount(Address::from([1; 20]), &mock_ledger), 1000);
}
```

### 5. Testing Access Control

Thoroughly test access control mechanisms to ensure privileged functions are protected:

```rust
#[test]
fn only_owner_can_set_parameters() {
    let mut contract = GovernanceContract::new(Address::from([1; 20])); // Owner address
    
    // Test with owner
    assert!(contract.set_parameter("fee", 100, Address::from([1; 20])));
    
    // Test with non-owner
    assert!(!contract.set_parameter("fee", 200, Address::from([2; 20])));
    
    // Verify parameter value
    assert_eq!(contract.get_parameter("fee"), 100);
}
```

### 6. Test Emitted Events

Verify that your contract correctly emits events on important state changes:

```rust
#[test]
fn transfer_emits_event() {
    let mut token = TokenContract::new("Test", "TST", 8, 1000);
    let sender = Address::from([1; 20]);
    let recipient = Address::from([2; 20]);
    
    // Setup event recorder (hypothetical)
    let mut event_recorder = mock::EventRecorder::new();
    token.set_event_recorder(&mut event_recorder);
    
    // Execute transfer
    token.transfer(sender, recipient, 100);
    
    // Verify event was emitted
    let events = event_recorder.get_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "Transfer");
    assert_eq!(events[0].params[0], sender);
    assert_eq!(events[0].params[1], recipient);
    assert_eq!(events[0].params[2], 100);
}
```

## Testing for Security

Security testing is critical for smart contracts. Include specific tests to verify:

### 1. Reentrancy Protection

```rust
#[test]
fn test_reentrancy_protection() {
    let mut contract = EscrowContract::new();
    let mut attacker = mock::ReentrantAttacker::new();
    
    // Fund the contract
    contract.deposit(1000);
    
    // Attempt reentrancy attack
    let result = contract.withdraw(500, &mut attacker);
    
    // Verify attack was prevented
    assert_eq!(contract.balance(), 1000); // Balance should be unchanged
    assert_eq!(attacker.balance(), 0);
}
```

### 2. Access Control Testing

```rust
#[test]
fn test_role_based_access_control() {
    let mut contract = GovernanceContract::new();
    
    // Setup roles
    contract.add_admin(Address::from([1; 20]));
    contract.add_operator(Address::from([2; 20]));
    
    // Test admin function
    assert!(contract.pause(Address::from([1; 20]))); // Admin can pause
    assert!(!contract.pause(Address::from([2; 20]))); // Operator cannot pause
    assert!(!contract.pause(Address::from([3; 20]))); // Random user cannot pause
    
    // Test operator function
    assert!(contract.update_price_feed(Address::from([1; 20]))); // Admin can update
    assert!(contract.update_price_feed(Address::from([2; 20]))); // Operator can update
    assert!(!contract.update_price_feed(Address::from([3; 20]))); // Random user cannot update
}
```

### 3. Testing for Integer Overflows

```rust
#[test]
fn test_overflow_protection() {
    let mut token = TokenContract::new("Test", "TST", 8, 1000);
    
    // Attempt to transfer a very large amount
    let result = token.transfer(
        Address::from([1; 20]),
        Address::from([2; 20]),
        u64::MAX
    );
    
    // Verify overflow is handled
    assert!(!result);
    assert_eq!(token.balance_of(Address::from([1; 20])), 1000); // Balance unchanged
}
```

## Continuous Integration

Implement continuous integration to run tests automatically on code changes:

1. Use GitHub Actions or similar CI system
2. Run tests on every pull request and merge to main branch
3. Check test coverage and fail if coverage drops below threshold
4. Include security scanning tools in the CI pipeline

Example GitHub Actions workflow:

```yaml
name: Neo Contract Tests

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
    - name: Run tests
      run: cargo test -p your-contract --features std
    - name: Check test coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin -p your-contract --features std --out Xml
    - name: Security audit
      run: |
        cargo install cargo-audit
        cargo audit
```

## Debugging Tips

### 1. Verbose Testing Output

Enable verbose output for more detailed test information:

```bash
RUST_BACKTRACE=1 cargo test -p your-contract --features std -- --nocapture
```

### 2. Debug Print Statements

Add debug print statements to help troubleshoot test failures:

```rust
#[test]
fn debug_test() {
    let mut contract = Contract::new();
    let result = contract.complex_operation();
    println!("Operation result: {:?}", result);
    println!("Contract state: {:?}", contract);
    assert!(result);
}
```

### 3. Isolate Failing Tests

Run a specific test to isolate failures:

```bash
cargo test -p your-contract --features std -- test_name --exact
```

## Example: Complete Test Suite

Here's an example of a comprehensive test suite for a token contract:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to create a test environment
    fn setup() -> TokenContract {
        TokenContract::new("TestToken", "TT", 8, 1000000)
    }
    
    #[test]
    fn test_constructor() {
        let token = setup();
        assert_eq!(token.name(), "TestToken");
        assert_eq!(token.symbol(), "TT");
        assert_eq!(token.decimals(), 8);
        assert_eq!(token.total_supply(), 1000000);
    }
    
    #[test]
    fn test_transfer() {
        let mut token = setup();
        let sender = Address::from([1; 20]);
        let recipient = Address::from([2; 20]);
        
        // Initial balances
        assert_eq!(token.balance_of(sender), 1000000);
        assert_eq!(token.balance_of(recipient), 0);
        
        // Test successful transfer
        assert!(token.transfer(sender, recipient, 1000));
        assert_eq!(token.balance_of(sender), 999000);
        assert_eq!(token.balance_of(recipient), 1000);
        
        // Test transfer exceeding balance
        assert!(!token.transfer(sender, recipient, 1000000));
        assert_eq!(token.balance_of(sender), 999000); // Unchanged
        assert_eq!(token.balance_of(recipient), 1000); // Unchanged
        
        // Test transfer to zero address
        assert!(!token.transfer(sender, Address::zero(), 1000));
        
        // Test zero amount transfer
        assert!(token.transfer(sender, recipient, 0));
        assert_eq!(token.balance_of(sender), 999000);
        assert_eq!(token.balance_of(recipient), 1000);
    }
    
    #[test]
    fn test_approve_and_transfer_from() {
        let mut token = setup();
        let owner = Address::from([1; 20]);
        let spender = Address::from([2; 20]);
        let recipient = Address::from([3; 20]);
        
        // Setup approval
        assert!(token.approve(owner, spender, 5000));
        assert_eq!(token.allowance(owner, spender), 5000);
        
        // Test successful transfer_from
        assert!(token.transfer_from(spender, owner, recipient, 3000));
        assert_eq!(token.balance_of(owner), 997000);
        assert_eq!(token.balance_of(recipient), 3000);
        assert_eq!(token.allowance(owner, spender), 2000); // Decreased by transfer amount
        
        // Test exceeding allowance
        assert!(!token.transfer_from(spender, owner, recipient, 3000));
        assert_eq!(token.allowance(owner, spender), 2000); // Unchanged
        
        // Test exceeding balance
        token.approve(owner, spender, 1000000);
        assert!(!token.transfer_from(spender, owner, recipient, 999999));
        
        // Test revoking approval
        assert!(token.approve(owner, spender, 0));
        assert_eq!(token.allowance(owner, spender), 0);
    }
    
    // Additional tests for edge cases, events, etc.
}
```

## Conclusion

Thorough testing is essential for developing reliable and secure Neo N3 smart contracts. By combining unit tests, integration tests, and scenario tests with security-focused testing approaches, you can significantly reduce the risk of bugs and vulnerabilities in your deployed contracts.

Remember that testing is not a one-time activity but an ongoing process throughout the development lifecycle. As you add features or modify your contract, ensure your test suite is updated to maintain high test coverage and quality assurance. 