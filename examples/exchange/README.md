# Neo Exchange Contract Example

This example demonstrates how to build interoperable Neo N3 smart contracts that can interact with other contracts on the blockchain.

## Interoperability Features

This exchange contract implements several interoperability patterns:

1. **NEP-17 Token Interactions**: The contract can call methods on NEP-17 token contracts to transfer tokens
2. **NEP-17 Receiver Implementation**: The contract implements `onNEP17Payment` to receive token transfers
3. **Whitelist System**: The contract maintains a whitelist of accepted tokens
4. **Exchange Functionality**: The contract allows trading between different tokens

## Contract-to-Contract Interaction Patterns

### 1. Calling Other Contracts

The exchange contract calls the `transfer` method on NEP-17 token contracts:

```rust
fn transfer_tokens(&self, token: &H160, to: &H160, amount: u64) -> bool {
    let mut args = Array::new();
    args.push(Any::from(Runtime::executing_script_hash())); // from (this contract)
    args.push(Any::from(*to)); // to 
    args.push(Any::integer(amount as i64)); // amount
    args.push(Any::null()); // data
    
    // Call the NEP-17 transfer method with proper permissions
    let result = Runtime::call_contract(
        token,
        "transfer",
        &args,
        CALL_FLAG_ALLOW_CALL | CALL_FLAG_ALLOW_NOTIFY
    );
    
    // Check result
    result.as_bool().unwrap_or(false)
}
```

### 2. Receiving Calls from Other Contracts

The exchange implements the NEP-17 payment handler:

```rust
// #[method]
pub fn onNEP17Payment(&mut self, from: H160, amount: i64, data: Vec<u8>) -> bool {
    // Ensure amount is positive
    if amount <= 0 {
        return false;
    }
    
    // Get token contract that is sending tokens
    let token = Runtime::calling_script_hash();
    
    // Check if token is whitelisted
    if !self.is_whitelisted(&token) {
        return false;
    }
    
    // Update reserves for the token
    self.update_reserve(&token, amount as u64, true);
    
    true
}
```

### 3. Reentrancy Protection

The contract includes protection against reentrancy attacks:

```rust
// #[method]
// Protects against reentrancy
pub fn swap(&mut self, token_in: H160, token_out: H160, amount_in: u64) -> bool {
    // Check for reentrancy
    if self.in_swap.get().unwrap_or(None).unwrap_or(false) {
        return false;
    }
    
    // Set reentrancy guard
    self.in_swap.set(&true).unwrap_or(());
    
    // Update state BEFORE external calls
    self.update_reserve(&token_in, amount_in, true);
    self.update_reserve(&token_out, amount_out, false);
    
    // Make external call AFTER state update
    if !self.transfer_tokens(&token_out, &trader, amount_out) {
        // Handle failure and rollback if needed
    }
    
    // Clear reentrancy guard
    self.in_swap.set(&false).unwrap_or(());
    
    true
}
```

### 4. Error Handling and State Recovery

The contract handles errors from external contract calls and recovers state:

```rust
// If token transfer fails, revert state changes
if !self.transfer_tokens(&token_out, &trader, amount_out) {
    // Refund if transfer fails (revert reserve changes)
    self.update_reserve(&token_in, amount_in, false);
    self.update_reserve(&token_out, amount_out, true);
    self.transfer_tokens(&token_in, &trader, amount_in);
    self.in_swap.set(&false).unwrap_or(());
    return false;
}
```

## Testing Contract Interactions

The example includes tests for contract interactions using mocks:

```rust
#[test]
fn test_swap_tokens() {
    // Arrange
    let mut contract = setup_test_contract();
    let (token_a, token_b) = get_test_tokens();
    
    // Mock token transfer to return true
    test_runtime::mock_contract_call_result(&token_a, "transfer", Any::boolean(true));
    test_runtime::mock_contract_call_result(&token_b, "transfer", Any::boolean(true));
    
    // Act
    let result = contract.swap(token_a, token_b, 100);
    
    // Assert
    assert!(result);
}
```

## Building and Deploying

```bash
# Build the contract
cargo build --release --target wasm32-unknown-unknown

# Compile to NEO VM bytecode
neo-compiler compile target/wasm32-unknown-unknown/release/exchange.wasm --output build/

# Deploy to a private network
neoxp contract deploy build/exchange.nef owner
```

## Interacting with the Contract

To interact with the deployed contract, you'll need to:

1. Deploy NEP-17 token contracts
2. Add the token contracts to the exchange whitelist
3. Create token pairs
4. Add liquidity
5. Perform swaps

Example with Neo Express:

```bash
# Get contract hashes
TOKEN_A_HASH="0x1111111111111111111111111111111111111111"
TOKEN_B_HASH="0x2222222222222222222222222222222222222222"
EXCHANGE_HASH="0xabcdef1234567890abcdef1234567890abcdef12"

# Whitelist tokens
neoxp contract invoke $EXCHANGE_HASH addToWhitelist "[$TOKEN_A_HASH]" --account owner
neoxp contract invoke $EXCHANGE_HASH addToWhitelist "[$TOKEN_B_HASH]" --account owner

# Add token pair
neoxp contract invoke $EXCHANGE_HASH addTokenPair "[$TOKEN_A_HASH,$TOKEN_B_HASH]" --account owner

# Add liquidity
neoxp contract invoke $EXCHANGE_HASH addLiquidity "[$TOKEN_A_HASH,$TOKEN_B_HASH,1000,1000]" --account owner

# Perform swap
neoxp contract invoke $EXCHANGE_HASH swap "[$TOKEN_A_HASH,$TOKEN_B_HASH,100]" --account user1
```

For more details on contract interoperability, see the [Interoperability Guide](../../docs/INTEROPERABILITY-GUIDE.md). 