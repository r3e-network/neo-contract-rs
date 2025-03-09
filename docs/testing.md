# Testing Neo Smart Contracts

This guide covers comprehensive testing approaches for Neo smart contracts written with the Neo Contract Rust Framework, including unit testing, integration testing, and simulated blockchain testing.

## Testing Approaches Overview

Testing smart contracts requires several approaches:

1. **Unit Testing**: Testing individual components of your contract logic
2. **Integration Testing**: Testing the contract as deployed on a blockchain
3. **Simulation Testing**: Testing with a simulated Neo blockchain environment
4. **Property-Based Testing**: Testing with randomized inputs to find edge cases
5. **Fuzz Testing**: Testing with automatically generated random inputs

Each approach addresses different aspects of contract correctness and security.

## Unit Testing with Rust

The Neo Contract Rust Framework allows you to write standard Rust unit tests for your contract logic.

### Basic Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_transfer() {
        // Initialize the contract
        let mut contract = token::TokenContract::new(
            Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap(),
            1_000_000
        );
        
        // Test parameters
        let from = Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap();
        let to = Address::from_str("NVRe7PCm1c6MkUwTVJWEp7KBm9BFhgnjkP").unwrap();
        let amount = 1000;
        
        // Override check_witness to always return true for testing
        // This requires mocking support or conditional compilation
        
        // Perform the transfer
        let result = contract.transfer(from, to, amount);
        
        // Assert the result is true
        assert!(result);
        
        // Check balances were updated correctly
        assert_eq!(contract.balance_of(from), 999_000);
        assert_eq!(contract.balance_of(to), 1000);
    }
}
```

### Test Fixtures and Helpers

Create test fixtures to reuse contract setups across tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test fixture for a token contract
    fn setup_token_contract() -> token::TokenContract {
        let owner = Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap();
        token::TokenContract::new(owner, 1_000_000)
    }
    
    #[test]
    fn test_initial_supply() {
        let contract = setup_token_contract();
        assert_eq!(contract.total_supply(), 1_000_000);
    }
    
    #[test]
    fn test_owner_balance() {
        let contract = setup_token_contract();
        let owner = Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap();
        assert_eq!(contract.balance_of(owner), 1_000_000);
    }
}
```

### Mocking Blockchain Functions

To test contracts that interact with the blockchain environment, you'll need to mock runtime functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::mock::{MockRuntime, MockStorage};
    
    #[test]
    fn test_with_mock_runtime() {
        // Create a mock runtime
        let mut runtime = MockRuntime::new();
        
        // Configure mock behavior
        runtime.set_check_witness_result(true);
        runtime.set_current_timestamp(1647356400); // March 15, 2022
        
        // Create contract with the mock runtime
        let mut contract = MyContract::new_with_runtime(&runtime);
        
        // Execute contract method that uses runtime
        let result = contract.time_sensitive_operation();
        
        // Assert expected behavior
        assert!(result);
    }
}
```

### Testing Storage Operations

Storage operations can be tested with a mock storage implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::mock::MockStorage;
    
    #[test]
    fn test_storage_operations() {
        // Create a mock storage
        let storage = MockStorage::new();
        
        // Create contract with mock storage
        let mut contract = MyContract::new_with_storage(&storage);
        
        // Execute storage operations
        contract.store_value("key1", "value1");
        
        // Verify storage state
        assert_eq!(contract.get_value("key1"), "value1");
        
        // Verify the mock storage directly
        assert_eq!(storage.get("prefix:key1"), "value1");
    }
}
```

## Integration Testing with Neo Contract Testing Framework

The `neo-contract-testing` crate provides tools for integration testing with a simulated Neo blockchain.

### Setting Up Integration Tests

