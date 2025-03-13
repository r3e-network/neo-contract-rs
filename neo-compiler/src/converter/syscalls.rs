//! Neo VM syscall mapping for WebAssembly imports.
//!
//! This module provides mapping between WebAssembly imports and Neo VM syscalls.

use lazy_static::lazy_static;
use std::collections::HashMap;

/// Neo VM interop service names.
pub mod interop {
    /// System namespace.
    pub const SYSTEM: &str = "System";
    /// Runtime namespace.
    pub const RUNTIME: &str = "Neo.Runtime";
    /// Blockchain namespace.
    pub const BLOCKCHAIN: &str = "Neo.Blockchain";
    /// Storage namespace.
    pub const STORAGE: &str = "Neo.Storage";
    /// Crypto namespace.
    pub const CRYPTO: &str = "Neo.Crypto";
}

// A mapping from module and function name to Neo VM syscall hash.
lazy_static! {
    static ref SYSCALL_MAP: HashMap<(&'static str, &'static str), u32> = {
        let mut m = HashMap::new();

        // System namespace
        m.insert(("neo", "runtime_get_trigger"), 0x75F5E941);
        m.insert(("neo", "runtime_check_witness"), 0xAC7E75F2);
        m.insert(("neo", "runtime_notify"), 0x74F18515);
        m.insert(("neo", "runtime_log"), 0x69E9C2C9);
        m.insert(("neo", "storage_get"), 0x425A406C);
        m.insert(("neo", "storage_put"), 0x5F75CC7A);
        m.insert(("neo", "storage_delete"), 0x7FA1895E);
        m.insert(("neo", "storage_find"), 0x4864A25F);
        m.insert(("neo", "contract_create"), 0xA841802C);
        m.insert(("neo", "contract_update"), 0xC6A64A34);
        m.insert(("neo", "contract_destroy"), 0x86A03DC8);
        m.insert(("neo", "iterator_next"), 0x2EE6F9AE);
        m.insert(("neo", "iterator_value"), 0x67BF275E);
        m.insert(("neo", "get_script_container"), 0xD5D0CE24);
        m.insert(("neo", "get_execution_engine"), 0xCACE9A7A);
        m.insert(("neo", "get_calling_script_hash"), 0x92D0DA5F);
        m.insert(("neo", "get_entry_script_hash"), 0x8E14B996);

        // Crypto namespace
        m.insert(("neo", "crypto_check_sig"), 0x6B15F5FD);
        m.insert(("neo", "crypto_sha256"), 0xCE3A4DAB);
        m.insert(("neo", "crypto_ripemd160"), 0xD45F8300);

        // Additional syscalls for compatibility with common WebAssembly imports
        m.insert(("env", "abort"), 0x6B15F5FD); // Map to a Neo VM abort equivalent

        m
    };
}

/// Gets the Neo VM syscall hash for a given import module and function name.
pub fn get_syscall_for_import(module: &str, function: &str) -> Option<u32> {
    SYSCALL_MAP.get(&(module, function)).copied()
}

/// Gets all supported syscall mappings.
pub fn get_all_syscalls() -> Vec<((&'static str, &'static str), u32)> {
    SYSCALL_MAP.iter().map(|((m, f), hash)| ((*m, *f), *hash)).collect()
}

/// Checks if a given import is supported as a syscall.
pub fn is_supported_syscall(module: &str, function: &str) -> bool { SYSCALL_MAP.contains_key(&(module, function)) }

/// Resolves a syscall for a given import module and function name.
/// If the syscall is supported, returns the hash; otherwise returns None.
pub fn resolve_syscall(module: &str, function: &str) -> Option<u32> { get_syscall_for_import(module, function) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_lookup() {
        assert_eq!(get_syscall_for_import("neo", "runtime_notify"), Some(0x74F18515));
        assert_eq!(get_syscall_for_import("neo", "storage_get"), Some(0x425A406C));
        assert_eq!(get_syscall_for_import("unknown", "function"), None);
    }

    #[test]
    fn test_is_supported_syscall() {
        assert!(is_supported_syscall("neo", "runtime_notify"));
        assert!(!is_supported_syscall("unknown", "function"));
    }
}
