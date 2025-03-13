// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::prelude::{Any, Array, ByteString, H160};
use crate::runtime::Runtime;

pub mod native;
pub mod nep11;
pub mod nep17;
pub mod nep5;
pub mod upgrade;

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
    pub fn new(script_hash: H160, name: ByteString) -> Self { Self { script_hash, name } }

    /// Get the script hash of the contract
    pub fn script_hash(&self) -> H160 { self.script_hash.clone() }

    /// Get the name of the contract
    pub fn name(&self) -> ByteString { self.name.clone() }

    /// Call a method on this contract
    pub fn call<T>(&self, method: &str, args: Array) -> Option<T>
    where T: TryFrom<Any> {
        let method_bs = ByteString::from(method);
        let result = Runtime::call_contract(self.script_hash.clone(), method_bs, args);

        T::try_from(result).ok()
    }

    /// Get the current executing contract
    pub fn executing() -> Self {
        let script_hash = Runtime::executing_script_hash();
        Self { script_hash, name: ByteString::from("Executing") }
    }

    /// Get the current calling contract
    pub fn calling() -> Self {
        let script_hash = Runtime::calling_script_hash();
        Self { script_hash, name: ByteString::from("Calling") }
    }
}
