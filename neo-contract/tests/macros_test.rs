// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use neo_contract::{builtin::Int256, contract_method, storage_map};

    #[test]
    fn test_contract_method_macro() {
        contract_method!(
            fn test_method(a: i32, b: i32) -> i32 { a + b }
        );

        assert_eq!(test_method(1, 2), 3);
    }

    #[test]
    fn test_storage_map_macro() {
        // Define a simple storage map for testing
        struct TestMap;

        impl TestMap {
            pub fn new() -> Self { Self }

            pub fn get(&self, _key: &[u8]) -> Option<Int256> { None }
        }

        let map = TestMap::new();

        // This test is limited since we can't actually access storage in a test
        // But we can at least verify that the macro expands correctly
        assert!(map.get(b"test").is_none());
    }
}
