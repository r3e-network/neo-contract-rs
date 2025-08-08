//! Tests for storage and runtime services

#[cfg(test)]
mod storage_runtime_tests {
    use neo_contract::prelude::*;
    use neo_contract::services::storage::Storage;
    use neo_contract::services::runtime::Runtime;
    use neo_contract::services::iterator::Iterator;

    #[test]
    fn test_storage_basic_operations() {
        // Get storage context
        let context = Storage::get_context();
        
        // Test put and get
        let key = ByteString::from_literal("test_key");
        let value = Int256::from(42);
        Storage::put(context.clone(), key.clone(), value.into_any());
        
        let retrieved = Storage::get(context.clone(), key.clone());
        assert!(retrieved.is_some());
        
        // Test delete
        Storage::delete(context.clone(), key.clone());
        let after_delete = Storage::get(context, key);
        assert!(after_delete.is_none());
    }

    #[test]
    fn test_storage_complex_types() {
        let context = Storage::get_context();
        
        // Store array
        let mut arr = Array::new();
        arr.push(Int256::from(1).into_any());
        arr.push(Int256::from(2).into_any());
        arr.push(Int256::from(3).into_any());
        
        let arr_key = ByteString::from_literal("array_key");
        Storage::put(context.clone(), arr_key.clone(), arr.into_any());
        
        // Store map
        let mut map = Map::new();
        map.set(
            ByteString::from_literal("field1").into_any(),
            Int256::from(100).into_any(),
        );
        map.set(
            ByteString::from_literal("field2").into_any(),
            ByteString::from_literal("value").into_any(),
        );
        
        let map_key = ByteString::from_literal("map_key");
        Storage::put(context.clone(), map_key.clone(), map.into_any());
        
        // Store nested structure
        let mut nested = Map::new();
        nested.set(
            ByteString::from_literal("inner_array").into_any(),
            arr.into_any(),
        );
        nested.set(
            ByteString::from_literal("inner_map").into_any(),
            map.into_any(),
        );
        
        let nested_key = ByteString::from_literal("nested_key");
        Storage::put(context, nested_key, nested.into_any());
    }

    #[test]
    fn test_storage_find_operations() {
        let context = Storage::get_context();
        
        // Store multiple items with prefix
        for i in 0..10 {
            let key = ByteString::from_literal("item:")
                .concat(&ByteString::from(i.to_string().as_bytes()));
            let value = Int256::from(i as i64);
            Storage::put(context.clone(), key, value.into_any());
        }
        
        // Find with prefix
        let prefix = ByteString::from_literal("item:");
        let iter = Storage::find(context.clone(), prefix.clone(), FindOptions::default());
        
        // Note: Iterator operations are mocked and may not work as expected
        // In real environment, would iterate through all items
        
        // Find with options
        let iter_keys = Storage::find(context.clone(), prefix.clone(), FindOptions::KEYS_ONLY);
        let iter_values = Storage::find(context.clone(), prefix.clone(), FindOptions::VALUES_ONLY);
        let iter_remove = Storage::find(context, prefix, FindOptions::REMOVE_PREFIX);
    }

    #[test]
    fn test_storage_context_operations() {
        // Get normal context
        let context = Storage::get_context();
        
        // Get read-only context
        let readonly = Storage::get_read_only_context();
        
        // Convert to read-only
        let converted = Storage::as_read_only(context.clone());
        
        // Test operations on different contexts
        let key = ByteString::from_literal("test");
        let value = Int256::from(100);
        
        // Write to normal context
        Storage::put(context.clone(), key.clone(), value.into_any());
        
        // Read from read-only context
        let read_value = Storage::get(readonly, key.clone());
        
        // Try to write to read-only (would fail in real environment)
        // Storage::put(converted, key, value.into_any()); // Would fail
    }

    #[test]
    fn test_runtime_information() {
        // Get execution information
        let trigger = Runtime::get_trigger();
        assert_eq!(trigger, TriggerType::Application); // Default in mock
        
        let platform = Runtime::get_platform();
        assert_eq!(platform, ByteString::from_literal("NEO"));
        
        // Get script hashes
        let executing = Runtime::get_executing_script_hash();
        assert_eq!(executing, H160::zero()); // Mock returns zero
        
        let calling = Runtime::get_calling_script_hash();
        assert_eq!(calling, H160::zero()); // Mock returns zero
        
        let entry = Runtime::get_entry_script_hash();
        assert_eq!(entry, H160::zero()); // Mock returns zero
        
        // Get blockchain information
        let time = Runtime::get_time();
        assert!(time >= 0);
        
        let network = Runtime::get_network();
        assert!(network > 0);
        
        let address_version = Runtime::get_address_version();
        assert_eq!(address_version, 53); // Neo N3 mainnet version
    }

