// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Contract upgrade pattern implementation
//! This module provides a standard way to make contracts upgradeable

// use crate::builtin::{H160, ByteString, Array, Any};
use crate::prelude::{H160, ByteString, Array, Any};
use crate::runtime::Runtime;
// use crate::types::context::StorageContext;
// use crate::storage::{StorageMap, Storable};
use crate::policy::voting::Storable;
// use crate::contract::native::neo;
use crate::error::{Error, ErrorCode, Result};

/// Keys for the upgrade pattern
pub mod keys {
    /// Key for storing the admin address
    pub const ADMIN: &[u8] = b"upgrade:admin";
    /// Key for storing the owner address
    pub const OWNER: &[u8] = b"upgrade:owner";
    /// Key for storing the upgrade script
    pub const UPGRADE_SCRIPT: &[u8] = b"upgrade:script";
}

/// Trait for contract upgrade functionality
pub trait Upgradeable {
    /// Set the admin address that can manage the contract
    fn set_admin(address: H160) -> Result<()>;
    
    /// Get the current admin address
    fn get_admin() -> Option<H160>;
    
    /// Set the owner address
    fn set_owner(address: H160) -> Result<()>;
    
    /// Get the current owner address
    fn get_owner() -> Option<H160>;
    
    /// Upgrade the contract with a new script
    fn upgrade(script: ByteString, manifest: ByteString) -> Result<()>;
    
    /// Check if the admin is the signer
    fn check_admin() -> Result<()>;
    
    /// Check if the owner is the signer
    fn check_owner() -> Result<()>;
}

/// Default implementation for contract upgrade functionality
pub struct ContractUpgrade;

impl Upgradeable for ContractUpgrade {
    fn set_admin(address: H160) -> Result<()> {
        // Check if current caller is the current admin
        if let Some(current_admin) = Self::get_admin() {
            if !Runtime::check_witness(&current_admin) {
                return Err(Error::with_message(ErrorCode::Unauthorized, "Only current admin can set a new admin"));
            }
        }
        
        // Use storage module directly instead of context methods
        let admin_key = keys::ADMIN;
        let bytes = address.as_bytes();
        
        // Store the admin address in storage
        crate::storage::put(admin_key, bytes);
        
        Ok(())
    }
    
    fn get_admin() -> Option<H160> {
        let admin_key = keys::ADMIN;
        
        // Get the admin address from storage
        if let Some(bytes) = crate::storage::get(admin_key) {
            if bytes.len() == 20 { // H160 is 20 bytes
                let mut array = [0u8; 20];
                array.copy_from_slice(&bytes);
                Some(H160::from_slice(&array))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn set_owner(address: H160) -> Result<()> {
        // Check if current caller is the admin
        Self::check_admin()?;
        
        let owner_key = keys::OWNER;
        let bytes = address.as_bytes();
        
        // Store the owner address in storage
        crate::storage::put(owner_key, bytes);
        
        Ok(())
    }
    
    fn get_owner() -> Option<H160> {
        let owner_key = keys::OWNER;
        
        // Get the owner address from storage
        if let Some(bytes) = crate::storage::get(owner_key) {
            if bytes.len() == 20 { // H160 is 20 bytes
                let mut array = [0u8; 20];
                array.copy_from_slice(&bytes);
                Some(H160::from_slice(&array))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn upgrade(script: ByteString, manifest: ByteString) -> Result<()> {
        // Check if current caller is the admin
        Self::check_admin()?;
        
        // Store the upgrade script for history tracking
        let script_key = keys::UPGRADE_SCRIPT;
        
        // Store the script in storage
        crate::storage::put(script_key, script.as_bytes());
        
        // Call the deploy method on the new contract
        let script_hash = H160::from_slice(script.as_bytes());
        let mut args = Array::new();
        args.push(Any::byte_string("deploy"));
        
        // Call the contract
        let result = Runtime::call_contract(
            script_hash, 
            ByteString::from("deploy"), 
            args
        );
        
        // Check if the call was successful
        if result.is_null() {
            Err(Error::with_message(ErrorCode::ExecutionError, "Contract upgrade failed"))
        } else {
            Ok(())
        }
    }
    
    fn check_admin() -> Result<()> {
        if let Some(admin) = Self::get_admin() {
            if !Runtime::check_witness(&admin) {
                return Err(Error::with_message(ErrorCode::Unauthorized, "Admin signature required"));
            }
            Ok(())
        } else {
            Err(Error::with_message(ErrorCode::InvalidState, "Admin not set"))
        }
    }
    
    fn check_owner() -> Result<()> {
        if let Some(owner) = Self::get_owner() {
            if !Runtime::check_witness(&owner) {
                return Err(Error::with_message(ErrorCode::Unauthorized, "Owner signature required"));
            }
            Ok(())
        } else {
            Err(Error::with_message(ErrorCode::InvalidState, "Owner not set"))
        }
    }
}

/// Helper macro to initialize a contract with an admin and owner
#[macro_export]
macro_rules! initialize_upgradeable {
    ($admin:expr) => {
        pub fn initialize_contract(admin: H160) -> bool {
            if $crate::contract::upgrade::ContractUpgrade::get_admin().is_some() {
                return false; // Already initialized
            }
            
            match $crate::contract::upgrade::ContractUpgrade::set_admin(admin) {
                Ok(_) => {
                    // Set the owner to be the same as admin initially
                    match $crate::contract::upgrade::ContractUpgrade::set_owner(admin) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                }
                Err(_) => false,
            }
        }
    };
}

/// Builder for creating an upgradeable contract
pub struct UpgradeableContractBuilder {
    /// Admin address for the contract
    admin: Option<H160>,
    /// Owner address for the contract
    owner: Option<H160>,
}

impl UpgradeableContractBuilder {
    /// Create a new upgradeable contract builder
    pub fn new() -> Self {
        Self {
            admin: None,
            owner: None,
        }
    }
    
    /// Set the admin address
    pub fn with_admin(mut self, admin: H160) -> Self {
        self.admin = Some(admin);
        self
    }
    
    /// Set the owner address
    pub fn with_owner(mut self, owner: H160) -> Self {
        self.owner = Some(owner);
        self
    }
    
    /// Initialize the contract with the specified admin and owner
    pub fn initialize(self) -> Result<()> {
        // Admin is required
        let admin = self.admin.ok_or_else(|| 
            Error::with_message(ErrorCode::InvalidArgument, "Admin address is required")
        )?;
        
        // Set the admin
        ContractUpgrade::set_admin(admin)?;
        
        // Set the owner if provided, otherwise use admin
        let owner = self.owner.unwrap_or(admin);
        ContractUpgrade::set_owner(owner)?;
        
        Ok(())
    }
}
