# Neo Contract Testing Framework

A testing framework for Neo N3 smart contracts written in Rust.

## Overview

This framework provides tools for testing Neo smart contracts without deploying them to a blockchain. It includes:

- Mock implementations of Neo blockchain components (storage, runtime, etc.)
- A contract simulator for running contracts in a controlled environment
- Assertion utilities for verifying contract behavior
- Debugging utilities for analyzing contract execution
- Test fixtures for setting up and tearing down test environments

## Features

- **No-blockchain testing**: Test contracts without deploying to a real blockchain
- **Mock components**: Simulate storage, runtime, events, and transactions
- **Simulator**: Run contracts in a controlled environment
- **Assertions**: Verify storage, events, witnesses, and more
- **Debugging**: Capture and analyze operations during contract execution
- **Fixtures**: Set up and tear down test environments
- **no_std support**: Use in environments without the standard library

## Usage

### Basic Example

```rust
use neo_contract_testing::prelude::*;
use my_contract::MyContract;

#[test]
fn test_my_contract() {
    // Create a simulator
    let simulator = ContractSimulator::new();
    
    // Initialize contract
    let result = simulator.invoke(
        || MyContract::new("initial_value".into()),
        vec![b"initial_value".to_vec()],
    );
    assert!(result.success);
    
    // Call a method
    let result = simulator.invoke(
        |args| MyContract::set_value("new_value".into()),
        vec![b"new_value".to_vec()],
    );
    assert!(result.success);
    
    // Verify storage changes
    assert!(result.storage_changes.iter().any(|change| {
        matches!(change, StorageChange::Put { key, value, .. } if value == b"new_value")
    }));
}
```

### Setting Up Test Context

```rust
use neo_contract_testing::prelude::*;

#[test]
fn test_with_context() {
    // Create a custom test context
    let context = TestContext::new()
        .with_block_height(1000)
        .with_timestamp(1620000000000)
        .with_witness(vec![1, 2, 3, 4, 5]);
    
    // Create a simulator with the context
    let simulator = ContractSimulator::with_context(context);
    
    // Run your tests...
}
```

### Using Assertions

```rust
use neo_contract_testing::prelude::*;

#[test]
fn test_with_assertions() {
    // Create a simulator
    let simulator = ContractSimulator::new();
    
    // Run contract...
    
    // Use assertions
    assert_storage_has(b"my_key").unwrap();
    assert_storage_equals(b"my_key", b"my_value").unwrap();
    assert_event_emitted("Transfer").unwrap();
    assert_event_count("Transfer", 1).unwrap();
}
```

### Debug Capture

```rust
use neo_contract_testing::prelude::*;

#[test]
fn test_with_debug() {
    // Create a simulator with debug enabled
    let simulator = ContractSimulator::new()
        .with_capture_debug(true, TracingMode::Full);
    
    // Run contract...
    
    // Get debug report
    let report = simulator.debug_report();
    println!("{}", report);
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
neo-contract-testing = "0.1.0"
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.