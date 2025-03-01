// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::runtime;
use crate::storage::map::StorageMap;
use crate::types::builtin::string::ByteString;
use crate::types::consts::*;

pub mod native;
pub mod nep11;
pub mod nep17;
pub mod nep5;

/// Contract represents a Neo contract
pub struct Contract;

impl Contract {
    /// Get the script hash of the contract
    pub fn script_hash() -> ByteString {
        runtime::get_executing_script_hash().to_hex_string().into()
    }

    /// Get the name of the contract
    pub fn name() -> ByteString {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_NAME_KEY])).get_string("")
            .unwrap_or_else(|| ByteString::from(DEFAULT_NAME))
    }

    /// Get the version of the contract
    pub fn version() -> ByteString {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_VERSION_KEY])).get_string("")
            .unwrap_or_else(|| ByteString::from(DEFAULT_VERSION))
    }

    /// Get the author of the contract
    pub fn author() -> ByteString {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_AUTHOR_KEY])).get_string("")
            .unwrap_or_else(|| ByteString::from(DEFAULT_AUTHOR))
    }

    /// Get the email of the contract
    pub fn email() -> ByteString {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_EMAIL_KEY])).get_string("")
            .unwrap_or_else(|| ByteString::from(DEFAULT_EMAIL))
    }

    /// Get the description of the contract
    pub fn description() -> ByteString {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_DESCRIPTION_KEY])).get_string("")
            .unwrap_or_else(|| ByteString::from(DEFAULT_DESCRIPTION))
    }
}
