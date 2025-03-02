// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Versioned storage module for maintaining data versions and migrations
//! This helps contracts to maintain backward compatibility when storage
//! schema changes

use alloc::string::String;
use alloc::vec::Vec;
use crate::builtin::{H160, ByteString, Int256};
use crate::storage::{StorageMap, Storable, Storage};
use crate::error::{Error, ErrorCode, Result};
use crate::Runtime;

/// Represents a version of the contract storage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageVersion {
    /// Major version number
    pub major: u16,
    /// Minor version number
    pub minor: u16,
    /// Patch version number
    pub patch: u16,
}

impl StorageVersion {
    /// Create a new storage version
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }
    
    /// Create a version from a semver string
    pub fn from_semver(version_str: &str) -> Option<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        
        let major = parts[0].parse::<u16>().ok()?;
        let minor = parts[1].parse::<u16>().ok()?;
        let patch = parts[2].parse::<u16>().ok()?;
        
        Some(Self { major, minor, patch })
    }
    
    /// Convert to semver string
    pub fn to_semver(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
    
    /// Check if this version is greater than another
    pub fn is_greater_than(&self, other: &Self) -> bool {
        self.major > other.major || 
        (self.major == other.major && self.minor > other.minor) ||
        (self.major == other.major && self.minor == other.minor && self.patch > other.patch)
    }
    
    /// Check if this version is compatible with another
    /// (same major version, same or greater minor version)
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major && 
        (self.minor > other.minor || 
         (self.minor == other.minor && self.patch >= other.patch))
    }
}

impl Storable for StorageVersion {
    fn to_storage(&self) -> ByteString {
        ByteString::from(self.to_semver())
    }
    
    fn from_storage(data: &ByteString) -> Option<Self> {
        StorageVersion::from_semver(&data.as_string())
    }
}

/// Storage version manager
pub struct VersionedStorage {
    /// Prefix for version keys
    prefix: ByteString,
    /// Current version of the contract
    current_version: StorageVersion,
}

impl VersionedStorage {
    /// Create a new versioned storage manager
    pub fn new(prefix: &[u8], current_version: StorageVersion) -> Self {
        Self {
            prefix: ByteString::from_bytes(prefix),
            current_version,
        }
    }
    
    /// Initialize or migrate storage
    pub fn initialize_or_migrate(&self) -> Result<()> {
        // Get the stored version
        let stored_version = self.get_stored_version();
        
        // If no stored version, initialize
        if stored_version.is_none() {
            return self.initialize();
        }
        
        let stored_version = stored_version.unwrap();
        
        // If stored version is same as current, do nothing
        if stored_version == self.current_version {
            return Ok(());
        }
        
        // If stored version is greater than current, error
        if stored_version.is_greater_than(&self.current_version) {
            return Err(Error::new(
                ErrorCode::InvalidState,
                "Stored version is newer than current version"
            ));
        }
        
        // Perform migration
        self.migrate(stored_version)
    }
    
    /// Initialize the storage
    pub fn initialize(&self) -> Result<()> {
        // Store the current version
        self.store_version()?;
        
        // Emit initialization event
        self.emit_storage_initialized();
        
        Ok(())
    }
    
    /// Migrate from one version to another
    pub fn migrate(&self, from_version: StorageVersion) -> Result<()> {
        // Store the new version
        self.store_version()?;
        
        // Emit migration event
        self.emit_storage_migrated(from_version.clone(), self.current_version.clone());
        
        Ok(())
    }
    
    /// Register a migration handler
    pub fn register_migration_handler<F>(&self, from_version: StorageVersion, handler: F) -> Result<()>
    where
        F: FnOnce() -> Result<()>
    {
        // Get the stored version
        let stored_version = self.get_stored_version();
        
        // If no stored version or not matching, do nothing
        if stored_version.is_none() || stored_version.unwrap() != from_version {
            return Ok(());
        }
        
        // Execute the handler
        handler()
    }
    
    /// Get the stored version
    pub fn get_stored_version(&self) -> Option<StorageVersion> {
        let version_key = self.get_version_key();
        let version_map = StorageMap::<ByteString, StorageVersion>::new(b"");
        
        version_map.get(&version_key)
    }
    
    /// Store the current version
    fn store_version(&self) -> Result<()> {
        let version_key = self.get_version_key();
        let version_map = StorageMap::<ByteString, StorageVersion>::new(b"");
        
        version_map.put(&version_key, &self.current_version);
        
        Ok(())
    }
    
    /// Helper to get the version key
    fn get_version_key(&self) -> ByteString {
        let key = format!("{}:version", self.prefix);
        ByteString::from(key)
    }
    
    /// Emit storage initialized event
    fn emit_storage_initialized(&self) {
        let event_name = ByteString::from("StorageInitialized");
        let mut event_data = crate::builtin::Array::<crate::builtin::Any>::new();
        
        event_data.push(crate::builtin::Any::from(self.current_version.to_storage()));
        
        Runtime::notify(&event_name, &event_data);
    }
    
    /// Emit storage migrated event
    fn emit_storage_migrated(&self, from_version: StorageVersion, to_version: StorageVersion) {
        let event_name = ByteString::from("StorageMigrated");
        let mut event_data = crate::builtin::Array::<crate::builtin::Any>::new();
        
        event_data.push(crate::builtin::Any::from(from_version.to_storage()));
        event_data.push(crate::builtin::Any::from(to_version.to_storage()));
        
        Runtime::notify(&event_name, &event_data);
    }
}

/// Versioned contract storage trait
pub trait VersionedContract {
    /// Get the current storage version
    fn get_storage_version() -> StorageVersion;
    
    /// Initialize or migrate storage
    fn initialize_or_migrate_storage() -> Result<()>;
    
    /// Add data migration logic
    fn register_migrations(versioned_storage: &VersionedStorage) -> Result<()>;
}
