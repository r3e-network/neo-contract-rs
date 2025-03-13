// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Role-based access control for Neo smart contracts
//! This module provides a framework for assigning roles to addresses
//! and implementing role-based access control.

use alloc::format;

use crate::prelude::{ByteString, StorageMap, H160};
use crate::runtime::Runtime;
use alloc::vec::Vec;

use crate::error::Result;
use crate::policy::Policy;

/// A role in the role-based access control system
#[derive(Debug, Clone, PartialEq)]
pub struct Role {
    /// Role identifier
    name: ByteString,
    /// Role description
    description: ByteString,
}

impl Role {
    /// Create a new role
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: ByteString::from(name),
            description: ByteString::from(description),
        }
    }

    /// Get the role name
    pub fn name(&self) -> ByteString { self.name.clone() }

    /// Get the role description
    pub fn description(&self) -> ByteString { self.description.clone() }
}

/// Role manager for handling role assignments
pub struct RoleManager {
    /// Prefix for role storage
    prefix: ByteString,
}

impl RoleManager {
    /// Create a new role manager
    pub fn new(prefix: &[u8]) -> Self { Self { prefix: ByteString::from_bytes(prefix) } }

    /// Assign a role to an address
    pub fn assign_role(&self, role: &Role, address: H160) -> Result<()> {
        let role_map = self.get_role_map(role);
        let _ = role_map.put(&address, &true);
        Ok(())
    }

    /// Remove a role from an address
    pub fn revoke_role(&self, role: &Role, address: H160) -> Result<()> {
        let role_map = self.get_role_map(role);
        let _ = role_map.delete(&address);
        Ok(())
    }

    /// Check if an address has a role
    pub fn has_role(&self, role: &Role, address: H160) -> bool {
        let role_map = self.get_role_map(role);
        // Properly unwrap the nested Option or default to false
        match role_map.get(&address) {
            Ok(Some(value)) => value,
            _ => false,
        }
    }

    /// Get all addresses with a specific role
    pub fn get_role_members(&self, role: &Role) -> Vec<H160> {
        let role_map = self.get_role_map(role);
        let mut members = Vec::new();

        // Create proper FindOptions for the search
        use crate::find_options::FindOptions;
        let _options = FindOptions::default();

        // Iterate through items and collect addresses with the role
        for item_result in role_map.iter() {
            // item_result is already a (H160, bool) tuple
            let (address, has_role) = item_result;
            if has_role {
                members.push(address);
            }
        }

        members
    }

    /// Create a policy that requires an address to have a role
    pub fn require_role(&self, role: &Role) -> RolePolicy {
        RolePolicy { role: role.clone(), role_manager: self.clone() }
    }

    /// Helper method to get the storage map for a role
    fn get_role_map(&self, role: &Role) -> StorageMap<H160, bool> {
        let key = format!("{}:{}", self.prefix, role.name());
        StorageMap::new(key)
    }
}

impl Clone for RoleManager {
    fn clone(&self) -> Self { Self { prefix: self.prefix.clone() } }
}

/// Common roles that can be used
pub mod common_roles {
    use super::Role;

    /// Admin role
    pub fn admin() -> Role { Role::new("admin", "Administrator with full access") }

    /// Owner role
    pub fn owner() -> Role { Role::new("owner", "Owner of the contract") }

    /// Operator role
    pub fn operator() -> Role { Role::new("operator", "Operator with privileged access") }

    /// User role
    pub fn user() -> Role { Role::new("user", "Regular user") }

    /// Pauser role
    pub fn pauser() -> Role { Role::new("pauser", "Can pause contract functions") }

    /// Minter role
    pub fn minter() -> Role { Role::new("minter", "Can mint new tokens") }

    /// Burner role
    pub fn burner() -> Role { Role::new("burner", "Can burn tokens") }
}

/// Policy that requires an address to have a role
pub struct RolePolicy {
    /// Role required
    role: Role,
    /// Role manager
    role_manager: RoleManager,
}

impl Policy for RolePolicy {
    fn allows(&self) -> bool {
        // Get the calling address
        let caller = Runtime::calling_script_hash();

        // Check if the caller has the required role
        self.role_manager.has_role(&self.role, caller)
    }

    fn demands(&self) -> bool { self.allows() }
}

/// Macro to define roles
#[macro_export]
macro_rules! define_roles {
    ($($name:ident => $description:expr),*) => {
        $(
            pub static $name: once_cell::sync::Lazy<Role> = once_cell::sync::Lazy::new(|| {
                Role::new(stringify!($name), $description)
            });
        )*
    };
}

/// Macro to enforce a role check
#[macro_export]
macro_rules! require_role {
    ($role_manager:expr, $role:expr) => {
        if !$role_manager.has_role(&$role, Runtime::calling_script_hash()) {
            return Err($crate::error::Error::new(
                $crate::error::ErrorCode::Unauthorized,
                concat!("Role required: ", stringify!($role)),
            ));
        }
    };

    ($role_manager:expr, $role:expr, $error_message:expr) => {
        if !$role_manager.has_role(&$role, Runtime::calling_script_hash()) {
            return Err($crate::error::Error::new($crate::error::ErrorCode::Unauthorized, $error_message));
        }
    };
}
