// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

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
}
