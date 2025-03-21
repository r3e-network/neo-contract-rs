# Unit Testing Templates for neo-contract-rs

This document provides templates and strategies for implementing unit tests for the neo-contract-rs framework.

## Testing Strategy

Unit tests should be implemented for each module and type in the framework to ensure correctness and reliability. These tests should be structured to:

1. Test each function with valid inputs
2. Test error conditions with invalid inputs
3. Test boundary conditions
4. Test interactions between related components

## Core Types Testing Templates

### ByteString Tests

```rust
#[cfg(test)]
mod byte_string_tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        let empty = ByteString::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
    }
    
    #[test]
    fn test_from_str() {
        let bs = ByteString::from("hello");
        assert_eq!(bs.len(), 5);
        assert_eq!(bs.as_bytes(), b"hello");
    }
    
    #[test]
    fn test_with_bytes() {
        let bytes = [1, 2, 3, 4, 5];
        let bs = ByteString::with_bytes(&bytes);
        assert_eq!(bs.len(), 5);
        assert_eq!(bs.as_bytes(), &bytes);
    }
    
    #[test]
    fn test_concat() {
        let a = ByteString::from("Hello, ");
        let b = ByteString::from("World!");
        let c = a.concat(&b);
        assert_eq!(c.as_bytes(), b"Hello, World!");
    }
    
    #[test]
    fn test_equals() {
        let a = ByteString::from("test");
        let b = ByteString::from("test");
        let c = ByteString::from("different");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
```

### H160 Tests

```rust
#[cfg(test)]
mod h160_tests {
    use super::*;
    
    #[test]
    fn test_zero() {
        let zero = H160::zero();
        assert_eq!(zero.to_bytes(), [0; 20]);
    }
    
    #[test]
    fn test_from_bytes() {
        let bytes = [1; 20];
        let h160 = H160::from_bytes(&bytes);
        assert_eq!(h160.to_bytes(), bytes);
    }
    
    #[test]
    fn test_equals() {
        let a = H160::from_bytes(&[1; 20]);
        let b = H160::from_bytes(&[1; 20]);
        let c = H160::from_bytes(&[2; 20]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
```

### Int256 Tests

```rust
#[cfg(test)]
mod int256_tests {
    use super::*;
    
    #[test]
    fn test_zero() {
        let zero = Int256::zero();
        assert!(zero.is_zero());
        assert!(!zero.is_negative());
        assert!(!zero.is_positive());
    }
    
    #[test]
    fn test_from_i32() {
        let n = Int256::from_i32(42);
        assert_eq!(n.to_i32(), 42);
        assert!(n.is_positive());
        
        let n = Int256::from_i32(-42);
        assert_eq!(n.to_i32(), -42);
        assert!(n.is_negative());
    }
    
    #[test]
    fn test_checked_add() {
        let a = Int256::from_i32(40);
        let b = Int256::from_i32(2);
        let c = a.checked_add(&b);
        assert_eq!(c.to_i32(), 42);
    }
    
    #[test]
    fn test_checked_sub() {
        let a = Int256::from_i32(44);
        let b = Int256::from_i32(2);
        let c = a.checked_sub(&b);
        assert_eq!(c.to_i32(), 42);
    }
    
    #[test]
    fn test_checked_mul() {
        let a = Int256::from_i32(21);
        let b = Int256::from_i32(2);
        let c = a.checked_mul(&b);
        assert_eq!(c.to_i32(), 42);
    }
    
    #[test]
    fn test_checked_div() {
        let a = Int256::from_i32(84);
        let b = Int256::from_i32(2);
        let c = a.checked_div(&b);
        assert_eq!(c.to_i32(), 42);
    }
    
    #[test]
    fn test_checked_neg() {
        let a = Int256::from_i32(42);
        let b = a.checked_neg();
        assert_eq!(b.to_i32(), -42);
    }
    
    #[test]
    fn test_conversion() {
        let n = Int256::from_i32(42);
        let bs = n.into_byte_string();
        let m = Int256::from_byte_string(bs);
        assert_eq!(n, m);
    }
}
```

### Array Tests

