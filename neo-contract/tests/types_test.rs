//! Unit tests for Neo contract types

#[cfg(test)]
mod types_tests {
    use neo_contract::types::*;

    #[test]
    fn test_h160_creation_and_conversion() {
        // Test zero address
        let zero = H160::zero();
        assert_eq!(zero.0, [0u8; 20]);

        // Test from bytes
        let bytes = [1u8; 20];
        let addr = H160(bytes);
        assert_eq!(addr.0, bytes);

        // Test into ByteString
        let bs = addr.into_byte_string();
        assert_eq!(bs.len(), 20);
    }

    #[test]
    fn test_int256_arithmetic() {
        let a = Int256::from(100);
        let b = Int256::from(50);

        // Addition
        let sum = a.checked_add(&b).unwrap();
        assert_eq!(sum, Int256::from(150));

        // Subtraction
        let diff = a.checked_sub(&b).unwrap();
        assert_eq!(diff, Int256::from(50));

        // Multiplication
        let product = a.checked_mul(&b).unwrap();
        assert_eq!(product, Int256::from(5000));

        // Division
        let quotient = a.checked_div(&b).unwrap();
        assert_eq!(quotient, Int256::from(2));

        // Modulo
        let remainder = a.checked_rem(&b).unwrap();
        assert_eq!(remainder, Int256::zero());
    }

    #[test]
    fn test_int256_comparison() {
        let a = Int256::from(100);
        let b = Int256::from(50);
        let c = Int256::from(100);

        assert!(a > b);
        assert!(b < a);
        assert!(a >= c);
        assert!(a <= c);
        assert!(a == c);
        assert!(a != b);
    }

    #[test]
    fn test_int256_edge_cases() {
        // Test zero
        let zero = Int256::zero();
        assert_eq!(zero, Int256::from(0));

        // Test one
        let one = Int256::one();
        assert_eq!(one, Int256::from(1));

        // Test negative
        let neg = Int256::from(-100);
        assert!(neg < zero);

        // Test overflow protection
        let max = Int256::from(i64::MAX);
        let result = max.checked_add(&Int256::one());
        assert!(result.is_some()); // Should handle without panic
    }

    #[test]
    fn test_byte_string_operations() {
        // Test from literal
        let bs1 = ByteString::from_literal("hello");
        assert_eq!(bs1.len(), 5);

        // Test concatenation
        let bs2 = ByteString::from_literal(" world");
        let combined = bs1.concat(&bs2);
        assert_eq!(combined.len(), 11);

        // Test from bytes
        let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
        let bs3 = ByteString::from(bytes.as_slice());
        assert_eq!(bs3.len(), 5);

        // Test empty
        let empty = ByteString::new();
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_array_operations() {
        // Create new array
        let mut arr = Array::new();
        assert_eq!(arr.length(), 0);

        // Push elements
        arr.push(Any::from(Int256::from(1)));
        arr.push(Any::from(Int256::from(2)));
        arr.push(Any::from(Int256::from(3)));
        assert_eq!(arr.length(), 3);

        // Get elements
        let first = arr.get(0).and_then(|a| a.as_int());
        assert_eq!(first, Some(Int256::from(1)));

        // Pop element
        let popped = arr.pop().and_then(|a| a.as_int());
        assert_eq!(popped, Some(Int256::from(3)));
        assert_eq!(arr.length(), 2);

        // From vec
        let vec = vec![Any::from(Int256::from(10)), Any::from(Int256::from(20))];
        let arr2 = Array::from_vec(vec);
        assert_eq!(arr2.length(), 2);
    }

    #[test]
    fn test_map_operations() {
        // Create new map
        let mut map = Map::new();
        assert_eq!(map.count(), 0);

        // Set values
        let key1 = ByteString::from_literal("key1");
        let val1 = Any::from(Int256::from(100));
        map.set(key1.clone().into_any(), val1.clone());
        assert_eq!(map.count(), 1);

        // Get values
        let retrieved = map.get(&key1.into_any());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().as_int(), Some(Int256::from(100)));

        // Has key
        assert!(map.has(&key1.into_any()));

        // Remove
        map.remove(&key1.into_any());
        assert_eq!(map.count(), 0);
        assert!(!map.has(&key1.into_any()));
    }

    #[test]
    fn test_public_key_operations() {
        // Create a public key
        let bytes = [0x02; 33]; // Compressed public key format
        let pubkey = PublicKey::from_bytes(&bytes);
        
        // Convert to H160
        let addr = pubkey.to_h160();
        assert_eq!(addr.0.len(), 20);

        // Into Any
        let any = pubkey.into_any();
        assert!(any.as_public_key().is_some());
    }

