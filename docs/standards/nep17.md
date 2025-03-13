# NEP-17 Token Standard Implementation

This document describes how to implement the NEP-17 Fungible Token Standard for Neo N3 using the Rust contract framework.

## Overview

NEP-17 is the fungible token standard for Neo N3, similar to ERC-20 in Ethereum. It defines a standard interface for fungible tokens, allowing for consistent behavior across different token implementations.

## Required Methods

A NEP-17 compliant token must implement the following methods:

| Method | Parameters | Return Type | Safe | Description |
|--------|------------|-------------|------|-------------|
| `name` | None | String | Yes | Returns the name of the token |
| `symbol` | None | String | Yes | Returns the symbol of the token |
| `decimals` | None | Integer (u8) | Yes | Returns the number of decimals used by the token |
| `totalSupply` | None | Integer | Yes | Returns the total token supply |
| `balanceOf` | account (Hash160) | Integer | Yes | Returns the token balance of the account |
| `transfer` | from (Hash160), to (Hash160), amount (Integer) | Boolean | No | Transfers amount of tokens from from to to |

## Required Events

A NEP-17 compliant token must emit the following event:

| Event | Parameters | Description |
|-------|------------|-------------|
| `Transfer` | from (Hash160\|null), to (Hash160\|null), amount (Integer) | Emitted when tokens are transferred, including mint and burn operations |

## Implementation Example

Here's a complete example of a NEP-17 token implementation using the Neo N3 contract framework in Rust:

```rust
#![no_std]

use neo_contract::prelude::*;

#[contract]
mod token {
    use super::*;
    
    // Token configuration
    const TOKEN_NAME: &str = "Example NEP-17 Token";
    const TOKEN_SYMBOL: &str = "EXT";
    const TOKEN_DECIMALS: u8 = 8;
    const INITIAL_SUPPLY: u64 = 1_000_000_000 * 10u64.pow(TOKEN_DECIMALS as u32);
    
    #[storage]
    pub struct Nep17Token {
        // Storage for token data
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
        // Owner of the contract
        owner: StorageItem<Address>,
    }
    
    impl Nep17Token {
        #[initialize]
        pub fn new() -> Self {
            let mut instance = Self {
                total_supply: StorageItem::new(b"total_supply"),
                balances: StorageMap::new(b"balance"),
                owner: StorageItem::new(b"owner"),
            };
            
            // Set contract owner
            let sender = Runtime::get_executing_script_hash();
            instance.owner.set(&sender);
            
            // Initialize total supply
            instance.total_supply.set(&INITIAL_SUPPLY);
            
            // Assign all tokens to owner
            instance.balances.insert(&sender, &INITIAL_SUPPLY);
            
            // Emit Transfer event for initial supply (mint operation)
            instance.emit_transfer(None, Some(sender), INITIAL_SUPPLY);
            
            instance
        }
        
        // NEP-17 standard methods
        
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
            self.balances.get(account).unwrap_or_default()
        }
        
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Input validation
            if amount == 0 {
                return false;
            }
            
            // Witness verification - ensure the sender authorized this operation
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check if sender has sufficient balance
            let from_balance = self.balance_of(&from);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            if from_balance == amount {
                self.balances.remove(&from);
            } else {
                self.balances.insert(&from, &(from_balance - amount));
            }
            
            let to_balance = self.balance_of(&to);
            self.balances.insert(&to, &(to_balance + amount));
            
            // Emit Transfer event
            self.emit_transfer(Some(from), Some(to), amount);
            
            true
        }
        
        // Additional methods (not required by NEP-17 but commonly implemented)
        
        pub fn mint(&mut self, to: Address, amount: u64) -> bool {
            // Only contract owner can mint
            let sender = Runtime::get_executing_script_hash();
            if sender != self.owner.get().unwrap() {
                return false;
            }
            
            // Update total supply
            let current_supply = self.total_supply();
            self.total_supply.set(&(current_supply + amount));
            
            // Update recipient balance
            let to_balance = self.balance_of(&to);
            self.balances.insert(&to, &(to_balance + amount));
            
            // Emit Transfer event for mint operation
            self.emit_transfer(None, Some(to), amount);
            
            true
        }
        
        pub fn burn(&mut self, from: Address, amount: u64) -> bool {
            // Ensure the token holder authorized this burn
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check balance
            let from_balance = self.balance_of(&from);
            if from_balance < amount {
                return false;
            }
            
            // Update balance
            if from_balance == amount {
                self.balances.remove(&from);
            } else {
                self.balances.insert(&from, &(from_balance - amount));
            }
            
            // Update total supply
            let current_supply = self.total_supply();
            self.total_supply.set(&(current_supply - amount));
            
            // Emit Transfer event for burn operation
            self.emit_transfer(Some(from), None, amount);
            
            true
        }
        
        // Helper method for emitting Transfer events
        fn emit_transfer(&self, from: Option<Address>, to: Option<Address>, amount: u64) {
            // Create event name as ByteString
            let event_name = ByteString::from("Transfer");
            
            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();
            
            // Add parameters as Any values
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),  // For mint operations (from = null)
            }
            
            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),  // For burn operations (to = null)
            }
            
            event_data.push(Any::from(amount));
            
            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }
    }
}
```

