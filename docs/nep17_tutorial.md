# Creating a NEP-17 Token Contract

This tutorial walks you through creating a basic NEP-17 token contract using neo-contract-rs.

## What is NEP-17?

NEP-17 is the Neo N3 token standard for fungible tokens, similar to ERC-20 on Ethereum. It defines a set of methods that all token contracts must implement to ensure compatibility with wallets, exchanges, and other services.

## Safe vs. Non-Safe Methods

In Neo N3 smart contracts, methods can be categorized as:

- **Safe Methods**: Read-only methods that don't modify contract state
- **Non-Safe Methods**: Methods that can modify contract state

Safe methods are marked with the `#[safe]` attribute when using attribute macros. This information is translated into the contract manifest, allowing the Neo Virtual Machine to optimize execution of these methods.

When implementing your token contract, it's important to mark all read-only methods as safe.

## Prerequisites

Before you start, make sure you have:

- Rust installed (https://rustup.rs/)
- WebAssembly target added: `rustup target add wasm32-unknown-unknown`
- Basic understanding of Rust and smart contracts

## Step 1: Create a new Rust project

```bash
cargo new --lib my_token
cd my_token
```

## Step 2: Configure your project

Update your `Cargo.toml`:

```toml
[package]
name = "my_token"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { git = "https://github.com/R3E-Network/neo-contract-rs" }
wee_alloc = "0.4.5"
```

## Step 3: Create your token contract

Create a new file `src/lib.rs` with the following content:

```rust
// Basic NEP-17 Token implementation
#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Array, Any},
    runtime::Runtime,
    contract_method, smart_contract,
    nep17::*,
};

use alloc::vec::Vec;
use core::panic::PanicInfo;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Define a panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Token structure
pub struct Token;

#[smart_contract]
impl Token {
    // Event emitted when tokens are transferred
    pub fn Transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {
        // This event is automatically emitted via Runtime::notify
    }
    
    // Token properties
    contract_method!(pub fn name() -> ByteString {
        ByteString::from("My Token")
    });
    
    contract_method!(pub fn symbol() -> ByteString {
        ByteString::from("MTK")
    });
    
    contract_method!(pub fn decimals() -> u8 {
        8 // 8 decimals for precision
    });
    
    // Read-only methods should be marked with #[safe] when using attribute macros
    contract_method!(pub fn total_supply() -> Int256 {
        // Supply limit of 100,000,000 tokens
        Int256::from(100_000_000) * Int256::from(10).pow(8)
    });
    
    // Read-only methods should be marked with #[safe] when using attribute macros
    contract_method!(pub fn balance_of(account: H160) -> Int256 {
        // In a real implementation, we would look up the account balance
        // For this simple example, we just return a fixed value
        if account == Runtime::executing_script_hash() {
            return Int256::from(1000);
        }
        Int256::zero()
    });
    
    // Transfer modifies state, so it should not be marked as safe
    contract_method!(pub fn transfer(from: H160, to: H160, amount: Int256, data: ByteString) -> bool {
        // Check that the caller is authorized to spend these tokens
        if !Runtime::check_witness(from.clone()) {
            return false;
        }
        
        // Check for valid amount
        if amount <= Int256::zero() {
            return false;
        }
        
        // Get current balances
        let from_balance = Self::balance_of(from.clone());
        
        // Ensure sufficient funds
        if from_balance < amount {
            return false;
        }
        
        // In a real implementation, we would update balances here
        // ...
        
        // Emit transfer event
        let mut args = Array::new();
        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));
        
        Runtime::notify(
            &ByteString::from("Transfer"),
            &args
        );
        
        true
    });
    
    // Contract lifecycle methods
    contract_method!(pub fn deploy(data: bool) -> bool {
        // This is called when the contract is deployed
        // Initialize token supply, etc.
        
        // Transfer initial supply to the contract creator
        let sender = Runtime::calling_script_hash();
        
        // Emit transfer event (minting)
        let mut args = Array::new();
        args.push(Any::null()); // null address for minting
        args.push(Any::from(sender));
        args.push(Any::from(Self::total_supply()));
        
        Runtime::notify(
            &ByteString::from("Transfer"),
            &args
        );
        
        true
    });
    
    contract_method!(pub fn initialize() -> bool {
        // Additional initialization if needed
        true
    });
}

// Implement the NEP17 trait for the Token
impl NEP17 for Token {
    fn symbol(&self) -> ByteString {
        ByteString::from("MTK")
    }
    
    fn decimals(&self) -> u8 {
        8
    }
    
    fn total_supply(&self) -> Int256 {
        // 100,000,000 tokens with 8 decimals
        Int256::from(100_000_000) * Int256::from(10).pow(8)
    }
    
    fn balance_of(&self, account: H160) -> Int256 {
        // Simple implementation
        if account == Runtime::executing_script_hash() {
            return Int256::from(1000);
        }
        Int256::zero()
    }
    
    fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: ByteString) -> bool {
        // Check that the caller is authorized to spend these tokens
        if !Runtime::check_witness(from.clone()) {
            return false;
        }
        
        // Check for valid amount
        if amount <= Int256::zero() {
            return false;
        }
        
        // Rest of the implementation...
        true
    }
}

## Step 4: Build your contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

This will produce a WebAssembly binary at `target/wasm32-unknown-unknown/release/my_token.wasm`.

## Step 5: Deploy your contract

To deploy your contract to the Neo N3 blockchain, you'll need to:

1. Convert the .wasm file to a .nef file using the Neo compiler
2. Create a manifest file
3. Deploy using the Neo CLI or other deployment tool

Note: Deployment tools for Rust-based Neo contracts are under development. In the meantime, you can use the Neo CLI to deploy the contract.

## Next Steps

- Add persistent storage to track token balances
- Implement token transfer functionality
- Add more advanced features like allowances for delegated transfers
- Test your contract on a Neo N3 TestNet before deploying to MainNet

## Further Reading

- [NEP-17 Standard](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki)
- [Neo N3 Documentation](https://docs.neo.org/)
- [Neo Contract Examples](https://github.com/R3E-Network/neo-contract-rs/tree/main/examples)