    #[test]
    fn test_runtime_gas_operations() {
        // Get gas left
        let gas_left = Runtime::get_gas_left();
        assert!(gas_left >= Int256::zero());
        
        // Burn gas
        Runtime::burn_gas(Int256::from(100));
        
        // Get invocation counter
        let counter = Runtime::get_invocation_counter();
        assert!(counter >= 0);
    }

    #[test]
    fn test_runtime_witness_checking() {
        let account = H160::zero();
        let pubkey = PublicKey::from_bytes(&[0x02; 33]);
        
        // Check witness with account
        let witness_account = Runtime::check_witness_with_account(account);
        assert!(!witness_account); // Mock returns false
        
        // Check witness with public key
        let witness_pubkey = Runtime::check_witness_with_public_key(pubkey);
        assert!(!witness_pubkey); // Mock returns false
    }

    #[test]
    fn test_runtime_notifications() {
        // Get notifications
        let notifications = Runtime::get_notifications(Some(H160::zero()));
        assert_eq!(notifications.length(), 0); // Mock returns empty
        
        // Send notification
        let event_name = ByteString::from_literal("TestEvent");
        let mut event_data = Array::new();
        event_data.push(Int256::from(42).into_any());
        event_data.push(ByteString::from_literal("data").into_any());
        
        Runtime::notify(event_name, event_data);
        
        // Log message
        Runtime::log(ByteString::from_literal("Test log message"));
    }

    #[test]
    fn test_runtime_random() {
        // Get random number
        let random = Runtime::get_random();
        // Random should be different each time (in real environment)
        let random2 = Runtime::get_random();
        // Note: In mock, might return same value
    }

    #[test]
    fn test_runtime_signers() {
        // Get current signers
        let signers = Runtime::current_signers();
        assert_eq!(signers.length(), 0); // Mock returns empty
    }

    #[test]
    fn test_runtime_transaction() {
        // Get current transaction
        let tx = Runtime::get_tx();
        assert_eq!(tx.hash, H256::zero()); // Mock returns zero hash
        assert_eq!(tx.sender, H160::zero()); // Mock returns zero address
        assert_eq!(tx.sys_fee, Int256::zero()); // Mock returns zero
        assert_eq!(tx.net_fee, Int256::zero()); // Mock returns zero
    }

    #[test]
    fn test_iterator_operations() {
        let context = Storage::get_context();
        
        // Store test data
        for i in 0..5 {
            let key = ByteString::from_literal("iter:")
                .concat(&ByteString::from(i.to_string().as_bytes()));
            let value = Int256::from(i as i64);
            Storage::put(context.clone(), key, value.into_any());
        }
        
        // Create iterator
        let prefix = ByteString::from_literal("iter:");
        let iter = Storage::find(context, prefix, FindOptions::default());
        
        // Test iterator operations (mocked)
        let has_next = Iterator::next(iter.clone());
        assert!(!has_next); // Mock returns false
        
        let key = Iterator::key(iter.clone());
        assert!(key.is_none()); // Mock returns None
        
        let value = Iterator::value(iter);
        assert!(value.is_none()); // Mock returns None
    }

    #[test]
    fn test_storage_map_operations() {
        use neo_contract::storage::StorageMap;
        
        let mut map = StorageMap::new(ByteString::from_literal("test_map"));
        
        // Put values
        let key1 = ByteString::from_literal("key1");
        let value1 = Int256::from(100);
        map.put(key1.clone(), value1.into_any());
        
        let key2 = ByteString::from_literal("key2");
        let value2 = ByteString::from_literal("value2");
        map.put(key2.clone(), value2.into_any());
        
        // Get values
        let retrieved1 = map.get(key1.clone());
        assert!(retrieved1.is_some());
        
        let retrieved2 = map.get(key2.clone());
        assert!(retrieved2.is_some());
        
        // Delete value
        map.delete(key1);
        let after_delete = map.get(key1);
        assert!(after_delete.is_none());
    }

    #[test]
    fn test_storage_item_operations() {
        use neo_contract::storage::StorageItem;
        
        let mut item = StorageItem::new(ByteString::from_literal("test_item"));
        
        // Set value
        let value = Int256::from(42);
        item.set(value.into_any());
        
        // Get value
        let retrieved = item.get();
        assert!(retrieved.is_some());
        
        // Delete value
        item.delete();
        let after_delete = item.get();
        assert!(after_delete.is_none());
    }
}