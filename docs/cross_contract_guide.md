# Neo Contract Cross-Contract Communication Guide

This guide explains how to implement secure and efficient cross-contract communication in Neo N3 smart contracts using the neo-contract-rs framework.

## Overview

Cross-contract communication allows smart contracts to interact with each other, enabling complex decentralized applications. Neo N3 provides several mechanisms for contracts to call each other and share data.

## Contract Calling Methods

### 1. Basic Contract Calls

```rust
use neo_contract::prelude::*;

impl MyContract {
    #[method]
    pub fn call_external_contract(&self, target: H160, amount: u64) -> bool {
        let args = vec![Any::from(amount)];
        
        match Runtime::call_contract(target, "transfer", &args) {
            Ok(result) => {
                // Handle successful call
                true
            }
            Err(_) => {
                // Handle failure
                false
            }
        }
    }
}
```

### 2. Safe Contract Calls with Error Handling

```rust
impl MyContract {
    fn safe_call_contract(&self, contract: H160, method: &str, args: &[Any]) -> Option<Any> {
        // Validate contract address
        if contract == H160::zero() {
            return None;
        }
        
        // Check if contract exists
        if !Runtime::contract_exists(contract) {
            return None;
        }
        
        // Make the call with error handling
        Runtime::call_contract(contract, method, args).ok()
    }
    
    #[method]
    pub fn secure_external_call(&mut self, token: H160, recipient: H160, amount: u64) -> bool {
        let args = vec![
            Any::from(recipient),
            Any::from(amount)
        ];
        
        match self.safe_call_contract(token, "transfer", &args) {
            Some(result) => {
                // Update internal state on successful external call
                self.update_transfer_record(token, recipient, amount);
                true
            }
            None => false
        }
    }
}
```

### 3. Contract Interface Definitions

```rust
// Define standardized interfaces for better type safety
pub trait NEP17Token {
    fn symbol(&self) -> String;
    fn decimals(&self) -> u8;
    fn total_supply(&self) -> u64;
    fn balance_of(&self, account: H160) -> u64;
    fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool;
}

impl MyContract {
    fn call_nep17_method(&self, token: H160, method: &str, args: &[Any]) -> Option<Any> {
        // Validate that the contract implements NEP-17
        if !self.validate_nep17_interface(token) {
            return None;
        }
        
        self.safe_call_contract(token, method, args)
    }
    
    fn validate_nep17_interface(&self, token: H160) -> bool {
        // Check if contract implements required NEP-17 methods
        let required_methods = ["symbol", "decimals", "totalSupply", "balanceOf", "transfer"];
        
        for method in &required_methods {
            if !self.contract_has_method(token, method) {
                return false;
            }
        }
        
        true
    }
}
```

## Cross-Contract Patterns

### 1. Registry Pattern

```rust
#[contract]
pub struct ContractRegistry {
    #[storage]
    registered_contracts: StorageMap<String, H160>,
    
    #[storage]
    contract_metadata: StorageMap<H160, ContractInfo>,
    
    #[storage]
    owner: StorageItem<H160>,
}

#[derive(Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub version: String,
    pub interface_hash: H256,
    pub is_active: bool,
}

impl ContractRegistry {
    #[method]
    pub fn register_contract(&mut self, name: String, contract: H160, info: ContractInfo) -> bool {
        if !self.require_owner() {
            return false;
        }
        
        // Validate contract
        if !Runtime::contract_exists(contract) {
            return false;
        }
        
        self.registered_contracts.put(&name, contract);
        self.contract_metadata.put(&contract, info);
        
        true
    }
    
    #[safe]
    pub fn get_contract(&self, name: &str) -> Option<H160> {
        self.registered_contracts.get(name)
    }
    
    #[method]
    pub fn call_registered_contract(&self, name: String, method: String, args: Vec<Any>) -> Option<Any> {
        if let Some(contract) = self.get_contract(&name) {
            if let Some(info) = self.contract_metadata.get(&contract) {
                if info.is_active {
                    return Runtime::call_contract(contract, &method, &args).ok();
                }
            }
        }
        None
    }
}
```

### 2. Proxy Pattern

```rust
#[contract]
pub struct ContractProxy {
    #[storage]
    implementation: StorageItem<H160>,
    
    #[storage]
    admin: StorageItem<H160>,
}

impl ContractProxy {
    #[method]
    pub fn upgrade(&mut self, new_implementation: H160) -> bool {
        if !self.require_admin() {
            return false;
        }
        
        // Validate new implementation
        if !Runtime::contract_exists(new_implementation) {
            return false;
        }
        
        self.implementation.put(new_implementation);
        true
    }
    
    #[method]
    pub fn delegate_call(&self, method: String, args: Vec<Any>) -> Option<Any> {
        if let Some(impl_contract) = self.implementation.get() {
            Runtime::call_contract(impl_contract, &method, &args).ok()
        } else {
            None
        }
    }
}
```

