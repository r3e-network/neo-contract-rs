// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
pub struct Notification {
    sender: H160,
    script_hash: H160,
    state: Array<Any>,
}

impl Notification {
    #[inline(always)]
    pub fn sender(&self) -> H160 {
        self.sender
    }

    #[inline(always)]
    pub fn script_hash(&self) -> H160 {
        self.script_hash
    }

    // TODO: implement state
    // #[inline(always)]
    // pub fn state(&self) -> Array<Any> {
    //     self.state
    // }
}
