# Attributes and Macros in Neo Contract Rust Framework

This guide provides a comprehensive overview of the attributes and macros available in the Neo Contract Rust Framework for building Neo N3 smart contracts in Rust.

## Introduction

The Neo Contract Rust Framework uses a declarative approach through Rust attributes to define smart contract components. This approach is inspired by the ink! framework for Substrate and offers a clean, intuitive way to write smart contracts.

## Core Contract Structure Attributes

### `#[contract]`

The main entry point for defining a Neo smart contract in Rust. It processes a module to extract method definitions, storage structure, events, and generate manifest information.

```rust
#[neo_contract::contract]
pub mod token {
    use neo_contract::prelude::*;
    
    // Contract implementation
}
```

### `#[storage]`

Marks a struct as the contract's storage definition. This struct defines all persistent state for the contract. Only one storage struct can be defined per contract.

```rust
#[storage]
pub struct TokenContract {
    name: StorageItem<String>,
    symbol: StorageItem<String>,
    total_supply: StorageItem<u64>,
    balances: StorageMap<Address, u64>,
}
```

### `#[constructor]`

Marks a function as a constructor for the smart contract. Constructors are called during contract deployment and initialize the contract's storage state. Only one constructor can be defined per contract.

```rust
#[constructor]
pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64, owner: Address) -> Self {
    let mut contract = Self {
        name: StorageItem::new(),
        symbol: StorageItem::new(),
        decimals: StorageItem::new(),
        total_supply: StorageItem::new(),
        balances: StorageMap::new(),
    };
    
    contract.name.set(name);
    contract.symbol.set(symbol);
    contract.decimals.set(decimals);
    contract.total_supply.set(total_supply);
    contract.balances.insert(&owner, total_supply);
    
    contract
}
```

### `#[method]`

Marks a function as a method of the smart contract. Methods marked with this attribute will be exposed in the contract's manifest and can be called from external contracts or applications. These methods can modify contract state.

```rust
#[method]
pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
    // Implementation that modifies state
    true
}
```

### `#[safe]`

Marks a function as a safe method of the smart contract. Safe methods do not modify contract state and are marked as "safe" in the contract manifest. They can be called without requiring a full transaction.

```rust
#[safe]
pub fn balance_of(&self, account: Address) -> u64 {
    self.balances.get(&account).unwrap_or_default()
}
```

## Event Handling Attributes

### `#[event]`

Defines a smart contract event. Events are recorded on the blockchain and can be subscribed to by external services.

```rust
#[event]
pub struct Transfer {
    #[indexed]
    from: Option<Address>,
    #[indexed]
    to: Option<Address>,
    amount: u64,
}
```

### `#[indexed]`

Marks a field in an event struct as indexed. Indexed fields can be used to filter event queries. This is similar to the "topics" concept in Ethereum events.

```rust
#[event]
pub struct ItemListed {
    #[indexed]
    item_id: ByteArray,
    #[indexed]
    seller: Address,
    price: u64,
}
```

## Security Attributes

### `#[no_reentrant]`

Marks a contract method as non-reentrant. Non-reentrant methods cannot be called again while they are still running, which prevents reentrancy attacks.

```rust
#[method]
#[no_reentrant]
pub fn withdraw(&mut self, amount: u64) -> bool {
    // Implementation that's protected from reentrancy
    true
}
```

## Manifest Configuration Attributes

### `#[manifest_extra]`

Adds extra information to the contract manifest. This allows defining additional metadata for the contract.

```rust
#[neo_contract::contract]
#[manifest_extra(
    author = "Neo Project",
    email = "contact@neo.org",
    description = "An example NEP-17 token",
    version = "1.0.0"
)]
pub mod token {
    // Contract implementation
}
```

### `#[supported_standards]`

Specifies the NEP standards supported by the contract.

```rust
#[neo_contract::contract]
#[supported_standards("NEP-17")]
pub mod token {
    // NEP-17 token implementation
}
```

### `#[contract_permission]`

Defines contract permissions. This specifies which contracts and methods the contract is allowed to call.

```rust
#[neo_contract::contract]
#[contract_permission(
    contract = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
    methods = ["transfer", "balanceOf"]
)]
pub mod token {
    // Contract implementation
}
```

### `#[contract_trust]`

Defines contract trust relationships. This specifies which contracts are trusted by this contract.

```rust
#[neo_contract::contract]
#[contract_trust("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
pub mod token {
    // Contract implementation
}
```

## Static Field Attributes

The framework provides several attributes for defining constants and static fields:

### Byte Array Constants

```rust
#[byte_array_fixed("0123456789abcdef")]
pub static FIXED_BYTES: [u8; 8] = [0; 8];

#[byte_array("0123456789abcdef")]
pub static BYTES: Vec<u8> = Vec::new();
```

### Contract Hash Constants

