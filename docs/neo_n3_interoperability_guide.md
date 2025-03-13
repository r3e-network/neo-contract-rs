# Neo N3 Interoperability Guide

This guide provides detailed instructions for implementing contract interoperability in Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Interoperability is a key feature of the Neo N3 blockchain, allowing smart contracts to interact with:

1. Other smart contracts on the blockchain
2. Native contracts (built-in system contracts)
3. Oracle services for off-chain data

## Contract-to-Contract Communication

### Calling Other Contracts

To call a method on another contract from your Neo N3 smart contract:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct MyContract {
    // Contract fields
}

#[neo_contract]
impl MyContract {
    #[method]
    pub fn call_another_contract(&self) -> bool {
        // 1. Define the target contract's script hash
        let target_contract_hash = H160::from("0x0123456789abcdef0123456789abcdef01234567");
        
        // 2. Define the method to call
        let method = ByteString::from("target_method");
        
        // 3. Define arguments as an Array of Any values
        let mut args = Array::<Any>::new();
        args.push(Any::from(10u32)); // Add an integer parameter
        args.push(Any::from(H160::from("0xabcdef0123456789abcdef0123456789abcdef01"))); // Add an address
        
        // 4. Call the contract
        let result = Contract::call(&target_contract_hash, &method, &args);
        
        // 5. Handle the result
        match result {
            Some(value) => {
                // Convert the Any value to the expected return type
                if let Ok(success) = value.as_bool() {
                    return success;
                }
                false
            }
            None => false
        }
    }
}
```

### Reading Return Values from Contract Calls

When handling return values from contract calls, cast the `Any` result to the expected type:

```rust
// For boolean results
if let Ok(success) = result.as_bool() {
    // Handle boolean result
}

// For integer results
if let Ok(amount) = result.as_int() {
    // Handle integer result
}

// For ByteString results
if let Ok(name) = result.as_bytes() {
    // Handle ByteString result
}

// For H160 address results
if let Ok(address_bytes) = result.as_bytes() {
    if address_bytes.len() == 20 {
        let address = H160::from_slice(&address_bytes);
        // Handle H160 address
    }
}
```

### Interoperability Permissions

In your contract manifest, you must specify permissions for the contracts your contract will call:

```json
"permissions": [
  {
    "contract": "0x0123456789abcdef0123456789abcdef01234567",
    "methods": ["target_method"]
  }
]
```

For wildcard permissions (not recommended for production):

```json
"permissions": [
  {
    "contract": "*",
    "methods": "*"
  }
]
```

## Interacting with Native Contracts

Neo N3 provides several native contracts with predefined functionality:

### GAS Token Contract

```rust
// Transfer GAS tokens
#[method]
pub fn transfer_gas(&self, to: H160, amount: u64) -> bool {
    let gas_contract = H160::from("0xd2a4cff31913016155e38e474a2c06d08be276cf"); // GAS contract hash
    let method = ByteString::from("transfer");
    
    let sender = Runtime::check_witness(&self.owner);
    if !sender {
        return false;
    }
    
    let mut args = Array::<Any>::new();
    args.push(Any::from(Runtime::executing_script_hash())); // from (this contract)
    args.push(Any::from(to)); // to address
    args.push(Any::from(amount)); // amount
    args.push(Any::new()); // data (optional, passing null)
    
    let result = Contract::call(&gas_contract, &method, &args);
    match result {
        Some(value) => value.as_bool().unwrap_or(false),
        None => false
    }
}
```

### NEO Token Contract

```rust
// Check NEO balance
#[safe]
pub fn check_neo_balance(&self, address: H160) -> u64 {
    let neo_contract = H160::from("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"); // NEO contract hash
    let method = ByteString::from("balanceOf");
    
    let mut args = Array::<Any>::new();
    args.push(Any::from(address));
    
    let result = Contract::call(&neo_contract, &method, &args);
    match result {
        Some(value) => value.as_int().unwrap_or(0) as u64,
        None => 0
    }
}
```

### Contract Management

For updating your contract:

```rust
#[method]
pub fn update(&self, script: ByteString, manifest: ByteString) -> bool {
    // Only contract owner can update
    if !Runtime::check_witness(&self.owner) {
        return false;
    }
    
    let management_contract = H160::from("0xfffdc93764dbaddd97c48f252a53ea4643faa3fd"); // Management contract hash
    let method = ByteString::from("update");
    
    let mut args = Array::<Any>::new();
    args.push(Any::from(script));
    args.push(Any::from(manifest));
    
    let result = Contract::call(&management_contract, &method, &args);
    match result {
        Some(value) => value.as_bool().unwrap_or(false),
        None => false
    }
}
```

### Other Native Contracts

Common native contracts in Neo N3:

| Contract | Hash | Description |
|----------|------|-------------|
| ContractManagement | 0xfffdc93764dbaddd97c48f252a53ea4643faa3fd | Manages contract deployment and updates |
| GasToken | 0xd2a4cff31913016155e38e474a2c06d08be276cf | The GAS utility token |
| NeoToken | 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5 | The NEO governance token |
| LedgerContract | 0xda65b600f7124ce6c79950c1772a36403104f2be | Provides access to blockchain info |
| OracleContract | 0xfe924b7cfe89ddd271abaf7210a80a7e11178758 | Facilitates oracle requests |
| PolicyContract | 0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b | Manages blockchain system settings |

## Oracle Services Integration

Neo N3 includes native oracle support for accessing off-chain data:

```rust
#[contract]
pub struct OracleConsumer {
    oracle_request_id: Storage<u64>,
    result_data: Storage<ByteString>,
}

