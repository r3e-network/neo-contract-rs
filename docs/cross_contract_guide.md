# Cross-Contract Communication Guide for Neo N3

This guide covers the patterns, best practices, and security considerations for communication between smart contracts on the Neo N3 blockchain. Effective cross-contract communication is essential for building complex decentralized applications consisting of multiple interacting contracts.

## Table of Contents

- [Introduction](#introduction)
- [Call Mechanisms in Neo N3](#call-mechanisms-in-neo-n3)
  - [Direct Contract Calls](#direct-contract-calls)
  - [Call Flags](#call-flags)
  - [Return Values](#return-values)
- [Common Communication Patterns](#common-communication-patterns)
  - [Contract Registry Pattern](#contract-registry-pattern)
  - [Factory Pattern](#factory-pattern)
  - [Proxy Pattern](#proxy-pattern)
  - [Event-Driven Communication](#event-driven-communication)
- [Security Considerations](#security-considerations)
  - [Reentrancy Protection](#reentrancy-protection)
  - [Access Control](#access-control)
  - [Circular Calls](#circular-calls)
  - [Error Handling](#error-handling)
- [Gas Considerations](#gas-considerations)
- [Debugging Tips](#debugging-tips)
- [Example Implementations](#example-implementations)
- [Conclusion](#conclusion)

## Introduction

Smart contracts often need to interact with other contracts to access functionality or data that resides in different parts of a decentralized application. The Neo N3 blockchain provides several mechanisms for contracts to communicate with each other, each with its own advantages, limitations, and security implications.

This guide will explain how to implement secure and efficient cross-contract communication using the Neo Contract Rust framework, with practical examples and best practices.

## Call Mechanisms in Neo N3

### Direct Contract Calls

In Neo N3, a contract can directly call methods of another contract using the contract hash (script hash) and method name:

```rust
use neo_contract::prelude::*;

#[method]
pub fn call_external_contract(target_contract: H160) -> bool {
    // Define the method name to call
    let method_name = "external_method";
    
    // Define the arguments to pass (if any)
    let args = vec![
        RuntimeValue::from(10u32),
        RuntimeValue::from("some string")
    ];
    
    // Call the external method
    let result: bool = Runtime::call_contract(&target_contract, method_name, &args).unwrap_or_default();
    
    result
}
```

The `Runtime::call_contract` method is the primary way to invoke external contracts in Neo N3. It takes:
- A contract hash (`H160`)
- A method name (string)
- Arguments as a vector of `RuntimeValue`

### Call Flags

When calling another contract, you can specify call flags to control the execution context:

```rust
#[method]
pub fn call_with_flags(target_contract: H160) -> bool {
    let method_name = "external_method";
    let args = vec![RuntimeValue::from(10u32)];
    
    // Use specific call flags
    let flags = CallFlags::ALL;
    
    // Call with flags
    let result: bool = Runtime::call_contract_with_flags(
        &target_contract, 
        method_name, 
        &args,
        flags
    ).unwrap_or_default();
    
    result
}
```

Neo N3 supports the following call flags:

- `CallFlags::NONE`: Default behavior
- `CallFlags::READ_ONLY`: Call is read-only and cannot modify state
- `CallFlags::ALLOW_CALL`: Allows the called contract to call methods of other contracts
- `CallFlags::ALLOW_NOTIFY`: Allows the called contract to emit notifications
- `CallFlags::ALL`: Allows all operations

### Return Values

Contracts can return values that are used by the calling contract:

```rust
#[method]
pub fn get_data_from_contract(data_contract: H160, key: String) -> Vec<u8> {
    let method_name = "get_data";
    let args = vec![RuntimeValue::from(key)];
    
    // Call and expect a Vec<u8> return value
    let result: Vec<u8> = Runtime::call_contract(&data_contract, method_name, &args)
        .expect("Failed to call contract");
    
    result
}
```

You need to ensure the return type matches what the called contract actually returns.

## Common Communication Patterns

### Contract Registry Pattern

The registry pattern maintains a central registry of contract addresses that other contracts can query:

```rust
#[contract]
pub struct ContractRegistry {
    contracts: StorageMap<String, H160>,
    owner: StorageItem<H160>,
}

#[contractimpl]
impl ContractRegistry {
    #[constructor]
    pub fn new() -> Self {
        let owner = Runtime::current_sender();
        Self {
            contracts: StorageMap::new(),
            owner: StorageItem::new(owner),
        }
    }
    
    #[method]
    pub fn register_contract(&mut self, name: String, contract_hash: H160) -> bool {
        // Only owner can register contracts
        let sender = Runtime::current_sender();
        if sender != self.owner.get() {
            return false;
        }
        
        self.contracts.insert(&name, &contract_hash);
        true
    }
    
    #[method]
    pub fn get_contract(&self, name: String) -> Option<H160> {
        self.contracts.get(&name)
    }
}
```

Other contracts can then query the registry to get the address of a specific contract:

```rust
#[method]
pub fn call_through_registry(&self, registry_contract: H160, contract_name: String) -> bool {
    // Get the contract address from the registry
    let args = vec![RuntimeValue::from(contract_name)];
    let target_contract: H160 = Runtime::call_contract(
        &registry_contract, 
        "get_contract", 
        &args
    ).expect("Failed to get contract address");
    
    // Call the target contract
    let call_args = vec![RuntimeValue::from(10u32)];
    let result: bool = Runtime::call_contract(
        &target_contract, 
        "some_method", 
        &call_args
    ).unwrap_or_default();
    
    result
}
```

This pattern allows for contract upgradability, as the registry can be updated to point to a new implementation without changing the client contracts.

### Factory Pattern

A factory contract creates and deploys other contracts:

```rust
#[contract]
pub struct ContractFactory {
    owner: StorageItem<H160>,
    created_contracts: StorageMap<H160, bool>,
}

#[contractimpl]
impl ContractFactory {
    #[constructor]
    pub fn new() -> Self {
        let owner = Runtime::current_sender();
        Self {
            owner: StorageItem::new(owner),
            created_contracts: StorageMap::new(),
        }
    }
    
    #[method]
    pub fn create_contract(&mut self, nef_file: Vec<u8>, manifest: Vec<u8>, data: Vec<u8>) -> H160 {
        // Only owner can create contracts
        let sender = Runtime::current_sender();
        if sender != self.owner.get() {
            panic!("Not authorized");
        }
        
        // Deploy the contract
        let contract_hash = Runtime::deploy_contract(&nef_file, &manifest, &data);
        
        // Track the created contract
        self.created_contracts.insert(&contract_hash, &true);
        
        // Return the new contract's hash
        contract_hash
    }
    
    #[method]
    pub fn is_created_by_factory(&self, contract_hash: H160) -> bool {
        self.created_contracts.get(&contract_hash).unwrap_or(false)
    }
}
```

This pattern is useful for creating standardized contracts with different initialization parameters.

### Proxy Pattern

The proxy pattern uses a stable proxy contract that delegates calls to a changeable implementation contract:

```rust
#[contract]
pub struct Proxy {
    owner: StorageItem<H160>,
    implementation: StorageItem<H160>,
}

#[contractimpl]
impl Proxy {
    #[constructor]
    pub fn new(initial_implementation: H160) -> Self {
        let owner = Runtime::current_sender();
        Self {
            owner: StorageItem::new(owner),
            implementation: StorageItem::new(initial_implementation),
        }
    }
    
    #[method]
    pub fn upgrade(&mut self, new_implementation: H160) -> bool {
        // Only owner can upgrade
        let sender = Runtime::current_sender();
        if sender != self.owner.get() {
            return false;
        }
        
        self.implementation.set(new_implementation);
        true
    }
    
    // Fallback method that delegates all calls to the implementation
    #[method]
    pub fn fallback(&self, method: String, args: Vec<RuntimeValue>) -> Vec<u8> {
        let impl_contract = self.implementation.get();
        
        // Forward the call to the implementation
        Runtime::call_contract(&impl_contract, &method, &args)
            .expect("Implementation call failed")
    }
}
```

Users interact with the proxy, which forwards calls to the current implementation. This enables upgrading the implementation without changing the contract address that users interact with.

### Event-Driven Communication

Contracts can communicate indirectly through events:

```rust
#[event]
struct DataChanged {
    #[index]
    key: String,
    value: Vec<u8>,
}

#[method]
pub fn set_data(&mut self, key: String, value: Vec<u8>) -> bool {
    // Update internal state
    self.data.insert(&key, &value);
    
    // Notify interested contracts or clients
    Runtime::notify(
        &DataChanged {
            key,
            value,
        }
    );
    
    true
}
```

Other contracts or off-chain applications can listen for these events and react accordingly. This is a loosely coupled form of communication.

## Security Considerations

### Reentrancy Protection

When calling external contracts, be aware of reentrancy vulnerabilities:

```rust
#[method]
pub fn transfer_with_notification(&mut self, to: H160, amount: u64) -> bool {
    // Check if the caller has enough balance
    let sender = Runtime::current_sender();
    let sender_balance = self.balances.get(&sender).unwrap_or(0);
    
    assert!(sender_balance >= amount, "Insufficient balance");
    
    // VULNERABLE: Updating state after the external call
    let notification_contract = H160::from_slice(&[/* contract hash */]);
    Runtime::call_contract(&notification_contract, "notify_transfer", &vec![
        RuntimeValue::from(sender),
        RuntimeValue::from(to),
        RuntimeValue::from(amount),
    ]).unwrap_or_default();
    
    // State is updated after the external call, opening reentrancy vulnerability
    self.balances.insert(&sender, &(sender_balance - amount));
    self.balances.insert(&to, &(self.balances.get(&to).unwrap_or(0) + amount));
    
    true
}
```

To protect against reentrancy, always update your state before calling external contracts:

```rust
#[method]
pub fn transfer_with_notification_safe(&mut self, to: H160, amount: u64) -> bool {
    // Check if the caller has enough balance
    let sender = Runtime::current_sender();
    let sender_balance = self.balances.get(&sender).unwrap_or(0);
    
    assert!(sender_balance >= amount, "Insufficient balance");
    
    // SAFE: Update state before external call
    self.balances.insert(&sender, &(sender_balance - amount));
    self.balances.insert(&to, &(self.balances.get(&to).unwrap_or(0) + amount));
    
    // Now make the external call
    let notification_contract = H160::from_slice(&[/* contract hash */]);
    Runtime::call_contract(&notification_contract, "notify_transfer", &vec![
        RuntimeValue::from(sender),
        RuntimeValue::from(to),
        RuntimeValue::from(amount),
    ]).unwrap_or_default();
    
    true
}
```

### Access Control

Implement proper access control for contracts that are meant to be called by specific contracts:

```rust
#[method]
pub fn restricted_operation(&mut self) -> bool {
    // Get the calling contract hash
    let calling_contract = Runtime::calling_script_hash();
    
    // Whitelist of authorized contracts
    let authorized = vec![
        H160::from_slice(&[/* authorized contract hash 1 */]),
        H160::from_slice(&[/* authorized contract hash 2 */]),
    ];
    
    // Check if the caller is authorized
    assert!(authorized.contains(&calling_contract), "Unauthorized caller");
    
    // Proceed with the operation
    // ...
    
    true
}
```

### Circular Calls

Avoid circular calls between contracts, as they can lead to unexpected behavior and excessive gas consumption:

```rust
// Contract A
#[method]
pub fn method_a(&mut self, contract_b: H160) -> bool {
    // Call contract B
    Runtime::call_contract(&contract_b, "method_b", &vec![
        RuntimeValue::from(Runtime::current_script_hash())
    ]).unwrap_or_default()
}

// Contract B
#[method]
pub fn method_b(&mut self, contract_a: H160) -> bool {
    // Dangerous: Potentially creates an infinite loop
    Runtime::call_contract(&contract_a, "method_a", &vec![
        RuntimeValue::from(Runtime::current_script_hash())
    ]).unwrap_or_default()
}
```

Implement proper termination conditions and track call depth to avoid infinite loops.

### Error Handling

Handle errors from external contract calls gracefully:

```rust
#[method]
pub fn safe_external_call(&self, target_contract: H160) -> bool {
    // Call with proper error handling
    match Runtime::call_contract::<bool>(&target_contract, "some_method", &vec![]) {
        Some(result) => {
            // Handle the successful result
            result
        },
        None => {
            // Handle the failure case
            println!("External call failed");
            false
        }
    }
}
```

## Gas Considerations

Cross-contract calls consume additional gas for the contract invocation overhead. Consider these tips to optimize gas usage:

1. **Batch Operations**: Combine multiple operations into a single contract call
2. **Minimize Calls**: Cache results when possible instead of making repeated calls
3. **Use Read-Only Calls**: For queries, use `CallFlags::READ_ONLY` to reduce gas costs
4. **Optimize Data Transfer**: Only pass the necessary data between contracts

## Debugging Tips

Debugging cross-contract interactions can be challenging. Here are some tips:

1. **Use Events**: Emit events at key points in your contract to track execution flow
2. **Implement Verbose Modes**: Add debug methods that can be toggled for more detailed logging
3. **Test Contracts Individually**: Verify each contract works correctly in isolation before testing interactions
4. **Mock Contracts**: For testing, create mock versions of external contracts with simplified behavior
5. **Use a Local Private Net**: Deploy and test your contracts on a private Neo N3 network for debugging

## Example Implementations

For complete examples of cross-contract communication, refer to these implementations:

1. [Contract Registry Example](../examples/cross_contract/registry)
2. [Proxy Pattern Example](../examples/cross_contract/proxy)
3. [DEX Example](../examples/defi/dex) - Shows multiple contracts interacting in a financial system

## Conclusion

Effective cross-contract communication is essential for building complex decentralized applications on Neo N3. By understanding the available mechanisms and implementing proper security measures, you can create robust and secure multi-contract systems.

Remember to always:
- Implement proper access control
- Protect against reentrancy attacks
- Optimize for gas efficiency
- Handle errors gracefully
- Test thoroughly across all contract interactions

For more information on related topics, see the following guides:
- [Contract Security Guide](./contract_security_guide.md)
- [Gas Optimization Guide](./gas_optimization.md)
- [Events Guide](./events_guide.md)
- [Transaction Patterns Guide](./transaction_patterns.md) 