//! Pagination for Neo Contract RS
//!
//! This module provides pagination functionality for storage.

use super::context::Context;
use super::item::Codec;
use super::iter::StorageIter;
use crate::error::Result;
use crate::find_options::FindOptions;
use alloc::vec::Vec;
use core::marker::PhantomData;

/// A page of storage entries
#[derive(Debug, Clone)]
pub struct Page<K, V> {
    /// The entries in this page
    pub entries: Vec<(K, V)>,

    /// The page number
    pub page: u32,

    /// The page size
    pub page_size: u32,

    /// The total number of entries
    pub total: u32,

    /// Whether there are more pages
    pub has_more: bool,
}

/// A paginator for storage entries
pub struct Paginator<K, V> {
    /// The prefix to search for
    prefix: Vec<u8>,

    /// The storage context
    context: Option<Context>,

    /// The page size
    page_size: u32,

    /// Find options
    options: FindOptions,

    /// The type of keys in this paginator
    _key_marker: PhantomData<K>,

    /// The type of values in this paginator
    _value_marker: PhantomData<V>,
}

impl<K, V> Paginator<K, V>
where
    K: Codec,
    V: Codec,
{
    /// Creates a new paginator with the given prefix and page size
    pub fn new(prefix: impl AsRef<[u8]>, page_size: u32) -> Self {
        Paginator {
            prefix: prefix.as_ref().to_vec(),
            context: None,
            page_size,
            options: FindOptions::NONE,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        }
    }

    /// Creates a new paginator with the given prefix, page size, and context
    pub fn with_context(prefix: impl AsRef<[u8]>, page_size: u32, context: Context) -> Self {
        Paginator {
            prefix: prefix.as_ref().to_vec(),
            context: Some(context),
            page_size,
            options: FindOptions::NONE,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        }
    }

    /// Sets the find options for this paginator
    pub fn with_options(mut self, options: FindOptions) -> Self {
        self.options = options;
        self
    }

    /// Gets a page of entries
    pub fn get_page(&self, page: u32) -> Result<Page<K, V>> {
        let skip = page * self.page_size;
        let take = self.page_size;

        // Get all entries with the prefix
        let items = if let Some(context) = &self.context {
            context.find(&self.prefix, self.options)
        } else {
            super::find(&self.prefix, self.options)
        };

        let total = items.len() as u32;
        let has_more = total > skip + take;

        // Skip and take the entries for this page
        let page_items = items.into_iter().skip(skip as usize).take(take as usize).collect::<Vec<_>>();

        // Create a storage iterator for the page
        let mut iter = StorageIter::<K, V>::new(page_items, &self.prefix);

        // Collect the entries
        let entries = iter.collect()?;

        Ok(Page { entries, page, page_size: self.page_size, total, has_more })
    }

    /// Gets all pages
    pub fn get_all_pages(&self) -> Result<Vec<Page<K, V>>> {
        let total_pages = self.total_pages();
        let mut pages = Vec::with_capacity(total_pages as usize);

        for page in 0..total_pages {
            pages.push(self.get_page(page)?);
        }

        Ok(pages)
    }

    /// Gets the total number of pages
    pub fn total_pages(&self) -> u32 {
        // Get all entries with the prefix
        let items = if let Some(context) = &self.context {
            context.find(&self.prefix, self.options)
        } else {
            super::find(&self.prefix, self.options)
        };

        let total = items.len() as u32;

        (total + self.page_size - 1) / self.page_size
    }

    /// Gets the total number of entries
    pub fn total_entries(&self) -> u32 {
        // Get all entries with the prefix
        let items = if let Some(context) = &self.context {
            context.find(&self.prefix, self.options)
        } else {
            super::find(&self.prefix, self.options)
        };

        items.len() as u32
    }

    /// Gets the prefix of this paginator
    pub fn prefix(&self) -> &[u8] { &self.prefix }

    /// Gets the context of this paginator
    pub fn context(&self) -> Option<&Context> { self.context.as_ref() }

    /// Sets the context for this paginator
    pub fn set_context(&mut self, context: Context) { self.context = Some(context); }

    /// Clears the context for this paginator
    pub fn clear_context(&mut self) { self.context = None; }

    /// Gets the page size of this paginator
    pub fn page_size(&self) -> u32 { self.page_size }

    /// Sets the page size for this paginator
    pub fn set_page_size(&mut self, page_size: u32) { self.page_size = page_size; }

    /// Gets the find options of this paginator
    pub fn options(&self) -> FindOptions { self.options }

    /// Sets the find options for this paginator
    pub fn set_options(&mut self, options: FindOptions) { self.options = options; }
}
