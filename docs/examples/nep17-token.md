# NEP-17 Token Example

This document provides a detailed explanation of the NEP-17 token example included in the Neo Contract Rust Framework. The NEP-17 standard is Neo's token standard, similar to Ethereum's ERC-20 standard, providing a common interface for fungible tokens.

## Overview

The NEP-17 example demonstrates how to implement a standard-compliant token contract using the Neo Contract Rust Framework. It includes all required methods and events defined in the NEP-17 standard.

## NEP-17 Standard Requirements

A NEP-17 token must implement the following methods:

1. **symbol**: Returns the token's symbol (e.g., "NEO", "GAS").
2. **decimals**: Returns the number of decimal places used by the token.
3. **totalSupply**: Returns the total token supply.
4. **balanceOf**: Returns the token balance of a specific account.
5. **transfer**: Transfers tokens from one account to another.

Additionally, it must define and emit a `Transfer` event for token transfers.

## Implementation

### Basic Structure

The example provides two implementation approaches:
1. A manually created script using the `neo-compiler` API
2. A Rust implementation using the `neo-contract` API

### Manual Implementation

The manual implementation in `neo-compiler/examples/nep17_token.rs` demonstrates how to create a NEP-17 token by directly generating Neo VM instructions.

Key components:

```rust
fn create_nep17_script() -> Result<Script> {
    let mut script = Script::new();
    
    // Method dispatch logic
    // ...
    
    // Method implementations
    // 'name' implementation
    // ...
    
    // 'symbol' implementation
    // ...
    
    // 'decimals' implementation
    // ...
    
    // 'totalSupply' implementation
    // ...
    
    // 'balanceOf' implementation
    // ...
    
    // 'transfer' implementation
    // ...
    
    Ok(script)
}
```

### Rust Implementation

The Rust implementation in `examples/nep17_token/src/lib.rs` uses the higher-level `neo-contract` API and macros.

```rust
use neo_contract::prelude::*;
use neo_macros::contract;

#[contract]
mod token {
    use neo_contract::prelude::*;
    
    const TOKEN_NAME: &str = "Example Token";
    const TOKEN_SYMBOL: &str = "EXT";
    const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY: u64 = 100_000_000 * 100_000_000; // 100M tokens
    
    const TOTAL_SUPPLY_KEY: &[u8] = b"totalSupply";
    const BALANCE_PREFIX: &[u8] = b"balance:";
    
    #[event]
    struct Transfer {
        from: Option<Address>,
        to: Option<Address>,
        amount: u64,
    }
    
    #[storage]
    struct TokenStorage {
        total_supply: u64,
        balances: Map<Address, u64>,
    }
    
    // Implementation of required methods...
}
```

## Contract Methods

Let's examine each required method in detail:

### symbol

Returns the token's symbol as a string:

```rust
#[method]
fn symbol() -> String {
    TOKEN_SYMBOL.to_string()
}
```

Manual implementation:
```rust
// 'symbol' method implementation
script.emit_comment("'symbol' method implementation");
script.emit_with_operand(OpCode::PUSHDATA1, b"symbol_method".to_vec());
script.emit_push_data(b"NET").expect("Failed to emit token symbol");
script.emit_opcode(OpCode::RET);
```

### decimals

Returns the number of decimal places used by the token:

```rust
#[method]
fn decimals() -> u8 {
    TOKEN_DECIMALS
}
```

Manual implementation:
```rust
// 'decimals' method implementation
script.emit_comment("'decimals' method implementation");
script.emit_with_operand(OpCode::PUSHDATA1, b"decimals_method".to_vec());
script.emit_push_data(&[8]).expect("Failed to emit decimals");
script.emit_opcode(OpCode::RET);
```

### totalSupply

Returns the total token supply:

```rust
#[method]
fn total_supply() -> u64 {
    let storage = Storage::new();
    storage.total_supply.get().unwrap_or_default()
}
```

