// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub const MINIMUM_RESPONSE_FEE: u64 = 0_10000000;

pub struct Oracle;

impl Oracle {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_oracle_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xfe924b7cfe89ddd271abaf7210a80a7e11178758")
    }

    #[inline(always)]
    pub fn get_price() -> Int256 { unsafe { env::contract::native_oracle_get_price() } }

    #[inline(always)]
    pub fn request(
        url: ByteString,
        filter: ByteString,
        callback: ByteString,
        user_data: Any,
        gas_for_response: Int256,
    ) {
        unsafe { env::contract::native_oracle_request(url, filter, callback, user_data, gas_for_response); }
    }
}
