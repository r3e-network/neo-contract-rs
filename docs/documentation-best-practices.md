# Documentation Best Practices for NEO Smart Contracts

This guide describes how to write documentation for your NEO smart contracts that will be properly extracted and included in the generated manifest file.

## Contract Documentation

The documentation for your entire contract should be placed as a doc comment above the `impl` block that has the `#[contract]` attribute:

```rust
/// This is a simple NEO token contract that implements the NEP-17 standard.
/// 
/// It provides basic functionality for a fungible token including:
/// - Token transfers between users
/// - Balance queries
/// - Total supply information
#[contract]
impl MyToken {
    // Contract methods go here
}
```

This contract description will be included in the generated manifest file's "Extra" field.

## Method Documentation

Each exported method should have comprehensive documentation that explains:
1. What the method does
2. Parameters and their purpose
3. Return value meaning
4. Any side effects or state changes

Example:

```rust
/// Transfers tokens from the sender's account to the specified receiver.
/// 
/// # Parameters
/// * `from` - The account to transfer tokens from
/// * `to` - The account to transfer tokens to
/// * `amount` - The amount of tokens to transfer
/// 
/// # Returns
/// * `bool` - True if the transfer was successful, false otherwise
pub fn transfer(from: Address, to: Address, amount: u128) -> bool {
    // Implementation here
}
```

## Parameter Naming

Use descriptive parameter names as they will be included in the manifest:

```rust
// Good - descriptive parameter names
pub fn transfer(from: Address, to: Address, amount: u128) -> bool { ... }

// Not as good - unclear parameter names
pub fn transfer(a: Address, b: Address, c: u128) -> bool { ... }
```

## Naming Conventions for Method Safety

The manifest generator automatically marks methods as "safe" (read-only) based on naming conventions. Follow these patterns for your methods:

### Read-only methods (marked as "safe"):
- `get_*` - For retrieving data (e.g., `get_balance`, `get_owner`)
- `query_*` - For querying data (e.g., `query_transactions`)
- `balance_*` - For balance-related queries (e.g., `balance_of`)
- `total_*` - For aggregate data (e.g., `total_supply`)
- `symbol` - For token symbol
- `decimals` - For token decimal places
- `contract_info` - For general contract information

### State-changing methods (marked as "unsafe"):
- `transfer` - For moving assets
- `mint` - For creating new assets
- `burn` - For destroying assets
- `set_*` - For updating state (e.g., `set_owner`)
- `update_*` - For modifying existing data
- `delete_*` - For removing data

## NEP Standard Implementation

If you want your contract to be properly recognized as implementing a standard, include all the required methods:

### For NEP-17 (Fungible Token):
```rust
pub fn symbol() -> String { ... }
pub fn decimals() -> u8 { ... }
pub fn totalSupply() -> u128 { ... }
pub fn balanceOf(account: Address) -> u128 { ... }
pub fn transfer(from: Address, to: Address, amount: u128, data: ByteString) -> bool { ... }
```

### For NEP-11 (Non-Fungible Token):
```rust
pub fn ownerOf(tokenId: ByteString) -> Address { ... }
pub fn tokens() -> Iterator { ... }
pub fn balanceOf(owner: Address) -> u128 { ... }
pub fn transfer(from: Address, to: Address, tokenId: ByteString, data: ByteString) -> bool { ... }
```

## Complete Example

Here's a complete example of a well-documented NEP-17 token contract:

```rust
use neo_contract::prelude::*;

/// SimpleToken - A basic NEP-17 compatible token on Neo N3
/// 
/// This token implements the full NEP-17 standard with the following features:
/// - Fixed supply of 1,000,000 tokens
/// - 8 decimal places
/// - Transfer functionality with optional data parameter
/// - Balance queries for any account
#[contract]
impl SimpleToken {
    /// Returns the token symbol
    ///
    /// # Returns
    /// * String - The token symbol "SIMPLE"
    pub fn symbol() -> String {
        "SIMPLE".to_string()
    }
    
    /// Returns the number of decimal places used by the token
    ///
    /// # Returns
    /// * u8 - The number of decimal places (8)
    pub fn decimals() -> u8 {
        8
    }
    
    /// Returns the total supply of tokens
    ///
    /// # Returns
    /// * u128 - The total token supply (1,000,000 * 10^8)
    pub fn totalSupply() -> u128 {
        1_000_000_00000000
    }
    
    /// Returns the token balance for the specified account
    ///
    /// # Parameters
    /// * `account` - The account address to check balance for
    ///
    /// # Returns
    /// * u128 - The account's token balance
    pub fn balanceOf(account: Address) -> u128 {
        // Implementation here
    }
    
    /// Transfers tokens from one account to another
    ///
    /// # Parameters
    /// * `from` - The account to transfer tokens from
    /// * `to` - The account to transfer tokens to
    /// * `amount` - The amount of tokens to transfer
    /// * `data` - Optional data to include with the transfer
    ///
    /// # Returns
    /// * bool - True if the transfer was successful, false otherwise
    pub fn transfer(from: Address, to: Address, amount: u128, data: ByteString) -> bool {
        // Implementation here
    }
}
```

With this documentation style, the manifest generator will create a rich and informative manifest file that accurately describes your contract's functionality.