Manual implementation:
```rust
// 'totalSupply' method implementation
script.emit_comment("'totalSupply' method implementation");
script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_method".to_vec());

// Get total supply from storage
let total_supply_key_bytes = b"totalSupply";
script.emit_push_data(total_supply_key_bytes).expect("Failed to emit storage key");
let storage_get_syscall = b"System.Storage.Get".to_vec();
script.emit_with_operand(OpCode::SYSCALL, storage_get_syscall);

// Default total supply if not found
script.emit_opcode(OpCode::DUP);
script.emit_opcode(OpCode::ISNULL);
script.emit_opcode(OpCode::JMPIF);
script.emit_push_data(b"totalSupply_default").expect("Failed to emit jump label");

// Return the stored total supply
script.emit_opcode(OpCode::RET);

// Default total supply
script.emit_with_operand(OpCode::PUSHDATA1, b"totalSupply_default".to_vec());
let total_supply: i32 = 100000000 * 100000000; // 100M tokens with 8 decimals
let total_supply_bytes = total_supply.to_le_bytes();
script.emit_push_data(&total_supply_bytes).expect("Failed to emit total supply");
script.emit_opcode(OpCode::RET);
```

### balanceOf

Returns the token balance of a specific account:

```rust
#[method]
fn balance_of(account: Address) -> u64 {
    let storage = Storage::new();
    storage.balances.get(&account).unwrap_or_default()
}
```

Manual implementation:
```rust
// 'balanceOf' method implementation
script.emit_comment("'balanceOf' method implementation");
script.emit_with_operand(OpCode::PUSHDATA1, b"balanceOf_method".to_vec());

// Get account parameter
script.emit_opcode(OpCode::LDARG1);

// Create storage key: account_balance_{account}
let account_balance_prefix = b"account_balance_";
script.emit_push_data(account_balance_prefix).expect("Failed to emit storage prefix");
script.emit_opcode(OpCode::SWAP);
script.emit_opcode(OpCode::CAT);

// Get balance from storage
let storage_get_syscall = b"System.Storage.Get".to_vec();
script.emit_with_operand(OpCode::SYSCALL, storage_get_syscall);

// Return zero if nothing found
script.emit_opcode(OpCode::DUP);
script.emit_opcode(OpCode::ISNULL);
script.emit_opcode(OpCode::JMPIF);
script.emit_push_data(b"balance_zero").expect("Failed to emit jump label");

// Convert stored value to integer
let storage_get_int_syscall = b"System.Storage.GetInt".to_vec();
script.emit_with_operand(OpCode::SYSCALL, storage_get_int_syscall);
script.emit_opcode(OpCode::RET);

// Return zero balance
script.emit_with_operand(OpCode::PUSHDATA1, b"balance_zero".to_vec());
script.emit_push_data(&[0]).expect("Failed to emit zero");
script.emit_opcode(OpCode::RET);
```

### transfer

Transfers tokens from one account to another:

```rust
#[method]
fn transfer(from: Address, to: Address, amount: u64) -> bool {
    // Verify caller is authorized
    if !runtime::check_witness(&from) {
        return false;
    }
    
    // Check amount > 0
    if amount == 0 {
        return false;
    }
    
    // Get current balances
    let storage = Storage::new();
    let from_balance = storage.balances.get(&from).unwrap_or_default();
    
    // Check if enough balance
    if from_balance < amount {
        return false;
    }
    
    // Update balances
    let to_balance = storage.balances.get(&to).unwrap_or_default();
    
    // Handle edge cases
    if from == to {
        return true;
    }
    
    if from_balance == amount {
        storage.balances.delete(&from);
    } else {
        storage.balances.put(&from, from_balance - amount);
    }
    
    storage.balances.put(&to, to_balance + amount);
    
    // Emit transfer event
    let transfer_event = Transfer {
        from: Some(from),
        to: Some(to),
        amount,
    };
    runtime::emit_event(transfer_event);
    
    true
}
```

