# Hello World Contract

A simple Neo N3 smart contract example written in Rust using the neo-contract-rs framework.

## Overview

This example demonstrates the basic structure and functionality of a Neo smart contract written in Rust. It implements:

1. A greeting function that returns customized messages
2. Contract information function
3. Basic storage operations to store and retrieve greetings

## Contract Methods

### `hello(name: ByteString) -> ByteString`

Returns a greeting message:
- If called with an empty string, returns "Hello, World!"
- If called with a name, returns "Hello, {name}!"

Example:
```
hello("Neo") => "Hello, Neo!"
```

### `contract_info() -> ByteString`

Returns information about the contract.

### `store_greeting(name: ByteString, message: ByteString)`

Stores a custom greeting message for a given name in the contract's storage.

### `get_greeting(name: ByteString) -> ByteString`

Retrieves a previously stored greeting message for a given name.

## Building the Contract

You can build the contract using:

```bash
# Make sure you have the wasm32-unknown-unknown target installed
rustup target add wasm32-unknown-unknown

# Build using make
make

# Or build manually
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/hello_world.wasm ./hello_world.wasm
```

## Deploying the Contract

After building, you can deploy the contract to a Neo N3 network using Neo-CLI or other Neo deployment tools.

### Using Neo-CLI

```bash
neo-cli deploy hello_world.wasm
```

## Interacting with the Contract

Once deployed, you can interact with the contract using Neo-CLI, the Neo SDK, or through a Neo wallet that supports dApps.

### Example Invocations

```bash
# Say hello
neo-cli invoke <contract-hash> hello ["Neo"]

# Store a custom greeting
neo-cli invoke <contract-hash> store_greeting ["Alice","Welcome to Neo!"]

# Retrieve the stored greeting
neo-cli invoke <contract-hash> get_greeting ["Alice"]
```

## Learning Points

This example demonstrates:

1. Basic Neo contract structure using Rust
2. Using Neo types like ByteString
3. Reading and writing to contract storage
4. Defining and exposing contract methods
5. Proper documentation with rustdoc comments 