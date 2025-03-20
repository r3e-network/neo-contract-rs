# Troubleshooting Guide for NEO Rust Contract Framework

This guide addresses common issues you might encounter when developing smart contracts with the NEO Rust Contract Framework.

## Compilation Issues

### 1. Macro Import Errors

**Issue**: Errors like `unresolved import neo_macros` when trying to use macros.

**Solution**: The macros are not properly exposed in the current version. Use a manual implementation approach:

```rust
// Instead of:
#[contract]
#[contract_author("Your Name")]
pub struct MyContract {
    // ...
}

// Use a manual approach:
mod my_contract {
    // Contract implementation
}

#[no_mangle]
pub fn deploying() -> bool {
    // Initialization code
    true
}

#[no_mangle]
pub fn invoke(action: String, args: Vec<Any>) -> Any {
    // Method dispatch logic
}
```

### 2. Storage Issues

**Issue**: Cannot use `StorageItem` or other storage abstractions.

**Solution**: Implement a manual storage approach:

```rust
pub struct ContractStorage {
    data: Vec<u8>,
    counter: u32,
}

impl ContractStorage {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            counter: 0,
        }
    }
    
    fn save_data(&mut self, data: &[u8]) {
        self.data = data.to_vec();
    }
    
    fn load_data(&self) -> &[u8] {
        &self.data
    }
}
```

### 3. Event Emission Errors

**Issue**: Cannot use the `emit()` method on event structs.

**Solution**: Manually construct and emit events:

```rust
// Instead of:
Transfer { from, to, amount }.emit();

// Use:
let mut event_args = Array::new();
event_args.push(Any::from(from));
event_args.push(Any::from(to));
event_args.push(Any::integer(amount));

Runtime::notify(&ByteString::from("Transfer"), &event_args);
```

### 4. String Conversion Issues

**Issue**: Cannot convert directly between `ByteString` and Rust's `String`.

**Solution**: Use helper functions:

```rust
fn byte_string_to_string(bs: &ByteString) -> String {
    String::from_utf8_lossy(bs.as_bytes()).into_owned()
}

fn string_to_byte_string(s: &str) -> ByteString {
    ByteString::from(s)
}
```

### 5. Type Conversion Issues

**Issue**: Type conversion errors when working with NEO types.

**Solution**: Use explicit conversion methods:

```rust
// Converting between types
let bs: ByteString = ByteString::from("Hello");
let bytes: Vec<u8> = bs.as_bytes().to_vec();
let str_value: String = String::from_utf8_lossy(bs.as_bytes()).into_owned();
let any_value: Any = Any::byte_string(bs);
```

## Runtime Issues

### 1. NEO VM Compilation Errors

**Issue**: Errors when compiling to NEO VM bytecode.

**Solution**: 
1. Check that your WASM is correctly generated
2. Ensure you're using compatible types
3. Run with the `--debug` flag to get more information:
   ```
   neo-compiler compile path/to/your_contract.wasm --output build/ --debug
   ```

### 2. Authentication Issues

**Issue**: Permission problems when contract is deployed.

**Solution**: Always use proper authentication checks:

```rust
// Check if caller is authorized
assert!(Runtime::check_witness(&owner), "Not authorized");
```

### 3. Serialization Issues

**Issue**: Problems with serializing/deserializing data.

**Solution**: Use explicit byte conversions and avoid complex types:

```rust
// Store data as bytes
let data_bytes = data.as_bytes().to_vec();

// Load and convert back
let loaded_string = String::from_utf8_lossy(&data_bytes).into_owned();
```

## Development Workflow Issues

### 1. Creating a New Contract

**Issue**: Unsure of the proper project structure.

**Solution**: Follow this template:
1. Create a library crate: `cargo new --lib my_contract`
2. Add no_std and wasm target: `rustup target add wasm32-unknown-unknown`
3. Update Cargo.toml with correct dependencies
4. Implement the contract with proper entry points
5. Build with: `cargo build --release --target wasm32-unknown-unknown`
6. Compile for NEO VM: `neo-compiler compile target/.../my_contract.wasm --output build/`

### 2. Testing Contracts

**Issue**: Difficulty testing contracts without deployment.

**Solution**: Create unit tests for logic when possible, then use the testing framework for integration testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_business_logic() {
        // Test core business logic independent of blockchain
        let mut contract = MyContract::new();
        assert_eq!(contract.calculate_value(10), 42);
    }
}
```

### 3. Debugging Tips

1. Add debug output in your contract during development
2. Use the `--debug` flag when compiling
3. Test functionality in small, isolated units
4. Check for common issues like incorrect type conversions
5. Review the NEO VM script to understand the compiled code

## Additional Resources

- See [DEVELOPMENT-GUIDE.md](DEVELOPMENT-GUIDE.md) for more detailed development instructions
- See [FUTURE-IMPROVEMENTS.md](FUTURE-IMPROVEMENTS.md) for upcoming features that will address some of these limitations
- Review the example contracts for working patterns and approaches 