// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{env, types::*};


#[inline(always)]
pub fn notify(event_name: ByteString, args: Array<Any>) {
    unsafe { env::syscall::system_runtime_notify(event_name, args) }
}