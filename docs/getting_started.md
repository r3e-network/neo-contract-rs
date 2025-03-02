# Getting Started with Neo Contract RS

This guide will help you get started with Neo Contract RS - a Rust framework for developing Neo N3 smart contracts.

## Prerequisites

Before you begin, make sure you have the following installed:

1. **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **WebAssembly Target**: Add the WebAssembly target to your Rust toolchain
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Git**: For cloning the repository

## Setting Up Your First Neo Contract Project

### Option 1: Using the Template (Recommended)

1. Clone the neo-contract-rs repository:
   ```bash
   git clone https://github.com/R3E-Network/neo-contract-rs.git
   cd neo-contract-rs
   ```

2. Copy an example as a starting point:
   ```bash
   cp -r examples/nep17_token my_contract
   cd my_contract
   ```

3. Update the `Cargo.toml` file with your project information:
   ```toml
   [package]
   name = "my_contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   neo-contract = { path = "../neo-contract" }
   wee_alloc = "0.4.5"
   ```

4. Modify the code in `src/lib.rs` to implement your contract logic.

### Option 2: Starting from Scratch

1. Create a new Rust library project:
   ```bash
   cargo new --lib my_contract
   cd my_contract
   ```

2. Configure your `Cargo.toml` file:
   ```toml
   [package]
   name = "my_contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   neo-contract = { git = "https://github.com/R3E-Network/neo-contract-rs" }
   wee_alloc = "0.4.5"
   ```

3. Create a basic contract in `src/lib.rs`:
   ```rust
   #![no_std]
   #![no_main]

   extern crate alloc;
   extern crate wee_alloc;

   use neo_contract::{
       builtin::{ByteString, Int256},
       contract_method, smart_contract,
   };

   // Use wee_alloc as the global allocator
   #[global_allocator]
   static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

   // Define a panic handler
   #[panic_handler]
   fn panic(_info: &core::panic::PanicInfo) -> ! {
       loop {}
   }

   pub struct MyContract;

   #[smart_contract]
   impl MyContract {
       contract_method!(pub fn hello() -> ByteString {
           ByteString::from("Hello, Neo N3!")
       });

       contract_method!(pub fn add(a: Int256, b: Int256) -> Int256 {
           a + b
       });
   }
   ```

## Building Your Contract

Build your contract with the WebAssembly target:

```bash
cargo build --target wasm32-unknown-unknown --release
```

This will generate a WebAssembly binary at `target/wasm32-unknown-unknown/release/my_contract.wasm`.

## Contract Development Approaches

Neo Contract RS supports two main approaches to contract development:

### 1. Traditional Macro Approach

This approach uses the `smart_contract` and `contract_method` macros:

```rust
#[smart_contract]
impl MyContract {
    contract_method!(pub fn my_method(param: Int256) -> Int256 {
        // Implementation
        param + 1
    });
}
```

### 2. Attribute Macro Approach (Recommended)

This approach is inspired by ink! and provides a more declarative style:

```rust
#[neo_contract::contract]
#[neo_contract::contract_author("Your Name")]
mod my_contract {
    #[neo(storage)]
    pub struct MyContract {
        value: Int256,
    }
    
    impl MyContract {
        #[neo(constructor)]
        pub fn new(initial_value: Int256) -> Self {
            Self { value: initial_value }
        }
        
        #[neo(message)]
        pub fn get_value(&self) -> Int256 {
            self.value
        }
        
        #[neo(message)]
        pub fn set_value(&mut self, new_value: Int256) {
            self.value = new_value;
        }
    }
}
```

## Testing Your Contract

Testing Neo N3 smart contracts involves:

1. **Unit Testing**: Write Rust unit tests for your contract logic
2. **Integration Testing**: Test your contract in a Neo N3 private network or TestNet

### Unit Testing Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        let contract = MyContract;
        let result = contract.add(Int256::from(5), Int256::from(7));
        assert_eq!(result, Int256::from(12));
    }
}
```

## Examples and References

- Check the [examples directory](https://github.com/R3E-Network/neo-contract-rs/tree/main/examples) for complete contract examples
- Refer to the [NEP-17 Tutorial](./nep17_tutorial.md) for creating a token contract
- See the [Deployment Guide](./deployment_guide.md) for deploying your contract

## Next Steps

- Learn about [NEP-17](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki) for creating fungible tokens
- Explore [Storage](https://docs.neo.org/docs/en-us/reference/scapi/framework/storage.html) for persisting data
- Understand [Contract Calls](https://docs.neo.org/docs/en-us/reference/scapi/framework/services/contractmanagement.html) for interacting with other contracts
