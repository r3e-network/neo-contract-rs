# Troubleshooting Neo Contract Rust Framework

This guide helps resolve common issues encountered when working with the Neo Contract Rust framework.

## Common Compilation Errors

### 1. Missing Parameters in Runtime Function Calls

**Error Example:**
```
expected 1 argument, found 0
```

**Problem:**
Functions like `Runtime::check_witness()` are called without required parameters.

**Solution:**
Update the function calls to include the required arguments. For example:

```rust
// Incorrect
let caller = Runtime::check_witness();

// Correct
let caller = Runtime::check_witness(&address);
```

### 2. Procedural Macro Resolution Failures

**Error Example:**
```
failed to resolve: could not find `neo_contract_module` in `prelude`
failed to resolve: could not find `manifest_method` in `prelude`
```

**Solution:**
- Ensure you have the latest version of `neo-macros` in your dependencies
- Add the feature flag `std` during development:
  ```bash
  cargo check -p your-example --features std
  ```
- If needed, modify your import structure:
  ```rust
  // Add explicit imports
  use neo_contract::prelude::*;
  use neo_contract::macros::{contract, method, safe, constructor};
  ```

### 3. Storage Trait Issues

**Error Example:**
```
failed to resolve: could not find `StorageContext` in `storage`
failed to resolve: could not find `Storage` in `prelude`
```

**Solution:**
Add explicit imports for storage-related types:

```rust
use neo_contract::types::context::StorageContext;
use neo_contract::types::storage::Storage;
```

### 4. Codec Implementation for Custom Types

**Error Example:**
```
the trait bound `alloc::string::String: Codec` is not satisfied
```

**Solution:**
Implement the `Codec` trait for custom types that need to be stored:

```rust
impl Codec for MyType {
    fn encode(&self) -> Vec<u8> {
        // Serialization logic here
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        // Deserialization logic here
    }
}
```

For standard types like `String`, use a wrapper or convert to a type that implements `Codec`:

```rust
// Store strings as ByteString which implements Codec
let byte_string = ByteString::from(my_string);
```

### 5. Type Mismatches in Storage Operations

**Error Example:**
```
mismatched types
expected `&u32`, found `u32`
```

**Solution:**
When setting values in storage, make sure to match the expected reference type:

```rust
// Incorrect
self.counter.set(value);

// Correct
self.counter.set(&value);  // Pass by reference when required
```

## Framework-Specific Issues

### 1. DAO Example Check Witness Issues

The DAO example has multiple instances where `Runtime::check_witness()` is called without parameters. To fix:

```rust
// In each method that needs authentication
// Change this:
let caller = Runtime::check_witness();

// To this:
let caller_address = Runtime::current_sender(); // Get caller address
let caller = Runtime::check_witness(&caller_address);
```

### 2. Hello World Example Storage Issues

The Hello World example has issues with `Codec` trait for String storage. To fix:

```rust
// Instead of directly storing strings
self.message.set(message);

// Convert to a known codec type first
let message_bytes = message.into_bytes();
self.message.set(&message_bytes);

// Then when retrieving
let message_bytes = self.message.get().unwrap_or_default();
let message = String::from_utf8(message_bytes).unwrap_or_default();
```

## Development Workflow Tips

1. **Use Feature Flags**:
   ```bash
   cargo check --features std
   cargo build --features std
   ```

2. **Incremental Testing**:
   Test small parts of your contract separately before integrating

3. **Inspect Generated Code**:
   Use `cargo expand` to view the expanded macros:
   ```bash
   cargo install cargo-expand
   cargo expand --features std
   ```

4. **Use Debug Builds First**:
   Only move to release builds after verifying functionality

## Contacting Support

If you continue to face issues:

1. Check the [Neo Discord](https://discord.gg/neo)
2. Open an issue on the [GitHub repository](https://github.com/neo-project/neo-contract-rs)
3. Search for similar issues in the repository's issue tracker

## Contributing Solutions

If you find and fix an issue not covered in this guide, please consider:

1. Opening a pull request with your fix
2. Adding your solution to this troubleshooting guide
3. Adding comments in the code to help others avoid the same issue 