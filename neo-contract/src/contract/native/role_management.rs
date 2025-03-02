// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::builtin::{H160, ByteString, Array, Any};
use crate::Runtime;
use crate::Role;

/// RoleManagement native contract
pub struct RoleManagement;

impl RoleManagement {
    /// Get the RoleManagement contract hash
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { 
            let h160 = crate::env::contract::native_role_management_contract_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(h160.0.as_slice()).unwrap()
        }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0x49cf4e5378ffcd4dec034fd98a174c5491e395e2").unwrap_or_else(H160::zero)
    }
    
    /// Get the designated nodes by role
    pub fn get_designated_by_role(role: Role, index: u32) -> Array<ByteString> {
        let method = ByteString::from("getDesignatedByRole");
        let mut args = Array::<Any>::new();
        args.push(Any::from(role as u8));
        args.push(Any::from(index as i64));
        
        let result = Runtime::call_contract(
            RoleManagement::hash(),
            method,
            args
        );
        
        match Array::<ByteString>::try_from(result) {
            Ok(nodes) => nodes,
            Err(_) => Array::<ByteString>::new(),
        }
    }
}
