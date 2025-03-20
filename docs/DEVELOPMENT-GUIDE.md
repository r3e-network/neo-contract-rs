# NEO Contract Rust Development Guide

This guide will help you set up your development environment and create, build, and deploy smart contracts using the Neo Contract Rust Framework.

## Setting Up Your Development Environment

### Prerequisites

1. **Rust and Cargo**
   - Install the latest version of Rust and Cargo from [rustup.rs](https://rustup.rs/)
   - Make sure your Rust installation is up to date with `rustup update`

2. **WebAssembly Target**
   - Add the WebAssembly target to your Rust installation:
     ```bash
     rustup target add wasm32-unknown-unknown
     ```

3. **Neo Compiler**
   - Build the Neo compiler from source:
     ```bash
     git clone https://github.com/R3E-Network/neo-contract-rs
     cd neo-contract-rs
     cargo build --release -p neo-compiler
     ```
   - The compiled binary will be in `target/release/neo-compiler`

### Project Structure

A typical Neo contract project includes the following files:

```
my-contract/
├── Cargo.toml          # Rust package manifest
├── src/
│   └── lib.rs          # Contract code
└── build/              # Compiled contract output
```

## Creating Your First Contract

### 1. Initialize a New Project

```bash
cargo new --lib my-contract
cd my-contract
```

### 2. Configure Cargo.toml

Update your `Cargo.toml` to include the Neo Contract Rust dependencies:

```toml
[package]
name = "my-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { git = "https://github.com/R3E-Network/neo-contract-rs", features = ["contract-macros"] }
neo-macros = { git = "https://github.com/R3E-Network/neo-contract-rs" }

[profile.release]
lto = true
opt-level = "z"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

### 3. Write a Basic Contract

Create a basic Neo contract in `src/lib.rs`:

```rust
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use neo_contract::prelude::*;
use neo_macros::{
    contract, contract_author, contract_description, contract_version,
    event, index, storage, constructor, method, safe
};

/// A simple Hello World contract for Neo N3
#[contract]
#[contract_author("Your Name")]
#[contract_description("A simple Hello World contract")]
#[contract_version("0.1.0")]
mod hello_contract {
    use super::*;
    use neo_contract::types::builtin::any::Any;
    use neo_contract::types::builtin::array::Array;
    use neo_contract::types::builtin::h160::H160;
    use neo_contract::types::builtin::string::ByteString;
    use neo_contract::Runtime;

    /// Event emitted when the message is updated
    #[event]
    struct MessageUpdated {
        #[index]
        from: H160,
        old_message: String,
        new_message: String,
    }

    /// Main contract storage
    #[storage]
    struct HelloContract {
        message: Item<ByteString>,
        owner: Item<H160>,
    }

    impl HelloContract {
        /// Initialize a new contract
        #[constructor]
        fn new() -> Self {
            // Set the default message
            let message = ByteString::from("Hello, Neo!");

            // Set the owner to the transaction sender
            let sender = Runtime::calling_script_hash();

            // Create and initialize the contract
            let mut contract = Self {
                message: Item::new(b"message"),
                owner: Item::new(b"owner"),
            };

            // Store initial values
            contract.message.set(&message).unwrap_or(());
            contract.owner.set(&sender).unwrap_or(());

            contract
        }

        /// Update the message
        #[method]
        fn update_message(&mut self, new_message: ByteString) -> bool {
            // Only the owner can update the message
            let sender = Runtime::calling_script_hash();
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();

            if sender != owner {
                return false;
            }

            // Get the current message
            let old_message = self.message.get().unwrap_or(None).unwrap_or_default();

            // Update the message
            self.message.set(&new_message).unwrap_or(());

            // Emit event
            let mut event_args = Array::new();
            event_args.push(Any::from(sender));
            event_args.push(Any::from(ByteString::from_utf8_str(&old_message)));
            event_args.push(Any::from(ByteString::from_utf8_str(&new_message)));
            Runtime::notify(&ByteString::from("MessageUpdated"), &event_args);

            true
        }

        /// Get the current message
        #[safe]
        fn get_message(&self) -> ByteString {
            self.message.get().unwrap_or(None).unwrap_or_default()
        }

        /// Get the owner of the contract
        #[safe]
        fn get_owner(&self) -> H160 {
            self.owner.get().unwrap_or(None).unwrap_or_default()
        }
    }
}

// Entry points are managed by the contract macro
#[no_mangle]
pub fn deploying() -> bool {
    true
}

#[no_mangle]
pub fn invoke(operation: String, args: Vec<Any>) -> Any {
    // Create an instance of the contract
    let mut contract = hello_contract::HelloContract::new();

    // Handle operations
    match operation.as_str() {
        "update_message" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }

            let new_message = if let Some(bs) = args[0].as_byte_string() {
                bs.clone()
            } else {
                return Any::boolean(false);
            };

            Any::boolean(contract.update_message(new_message))
        },
        "get_message" => {
            Any::byte_string(contract.get_message())
        },
        "get_owner" => {
            Any::h160(contract.get_owner())
        },
        _ => {
            Any::null()
        }
    }
}
```

## Building and Compiling

### 1. Build WebAssembly

Build your contract to WebAssembly:

```bash
cargo build --release --target wasm32-unknown-unknown
```

This will generate a `target/wasm32-unknown-unknown/release/my_contract.wasm` file.

### 2. Compile to NEO VM Bytecode

Use the Neo compiler to convert the WebAssembly file to Neo VM bytecode:

```bash
neo-compiler compile target/wasm32-unknown-unknown/release/my_contract.wasm --output build/
```

This will generate two files in the `build/` directory:
- `my_contract.nef`: The Neo Executable Format file
- `my_contract.manifest.json`: The contract manifest

## Testing

### 1. Create a Test File

Create a test file in `tests/`:

```rust
// tests/contract_test.rs
use neo_contract_testing::prelude::*;