```rust
#[cfg(test)]
mod array_tests {
    use super::*;
    
    #[test]
    fn test_new() {
        let array = Array::<i32>::new();
        assert_eq!(array.len(), 0);
    }
    
    #[test]
    fn test_with_capacity() {
        let array = Array::<i32>::with_capacity(10);
        assert_eq!(array.len(), 0);
        // Additional capacity tests if available
    }
    
    #[test]
    fn test_push_get() {
        let mut array = Array::<i32>::new();
        array.push(42);
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0), 42);
    }
    
    #[test]
    fn test_set() {
        let mut array = Array::<i32>::new();
        array.push(0);
        array.set(0, 42);
        assert_eq!(array.get(0), 42);
    }
}
```

## Storage Testing Templates

### StorageMap Tests

```rust
#[cfg(test)]
mod storage_map_tests {
    use super::*;
    
    // Note: In a real testing environment, we would need to mock the storage system
    // This is a template for when we have a mock implementation
    
    #[test]
    fn test_put_get() {
        let mut mock_env = MockNeoEnvironment::new();
        let mut storage = StorageMap::new_with_env(&mut mock_env);
        
        let key = ByteString::from("test_key");
        let value = ByteString::from("test_value");
        
        storage.put(key.clone(), value.clone());
        let retrieved = storage.get(key.clone());
        
        assert!(!retrieved.is_null());
        assert_eq!(retrieved.unwrap(), value);
    }
    
    #[test]
    fn test_delete() {
        let mut mock_env = MockNeoEnvironment::new();
        let mut storage = StorageMap::new_with_env(&mut mock_env);
        
        let key = ByteString::from("test_key");
        let value = ByteString::from("test_value");
        
        storage.put(key.clone(), value);
        storage.delete(key.clone());
        
        let retrieved = storage.get(key.clone());
        assert!(retrieved.is_null());
    }
    
    #[test]
    fn test_contains_key() {
        let mut mock_env = MockNeoEnvironment::new();
        let mut storage = StorageMap::new_with_env(&mut mock_env);
        
        let key = ByteString::from("test_key");
        let value = ByteString::from("test_value");
        
        assert!(!storage.contains_key(key.clone()));
        
        storage.put(key.clone(), value);
        assert!(storage.contains_key(key.clone()));
    }
}
```

## Runtime Testing Templates

### Runtime Function Tests

```rust
#[cfg(test)]
mod runtime_tests {
    use super::*;
    
    // Note: These tests require a mock runtime environment
    
    #[test]
    fn test_check_witness() {
        let mut mock_env = MockNeoEnvironment::new();
        let account = H160::from_bytes(&[1; 20]);
        
        // Set up witness in mock environment
        mock_env.add_witness(account.clone());
        
        assert!(runtime::check_witness_with_env(&mock_env, account));
        
        let other_account = H160::from_bytes(&[2; 20]);
        assert!(!runtime::check_witness_with_env(&mock_env, other_account));
    }
    
    #[test]
    fn test_notify() {
        let mut mock_env = MockNeoEnvironment::new();
        let event_name = ByteString::from("TestEvent");
        let event_data = ByteString::from("TestData");
        
        runtime::notify_with_env(&mut mock_env, event_name.clone(), event_data.clone());
        
        // Verify notification was sent
        let notifications = mock_env.get_notifications();
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].name, event_name);
        assert_eq!(notifications[0].data, event_data);
    }
    
    #[test]
    fn test_timestamp() {
        let mut mock_env = MockNeoEnvironment::new();
        mock_env.set_timestamp(1234567890);
        
        assert_eq!(runtime::get_timestamp_with_env(&mock_env), 1234567890);
    }
}
```

## Contract Testing Templates

### Contract Macro Tests

```rust
#[cfg(test)]
mod contract_macro_tests {
    use super::*;
    
    // Define a test contract
    pub struct TestContract;
    
    #[neo::contract]
    impl TestContract {
        pub fn test_method(input: i32) -> i32 {
            input * 2
        }
        
        pub fn test_byte_string(input: ByteString) -> ByteString {
            let mut result = ByteString::from("Result: ");
            result = result.concat(&input);
            result
        }
    }
    
    #[test]
    fn test_contract_methods() {
        // Test that the contract methods work as expected
        assert_eq!(TestContract::test_method(21), 42);
        
        let input = ByteString::from("test");
        let expected = ByteString::from("Result: test");
        assert_eq!(TestContract::test_byte_string(input), expected);
    }
}
```