```rust
#[contract_hash_fixed("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
pub static FIXED_CONTRACT_HASH: [u8; 20] = [0; 20];

#[contract_hash("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
pub static CONTRACT_HASH: Vec<u8> = Vec::new();
```

### Hash160 Constants

```rust
#[hash160_fixed("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
pub static FIXED_HASH160: [u8; 20] = [0; 20];

#[hash160("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
pub static HASH160: Vec<u8> = Vec::new();
```

### Integer Constants

```rust
#[integer_fixed(100)]
pub static FIXED_INTEGER: i32 = 0;

#[integer(100)]
pub static INTEGER: i32 = 0;
```

### Public Key Constants

```rust
#[public_key_fixed("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c")]
pub static FIXED_PUBLIC_KEY: [u8; 33] = [0; 33];

#[public_key("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c")]
pub static PUBLIC_KEY: Vec<u8> = Vec::new();
```

### String Constants

```rust
#[string_fixed("Hello, Neo")]
pub static FIXED_STRING: [u8; 10] = [0; 10];

#[string("Hello, Neo")]
pub static STRING: String = String::new();
```

## Combining Attributes

Attributes can be combined in various ways:

```rust
#[method]
#[no_reentrant]
pub fn secure_transfer(&mut self, to: Address, amount: u64) -> bool {
    // Implementation with reentrancy protection
    true
}
```

```rust
#[neo_contract::contract]
#[manifest_extra(
    author = "Neo Developer",
    description = "A decentralized exchange contract",
)]
#[supported_standards("NEP-17")]
#[contract_permission(
    contract = "*",
    methods = ["transfer", "balanceOf"]
)]
pub mod dex {
    // DEX implementation
}
```

## Complete Examples

### Basic Token Contract

```rust
use neo_contract::prelude::*;

#[event]
pub struct Transfer {
    #[indexed]
    from: Option<Address>,
    #[indexed]
    to: Option<Address>,
    amount: u64,
}

#[neo_contract::contract]
#[manifest_extra(
    author = "Neo Project",
    email = "contact@neo.org",
    description = "Basic NEP-17 Token",
    version = "1.0.0"
)]
#[supported_standards("NEP-17")]
pub mod token {
    use super::*;
    
    #[storage]
    pub struct Token {
        name: StorageItem<String>,
        symbol: StorageItem<String>,
        decimals: StorageItem<u8>,
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }
    
    impl Token {
        #[constructor]
        pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64, owner: Address) -> Self {
            let mut this = Self {
                name: StorageItem::new(),
                symbol: StorageItem::new(),
                decimals: StorageItem::new(),
                total_supply: StorageItem::new(),
                balances: StorageMap::new(),
            };
            
            this.name.set(name);
            this.symbol.set(symbol);
            this.decimals.set(decimals);
            this.total_supply.set(total_supply);
            this.balances.insert(&owner, total_supply);
            
            emit!(Transfer {
                from: None,
                to: Some(owner),
                amount: total_supply,
            });
            
            this
        }
        
        #[safe]
        pub fn name(&self) -> String {
            self.name.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn symbol(&self) -> String {
            self.symbol.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn decimals(&self) -> u8 {
            self.decimals.get().unwrap_or(8)
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        #[method]
        pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
            let from = runtime::get_calling_scripthashs().first().unwrap().clone();
            
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            if amount == 0 {
                emit!(Transfer {
                    from: Some(from),
                    to: Some(to),
                    amount: 0,
                });
                return true;
            }
            
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(&from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.insert(&to, new_to_balance);
            
            emit!(Transfer {
                from: Some(from),
                to: Some(to),
                amount,
            });
            
            true
        }
    }
}
```

## Best Practices

1. **Use Safe Methods**: Mark view-only methods with `#[safe]` to allow them to be called without requiring a transaction.

2. **Protect Against Reentrancy**: Use the `#[no_reentrant]` attribute on methods that transfer assets or update critical state.

3. **Organize Events**: Define events at the module level and use the `#[indexed]` attribute for fields that need to be searchable.

4. **Complete Metadata**: Use `#[manifest_extra]` to provide comprehensive information about your contract.

5. **Explicit Standards Support**: Use `#[supported_standards]` to clearly indicate which Neo Enhancement Proposals your contract implements.

6. **Security-First Permissions**: Use `#[contract_permission]` to restrict which contracts your contract can call, following the principle of least privilege.

7. **Document Constants**: For better code readability, document the purpose of static values defined with the constant attributes.

## Conclusion

The attributes and macros provided by the Neo Contract Rust Framework offer a powerful, declarative way to build Neo smart contracts in Rust. By following the patterns outlined in this guide, you can create well-structured, secure, and maintainable contracts that leverage the full capabilities of the Neo N3 blockchain.

These attributes handle much of the boilerplate code that would otherwise be required, allowing you to focus on your contract's core business logic while ensuring proper integration with the Neo blockchain ecosystem.