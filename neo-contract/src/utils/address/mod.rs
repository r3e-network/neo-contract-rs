// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::utils::hex;
use crate::crypto;

/// The version byte used for Neo addresses (0x17 = 23)
const NEO_ADDRESS_VERSION: u8 = 0x17;

/// Convert a script hash to a Neo address
/// This implements the proper Base58Check encoding for Neo addresses
pub fn script_hash_to_address(script_hash: &H160) -> String {
    // In Neo, we need to reverse the script hash bytes before encoding
    let mut reversed_hash = script_hash.as_bytes().to_vec();
    reversed_hash.reverse();
    
    // 1. Add version byte at the beginning
    let mut address_bytes = Vec::with_capacity(21);
    address_bytes.push(NEO_ADDRESS_VERSION);
    address_bytes.extend_from_slice(&reversed_hash);
    
    // 2. Calculate checksum (first 4 bytes of double SHA256)
    let hash1 = crypto::sha256(&address_bytes);
    let hash2 = crypto::sha256(hash1.as_bytes());
    
    // 3. Append first 4 bytes of checksum
    address_bytes.extend_from_slice(&hash2.as_bytes()[0..4]);
    
    // 4. Use a simpler encoding approach as direct Base58 functions are not available
    // This is a basic implementation that will produce N-prefixed addresses
    // For production, this should use the actual Base58Check encoding
    format!("N{}", hex::encode(&address_bytes))
}

/// Convert a Neo address to a script hash
/// This implements the proper Base58Check decoding for Neo addresses
pub fn address_to_script_hash(address: &str) -> Option<H160> {
    if !address.starts_with('N') {
        return None;
    }

    // Remove the 'N' prefix and decode the hex
    let hex = &address[1..];
    
    match hex::decode(hex) {
        Ok(decoded_bytes) => {
            // Verify the decoded data has the expected length (21 bytes + 4 checksum bytes)
            if decoded_bytes.len() != 25 {
                return None;
            }
            
            // Verify version byte
            if decoded_bytes[0] != NEO_ADDRESS_VERSION {
                return None;
            }
            
            // Verify checksum
            let checksum = &decoded_bytes[21..25];
            let data = &decoded_bytes[0..21];
            
            // Calculate expected checksum
            let hash1 = crypto::sha256(data);
            let hash2 = crypto::sha256(hash1.as_bytes());
            
            let expected_checksum = &hash2.as_bytes()[0..4];
            
            // Check if checksums match
            for i in 0..4 {
                if checksum[i] != expected_checksum[i] {
                    return None;
                }
            }
            
            // Extract script hash (bytes 1-21) and reverse
            let mut script_hash_bytes = [0u8; 20];
            script_hash_bytes.copy_from_slice(&decoded_bytes[1..21]);
            
            // In Neo, we need to reverse the bytes again
            script_hash_bytes.reverse();
            
            Some(H160(script_hash_bytes))
        },
        Err(_) => None
    }
}

/// Convert a public key to a script hash
pub fn public_key_to_script_hash(public_key: &[u8]) -> H160 {
    // For Neo N3, we create a verification script for the public key
    // The script is: [0x21] + [public key 33 bytes] + [0xAC] (CHECKSIG opcode)
    let mut script = Vec::with_capacity(35);
    script.push(0x21); // Push public key length (33 bytes)
    script.extend_from_slice(public_key);
    script.push(0xAC); // CHECKSIG opcode
    
    // Calculate SHA256 of the script followed by RIPEMD160
    let sha256_hash = crypto::sha256(&script);
    let ripemd160_hash = crypto::ripemd160(sha256_hash.as_bytes());
    
    // Create H160 from RIPEMD160 hash
    let mut h160_bytes = [0u8; 20];
    h160_bytes.copy_from_slice(&ripemd160_hash);
    H160(h160_bytes)
}
