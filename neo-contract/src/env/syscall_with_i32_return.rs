// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! System calls that return i32 values.
//! This module provides access to Neo VM system calls with integer return values.

#[allow(unused)]
pub struct SystemCall(pub usize);

/// System runtime get script container
pub static system_runtime_get_script_container: SystemCall = SystemCall(0);

/// System runtime get script container type
pub static system_runtime_get_script_container_type: SystemCall = SystemCall(1);

/// System storage find
pub static system_storage_find: SystemCall = SystemCall(2);