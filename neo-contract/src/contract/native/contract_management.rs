// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::contract::Contract as SimpleContract;
use crate::prelude::{Any, Array, ByteString, Int256, H160};
use crate::runtime::Runtime;
use crate::types::contract::Contract;

/// ContractManagement native contract
///
/// This contract handles deployment, update, and management of smart contracts
/// on the Neo N3 blockchain.
pub struct ContractManagement;

impl ContractManagement {
    /// Get the ContractManagement contract hash
    pub fn hash() -> H160 {
        // Official ContractManagement contract hash: 0xfffdc93764dbaddd97c48f252a53ea4643faa3fd
        let bytes = [
            0xff, 0xfd, 0xc9, 0x37, 0x64, 0xdb, 0xad, 0xdd, 0x97, 0xc4, 0x8f, 0x25, 0x2a, 0x53, 0xea, 0x46, 0x43, 0xfa,
            0xa3, 0xfd,
        ];
        H160::from_slice(&bytes)
    }

    /// Helper function to deserialize a contract from the native contract response
    fn deserialize_contract(contract_data: &Any) -> Option<SimpleContract> {
        if let Any::Array(contract_fields) = contract_data {
            // Contract data structure in Neo protocol:
            // [0]: script hash (ByteString)
            // [1]: manifest (Array or Map)
            // Within manifest:
            //   - name (String)
            //   - other manifest fields...

            // Extract script hash
            let script_hash = if let Some(Any::ByteString(hash_bytes)) = contract_fields.get(0) {
                if hash_bytes.len() != 20 {
                    return None;
                }

                let mut bytes = [0u8; 20];
                bytes.copy_from_slice(hash_bytes.as_bytes());
                H160(bytes)
            } else {
                return None;
            };

            // Extract name from manifest
            let name = if let Some(Any::Array(manifest)) = contract_fields.get(1) {
                if let Some(Any::ByteString(name_bytes)) = manifest.get(0) {
                    name_bytes.clone()
                } else {
                    // Default name if not found
                    ByteString::from("Unknown")
                }
            } else {
                // Default name if manifest not found
                ByteString::from("Unknown")
            };

            // Create a SimpleContract (the more accessible contract representation)
            return Some(SimpleContract::new(script_hash, name));
        }

        None
    }

    /// Deploy a new contract
    ///
    /// # Arguments
    ///
    /// * `nef_file` - The NEF (Neo Executable Format) file containing the contract code
    /// * `manifest` - The contract manifest
    /// * `data` - Optional data for the contract constructor
    ///
    /// # Returns
    ///
    /// The script hash of the newly deployed contract, or None if deployment failed
    pub fn deploy(nef_file: &[u8], manifest: &ByteString, data: Option<Any>) -> Option<H160> {
        let method = ByteString::from("deploy");
        let mut args = Array::new();

        // Add NEF file bytes as ByteString
        args.push(Any::byte_string(nef_file.to_vec()));

        // Add manifest
        args.push(Any::from(manifest.clone()));

        // Add data or null
        if let Some(init_data) = data {
            args.push(init_data);
        } else {
            args.push(Any::null());
        }

        let result = Runtime::call_contract(Self::hash(), method, args);

        // The contract should return the script hash of the deployed contract
        if let Any::ByteString(hash_bytes) = result {
            if hash_bytes.len() == 20 {
                let mut bytes = [0u8; 20];
                bytes.copy_from_slice(hash_bytes.as_bytes());
                return Some(H160(bytes));
            }
        }

        None
    }

