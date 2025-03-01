# neo-contract-rs
Writing Neo-Smart-Contract with Rust

## Attribute Macros

Neo Contract RS now supports ink!-style attribute macros for defining smart contracts.
This provides a more unified approach to contract development.

### Example

```rust
#[neo_contract::contract]
#[neo_contract::contract_author("R3E Network")]
#[neo_contract::contract_email("dev@r3e.network")]
#[neo_contract::contract_description("An example token contract")]
#[neo_contract::contract_version("0.1.0")]
#[neo_contract::supported_standards("NEP-17")]
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

### Contract Structure Attributes

- `#[neo_contract::contract]` - Defines a Neo N3 smart contract module
- `#[neo(storage)]` - Marks a struct as the contract's storage
- `#[neo(constructor)]` - Marks a method as a contract constructor
- `#[neo(message)]` - Marks a method as a contract message (callable from outside)
- `#[neo(event)]` - Marks a method as a contract event

### Contract Metadata Attributes

- `#[neo_contract::manifest_extra("key", "value")]` - Adds custom metadata to the contract manifest
- `#[neo_contract::contract_author("Author Name")]` - Specifies the contract author
- `#[neo_contract::contract_email("email@example.com")]` - Specifies the author's email
- `#[neo_contract::contract_description("Description")]` - Provides a contract description
- `#[neo_contract::contract_version("1.0.0")]` - Specifies the contract version
- `#[neo_contract::contract_source_code("https://github.com/...")]` - Links to the source code

### Contract Permission and Standards Attributes

- `#[neo_contract::contract_permission("contract_hash", "method1", "method2")]` - Specifies which contracts and methods can be called
- `#[neo_contract::contract_trust("contract_hash")]` - Specifies which contracts are trusted
- `#[neo_contract::supported_standards("NEP-17", "NEP-11")]` - Declares supported standards

The traditional macro approach is still supported for backward compatibility.

## Examples

Check the `examples` directory for complete contract examples:

- `ink_style_token_with_attributes` - A token contract using the ink!-style attribute macros
- `nep17_token` - A NEP-17 token implementation
- `contract_call` - Example of contract-to-contract calls
