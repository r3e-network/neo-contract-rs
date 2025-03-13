//! Versioned Storage for Neo Contract RS
//!
//! This module provides versioned storage for Neo smart contracts.

use super::context::Context;
use super::item::{Codec, Item};
use crate::error::Result;
use alloc::vec::Vec;
use core::marker::PhantomData;

/// A trait for types that can be migrated between versions
pub trait Migrate {
    /// Migrates a value from the given version to the current version
    fn migrate(bytes: &[u8], version: u32) -> Result<Self>
    where Self: Sized;
}

/// A versioned storage item
pub struct VersionedItem<T> {
    /// The key used to store the value in storage
    key: Vec<u8>,

    /// The key used to store the version in storage
    version_key: Vec<u8>,

    /// The current version
    current_version: u32,

    /// The storage context
    context: Option<Context>,

    /// The type of value stored in this item
    _marker: PhantomData<T>,
}

impl<T> VersionedItem<T>
where T: Codec + Migrate
{
    /// Creates a new versioned storage item with the given key and current version
    pub fn new(key: impl AsRef<[u8]>, current_version: u32) -> Self {
        let mut version_key = key.as_ref().to_vec();
        version_key.extend_from_slice(b".version");

        VersionedItem {
            key: key.as_ref().to_vec(),
            version_key,
            current_version,
            context: None,
            _marker: PhantomData,
        }
    }

    /// Creates a new versioned storage item with the given key, current version, and context
    pub fn with_context(key: impl AsRef<[u8]>, current_version: u32, context: Context) -> Self {
        let mut version_key = key.as_ref().to_vec();
        version_key.extend_from_slice(b".version");

        VersionedItem {
            key: key.as_ref().to_vec(),
            version_key,
            current_version,
            context: Some(context),
            _marker: PhantomData,
        }
    }

    /// Gets the version from storage
    fn get_version(&self) -> Result<Option<u32>> {
        let mut version_item = Item::<u32>::new(&self.version_key);
        if let Some(context) = &self.context {
            version_item.set_context(context.clone());
        }
        version_item.get()
    }

    /// Sets the version in storage
    fn set_version(&self, version: u32) -> Result<()> {
        let mut version_item = Item::<u32>::new(&self.version_key);
        if let Some(context) = &self.context {
            version_item.set_context(context.clone());
        }
        version_item.set(&version)
    }

    /// Gets the value from storage
    pub fn get(&self) -> Result<Option<T>> {
        // Get the stored version
        let stored_version = match self.get_version()? {
            Some(version) => version,
            None => return Ok(None), // No version means no value
        };

        // Get the raw value
        let value = if let Some(context) = &self.context { context.get(&self.key) } else { super::get(&self.key) };

        match value {
            Some(bytes) => {
                if stored_version == self.current_version {
                    // Versions match, decode normally
                    Ok(Some(T::decode(&bytes)?))
                } else {
                    // Versions don't match, migrate
                    Ok(Some(T::migrate(&bytes, stored_version)?))
                }
            }
            None => Ok(None),
        }
    }

    /// Sets the value in storage
    pub fn set(&self, value: &T) -> Result<()> {
        let bytes = value.encode();

        // Store the value
        if let Some(context) = &self.context {
            context.put(&self.key, &bytes);
        } else {
            super::put(&self.key, &bytes);
        }

        // Store the version
        self.set_version(self.current_version)
    }

    /// Removes the value from storage
    pub fn clear(&self) -> Result<()> {
        // Remove the value
        if let Some(context) = &self.context {
            context.delete(&self.key);
            context.delete(&self.version_key);
        } else {
            super::delete(&self.key);
            super::delete(&self.version_key);
        }

        Ok(())
    }

    /// Checks if the item exists in storage
    pub fn exists(&self) -> bool {
        if let Some(context) = &self.context {
            context.has(&self.version_key)
        } else {
            super::has(&self.version_key)
        }
    }

    /// Gets the key of this item
    pub fn key(&self) -> &[u8] { &self.key }

    /// Gets the version key of this item
    pub fn version_key(&self) -> &[u8] { &self.version_key }

    /// Gets the current version of this item
    pub fn current_version(&self) -> u32 { self.current_version }

    /// Gets the context of this item
    pub fn context(&self) -> Option<&Context> { self.context.as_ref() }

    /// Sets the context for this item
    pub fn set_context(&mut self, context: Context) { self.context = Some(context); }

    /// Clears the context for this item
    pub fn clear_context(&mut self) { self.context = None; }
}
