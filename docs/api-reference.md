# neo-contract-rs API Reference

This document provides a reference for the main API components of the neo-contract-rs framework.

## Core Types

### ByteString

A Neo VM-compatible string type for working with byte data.

```rust
// Creating a ByteString
let empty = ByteString::empty();
let from_str = ByteString::from("hello");
let from_bytes = ByteString::with_bytes(&[0x01, 0x02, 0x03]);

// Operations
let len = bytes.len();
let is_empty = bytes.is_empty();
```

### H160

Neo address type (20-byte hash).

```rust
// Creating an H160
let zero_addr = H160::zero();
let from_bytes = H160::from_bytes(&[0; 20]);

// Conversion
let as_bytes = h160.to_bytes();
```

### Int256

256-bit integer type.

```rust
// Creating an Int256
let zero = Int256::zero();
let from_i32 = Int256::from_i32(42);

// Arithmetic operations
let sum = a.checked_add(&b);
let product = a.checked_mul(&b);
```

### Array<T>

Neo VM-compatible array type.

```rust
// Creating an Array
let empty = Array::<i32>::new();
let with_capacity = Array::<ByteString>::with_capacity(10);

// Operations
array.push(item);
let item = array.get(index);
let len = array.len();
```

## Storage

### StorageMap

Abstracts Neo's key-value storage.

```rust
// Create storage context
let mut storage = StorageMap::new();

// Basic operations
storage.put(key, value);
let value = storage.get(key);
storage.delete(key);

// Check if key exists
if storage.contains_key(key) {
    // Key exists
}

// Get value with default
let value = storage.get_or_default(key, default);
```

## Runtime

### Functions

```rust
// Contract execution context
let calling_script_hash = runtime::calling_script_hash();
let executing_script_hash = runtime::executing_script_hash();
let entry_script_hash = runtime::entry_script_hash();

// Check witness
let is_witness = runtime::check_witness(account);

// Gas operations
let gas_left = runtime::gas_left();

// Platform info
let platform = runtime::platform();
let trigger = runtime::trigger();

// Random number generation
let random = runtime::get_random();

// Time
let timestamp = runtime::get_timestamp();

// Notifications
runtime::notify(event_name, data);
```

## Contract

### Attribute Macros

```rust
// Contract implementation
#[neo::contract]
impl MyContract {
    pub fn my_method(arg1: i32, arg2: ByteString) -> bool {
        // Contract implementation
    }
}

// Custom struct serialization
#[neo::structs]
struct MyStruct {
    field1: i32,
    field2: ByteString,
}
```

### Predefined Contract Interfaces

#### NEP-17 Token Standard

```rust
#[neo::contract]
impl Nep17Token for MyToken {
    fn symbol() -> ByteString {
        ByteString::from("TKN")
    }

    fn decimals() -> u32 {
        8
    }

    // Optionally override default implementations
    fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        // Custom transfer logic
    }
}
```

## Crypto

### Functions

```rust
// SHA-256 hash
let hash = crypto::sha256(data);

// Verify signature
let is_valid = crypto::verify_signature(data, signature, public_key);

// RIPEMD-160 hash
let ripemd_hash = crypto::ripemd160(data);
```

## Events

### Emitting Events

```rust
// Simple notification
runtime::notify(event_name, data);

// Structured event
#[neo::structs]
struct TransferEvent {
    from: H160,
    to: H160,
    amount: Int256,
}

let event = TransferEvent { from, to, amount };
runtime::notify(ByteString::from("Transfer"), event);
```

## Environment

Low-level functions for Neo VM interaction (rarely used directly):

```rust
// System call invocation
unsafe { env::syscall::system_runtime_notify(name, data) };

// Contract calls
unsafe { env::syscall::system_contract_call(contract, method, flags, args) };
``` 