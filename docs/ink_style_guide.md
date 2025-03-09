# ink! Style Guide for Neo Smart Contracts

This guide explains how to write Neo smart contracts using the ink!-style API provided by the Neo Contract Rust Framework. The ink!-style approach offers a clean, attribute-based interface that simplifies contract development.

## Introduction to ink! Style

The ink! programming model, originally developed for Substrate-based blockchains, provides a user-friendly way to write smart contracts using Rust attributes. Neo Contract Rust Framework adopts this style to provide a familiar and expressive API for Rust developers.

Key benefits include:
- Declarative approach with attributes
- Clear separation of storage and logic
- Type-safe storage operations
- Simplified event handling
- Automatic ABI generation

## Basic Structure

An ink!-style Neo contract has this basic structure:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct MyContract {
    // Storage fields
    value: StorageItem<u64>,
    mapping: StorageMap<Address, String>,
}

#[event]
pub struct ValueChanged {
    pub old_value: u64,
    pub new_value: u64,
}

#[contractimpl]
impl MyContract {
    #[constructor]
    pub fn new(initial_value: u64) -> Self {
        Self {
            value: StorageItem::new(initial_value),
            mapping: StorageMap::new(),
        }
    }

    #[method]
    pub fn get_value(&self) -> u64 {
        self.value.get()
    }

    #[method]
    pub fn set_value(&mut self, new_value: u64) {
        let old_value = self.value.get();
        self.value.set(new_value);
        
        emit!(ValueChanged {
            old_value,
            new_value,
        });
    }
}
```

Let's break down each component.

## Contract Definition

The `#[contract]` attribute marks a struct as a Neo smart contract. This struct defines the contract's storage layout:

```rust
#[contract]
pub struct MyContract {
    value: StorageItem<u64>,
    mapping: StorageMap<Address, String>,
}
```

Each field represents a storage area in the contract:
- `StorageItem<T>`: Stores a single value of type T
- `StorageMap<K, V>`: Stores a mapping from keys of type K to values of type V

## Contract Implementation

The `#[contractimpl]` attribute marks an implementation block for the contract:

```rust
#[contractimpl]
impl MyContract {
    // Methods go here
}
```

This is where you define constructors and methods for your contract.

### Constructors

The `#[constructor]` attribute marks a method as a contract constructor:

```rust
#[constructor]
pub fn new(initial_value: u64) -> Self {
    Self {
        value: StorageItem::new(initial_value),
        mapping: StorageMap::new(),
    }
}
```

Constructors:
- Must return `Self`
- Are called once when the contract is deployed
- Initialize the contract's storage
- Can have parameters to allow customization during deployment

You can define multiple constructors with different names and parameters:

```rust
#[constructor]
pub fn default() -> Self {
    Self::new(0)
}

#[constructor]
pub fn with_value(initial_value: u64) -> Self {
    Self::new(initial_value)
}
```

### Methods

The `#[method]` attribute marks a method as callable from outside the contract:

```rust
#[method]
pub fn get_value(&self) -> u64 {
    self.value.get()
}

#[method]
pub fn set_value(&mut self, new_value: u64) {
    let old_value = self.value.get();
    self.value.set(new_value);
    
    emit!(ValueChanged {
        old_value,
        new_value,
    });
}
```

Methods:
- Must be public (`pub`)
- Can take `&self` for read-only access or `&mut self` for state-modifying operations
- Can have additional parameters and return values
- Support most Rust types that can be serialized/deserialized

## Events

Events allow your contract to notify external applications when something happens. Define events with the `#[event]` attribute:

```rust
#[event]
pub struct ValueChanged {
    pub old_value: u64,
    pub new_value: u64,
}
```

Emit events using the `emit!` macro:

```rust
emit!(ValueChanged {
    old_value,
    new_value,
});
```

Event fields:
- Must be public (`pub`)
- Must use types that can be serialized
- Can be referenced in client applications to listen for contract activity

## Storage Primitives

### StorageItem

`StorageItem<T>` provides a type-safe way to store and retrieve a single value:

```rust
// Define in contract struct
value: StorageItem<u64>,

// Initialize in constructor
value: StorageItem::new(initial_value),

// Get value
let current = self.value.get();

// Set value
self.value.set(new_value);
```

### StorageMap

`StorageMap<K, V>` provides a key-value mapping:

