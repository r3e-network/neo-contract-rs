# Testing Guide for neo-contract-rs Contracts

This guide provides strategies and patterns for testing Neo smart contracts written with the neo-contract-rs framework.

## Challenges of Testing Neo Contracts

Testing smart contracts presents unique challenges:

1. Neo contracts compile to WebAssembly and run in the Neo VM environment
2. Many contract functions interact with blockchain state (storage, witnesses, etc.)
3. Some operations require blockchain context (timestamps, gas, etc.)

## Testing Approaches

### 1. Unit Testing with Mock Environment

For contracts that don't heavily rely on blockchain-specific features, you can create mock implementations of Neo's environment.

Create a testing module in your contract:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock storage implementation
    struct MockStorage {
        data: std::collections::HashMap<Vec<u8>, Vec<u8>>
    }
    
    impl MockStorage {
        fn new() -> Self {
            Self {
                data: std::collections::HashMap::new()
            }
        }
        
        fn put(&mut self, key: &[u8], value: &[u8]) {
            self.data.insert(key.to_vec(), value.to_vec());
        }
        
        fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
            self.data.get(key).cloned()
        }
    }
    
    #[test]
    fn test_contract_logic() {
        // Setup mock environment
        let mut storage = MockStorage::new();
        
        // Test contract logic
        // ...
    }
}
```

### 2. Integration Testing with Neo-Express

For full integration testing, you can use [Neo-Express](https://github.com/neo-project/neo-express) to create a private Neo blockchain for testing.

1. Deploy your contract to Neo-Express
2. Interact with it using Neo SDK
3. Verify the results

### 3. Conditional Compilation for Testability

Restructure your contract to allow for testing:

```rust
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use neo_contract as neo;
use neo::{contract::*, types::*};

pub struct MyContract;

// Core logic separated from contract interface
impl MyContract {
    pub fn internal_logic(input: u32) -> u32 {
        // Pure logic that can be tested directly
        input * 2
    }
}

#[neo::contract]
impl ContractInterface for MyContract {
    pub fn my_method(input: u32) -> u32 {
        // Call internal logic
        MyContract::internal_logic(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_internal_logic() {
        assert_eq!(MyContract::internal_logic(21), 42);
    }
}
```

### 4. Custom Test Harness

For more complex contracts, create a dedicated test harness:

```rust
#[cfg(test)]
mod test_utils {
    use neo_contract::types::*;
    
    pub struct TestEnvironment {
        pub storage: std::collections::HashMap<Vec<u8>, Vec<u8>>,
        pub witnesses: Vec<H160>,
        pub timestamp: u64,
        // Other environment variables
    }
    
    impl TestEnvironment {
        pub fn new() -> Self {
            Self {
                storage: std::collections::HashMap::new(),
                witnesses: Vec::new(),
                timestamp: 0,
                // Initialize other fields
            }
        }
        
        pub fn with_storage(mut self, key: &[u8], value: &[u8]) -> Self {
            self.storage.insert(key.to_vec(), value.to_vec());
            self
        }
        
        pub fn with_witness(mut self, account: H160) -> Self {
            self.witnesses.push(account);
            self
        }
        
        // Other builder methods
    }
}
```

## Testing NEP-17 Token Contracts

Here's a specific example for testing a NEP-17 token:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_decimals() {
        assert_eq!(MyToken::decimals(), 8);
    }
    
    #[test]
    fn test_token_symbol() {
        assert_eq!(MyToken::symbol().as_bytes(), b"MTK");
    }
    
    // More complex tests would require mocking the Neo environment
}
```

## Test Coverage Recommendations

Aim to test:

1. **Pure Functions**: Any function that doesn't interact with blockchain state
2. **State Transitions**: Contract state changes (using mock storage)
3. **Error Conditions**: Ensure proper handling of invalid inputs
4. **Access Control**: Verify authorization checks work correctly
5. **Edge Cases**: Test boundary conditions like zero values, empty strings

## Advanced Testing with Neo Blockchain Emulation

For advanced testing, consider:

1. Creating a mock Neo VM implementation
2. Using [neo-go](https://github.com/nspcc-dev/neo-go) for testing in a simulated environment
3. Creating specialized testing utilities for common patterns

## Continuous Integration

Set up CI for automatic testing:

```yaml
# .github/workflows/test.yml
name: Tests

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
    - name: Run Tests
      run: cargo test
```

## Debugging Tips

1. Use `println!` debugging in test mode (not available in contracts)
2. Implement detailed error types for easier debugging
3. Test state transitions step by step
4. Use Neo's debug tools with compiled contracts

## Future Testing Improvements

The neo-contract-rs project aims to develop better testing tools:

1. Mocking libraries for Neo runtime
2. Testing utilities for common contract patterns
3. Integration with Neo blockchain simulation tools 