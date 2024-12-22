// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{contract::*, runtime, storage::StorageMap, types::*};

pub const DEFAULT_TOTAL_SUPPLY_KEY: u8 = 0x00;
pub const DEFAULT_BALANCE_KEY: u8 = 0x01;

pub trait Nep17Token: TokenContract {
    fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        if amount.is_negative() {
            runtime::abort();
            return false; // unreachable
        }

        if runtime::check_witness_with_account(from) {
            return false;
        }

        if amount.is_positive() {
            //
        }

        return true;
    }

    // fn transfer_with_data(from: H160, to: H160, amount: Int256, data: Any) -> bool;

    fn mint(_account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::abort();
            return;
        }
    }

    fn burn(_account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::abort();
            return;
        }
    }
}

pub fn update_nep17_balance<const BALANCE_PREFIX: u8>(_account: H160, _amount: Int256) {
    let storage = StorageMap::new();
}
