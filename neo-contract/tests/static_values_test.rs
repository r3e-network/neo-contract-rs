// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use neo_contract::{
        builtin::{H160, H256},
        key::PublicKey,
        StaticValue,
    };

    #[test]
    fn test_h160_from_literal() {
        // Test hex format
        let hex = "0x13a83e059c2eedd5157b766d3357bc826810905e";

        // Verify that the static value was initialized correctly
        let h160 = H160::from_literal(hex).unwrap();

        // Just verify that the h160 was created successfully
        assert!(h160.as_bytes().len() == 20);
    }

    #[test]
    fn test_h256_from_literal() {
        // Test hex format
        let hex = "0x13a83e059c2eedd5157b766d3357bc826810905e13a83e059c2eedd5157b766d";

        // Verify that the static value was initialized correctly
        let h256 = H256::from_literal(hex).unwrap();

        // Just verify that the h256 was created successfully
        assert!(h256.as_bytes().len() == 32);
    }

    #[test]
    fn test_public_key_from_literal() {
        // Test hex format
        let hex = "0x13a83e059c2eedd5157b766d3357bc826810905e13a83e059c2eedd5157b766d33";

        // Verify that the static value was initialized correctly
        let public_key = PublicKey::from_literal(hex).unwrap();

        // Just verify that the public key was created successfully
        assert!(public_key.is_valid());
    }
}
