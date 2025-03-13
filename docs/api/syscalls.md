# Neo N3 Syscalls

This document describes the syscalls available in the Neo N3 contract framework, which provide direct access to the Neo VM's native functions.

## Overview

Syscalls are the interface between Neo smart contracts and the underlying blockchain. They allow contracts to:

- Access blockchain data (blocks, transactions)
- Interact with storage
- Perform cryptographic operations
- Manage contract execution
- Emit events

The framework provides a Rust-friendly interface to these syscalls through the `env::syscall` module.

## Available Syscalls

### Blockchain Access

#### `blockchain_get_height() -> u32`

Returns the current blockchain height.

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_current_height() -> u32 {
    blockchain_get_height()
}
```

#### `blockchain_get_block(hash: &[u8]) -> Vec<u8>`

Retrieves a block by its hash.

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_block_by_hash(hash: H256) -> Vec<u8> {
    blockchain_get_block(hash.as_bytes())
}
```

#### `blockchain_get_transaction(hash: &[u8]) -> Vec<u8>`

Retrieves a transaction by its hash.

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_transaction(hash: H256) -> Vec<u8> {
    blockchain_get_transaction(hash.as_bytes())
}
```

#### `blockchain_get_transaction_height(hash: &[u8]) -> i32`

Gets the height of a transaction.

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_tx_height(hash: H256) -> i32 {
    blockchain_get_transaction_height(hash.as_bytes())
}
```

### Contract Management

#### `contract_create(nef_file: &[u8], manifest: &[u8]) -> Vec<u8>`

Creates a new contract.

```rust
use neo_contract::prelude::*;

#[method]
pub fn deploy_contract(nef_file: Vec<u8>, manifest: Vec<u8>) -> Vec<u8> {
    contract_create(&nef_file, &manifest)
}
```

#### `contract_update(nef_file: &[u8], manifest: &[u8])`

Updates an existing contract.

```rust
use neo_contract::prelude::*;

#[method]
pub fn update_contract(nef_file: Vec<u8>, manifest: Vec<u8>) {
    contract_update(&nef_file, &manifest);
}
```

#### `contract_destroy()`

Destroys the current contract.

```rust
use neo_contract::prelude::*;

#[method]
pub fn self_destruct() {
    contract_destroy();
}
```

### Cryptography

#### `crypto_check_sig(pubkey: &[u8], signature: &[u8], message: &[u8]) -> bool`

Verifies a signature against a public key and message.

```rust
use neo_contract::prelude::*;

#[method]
pub fn verify_signature(pubkey: Vec<u8>, signature: Vec<u8>, message: Vec<u8>) -> bool {
    crypto_check_sig(&pubkey, &signature, &message)
}
```

#### `crypto_sha256(data: &[u8]) -> Vec<u8>`

Computes the SHA-256 hash of data.

```rust
use neo_contract::prelude::*;

#[method]
pub fn hash_data(data: Vec<u8>) -> H256 {
    let hash_bytes = crypto_sha256(&data);
    H256::from_slice(&hash_bytes)
}
```

### Runtime

#### `runtime_check_witness(hash_or_pubkey: &[u8]) -> bool`

Verifies that the transaction sender is the owner of the script hash or public key.

```rust
use neo_contract::prelude::*;

#[method]
pub fn check_authorization(address: Address) -> bool {
    runtime_check_witness(address.as_bytes())
}
```

#### `runtime_notify(event_name: &[u8], data: &[u8])`

Emits an event.

```rust
use neo_contract::prelude::*;

#[method]
pub fn emit_event(name: String, data: Vec<u8>) {
    runtime_notify(name.as_bytes(), &data);
}
```

#### `runtime_get_random() -> u64`

Gets a random number from the blockchain.

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_random_number() -> u64 {
    runtime_get_random()
}
```

#### `runtime_platform() -> Vec<u8>`

Gets the current execution platform ("NEO").

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_platform() -> String {
    let platform_bytes = runtime_platform();
    String::from_utf8(platform_bytes).unwrap_or_default()
}
```

### Storage

#### `storage_put(ctx: &[u8], key: &[u8], value: &[u8])`

Stores a value in contract storage.

```rust
use neo_contract::prelude::*;

#[method]
pub fn save_data(key: String, value: Vec<u8>) {
    let ctx = storage_get_context();
    storage_put(&ctx, key.as_bytes(), &value);
}
```

#### `storage_get(ctx: &[u8], key: &[u8]) -> Vec<u8>`

Retrieves a value from contract storage.

```rust
use neo_contract::prelude::*;

#[method]
pub fn load_data(key: String) -> Vec<u8> {
    let ctx = storage_get_context();
    storage_get(&ctx, key.as_bytes())
}
```

#### `storage_delete(ctx: &[u8], key: &[u8])`

Deletes a value from contract storage.

```rust
use neo_contract::prelude::*;

#[method]
pub fn remove_data(key: String) {
    let ctx = storage_get_context();
    storage_delete(&ctx, key.as_bytes());
}
```