### 3. Factory Pattern

```rust
#[contract]
pub struct TokenFactory {
    #[storage]
    created_tokens: StorageMap<H160, TokenInfo>,
    
    #[storage]
    template_contract: StorageItem<H160>,
    
    #[storage]
    token_count: StorageItem<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    pub creator: H160,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub created_at: u64,
}

impl TokenFactory {
    #[method]
    pub fn create_token(&mut self, name: String, symbol: String, supply: u64) -> Option<H160> {
        let creator = Runtime::calling_script_hash();
        let template = self.template_contract.get()?;
        
        // Deploy new contract using template
        let init_args = vec![
            Any::from(name.clone()),
            Any::from(symbol.clone()),
            Any::from(supply),
            Any::from(creator)
        ];
        
        if let Ok(new_token) = Runtime::deploy_contract(template, &init_args) {
            let token_info = TokenInfo {
                creator,
                name,
                symbol,
                total_supply: supply,
                created_at: Runtime::time(),
            };
            
            self.created_tokens.put(&new_token, token_info);
            
            let count = self.token_count.get().unwrap_or(0);
            self.token_count.put(count + 1);
            
            Some(new_token)
        } else {
            None
        }
    }
}
```

## Advanced Cross-Contract Techniques

### 1. Contract Composition

```rust
#[contract]
pub struct CompositeContract {
    #[storage]
    token_contract: StorageItem<H160>,
    
    #[storage]
    oracle_contract: StorageItem<H160>,
    
    #[storage]
    exchange_contract: StorageItem<H160>,
}

impl CompositeContract {
    #[method]
    pub fn complex_operation(&mut self, amount: u64) -> bool {
        let token = self.token_contract.get().unwrap();
        let oracle = self.oracle_contract.get().unwrap();
        let exchange = self.exchange_contract.get().unwrap();
        
        // 1. Get current price from oracle
        let price_args = vec![Any::from("TOKEN_USD")];
        let price = Runtime::call_contract(oracle, "getPrice", &price_args)
            .ok()?
            .try_into::<u64>()
            .ok()?;
        
        // 2. Calculate required token amount
        let token_amount = (amount * 10000) / price; // Assuming price is in basis points
        
        // 3. Transfer tokens to exchange
        let transfer_args = vec![
            Any::from(Runtime::executing_script_hash()),
            Any::from(exchange),
            Any::from(token_amount)
        ];
        
        let transfer_success = Runtime::call_contract(token, "transfer", &transfer_args)
            .ok()?
            .try_into::<bool>()
            .unwrap_or(false);
        
        if !transfer_success {
            return false;
        }
        
        // 4. Execute trade on exchange
        let trade_args = vec![
            Any::from(token),
            Any::from(token_amount),
            Any::from(amount)
        ];
        
        Runtime::call_contract(exchange, "executeSwap", &trade_args)
            .ok()?
            .try_into::<bool>()
            .unwrap_or(false)
    }
}
```

### 2. Event-Driven Communication

```rust
#[event]
pub struct CrossContractCall {
    #[index]
    pub caller: H160,
    #[index]
    pub target: H160,
    pub method: String,
    pub success: bool,
}

impl MyContract {
    #[method]
    pub fn monitored_call(&self, target: H160, method: String, args: Vec<Any>) -> bool {
        let caller = Runtime::executing_script_hash();
        
        let success = Runtime::call_contract(target, &method, &args).is_ok();
        
        // Emit event for monitoring
        self.emit_cross_contract_call(caller, target, method, success);
        
        success
    }
}
```

### 3. Callback Pattern

```rust
#[contract]
pub struct CallbackContract {
    #[storage]
    pending_callbacks: StorageMap<H256, PendingCallback>,
}

#[derive(Serialize, Deserialize)]
pub struct PendingCallback {
    pub caller: H160,
    pub callback_method: String,
    pub callback_data: Vec<u8>,
    pub expiry: u64,
}

impl CallbackContract {
    #[method]
    pub fn async_operation(&mut self, target: H160, data: Vec<u8>) -> H256 {
        let callback_id = self.generate_callback_id();
        let caller = Runtime::calling_script_hash();
        
        // Store callback information
        let callback = PendingCallback {
            caller,
            callback_method: "handle_result".to_string(),
            callback_data: data.clone(),
            expiry: Runtime::time() + 86400, // 24 hours
        };
        
        self.pending_callbacks.put(&callback_id, callback);
        
        // Call external contract with callback ID
        let args = vec![
            Any::from(data),
            Any::from(callback_id.as_bytes().to_vec())
        ];
        
        Runtime::call_contract(target, "process_async", &args);
        
        callback_id
    }
    
    #[method]
    pub fn execute_callback(&mut self, callback_id: H256, result: Vec<u8>) -> bool {
        if let Some(callback) = self.pending_callbacks.get(&callback_id) {
            if Runtime::time() > callback.expiry {
                self.pending_callbacks.delete(&callback_id);
                return false; // Expired
            }
            
            // Execute callback
            let args = vec![
                Any::from(callback_id.as_bytes().to_vec()),
                Any::from(result)
            ];
            
            let success = Runtime::call_contract(
                callback.caller,
                &callback.callback_method,
                &args
            ).is_ok();
            
            // Clean up
            self.pending_callbacks.delete(&callback_id);
            
            success
        } else {
            false
        }
    }
}
```

