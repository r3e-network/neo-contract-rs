//! Utility functions
//!
//! This module provides various utility functions used throughout the compiler.

use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Ensures that a directory exists, creating it if necessary
///
/// # Arguments
///
/// * `path` - The directory path
///
/// # Returns
///
/// Result indicating success or failure
pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).with_context(|| format!("Failed to create directory at {}", path.display()))?;
    } else if !path.is_dir() {
        anyhow::bail!("Path exists but is not a directory: {}", path.display());
    }

    Ok(())
}

/// Writes a string to a file
///
/// # Arguments
///
/// * `path` - The file path
/// * `content` - The content to write
///
/// # Returns
///
/// Result indicating success or failure
pub fn write_file_string(path: &Path, content: &str) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_directory_exists(parent)?;
    }

    let mut file = File::create(path).with_context(|| format!("Failed to create file at {}", path.display()))?;

    file.write_all(content.as_bytes()).with_context(|| format!("Failed to write to file at {}", path.display()))?;

    Ok(())
}

/// Reads a file to a string
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Returns
///
/// The file content as a string
pub fn read_file_string(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file at {}", path.display()))
}

/// Writes binary data to a file
///
/// # Arguments
///
/// * `path` - The file path
/// * `data` - The binary data to write
///
/// # Returns
///
/// Result indicating success or failure
pub fn write_file_binary(path: &Path, data: &[u8]) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_directory_exists(parent)?;
    }

    let mut file = File::create(path).with_context(|| format!("Failed to create file at {}", path.display()))?;

    file.write_all(data).with_context(|| format!("Failed to write to file at {}", path.display()))?;

    Ok(())
}

/// Reads a file to a byte vector
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Returns
///
/// The file content as a byte vector
pub fn read_file_binary(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("Failed to read file at {}", path.display()))
}

/// Formats a size in bytes to a human-readable string
///
/// # Arguments
///
/// * `size` - The size in bytes
///
/// # Returns
///
/// A human-readable size string (e.g., "1.23 KB")
pub fn format_size(size: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;

    if size < KB as usize {
        format!("{} bytes", size)
    } else if size < MB as usize {
        format!("{:.2} KB", size as f64 / KB)
    } else {
        format!("{:.2} MB", size as f64 / MB)
    }
}

/// Sanitizes a name to be used in a filename
///
/// # Arguments
///
/// * `name` - The name to sanitize
///
/// # Returns
///
/// A sanitized name safe for use in filenames
pub fn sanitize_filename(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_")
}

/// Converts a string to CamelCase
///
/// # Arguments
///
/// * `input` - The input string
///
/// # Returns
///
/// The CamelCase version of the input string
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else {
            capitalize_next = true;
        }
    }

    result
}

/// Converts a string to snake_case
///
/// # Arguments
///
/// * `input` - The input string
///
/// # Returns
///
/// The snake_case version of the input string
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_is_uppercase = false;

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_uppercase {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_is_uppercase = true;
        } else {
            result.push(c);
            prev_is_uppercase = false;
        }
    }

    result
}