```rust
use neo_contract_testing::{
    fixture::ContractFixture,
    simulator::BlockchainSimulator,
    assertions::assert_events,
};

#[test]
fn test_token_contract_integration() {
    // Create a blockchain simulator
    let mut simulator = BlockchainSimulator::new();
    
    // Deploy the contract
    let contract = simulator.deploy_contract("./target/wasm32-unknown-unknown/release/token.wasm");
    
    // Create test accounts
    let owner = simulator.create_account_with_balance(1000);
    let user = simulator.create_account_with_balance(0);
    
    // Initialize the contract
    let result = simulator.invoke(
        contract.address(),
        "constructor",
        &[owner.address(), 1_000_000]
    );
    
    // Assert successful execution
    assert!(result.success);
    
    // Transfer tokens
    let transfer_result = simulator.invoke_as(
        owner,
        contract.address(),
        "transfer",
        &[owner.address(), user.address(), 500]
    );
    
    // Assert successful transfer
    assert!(transfer_result.success);
    
    // Check events were emitted correctly
    assert_events!(
        transfer_result.events,
        {
            name: "Transfer",
            params: [
                owner.address(),
                user.address(),
                500
            ]
        }
    );
    
    // Check balances
    let owner_balance = simulator.invoke(
        contract.address(),
        "balance_of",
        &[owner.address()]
    ).result_as::<u64>();
    
    let user_balance = simulator.invoke(
        contract.address(),
        "balance_of",
        &[user.address()]
    ).result_as::<u64>();
    
    assert_eq!(owner_balance, 999_500);
    assert_eq!(user_balance, 500);
}
```

### Testing Multiple Contracts Interaction

Test contracts that interact with each other:

```rust
#[test]
fn test_contract_interaction() {
    let mut simulator = BlockchainSimulator::new();
    
    // Deploy both contracts
    let token_contract = simulator.deploy_contract("./target/wasm32-unknown-unknown/release/token.wasm");
    let marketplace_contract = simulator.deploy_contract("./target/wasm32-unknown-unknown/release/marketplace.wasm");
    
    // Setup contracts
    let owner = simulator.create_account_with_balance(1000);
    
    // Initialize token contract
    simulator.invoke_as(
        owner,
        token_contract.address(),
        "constructor",
        &[owner.address(), 1_000_000]
    );
    
    // Initialize marketplace contract
    simulator.invoke_as(
        owner,
        marketplace_contract.address(),
        "constructor",
        &[token_contract.address()]
    );
    
    // Test interaction between contracts
    // E.g., approve tokens from token contract to marketplace
    simulator.invoke_as(
        owner,
        token_contract.address(),
        "approve",
        &[marketplace_contract.address(), 10000]
    );
    
    // Test marketplace spending tokens from owner via token contract
    let result = simulator.invoke_as(
        owner,
        marketplace_contract.address(),
        "create_item",
        &["Item1", 1000]
    );
    
    assert!(result.success);
    
    // Verify token transfer occurred
    let marketplace_balance = simulator.invoke(
        token_contract.address(),
        "balance_of",
        &[marketplace_contract.address()]
    ).result_as::<u64>();
    
    assert_eq!(marketplace_balance, 1000);
}
```

## Testing on Private Networks

For end-to-end testing with a real Neo blockchain:

### Setting Up a Private Network

1. Use Neo Express to create a private network:
```bash
neoxp create -c 1
```

2. Generate test wallets:
```bash
neoxp wallet create owner
neoxp wallet create user1
```

3. Transfer GAS to test wallets:
```bash
neoxp transfer gas 100 genesis owner
neoxp transfer gas 100 genesis user1
```

### Deploying and Testing Contracts

1. Deploy the contract:
```bash
neoxp contract deploy ./path/to/contract.nef owner
```

2. Test contract invocation:
```bash
neoxp contract invoke <contract-hash> transfer '[{"type":"Hash160","value":"<owner-hash>"},{"type":"Hash160","value":"<user1-hash>"},{"type":"Integer","value":"1000"}]' --account owner
```

3. Check contract state:
```bash
neoxp contract invoke <contract-hash> balanceOf '[{"type":"Hash160","value":"<user1-hash>"}]'
```

## Property-Based Testing

