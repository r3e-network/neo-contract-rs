# NEP-17 Token Tutorial

This tutorial guides you through creating a NEP-17 compliant fungible token using the Neo Contract Rust Framework. NEP-17 is the standard for fungible tokens on the Neo N3 blockchain.

## What is NEP-17?

NEP-17 is Neo's fungible token standard, similar to Ethereum's ERC-20. It defines a set of methods and events that a token contract must implement to be compatible with wallets, exchanges, and other contracts.

### Required Methods:

- `symbol`: Returns the token's symbol (e.g., "NEO", "GAS")
- `decimals`: Returns the number of decimal places (e.g., 8)
- `totalSupply`: Returns the total token supply
- `balanceOf`: Returns the token balance of an account
- `transfer`: Transfers tokens from one account to another

### Required Events:

- `Transfer`: Emitted when tokens are transferred

## Getting Started

Let's create a simple NEP-17 token contract:

### 1. Project Setup

First, create a new Rust project:

```bash
cargo new --lib my_token
cd my_token
```

### 2. Configure Cargo.toml

Edit your `Cargo.toml` file:

```toml
[package]
name = "my_token"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
neo-contract = "0.1.0"
serde = { version = "1.0", features = ["derive"] }

[features]
std = ["neo-contract/std"]
default = ["std"]

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
```

### 3. Basic Token Implementation

Now, let's implement a basic NEP-17 token in `src/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use neo_contract::prelude::*;

#[contract]
pub mod token {
    use super::*;
    
    #[storage]
    pub struct TokenContract {
        // Token metadata
        name: StorageItem<String>,
        symbol: StorageItem<String>,
        decimals: StorageItem<u8>,
        
        // Token data
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }
    
    // Events
    #[event]
    pub struct Transfer {
        #[indexed]
        pub from: Option<Address>,
        
        #[indexed]
        pub to: Option<Address>,
        
        pub amount: u64,
    }
    
    impl TokenContract {
        #[constructor]
        pub fn new(owner: Address, total_supply: u64) -> Self {
            let mut contract = Self {
                name: StorageItem::new("MyToken".to_string()),
                symbol: StorageItem::new("MTK".to_string()),
                decimals: StorageItem::new(8),
                total_supply: StorageItem::new(total_supply),
                balances: StorageMap::new(),
            };
            
            // Initial supply goes to owner
            contract.balances.insert(&owner, total_supply);
            
            // Emit transfer event from null address
            emit!(Transfer {
                from: None,
                to: Some(owner),
                amount: total_supply,
            });
            
            contract
        }
        
        // NEP-17 Methods
        
        #[method]
        pub fn symbol(&self) -> String {
            self.symbol.get()
        }
        
        #[method]
        pub fn name(&self) -> String {
            self.name.get()
        }
        
        #[method]
        pub fn decimals(&self) -> u8 {
            self.decimals.get()
        }
        
        #[method]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get()
        }
        
        #[method]
        pub fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or(0)
        }
        
        #[method]
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Check conditions
            assert!(!to.is_zero(), "Cannot transfer to null address");
            
            if from != runtime::calling_script_hash() {
                assert!(runtime::check_witness(&from), "No authorization");
            }
            
            // Get balances
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            if amount > 0 {
                let to_balance = self.balance_of(to);
                
                // Subtract from sender
                if from_balance == amount {
                    self.balances.remove(&from);
                } else {
                    self.balances.insert(&from, from_balance - amount);
                }
                
                // Add to recipient
                self.balances.insert(&to, to_balance + amount);
            }
            
            // Emit transfer event
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

This implements a basic NEP-17 token with the required methods and events.

## Compiling and Testing

### 1. Build Your Token

```bash
# Debug build
cargo build --target wasm32-unknown-unknown

# Release build (for deployment)
cargo build --target wasm32-unknown-unknown --release
```

### 2. Optimize the WASM (Optional)

```bash
wasm-opt -Oz -o my_token_opt.wasm target/wasm32-unknown-unknown/release/my_token.wasm
```

### 3. Compile to NEO Format

```bash
neo-compiler compile my_token_opt.wasm
```

