// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Contract upgrade pattern implementation
//! This module provides a standard way to make contracts upgradeable

use crate::builtin::{H160, ByteString, Array, Any};
use crate::Runtime;
use crate::types::context::StorageContext;
use crate::storage::{StorageMap, Storable};
use crate::contract::native::neo;
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
        let context = Runtime::storage_context();
        let admin_map = StorageMap::<ByteString, H160>::new(b"");
        let admin_key = ByteString::from_bytes(keys::ADMIN);
        
        // Check if current caller is the current admin
        if let Some(current_admin) = Self::get_admin() {
            if !Runtime::check_witness(current_admin) {
                return Err(Error::new(ErrorCode::Unauthorized, "Only current admin can set a new admin"));
            }
        }
        
        admin_map.put(&admin_key, &address);
        Ok(())
    }
    
    fn get_admin() -> Option<H160> {
        let context = Runtime::storage_context();
        let admin_map = StorageMap::<ByteString, H160>::new(b"");
        let admin_key = ByteString::from_bytes(keys::ADMIN);
        
        admin_map.get(&admin_key)
    }
    
    fn set_owner(address: H160) -> Result<()> {
        let context = Runtime::storage_context();
        let owner_map = StorageMap::<ByteString, H160>::new(b"");
        let owner_key = ByteString::from_bytes(keys::OWNER);
        
        // Check if current caller is the admin
        Self::check_admin()?;
        
        owner_map.put(&owner_key, &address);
        Ok(())
    }
    
    fn get_owner() -> Option<H160> {
        let context = Runtime::storage_context();
        let owner_map = StorageMap::<ByteString, H160>::new(b"");
        let owner_key = ByteString::from_bytes(keys::OWNER);
        
        owner_map.get(&owner_key)
    }
    
    fn upgrade(script: ByteString, manifest: ByteString) -> Result<()> {
        // Check if current caller is the admin
        Self::check_admin()?;
        
        // Store the upgrade script for history tracking
        let context = Runtime::storage_context();
        let script_map = StorageMap::<ByteString, ByteString>::new(b"");
        let script_key = ByteString::from_bytes(keys::UPGRADE_SCRIPT);
        
        script_map.put(&script_key, &script);
        
        // Call the Neo.Contract.Update system call
        let method = ByteString::from("update");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script));
        args.push(Any::from(manifest));
        
        let contract_hash = Runtime::executing_script_hash();
        let result = Runtime::call_contract(contract_hash, method, args);
        
        // Check if the upgrade was successful
        match bool::try_from(result) {
            Ok(true) => Ok(()),
            _ => Err(Error::new(ErrorCode::ContractCallError, "Contract upgrade failed")),
        }
    }
    
    fn check_admin() -> Result<()> {
        if let Some(admin) = Self::get_admin() {
            if !Runtime::check_witness(admin) {
                return Err(Error::new(ErrorCode::Unauthorized, "Admin signature required"));
            }
            Ok(())
        } else {
            Err(Error::new(ErrorCode::InvalidState, "Admin not set"))
        }
    }
    
    fn check_owner() -> Result<()> {
        if let Some(owner) = Self::get_owner() {
            if !Runtime::check_witness(owner) {
                return Err(Error::new(ErrorCode::Unauthorized, "Owner signature required"));
            }
            Ok(())
        } else {
            Err(Error::new(ErrorCode::InvalidState, "Owner not set"))
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
            Error::new(ErrorCode::InvalidArgument, "Admin address is required")
        )?;
        
        // Set the admin
        ContractUpgrade::set_admin(admin)?;
        
        // Set the owner if provided, otherwise use admin
        let owner = self.owner.unwrap_or(admin);
        ContractUpgrade::set_owner(owner)?;
        
        Ok(())
    }
}