```rust
// Define in contract struct
balances: StorageMap<Address, u64>,

// Initialize in constructor
balances: StorageMap::new(),

// Insert or update a value
self.balances.insert(&address, amount);

// Get a value (returns Option<V>)
let balance = self.balances.get(&address).unwrap_or(0);

// Remove a value
self.balances.remove(&address);

// Check if a key exists
if self.balances.contains_key(&address) {
    // ...
}
```

### Composite Storage

For more complex structures, you can nest storage primitives:

```rust
#[contract]
pub struct Marketplace {
    // Map from item ID to Item struct
    items: StorageMap<u64, Item>,
    // Counter for generating unique item IDs
    next_item_id: StorageItem<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    price: u64,
    seller: Address,
}

#[contractimpl]
impl Marketplace {
    #[constructor]
    pub fn new() -> Self {
        Self {
            items: StorageMap::new(),
            next_item_id: StorageItem::new(1),
        }
    }
    
    #[method]
    pub fn list_item(&mut self, name: String, price: u64) -> u64 {
        let seller = runtime::current_sender();
        let item_id = self.next_item_id.get();
        
        // Create and store the item
        let item = Item {
            name,
            price,
            seller,
        };
        
        self.items.insert(&item_id, item);
        
        // Increment the ID counter
        self.next_item_id.set(item_id + 1);
        
        item_id
    }
    
    #[method]
    pub fn get_item(&self, item_id: u64) -> Option<Item> {
        self.items.get(&item_id)
    }
}
```

## Advanced Features

### Safe Methods

The `#[safe]` attribute marks a method as read-only, which has several benefits:

```rust
#[method]
#[safe]
pub fn get_value(&self) -> u64 {
    self.value.get()
}
```

Safe methods:
- Cannot modify contract state
- Have lower gas costs
- Can be called without a transaction
- Are automatically enforced by the runtime

See [Safe Methods Guide](safe_methods.md) for more details.

### Security Attributes

Add security features using attributes:

```rust
// Prevent reentrancy attacks
#[method]
#[no_reentrant]
pub fn transfer(&mut self, to: Address, amount: u64) {
    // ...
}

// Additional security checks
#[method]
#[owner_only]
pub fn admin_function(&mut self) {
    // ...
}
```

### Payable Methods

For methods that accept GAS transfers:

```rust
#[method]
#[payable]
pub fn deposit(&mut self) {
    let sender = runtime::current_sender();
    let amount = runtime::gas_attached();
    
    // Add to sender's balance
    let current_balance = self.balances.get(&sender).unwrap_or(0);
    self.balances.insert(&sender, current_balance + amount);
}
```

## Complete Examples

### NEP-17 Token (Fungible Token)

```rust
use neo_contract::prelude::*;

#[contract]
pub struct Token {
    name: StorageItem<String>,
    symbol: StorageItem<String>,
    decimals: StorageItem<u8>,
    total_supply: StorageItem<u64>,
    balances: StorageMap<Address, u64>,
}

#[event]
pub struct Transfer {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub amount: u64,
}

#[contractimpl]
impl Token {
    #[constructor]
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64) -> Self {
        let sender = runtime::current_sender();
        let mut contract = Self {
            name: StorageItem::new(name),
            symbol: StorageItem::new(symbol),
            decimals: StorageItem::new(decimals),
            total_supply: StorageItem::new(total_supply),
            balances: StorageMap::new(),
        };
        
        // Assign all tokens to the creator
        contract.balances.insert(&sender, total_supply);
        
        // Emit transfer event from null address
        emit!(Transfer {
            from: None,
            to: Some(sender),
            amount: total_supply,
        });
        
        contract
    }
    
    #[method]
    #[safe]
    pub fn name(&self) -> String {
        self.name.get()
    }
    
    #[method]
    #[safe]
    pub fn symbol(&self) -> String {
        self.symbol.get()
    }
    
    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        self.decimals.get()
    }
    
    #[method]
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get()
    }
    
    #[method]
    #[safe]
    pub fn balance_of(&self, account: Address) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    #[method]
    pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
        let from = runtime::current_sender();
        
        assert!(to != Address::zero(), "Cannot transfer to null address");
        
        let from_balance = self.balances.get(&from).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        self.balances.insert(&from, from_balance - amount);
        let to_balance = self.balances.get(&to).unwrap_or(0);
        self.balances.insert(&to, to_balance + amount);
        
        // Emit transfer event
        emit!(Transfer {
            from: Some(from),
            to: Some(to),
            amount,
        });
        
        true
    }
}
```

### NFT Collection (NEP-11)

