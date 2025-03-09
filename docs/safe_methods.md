# Safe Methods in Neo Smart Contracts

This guide explains how to use safe (read-only) methods in Neo smart contracts written with the Neo Contract Framework for Rust.

## What Are Safe Methods?

Safe methods are contract functions that do not modify the blockchain state. They only read data without making any modifications. In Neo, safe methods:

- Do not require consensus
- Can be executed without network fees (free invocation)
- Cannot change storage, emit events, or transfer assets
- Return data to the caller efficiently

## Benefits of Safe Methods

Using safe methods offers several advantages:

1. **Reduced Costs**: Users don't pay fees to call safe methods
2. **Improved Performance**: Safe methods execute faster since they don't require consensus
3. **Better UX**: Frontend applications can retrieve data without requiring wallet signatures
4. **Enhanced Security**: Reduced attack surface for read-only operations

## Declaring Safe Methods

In the Neo Contract Framework, you can mark a method as safe using the `#[safe]` attribute. Important: the `#[safe]` attribute automatically implies that the function is a method, so you don't need to also add the `#[method]` attribute.

```rust
#[neo_contract::contract]
mod token_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct TokenContract {
        balances: Map<Address, u64>,
        total_supply: Item<u64>,
    }
    
    impl TokenContract {
        // Regular method that modifies state - needs #[method]
        #[method]
        fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // State-changing logic
            true
        }
        
        // Safe method that only reads state
        // Note: no need for #[method] - #[safe] already implies it's a method
        #[safe]
        fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        // Another safe method - also no need for #[method]
        #[safe]
        fn total_supply(&self) -> u64 {
            *self.total_supply.get()
        }
    }
}
```

## Method Attributes Summary

Here's a summary of the attributes used to mark methods:

1. `#[method]` - Marks a function as a contract method that can modify state (requires `&mut self`)
2. `#[safe]` - Marks a function as a read-only contract method (requires `&self`)
   - This implicitly includes method functionality, so don't use `#[method]` with it
3. `#[constructor]` - Marks a function as the contract's constructor

## How It Works

When you mark a method with `#[safe]`, several things happen:

1. The compiler includes the method in the contract manifest with the `safe: true` property
2. The method receives `&self` (immutable reference) instead of `&mut self`
3. The Neo VM sets appropriate call flags when the method is invoked

## Requirements for Safe Methods

For a method to be safely marked as `#[safe]`:

1. It must receive `&self` (immutable reference) instead of `&mut self`
2. It must not modify storage (all storage access must be read-only)
3. It must not emit events
4. It must not transfer assets or call methods that transfer assets
5. It must not use unsafe Neo syscalls that modify state

## Common Patterns

### Read-Only Queries

Safe methods are perfect for query operations:

```rust
#[safe]
fn get_token_info(&self) -> TokenInfo {
    TokenInfo {
        name: self.name.get().clone(),
        symbol: self.symbol.get().clone(),
        decimals: *self.decimals.get(),
        total_supply: *self.total_supply.get(),
    }
}
```

### View Methods for Collections

For collections, you can provide safe accessors:

```rust
#[safe]
fn get_voter_weight(&self, address: Address) -> u64 {
    self.voter_weights.get(&address).unwrap_or_default()
}

#[safe]
fn get_proposal(&self, id: u64) -> Option<Proposal> {
    self.proposals.get(&id).cloned()
}
```

### Pagination and Iterators

For larger collections, you can implement paginated access:

```rust
#[safe]
fn get_proposals(&self, start_idx: u64, count: u64) -> Vec<Proposal> {
    let mut result = Vec::new();
    let end_idx = start_idx + count;
    
    for idx in start_idx..end_idx {
        if let Some(proposal) = self.proposals.get(&idx) {
            result.push(proposal.clone());
        }
    }
    
    result
}
```

## Best Practices

### Do's

- **Do** mark all read-only methods with `#[safe]` instead of `#[method]`
- **Do** use safe methods for all data retrieval operations when possible
- **Do** return complete data structures when practical to minimize multiple calls
- **Do** implement pagination for methods that might return large datasets

### Don'ts

- **Don't** mark methods as safe if they modify state (compiler will catch this)
- **Don't** add both `#[method]` and `#[safe]` to the same function (redundant and confusing)
- **Don't** make unsafe syscalls from safe methods
- **Don't** confuse return value immutability with method safety
- **Don't** perform excessively complex computations in safe methods (they still consume resources)

## Testing Safe Methods

When testing, you should verify that your safe methods are properly marked and behave correctly:

```rust
#[test]
fn test_safe_methods() {
    let mut fixture = setup::new_fixture();
    let contract = deploy::deploy(&mut fixture, "MyContract", &[]);
    
    // Test that total_supply is marked as safe (no need for signing)
    let supply: u64 = read::call(&mut fixture, &contract, "total_supply", &[]).unwrap();
    assert_eq!(supply, 1000000);
    
    // For comparison, this requires signing:
    let result: bool = invoke::call(&mut fixture, &contract, "transfer", &[
        "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc".into(),
        "NV8R5aLqogCPbJ6HgzXy2gU1j4ZG9Mji1e".into(),
        100u64.into()
    ]).unwrap();
}
```

## Calling Safe Methods from Other Contracts

When one contract calls a safe method on another contract, the safety property is maintained:

```rust
#[method]
fn process_data(&mut self, token_contract: Address) -> u64 {
    // This call doesn't change state on token_contract,
    // so it remains safe even when called from a non-safe method
    let balance = self.call_contract::<u64>(
        &token_contract,
        "balance_of",
        (runtime::calling_script_hash(),)
    ).unwrap_or_default();
    
    // Do something with the balance...
    balance
}
```

## Conclusion

Safe methods are a powerful feature in Neo smart contracts. By properly marking your read-only methods with `#[safe]`, you provide users with fee-free access to contract data, improve performance, and enhance the overall user experience of your dApp. Remember that `#[safe]` already implies the method is exposed in the contract, so there's no need to also use `#[method]`.
