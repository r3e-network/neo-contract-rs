# NEP-17 Token Implementation Guide

This guide explains how to implement a [NEP-17](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki) token using the neo-contract-rs framework. NEP-17 is the Neo N3 fungible token standard, similar to Ethereum's ERC-20.

## NEP-17 Standard Requirements

A NEP-17 token must implement the following:

### Methods

1. **name()**: Returns the name of the token
2. **symbol()**: Returns the symbol of the token
3. **decimals()**: Returns the number of decimals used by the token
4. **totalSupply()**: Returns the total token supply
5. **balanceOf(address)**: Returns the token balance of an address
6. **transfer(from, to, amount)**: Transfers tokens from one address to another

### Events

1. **Transfer(from, to, amount)**: Emitted when tokens are transferred, including minting (from = null) and burning (to = null)

## Implementation Overview

Our implementation uses a manual approach that works with the current framework state without relying on macros:

1. **Token Data Structure**: Defines token parameters and balance storage
2. **Method Implementation**: Implements required NEP-17 methods
3. **Event Handling**: Emits standard events for transfers
4. **Entry Points**: Implements NEO VM entry points for deployment and method dispatch

## Code Structure

### Token Contract State

```rust
pub struct TokenContract {
    // Token info
    name: String,
    symbol: String,
    decimals: u8,
    // Storage
    total_supply: u64,
    balances: Vec<(H160, u64)>, // Simple in-memory storage
    // Admin
    owner: H160,
}
```

### Core Methods

```rust
// Required NEP-17 methods
pub fn name(&self) -> String { self.name.clone() }
pub fn symbol(&self) -> String { self.symbol.clone() }
pub fn decimals(&self) -> u8 { self.decimals }
pub fn total_supply(&self) -> u64 { self.total_supply }
pub fn balance_of(&self, address: H160) -> u64 { self.get_balance(&address) }
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool { /* ... */ }
```

### Event Emission

```rust
fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: u64) {
    let mut event_args = Array::new();
    
    // Handle from address (null for minting)
    match from {
        Some(addr) => event_args.push(Any::from(addr)),
        None => event_args.push(Any::null()),
    }
    
    // Handle to address (null for burning)
    match to {
        Some(addr) => event_args.push(Any::from(addr)),
        None => event_args.push(Any::null()),
    }
    
    event_args.push(Any::integer(amount));
    
    Runtime::notify(&ByteString::from("Transfer"), &event_args);
}
```

### Entry Points

The contract implements two NEO VM entry points:

```rust
#[no_mangle]
pub fn deploying() -> bool {
    true // Initialization logic
}

#[no_mangle]
pub fn invoke(action: String, args: Vec<Any>) -> Any {
    // Method dispatch
    match action.as_str() {
        "name" => { /* ... */ },
        "symbol" => { /* ... */ },
        // ... other methods
    }
}
```

## Special Token Operations

### Minting Tokens

```rust
pub fn mint(&mut self, to: H160, amount: u64) -> bool {
    // Authorization check
    assert!(Runtime::check_witness(&self.owner), "Only owner can mint");
    
    // Update balances
    let to_balance = self.get_balance(&to);
    self.set_balance(to, to_balance + amount);
    
    // Update total supply
    self.total_supply += amount;
    
    // Emit transfer event (mint from null address)
    self.emit_transfer(None, Some(to), amount);
    
    true
}
```

### Burning Tokens

```rust
pub fn burn(&mut self, from: H160, amount: u64) -> bool {
    // Authorization check
    assert!(Runtime::check_witness(&self.owner), "Only owner can burn");
    
    // Update balances
    let from_balance = self.get_balance(&from);
    self.set_balance(from, from_balance - amount);
    
    // Update total supply
    self.total_supply -= amount;
    
    // Emit transfer event (burn to null address)
    self.emit_transfer(Some(from), None, amount);
    
    true
}
```

## Storage Considerations

This example uses in-memory storage for simplicity, but a real contract should use persistent storage:

```rust
// Instead of:
balances: Vec<(H160, u64)>

// Use:
balances: StorageMap<H160, u64>
```

When the framework's storage abstractions are fully functional, they should be used for persistent data storage.

## Security Considerations

1. **Authorization**: Always check that operations are authorized with `Runtime::check_witness`
2. **Balance Checks**: Verify sufficient balances before transfers
3. **Arithmetic Safety**: Prevent overflows in balance calculations
4. **Zero Address**: Handle the zero address appropriately (e.g., rejecting transfers to zero address)
5. **Event Emission**: Always emit events for transfers, especially for mint and burn operations

## Testing

Test your NEP-17 token implementation with various scenarios:

1. Basic transfers
2. Transfers with insufficient balance
3. Minting new tokens
4. Burning existing tokens
5. Authorization checks

## Deployment

To deploy your NEP-17 token:

1. Build with `cargo build --release --target wasm32-unknown-unknown`
2. Compile for NEO VM with `neo-compiler compile target/.../nep17_token.wasm --output build/`
3. Deploy the generated NEF and manifest files to the Neo N3 blockchain

## Advanced Features

Consider adding these advanced features to your token:

1. **Owner Administration**: Methods to change token owner
2. **Pausing**: Ability to pause transfers in emergency situations
3. **Allowances**: Support for delegated transfers (similar to ERC-20 approve/transferFrom)
4. **Token Metadata**: Additional methods for token metadata 