```rust
use neo_contract::prelude::*;
use std::collections::HashMap;

#[contract]
pub struct NFTCollection {
    name: StorageItem<String>,
    symbol: StorageItem<String>,
    tokens: StorageMap<String, TokenMetadata>,
    owners: StorageMap<String, Address>,
    balances: StorageMap<Address, u64>,
    token_count: StorageItem<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenMetadata {
    name: String,
    description: String,
    image: String,
    properties: HashMap<String, String>,
}

#[event]
pub struct Transfer {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub token_id: String,
}

#[contractimpl]
impl NFTCollection {
    #[constructor]
    pub fn new(name: String, symbol: String) -> Self {
        Self {
            name: StorageItem::new(name),
            symbol: StorageItem::new(symbol),
            tokens: StorageMap::new(),
            owners: StorageMap::new(),
            balances: StorageMap::new(),
            token_count: StorageItem::new(0),
        }
    }
    
    #[method]
    #[safe]
    pub fn name(&self) -> String {
        self.name.get()
    }
    
    #[method]
    #[safe]
    pub fn symbol(&self) -> String {
        self.symbol.get()
    }
    
    #[method]
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.token_count.get()
    }
    
    #[method]
    #[safe]
    pub fn balance_of(&self, owner: Address) -> u64 {
        self.balances.get(&owner).unwrap_or(0)
    }
    
    #[method]
    #[safe]
    pub fn owner_of(&self, token_id: String) -> Option<Address> {
        self.owners.get(&token_id)
    }
    
    #[method]
    #[safe]
    pub fn token_metadata(&self, token_id: String) -> Option<TokenMetadata> {
        self.tokens.get(&token_id)
    }
    
    #[method]
    pub fn mint(&mut self, metadata: TokenMetadata) -> String {
        let sender = runtime::current_sender();
        let token_count = self.token_count.get();
        let token_id = format!("NFT-{}", token_count + 1);
        
        // Store token metadata
        self.tokens.insert(&token_id, metadata);
        self.owners.insert(&token_id, sender);
        
        // Update owner's balance
        let balance = self.balances.get(&sender).unwrap_or(0);
        self.balances.insert(&sender, balance + 1);
        
        // Update token count
        self.token_count.set(token_count + 1);
        
        // Emit transfer event
        emit!(Transfer {
            from: None,
            to: Some(sender),
            token_id: token_id.clone(),
        });
        
        token_id
    }
    
    #[method]
    pub fn transfer(&mut self, to: Address, token_id: String) -> bool {
        let from = runtime::current_sender();
        
        // Verify ownership
        let owner = self.owners.get(&token_id).expect("Token does not exist");
        assert!(owner == from, "Not the token owner");
        assert!(to != Address::zero(), "Cannot transfer to null address");
        
        // Update ownership
        self.owners.insert(&token_id, to);
        
        // Update balances
        let from_balance = self.balances.get(&from).unwrap_or(0);
        self.balances.insert(&from, from_balance - 1);
        
        let to_balance = self.balances.get(&to).unwrap_or(0);
        self.balances.insert(&to, to_balance + 1);
        
        // Emit transfer event
        emit!(Transfer {
            from: Some(from),
            to: Some(to),
            token_id,
        });
        
        true
    }
}
```

## Converting from Module-Style to ink!-Style

If you have existing Neo contracts written in the module style, you can convert them to ink! style:

### Module Style (Old):

```rust
#[contract]
mod my_contract {
    use super::*;
    
    #[storage]
    struct Storage {
        counter: StorageItem<u64>,
    }
    
    impl Storage {
        pub fn new() -> Self {
            Self {
                counter: StorageItem::new(0),
            }
        }
    }
    
    #[methods]
    impl Storage {
        #[method]
        pub fn increment(&mut self) {
            let current = self.counter.get();
            self.counter.set(current + 1);
        }
        
        #[method]
        pub fn get_counter(&self) -> u64 {
            self.counter.get()
        }
    }
}
```

### ink! Style (New):

```rust
use neo_contract::prelude::*;

#[contract]
pub struct MyContract {
    counter: StorageItem<u64>,
}

#[contractimpl]
impl MyContract {
    #[constructor]
    pub fn new() -> Self {
        Self {
            counter: StorageItem::new(0),
        }
    }
    
    #[method]
    pub fn increment(&mut self) {
        let current = self.counter.get();
        self.counter.set(current + 1);
    }
    
    #[method]
    pub fn get_counter(&self) -> u64 {
        self.counter.get()
    }
}
```

## Best Practices

### State Management