#[neo_contract]
impl OracleConsumer {
    #[method]
    pub fn request_price_data(&mut self) -> bool {
        // 1. Define the oracle contract hash
        let oracle_contract = H160::from("0xfe924b7cfe89ddd271abaf7210a80a7e11178758");
        
        // 2. Define the callback method that will be called when oracle responds
        let callback = ByteString::from("callback_price_data");
        
        // 3. Define the URL to fetch data from
        let url = ByteString::from("https://api.example.com/price/btc");
        
        // 4. Define the filter to extract data (JSONPath)
        let filter = ByteString::from("$.price");
        
        // 5. Set request parameters
        let mut args = Array::<Any>::new();
        args.push(Any::from(url));      // URL
        args.push(Any::from(filter));   // JSONPath filter
        args.push(Any::from(callback)); // Callback method name
        args.push(Any::from(1));        // User data (can be any value)
        args.push(Any::from(Runtime::executing_script_hash())); // Callback contract
        
        // 6. Make the oracle request
        let result = Contract::call(&oracle_contract, &ByteString::from("request"), &args);
        
        // 7. Store the request ID
        if let Some(value) = result {
            if let Ok(id) = value.as_int() {
                self.oracle_request_id.put(id as u64);
                return true;
            }
        }
        
        false
    }
    
    // Callback method that will be called by the oracle
    #[method]
    pub fn callback_price_data(&mut self, id: u64, result: ByteString) -> bool {
        // Only oracle contract can call this method
        let oracle_contract = H160::from("0xfe924b7cfe89ddd271abaf7210a80a7e11178758");
        if Runtime::calling_script_hash() != oracle_contract {
            return false;
        }
        
        // Verify this is the request we're expecting
        if self.oracle_request_id.get() != id {
            return false;
        }
        
        // Store the result
        self.result_data.put(result);
        
        // Emit an event
        let event_name = ByteString::from("OracleDataReceived");
        let mut event_data = Array::<Any>::new();
        event_data.push(Any::from(id));
        event_data.push(Any::from(result));
        Runtime::notify(&event_name, &event_data);
        
        true
    }
    
    // Get the latest oracle data
    #[safe]
    pub fn get_price_data(&self) -> ByteString {
        self.result_data.get()
    }
}
```

## Best Practices for Contract Interoperability

### Security Considerations

1. **Verify Calling Contracts**: Always check who is calling your contract when handling sensitive operations:

```rust
#[method]
pub fn sensitive_method(&self) -> bool {
    // Only allow calls from a trusted contract
    let trusted_contract = H160::from("0x0123456789abcdef0123456789abcdef01234567");
    if Runtime::calling_script_hash() != trusted_contract {
        return false;
    }
    
    // Proceed with sensitive operation
    true
}
```

2. **Use Specific Permissions**: Avoid wildcard permissions in your manifest. Specify exactly which contracts and methods your contract will call.

3. **Check Return Values**: Always handle the possibility of `None` or unexpected values when calling other contracts.

4. **Reentrancy Protection**: Implement guards against reentrancy attacks:

```rust
#[contract]
pub struct SecureContract {
    locked: Storage<bool>,
    // Other fields
}

#[neo_contract]
impl SecureContract {
    #[method]
    pub fn secure_method(&mut self) -> bool {
        // Check if already locked
        if self.locked.get() {
            return false;
        }
        
        // Lock the contract
        self.locked.put(true);
        
        // Call external contract
        let external_contract = H160::from("0x0123456789abcdef0123456789abcdef01234567");
        let result = Contract::call(&external_contract, &ByteString::from("some_method"), &Array::<Any>::new());
        
        // Perform state changes
        // ...
        
        // Unlock the contract
        self.locked.put(false);
        
        true
    }
}
```

### Efficiency Guidelines

1. **Batch Calls**: Combine multiple operations into one call where possible.

2. **Cache External Data**: Store frequently used data from other contracts to avoid repeated calls.

3. **Optimize Argument Passing**: Only pass necessary data to reduce gas costs.

### Testing Interoperability

Test contract interactions using:

1. **Mock Contracts**: Create simplified test versions of contracts for unit testing.

2. **Integration Testing**: Test on a private Neo N3 network to verify real interactions.

3. **Event Verification**: Check for proper events when contracts interact.

## Common Interoperability Patterns

### Token Escrow Pattern

```rust
#[contract]
pub struct EscrowContract {
    deposits: StorageMap<(H160, H160), u64>, // (token_hash, user) -> amount
}

