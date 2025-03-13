# Simple Token - Neo N3 NEP-17 Example

This example demonstrates a basic NEP-17 compatible token for the Neo N3 blockchain implemented in Rust using the Neo Contract framework.

## Token Features

- **Standard Compliance**: Implements the NEP-17 fungible token standard
- **Token Metadata**: Name, symbol, and decimals
- **Basic Operations**: Transfer, mint, and burn functionality
- **Event Emission**: Standard Transfer events with indexed parameters
- **Access Control**: Owner-only minting capabilities

## Contract Structure

### Token Metadata

The token includes standard metadata:
- **Name**: "Simple Token"
- **Symbol**: "SIMPLE"
- **Decimals**: 8

### Storage

The contract stores:
- `total_supply`: The total amount of tokens in circulation
- `balances`: A map of address balances

### Methods

**NEP-17 Standard Methods**:
- `symbol()`: Returns the token symbol
- `name()`: Returns the token name
- `decimals()`: Returns the number of decimal places
- `total_supply()`: Returns the total token supply
- `balance_of(account: H160)`: Returns the balance of an address
- `transfer(to: H160, amount: Int256)`: Transfers tokens between addresses

**Additional Methods**:
- `mint(to: H160, amount: Int256)`: Creates new tokens (owner only)
- `burn(amount: Int256)`: Destroys tokens from the caller's balance

### Events

The contract emits:
- `Transfer`: When tokens are transferred, minted, or burned

## Code Highlights

### NEP-17 Token Implementation

```rust
#[contract]
#[contract_permission(storage, *)]
#[manifest_extra(
    name = "Simple Token",
    author = "Neo Contract RS",
    email = "dev@neo.org",
    description = "A simple NEP-17 token example"
)]
#[supported_standards("NEP-17")]
mod token_contract {
    // Contract implementation...
}
```

### Storage Definition

```rust
#[storage]
pub struct Token {
    // Total token supply
    total_supply: Int256,
    // Map of address balances
    balances: Map<H160, Int256>,
}
```

### Transfer Implementation

```rust
fn do_transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
    // Security checks
    if to.is_zero() || amount.is_zero() || amount < Int256::zero() {
        return false;
    }
    
    // Check if sender has enough balance
    let from_balance = self.balances.get(&from).unwrap_or_default();
    if from_balance < amount {
        return false;
    }
    
    // Update balances
    let new_from_balance = from_balance - amount.clone();
    // ...update storage...
    
    // Emit event
    let event = Transfer {
        from: Some(from),
        to: Some(to),
        amount: amount.clone(),
    };
    event.emit();
    
    true
}
```

## Building the Contract

To build the contract:

```bash
# Development build
cargo build -p simple_token --features std

# Production build
cargo build -p simple_token --release
```

## Using the Contract

After deployment, you can interact with the token using Neo CLI or any Neo SDK:

```javascript
// JavaScript example with neo-js
const { rpc, sc, wallet } = require('@cityofzion/neo-js');

// Connect to the contract
const simpleToken = new sc.Contract('0xYourContractScriptHash');

// Check token details
const name = await simpleToken.call('name');
const symbol = await simpleToken.call('symbol');
const decimals = await simpleToken.call('decimals');
const totalSupply = await simpleToken.call('totalSupply');

console.log(`Token: ${name} (${symbol})`);
console.log(`Decimals: ${decimals}`);
console.log(`Total Supply: ${totalSupply / Math.pow(10, decimals)}`);

// Check an address balance
const balance = await simpleToken.call('balanceOf', ['NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj']);
console.log(`Balance: ${balance / Math.pow(10, decimals)}`);

// Transfer tokens (requires a signed transaction)
const account = wallet.Account.fromWIF('YourPrivateKeyWIF');
const transferTx = await simpleToken.invoke(
  'transfer',
  [
    'NYxb4fSZVKAz8YHBehLW1BNQpGP8Em2pf7',  // to address
    10 * Math.pow(10, decimals)             // amount with decimals
  ],
  account
);
await transferTx.send();
```

## Educational Value

This example demonstrates:

1. **NEP-17 Standard Implementation**: The core methods required for Neo tokens
2. **Storage Patterns**: Using maps for token balances
3. **Event Patterns**: Proper event emission with indexed parameters
4. **Security Patterns**: Check ownership, validate transfers, and prevent common issues
5. **Neo Contract Annotations**: Using macros for contract definition

## Known Issues and Workarounds

As with other examples, you may encounter:

1. **Procedural Macro Issues**: The `#[contract]` and other macros may not resolve correctly
2. **Int256 Type Issues**: The `Int256` type might need explicit importing or compatibility handling
3. **Runtime Function Signature Mismatches**: Check for current function signatures in the framework

## License

This example is provided under the same license as the Neo Contract Rust framework. 