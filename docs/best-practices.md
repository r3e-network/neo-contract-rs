# Best Practices for Neo N3 Contract Development

This document outlines recommended practices for developing secure, efficient, and maintainable smart contracts for the Neo N3 blockchain using the Rust framework.

## Contract Design

### Modular Architecture

- **Single Responsibility**: Design contracts with a single, well-defined purpose
- **Interface Separation**: Split complex functionality into multiple contracts
- **Upgradability**: Consider using contract management patterns for upgradability
- **Standard Compliance**: Follow established standards (NEP-17, NEP-11, etc.)

### State Management

- **Minimal Storage**: Store only essential data to minimize GAS costs
- **Data Normalization**: Avoid data duplication in storage
- **Key Structure**: Use hierarchical key prefixes for complex data structures
- **Default Values**: Handle missing values gracefully with default values

## Security Considerations

### Access Control

- **Witness Verification**: Always verify transaction signers with `Runtime::check_witness()`
- **Role Management**: Implement proper role management for contracts with multiple administrators
- **Contract Owner**: Consider implementing owner/admin functionality for contract management

```rust
// Example of proper witness verification
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    // Ensure the sender is authorized
    if !Runtime::check_witness(&from) {
        return false;
    }
    
    // Rest of the transfer logic
    // ...
}
```

### Input Validation

- **Bounds Checking**: Validate all numeric inputs for reasonable bounds
- **Address Validation**: Verify addresses are valid before use
- **Zero-Value Handling**: Handle zero values appropriately (e.g., zero transfers)

```rust
// Example of input validation
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    // Check for zero amount
    if amount == 0 {
        return false;
    }
    
    // Check for valid addresses
    if from.is_zero() || to.is_zero() {
        return false;
    }
    
    // Rest of the transfer logic
    // ...
}
```

### Reentrancy Protection

- **State Updates First**: Update internal state before making external calls
- **Mutex Pattern**: Consider using a mutex-like pattern for vulnerable functions
- **State Guards**: Use state variables to prevent reentrant calls

```rust
// Example of reentrancy protection
#[storage]
pub struct Contract {
    locked: StorageItem<bool>,
    // Other storage items...
}

impl Contract {
    pub fn vulnerable_function(&mut self) -> bool {
        // Check if already locked
        if self.locked.get().unwrap_or_default() {
            return false;
        }
        
        // Lock the contract
        self.locked.set(&true);
        
        // Perform sensitive operations
        // ...
        
        // Unlock the contract
        self.locked.set(&false);
        
        true
    }
}
```

### Error Handling

- **Explicit Returns**: Use explicit boolean returns for success/failure
- **Error Messages**: Provide clear error messages for failures
- **Exception Avoidance**: Design to avoid runtime exceptions

## Gas Optimization

### Storage Efficiency

- **Batch Operations**: Batch storage operations when possible
- **Clean Up**: Remove unused storage items to reclaim space
- **Lazy Loading**: Only load data when necessary

### Computation Efficiency

- **Safe Methods**: Mark read-only methods with `#[safe]` attribute
- **Loop Optimization**: Minimize loop operations and consider GAS costs
- **Caching**: Cache frequently used values in memory

### Binary Size

- **Code Reuse**: Reuse code where possible to minimize binary size
- **Dead Code Elimination**: Remove unused functions and variables
- **Optimization Flags**: Use appropriate Rust optimization flags for WASM compilation

## Event Standards

### Event Emission

- **Consistent Naming**: Use descriptive and consistent event names
- **Standard Compliance**: Follow the event patterns defined in NEP standards
- **Proper Runtime Notifications**: Use `Runtime::notify` with correct parameters

```rust
// Recommended NEP-11 transfer event emission
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, token_id: ByteString, amount: Int256) {
    // Create event name as ByteString
    let event_name = ByteString::from("Transfer");
    
    // Create an Array to hold parameters
    let mut event_data = Array::<Any>::new();
    
    // Add parameters as Any values
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Use Any::new() for null values
    }
    
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    event_data.push(Any::from(token_id));
    event_data.push(Any::from(amount));
    
    // Emit the event
    Runtime::notify(&event_name, &event_data);
}
```

- **Standard Events**: Emit standard events for compatibility (e.g., NEP-17 Transfer)
- **Event Structure**: Follow the recommended event structure
- **Null Handling**: Use `Any::new()` for null values in events

```rust
// Example of proper event emission for NEP-17 Transfer
fn emit_transfer(&self, from: Option<Address>, to: Option<Address>, amount: u64) {
    let event_name = ByteString::from("Transfer");
    let mut event_data = Array::<Any>::new();
    
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    event_data.push(Any::from(amount));
    
    Runtime::notify(&event_name, &event_data);
}
```

## Testing

### Unit Testing

- **Test Coverage**: Aim for high test coverage of contract functionality
- **Edge Cases**: Test edge cases and boundary conditions
- **Mock Dependencies**: Use mock implementations for testing