#### `storage_get_context() -> Vec<u8>`

Gets the current storage context.

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_storage_context() -> Vec<u8> {
    storage_get_context()
}
```

### Framework Abstractions

The Neo N3 contract framework provides higher-level abstractions over these syscalls to make them more convenient to use:

```rust
use neo_contract::prelude::*;

#[contract]
mod token {
    use super::*;
    
    #[storage]
    pub struct TokenContract {
        // Instead of using raw storage syscalls
        balances: StorageMap<Address, u64>,
    }
    
    impl TokenContract {
        #[method]
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Instead of calling runtime_check_witness directly
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Instead of manually formatting event data
            self.emit_transfer(Some(from), Some(to), amount);
            
            true
        }
        
        fn emit_transfer(&self, from: Option<Address>, to: Option<Address>, amount: u64) {
            // High-level event emission
            let event_name = ByteString::from("Transfer");
            let mut event_data = Array::<Any>::new();
            
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }
            
            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }
            
            event_data.push(Any::from(amount));
            
            // Uses runtime_notify under the hood
            Runtime::notify(&event_name, &event_data);
        }
    }
}
```

## Contract Interactions

### `contract_call(hash: &[u8], method: &[u8], args: &[Any], call_flags: u32) -> Vec<u8>`

Calls a method on another contract.

```rust
use neo_contract::prelude::*;

#[method]
pub fn call_another_contract(contract_hash: H160, amount: u64) -> bool {
    let mut args = Array::<Any>::new();
    args.push(Any::from(Self::owner()));
    args.push(Any::from(amount));
    
    // Call the 'transfer' method on another contract
    let result = contract_call(
        contract_hash.as_bytes(),
        b"transfer",
        &args,
        CallFlags::ALL
    );
    
    // Process the result
    if result.is_empty() {
        return false;
    }
    true
}
```

### `contract_update(nef_file: &[u8], manifest: &[u8])`

Updates the current contract with new code.

```rust
use neo_contract::prelude::*;

#[method]
pub fn update_contract(nef_file: Vec<u8>, manifest: Vec<u8>) -> bool {
    // First check if caller is authorized
    if !Runtime::check_witness(&Self::owner()) {
        return false;
    }
    
    // Update the contract
    contract_update(&nef_file, &manifest);
    true
}
```

## Native Contracts

Neo N3 provides several native contracts that are accessible through syscalls:

- **ContractManagement**: For deploying, updating, and destroying contracts
- **StdLib**: Standard library functions
- **CryptoLib**: Cryptographic operations
- **LedgerContract**: Access to blockchain data
- **NeoToken**: Native NEO token
- **GasToken**: Native GAS token

Example of calling a native contract:

```rust
use neo_contract::prelude::*;

#[method]
pub fn get_neo_balance(account: Address) -> u64 {
    let neo_token = NeoToken::instance();
    neo_token.balance_of(&account)
}
```

## Platform-Specific Considerations

### no_std Environment

The Neo N3 contract framework operates in a `no_std` environment, which means it doesn't have access to the standard library. Instead, it uses the `alloc` crate for basic memory management.

Key considerations for `no_std` compatibility:

- Always use `alloc::vec::Vec` instead of `std::vec::Vec`
- Use `alloc::string::String` instead of `std::string::String`
- Replace `std::collections` with `alloc::collections`
- Use core utilities like `core::ptr`, `core::str`, and `core::option` instead of their std equivalents

```rust
// Example of no_std compatible code in Neo contracts
use alloc::vec::Vec;
use alloc::string::String;

#[method]
pub fn process_data(input: Vec<u8>) -> String {
    // Process using only no_std compatible code
    let mut result = Vec::new();
    for byte in input.iter() {
        result.push(byte + 1);
    }
    
    // Safe conversion with error handling
    String::from_utf8(result).unwrap_or_else(|_| String::from("Invalid UTF-8"))
}
```

### wasm32 Target

The framework is designed to compile to WebAssembly (`wasm32-unknown-unknown` target) before being converted to Neo VM bytecode. Some syscalls may have different implementations for the WASM target.

```rust
#[cfg(target_arch = "wasm32")]
fn blockchain_get_height() -> u32 {
    // WASM implementation
}

#[cfg(not(target_arch = "wasm32"))]
fn blockchain_get_height() -> u32 {
    // Non-WASM implementation (for testing)
}
```

### Gas Costs

Different syscalls have different GAS costs. Be aware of these costs when designing your contract:

- Storage operations are relatively expensive
- Cryptographic operations can be costly
- Blockchain data access has moderate costs
- Simple runtime operations are inexpensive

## Best Practices

1. **Use Abstractions**: Prefer the high-level abstractions provided by the framework over raw syscalls
2. **Error Handling**: Handle errors from syscalls gracefully
3. **Gas Optimization**: Be mindful of the GAS costs of different syscalls
4. **Security**: Always use `runtime_check_witness` to verify transaction signers
5. **Testing**: Test syscalls in a simulated environment before deployment