Property-based testing generates random inputs to find edge cases:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn transfer_properties(
        initial_supply in 1000..1_000_000_u64,
        transfer_amount in 1..500_u64
    ) {
        // Create contract with random initial supply
        let mut contract = token::TokenContract::new(
            Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap(),
            initial_supply
        );
        
        let owner = Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap();
        let recipient = Address::from_str("NVRe7PCm1c6MkUwTVJWEp7KBm9BFhgnjkP").unwrap();
        
        // Property 1: Transfer should succeed if balance is sufficient
        if initial_supply >= transfer_amount {
            let result = contract.transfer(owner, recipient, transfer_amount);
            prop_assert!(result);
            
            // Property 2: Balance of recipient should increase by transfer amount
            prop_assert_eq!(contract.balance_of(recipient), transfer_amount);
            
            // Property 3: Balance of sender should decrease by transfer amount
            prop_assert_eq!(contract.balance_of(owner), initial_supply - transfer_amount);
        } else {
            // Property 4: Transfer should fail if balance is insufficient
            let result = contract.transfer(owner, recipient, transfer_amount);
            prop_assert!(!result);
        }
    }
}
```

## Continuous Integration Testing

Set up CI to automatically test your contracts on each commit:

### GitHub Actions Example

```yaml
name: Contract Tests

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
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Build contracts
        run: |
          cargo build --release --target wasm32-unknown-unknown
          
      - name: Run unit tests
        run: cargo test
        
      - name: Install neo-cli
        run: |
          wget https://github.com/neo-project/neo-cli/releases/download/v3.1.0/neo-cli-linux-x64.zip
          unzip neo-cli-linux-x64.zip -d neo-cli
          
      - name: Create private network
        run: |
          cd neo-cli
          ./neo-cli create-private-chain
          
      - name: Deploy and test contracts
        run: |
          cd neo-cli
          ./neo-cli deploy ../target/wasm32-unknown-unknown/release/token.nef ../token.manifest.json
          ./neo-cli invoke <contract-hash> balanceOf '[{"type":"Hash160","value":"<genesis-hash>"}]'
```

## Best Testing Practices

1. **Test Coverage**: Aim for high test coverage of your contract code
2. **Failure Testing**: Explicitly test error cases and edge conditions
3. **Economic Attacks**: Test potential economic exploit scenarios
4. **Gas Estimation**: Test gas consumption for all methods
5. **State Transitions**: Verify contract state changes work as expected
6. **Events**: Confirm events are emitted correctly with proper parameters
7. **Permission Controls**: Test all permission and access control logic
8. **Interoperability**: Test interactions with other contracts

## Specialized Testing Tools

### Gas Profiling

```rust
use neo_contract_testing::profiling::GasProfiler;

#[test]
fn test_gas_usage() {
    let mut simulator = BlockchainSimulator::new();
    let contract = simulator.deploy_contract("./target/wasm32-unknown-unknown/release/token.wasm");
    
    // Create gas profiler
    let mut profiler = GasProfiler::new();
    
    // Start profiling
    profiler.start();
    
    // Invoke contract method
    let result = simulator.invoke(
        contract.address(),
        "transfer",
        &[address1, address2, 1000]
    );
    
    // Stop profiling
    profiler.stop();
    
    // Get gas usage report
    let report = profiler.report();
    
    // Assert gas usage is within acceptable limits
    assert!(report.gas_used < 10_000_000);
    
    // Print detailed report
    println!("{}", report);
}
```

### Security Analysis

```rust
use neo_contract_testing::security::SecurityAnalyzer;

#[test]
fn test_security_vulnerabilities() {
    let analyzer = SecurityAnalyzer::new("./target/wasm32-unknown-unknown/release/token.wasm");
    
    // Run security checks
    let report = analyzer.analyze();
    
    // Assert no critical vulnerabilities
    assert!(report.critical_vulnerabilities.is_empty());
    
    // Print security report
    println!("{}", report);
}
```

## Conclusion

A comprehensive testing strategy combines:

1. Unit tests for logic correctness
2. Integration tests for contract behavior
3. Simulated blockchain tests for realistic scenarios
4. On-chain tests for final verification

By applying these testing approaches, you can increase confidence in your contract's correctness, security, and reliability before deploying to the Neo MainNet.

## Resources

- [Neo Contract Testing Framework](https://github.com/neo-project/neo-contract-testing)
- [Neo Express](https://github.com/neo-project/neo-express)
- [Proptest Crate](https://docs.rs/proptest)
- [Neo Contract Examples](https://github.com/neo-project/examples)