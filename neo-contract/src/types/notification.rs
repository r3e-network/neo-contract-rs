// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[crate::inner_structs]
pub struct Notification {
    #[get(pub)]
    sender: H160,

    #[get(pub)]
    script_hash: H160,

    #[get(pub)]
    state: Array<Any>,
}