    /// Update an existing contract
    ///
    /// # Arguments
    ///
    /// * `script_hash` - The script hash of the contract to update
    /// * `nef_file` - The new NEF file
    /// * `manifest` - The new contract manifest
    /// * `data` - Optional data for initialization
    ///
    /// # Returns
    ///
    /// Returns true if the update was successful, false otherwise
    pub fn update(script_hash: &H160, nef_file: &[u8], manifest: &ByteString, data: Option<Any>) -> bool {
        let method = ByteString::from("update");
        let mut args = Array::new();

        // Add script hash
        args.push(Any::from(script_hash.clone()));

        // Add NEF file bytes as ByteString
        args.push(Any::byte_string(nef_file.to_vec()));

        // Add manifest
        args.push(Any::from(manifest.clone()));

        // Add data or null
        if let Some(init_data) = data {
            args.push(init_data);
        } else {
            args.push(Any::null());
        }

        let result = Runtime::call_contract(Self::hash(), method, args);

        // Check if update was successful
        if let Any::Boolean(success) = result {
            return success;
        }

        false
    }

    /// Destroy a contract
    ///
    /// # Arguments
    ///
    /// * `script_hash` - The script hash of the contract to destroy
    ///
    /// # Returns
    ///
    /// Returns true if the contract was successfully destroyed
    pub fn destroy(script_hash: &H160) -> bool {
        let method = ByteString::from("destroy");
        let mut args = Array::new();

        // Add script hash
        args.push(Any::from(script_hash.clone()));

        let result = Runtime::call_contract(Self::hash(), method, args);

        // Check if destroy was successful
        if let Any::Boolean(success) = result {
            return success;
        }

        false
    }

    /// Get the minimum deployment fee
    ///
    /// # Returns
    ///
    /// The minimum fee required to deploy a contract, in GAS
    pub fn get_minimum_deployment_fee() -> Int256 {
        let method = ByteString::from("getMinimumDeploymentFee");
        let args = Array::new();

        let result = Runtime::call_contract(Self::hash(), method, args);

        // The result should be an integer representing the minimum fee
        if let Any::Integer(fee) = result {
            return fee;
        }

        Int256::zero()
    }

    /// Get contract details
    ///
    /// # Arguments
    ///
    /// * `script_hash` - The script hash of the contract to query
    ///
    /// # Returns
    ///
    /// An option containing the contract if it exists
    pub fn get_contract(script_hash: &H160) -> Option<SimpleContract> {
        let method = ByteString::from("getContract");
        let mut args = Array::new();

        // Add script hash
        args.push(Any::from(script_hash.clone()));

        let result = Runtime::call_contract(Self::hash(), method, args);

        // Deserialize the contract data
        Self::deserialize_contract(&result)
    }

    /// Check if a contract exists
    ///
    /// # Arguments
    ///
    /// * `script_hash` - The script hash to check
    ///
    /// # Returns
    ///
    /// Returns true if the contract exists, false otherwise
    pub fn has_contract(script_hash: &H160) -> bool {
        let method = ByteString::from("hasContract");
        let mut args = Array::new();

        // Add script hash
        args.push(Any::from(script_hash.clone()));

        let result = Runtime::call_contract(Self::hash(), method, args);

        // Check if the contract exists
        if let Any::Boolean(exists) = result {
            return exists;
        }

        false
    }

    /// Get the contract hash from a sender and nef checksum
    ///
    /// # Arguments
    ///
    /// * `sender` - The script hash of the sender
    /// * `nef_checksum` - The checksum of the NEF file
    ///
    /// # Returns
    ///
    /// The contract hash
    pub fn get_contract_hash_from_sender(sender: &H160, nef_checksum: u32) -> H160 {
        let method = ByteString::from("getContractHash");
        let mut args = Array::new();

        // Add sender script hash
        args.push(Any::from(sender.clone()));

        // Add NEF checksum
        args.push(Any::integer(nef_checksum));

        let result = Runtime::call_contract(Self::hash(), method, args);

        // Extract contract hash from result
        if let Any::ByteString(hash_bytes) = result {
            if hash_bytes.len() == 20 {
                let mut bytes = [0u8; 20];
                bytes.copy_from_slice(hash_bytes.as_bytes());
                return H160(bytes);
            }
        }

        H160::zero()
    }
}
