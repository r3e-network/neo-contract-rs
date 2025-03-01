// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;

pub mod native;
pub mod nep11;
pub mod nep17;
pub mod nep5;

/// Contract represents a Neo N3 smart contract
#[derive(Debug, Clone)]
pub struct Contract {
    /// The script hash of the contract
    script_hash: H160,
    /// The name of the contract
    name: ByteString,
}

impl Contract {
    /// Create a new contract
    pub fn new(script_hash: H160, name: ByteString) -> Self {
        Self { script_hash, name }
    }

    /// Get the script hash of the contract
    pub fn script_hash(&self) -> H160 {
        self.script_hash.clone()
    }

    /// Get the name of the contract
    pub fn name(&self) -> ByteString {
        self.name.clone()
    }
}