### 4. Test Your Token

Let's add a basic test to ensure our token works as expected:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_basics() {
        // Create an owner address
        let owner = Address::from_str("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj").unwrap();
        
        // Create the token with 1,000,000 tokens for the owner
        let mut token = token::TokenContract::new(owner, 1_000_000);
        
        // Check token metadata
        assert_eq!(token.name(), "MyToken");
        assert_eq!(token.symbol(), "MTK");
        assert_eq!(token.decimals(), 8);
        
        // Check total supply
        assert_eq!(token.total_supply(), 1_000_000);
        
        // Check owner balance
        assert_eq!(token.balance_of(owner), 1_000_000);
        
        // Create a recipient address
        let recipient = Address::from_str("NVRe7PCm1c6MkUwTVJWEp7KBm9BFhgnjkP").unwrap();
        
        // Test transfer
        let transfer_amount = 50_000;
        
        // Note: In tests, we'd need to mock the runtime::check_witness and runtime::calling_script_hash
        // For simplicity, we'll modify our code to skip these checks during testing
        let result = token.transfer(owner, recipient, transfer_amount);
        
        // Verify transfer succeeded
        assert!(result);
        
        // Check updated balances
        assert_eq!(token.balance_of(owner), 950_000);
        assert_eq!(token.balance_of(recipient), 50_000);
    }
}
```

Run the test with:

```bash
cargo test
```

## Enhanced Implementation

Now, let's enhance our token with more features:

### 1. Adding Approval Functionality

Although not part of the NEP-17 standard, approvals are a common feature in token contracts:

```rust
#[storage]
pub struct TokenContract {
    // ... existing fields
    allowances: StorageMap<(Address, Address), u64>, // (owner, spender) -> amount
}

#[event]
pub struct Approval {
    #[indexed]
    pub owner: Address,
    
    #[indexed]
    pub spender: Address,
    
    pub amount: u64,
}

// Add these methods
#[method]
pub fn approve(&mut self, owner: Address, spender: Address, amount: u64) -> bool {
    assert!(runtime::check_witness(&owner), "No authorization");
    
    self.allowances.insert(&(owner, spender), amount);
    
    emit!(Approval {
        owner,
        spender,
        amount,
    });
    
    true
}

#[method]
pub fn allowance(&self, owner: Address, spender: Address) -> u64 {
    self.allowances.get(&(owner, spender)).unwrap_or(0)
}

#[method]
pub fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: u64) -> bool {
    assert!(!to.is_zero(), "Cannot transfer to null address");
    assert!(runtime::check_witness(&spender), "No authorization");
    
    // Check allowance
    let current_allowance = self.allowance(from, spender);
    if current_allowance < amount {
        return false;
    }
    
    // Check balance
    let from_balance = self.balance_of(from);
    if from_balance < amount {
        return false;
    }
    
    // Update allowance
    self.allowances.insert(&(from, spender), current_allowance - amount);
    
    // Update balances (similar to transfer)
    if amount > 0 {
        let to_balance = self.balance_of(to);
        
        if from_balance == amount {
            self.balances.remove(&from);
        } else {
            self.balances.insert(&from, from_balance - amount);
        }
        
        self.balances.insert(&to, to_balance + amount);
    }
    
    // Emit transfer event
    emit!(Transfer {
        from: Some(from),
        to: Some(to),
        amount,
    });
    
    true
}
```

### 2. Adding Minting and Burning

Add methods to mint new tokens or burn existing ones:

```rust
#[method]
pub fn mint(&mut self, to: Address, amount: u64) -> bool {
    // Only allow the contract owner to mint
    let owner = runtime::current_sender();
    assert!(owner == self.owner.get(), "Only owner can mint");
    assert!(!to.is_zero(), "Cannot mint to null address");
    
    if amount > 0 {
        // Update recipient balance
        let to_balance = self.balance_of(to);
        self.balances.insert(&to, to_balance + amount);
        
        // Update total supply
        let new_supply = self.total_supply() + amount;
        self.total_supply.set(new_supply);
        
        // Emit transfer event from null address
        emit!(Transfer {
            from: None,
            to: Some(to),
            amount,
        });
    }
    
    true
}

