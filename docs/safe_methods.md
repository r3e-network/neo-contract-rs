# Safe Methods in Neo N3 Smart Contracts

In Neo N3 smart contracts, methods can be categorized as either "safe" or "non-safe" based on whether they modify the contract's state.

## Understanding Safe vs. Non-Safe Methods

### Safe Methods
Safe methods are read-only operations that do not modify the contract's storage state. These methods have the following characteristics:
- They take `&self` (immutable reference) as the first parameter
- They only perform read operations on the contract storage
- They can be executed without requiring a full verification from the blockchain

### Non-Safe Methods
Non-safe methods can potentially modify the contract's storage state. These methods have the following characteristics:
- They take `&mut self` (mutable reference) as the first parameter
- They can perform write operations on the contract storage
- They require full verification from the blockchain to execute

## Using the `#[safe]` Attribute

In neo-contract-rs, you can mark a method as safe by adding the `#[safe]` attribute:

```rust
#[message]
#[safe]
pub fn balance_of(&self, account: H160) -> Int256 {
    // Read-only implementation
}
```

The `#[safe]` attribute is transformed into a `"safe": true` flag in the contract manifest file, which informs the Neo Virtual Machine that the method can be executed without state changes, allowing for optimized execution.

## Usage Examples

### Read-Only Methods (Should be marked as safe)

```rust
// Get token name - read-only method
#[message]
#[safe]
pub fn get_name(&self) -> ByteString {
    self.token_name.clone()
}

// Get token balance - read-only method
#[message]
#[safe]
pub fn balance_of(&self, account: H160) -> Int256 {
    match self.balances.get(&account) {
        Some(balance) => balance.clone(),
        None => Int256::zero(),
    }
}
```

### State-Modifying Methods (Should NOT be marked as safe)

```rust
// Transfer tokens - modifies state
#[message]
pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
    // Implementation that modifies state
}

// Mint new tokens - modifies state
#[message]
pub fn mint(&mut self, to: H160, amount: Int256) -> bool {
    // Implementation that modifies state
}
```

## Benefits of Using Safe Methods

1. **Performance**: Safe methods can be executed more efficiently as they don't need to update the blockchain state
2. **Cost**: Calling safe methods typically requires less GAS as they don't modify storage
3. **Security**: Clearly distinguishing between read-only and state-modifying operations enhances contract security

## Contract Manifest Representation

In the contract manifest JSON file, safe methods are represented with a `"safe": true` property:

```json
{
  "methods": [
    {
      "name": "balance_of",
      "parameters": [
        {
          "name": "account",
          "type": "Hash160"
        }
      ],
      "returntype": "Integer",
      "offset": 0,
      "safe": true
    },
    {
      "name": "transfer",
      "parameters": [
        {
          "name": "from",
          "type": "Hash160"
        },
        {
          "name": "to",
          "type": "Hash160"
        },
        {
          "name": "amount",
          "type": "Integer"
        }
      ],
      "returntype": "Boolean",
      "offset": 0,
      "safe": false
    }
  ]
}
```

## Best Practices

1. Always mark read-only methods with the `#[safe]` attribute
2. Never mark state-modifying methods as safe
3. Place the `#[safe]` attribute after the `#[message]` attribute for clarity
4. Ensure that methods marked as safe never modify contract storage
