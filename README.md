# neo-contract-rs
Writing Neo-Smart-Contract with Rust

## Attribute Macros

Neo Contract RS now supports ink!-style attribute macros for defining smart contracts.
This provides a more unified approach to contract development.

### Example

```rust
#[neo_contract::contract]
mod my_contract {
    #[neo(storage)]
    pub struct MyContract {
        value: bool,
    }
    
    impl MyContract {
        #[neo(constructor)]
        pub fn new(initial_value: bool) -> Self {
            Self { value: initial_value }
        }
        
        #[neo(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        
        #[neo(event)]
        pub fn value_changed(old_value: bool, new_value: bool) {}
    }
}
```

### Attributes

- `#[neo_contract::contract]` - Defines a Neo N3 smart contract module
- `#[neo(storage)]` - Marks a struct as the contract's storage
- `#[neo(constructor)]` - Marks a method as a contract constructor
- `#[neo(message)]` - Marks a method as a contract message (callable from outside)
- `#[neo(event)]` - Marks a method as a contract event

The traditional macro approach is still supported for backward compatibility.
