// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    // Import the proc macros for testing
    use neo_contract_proc_macros::{
        byte_array, contract_hash, contract_permission, contract_trust, hash160, integer, manifest_extra, public_key,
        string, supported_standards,
    };

    #[test]
    fn test_static_field_initialization() {
        // This is a compile-time test, so it will pass if it compiles
        #[byte_array("0102030405")]
        static BYTE_ARRAY: [u8; 5] = [0; 5];

        #[hash160("0102030405060708090a0b0c0d0e0f1011121314")]
        static HASH160: [u8; 20] = [0; 20];

        #[integer("42")]
        static INTEGER: i64 = 0;

        #[public_key("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c")]
        static PUBLIC_KEY: [u8; 33] = [0; 33];

        #[string("Hello, Neo!")]
        static STRING: &str = "";

        #[contract_hash("0102030405060708090a0b0c0d0e0f1011121314")]
        static CONTRACT_HASH: [u8; 20] = [0; 20];

        // Verify the values
        assert_eq!(BYTE_ARRAY, [1, 2, 3, 4, 5]);
        assert_eq!(INTEGER, 42);
        assert_eq!(STRING, "Hello, Neo!");
    }

    // Test contract structure attributes
    #[test]
    fn test_contract_structure_attributes() {
        // This is a compile-time test, so it will pass if it compiles
        #[manifest_extra("Author", "Neo Team")]
        #[manifest_extra("Email", "dev@neo.org")]
        #[manifest_extra("Description", "Neo Smart Contract")]
        #[contract_permission("0102030405060708090a0b0c0d0e0f1011121314", "transfer", "balanceOf")]
        #[contract_trust("0102030405060708090a0b0c0d0e0f1011121314")]
        #[supported_standards("NEP-17", "NEP-11")]
        struct TestContract;
    }
}