## Testing a NEP-17 Token

Here's how to test your NEP-17 token implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract::testing::*;
    
    #[test]
    fn test_nep17_basic_functionality() {
        // Set up test environment
        let mut env = TestEnvironment::new();
        
        // Deploy the token contract
        let token = env.deploy_contract::<token::Nep17Token>();
        
        // Get test accounts
        let owner = env.get_script_hash();
        let user1 = Address::from_public_key(&[1; 33]);
        let user2 = Address::from_public_key(&[2; 33]);
        
        // Test token properties
        assert_eq!(token.name(), "Example NEP-17 Token");
        assert_eq!(token.symbol(), "EXT");
        assert_eq!(token.decimals(), 8);
        
        // Check initial supply
        let initial_supply = 1_000_000_000 * 10u64.pow(8);
        assert_eq!(token.total_supply(), initial_supply);
        
        // Check owner balance
        assert_eq!(token.balance_of(&owner), initial_supply);
        
        // Test transfer
        env.as_signer(owner);
        assert!(token.transfer(owner, user1, 1000));
        
        // Check balances after transfer
        assert_eq!(token.balance_of(&owner), initial_supply - 1000);
        assert_eq!(token.balance_of(&user1), 1000);
        
        // Check events
        let events = env.get_events();
        assert_eq!(events.len(), 2); // Initial mint + transfer
        
        // Check transfer event
        let transfer_event = &events[1];
        assert_eq!(transfer_event.name, "Transfer");
        assert_eq!(transfer_event.params.len(), 3);
        assert_eq!(transfer_event.params[0].as_address(), Some(owner));
        assert_eq!(transfer_event.params[1].as_address(), Some(user1));
        assert_eq!(transfer_event.params[2].as_int(), Some(1000));
    }
}
```

## Deployment and Interaction

After compiling your NEP-17 token contract, you can deploy it to the Neo N3 blockchain:

```bash
# Build the WebAssembly binary
cargo build --target wasm32-unknown-unknown --release

# Convert to Neo VM format
neo-compiler target/wasm32-unknown-unknown/release/my_token.wasm
```

This will generate a NEF file and manifest that can be deployed to a Neo N3 blockchain.

## Common Extensions

Beyond the standard NEP-17 methods, these extensions are commonly implemented:

1. **Ownership and Administration**:
   - `getOwner()`: Get current contract owner
   - `transferOwnership(newOwner)`: Transfer contract ownership
   - `acceptOwnership()`: Accept ownership transfer

2. **Minting and Burning**:
   - `mint(to, amount)`: Create new tokens
   - `burn(from, amount)`: Destroy tokens

3. **Allowances (similar to ERC-20)**:
   - `approve(spender, amount)`: Approve spender to transfer tokens
   - `allowance(owner, spender)`: Get approved amount
   - `transferFrom(from, to, amount)`: Transfer from an approved account

## Best Practices

1. **Security**:
   - Always verify transaction signers with `Runtime::check_witness()`
   - Implement proper access control for admin functions
   - Check for integer overflow/underflow

2. **Gas Optimization**:
   - Mark read-only methods with `#[safe]`
   - Remove unused storage to reclaim GAS
   - Batch operations when possible

3. **Compatibility**:
   - Follow the NEP-17 standard exactly for maximum compatibility
   - Use standard event formats
   - Test against standard NEP-17 interfaces
