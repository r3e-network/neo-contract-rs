// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Pagination module for handling large result sets in smart contracts

use alloc::string::String;
use alloc::vec::Vec;
use crate::builtin::{ByteString, Any, Array};
use crate::storage::{StorageMap, Storable, StorageContext};
use crate::error::{Error, ErrorCode, Result};

/// Page info for pagination
pub struct PageInfo {
    /// Total number of items
    pub total: usize,
    /// Current page number
    pub page: usize,
    /// Number of items per page
    pub page_size: usize,
    /// Whether there is a next page
    pub has_next: bool,
    /// Whether there is a previous page
    pub has_prev: bool,
}

impl PageInfo {
    /// Calculate total number of pages
    pub fn total_pages(&self) -> usize {
        if self.total == 0 {
            0
        } else {
            (self.total + self.page_size - 1) / self.page_size
        }
    }
}

impl Storable for PageInfo {
    fn to_storage(&self) -> ByteString {
        let data = format!("{}|{}|{}", self.total, self.page, self.page_size);
        ByteString::from(data)
    }
    
    fn from_storage(data: &ByteString) -> Option<Self> {
        let parts: Vec<&str> = data.as_string().split('|').collect();
        if parts.len() != 3 {
            return None;
        }
        
        let total = parts[0].parse::<usize>().ok()?;
        let page = parts[1].parse::<usize>().ok()?;
        let page_size = parts[2].parse::<usize>().ok()?;
        
        let has_next = page < (total + page_size - 1) / page_size;
        let has_prev = page > 1;
        
        Some(Self {
            total,
            page,
            page_size,
            has_next,
            has_prev,
        })
    }
}

/// Paginated result set
pub struct Page<T> {
    /// Items in the current page
    pub items: Vec<T>,
    /// Page information
    pub page_info: PageInfo,
}

/// Trait for pageable collections
pub trait Pageable<K, V> {
    /// Get a page of items
    fn get_page(&self, page: usize, page_size: usize) -> Page<(K, V)>;
    
    /// Count the total number of items
    fn count(&self) -> usize;
}

impl<K: Storable + Clone, V: Storable + Clone> Pageable<K, V> for StorageMap<K, V> {
    fn get_page(&self, page: usize, page_size: usize) -> Page<(K, V)> {
        let mut items = Vec::new();
        let total = self.count();
        
        // Calculate start and end indices
        let start = (page - 1) * page_size;
        let end = core::cmp::min(start + page_size, total);
        
        // Get iterator
        let iter = self.iter();
        
        // Skip to start index
        let mut index = 0;
        for item in iter {
            if let Some((k, v)) = item {
                if index >= start && index < end {
                    items.push((k, v));
                }
                if index >= end {
                    break;
                }
                index += 1;
            }
        }
        
        // Create page info
        let page_info = PageInfo {
            total,
            page,
            page_size,
            has_next: page * page_size < total,
            has_prev: page > 1,
        };
        
        Page {
            items,
            page_info,
        }
    }
    
    fn count(&self) -> usize {
        let mut count = 0;
        for _ in self.iter() {
            count += 1;
        }
        count
    }
}

/// Helper for managing paginated data
pub struct PaginationHelper<K, V> {
    /// Storage map to paginate
    storage_map: StorageMap<K, V>,
    /// Cache for total count
    count_cache: Option<usize>,
}

impl<K: Storable + Clone, V: Storable + Clone> PaginationHelper<K, V> {
    /// Create a new pagination helper
    pub fn new(storage_map: StorageMap<K, V>) -> Self {
        Self {
            storage_map,
            count_cache: None,
        }
    }
    
    /// Get a page of items
    pub fn get_page(&mut self, page: usize, page_size: usize) -> Page<(K, V)> {
        // Validate page and page_size
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };
        
        // Get the total count, using cache if available
        let total = if let Some(count) = self.count_cache {
            count
        } else {
            let count = self.storage_map.count();
            self.count_cache = Some(count);
            count
        };
        
        // Calculate start and end indices
        let start = (page - 1) * page_size;
        let end = core::cmp::min(start + page_size, total);
        
        // Get iterator
        let iter = self.storage_map.iter();
        
        // Skip to start index and collect items
        let mut items = Vec::new();
        let mut index = 0;
        for item in iter {
            if let Some((k, v)) = item {
                if index >= start && index < end {
                    items.push((k, v));
                }
                if index >= end {
                    break;
                }
                index += 1;
            }
        }
        
        // Create page info
        let page_info = PageInfo {
            total,
            page,
            page_size,
            has_next: page * page_size < total,
            has_prev: page > 1,
        };
        
        Page {
            items,
            page_info,
        }
    }
    
    /// Get total count of items
    pub fn count(&mut self) -> usize {
        if let Some(count) = self.count_cache {
            count
        } else {
            let count = self.storage_map.count();
            self.count_cache = Some(count);
            count
        }
    }
    
    /// Invalidate count cache
    pub fn invalidate_cache(&mut self) {
        self.count_cache = None;
    }
}

/// Convert a page to an array of Any objects for notification or return value
pub fn page_to_array<K: Storable + Clone, V: Storable + Clone>(page: &Page<(K, V)>) -> Array<Any> {
    let mut result = Array::<Any>::new();
    
    // Add page info
    let mut page_info_array = Array::<Any>::new();
    page_info_array.push(Any::from(page.page_info.total));
    page_info_array.push(Any::from(page.page_info.page));
    page_info_array.push(Any::from(page.page_info.page_size));
    page_info_array.push(Any::from(page.page_info.has_next));
    page_info_array.push(Any::from(page.page_info.has_prev));
    
    result.push(Any::from(page_info_array));
    
    // Add items
    let mut items_array = Array::<Any>::new();
    for (k, v) in &page.items {
        let mut item_array = Array::<Any>::new();
        item_array.push(Any::from(k.to_storage()));
        item_array.push(Any::from(v.to_storage()));
        items_array.push(Any::from(item_array));
    }
    
    result.push(Any::from(items_array));
    
    result
}