Manual implementation (simplified):
```rust
// 'transfer' method implementation
script.emit_comment("'transfer' method implementation");
script.emit_with_operand(OpCode::PUSHDATA1, b"transfer_method".to_vec());

// Get parameters
script.emit_opcode(OpCode::LDARG1); // from
script.emit_opcode(OpCode::LDARG2); // to
script.emit_opcode(OpCode::LDARG3); // amount

// Check amount > 0
script.emit_opcode(OpCode::DUP);
script.emit_push_data(&[0]).expect("Failed to emit zero for comparison");
script.emit_opcode(OpCode::GT);
// ... more transfer logic ...

// Emit Transfer event
let amount: i64 = 1000;
emit_transfer_event(&mut script, from_addr, to_addr, amount).expect("Failed to emit transfer event");

// Return success
script.emit_push_data(&[1]).expect("Failed to emit true");
script.emit_opcode(OpCode::RET);
```

## Contract Manifest

The NEP-17 example also includes a contract manifest defining the contract's methods, events, and permissions:

```rust
fn create_nep17_manifest() -> Result<Manifest> {
    let mut manifest = Manifest::new("Neo Example Token");
    
    // Add methods
    let name_method = ContractMethodDefinition::new(
        "name".to_string(),
        vec![],
        "String".to_string(),
        true, // safe (read-only)
    );
    manifest.abi.add_method(name_method);
    
    // ... more method definitions ...
    
    // Add Transfer event
    let transfer_event = ContractEventDefinition::new(
        "Transfer".to_string(),
        vec![
            ContractParameterDefinition::new("from".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("to".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("amount".to_string(), "Integer".to_string()),
        ],
    );
    manifest.abi.add_event(transfer_event);
    
    // Set features
    manifest.set_feature("storage", true).expect("Failed to set storage feature");
    manifest.set_feature("payable", false).expect("Failed to set payable feature");
    
    // Add supported standards
    manifest.add_supported_standard("NEP-17").expect("Failed to add standard");
    
    Ok(manifest)
}
```

## Building and Deploying

To build and deploy the NEP-17 token example:

### Manual Script Approach

```bash
# Build the manual script example
cargo run --example nep17_token

# This generates:
# - output/nep17_token.neo (Neo VM script)
# - output/nep17_token.nef (NEF file)
# - output/nep17_token.manifest.json (Manifest file)
```

### Rust Implementation Approach

```bash
# Build the Rust contract
cd examples/nep17_token
cargo build --target wasm32-unknown-unknown --release

# Convert WASM to NEF
neo-compiler -i target/wasm32-unknown-unknown/release/nep17_token.wasm \
             -o nep17_token.nef \
             -m nep17_token.manifest.json \
             --name "NEP17 Example Token"
```

## Interacting with the Contract

After deployment, you can interact with the NEP-17 token contract using any Neo N3 client:

### Using neo-cli

```bash
# Invoke the symbol method
neo> invoke 0x1234567890abcdef1234567890abcdef12345678 symbol []

# Check the token balance
neo> invoke 0x1234567890abcdef1234567890abcdef12345678 balanceOf [{"type":"Hash160","value":"0x0000000000000000000000000000000000000001"}]

# Transfer tokens
neo> invoke 0x1234567890abcdef1234567890abcdef12345678 transfer [
    {"type":"Hash160","value":"0x0000000000000000000000000000000000000001"},
    {"type":"Hash160","value":"0x0000000000000000000000000000000000000002"},
    {"type":"Integer","value":"1000000000"}
]
```

### Using neo-sdk

```typescript
// Connect to the Neo blockchain
const neosdk = new Neon.api.NeoSDK({
    network: 'testnet',
    privateKey: 'KxDgvEKzgSBPPfuVfw67oPQBSjidEiqTHURKSDL1R7yGaGYAeYnr'
});

// Get the token symbol
const symbol = await neosdk.invokeFunction({
    scriptHash: '0x1234567890abcdef1234567890abcdef12345678',
    operation: 'symbol',
    args: []
});

// Get a token balance
const balance = await neosdk.invokeFunction({
    scriptHash: '0x1234567890abcdef1234567890abcdef12345678',
    operation: 'balanceOf',
    args: [
        { type: 'Hash160', value: '0x0000000000000000000000000000000000000001' }
    ]
});

// Transfer tokens
const result = await neosdk.invokeFunction({
    scriptHash: '0x1234567890abcdef1234567890abcdef12345678',
    operation: 'transfer',
    args: [
        { type: 'Hash160', value: '0x0000000000000000000000000000000000000001' },
        { type: 'Hash160', value: '0x0000000000000000000000000000000000000002' },
        { type: 'Integer', value: '1000000000' }
    ],
    signers: [
        {
            account: neosdk.wallet.getScriptHashFromAddress(neosdk.wallet.address),
            scopes: 'CalledByEntry'
        }
    ]
});
```

