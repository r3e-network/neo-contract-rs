# Neo N3 NEP-17 Token Standard Implementation Guide

This guide provides a comprehensive overview of implementing the NEP-17 fungible token standard for Neo N3 using the neo-contract-rs framework.

## Overview

NEP-17 is the fungible token standard for Neo N3, replacing the previous NEP-5 standard used in Neo Legacy. It defines a set of methods and events that a compliant token contract must implement to ensure interoperability with wallets, exchanges, and other smart contracts.

## Required Methods

A NEP-17 compliant token must implement the following methods:

### 1. Symbol

Returns the token's symbol.

```rust
#[safe]
fn symbol(&self) -> ByteString {
    self.symbol.get().unwrap()
}
```

### 2. Decimals

Returns the number of decimal places the token uses.

```rust
#[safe]
fn decimals(&self) -> u8 {
    self.decimals.get().unwrap()
}
```

### 3. TotalSupply

Returns the total token supply.

```rust
#[safe]
fn total_supply(&self) -> u64 {
    self.total_supply.get().unwrap()
}
```

### 4. BalanceOf

Returns the token balance of a specific address.

```rust
#[safe]
fn balance_of(&self, account: H160) -> u64 {
    self.balances.get(&account).unwrap_or_default()
}
```

### 5. Transfer

Transfers tokens from one address to another.

```rust
#[method]
fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Verify the transaction sender is authorized
    assert!(Runtime::check_witness(&from), "No authorization");
    
    // Check for zero address
    assert!(!to.is_zero(), "Cannot transfer to zero address");
    
    // Check if amount is greater than 0
    if amount == 0 {
        return true;
    }
    
    // Check if from has sufficient balance
    let from_balance = self.balances.get(&from).unwrap_or_default();
    assert!(from_balance >= amount, "Insufficient balance");
    
    // Update balances
    if from_balance == amount {
        self.balances.remove(&from);
    } else {
        self.balances.insert(from, from_balance - amount);
    }
    
    let to_balance = self.balances.get(&to).unwrap_or_default();
    self.balances.insert(to, to_balance + amount);
    
    // Emit the transfer event
    Transfer::emit(Some(from), Some(to), amount);
    
    true
}
```

## Required Events

### Transfer Event

A compliant NEP-17 token must emit a `Transfer` event when tokens are transferred, including when tokens are created (from is null) or destroyed (to is null).

```rust
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters, handling None values correctly
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        event_data.push(Any::from(amount));
        
        // Emit event using Neo N3 Runtime::notify
        Runtime::notify(&event_name, &event_data);
    }
}
```

## Complete Implementation

Here's a complete implementation of a NEP-17 token contract with all required methods and events:

```rust
use neo_contract::prelude::*;

#[storage]
pub struct TokenContract {
    name: StorageItem<ByteString>,
    symbol: StorageItem<ByteString>,
    decimals: StorageItem<u8>,
    total_supply: StorageItem<u64>,
    balances: StorageMap<H160, u64>,
    owner: StorageItem<H160>,
}

// Transfer event structure
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // Proper Neo N3 null representation
        }
        
        event_data.push(Any::from(amount));
        
        Runtime::notify(&event_name, &event_data);
    }
}

impl TokenContract {
    #[constructor]
    pub fn new(owner: H160) -> Self {
        let mut this = Self {
            name: StorageItem::new(ByteString::from("Example Token")),
            symbol: StorageItem::new(ByteString::from("EXT")),
            decimals: StorageItem::new(8),
            total_supply: StorageItem::new(0),
            balances: StorageMap::new(),
            owner: StorageItem::new(owner),
        };
        
        // Initial mint to owner
        this.mint(owner, 100_000_000 * 100_000_000); // 100M tokens with 8 decimals
        
        this
    }
    
    // NEP-17 Methods
    
    #[safe]
    pub fn symbol(&self) -> ByteString {
        self.symbol.get().unwrap()
    }
    
    #[safe]
    pub fn name(&self) -> ByteString {
        self.name.get().unwrap()
    }
    
    #[safe]
    pub fn decimals(&self) -> u8 {
        self.decimals.get().unwrap()
    }
    
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap()
    }
    
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or_default()
    }
    
    #[method]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Verify the transaction sender is authorized
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Check for zero address
        assert!(!to.is_zero(), "Cannot transfer to zero address");
        
        // Check if amount is greater than 0
        if amount == 0 {
            return true;
        }
        
        // Check if from has sufficient balance
        let from_balance = self.balances.get(&from).unwrap_or_default();
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        if from_balance == amount {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, from_balance - amount);
        }
        
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(to, to_balance + amount);
        
        // Emit the transfer event
        Transfer::emit(Some(from), Some(to), amount);
        
        true
    }
    
    // Additional Administrative Methods
    
    #[method]
    pub fn mint(&mut self, to: H160, amount: u64) -> bool {
        // Only owner can mint
        let owner = self.owner.get().unwrap();
        assert!(Runtime::check_witness(&owner), "Only owner can mint");
        
        // Check for zero address
        assert!(!to.is_zero(), "Cannot mint to zero address");
        
        // Update total supply
        let current_supply = self.total_supply.get().unwrap();
        self.total_supply.set(current_supply + amount);
        
        // Update recipient balance
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(to, to_balance + amount);
        
        // Emit transfer event (from = null indicates new tokens were created)
        Transfer::emit(None, Some(to), amount);
        
        true
    }
    
    #[method]
    pub fn burn(&mut self, from: H160, amount: u64) -> bool {
        // Verify the transaction sender is authorized
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Check if from has sufficient balance
        let from_balance = self.balances.get(&from).unwrap_or_default();
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        if from_balance == amount {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, from_balance - amount);
        }
        
        // Update total supply
        let current_supply = self.total_supply.get().unwrap();
        self.total_supply.set(current_supply - amount);
        
        // Emit transfer event (to = null indicates tokens were destroyed)
        Transfer::emit(Some(from), None, amount);
        
        true
    }
}
```

