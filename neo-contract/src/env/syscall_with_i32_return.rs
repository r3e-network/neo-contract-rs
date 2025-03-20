// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Syscalls that return i32 values
//!
//! This module provides syscalls that return i32 values, which are typically
//! used for functions that return handles or status codes.

/// A wrapper around a system call number that returns an i32
#[derive(Debug, Clone, Copy)]
pub struct SystemCall(pub usize);

/// System runtime calls that get script container information
pub static system_runtime_get_script_container: SystemCall = SystemCall(0);

/// System runtime calls that get script container type
pub static system_runtime_get_script_container_type: SystemCall = SystemCall(1);

/// System storage find call
pub static system_storage_find: SystemCall = SystemCall(2);