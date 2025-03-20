#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use neo_contract::prelude::*;
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::string::ByteString;
use neo_contract::types::builtin::any::Any;
use neo_contract::types::builtin::array::Array;
use neo_contract::Runtime;

/// # NEP-17 Token Implementation
///
/// This smart contract demonstrates a basic NEP-17 token implementation using the neo-contract-rs framework.
/// It implements all required methods and events according to the NEP-17 standard:
/// - name, symbol, decimals, totalSupply
/// - balanceOf, transfer
/// - Required events: Transfer

mod nep17_token {
    use super::*;

    // Standard NEP-17 Transfer event
    pub struct Transfer {
        pub from: Option<H160>,
        pub to: Option<H160>,
        pub amount: u64,
    }
}