#[method]
pub fn burn(&mut self, from: Address, amount: u64) -> bool {
    assert!(runtime::check_witness(&from), "No authorization");
    
    if amount > 0 {
        // Check balance
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        // Update balance
        if from_balance == amount {
            self.balances.remove(&from);
        } else {
            self.balances.insert(&from, from_balance - amount);
        }
        
        // Update total supply
        let new_supply = self.total_supply() - amount;
        self.total_supply.set(new_supply);
        
        // Emit transfer event to null address
        emit!(Transfer {
            from: Some(from),
            to: None,
            amount,
        });
    }
    
    true
}
```

## Using the NEP-17 Attribute

The framework provides a `#[nep17]` attribute to simplify NEP-17 implementation:

```rust
#[contract]
#[nep17]
pub mod token {
    use super::*;
    
    #[storage]
    pub struct TokenContract {
        balances: StorageMap<Address, u64>,
        total_supply: StorageItem<u64>,
    }
    
    impl TokenContract {
        #[constructor]
        pub fn new(owner: Address, total_supply: u64) -> Self {
            let mut contract = Self {
                balances: StorageMap::new(),
                total_supply: StorageItem::new(total_supply),
            };
            
            // Initial supply goes to owner
            contract.balances.insert(&owner, total_supply);
            
            // The nep17 attribute automatically handles events
            contract
        }
        
        // The nep17 attribute requires you to implement these methods
        
        #[method]
        pub fn symbol(&self) -> String {
            "MTK".to_string()
        }
        
        #[method]
        pub fn decimals(&self) -> u8 {
            8
        }
        
        #[method]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get()
        }
        
        #[method]
        pub fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or(0)
        }
        
        // The nep17 attribute will generate the transfer method based on
        // the balances map and total_supply storage items
    }
}
```

## Security Considerations

When implementing NEP-17 tokens, consider these security aspects:

### 1. Prevent Integer Overflow/Underflow

```rust
// Bad: Could overflow
self.balances.insert(&to, to_balance + amount);

// Better: Use checked operations
let new_balance = to_balance.checked_add(amount).expect("Balance overflow");
self.balances.insert(&to, new_balance);
```

### 2. Prevent Reentrancy Attacks

Use the `#[no_reentrant]` attribute to prevent reentrancy:

```rust
#[method(no_reentrant)]
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    // Implementation...
}
```

### 3. Check for Zero Address

```rust
assert!(!to.is_zero(), "Cannot transfer to null address");
```

### 4. Proper Authorization

```rust
assert!(runtime::check_witness(&from), "No authorization");
```

## Advanced NEP-17 Features

### 1. Pausable Token

Add functionality to pause transfers during emergencies:

```rust
#[storage]
pub struct TokenContract {
    // ... existing fields
    paused: StorageItem<bool>,
    owner: StorageItem<Address>,
}

#[method]
pub fn pause(&mut self) -> bool {
    let sender = runtime::current_sender();
    assert!(sender == self.owner.get(), "Only owner can pause");
    
    self.paused.set(true);
    true
}

#[method]
pub fn unpause(&mut self) -> bool {
    let sender = runtime::current_sender();
    assert!(sender == self.owner.get(), "Only owner can unpause");
    
    self.paused.set(false);
    true
}

#[method]
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    assert!(!self.paused.get(), "Token transfers are paused");
    
    // Rest of transfer implementation...
}
```

### 2. Token with Fees

Implement transfer fees:

