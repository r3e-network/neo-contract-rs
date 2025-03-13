# Neo Contract Annotation Update Guide

This guide explains how to update Neo smart contract examples to use the modern Neo Contract annotation syntax. Use this guide to update any remaining examples in the repository.

## Key Annotations

| Old Style | New Style | Purpose |
|-----------|-----------|---------|
| `#[contract]` | `#[neo_contract::contract]` | Marks a struct as a smart contract |
| `#[method]` or `#[method]` | `#[method]` | Exposes methods for external calls |
| `#[safe]` | `#[safe]` | Marks read-only methods for gas optimization |
| N/A | `#[no_reentry]` | Prevents re-entrancy attacks |
| `#[constructor]` | `#[constructor]` | Identifies the initialization method |
| `#[event]` with impl | `#[neo_contract::event(...)]` | Defines structured events |
| `#[storage]` | Contract struct | Storage is now defined in the contract struct |
| Event emission with `Runtime::notify` | `Event {}.notify(&param1, &param2)` | Simpler event emission |

## Step-by-Step Update Process

1. **Update Contract Structure**:
   - Replace `mod contract_name { ... }` with `pub struct ContractName { ... }`
   - Move storage fields directly into the contract struct
   - Add `#[neo_contract::contract]` above the struct

2. **Update Events**:
   - Replace event structs with `#[neo_contract::event(...)]` annotation
   - Remove manual event emission code (impl blocks with emit methods)
   - Replace event emission with `EventName {}.notify(&param1, &param2, ...)`

3. **Update Methods**:
   - Remove any `#[neo_contract::manifest]` annotations from impl blocks (manifest is auto-generated)
   - Add `#[method]` to all externally callable methods
   - Add `#[safe]` to all read-only methods 
   - Add `#[no_reentry]` to state-modifying methods
   - Make methods `pub` for visibility

4. **Update Storage Access**:
   - Replace `Item<T>` with `StorageMap<String, T>`
   - Replace `Map<K, V>` with `StorageMap<K, V>`
   - Update storage access patterns:
     - From: `item.get()`, `item.set(value)`
     - To: `map.get("key")`, `map.insert("key", value)`

5. **Update Constructor**:
   - Use `#[constructor]` annotation (same as before)
   - Initialize contract with all storage maps
   - Add explicit values to storage in constructor body

## Example Transformation

### Old Style:

```rust
#[contract]
mod token {
    #[storage]
    struct Token {
        supply: Item<u64>,
        balances: Map<Address, u64>,
    }
    
    #[event]
    struct Transfer {
        #[index]
        from: Address,
        #[index]
        to: Address,
        amount: u64,
    }
    
    impl Transfer {
        fn emit(from: Address, to: Address, amount: u64) {
            // Event emission code
        }
    }
    
    impl Token {
        #[constructor]
        fn new(supply: u64) -> Self {
            // Construction code
        }
        
        #[method]
        fn transfer(&mut self, to: Address, amount: u64) -> bool {
            // Transfer code
        }
        
        #[safe]
        fn balance_of(&self, address: Address) -> u64 {
            // Balance code
        }
    }
}
```

### New Style:

```rust
#[neo_contract::event(
    from: Address,
    to: Address,
    amount: u64
)]
struct Transfer {}

#[neo_contract::contract]
pub struct Token {
    supply: StorageMap<String, u64>,
    balances: StorageMap<Address, u64>,
}

impl Token {
    #[constructor]
    pub fn new(supply: u64) -> Self {
        let mut instance = Self {
            supply: StorageMap::new(b"supply"),
            balances: StorageMap::new(b"balances"),
        };
        instance.supply.insert("value", supply);
        instance
    }
    
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
        // Transfer code
        Transfer {}.notify(&from, &to, &amount);
        true
    }
    
    #[method]
    #[safe]
    pub fn balance_of(&self, address: Address) -> u64 {
        self.balances.get(&address).unwrap_or_default()
    }
}
```

## Tips for Specific Patterns

### 1. Event Parameters:
- When emitting events, pass references with `&` for all parameters
- Use `Option::<Type>::None` for explicit None values

### 2. Storage Keys:
- For single values, use consistent keys like "value", "count", or "address"
- For maps, you don't need key strings - the key is already provided in the map access

### 3. Error Handling:
- Use `assert!` for contract requirements instead of if/return patterns
- Use `unwrap_or_default()` to handle missing storage values

### 4. Runtime API:
- Use `Runtime::calling_script_hash()` to get the caller
- Use `Runtime::check_witness(&address)` to verify authorization
- Use `Runtime::call_contract(&address, "method", &[arg1, arg2])` for contract calls

## Testing Annotations

After updating the contract, ensure it compiles successfully. The Neo compiler will recognize these annotations and generate the appropriate contract manifest.

## Specific Example Updates

We've updated the following examples to use the new annotation syntax:

1. **Simple Token**: Basic NEP-17 token implementation
2. **Event Demo**: Demonstrates various event patterns
3. **NEP-17 Token**: Standard-compliant token implementation 
4. **Ledger Workshop Example**: Complex escrow contract using Ledger API
5. **Transfer**: Simple native asset transfer demonstration
6. **Ink Style Token with Attributes**: Advanced NEP-17 token with admin controls
7. **Contract Call**: Contract-to-contract interaction with method invocation
8. **DEX (Decentralized Exchange)**: Automated market maker with liquidity pool management

Use these as reference implementations when updating other examples. 