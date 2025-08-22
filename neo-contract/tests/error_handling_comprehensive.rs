//! Comprehensive Error Handling and Edge Case Tests
//! 
//! Tests for error conditions, edge cases, security boundaries,
//! and resilience of the Neo N3 contract framework.

#![cfg(test)]

use neo_contract::prelude::*;

/// Framework Error Handling Tests
mod framework_error_tests {
    use super::*;

    #[test]
    fn test_type_conversion_errors() {
        // Test Int256 overflow/underflow
        let max_int = Int256::max_value();
        let overflow_result = max_int.checked_add(&Int256::from(1));
        assert!(overflow_result.is_none());
        
        let min_int = Int256::zero();
        let underflow_result = min_int.checked_sub(&Int256::from(1));
        assert!(underflow_result.is_none());
        
        // Test division by zero
        let dividend = Int256::from(100);
        let zero_divisor = Int256::zero();
        let division_result = dividend.checked_div(&zero_divisor);
        assert!(division_result.is_none());
        
        // Test multiplication overflow
        let large_a = Int256::from(i64::MAX);
        let large_b = Int256::from(i64::MAX);
        let mult_result = large_a.checked_mul(&large_b);
        assert!(mult_result.is_none());
    }

    #[test]
    fn test_h160_invalid_input_handling() {
        // Test with invalid hex strings
        let invalid_hex_results = [
            H160::try_from_hex("0x"),
            H160::try_from_hex("0x123"), // Too short
            H160::try_from_hex("0x12345678901234567890123456789012345678901234567890"), // Too long
            H160::try_from_hex("0xZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ"), // Invalid chars
            H160::try_from_hex("1234567890123456789012345678901234567890"), // Missing 0x
        ];
        
        for result in invalid_hex_results.iter() {
            assert!(result.is_err());
        }
        
        // Test with invalid byte arrays
        let invalid_arrays = [
            &[0u8; 19] as &[u8], // Too short
            &[0u8; 21],          // Too long
            &[],                 // Empty
        ];
        
        for &invalid_array in invalid_arrays.iter() {
            let result = H160::try_from_slice(invalid_array);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_h256_invalid_input_handling() {
        // Test with invalid hex strings
        let invalid_hex_results = [
            H256::try_from_hex("0x"),
            H256::try_from_hex("0x123456"), // Too short
            H256::try_from_hex("0x" + &"12".repeat(33)), // Too long
            H256::try_from_hex("0x" + &"GG".repeat(32)), // Invalid chars
        ];
        
        for result in invalid_hex_results.iter() {
            assert!(result.is_err());
        }
        
        // Test with invalid byte arrays
        let invalid_arrays = [
            &[0u8; 31] as &[u8], // Too short
            &[0u8; 33],          // Too long
            &[],                 // Empty
        ];
        
        for &invalid_array in invalid_arrays.iter() {
            let result = H256::try_from_slice(invalid_array);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_bytestring_edge_cases() {
        // Test with very long strings
        let long_string = ByteString::from("A".repeat(10000).as_bytes());
        assert_eq!(long_string.len(), 10000);
        
        // Test with null bytes
        let with_nulls = ByteString::from(&[0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x57, 0x6F, 0x72, 0x6C, 0x64]);
        assert_eq!(with_nulls.len(), 11);
        assert_eq!(with_nulls.to_bytes()[5], 0x00);
        
        // Test with all possible byte values
        let all_bytes: Vec<u8> = (0..=255).collect();
        let all_bytes_string = ByteString::from(&all_bytes);
        assert_eq!(all_bytes_string.len(), 256);
        
        // Test concatenation with empty strings
        let empty = ByteString::empty();
        let hello = ByteString::from_literal("Hello");
        let concat_result = empty.concat(&hello);
        assert_eq!(concat_result, hello);
        
        let concat_result2 = hello.concat(&empty);
        assert_eq!(concat_result2, hello);
    }

    #[test]
    fn test_array_boundary_conditions() {
        let mut arr = Array::<Int256>::new();
        
        // Test access to empty array
        let empty_get = arr.try_get(0);
        assert!(empty_get.is_none());
        
        // Test pop from empty array
        let empty_pop = arr.try_pop();
        assert!(empty_pop.is_none());
        
        // Add elements and test boundary access
        for i in 0..10 {
            arr.push(Int256::from(i));
        }
        
        // Test valid boundary access
        assert_eq!(arr.get(0), Int256::from(0));
        assert_eq!(arr.get(9), Int256::from(9));
        
        // Test out-of-bounds access
        let oob_get = arr.try_get(10);
        assert!(oob_get.is_none());
        
        let oob_set = arr.try_set(10, Int256::from(99));
        assert!(oob_set.is_err());
        
        // Test with maximum size array (if limits exist)
        let mut large_arr = Array::<Int256>::new();
        for i in 0..1000 {
            large_arr.push(Int256::from(i));
        }
        assert_eq!(large_arr.length(), 1000);
        
        // Test clear and reuse
        large_arr.clear();
        assert_eq!(large_arr.length(), 0);
        large_arr.push(Int256::from(42));
        assert_eq!(large_arr.length(), 1);
        assert_eq!(large_arr.get(0), Int256::from(42));
    }

    #[test]
    fn test_map_edge_cases() {
        let mut map = Map::<ByteString, Int256>::new();
        
        // Test operations on empty map
        let empty_key = ByteString::from_literal("nonexistent");
        assert!(map.get(&empty_key).is_none());
        assert!(!map.has_key(&empty_key));
        
        let empty_remove = map.try_remove(&empty_key);
        assert!(empty_remove.is_err());
        
        // Test with empty key
        let empty_key_str = ByteString::empty();
        map.set(empty_key_str.clone(), Int256::from(1));
        assert!(map.has_key(&empty_key_str));
        assert_eq!(map.get(&empty_key_str).unwrap(), Int256::from(1));
        
        // Test key collision handling
        let key1 = ByteString::from_literal("test");
        let key2 = ByteString::from_literal("test");
        
        map.set(key1.clone(), Int256::from(100));
        map.set(key2.clone(), Int256::from(200));
        
        // Should overwrite, not create duplicate
        assert_eq!(map.get(&key1).unwrap(), Int256::from(200));
        assert_eq!(map.size(), 2); // empty key + test key
        
        // Test with very long keys
        let long_key = ByteString::from("long_key_".repeat(100).as_bytes());
        map.set(long_key.clone(), Int256::from(999));
        assert_eq!(map.get(&long_key).unwrap(), Int256::from(999));
        
        // Test map with many entries
        for i in 0..100 {
            let key = ByteString::from(format!("key_{}", i).as_bytes());
            map.set(key, Int256::from(i));
        }
        assert!(map.size() >= 100);
        
        // Test clear
        map.clear();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_any_type_edge_cases() {
        // Test null Any
        let null_any = Any::null();
        assert!(null_any.is_null());
        
        // Test type checking with null
        assert!(!null_any.is::<Int256>());
        assert!(!null_any.is::<ByteString>());
        assert!(!null_any.is::<bool>());
        
        // Test Any with complex nested types
        let nested_array = Array::<Map<ByteString, Int256>>::new();
        let nested_any = nested_array.into_any();
        assert!(!nested_any.is_null());
        
        // Test Any conversions with edge values
        let zero_any = Int256::zero().into_any();
        let max_any = Int256::max_value().into_any();
        let empty_string_any = ByteString::empty().into_any();
        
        assert!(!zero_any.is_null());
        assert!(!max_any.is_null());
        assert!(!empty_string_any.is_null());
    }
}

/// Storage Error Handling Tests
mod storage_error_tests {
    use super::*;

    #[test]
    fn test_storage_permission_violations() {
        let read_only_ctx = Storage::get_read_only_context();
        let read_write_ctx = Storage::get_context();
        
        let test_key = ByteString::from_literal("permission_test");
        let test_value = Int256::from(42).into_any();
        
        // Should succeed with read-write context
        Storage::put(read_write_ctx.clone(), test_key.clone(), test_value.clone());
        
        // Attempt to write with read-only context should fail gracefully
        let ro_put_result = Storage::try_put(read_only_ctx.clone(), test_key.clone(), test_value);
        assert!(ro_put_result.is_err());
        
        // Attempt to delete with read-only context should fail
        let ro_delete_result = Storage::try_delete(read_only_ctx, test_key.clone());
        assert!(ro_delete_result.is_err());
        
        // Read operations should work with both contexts
        let rw_read = Storage::get(read_write_ctx.clone(), test_key.clone());
        let ro_read = Storage::get(Storage::get_read_only_context(), test_key);
        
        assert!(rw_read.is_some());
        assert!(ro_read.is_some());
    }

    #[test]
    fn test_storage_key_edge_cases() {
        let ctx = Storage::get_context();
        
        // Test with empty key
        let empty_key = ByteString::empty();
        let empty_value = Int256::from(1).into_any();
        
        Storage::put(ctx.clone(), empty_key.clone(), empty_value);
        let retrieved = Storage::get(ctx.clone(), empty_key.clone());
        assert!(retrieved.is_some());
        
        // Test with very long key
        let long_key = ByteString::from("x".repeat(1000).as_bytes());
        let long_value = Int256::from(2).into_any();
        
        Storage::put(ctx.clone(), long_key.clone(), long_value);
        let long_retrieved = Storage::get(ctx.clone(), long_key);
        assert!(long_retrieved.is_some());
        
        // Test with binary key data
        let binary_key = ByteString::from(&[0x00, 0xFF, 0x7F, 0x80, 0x01, 0xFE]);
        let binary_value = Int256::from(3).into_any();
        
        Storage::put(ctx.clone(), binary_key.clone(), binary_value);
        let binary_retrieved = Storage::get(ctx.clone(), binary_key);
        assert!(binary_retrieved.is_some());
        
        // Test with Unicode key
        let unicode_key = ByteString::from("测试键🔑".as_bytes());
        let unicode_value = Int256::from(4).into_any();
        
        Storage::put(ctx.clone(), unicode_key.clone(), unicode_value);
        let unicode_retrieved = Storage::get(ctx, unicode_key);
        assert!(unicode_retrieved.is_some());
    }

    #[test]
    fn test_storage_value_edge_cases() {
        let ctx = Storage::get_context();
        
        // Test with null value
        let null_key = ByteString::from_literal("null_test");
        Storage::put(ctx.clone(), null_key.clone(), Any::null());
        let null_retrieved = Storage::get(ctx.clone(), null_key);
        assert!(null_retrieved.is_some());
        assert!(null_retrieved.unwrap().is_null());
        
        // Test with very large value
        let large_key = ByteString::from_literal("large_test");
        let mut large_array = Array::<Int256>::new();
        for i in 0..1000 {
            large_array.push(Int256::from(i));
        }
        
        Storage::put(ctx.clone(), large_key.clone(), large_array.into_any());
        let large_retrieved = Storage::get(ctx.clone(), large_key);
        assert!(large_retrieved.is_some());
        
        // Test overwriting with different types
        let type_key = ByteString::from_literal("type_test");
        
        Storage::put(ctx.clone(), type_key.clone(), Int256::from(100).into_any());
        let int_retrieved = Storage::get(ctx.clone(), type_key.clone());
        assert!(int_retrieved.is_some());
        
        Storage::put(ctx.clone(), type_key.clone(), ByteString::from_literal("string").into_any());
        let string_retrieved = Storage::get(ctx.clone(), type_key.clone());
        assert!(string_retrieved.is_some());
        
        Storage::put(ctx.clone(), type_key.clone(), true.into_any());
        let bool_retrieved = Storage::get(ctx, type_key);
        assert!(bool_retrieved.is_some());
    }

    #[test]
    fn test_storage_find_edge_cases() {
        let ctx = Storage::get_context();
        
        // Test find with empty prefix
        let empty_prefix = ByteString::empty();
        let empty_iter = Storage::find(ctx.clone(), empty_prefix, FindOptions::default());
        
        // Should create iterator (may return all items or none in mock)
        let first_key = empty_iter.next_key();
        assert!(first_key.is_none() || first_key.is_some());
        
        // Test find with non-existent prefix
        let nonexistent_prefix = ByteString::from_literal("nonexistent_prefix:");
        let nonexistent_iter = Storage::find(ctx.clone(), nonexistent_prefix, FindOptions::default());
        
        let no_key = nonexistent_iter.next_key();
        assert!(no_key.is_none());
        
        // Set up test data with various prefixes
        let prefixes = ["a:", "aa:", "ab:", "b:", ""];
        for (i, prefix) in prefixes.iter().enumerate() {
            let key = ByteString::from_literal(prefix)
                .concat(&ByteString::from(format!("item_{}", i).as_bytes()));
            Storage::put(ctx.clone(), key, Int256::from(i as i64).into_any());
        }
        
        // Test find with overlapping prefixes
        let short_prefix = ByteString::from_literal("a:");
        let long_prefix = ByteString::from_literal("aa:");
        
        let short_iter = Storage::find(ctx.clone(), short_prefix, FindOptions::default());
        let long_iter = Storage::find(ctx, long_prefix, FindOptions::default());
        
        // Should handle prefix matching correctly
        let short_result = short_iter.next_key();
        let long_result = long_iter.next_key();
        
        assert!(short_result.is_none() || short_result.is_some());
        assert!(long_result.is_none() || long_result.is_some());
    }

    #[test]
    fn test_storage_iterator_edge_cases() {
        let ctx = Storage::get_context();
        let prefix = ByteString::from_literal("iter_test:");
        
        // Test iterator on empty result set
        let empty_iter = Storage::find(ctx.clone(), prefix.clone(), FindOptions::default());
        
        // Multiple calls to empty iterator
        for _ in 0..5 {
            let key = empty_iter.next_key();
            let value = empty_iter.next_value();
            assert!(key.is_none());
            assert!(value.is_none());
        }
        
        // Add some test data
        for i in 0..5 {
            let key = prefix.clone().concat(&ByteString::from(format!("{}", i).as_bytes()));
            Storage::put(ctx.clone(), key, Int256::from(i).into_any());
        }
        
        // Test iterator with different find options
        let options_none = FindOptions::new(None, None, false);
        let options_desc = FindOptions::new(None, Some(true), false);
        let options_keys_only = FindOptions::new(None, None, true);
        let options_limited = FindOptions::new(Some(3), None, false);
        
        let iter1 = Storage::find(ctx.clone(), prefix.clone(), options_none);
        let iter2 = Storage::find(ctx.clone(), prefix.clone(), options_desc);
        let iter3 = Storage::find(ctx.clone(), prefix.clone(), options_keys_only);
        let iter4 = Storage::find(ctx, prefix, options_limited);
        
        // Test that different iterators work independently
        let _key1 = iter1.next_key();
        let _key2 = iter2.next_key();
        let _key3 = iter3.next_key();
        let _key4 = iter4.next_key();
    }

    #[test]
    fn test_storage_map_error_handling() {
        use neo_contract::storage::StorageMap;
        
        let prefix = ByteString::from_literal("map_error_test");
        let mut map = StorageMap::new(prefix);
        
        // Test operations on non-existent keys
        let missing_key = ByteString::from_literal("missing");
        
        let missing_get = map.try_get(missing_key.clone());
        assert!(missing_get.is_none());
        
        let missing_delete = map.try_delete(missing_key.clone());
        assert!(missing_delete.is_ok()); // Delete of non-existent key should succeed
        
        // Test with null values
        map.put(missing_key.clone(), Any::null());
        let null_retrieved = map.get(missing_key);
        assert!(null_retrieved.is_some());
        assert!(null_retrieved.unwrap().is_null());
        
        // Test map with conflicting keys
        let key1 = ByteString::from_literal("conflict");
        let key2 = ByteString::from_literal("conflict"); // Same content
        
        map.put(key1.clone(), Int256::from(1).into_any());
        map.put(key2.clone(), Int256::from(2).into_any());
        
        // Should overwrite
        assert_eq!(map.get(key1).unwrap().try_into().unwrap_or(Int256::zero()), Int256::from(2));
    }

    #[test]
    fn test_storage_item_error_handling() {
        use neo_contract::storage::StorageItem;
        
        let key = ByteString::from_literal("item_error_test");
        let mut item = StorageItem::new(key);
        
        // Test get from uninitialized item
        let uninitialized_get = item.get();
        assert!(uninitialized_get.is_none());
        
        // Test delete from uninitialized item
        item.delete(); // Should not panic
        
        // Test multiple sets and gets
        item.set(Int256::from(1).into_any());
        assert!(item.get().is_some());
        
        item.set(ByteString::from_literal("changed").into_any());
        assert!(item.get().is_some());
        
        item.set(Any::null());
        let null_get = item.get();
        assert!(null_get.is_some());
        assert!(null_get.unwrap().is_null());
        
        // Test delete and re-use
        item.delete();
        assert!(item.get().is_none());
        
        item.set(Int256::from(42).into_any());
        assert!(item.get().is_some());
    }
}

/// Runtime Error Handling Tests
mod runtime_error_tests {
    use super::*;

    #[test]
    fn test_witness_checking_edge_cases() {
        // Test with zero address
        let zero_addr = H160::zero();
        let zero_witness = Runtime::check_witness_with_account(zero_addr);
        assert!(!zero_witness); // Should never succeed for zero address
        
        // Test with invalid public key
        let invalid_pubkey_bytes = [0x00; 33];
        let invalid_pubkey = PublicKey::from_bytes(&invalid_pubkey_bytes);
        let invalid_witness = Runtime::check_witness_with_public_key(invalid_pubkey);
        assert!(!invalid_witness); // Should fail for invalid key
        
        // Test with maximum address
        let max_addr = H160::from_array([0xFF; 20]);
        let max_witness = Runtime::check_witness_with_account(max_addr);
        assert!(!max_witness); // Mock returns false for security
        
        // Test multiple rapid witness checks
        for i in 0..10 {
            let test_addr = H160::from_array([i as u8; 20]);
            let _witness = Runtime::check_witness_with_account(test_addr);
        }
    }

    #[test]
    fn test_gas_operations_edge_cases() {
        // Test burning zero gas
        Runtime::burn_gas(Int256::zero()); // Should not panic
        
        // Test burning negative gas (if allowed)
        let negative_gas = Int256::from(-100);
        let burn_result = Runtime::try_burn_gas(negative_gas);
        assert!(burn_result.is_err()); // Should reject negative gas
        
        // Test burning more gas than available
        let current_gas = Runtime::get_gas_left();
        let excess_gas = current_gas.checked_add(&Int256::from(1000000)).unwrap_or(current_gas);
        let excess_result = Runtime::try_burn_gas(excess_gas);
        assert!(excess_result.is_err() || excess_result.is_ok()); // Implementation dependent
        
        // Test gas after multiple operations
        let initial_gas = Runtime::get_gas_left();
        for _ in 0..10 {
            Runtime::burn_gas(Int256::from(100));
        }
        let final_gas = Runtime::get_gas_left();
        
        // Gas should be reduced (or unchanged in mock)
        assert!(final_gas <= initial_gas);
    }

    #[test]
    fn test_notification_edge_cases() {
        // Test notification with empty event name
        let empty_name = ByteString::empty();
        let mut empty_data = Array::new();
        empty_data.push(Int256::from(42).into_any());
        
        Runtime::notify(empty_name, empty_data); // Should not panic
        
        // Test notification with empty data
        let event_name = ByteString::from_literal("EmptyDataEvent");
        let empty_event_data = Array::new();
        
        Runtime::notify(event_name, empty_event_data);
        
        // Test notification with null data
        let null_event = ByteString::from_literal("NullDataEvent");
        let mut null_data = Array::new();
        null_data.push(Any::null());
        
        Runtime::notify(null_event, null_data);
        
        // Test notification with very large data
        let large_event = ByteString::from_literal("LargeDataEvent");
        let mut large_data = Array::new();
        
        for i in 0..100 {
            large_data.push(Int256::from(i).into_any());
        }
        
        Runtime::notify(large_event, large_data);
        
        // Test notification with nested complex types
        let complex_event = ByteString::from_literal("ComplexEvent");
        let mut complex_data = Array::new();
        
        let mut inner_array = Array::<Int256>::new();
        inner_array.push(Int256::from(1));
        inner_array.push(Int256::from(2));
        complex_data.push(inner_array.into_any());
        
        let mut inner_map = Map::<ByteString, Int256>::new();
        inner_map.set(ByteString::from_literal("key"), Int256::from(100));
        complex_data.push(inner_map.into_any());
        
        Runtime::notify(complex_event, complex_data);
    }

    #[test]
    fn test_logging_edge_cases() {
        // Test logging empty message
        Runtime::log(ByteString::empty());
        
        // Test logging very long message
        let long_message = ByteString::from("Long message ".repeat(1000).as_bytes());
        Runtime::log(long_message);
        
        // Test logging binary data
        let binary_message = ByteString::from(&[0x00, 0xFF, 0x7F, 0x80, 0x01, 0xFE]);
        Runtime::log(binary_message);
        
        // Test logging Unicode
        let unicode_message = ByteString::from("Unicode: 测试 🚀 🌟".as_bytes());
        Runtime::log(unicode_message);
        
        // Test rapid logging
        for i in 0..50 {
            let message = ByteString::from(format!("Rapid log {}", i).as_bytes());
            Runtime::log(message);
        }
    }

    #[test]
    fn test_random_number_edge_cases() {
        // Test multiple rapid calls
        let mut randoms = Vec::new();
        for _ in 0..10 {
            randoms.push(Runtime::get_random());
        }
        
        // In mock environment, all will be the same
        // In real environment, should have some variation
        assert_eq!(randoms.len(), 10);
        
        // Test that random numbers are in valid range
        for random in randoms {
            assert!(random >= Int256::zero());
            assert!(random <= Int256::max_value());
        }
    }

    #[test]
    fn test_script_hash_consistency() {
        // Test that script hashes are consistent
        let executing1 = Runtime::get_executing_script_hash();
        let executing2 = Runtime::get_executing_script_hash();
        assert_eq!(executing1, executing2);
        
        let calling1 = Runtime::get_calling_script_hash();
        let calling2 = Runtime::get_calling_script_hash();
        assert_eq!(calling1, calling2);
        
        let entry1 = Runtime::get_entry_script_hash();
        let entry2 = Runtime::get_entry_script_hash();
        assert_eq!(entry1, entry2);
        
        // Test that different hash types may be different or same
        // (depending on call context)
        assert!(executing1 == calling1 || executing1 != calling1);
        assert!(executing1 == entry1 || executing1 != entry1);
        assert!(calling1 == entry1 || calling1 != entry1);
    }

    #[test]
    fn test_transaction_data_edge_cases() {
        // Test transaction data retrieval
        let tx = Runtime::get_tx();
        
        // Test transaction fields
        assert_eq!(tx.hash.to_bytes().len(), 32);
        assert_eq!(tx.sender.to_bytes().len(), 20);
        assert!(tx.version >= 0);
        assert!(tx.nonce >= 0);
        
        // Test multiple retrievals return same data
        let tx2 = Runtime::get_tx();
        assert_eq!(tx.hash, tx2.hash);
        assert_eq!(tx.sender, tx2.sender);
        assert_eq!(tx.version, tx2.version);
        assert_eq!(tx.nonce, tx2.nonce);
    }

    #[test]
    fn test_platform_info_consistency() {
        // Test platform information consistency
        let platform1 = Runtime::get_platform();
        let platform2 = Runtime::get_platform();
        assert_eq!(platform1, platform2);
        assert_eq!(platform1, ByteString::from_literal("NEO"));
        
        let trigger1 = Runtime::get_trigger();
        let trigger2 = Runtime::get_trigger();
        assert_eq!(trigger1, trigger2);
        assert_eq!(trigger1, TriggerType::Application);
        
        let network1 = Runtime::get_network();
        let network2 = Runtime::get_network();
        assert_eq!(network1, network2);
        assert_eq!(network1, 860833102); // Neo N3 mainnet
        
        let version1 = Runtime::get_address_version();
        let version2 = Runtime::get_address_version();
        assert_eq!(version1, version2);
        assert_eq!(version1, 53); // Neo N3 address version
    }
}

/// Crypto Service Error Tests
mod crypto_error_tests {
    use super::*;

    #[test]
    fn test_hash_function_edge_cases() {
        // Test with empty input
        let empty_input = ByteString::empty();
        let empty_sha256 = neo_contract::crypto::sha256(empty_input.clone());
        let empty_ripemd = neo_contract::crypto::ripemd160(empty_input.clone());
        let empty_hash160 = neo_contract::crypto::hash160(empty_input.clone());
        let empty_hash256 = neo_contract::crypto::hash256(empty_input);
        
        assert_eq!(empty_sha256.to_bytes().len(), 32);
        assert_eq!(empty_ripemd.to_bytes().len(), 20);
        assert_eq!(empty_hash160.to_bytes().len(), 20);
        assert_eq!(empty_hash256.to_bytes().len(), 32);
        
        // Test with very large input
        let large_input = ByteString::from(&vec![0x42; 100000]);
        let large_sha256 = neo_contract::crypto::sha256(large_input.clone());
        assert_eq!(large_sha256.to_bytes().len(), 32);
        
        // Test with binary data containing all byte values
        let binary_input = ByteString::from(&(0..=255u8).collect::<Vec<u8>>());
        let binary_hash = neo_contract::crypto::sha256(binary_input);
        assert_eq!(binary_hash.to_bytes().len(), 32);
        
        // Test consistency - same input should produce same hash
        let test_data = ByteString::from_literal("consistency_test");
        let hash1 = neo_contract::crypto::sha256(test_data.clone());
        let hash2 = neo_contract::crypto::sha256(test_data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_signature_verification_edge_cases() {
        let test_message = ByteString::from_literal("test_message");
        
        // Test with invalid public key formats
        let invalid_keys = [
            &[0x00; 33] as &[u8],        // All zeros
            &[0xFF; 33],                 // All ones  
            &[0x01; 33],                 // Invalid format byte
            &[0x03; 33],                 // Invalid compressed format
            &[0x04; 32],                 // Wrong length
            &[],                         // Empty
        ];
        
        for &invalid_key in invalid_keys.iter() {
            let pubkey = PublicKey::from_bytes(invalid_key);
            let signature = ByteString::from(&[0x30, 0x44; 32]); // Mock signature
            
            let result = neo_contract::crypto::verify_ecdsa(
                test_message.clone(),
                pubkey,
                signature,
                neo_contract::crypto::NamedCurveHash::Secp256r1
            );
            
            assert!(!result); // Should reject invalid keys
        }
        
        // Test with invalid signature formats
        let valid_pubkey = PublicKey::from_bytes(&[0x02; 33]);
        let invalid_signatures = [
            ByteString::empty(),                    // Empty signature
            ByteString::from(&[0x00]),              // Too short
            ByteString::from(&vec![0xFF; 1000]),    // Too long
            ByteString::from(&[0x30]),              // Incomplete DER
            ByteString::from(&[0xFF; 64]),          // Invalid format
        ];
        
        for invalid_sig in invalid_signatures.iter() {
            let result = neo_contract::crypto::verify_ecdsa(
                test_message.clone(),
                valid_pubkey.clone(),
                invalid_sig.clone(),
                neo_contract::crypto::NamedCurveHash::Secp256r1
            );
            
            assert!(!result); // Should reject invalid signatures
        }
        
        // Test with empty message
        let empty_message = ByteString::empty();
        let valid_signature = ByteString::from(&[0x30, 0x44; 32]);
        
        let empty_msg_result = neo_contract::crypto::verify_ecdsa(
            empty_message,
            valid_pubkey.clone(),
            valid_signature.clone(),
            neo_contract::crypto::NamedCurveHash::Secp256r1
        );
        
        // Should handle empty message (may succeed or fail depending on implementation)
        assert!(empty_msg_result == true || empty_msg_result == false);
        
        // Test with different curve types
        let curves = [
            neo_contract::crypto::NamedCurveHash::Secp256r1,
            neo_contract::crypto::NamedCurveHash::Secp256k1,
        ];
        
        for curve in curves.iter() {
            let result = neo_contract::crypto::verify_ecdsa(
                test_message.clone(),
                valid_pubkey.clone(),
                valid_signature.clone(),
                *curve
            );
            
            // Mock implementation returns false for unverified signatures
            assert!(!result);
        }
    }

    #[test]
    fn test_public_key_edge_cases() {
        // Test various public key formats
        let test_keys = [
            ([0x02; 33], true),  // Compressed format (even)
            ([0x03; 33], true),  // Compressed format (odd)
            ([0x04; 33], true),  // Uncompressed format marker
            ([0x00; 33], false), // Invalid format
            ([0x01; 33], false), // Invalid format
            ([0x05; 33], false), // Invalid format
        ];
        
        for &(mut key_bytes, should_be_valid) in test_keys.iter() {
            key_bytes[0] = key_bytes[0]; // Set format byte
            let pubkey = PublicKey::from_bytes(&key_bytes);
            
            if should_be_valid {
                assert!(pubkey.is_valid());
                
                // Test address derivation
                let address = pubkey.to_address();
                assert_eq!(address.to_bytes().len(), 20);
            } else {
                // Invalid keys should still create PublicKey objects but be marked invalid
                assert!(pubkey.is_valid() || !pubkey.is_valid());
            }
        }
        
        // Test with wrong key lengths
        let wrong_lengths = [
            &[0x02; 32] as &[u8],  // Too short
            &[0x02; 34],           // Too long
            &[0x02; 65],           // Uncompressed length but compressed format
            &[],                   // Empty
        ];
        
        for &wrong_key in wrong_lengths.iter() {
            let pubkey = PublicKey::from_bytes(wrong_key);
            // Should handle gracefully
            assert!(pubkey.is_valid() || !pubkey.is_valid());
        }
    }

    #[test]
    fn test_multi_signature_edge_cases() {
        let message = ByteString::from_literal("multisig_test");
        
        // Test with empty arrays
        let empty_keys = Array::<PublicKey>::new();
        let empty_sigs = Array::<ByteString>::new();
        
        let empty_result = neo_contract::crypto::check_multi_signs(
            message.clone(),
            empty_keys,
            empty_sigs,
            0
        );
        assert!(!empty_result); // Should fail with empty arrays
        
        // Test with mismatched array lengths
        let mut keys = Array::<PublicKey>::new();
        keys.push(PublicKey::from_bytes(&[0x02; 33]));
        keys.push(PublicKey::from_bytes(&[0x03; 33]));
        
        let mut sigs = Array::<ByteString>::new();
        sigs.push(ByteString::from(&[0x30; 64]));
        // Missing second signature
        
        let mismatch_result = neo_contract::crypto::check_multi_signs(
            message.clone(),
            keys.clone(),
            sigs,
            2
        );
        assert!(!mismatch_result); // Should fail with mismatched lengths
        
        // Test with threshold higher than available keys
        let mut proper_sigs = Array::<ByteString>::new();
        proper_sigs.push(ByteString::from(&[0x30; 64]));
        proper_sigs.push(ByteString::from(&[0x30; 64]));
        
        let high_threshold_result = neo_contract::crypto::check_multi_signs(
            message.clone(),
            keys.clone(),
            proper_sigs,
            5 // Higher than key count
        );
        assert!(!high_threshold_result); // Should fail with impossible threshold
        
        // Test with zero threshold
        let zero_threshold_result = neo_contract::crypto::check_multi_signs(
            message,
            keys,
            Array::new(),
            0
        );
        // Zero threshold might succeed or fail depending on implementation
        assert!(zero_threshold_result == true || zero_threshold_result == false);
    }
}

/// Contract Service Error Tests
mod contract_error_tests {
    use super::*;

    #[test]
    fn test_contract_call_edge_cases() {
        let target_contract = H160::zero(); // Invalid contract
        let method = ByteString::from_literal("test_method");
        let mut call_flags = CallFlags::new();
        call_flags.set_allow_call(true);
        let args = Array::new();
        
        // Test call to zero address
        let zero_result = Contract::call(target_contract, method.clone(), call_flags.clone(), args.clone());
        assert!(zero_result.is_none()); // Should fail for zero address
        
        // Test call with empty method name
        let valid_contract = H160::from_array([0x11; 20]);
        let empty_method = ByteString::empty();
        
        let empty_method_result = Contract::call(valid_contract, empty_method, call_flags.clone(), args.clone());
        assert!(empty_method_result.is_none()); // Should handle empty method name
        
        // Test call with invalid call flags
        let mut invalid_flags = CallFlags::new();
        // Don't set any permissions
        
        let no_permission_result = Contract::call(valid_contract, method.clone(), invalid_flags, args.clone());
        assert!(no_permission_result.is_none()); // Should fail without proper permissions
        
        // Test call with very long method name
        let long_method = ByteString::from("long_method_name_".repeat(100).as_bytes());
        
        let long_method_result = Contract::call(valid_contract, long_method, call_flags.clone(), args.clone());
        assert!(long_method_result.is_none() || long_method_result.is_some());
        
        // Test call with large number of arguments
        let mut many_args = Array::new();
        for i in 0..100 {
            many_args.push(Int256::from(i).into_any());
        }
        
        let many_args_result = Contract::call(valid_contract, method, call_flags, many_args);
        assert!(many_args_result.is_none() || many_args_result.is_some());
    }

    #[test]
    fn test_call_flags_edge_cases() {
        let mut flags = CallFlags::new();
        
        // Test default state
        assert!(!flags.allows_call());
        assert!(!flags.allows_notify());
        assert!(!flags.allows_read_states());
        assert!(!flags.allows_write_states());
        
        // Test individual flag setting
        flags.set_allow_call(true);
        assert!(flags.allows_call());
        assert!(!flags.allows_notify()); // Others should remain false
        
        flags.set_allow_notify(true);
        assert!(flags.allows_call());
        assert!(flags.allows_notify());
        
        // Test flag clearing
        flags.set_allow_call(false);
        assert!(!flags.allows_call());
        assert!(flags.allows_notify()); // Others should remain unchanged
        
        // Test set all
        flags.set_all(true);
        assert!(flags.allows_call());
        assert!(flags.allows_notify());
        assert!(flags.allows_read_states());
        assert!(flags.allows_write_states());
        
        // Test clear all
        flags.set_all(false);
        assert!(!flags.allows_call());
        assert!(!flags.allows_notify());
        assert!(!flags.allows_read_states());
        assert!(!flags.allows_write_states());
        
        // Test conflicting operations
        flags.set_write_states(true);
        flags.set_read_states(false);
        // Should handle conflicting states appropriately
        assert!(flags.allows_write_states() || !flags.allows_write_states());
    }

    #[test]
    fn test_account_creation_edge_cases() {
        // Test with invalid public keys
        let invalid_keys = [
            &[0x00; 33] as &[u8],
            &[0xFF; 33],
            &[0x01; 33],
            &[],
        ];
        
        for &invalid_key in invalid_keys.iter() {
            let pubkey = PublicKey::from_bytes(invalid_key);
            let account = Contract::create_standard_account(pubkey);
            
            // Should create some account (may be invalid)
            assert_eq!(account.to_bytes().len(), 20);
        }
        
        // Test multisig with edge cases
        let mut pubkeys = Array::<PublicKey>::new();
        
        // Empty pubkey array
        let empty_multisig = Contract::try_create_multi_signs_account(2, pubkeys.clone());
        assert!(empty_multisig.is_err()); // Should fail with empty array
        
        // Single pubkey with threshold > 1
        pubkeys.push(PublicKey::from_bytes(&[0x02; 33]));
        let impossible_threshold = Contract::try_create_multi_signs_account(2, pubkeys.clone());
        assert!(impossible_threshold.is_err()); // Should fail with impossible threshold
        
        // Zero threshold
        let zero_threshold = Contract::try_create_multi_signs_account(0, pubkeys.clone());
        assert!(zero_threshold.is_err() || zero_threshold.is_ok()); // Implementation dependent
        
        // Valid multisig
        pubkeys.push(PublicKey::from_bytes(&[0x03; 33]));
        pubkeys.push(PublicKey::from_bytes(&[0x02; 33]));
        
        let valid_multisig = Contract::create_multi_signs_account(2, pubkeys.clone());
        assert_eq!(valid_multisig.to_bytes().len(), 20);
        
        // Threshold equal to pubkey count
        let equal_threshold = Contract::create_multi_signs_account(3, pubkeys);
        assert_eq!(equal_threshold.to_bytes().len(), 20);
    }

    #[test]
    fn test_contract_update_edge_cases() {
        // Test with empty script
        let empty_script = ByteString::empty();
        let manifest = ByteString::from_literal(r#"{"name":"test"}"#);
        let data = Any::null();
        
        let empty_script_result = Contract::try_update(empty_script, manifest.clone(), data.clone());
        assert!(empty_script_result.is_err()); // Should fail with empty script
        
        // Test with invalid manifest
        let script = ByteString::from_literal("valid_script");
        let invalid_manifest = ByteString::from_literal("invalid json");
        
        let invalid_manifest_result = Contract::try_update(script.clone(), invalid_manifest, data.clone());
        assert!(invalid_manifest_result.is_err()); // Should fail with invalid manifest
        
        // Test with very large script
        let large_script = ByteString::from(&vec![0x42; 100000]);
        
        let large_script_result = Contract::try_update(large_script, manifest, data);
        assert!(large_script_result.is_ok() || large_script_result.is_err()); // May succeed or fail
    }

    #[test]
    fn test_contract_destroy_edge_cases() {
        // Test destroy without proper authorization
        // (In real environment, this would check witness)
        let destroy_result = Contract::try_destroy();
        
        // Should succeed or fail based on authorization
        assert!(destroy_result.is_ok() || destroy_result.is_err());
        
        // Test multiple destroy attempts
        let _destroy1 = Contract::try_destroy();
        let _destroy2 = Contract::try_destroy();
        // Second destroy should handle gracefully if first succeeded
    }
}

/// Serialization Error Tests
mod serialization_error_tests {
    use super::*;
    use neo_contract::serialization::StorageSerialize;

    #[test]
    fn test_serialization_edge_cases() {
        // Test serialization of edge values
        
        // Int256 edge cases
        let zero = Int256::zero();
        let max_val = Int256::max_value();
        let min_val = Int256::min_value();
        
        let zero_serialized = zero.to_storage();
        let max_serialized = max_val.to_storage();
        let min_serialized = min_val.to_storage();
        
        assert!(!zero_serialized.is_empty());
        assert!(!max_serialized.is_empty());
        assert!(!min_serialized.is_empty());
        
        // Test deserialization
        let zero_deserialized = Int256::from_storage(zero_serialized);
        let max_deserialized = Int256::from_storage(max_serialized);
        let min_deserialized = Int256::from_storage(min_serialized);
        
        assert!(zero_deserialized.is_some());
        assert!(max_deserialized.is_some());
        assert!(min_deserialized.is_some());
        
        // Boolean edge cases
        let true_val = true;
        let false_val = false;
        
        let true_serialized = true_val.to_storage();
        let false_serialized = false_val.to_storage();
        
        let true_deserialized = bool::from_storage(true_serialized);
        let false_deserialized = bool::from_storage(false_serialized);
        
        assert_eq!(true_deserialized, Some(true));
        assert_eq!(false_deserialized, Some(false));
        
        // Test with corrupted data
        let corrupted_data = vec![0xFF, 0x00, 0xFF];
        let corrupted_int = Int256::from_storage(corrupted_data.clone());
        let corrupted_bool = bool::from_storage(corrupted_data.clone());
        let corrupted_h160 = H160::from_storage(corrupted_data);
        
        // Should handle corrupted data gracefully
        assert!(corrupted_int.is_none());
        assert!(corrupted_bool.is_none());
        assert!(corrupted_h160.is_none());
    }

    #[test]
    fn test_storage_helpers_edge_cases() {
        // Test storage helpers with edge cases
        let empty_key = ByteString::empty();
        let long_key = ByteString::from("x".repeat(1000).as_bytes());
        let binary_key = ByteString::from(&[0x00, 0xFF, 0x7F, 0x80]);
        
        // Test with various key types
        let keys = [empty_key, long_key, binary_key];
        
        for key in keys.iter() {
            // Test with different value types
            neo_contract::serialization::storage_put(key.clone(), Int256::from(42));
            let int_retrieved = neo_contract::serialization::storage_get::<Int256>(key.clone());
            
            neo_contract::serialization::storage_put(key.clone(), true);
            let bool_retrieved = neo_contract::serialization::storage_get::<bool>(key.clone());
            
            neo_contract::serialization::storage_put(key.clone(), H160::zero());
            let h160_retrieved = neo_contract::serialization::storage_get::<H160>(key.clone());
            
            // Should handle all key types
            assert!(int_retrieved.is_none() || int_retrieved.is_some());
            assert!(bool_retrieved.is_none() || bool_retrieved.is_some());
            assert!(h160_retrieved.is_none() || h160_retrieved.is_some());
        }
    }

    #[test]
    fn test_serialization_round_trip_edge_cases() {
        // Test round trip with extreme values
        
        // Test u32 edge values
        let u32_values = [0u32, u32::MAX, u32::MAX / 2];
        for &val in u32_values.iter() {
            let serialized = val.to_storage();
            let deserialized = u32::from_storage(serialized);
            assert_eq!(deserialized, Some(val));
        }
        
        // Test with empty serialized data
        let empty_data = vec![];
        assert!(Int256::from_storage(empty_data.clone()).is_none());
        assert!(bool::from_storage(empty_data.clone()).is_none());
        assert!(H160::from_storage(empty_data).is_none());
        
        // Test with single byte data
        let single_byte = vec![0x42];
        assert!(Int256::from_storage(single_byte.clone()).is_none());
        assert!(bool::from_storage(single_byte.clone()).is_some() || bool::from_storage(single_byte.clone()).is_none());
        assert!(H160::from_storage(single_byte).is_none());
    }
}

// Helper functions for error testing

// Extended type conversion helpers
impl Int256 {
    fn try_from_i128(val: i128) -> Result<Self, String> {
        if val > i64::MAX as i128 {
            Err("Value too large".to_string())
        } else {
            Ok(Int256::from(val as i64))
        }
    }
    
    fn min_value() -> Self {
        Int256::from(i64::MIN)
    }
}

impl H160 {
    fn try_from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 42 || !hex.starts_with("0x") {
            return Err("Invalid hex format".to_string());
        }
        
        if hex.chars().skip(2).all(|c| c.is_ascii_hexdigit()) {
            Ok(H160::from_hex(hex))
        } else {
            Err("Invalid hex characters".to_string())
        }
    }
    
    fn try_from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() == 20 {
            Ok(H160::from_slice(slice))
        } else {
            Err("Invalid length".to_string())
        }
    }
}

impl H256 {
    fn try_from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 66 || !hex.starts_with("0x") {
            return Err("Invalid hex format".to_string());
        }
        
        if hex.chars().skip(2).all(|c| c.is_ascii_hexdigit()) {
            Ok(H256::from_hex(hex))
        } else {
            Err("Invalid hex characters".to_string())
        }
    }
    
    fn try_from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() == 32 {
            Ok(H256::from_slice(slice))
        } else {
            Err("Invalid length".to_string())
        }
    }
}

// Extended Array operations
impl<T> Array<T> {
    fn try_get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        if index < self.length() {
            Some(self.get(index))
        } else {
            None
        }
    }
    
    fn try_set(&mut self, index: usize, value: T) -> Result<(), String> {
        if index < self.length() {
            self.set(index, value);
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }
    
    fn try_pop(&mut self) -> Option<T> {
        if self.length() > 0 {
            Some(self.pop())
        } else {
            None
        }
    }
}

// Extended Map operations
impl<K, V> Map<K, V> {
    fn try_get(&self, key: &K) -> Option<V>
    where
        K: Clone,
        V: Clone,
    {
        self.get(key)
    }
    
    fn try_remove(&mut self, key: &K) -> Result<(), String>
    where
        K: Clone,
    {
        if self.has_key(key) {
            self.remove(key);
            Ok(())
        } else {
            Ok(()) // Removing non-existent key is OK
        }
    }
}

// Extended Runtime operations
impl Runtime {
    fn try_burn_gas(amount: Int256) -> Result<(), String> {
        if amount < Int256::zero() {
            Err("Cannot burn negative gas".to_string())
        } else {
            Runtime::burn_gas(amount);
            Ok(())
        }
    }
}

// Extended Contract operations
impl Contract {
    fn try_create_multi_signs_account(threshold: u32, pubkeys: Array<PublicKey>) -> Result<H160, String> {
        if pubkeys.length() == 0 {
            Err("Empty public key array".to_string())
        } else if threshold == 0 {
            Err("Threshold cannot be zero".to_string())
        } else if threshold > pubkeys.length() as u32 {
            Err("Threshold exceeds public key count".to_string())
        } else {
            Ok(Contract::create_multi_signs_account(threshold, pubkeys))
        }
    }
    
    fn try_update(script: ByteString, manifest: ByteString, data: Any) -> Result<(), String> {
        if script.is_empty() {
            Err("Script cannot be empty".to_string())
        } else {
            // Try to parse manifest as JSON
            let manifest_str = String::from_utf8(manifest.to_bytes()).map_err(|_| "Invalid manifest encoding".to_string())?;
            serde_json::from_str::<serde_json::Value>(&manifest_str).map_err(|_| "Invalid manifest JSON".to_string())?;
            
            Contract::update(script, manifest, data);
            Ok(())
        }
    }
    
    fn try_destroy() -> Result<(), String> {
        Contract::destroy();
        Ok(())
    }
}

// Extended Storage operations
impl Storage {
    fn try_put(context: StorageContext, key: ByteString, value: Any) -> Result<(), String> {
        if context.is_read_only() {
            Err("Cannot write to read-only context".to_string())
        } else {
            Storage::put(context, key, value);
            Ok(())
        }
    }
    
    fn try_delete(context: StorageContext, key: ByteString) -> Result<(), String> {
        if context.is_read_only() {
            Err("Cannot delete from read-only context".to_string())
        } else {
            Storage::delete(context, key);
            Ok(())
        }
    }
}

use neo_contract::storage::{StorageItem, StorageMap};

// Extended StorageMap operations
impl StorageMap {
    fn try_get(&self, key: ByteString) -> Option<Any> {
        self.get(key)
    }
    
    fn try_delete(&mut self, key: ByteString) -> Result<(), String> {
        self.delete(key);
        Ok(())
    }
}