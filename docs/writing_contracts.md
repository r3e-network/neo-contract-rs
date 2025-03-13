# Writing Neo Contracts with Neo Contract Rust

This guide explains how to write Neo N3 smart contracts using the Neo Contract Rust framework. The framework provides an ink!-inspired syntax that leverages Rust's type system and safety features.

## Setup

Before you start, make sure you have:

1. Rust toolchain installed (via [rustup](https://rustup.rs/))
2. WebAssembly target added: `rustup target add wasm32-unknown-unknown`
3. Neo Compiler installed: `cargo install neo-compiler`

## Creating a New Contract Project

Create a new cargo project:

```bash
cargo new --lib my_contract
cd my_contract
```

Add the necessary dependencies to your `Cargo.toml`:

```toml
[package]
name = "my_contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = "0.1.0"
```

## Contract Structure

The basic structure of a Neo contract using our framework is:

```rust
#[neo_contract::contract]
pub mod my_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    pub struct MyContract {
        message: StorageItem<String>,
    }
    
    impl MyContract {
        #[constructor]
        pub fn new() -> Self {
            Self {
                message: StorageItem::new(),
            }
        }
        
        #[method]
        pub fn get_message(&self) -> String {
            self.message.get().unwrap_or_default()
        }
        
        #[method]
        pub fn set_message(&mut self, message: String) {
            self.message.set(message);
        }
    }
}
```

Let's break down the key components:

### 1. Contract Module

The `#[contract]` attribute marks a module as a Neo smart contract:

```rust
#[neo_contract::contract]
pub mod my_contract {
    // Contract code goes here
}
```

### 2. Storage Structure

The `#[storage]` attribute defines the contract's persistent storage:

```rust
#[storage]
pub struct MyContract {
    message: StorageItem<String>,
    balances: StorageMap<Address, u64>,
}
```

### 3. Constructor

The `#[constructor]` attribute marks a function as the contract's constructor, which is called once when the contract is deployed:

```rust
#[constructor]
pub fn new() -> Self {
    Self {
        message: StorageItem::new(),
    }
}
```

### 4. Methods

The `#[method]` attribute marks functions that can be called from outside the contract:

```rust
#[method]
pub fn set_message(&mut self, message: String) {
    self.message.set(message);
}
```

For read-only methods that don't modify state, you can add the `#[safe]` attribute:

```rust
#[method]
#[safe]
pub fn get_message(&self) -> String {
    self.message.get().unwrap_or_default()
}
```

## Storage Types

The framework provides several storage types:

### StorageItem

`StorageItem<T>` stores a single value of type `T`:

```rust
let value: StorageItem<u64> = StorageItem::new();
value.set(42);
let v = value.get().unwrap_or_default();
```

### StorageMap

`StorageMap<K, V>` stores key-value pairs:

```rust
let balances: StorageMap<Address, u64> = StorageMap::new();
balances.insert(&address, 100);
let balance = balances.get(&address).unwrap_or_default();
```

### StorageMap with Custom Keys

For complex keys, you can specify a prefix:

```rust
let votes: StorageMap<(Address, u32), bool> = StorageMap::new_with_prefix(b"votes");
votes.insert(&(voter, proposal_id), true);
```

## Events

Events allow contracts to notify external applications about state changes:

```rust
#[event]
pub struct Transfer {
    #[index]
    pub from: Address,
    #[index]
    pub to: Address,
    pub amount: u64,
}

// Inside a method
emit!(Transfer {
    from: sender,
    to: recipient,
    amount: 100,
});
```

The `#[indexed]` attribute marks fields that can be efficiently queried by external applications.

## Contract Call Example

Contracts can call other contracts:

```rust
#[method]
pub fn call_other_contract(&mut self, token_contract: Address, recipient: Address) -> bool {
    // Create a reference to the token contract
    let token = NEP17Contract::at(token_contract);
    
    // Call the transfer method on the token contract
    token.transfer(runtime::calling_script_hash(), recipient, 100)
}
```

## Common Patterns

### Owner Management

```rust
#[storage]
pub struct OwnedContract {
    owner: StorageItem<Address>,
    // Other fields...
}

impl OwnedContract {
    #[constructor]
    pub fn new() -> Self {
        let mut contract = Self {
            owner: StorageItem::new(),
            // Initialize other fields...
        };
        
        // Set the deployer as the owner
        contract.owner.set(runtime::calling_script_hash());
        
        contract
    }
    
    #[method]
    pub fn transfer_ownership(&mut self, new_owner: Address) -> bool {
        // Only the current owner can transfer ownership
        assert!(self.owner.get().unwrap() == runtime::calling_script_hash(), "Not authorized");
        self.owner.set(new_owner);
        true
    }
    
    // Helper method to check ownership
    fn check_owner(&self) {
        assert!(
            self.owner.get().unwrap() == runtime::calling_script_hash(),
            "Not authorized"
        );
    }
}
```

### Reentrancy Protection

Use the `#[no_reentrant]` attribute to prevent reentrancy attacks:

```rust
#[method]
#[no_reentrant]
pub fn withdraw(&mut self, amount: u64) -> bool {
    let sender = runtime::calling_script_hash();
    let balance = self.balances.get(&sender).unwrap_or_default();
    
    assert!(balance >= amount, "Insufficient balance");
    
    self.balances.insert(&sender, balance - amount);
    
    // External call that could potentially call back into this contract
    // The no_reentrant attribute prevents this from happening
    transfer_neo(sender, amount);
    
    true
}
```

## NEP-17 Token Example

Here's a complete example of a NEP-17 compatible token:

```rust
#[neo_contract::contract]
pub mod token {
    use neo_contract::prelude::*;
    
    #[event]
    pub struct Transfer {
        #[index]
        pub from: Option<Address>,
        #[index]
        pub to: Option<Address>,
        pub amount: u64,
    }
    
    #[storage]
    #[supported_standards("NEP-17")]
    pub struct Token {
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }
    
    impl Token {
        #[constructor]
        pub fn new(total_supply: u64) -> Self {
            let mut token = Self {
                total_supply: StorageItem::new(),
                balances: StorageMap::new(),
            };
            
            let sender = runtime::calling_script_hash();
            token.total_supply.set(total_supply);
            token.balances.insert(&sender, total_supply);
            
            emit!(Transfer {
                from: None,
                to: Some(sender),
                amount: total_supply,
            });
            
            token
        }
        
        #[safe]
        pub fn symbol(&self) -> String {
            "TKN".to_string()
        }
        
        #[safe]
        pub fn decimals(&self) -> u8 {
            8
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn balance_of(&self, owner: Address) -> u64 {
            self.balances.get(&owner).unwrap_or_default()
        }
        
        #[method]
        pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
            let from = runtime::calling_script_hash();
            
            // Check if the sender is sending to themselves
            if from == to {
                // No need to change balances, just emit the event
                emit!(Transfer {
                    from: Some(from),
                    to: Some(to),
                    amount,
                });
                return true;
            }
            
            let from_balance = self.balance_of(from);
            
            // Check if the sender has enough balance
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Update balances
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(&from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.insert(&to, new_to_balance);
            
            // Emit the transfer event
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

## Building and Deploying

### Compilation

Compile your contract to WebAssembly:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Convert the WASM to Neo VM format:

```bash
neo-compiler compile --wasm-file target/wasm32-unknown-unknown/release/my_contract.wasm
```

This will generate two files:
- `my_contract.nef`: The Neo Executable Format file
- `my_contract.manifest.json`: The contract manifest

### Deployment

You can deploy your contract using:

1. **Neo-CLI**: Use the `deploy` command.
2. **Neo-GUI**: Use the deployment interface.
3. **Neo SDK**: Use the deployment API in your code.

Example using Neo-CLI:

```bash
neo-cli deploy my_contract.nef
```

## Debugging Tips

1. **Use the Testing Framework**: Write unit tests using the `neo-contract-testing` framework.
2. **Use Runtime Assertions**: Add `assert!` statements to validate assumptions.
3. **Emit Debug Events**: Use events to log information during contract execution.
4. **Check Return Values**: Always check return values and handle errors.

## Best Practices

1. **Minimize Storage Writes**: Storage operations are expensive, so minimize them.
2. **Use Safe Methods**: Mark read-only methods as `#[safe]` to reduce gas costs.
3. **Validate Inputs**: Always validate inputs to prevent unexpected behavior.
4. **Check Ownership**: Implement access control to protect sensitive operations.
5. **Prevent Reentrancy**: Use the `#[no_reentrant]` attribute for methods that interact with other contracts.
6. **Keep Contracts Small**: Split large contracts into smaller, focused ones.

## Advanced Topics

### Contract Upgrades

```rust
#[method]
pub fn upgrade(&mut self, script: Vec<u8>, manifest: Vec<u8>) -> bool {
    // Only the owner can upgrade the contract
    self.check_owner();
    
    // Call the contract update syscall
    contract::update(script, manifest);
    
    true
}
```

### Storage Versioning

For upgradeable contracts, you might need to migrate storage:

```rust
#[storage]
pub struct VersionedContract {
    version: StorageItem<u8>,
    // V1 fields
    old_data: StorageItem<Vec<u8>>,
    // V2 fields
    new_data: StorageMap<String, Vec<u8>>,
}

impl VersionedContract {
    #[constructor]
    pub fn new() -> Self {
        let mut contract = Self {
            version: StorageItem::new(),
            old_data: StorageItem::new(),
            new_data: StorageMap::new(),
        };
        
        contract.version.set(1);
        
        contract
    }
    
    #[method]
    pub fn migrate_to_v2(&mut self) {
        // Check current version
        let current_version = self.version.get().unwrap_or_default();
        assert!(current_version == 1, "Wrong version");
        
        // Migrate data
        if let Some(old_data) = self.old_data.get() {
            self.new_data.insert(&"migrated".to_string(), old_data);
            self.old_data.remove();
        }
        
        // Update version
        self.version.set(2);
    }
}
```

## Conclusion

This guide covered the basics of writing Neo smart contracts with the Neo Contract Rust framework. Explore the examples directory for more complex contract examples, and check the reference documentation for detailed information about all the available features.

For more information, see:
- [NEP-17 Tutorial](./nep17_tutorial.md)
- [Safe Methods](./safe_methods.md)
- [Deployment Guide](./deployment_guide.md)