# NEP-17 Fungible Token Implementation for Neo N3

This example demonstrates a complete implementation of the NEP-17 Fungible Token Standard for the Neo N3 blockchain using the neo-contract-rs framework.

## Overview

NEP-17 is the fungible token standard for the Neo N3 blockchain, similar to ERC-20 in Ethereum. This implementation showcases best practices for Neo N3 smart contract development with a focus on proper event handling, security, and gas optimization.

## Key Features

- **Full NEP-17 Compliance**: Implements all required methods and events specified in the [NEP-17 standard](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki)
- **Neo N3 Event Handling**: Uses the proper `Runtime::notify` pattern for event emission
- **Safe Method Annotations**: Correctly marks read-only methods with `#[safe]` for optimized invocation and gas savings
- **Security Measures**: Implements authorization checks and reentrancy protection
- **Extension Functionality**: Includes additional features like minting, burning, and ownership management

## Implementation Details

### Token Metadata

The token implementation includes standard metadata:

```rust
const TOKEN_NAME: &str = "Sample NEP17 Token";
const TOKEN_SYMBOL: &str = "NEP17";
const TOKEN_DECIMALS: u8 = 8;
const TOKEN_TOTAL_SUPPLY: u64 = 100_000_000 * 100_000_000; // 100M tokens with 8 decimals
```

### Storage Structure

The contract uses Neo N3's storage capabilities to maintain state:

```rust
#[storage]
struct NEP17Token {
    /// Token balances for each address
    balances: Map<Address, u64>,
    
    /// Total token supply
    total_supply: Item<u64>,
    
    /// Contract owner address
    owner: Item<Address>,
}
```

### NEP-17 Required Methods

The contract implements all required NEP-17 methods:

1. **`symbol()`**: Returns the token's symbol
2. **`decimals()`**: Returns the token's decimal precision
3. **`totalSupply()`**: Returns the total supply of tokens
4. **`balanceOf(account)`**: Returns the token balance of the specified account
5. **`transfer(from, to, amount, data)`**: Transfers tokens between accounts

### Proper Neo N3 Event Implementation

This example demonstrates the proper way to implement events in Neo N3:

```rust
/// Event emitted when tokens are transferred
struct Transfer {}

/// Implementation for properly emitting the Transfer event using Neo N3 standards
impl Transfer {
    /// Static method to emit the Transfer event in Neo N3 format
    pub fn emit(from: Option<Address>, to: Option<Address>, amount: u64) {
        // Create event name as ByteString (required for Neo N3)
        let event_name = ByteString::from("Transfer");
        
        // Create Array to hold event parameters (required for Neo N3)
        let mut event_data = Array::<Any>::new();
        
        // Add parameters with proper Neo N3 format
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // null for minting
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()), // null for burning
        }
        
        event_data.push(Any::from(amount));
        
        // Emit the event using Runtime::notify (required for Neo N3)
        Runtime::notify(&event_name, &event_data);
    }
}
```

### Safe Method Annotations

Read-only methods are properly marked with the `#[safe]` attribute to optimize gas usage and improve security:

```rust
/// Get the name of the token
#[method]
#[safe]
fn name(&self) -> String {
    TOKEN_NAME.to_string()
}

/// Get the symbol of the token
#[method]
#[safe]
fn symbol(&self) -> String {
    TOKEN_SYMBOL.to_string()
}

/// Get the token balance of the specified address
#[method]
#[safe]
fn balance_of(&self, address: Address) -> u64 {
    self.balances.get(&address).unwrap_or_default()
}
```

### Security Features

The contract includes important security features:

1. **Authorization Checks**: Using `Runtime::check_witness` to verify transaction signers
2. **Reentrancy Protection**: Using `#[no_reentrant]` attribute on state-modifying methods
3. **Ownership Controls**: Limiting sensitive operations to the contract owner

## Building and Deploying

To compile and deploy this NEP-17 token to the Neo N3 blockchain:

1. **Compile to WebAssembly**:
   ```bash
   cd examples/nep17
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Convert to Neo N3 format**:
   ```bash
   neo-compiler compile \
       ../../target/wasm32-unknown-unknown/release/nep17_example.wasm \
       --output ./build
   ```

3. **Deploy using Neo CLI**:
   ```bash
   neo-cli deploy ./build/nep17_example.nef ./build/nep17_example.manifest.json
   ```

## Testing

This contract includes comprehensive tests that demonstrate best practices for testing Neo N3 smart contracts. The tests verify:

1. Token creation and initial supply
2. Token transfers and balance updates
3. Event emission conformance
4. Authorization controls and security measures

## Best Practices Demonstrated

This implementation showcases several Neo N3 best practices:

1. **Proper Event Emission**: Using `Runtime::notify` instead of macros for Neo N3 compatibility
2. **Safe Methods**: Marking read-only methods with `#[safe]` for gas optimization
3. **Security Checks**: Always verifying transaction signers before state modifications
4. **Clean Storage Design**: Using structured storage types for better organization
5. **Event Indexing**: Properly marking indexed event parameters for blockchain indexing

## Neo N3 Contract Interaction

To interact with this token from other contracts, utilize the proper contract call patterns:

```rust
// Call the transfer method on the NEP-17 token
let token_script_hash = H160::from_hex("your_token_hash").unwrap();
let method = "transfer";
let args = vec![
    StackItem::from(from_address),
    StackItem::from(to_address),
    StackItem::from(amount),
    StackItem::from(data),
];

// Make the call with proper flags
let result = contract::call(
    &token_script_hash,
    method,
    &args,
    CallFlags::All
);
```

For more information on Neo N3 contract development, see the [Neo N3 Implementation Guide](../../docs/neo_n3_implementation_guide.md).
