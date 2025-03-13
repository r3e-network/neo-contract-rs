// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

// use crate::builtin::{H160, ByteString, Int256, Array, Any};
use crate::prelude::{Any, Array, ByteString, Int256, H160};
use crate::runtime::Runtime;

/// GAS native contract
pub struct Gas;

/// GAS script hash
pub const SCRIPT_HASH: H160 = H160([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01,
]);

impl Gas {
    /// Get the GAS contract hash
    pub fn hash() -> H160 { SCRIPT_HASH }

    /// Transfer GAS from one account to another
    pub fn transfer(from: H160, to: H160, amount: Int256, data: Option<ByteString>) -> bool {
        let method = ByteString::from("transfer");
        let mut args = Array::new();

        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));

        if let Some(data_bs) = data {
            args.push(Any::from(data_bs));
        } else {
            args.push(Any::null());
        }

        let result = Runtime::call_contract(Self::hash(), method, args);

        // Check for success
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }

    /// Get the symbol of the GAS token
    pub fn symbol() -> ByteString {
        let method = ByteString::from("symbol");
        let args = Array::new();

        let result = Runtime::call_contract(Gas::hash(), method, args);

        // Extract ByteString value
        if let Any::ByteString(value) = result {
            value
        } else {
            ByteString::from("GAS")
        }
    }

    /// Get the decimals of the GAS token
    pub fn decimals() -> u8 {
        let method = ByteString::from("decimals");
        let args = Array::new();

        let result = Runtime::call_contract(Gas::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            // Convert to u8 (GAS has 8 decimals)
            // Note: Using as u8 will automatically truncate to the range 0-255
            let value_u8 = value.to_u64().unwrap_or(8) as u8;
            value_u8
        } else {
            8 // GAS has 8 decimals
        }
    }

    /// Get the total supply of the GAS token
    pub fn total_supply() -> Int256 {
        let method = ByteString::from("totalSupply");
        let args = Array::new();

        let result = Runtime::call_contract(Gas::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get the balance of GAS for an account
    pub fn balance_of(account: H160) -> Int256 {
        let method = ByteString::from("balanceOf");
        let mut args = Array::new();
        args.push(Any::from(account));

        let result = Runtime::call_contract(Gas::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get the name of the GAS token
    pub fn name() -> ByteString {
        let method = ByteString::from("name");
        let args = Array::new();

        let result = Runtime::call_contract(Gas::hash(), method, args);

        // Extract ByteString value
        if let Any::ByteString(value) = result {
            value
        } else {
            ByteString::from("GAS")
        }
    }
}
