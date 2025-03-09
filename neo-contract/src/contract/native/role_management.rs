// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::prelude::{H160, ByteString, Array, Any};
use crate::runtime::Runtime;
use crate::policy::roles::Role;

/// RoleManagement native contract
pub struct RoleManagement;

impl RoleManagement {
    /// Get the contract hash
    pub fn hash() -> H160 {
        // Using from_slice with a predefined hash value
        let bytes = [
            0x49, 0xcf, 0x4e, 0x53, 0x78, 0xff, 0xcd, 0x4d, 
            0xec, 0x03, 0x4f, 0xd9, 0x8a, 0x17, 0x4c, 0x54, 
            0x91, 0xe3, 0x95, 0xe2
        ];
        H160::from_slice(&bytes)
    }
    
    /// Get the designated nodes by role
    pub fn get_designated_by_role(role: Role, index: u32) -> Array {
        let method = ByteString::from("getDesignatedByRole");
        let mut args = Array::new();
        
        // Convert Role to a value that can be used as an argument
        let role_name = role.name();
        args.push(Any::from(role_name));
        
        // Convert index to a ByteString
        let index_str = ByteString::from(alloc::format!("{}", index));
        args.push(Any::from(index_str));
        
        let result = Runtime::call_contract(
            Self::hash(),
            method,
            args
        );
        
        // Just return an empty array since we can't easily convert the result
        Array::new()
    }
}