## Best Practices

When implementing a NEP-17 token contract:

1. **Follow the Standard**: Implement all required methods and events.

2. **Security First**: Always verify caller permissions using `check_witness`.

3. **Prevent Overflow**: Check for balance overflows during transfers.

4. **Handle Zero Accounts**: Consider special handling for zero addresses.

5. **Emit Events**: Always emit `Transfer` events for transfers.

6. **Use Constants**: Define token properties as constants.

7. **Optimize Storage**: Minimize storage operations for gas efficiency.

8. **Document Your Contract**: Add comments and documentation.

## Extensions and Customizations

The NEP-17 token example can be extended and customized in several ways:

### Mintable Token

Add a `mint` method to create new tokens:

```rust
#[method]
fn mint(to: Address, amount: u64) -> bool {
    // Only contract owner can mint
    let owner = Address::from_script_hash(&[/* owner script hash */]);
    if !runtime::check_witness(&owner) {
        return false;
    }
    
    // Update total supply and balance
    let storage = Storage::new();
    let total_supply = storage.total_supply.get().unwrap_or_default();
    let to_balance = storage.balances.get(&to).unwrap_or_default();
    
    storage.total_supply.put(total_supply + amount);
    storage.balances.put(&to, to_balance + amount);
    
    // Emit Transfer event
    let transfer_event = Transfer {
        from: None,
        to: Some(to),
        amount,
    };
    runtime::emit_event(transfer_event);
    
    true
}
```

### Burnable Token

Add a `burn` method to destroy tokens:

```rust
#[method]
fn burn(account: Address, amount: u64) -> bool {
    // Verify caller is authorized
    if !runtime::check_witness(&account) {
        return false;
    }
    
    // Check amount > 0
    if amount == 0 {
        return false;
    }
    
    // Get current balance
    let storage = Storage::new();
    let account_balance = storage.balances.get(&account).unwrap_or_default();
    
    // Check if enough balance
    if account_balance < amount {
        return false;
    }
    
    // Update total supply and balance
    let total_supply = storage.total_supply.get().unwrap_or_default();
    
    storage.total_supply.put(total_supply - amount);
    
    if account_balance == amount {
        storage.balances.delete(&account);
    } else {
        storage.balances.put(&account, account_balance - amount);
    }
    
    // Emit Transfer event
    let transfer_event = Transfer {
        from: Some(account),
        to: None,
        amount,
    };
    runtime::emit_event(transfer_event);
    
    true
}
```

### Pausable Token

Add a pause mechanism to temporarily freeze transfers:

```rust
#[storage]
struct TokenStorage {
    total_supply: u64,
    balances: Map<Address, u64>,
    paused: bool,
    owner: Address,
}

#[method]
fn pause() -> bool {
    let storage = Storage::new();
    let owner = storage.owner.get().unwrap();
    
    // Only owner can pause
    if !runtime::check_witness(&owner) {
        return false;
    }
    
    storage.paused.put(true);
    true
}

#[method]
fn unpause() -> bool {
    let storage = Storage::new();
    let owner = storage.owner.get().unwrap();
    
    // Only owner can unpause
    if !runtime::check_witness(&owner) {
        return false;
    }
    
    storage.paused.put(false);
    true
}

#[method]
fn transfer(from: Address, to: Address, amount: u64) -> bool {
    let storage = Storage::new();
    
    // Check if paused
    if storage.paused.get().unwrap_or_default() {
        return false;
    }
    
    // Transfer logic...
}
```

## Conclusion

The NEP-17 token example demonstrates how to implement a standard-compliant token contract using the Neo Contract Rust Framework. It provides both a low-level manual implementation and a high-level Rust implementation, serving as a reference for creating your own token contracts.

By following the standard and best practices, you can create reliable and interoperable token contracts for the Neo N3 blockchain.