### no_std Compatibility

- **Avoid Standard Library**: Neo N3 contracts must be compatible with `no_std` environments since the Neo VM doesn't support the full Rust standard library
- **Use Alloc Consistently**: Always use `alloc` crate instead of `std` for collections and memory management
  - Use `alloc::vec::Vec` instead of `std::vec::Vec`
  - Use `alloc::string::String` instead of `std::string::String`
  - Use `alloc::collections::BTreeMap` instead of `std::collections::HashMap`
- **Use Core**: Prefer `core` primitives over `std` equivalents
  - Use `core::option::Option` instead of `std::option::Option`
  - Use `core::result::Result` instead of `std::result::Result`
  - Use `core::ptr` instead of `std::ptr`
- **Avoid Println**: Use debug assertions instead of println macros
- **Static Storage**: Handle static mutable storage safely with proper unsafe blocks
- **Variable Naming**: For unused parameters in syscall implementations, prefix with underscore to avoid lint warnings
- **Function Return Values**: Handle unit return values correctly to avoid let-unit warnings

```rust
// Example of no_std compatible code for Neo N3 contracts
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::convert::TryFrom;

// Proper function with no_std compatibility
pub fn process_data(input: Vec<u8>) -> Result<String, &'static str> {
    // Use Vec from alloc
    let mut result = Vec::new();
    for byte in input.iter() {
        result.push(byte + 1);
    }
    
    // Use BTreeMap from alloc instead of HashMap
    let mut map = BTreeMap::new();
    map.insert(1, "value");
    
    // Safe conversion with error handling
    String::from_utf8(result).map_err(|_| "Invalid UTF-8")
}

// Instead of println!
debug_assert!(condition, "message");

// Safe handling of static mutable storage
static mut STORAGE: Option<BTreeMap<Vec<u8>, Vec<u8>>> = None;

pub fn with_storage<F, R>(f: F) -> R
where
    F: FnOnce(&mut BTreeMap<Vec<u8>, Vec<u8>>) -> R,
{
    unsafe {
        // Initialize storage if needed
        if STORAGE.is_none() {
            STORAGE = Some(BTreeMap::new());
        }
        
        // Execute function with mutable reference
        f(STORAGE.as_mut().unwrap())
    }
}
```

### Integration Testing

- **Contract Interactions**: Test interactions between multiple contracts
- **TestNet Deployment**: Test on Neo N3 TestNet before deploying to MainNet
- **Gas Consumption**: Profile GAS costs of operations
- **NeoExpress**: Use NeoExpress for local testing

## Safe vs. Non-Safe Methods

- **Mark Read-Only Methods**: Use the `#[safe]` attribute for methods that don't modify state
- **Manifest Reflection**: Safe methods are represented in the contract manifest with `"safe": true`
- **Gas Optimization**: Safe methods consume less GAS as they don't modify state

```rust
#[safe]
pub fn get_balance(account: &Address) -> u64 {
    // This method only reads state and doesn't modify it
    let key = make_key(PREFIX_BALANCE, account);
    if !Storage::has(&key) {
        return 0;
    }
    Storage::get_int(&key)
}

// Non-safe method (state-modifying)
pub fn transfer(from: &Address, to: &Address, amount: u64) -> bool {
    // Check witness and modify state
    if !Runtime::check_witness(from) {
        return false;
    }
    // ... transfer logic ...
    true
}
```

## Storage Patterns

- **Prefix-Based Keys**: Use consistent prefixes for different data types
- **Serialization**: Use consistent serialization methods for complex types
- **Case Sensitivity**: Consider case sensitivity in key naming
- **FindOptions**: Use appropriate FindOptions for storage queries

```rust
// Example of prefix-based key storage
const PREFIX_TOKEN: &[u8] = b"token";
const PREFIX_ACCOUNT: &[u8] = b"account";
const PREFIX_SUPPLY: &[u8] = b"supply";

// Helper to create consistent keys with prefixes
fn make_key(prefix: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(prefix.len() + 1 + key.len());
    result.extend_from_slice(prefix);
    result.push(b':'); // Use separator for clarity
    result.extend_from_slice(key);
    result
}

// Using FindOptions for prefix-based searches
pub fn get_all_tokens(owner: &Address) -> Vec<TokenData> {
    let prefix = make_key(PREFIX_TOKEN, owner.to_bytes());
    let mut options = FindOptions::default();
    options.add(FindOptions::REMOVE_PREFIX);
    
    Storage::find(&prefix, &options)
        .iter()
        .map(|(key, value)| deserialize_token_data(value))
        .collect()
}
```
- **Deployment Tests**: Test contract deployment and initialization
- **Gas Estimation**: Measure GAS consumption for key operations

## Documentation

### Code Documentation

