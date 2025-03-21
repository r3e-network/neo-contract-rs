# Simple Storage Contract

A basic key-value storage contract for Neo N3 written in Rust using the neo-contract-rs framework.

## Overview

This contract demonstrates how to implement storage operations in Neo smart contracts. It provides:

1. Basic key-value storage operations (set, get, delete)
2. Existence checking for keys
3. Atomic counter implementation 
4. Batch operations

## Contract Methods

### `set(key: ByteString, value: ByteString)`

Stores a value associated with a key in the contract's storage.

### `get(key: ByteString) -> ByteString`

Retrieves a value associated with a key from the contract's storage.
Returns an empty ByteString if the key doesn't exist.

### `delete(key: ByteString)`

Removes a key-value pair from the contract's storage.

### `has_key(key: ByteString) -> bool`

Checks if a key exists in the contract's storage.
Returns true if the key exists, false otherwise.

### `increment(key: ByteString) -> Int256`

Increments an integer value stored at a given key.
If the key doesn't exist, it initializes it with 1.
Returns the new value after incrementing.

### `set_batch(keys: Array<ByteString>, values: Array<ByteString>) -> bool`

Stores multiple key-value pairs in a single operation.
Returns true if successful, false if the arrays have different lengths.

### `name() -> ByteString`

Returns the name of the contract ("SimpleStorage").

### `contract_info() -> ByteString`

Returns information about the contract.

## Building the Contract

```bash
# Make sure you have the wasm32-unknown-unknown target installed
rustup target add wasm32-unknown-unknown

# Build using make
make

# Or build manually
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/simple_storage.wasm ./simple_storage.wasm
```

## Deploying the Contract

After building, you can deploy the contract to a Neo N3 network using Neo-CLI or other Neo deployment tools:

```bash
neo-cli deploy simple_storage.wasm
```

## Example Usage

Here are some examples of how to interact with the contract:

```bash
# Store a value
neo-cli invoke <contract-hash> set ["myKey","Hello Neo!"]

# Retrieve a value
neo-cli invoke <contract-hash> get ["myKey"]

# Delete a value
neo-cli invoke <contract-hash> delete ["myKey"]

# Check if key exists
neo-cli invoke <contract-hash> has_key ["myKey"]

# Increment a counter
neo-cli invoke <contract-hash> increment ["counter"]

# Store multiple values
neo-cli invoke <contract-hash> set_batch [[\"key1\",\"key2\",\"key3\"],[\"value1\",\"value2\",\"value3\"]]
```

## Storage Patterns

This example demonstrates several important storage patterns:

1. **Basic Storage**: Simple key-value operations
2. **Null Handling**: Safe retrieval with null checks
3. **Atomic Counters**: Incremental storage values
4. **Batch Operations**: Efficient multiple storage operations

## Learning Points

1. Storage operations in Neo smart contracts
2. Working with Neo's ByteString type
3. Type conversion between byte strings and integers
4. Storage existence checking
5. Error handling in storage operations 