## Serialization Testing Templates

### Structs Macro Tests

```rust
#[cfg(test)]
mod structs_macro_tests {
    use super::*;
    
    #[neo::structs]
    struct TestStruct {
        field1: i32,
        field2: ByteString,
    }
    
    #[test]
    fn test_struct_serialization() {
        let test_struct = TestStruct {
            field1: 42,
            field2: ByteString::from("test"),
        };
        
        // Serialize
        let serialized = test_struct.serialize();
        
        // Deserialize
        let deserialized = TestStruct::deserialize(serialized).unwrap();
        
        // Verify equality
        assert_eq!(deserialized.field1, test_struct.field1);
        assert_eq!(deserialized.field2, test_struct.field2);
    }
}
```

## Implementing a Mock Neo Environment

To enable effective unit testing, we need to create a mock Neo environment:

```rust
// A mock implementation of the Neo environment for testing
pub struct MockNeoEnvironment {
    storage: std::collections::HashMap<Vec<u8>, Vec<u8>>,
    witnesses: Vec<H160>,
    timestamp: u64,
    notifications: Vec<Notification>,
    // Add other state as needed
}

pub struct Notification {
    pub name: ByteString,
    pub data: Any,
}

impl MockNeoEnvironment {
    pub fn new() -> Self {
        Self {
            storage: std::collections::HashMap::new(),
            witnesses: Vec::new(),
            timestamp: 0,
            notifications: Vec::new(),
        }
    }
    
    // Storage operations
    pub fn storage_put(&mut self, key: &[u8], value: &[u8]) {
        self.storage.insert(key.to_vec(), value.to_vec());
    }
    
    pub fn storage_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }
    
    pub fn storage_delete(&mut self, key: &[u8]) {
        self.storage.remove(key);
    }
    
    pub fn storage_contains(&self, key: &[u8]) -> bool {
        self.storage.contains_key(key)
    }
    
    // Witness operations
    pub fn add_witness(&mut self, account: H160) {
        self.witnesses.push(account);
    }
    
    pub fn has_witness(&self, account: H160) -> bool {
        self.witnesses.contains(&account)
    }
    
    // Time operations
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
    
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    
    // Notification operations
    pub fn add_notification(&mut self, name: ByteString, data: Any) {
        self.notifications.push(Notification { name, data });
    }
    
    pub fn get_notifications(&self) -> &[Notification] {
        &self.notifications
    }
}

// Extension traits for framework types to work with mock environment

trait WithMockEnv {
    fn new_with_env(env: &mut MockNeoEnvironment) -> Self;
}

impl WithMockEnv for StorageMap {
    fn new_with_env(env: &mut MockNeoEnvironment) -> Self {
        // Implementation that uses mock environment instead of Neo VM
        // This is a placeholder - actual implementation would depend on StorageMap internals
        StorageMap { env: env }
    }
}

// Extension functions for runtime operations with mock environment
impl runtime {
    pub fn check_witness_with_env(env: &MockNeoEnvironment, account: H160) -> bool {
        env.has_witness(account)
    }
    
    pub fn notify_with_env(env: &mut MockNeoEnvironment, name: ByteString, data: Any) {
        env.add_notification(name, data);
    }
    
    pub fn get_timestamp_with_env(env: &MockNeoEnvironment) -> u64 {
        env.get_timestamp()
    }
}
```

## Test Coverage Goals

For each module in the neo-contract-rs framework, aim for:

1. **ByteString**: 100% method coverage
2. **H160**: 100% method coverage
3. **Int256**: 100% method coverage with special attention to boundary cases
4. **Array**: 100% method coverage
5. **StorageMap**: 100% method coverage
6. **Runtime functions**: 100% coverage
7. **Contract macro**: Test at least 3 representative cases
8. **Structs macro**: Test simple and complex struct cases

## Integration with Cargo Test

To run tests:

```bash
cargo test
```

To run tests with coverage reports (requires cargo-tarpaulin):

```bash
cargo install cargo-tarpaulin
cargo tarpaulin
```

## Future Testing Improvements

1. Develop a comprehensive Neo VM mock environment
2. Create test utilities for common testing patterns
3. Add property-based testing with frameworks like proptest
4. Create integration test harnesses for deployed contracts 