- **Function Comments**: Document all public functions with clear descriptions
- **Parameter Documentation**: Document parameters and return values
- **Storage Documentation**: Document storage structure and key formats

### External Documentation

- **ABI Documentation**: Document the contract ABI and expected behavior
- **Deployment Guide**: Provide instructions for contract deployment
- **Integration Guide**: Document how to integrate with other contracts or dApps

## Deployment

### Testing Environment

- **TestNet Deployment**: Deploy and test on TestNet before MainNet
- **GAS Estimation**: Estimate GAS costs for contract operations
- **Parameter Verification**: Verify initialization parameters before MainNet deployment

### Versioning

- **Semantic Versioning**: Use semantic versioning for contract updates
- **Changelog**: Maintain a changelog for contract versions
- **Migration Plan**: Document migration plans for contract upgrades

## NEP Standards Compliance

### NEP-17 (Fungible Tokens)

- **Required Methods**: Implement all required methods (`name`, `symbol`, `decimals`, `totalSupply`, `balanceOf`, `transfer`)
- **Event Emissions**: Emit proper `Transfer` events
- **SafeTransfer**: Consider implementing safeguards for transfers

### NEP-11 (Non-Fungible Tokens)

- **Required Methods**: Implement all required methods
- **Metadata Structure**: Follow the standard metadata structure
- **Token Properties**: Properly handle token uniqueness and properties

## Example Best Practices for Token Contract

```rust
#![no_std]

use neo_contract::prelude::*;

#[contract]
mod token {
    use super::*;
    
    const TOKEN_NAME: &str = "Best Practice Token";
    const TOKEN_SYMBOL: &str = "BPT";
    const TOKEN_DECIMALS: u8 = 8;
    const INITIAL_SUPPLY: u64 = 1_000_000_000 * 10u64.pow(TOKEN_DECIMALS as u32);
    
    #[storage]
    pub struct TokenContract {
        // Storage structure with clear naming
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
        // Contract owner for administrative functions
        owner: StorageItem<Address>,
    }
    
    // Contract implementation with comprehensive functionality
    impl TokenContract {
        #[initialize]
        pub fn new() -> Self {
            let mut instance = Self {
                total_supply: StorageItem::new(b"total_supply"),
                balances: StorageMap::new(b"balance"),
                owner: StorageItem::new(b"owner"),
            };
            
            // Set the contract owner
            let sender = Runtime::get_executing_script_hash();
            instance.owner.set(&sender);
            
            // Set initial supply
            instance.total_supply.set(&INITIAL_SUPPLY);
            instance.balances.insert(&sender, &INITIAL_SUPPLY);
            
            // Emit transfer event for initial supply
            instance.emit_transfer(None, Some(sender), INITIAL_SUPPLY);
            
            instance
        }
        
        // Safe methods for reading contract state
        
        #[safe]
        pub fn name(&self) -> String {
            TOKEN_NAME.into()
        }
        
        #[safe]
        pub fn symbol(&self) -> String {
            TOKEN_SYMBOL.into()
        }
        
        #[safe]
        pub fn decimals(&self) -> u8 {
            TOKEN_DECIMALS
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn balance_of(&self, account: &Address) -> u64 {
            if account.is_zero() {
                return 0;
            }
            self.balances.get(account).unwrap_or_default()
        }
        
        // State-modifying methods with proper checks
        
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Input validation
            if amount == 0 || from.is_zero() || to.is_zero() {
                return false;
            }
            
            // Authorization check
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Balance check
            let from_balance = self.balance_of(&from);
            if from_balance < amount {
                return false;
            }
            
            // State updates before external calls (reentrancy protection)
            if amount == from_balance {
                self.balances.remove(&from);
            } else {
                self.balances.insert(&from, &(from_balance - amount));
            }
            
            let to_balance = self.balance_of(&to);
            self.balances.insert(&to, &(to_balance + amount));
            
            // Emit event after state changes
            self.emit_transfer(Some(from), Some(to), amount);
            
            true
        }
        
        // Administrative functions with access control
        
        pub fn mint(&mut self, to: Address, amount: u64) -> bool {
            // Only owner can mint
            let sender = Runtime::get_executing_script_hash();
            if sender != self.owner.get().unwrap() {
                return false;
            }
            
            // Input validation
            if amount == 0 || to.is_zero() {
                return false;
            }
            
            // Update state
            let current_supply = self.total_supply();
            self.total_supply.set(&(current_supply + amount));
            
            let to_balance = self.balance_of(&to);
            self.balances.insert(&to, &(to_balance + amount));
            
            // Emit event
            self.emit_transfer(None, Some(to), amount);
            
            true
        }
        
        // Standardized event emission
        
        fn emit_transfer(&self, from: Option<Address>, to: Option<Address>, amount: u64) {
            let event_name = ByteString::from("Transfer");
            let mut event_data = Array::<Any>::new();
            
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }
            
            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }
            
            event_data.push(Any::from(amount));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
}