1. **Minimize Storage Operations**:
   ```rust
   // Inefficient - multiple get/set operations
   pub fn transfer(&mut self, to: Address, amount: u64) {
       let from_balance = self.balances.get(&from);
       self.balances.set(&from, from_balance - amount);
       
       let to_balance = self.balances.get(&to);
       self.balances.set(&to, to_balance + amount);
   }
   
   // Efficient - load once, store once
   pub fn transfer(&mut self, to: Address, amount: u64) {
       let from = runtime::current_sender();
       let from_balance = self.balances.get(&from).unwrap_or(0);
       let to_balance = self.balances.get(&to).unwrap_or(0);
       
       assert!(from_balance >= amount, "Insufficient balance");
       
       self.balances.insert(&from, from_balance - amount);
       self.balances.insert(&to, to_balance + amount);
   }
   ```

2. **Use Appropriate Storage Types**:
   - `StorageItem` for single values
   - `StorageMap` for key-value mappings
   - Consider custom storage solutions for complex data structures

3. **Initialize Storage in Constructors**:
   ```rust
   #[constructor]
   pub fn new() -> Self {
       Self {
           value: StorageItem::new(0),
           map: StorageMap::new(),
       }
   }
   ```

### Security

1. **Use `#[no_reentrant]` for Methods That Interact with Other Contracts**:
   ```rust
   #[method]
   #[no_reentrant]
   pub fn external_call(&mut self, contract: Address) {
       // This method is protected from reentrancy attacks
       // ...
   }
   ```

2. **Validate Inputs**:
   ```rust
   #[method]
   pub fn withdraw(&mut self, amount: u64) {
       assert!(amount > 0, "Amount must be positive");
       assert!(amount <= self.balance.get(), "Insufficient funds");
       // ...
   }
   ```

3. **Use Access Control**:
   ```rust
   #[method]
   pub fn admin_function(&mut self) {
       let sender = runtime::current_sender();
       assert!(sender == self.owner.get(), "Only owner can call this function");
       // ...
   }
   ```

### Gas Optimization

1. **Mark Read-Only Methods as Safe**:
   ```rust
   #[method]
   #[safe]
   pub fn view_data(&self) -> u64 {
       // ...
   }
   ```

2. **Batch Operations When Possible**:
   ```rust
   // Process multiple updates at once
   #[method]
   pub fn batch_update(&mut self, updates: Vec<(Address, u64)>) {
       for (address, value) in updates {
           self.values.insert(&address, value);
       }
   }
   ```

3. **Avoid Redundant Storage Operations**:
   ```rust
   // Check if an update is needed
   #[method]
   pub fn conditional_update(&mut self, key: String, value: u64) {
       if self.values.get(&key) != Some(value) {
           self.values.insert(&key, value);
       }
   }
   ```

## Testing

Test your ink!-style contracts using Rust's standard testing framework:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constructor() {
        let contract = MyContract::new(42);
        assert_eq!(contract.get_value(), 42);
    }
    
    #[test]
    fn test_set_value() {
        let mut contract = MyContract::new(0);
        contract.set_value(100);
        assert_eq!(contract.get_value(), 100);
    }
}
```

For more advanced testing techniques, see the [Testing Guide](testing_guide.md).

## Integration with Neo VM Specifics

While ink! style provides an abstraction over Neo VM, sometimes you may need to interact with Neo-specific features:

```rust
#[method]
pub fn get_blockchain_info(&self) -> (u64, String) {
    // Get current block height
    let height = runtime::block_height();
    
    // Get transaction hash
    let tx_hash = runtime::transaction_hash().to_string();
    
    (height, tx_hash)
}

#[method]
pub fn call_another_contract(&self, contract_hash: Address) -> u64 {
    // Call a method on another contract
    let result: u64 = runtime::call_contract(
        &contract_hash,
        "someMethod",
        &[],
        CallFlags::All
    );
    
    result
}
```

## Troubleshooting

### Common Issues

1. **Storage Serialization Errors**:
   - Ensure types implement `Serialize` and `Deserialize`
   - Add `#[derive(Serialize, Deserialize)]` to custom types

2. **Method Not Found Errors**:
   - Check that methods are marked with `#[method]`
   - Ensure method names follow the correct naming convention

3. **Security Attribute Errors**:
   - Check that security attributes are applied correctly
   - Ensure dependencies are properly defined

## Conclusion

The ink!-style API provides a powerful and expressive way to write Neo smart contracts in Rust. By using this attribute-based approach, you can create clean, maintainable contracts with strong type safety and efficient storage operations.

For more examples, check out the `examples/` directory in the Neo Contract Rust Framework repository.

Happy contract development!