    #[test]
    fn test_any_type_conversions() {
        // Int conversion
        let int = Int256::from(42);
        let any_int = Any::from(int.clone());
        assert_eq!(any_int.as_int(), Some(int));

        // ByteString conversion
        let bs = ByteString::from_literal("test");
        let any_bs = Any::from(bs.clone());
        assert_eq!(any_bs.as_bytes(), Some(bs));

        // Bool conversion
        let any_bool = Any::from(true);
        assert_eq!(any_bool.as_bool(), Some(true));

        // H160 conversion
        let addr = H160::zero();
        let any_addr = Any::from(addr);
        assert_eq!(any_addr.as_h160(), Some(addr));

        // Array conversion
        let arr = Array::new();
        let any_arr = Any::from(arr.clone());
        assert!(any_arr.as_array().is_some());

        // Null check
        let null = Any::null();
        assert!(null.is_null());
    }

    #[test]
    fn test_bytes_operations() {
        // Create from vec
        let vec = vec![1, 2, 3, 4, 5];
        let bytes = Bytes::from(vec.clone());
        assert_eq!(bytes.len(), 5);

        // Get byte at index
        assert_eq!(bytes.get(0), Some(1));
        assert_eq!(bytes.get(4), Some(5));
        assert_eq!(bytes.get(5), None);

        // Into vec
        let vec2 = bytes.into_vec();
        assert_eq!(vec2, vec);

        // Empty bytes
        let empty = Bytes::new();
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_call_flags() {
        // Test individual flags
        assert_eq!(CallFlags::NONE.0, 0x00);
        assert_eq!(CallFlags::READ_STATES.0, 0x01);
        assert_eq!(CallFlags::STATES.0, 0x02);
        assert_eq!(CallFlags::READ_ONLY.0, 0x03);
        assert_eq!(CallFlags::ALL.0, 0x04);

        // Test flag combinations
        let combined = CallFlags(CallFlags::READ_STATES.0 | CallFlags::STATES.0);
        assert_eq!(combined.0, 0x03);
    }

    #[test]
    fn test_find_options() {
        // Default options
        let default = FindOptions::default();
        assert_eq!(default.0, 0);

        // Values only
        let values = FindOptions::VALUES_ONLY;
        assert_eq!(values.0, 1 << 0);

        // Keys only
        let keys = FindOptions::KEYS_ONLY;
        assert_eq!(keys.0, 1 << 1);

        // Remove prefix
        let remove = FindOptions::REMOVE_PREFIX;
        assert_eq!(remove.0, 1 << 2);

        // Pick fields
        let pick0 = FindOptions::PICK_FIELD_0;
        assert_eq!(pick0.0, 1 << 3);

        let pick1 = FindOptions::PICK_FIELD_1;
        assert_eq!(pick1.0, 1 << 4);
    }

    #[test]
    fn test_trigger_type() {
        // Test trigger types
        assert_eq!(TriggerType::OnPersist as u8, 0x01);
        assert_eq!(TriggerType::PostPersist as u8, 0x02);
        assert_eq!(TriggerType::Verification as u8, 0x20);
        assert_eq!(TriggerType::Application as u8, 0x40);
        assert_eq!(TriggerType::System as u8, 0x01);
        assert_eq!(TriggerType::All as u8, 0x0F);
    }

    #[test]
    fn test_h256_operations() {
        use neo_contract::types::builtin::H256;

        // Test zero
        let zero = H256::zero();
        assert_eq!(zero.0, [0u8; 32]);

        // Test from bytes
        let bytes = [0xFF; 32];
        let hash = H256(bytes);
        assert_eq!(hash.0, bytes);

        // Test equality
        let hash2 = H256(bytes);
        assert_eq!(hash, hash2);

        // Test inequality
        let hash3 = H256::zero();
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_notification_structure() {
        let script_hash = H160::zero();
        let event_name = ByteString::from_literal("Transfer");
        let state = Array::new();

        let notification = Notification {
            script_hash,
            event_name: event_name.clone(),
            state: state.clone(),
        };

        assert_eq!(notification.script_hash, script_hash);
        assert_eq!(notification.event_name, event_name);
        assert_eq!(notification.state.length(), 0);
    }

    #[test]
    fn test_tx_structure() {
        let hash = H256::zero();
        let sender = H160::zero();
        
        let tx = Tx {
            hash,
            version: 0,
            nonce: 1234567890,
            sender,
            sys_fee: Int256::from(100000),
            net_fee: Int256::from(50000),
            valid_until_block: 1000000,
            script: ByteString::new(),
        };

        assert_eq!(tx.hash, hash);
        assert_eq!(tx.version, 0);
        assert_eq!(tx.nonce, 1234567890);
        assert_eq!(tx.sender, sender);
        assert_eq!(tx.sys_fee, Int256::from(100000));
        assert_eq!(tx.net_fee, Int256::from(50000));
        assert_eq!(tx.valid_until_block, 1000000);
    }
}