#[neo_contract]
impl EscrowContract {
    // Deposit tokens into escrow
    #[method]
    pub fn deposit(&mut self, token_hash: H160, amount: u64) -> bool {
        let sender = Runtime::calling_script_hash();
        
        // Call token contract to transfer tokens
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        args.push(Any::from(sender)); // from
        args.push(Any::from(Runtime::executing_script_hash())); // to (this contract)
        args.push(Any::from(amount)); // amount
        args.push(Any::new()); // data (null)
        
        let result = Contract::call(&token_hash, &method, &args);
        
        match result {
            Some(value) => {
                if value.as_bool().unwrap_or(false) {
                    // Update deposit record
                    let key = (token_hash, sender);
                    let current = self.deposits.get(&key).unwrap_or(0);
                    self.deposits.insert(&key, current + amount);
                    
                    // Emit deposit event
                    let event_name = ByteString::from("Deposit");
                    let mut event_data = Array::<Any>::new();
                    event_data.push(Any::from(sender));
                    event_data.push(Any::from(token_hash));
                    event_data.push(Any::from(amount));
                    Runtime::notify(&event_name, &event_data);
                    
                    return true;
                }
            }
            None => {}
        }
        
        false
    }
    
    // Withdraw tokens from escrow
    #[method]
    pub fn withdraw(&mut self, token_hash: H160, amount: u64) -> bool {
        let sender = Runtime::calling_script_hash();
        
        // Check if user has sufficient deposit
        let key = (token_hash, sender);
        let current = self.deposits.get(&key).unwrap_or(0);
        if current < amount {
            return false;
        }
        
        // Update deposit record first to prevent reentrancy
        self.deposits.insert(&key, current - amount);
        
        // Transfer tokens back to user
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        args.push(Any::from(Runtime::executing_script_hash())); // from (this contract)
        args.push(Any::from(sender)); // to
        args.push(Any::from(amount)); // amount
        args.push(Any::new()); // data (null)
        
        let result = Contract::call(&token_hash, &method, &args);
        
        match result {
            Some(value) => {
                if value.as_bool().unwrap_or(false) {
                    // Emit withdraw event
                    let event_name = ByteString::from("Withdraw");
                    let mut event_data = Array::<Any>::new();
                    event_data.push(Any::from(sender));
                    event_data.push(Any::from(token_hash));
                    event_data.push(Any::from(amount));
                    Runtime::notify(&event_name, &event_data);
                    
                    return true;
                } else {
                    // Revert the deposit record if transfer failed
                    self.deposits.insert(&key, current);
                }
            }
            None => {
                // Revert the deposit record if transfer failed
                self.deposits.insert(&key, current);
            }
        }
        
        false
    }
    
    // Check user's deposit balance
    #[safe]
    pub fn balance_of(&self, token_hash: H160, user: H160) -> u64 {
        let key = (token_hash, user);
        self.deposits.get(&key).unwrap_or(0)
    }
}
```

### Multi-Contract Architecture

For complex dApps, you can implement a modular design with multiple contracts:

1. **Registry Contract**: Manages the addresses of other contracts in your system
2. **Access Control Contract**: Handles permissions
3. **Data Storage Contracts**: Store specific data types
4. **Business Logic Contracts**: Implement core functionality

Example Registry Contract:

```rust
#[contract]
pub struct RegistryContract {
    owner: Storage<H160>,
    contracts: StorageMap<ByteString, H160>,
}

#[neo_contract]
impl RegistryContract {
    #[method]
    pub fn init(&mut self, owner: H160) -> bool {
        if !self.owner.is_empty() {
            return false; // Already initialized
        }
        
        self.owner.put(owner);
        true
    }
    
    #[method]
    pub fn register_contract(&mut self, name: ByteString, address: H160) -> bool {
        // Only owner can register contracts
        if Runtime::check_witness(&self.owner.get()) {
            self.contracts.insert(&name, address);
            
            // Emit registration event
            let event_name = ByteString::from("ContractRegistered");
            let mut event_data = Array::<Any>::new();
            event_data.push(Any::from(name));
            event_data.push(Any::from(address));
            Runtime::notify(&event_name, &event_data);
            
            return true;
        }
        
        false
    }
    
    #[safe]
    pub fn get_contract(&self, name: ByteString) -> H160 {
        self.contracts.get(&name).unwrap_or_default()
    }
}
```

## Conclusion

Neo N3 provides powerful interoperability features that allow contracts to communicate effectively with other contracts and external services. By following the patterns and best practices outlined in this guide, you can build complex, secure, and efficient decentralized applications on the Neo N3 blockchain.

For more information on Neo N3 contract development, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Security Guide](./neo_n3_security_guide.md)
- [Neo N3 Deployment Guide](./neo_n3_deployment_guide.md)
- [Neo N3 NEP-17 Guide](./neo_n3_nep17_guide.md)
- [Neo N3 NEP-11 Guide](./neo_n3_nep11_guide.md)