## Best Practices for NEP-17 Implementation

### 1. Safe Methods

Mark all read-only methods with `#[safe]` to optimize gas costs and prevent state changes:

```rust
#[safe]
fn balance_of(&self, account: H160) -> u64 {
    // Implementation
}
```

### 2. Security Checks

Always verify transaction authorization:

```rust
assert!(Runtime::check_witness(&from), "No authorization");
```

### 3. Event Emission

Ensure proper event emission using Neo N3 standards:

```rust
// For token minting (from = null)
Transfer::emit(None, Some(recipient), amount);

// For token burning (to = null)
Transfer::emit(Some(holder), None, amount);

// For token transfers
Transfer::emit(Some(sender), Some(recipient), amount);
```

### 4. Zero Value Transfers

Handle zero-value transfers appropriately:

```rust
if amount == 0 {
    return true; // Early return for zero amount transfers
}
```

### 5. Integer Overflow/Underflow Protection

Always check for integer overflow and underflow:

```rust
// Check for overflow when adding to balance
assert!(u64::MAX - to_balance >= amount, "Balance overflow");

// Check for underflow when removing from balance
assert!(from_balance >= amount, "Insufficient balance");
```

### 6. Storage Efficiency

Remove zero balances from storage instead of storing zeros:

```rust
if new_balance == 0 {
    self.balances.remove(&account);
} else {
    self.balances.insert(account, new_balance);
}
```

## Deployment and Testing

### Building for Neo N3

```bash
# Build WebAssembly binary
cargo build --target wasm32-unknown-unknown --release

# Compile to Neo N3 bytecode
neo-compiler compile target/wasm32-unknown-unknown/release/nep17_token.wasm --output ./build
```

### Testing

Test your NEP-17 token implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract_testing::prelude::*;
    
    #[test]
    fn test_nep17_compliance() {
        // Set up test environment
        let mut context = TestingContext::new();
        
        // Create test accounts
        let owner = H160::from([1u8; 20]);
        let user1 = H160::from([2u8; 20]);
        
        // Deploy contract
        context.set_caller(owner);
        let mut contract = TokenContract::new(owner);
        
        // Test basic properties
        assert_eq!(contract.symbol(), ByteString::from("EXT"));
        assert_eq!(contract.decimals(), 8);
        
        // Test transfer
        assert!(contract.transfer(owner, user1, 1000));
        assert_eq!(contract.balance_of(user1), 1000);
        
        // Test zero-value transfer
        assert!(contract.transfer(owner, user1, 0));
        
        // Test invalid transfer (insufficient balance)
        context.set_caller(user1);
        let result = std::panic::catch_unwind(|| {
            contract.transfer(user1, owner, 2000)
        });
        assert!(result.is_err());
    }
}
```

## Integration with Neo N3 Ecosystem

NEP-17 tokens can interact with other components of the Neo N3 ecosystem:

### DeFi Applications

```rust
#[method]
pub fn deposit_to_defi(&mut self, user: H160, defi_contract: H160, amount: u64) -> bool {
    // Verify authorization
    assert!(Runtime::check_witness(&user), "No authorization");
    
    // Transfer tokens to DeFi contract
    self.transfer(user, defi_contract, amount);
    
    // Notify DeFi contract about deposit
    let args = vec![
        Any::from(user),
        Any::from(amount),
    ];
    
    contract::call(
        &defi_contract,
        "deposit_callback",
        &args,
        CallFlags::All
    );
    
    true
}
```

### Token Swaps

```rust
#[method]
pub fn exchange_tokens(&mut self, user: H160, other_token: H160, amount: u64) -> bool {
    // Verify authorization
    assert!(Runtime::check_witness(&user), "No authorization");
    
    // Calculate exchange rate (simplified example)
    let exchange_rate = 2; // 1:2 ratio
    let other_amount = amount * exchange_rate;
    
    // Burn our tokens
    self.burn(user, amount);
    
    // Call other token to mint
    let args = vec![
        Any::from(user),
        Any::from(other_amount),
    ];
    
    contract::call(
        &other_token,
        "mint",
        &args,
        CallFlags::All
    );
    
    true
}
```

## Conclusion

This guide provides a comprehensive overview of implementing NEP-17 tokens for Neo N3 using the neo-contract-rs framework. By following these standards and best practices, you'll ensure your token is compatible with wallets, exchanges, and other smart contracts in the Neo N3 ecosystem.

For more information, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Runtime Guide](./neo_n3_runtime_guide.md)
- [Neo N3 Storage Guide](./neo_n3_storage_guide.md)
- [Neo N3 Security Guide](./neo_n3_security_guide.md)