use my_contract::{deploy, invoke};

#[test]
fn test_hello_contract() {
    // Create a new test context
    let mut context = TestContext::new();

    // Deploy the contract
    let result = context.deploy(|| deploy());
    assert!(result.is_ok());

    // Call the get_message method
    let result = context.call(|| invoke("get_message", vec![]));
    assert_eq!(result.as_byte_string().unwrap().to_string(), "Hello, Neo!");

    // Update the message
    let new_message = neo_contract::ByteString::from("Hello, World!");
    let result = context.call(|| invoke("update_message", vec![new_message.into()]));
    assert!(result.as_boolean().unwrap());

    // Call the get_message method again
    let result = context.call(|| invoke("get_message", vec![]));
    assert_eq!(result.as_byte_string().unwrap().to_string(), "Hello, World!");
}
```

### 2. Run Tests

Run the tests:

```bash
cargo test
```

## Deploying to NEO N3 Blockchain

To deploy your contract to the NEO N3 blockchain, you need to use the NEO N3 CLI or SDK.

### Using NEO CLI

1. **Install NEO N3 CLI**
   - Follow the instructions at [NEO CLI GitHub](https://github.com/neo-project/neo-cli)

2. **Deploy the Contract**
   - Use the `deploy` command with your `.nef` and `.manifest.json` files:
     ```bash
     neo-cli deploy my_contract.nef my_contract.manifest.json
     ```

3. **Invoke the Contract**
   - Use the `invoke` command:
     ```bash
     neo-cli invoke <contract-hash> update_message '"New message"'
     ```

## Best Practices

### 1. Storage

- Use the `#[storage]` attribute to define your contract storage
- Use descriptive names for storage items
- Consider storage layouts carefully to minimize gas costs

### 2. Events

- Use the `#[event]` attribute to define events
- Mark fields that should be indexed with the `#[index]` attribute
- Emit events for all important state changes

### 3. Error Handling

- Use proper error handling with the `Result` type
- Return meaningful error messages
- Use `unwrap_or()` with a default value for non-critical errors

### 4. Security

- Implement proper access control
- Validate all inputs
- Guard against reentrancy attacks

## Troubleshooting

### Common Issues

1. **Missing Macros**
   - Make sure you've imported all the necessary macros:
     ```rust
     use neo_macros::{
         contract, contract_author, contract_description, contract_version,
         event, index, storage, constructor, method, safe
     };
     ```

2. **Missing Types**
   - Make sure you've imported all the necessary types:
     ```rust
     use alloc::string::String;
     use alloc::vec::Vec;
     use neo_contract::prelude::*;
     ```

3. **Compilation Errors**
   - Check the [FIXED-ISSUES.md](FIXED-ISSUES.md) for common issues and their solutions

### Getting Help

- Check the [documentation](https://github.com/R3E-Network/neo-contract-rs/tree/main/docs)
- Create an issue on the [GitHub repository](https://github.com/R3E-Network/neo-contract-rs/issues)
- Join the NEO community on [Discord](https://discord.com/invite/neo) 