// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use neo_contract::prelude::*;

/// MinimalExample is a simple example contract demonstrating
/// the minimal requirements for a Neo N3 smart contract in Rust.
#[contract_author("Neo Contract Rust Team")]
#[contract_permission("*:*")]
#[contract_meta("Version", "1.0.0")]
pub struct MinimalExample {
    // Storage prefix for data
    data_prefix: ByteString,
}

#[contract_impl]
impl MinimalExample {
    pub fn init() -> Self {
        Self {
            data_prefix: ByteString::from_literal("data"),
        }
    }

    #[method]
    #[safe]
    pub fn name(&self) -> ByteString {
        ByteString::from_literal("MinimalExample")
    }

    #[method]
    #[safe]
    pub fn version(&self) -> ByteString {
        ByteString::from_literal("1.0.0")
    }

    #[method]
    pub fn store(&self, key: ByteString, value: ByteString) -> bool {
        // Check if the caller is authorized
        if !Runtime::check_witness_with_account(Runtime::get_executing_script_hash()) {
            return false;
        }

        // Store the value
        let storage_key = self.data_prefix.concat(&key);
        Storage::put(storage_key, value);
        true
    }

    #[method]
    #[safe]
    pub fn get(&self, key: ByteString) -> ByteString {
        let storage_key = self.data_prefix.concat(&key);
        Storage::get(storage_key).unwrap_or(ByteString::empty())
    }
}
