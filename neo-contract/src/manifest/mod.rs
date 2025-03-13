// Neo N3 Contract Manifest Module
//
// This module provides functions for registering contract manifest information
// such as methods, events, and other metadata required by the Neo N3 blockchain.

use crate::Runtime;
use alloc::format;
use core::sync::atomic::{AtomicBool, Ordering};

// Flag to track if the contract has been registered
static CONTRACT_REGISTERED: AtomicBool = AtomicBool::new(false);

/// Register a contract in the Neo N3 manifest
///
/// This function should be called by the #[contract] macro to register
/// the contract's metadata in the manifest.
pub fn register_contract(
    name: &str,
    version: &str,
    author: &str,
    description: &str,
    email: &str,
    dynamic_invoke: bool,
) {
    // We only need to do this once
    if CONTRACT_REGISTERED.swap(true, Ordering::SeqCst) {
        return;
    }

    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registering Neo N3 contract: {}", name).into());
    Runtime::log(&format!("Author: {}, Version: {}", author, version).into());
    Runtime::log(&format!("Description: {}", description).into());
    Runtime::log(&format!("Email: {}", email).into());
    Runtime::log(&format!("Dynamic invoke: {}", dynamic_invoke).into());

    // In a real implementation, this would store this information in the manifest
    // For now, we just log it for debugging
}

/// Register a method in the Neo N3 manifest
///
/// This function should be called by the #[method] and #[safe] macros to register
/// the method's metadata in the manifest.
pub fn register_method(name: &str, safe: bool, param_names: &[&str], param_types: &[&str], return_type: &str) {
    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registering Neo N3 method: {}", name).into());
    Runtime::log(&format!("Safe: {}", safe).into());
    Runtime::log(&format!("Parameter names: {:?}", param_names).into());
    Runtime::log(&format!("Parameter types: {:?}", param_types).into());
    Runtime::log(&format!("Return type: {}", return_type).into());

    // In a real implementation, this would store this information in the manifest
    // For now, we just log it for debugging
}

/// Simplified method to register a safe method (backward compatibility)
pub fn register_safe_method(name: &str) { register_method(name, true, &[], &[], ""); }

/// Register an event in the Neo N3 manifest
///
/// This function should be called by the #[event] macro to register
/// the event's metadata in the manifest.
///
/// # Arguments
/// * `name` - The name of the event
/// * `param_names` - The names of the parameters
/// * `param_types` - The types of the parameters (Neo VM types)
/// * `indexed_params` - Whether each parameter is indexed (for event filtering)
///
/// # Neo N3 Event Registration
/// In Neo N3, events must be registered in the contract manifest, and the
/// parameters can be marked as indexed for better filtering capabilities.
pub fn register_event(
    name: &str,
    param_names: &[&str],
    param_types: &[&str], // New parameter to specify Neo VM types
    indexed_params: &[bool],
) {
    // Validate input parameters
    if param_names.len() != indexed_params.len() || (param_types.len() > 0 && param_names.len() != param_types.len()) {
        Runtime::log(
            &format!(
                "Error: Parameter array lengths don't match: names={}, types={}, indexed={}",
                param_names.len(),
                param_types.len(),
                indexed_params.len()
            )
            .into(),
        );
        return;
    }

    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registering Neo N3 event: {}", name).into());
    Runtime::log(&format!("Parameter names: {:?}", param_names).into());
    if param_types.len() > 0 {
        Runtime::log(&format!("Parameter types: {:?}", param_types).into());
    }
    Runtime::log(&format!("Indexed parameters: {:?}", indexed_params).into());

    // In a real implementation, this would store this information in the manifest
    // For now, we just log it for debugging
}

/// Simplified event registration for backward compatibility
pub fn register_simple_event(name: &str, param_names: &[&str], indexed_params: &[bool]) {
    register_event(name, param_names, &[], indexed_params);
}

/// Register a supported standard in the Neo N3 manifest
///
/// This function should be called by the #[supported_standards] macro to register
/// the standards that the contract implements.
pub fn register_supported_standard(standard: &str) {
    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registering Neo N3 supported standard: {}", standard).into());

    // In a real implementation, this would store this information in the manifest
    // For now, we just log it for debugging
}