```rust
#[storage]
pub struct TokenContract {
    // ... existing fields
    fee_percentage: StorageItem<u64>, // In basis points (1/100 of a percent)
    fee_recipient: StorageItem<Address>,
}

#[method]
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    // ... authorization checks
    
    let from_balance = self.balance_of(from);
    if from_balance < amount {
        return false;
    }
    
    // Calculate fee
    let fee_bps = self.fee_percentage.get();
    let fee = amount * fee_bps / 10000; // Convert basis points to actual percentage
    let transfer_amount = amount - fee;
    
    // Update balances
    if amount > 0 {
        let to_balance = self.balance_of(to);
        
        // Subtract from sender
        if from_balance == amount {
            self.balances.remove(&from);
        } else {
            self.balances.insert(&from, from_balance - amount);
        }
        
        // Add to recipient
        self.balances.insert(&to, to_balance + transfer_amount);
        
        // Add fee to fee recipient
        if fee > 0 {
            let fee_recipient = self.fee_recipient.get();
            let recipient_balance = self.balance_of(fee_recipient);
            self.balances.insert(&fee_recipient, recipient_balance + fee);
            
            // Emit fee transfer event
            emit!(Transfer {
                from: Some(from),
                to: Some(fee_recipient),
                amount: fee,
            });
        }
    }
    
    // Emit main transfer event
    emit!(Transfer {
        from: Some(from),
        to: Some(to),
        amount: transfer_amount,
    });
    
    true
}
```

### 3. Token with Timelock

Implement a token that can be locked for a period:

```rust
#[storage]
pub struct TokenContract {
    // ... existing fields
    lock_time: StorageMap<Address, u64>, // Timestamp until which tokens are locked
}

#[method]
pub fn lock_tokens(&mut self, address: Address, until_timestamp: u64) -> bool {
    let sender = runtime::current_sender();
    assert!(sender == self.owner.get(), "Only owner can lock tokens");
    
    self.lock_time.insert(&address, until_timestamp);
    true
}

#[method]
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
    // Check if sender's tokens are locked
    if let Some(lock_until) = self.lock_time.get(&from) {
        let current_time = runtime::time();
        assert!(current_time >= lock_until, "Tokens are still locked");
    }
    
    // Rest of transfer implementation...
}
```

## Interacting with Your Token

### From Neo CLI

Deploy your token:

```bash
neo-cli deploy MyToken.nef MyToken.manifest.json
```

Invoke token methods:

```bash
# Check symbol
neo-cli invokecontract <contract-hash> symbol []

# Check total supply
neo-cli invokecontract <contract-hash> totalSupply []

# Check balance
neo-cli invokecontract <contract-hash> balanceOf [{"type":"Hash160","value":"<address-hash>"}]

# Transfer tokens
neo-cli invokecontract <contract-hash> transfer [{"type":"Hash160","value":"<from-hash>"},{"type":"Hash160","value":"<to-hash>"},{"type":"Integer","value":"1000"}]
```

### From Neo SDK (Java)

```java
// Initialize the SDK
Neow3j neow3j = Neow3j.build(new HttpService("http://localhost:10332"));

// Get the token contract
SmartContract tokenContract = new SmartContract(<contract-script-hash>, neow3j);

// Call read-only methods
String symbol = tokenContract.callFunctionReturningString("symbol");
BigInteger totalSupply = tokenContract.callFunctionReturningInt("totalSupply");

// Check balance
Address address = new Address("<address>");
BigInteger balance = tokenContract.callFunctionReturningInt("balanceOf", 
    ContractParameter.hash160(address));

// Transfer tokens (requires signing)
Hash256 txHash = tokenContract.invokeFunction("transfer",
    ContractParameter.hash160(fromAddress),
    ContractParameter.hash160(toAddress),
    ContractParameter.integer(1000))
    .signers(AccountSigner.calledByEntry(account))
    .sign()
    .send();
```

## Conclusion

This tutorial covered the implementation of NEP-17 tokens using the Neo Contract Rust Framework, from basic compliance to advanced features like approvals, minting, burning, pausing, fees, and timelocks.

Key takeaways:

1. NEP-17 is Neo's fungible token standard
2. The Neo Contract Rust Framework makes it easy to implement compliant tokens
3. The `#[nep17]` attribute simplifies implementation
4. Always consider security aspects like overflow protection and reentrancy
5. Advanced features can be added for specific token requirements

For your next steps:

1. Explore the examples directory for more token implementations
2. Learn about NEP-11 for non-fungible tokens
3. Experiment with token economics and governance features
4. Consider implementing a token with advanced functionality like staking or voting

With the knowledge from this tutorial, you're ready to create your own custom NEP-17 tokens on the Neo N3 blockchain.
