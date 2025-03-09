# Neo Contract Framework for Rust

This repository contains a framework for writing Neo N3 smart contracts in Rust with an ink!-style syntax. The framework compiles Rust code to WebAssembly as an intermediate representation, then converts the WebAssembly bytecode to Neo VM bytecode.

## Documentation

### Getting Started

- [Getting Started](getting_started.md) - Setup instructions and your first contract
- [Ink Style Guide](ink_style_guide.md) - How to write contracts using ink!-style syntax

### Technical Details

- [WebAssembly to Neo Conversion](wasm_to_neo_conversion.md) - How WebAssembly is converted to Neo VM bytecode
- [Implementation Summary](implementation_summary.md) - Current state of the framework and roadmap
- [Safe Methods](safe_methods.md) - Writing read-only contract methods

### Tutorials

- [NEP-17 Token Tutorial](nep17_tutorial.md) - Creating a fungible token contract

## Architecture

The Neo Contract Framework consists of three main components:

1. **Neo Contract SDK**: Rust crate providing macros, traits, and utilities for writing Neo smart contracts
2. **Neo Compiler**: Tool that compiles Rust smart contracts to Neo VM bytecode
3. **Neo Contract Macros**: Procedural macros for the ink!-style syntax

### Compilation Process

```
┌─────────────────┐     ┌───────────────┐     ┌─────────────┐
│                 │     │               │     │             │
│   Rust Source   │ --> │   WebAssembly │ --> │   Neo VM    │
│  (ink!-style)   │     │    (.wasm)    │     │  Bytecode   │
│                 │     │               │     │             │
└─────────────────┘     └───────────────┘     └─────────────┘
         │                     │                     │
         ▼                     ▼                     ▼
┌─────────────────┐     ┌───────────────┐     ┌─────────────┐
│                 │     │               │     │             │
│    Rust SDK     │     │  WASM Parser  │     │ NEF File &  │
│    & Macros     │     │  & Converter  │     │  Manifest   │
│                 │     │               │     │             │
└─────────────────┘     └───────────────┘     └─────────────┘
```

## Example Contract

Here's a simple NEP-17 token contract written using the Neo Contract Framework:

```rust
#[contract]
mod token {
    use neo_contract::prelude::*;
    
    #[storage]
    struct TokenContract {
        total_supply: Item<u64>,
        balances: Map<Address, u64>,
    }
    
    impl TokenContract {
        #[constructor]
        fn new(owner: Address, supply: u64) -> Self {
            let mut balances = Map::new();
            balances.insert(owner, supply);
            
            Self {
                total_supply: Item::new(supply),
                balances,
            }
        }
        
        #[method]
        fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            assert!(runtime::check_witness(&from), "No authorization");
            assert!(to != Address::zero(), "Invalid to address");
            
            let from_balance = self.balances.get(&from).unwrap_or_default();
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Update balances
            self.balances.insert(from, from_balance - amount);
            let to_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(to, to_balance + amount);
            
            // Emit transfer event
            events::transfer(&from, &to, amount);
            
            true
        }
        
        #[safe]
        fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        #[safe]
        fn total_supply(&self) -> u64 {
            *self.total_supply.get()
        }
    }
}
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
