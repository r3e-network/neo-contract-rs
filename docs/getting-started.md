# Getting Started with neo-contract-rs

This guide will help you set up your development environment and create your first Neo N3 smart contract using Rust.

## Prerequisites

Before starting, ensure you have:

1. [Rust](https://www.rust-lang.org/tools/install) installed (1.60+ recommended)
2. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed
3. Basic knowledge of Rust and Neo blockchain concepts

## Setting Up Your Environment

### 1. Install the Rust toolchain for WebAssembly

```bash
rustup target add wasm32-unknown-unknown
```

### 2. Clone the neo-contract-rs repository (if developing against the source)

```bash
git clone https://github.com/R3E-Network/neo-contract-rs.git
cd neo-contract-rs
```

## Creating Your First Contract

### 1. Set up a new Rust project

```bash
cargo new --lib my-neo-contract
cd my-neo-contract
```

### 2. Configure your Cargo.toml

Add the following to your Cargo.toml file:

```toml
[package]
name = "my-neo-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { version = "0.1.0", default-features = false, features = ["wasm"] } # Or path dependency if using local source

[features]
default = ["wasm"]
wasm = []

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
```

### 3. Create a minimal contract

Edit `src/lib.rs`:

```rust
#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, types::*};

pub struct MyContract;

#[neo::contract]
impl MyContract {
    pub fn hello() -> ByteString {
        ByteString::from("Hello, Neo!")
    }
}
```

### 4. Create a Makefile for easy building

```makefile
# Copyright @ 2024 - present, R3E Network
# All Rights Reserved

PHONY += compile

RUST_FLAGS = "-Ctarget-feature=+multivalue \
    -Cllvm-args=--combiner-store-merging=false \
    -Clink-arg=--initial-memory=262144 \
    -Clink-arg=-zstack-size=131072 \
    --cfg=target_arch=\"wasm32\""

# compile with optimization and proper wasm targeting
compile:
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RUST_FLAGS) cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm
	cp target/wasm32-unknown-unknown/release/my_neo_contract.wasm ./my_neo_contract.wasm

clean:
	cargo clean
```

## Building a NEP-17 Token

Let's create a simple NEP-17 token contract:

```rust
#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, types::*};

pub struct MyToken;

#[neo::contract]
impl Nep17Token for MyToken {
    fn symbol() -> ByteString {
        ByteString::from("MTK")
    }
    
    fn decimals() -> u32 {
        8
    }
    
    fn _initialize() {
        // This is called when the contract is deployed
        let owner = runtime::calling_script_hash();
        
        // Mint initial supply to the owner
        let initial_supply = Int256::from_i32(1_000_000);
        MyToken::mint(owner, initial_supply);
    }
}
```

## Working with Storage

Storage operations are common in smart contracts. Here's how to use storage:

```rust
#[neo::contract]
impl MyContract {
    pub fn set_value(key: ByteString, value: ByteString) {
        let mut storage = StorageMap::new();
        storage.put(key, value);
    }
    
    pub fn get_value(key: ByteString) -> ByteString {
        let storage = StorageMap::new();
        let value = storage.get(key);
        
        if value.is_null() {
            return ByteString::empty();
        }
        
        value.unwrap()
    }
}
```

## Emitting Events

Events allow your contract to notify external systems of important changes:

```rust
#[neo::contract]
impl MyContract {
    pub fn do_something(param: ByteString) {
        // Contract logic...
        
        // Emit an event
        runtime::notify(
            ByteString::from("SomethingHappened"),
            param
        );
    }
}
```

## Testing Your Contract

Neo contracts are compiled to WASM, which makes them difficult to test directly. However, you can use Neo's test frameworks or develop mock testing environments.

A basic approach is to compile your contract with conditional compilation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hello() {
        let result = MyContract::hello();
        assert_eq!(result, ByteString::from("Hello, Neo!"));
    }
}
```

## Deploying Your Contract

Once your contract is compiled to WASM, you can deploy it to the Neo N3 blockchain using:

1. Neo-CLI
2. Neo-GUI
3. Programmatic deployment via SDKs

See the Neo N3 documentation for detailed deployment instructions.

## Next Steps

After creating your first contract, explore:

1. More complex NEP standards (NEP-11 for NFTs)
2. Contract migration patterns
3. Integration with off-chain systems
4. Advanced storage patterns for complex data 