## Security Considerations

### 1. Reentrancy Protection

```rust
#[contract]
pub struct SecureCrossCaller {
    #[storage]
    reentrancy_lock: StorageItem<bool>,
}

impl SecureCrossCaller {
    #[method]
    #[no_reentrant] // Use framework protection
    pub fn secure_external_call(&mut self, target: H160, amount: u64) -> bool {
        // Additional manual protection if needed
        if self.reentrancy_lock.get().unwrap_or(false) {
            return false;
        }
        
        self.reentrancy_lock.put(true);
        
        // Update state before external call
        let caller = Runtime::calling_script_hash();
        let balance = self.balances.get(&caller).unwrap_or(0);
        
        if balance < amount {
            self.reentrancy_lock.put(false);
            return false;
        }
        
        self.balances.put(&caller, balance - amount);
        
        // External call
        let args = vec![Any::from(amount)];
        let result = Runtime::call_contract(target, "receive", &args).is_ok();
        
        self.reentrancy_lock.put(false);
        result
    }
}
```

### 2. Contract Validation

```rust
impl MyContract {
    fn validate_contract_call(&self, target: H160, method: &str) -> bool {
        // Check if contract exists
        if !Runtime::contract_exists(target) {
            return false;
        }
        
        // Check if contract is on whitelist
        if !self.is_whitelisted_contract(target) {
            return false;
        }
        
        // Check if method is allowed
        if !self.is_allowed_method(target, method) {
            return false;
        }
        
        true
    }
    
    fn is_whitelisted_contract(&self, contract: H160) -> bool {
        self.whitelisted_contracts.get(&contract).unwrap_or(false)
    }
    
    fn is_allowed_method(&self, contract: H160, method: &str) -> bool {
        if let Some(allowed_methods) = self.allowed_methods.get(&contract) {
            allowed_methods.contains(&method.to_string())
        } else {
            false
        }
    }
}
```

### 3. Gas Limit Management

```rust
impl MyContract {
    #[method]
    pub fn gas_limited_call(&self, target: H160, method: String, args: Vec<Any>, gas_limit: u64) -> bool {
        let gas_before = Runtime::gas_left();
        
        if gas_before < gas_limit {
            return false; // Not enough gas
        }
        
        // Reserve gas for cleanup
        let reserved_gas = 10000;
        let available_gas = gas_before - reserved_gas;
        
        if available_gas < gas_limit {
            return false;
        }
        
        // Make call with monitoring
        let result = Runtime::call_contract(target, &method, &args);
        
        let gas_used = gas_before - Runtime::gas_left();
        
        // Log gas usage for monitoring
        Runtime::log(&format!("Cross-contract call used {} gas", gas_used));
        
        result.is_ok()
    }
}
```

## Testing Cross-Contract Interactions

```rust
#[cfg(test)]
mod cross_contract_tests {
    use super::*;
    
    #[test]
    fn test_cross_contract_call() {
        let mut caller = CallerContract::new();
        let mut target = TargetContract::new();
        
        // Deploy both contracts in test environment
        let target_address = deploy_contract(target);
        
        // Test successful call
        let result = caller.call_external_contract(target_address, 100);
        assert!(result);
        
        // Verify state changes in both contracts
        assert_eq!(caller.get_call_count(), 1);
        assert_eq!(target.get_last_amount(), 100);
    }
    
    #[test]
    fn test_failed_call_handling() {
        let mut caller = CallerContract::new();
        let invalid_address = H160::zero();
        
        // Test call to invalid contract
        let result = caller.call_external_contract(invalid_address, 100);
        assert!(!result);
        
        // Verify no state changes on failure
        assert_eq!(caller.get_call_count(), 0);
    }
}
```

## Best Practices

### 1. Interface Standardization

- Define clear interfaces for contract interactions
- Use standardized method names and signatures
- Document expected behavior and error conditions

### 2. Error Handling

- Always check return values from external calls
- Handle failures gracefully without breaking contract state
- Provide meaningful error messages and recovery mechanisms

### 3. Gas Management

- Monitor gas usage in cross-contract calls
- Set appropriate gas limits for external operations
- Reserve gas for cleanup and error handling

### 4. Security

- Validate external contract addresses
- Use reentrancy protection
- Implement access controls for critical operations
- Whitelist trusted contracts

## See Also

- [Contract Security Guide](contract_security_guide.md)
- [Storage Guide](storage_guide.md)
- [Events Guide](events_guide.md)
- [Gas Optimization Guide](gas_optimization.md) 