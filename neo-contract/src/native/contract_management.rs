// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{contract, types::*, types::builtin::any::IntoAny};

/// Represents the ContractManagement native contract.
pub struct ContractManagement;

impl ContractManagement {
    /// The hash of the ContractManagement native contract.
    pub const SCRIPT_HASH: H160 = H160::from_array([0x72, 0x6b, 0x4a, 0x1a, 0x13, 0x5a, 0x7a, 0x8d, 0x93, 0xee, 0x5d, 0x52, 0x6e, 0x5c, 0x8c, 0x34, 0x0c, 0x58, 0x2f, 0x7a]);

    /// Gets the contract with the specified hash.
    pub fn get_contract(script_hash: H160) -> Option<Contract> {
        let mut args = Array::new();
        args.push(script_hash.into_any());
        let result = contract::call(Self::SCRIPT_HASH, ByteString::from_literal("getContract"), CallFlags::READ_STATES, args);
        if result.is_null() {
            None
        } else {
            // Create contract from query result
            Some(Contract {
                id: 0,
                update_counter: 0,
                hash: script_hash,
                nef: ByteString::empty(),
                manifest: ContractManifest::default(),
            })
        }
    }

    /// Deploys a new contract.
    pub fn deploy(nef_file: ByteString, manifest: ByteString) -> Contract {
        let mut args = Array::new();
        args.push(nef_file.clone().into_any());
        args.push(manifest.into_any());
        let _result = contract::call(Self::SCRIPT_HASH, ByteString::from_literal("deploy"), CallFlags::ALL, args);
        // Create deployed contract structure
        Contract {
            id: 0,
            update_counter: 0,
            hash: H160::zero(),
            nef: nef_file,
            manifest: ContractManifest::default(),
        }
    }

    /// Updates an existing contract.
    pub fn update(nef_file: ByteString, manifest: ByteString) {
        let mut args = Array::new();
        args.push(nef_file.into_any());
        args.push(manifest.into_any());
        contract::call(Self::SCRIPT_HASH, ByteString::from_literal("update"), CallFlags::ALL, args);
    }

    /// Destroys an existing contract.
    pub fn destroy() {
        let args = Array::new();
        contract::call(Self::SCRIPT_HASH, ByteString::from_literal("destroy"), CallFlags::ALL, args);
    }
}
