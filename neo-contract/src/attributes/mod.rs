// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Attributes for Neo N3 smart contracts
//!
//! This module provides the attribute implementations used in Neo N3 smart contracts.
//! These attributes are used to generate the contract manifest and provide metadata
//! for the Neo N3 blockchain.

mod safe;

pub use safe::SafeAttribute;

use alloc::string::String;

/// Contract metadata attribute for specifying the contract name
#[derive(Debug, Clone)]
pub struct ContractNameAttribute(pub String);

/// Contract metadata attribute for specifying the contract version
#[derive(Debug, Clone)]
pub struct ContractVersionAttribute(pub String);

/// Contract metadata attribute for specifying the contract author
#[derive(Debug, Clone)]
pub struct ContractAuthorAttribute(pub String);

/// Contract metadata attribute for specifying the contract email
#[derive(Debug, Clone)]
pub struct ContractEmailAttribute(pub String);

/// Contract metadata attribute for specifying the contract description
#[derive(Debug, Clone)]
pub struct ContractDescriptionAttribute(pub String);

/// Contract permission attribute for specifying contract permissions
#[derive(Debug, Clone)]
pub struct ContractPermissionAttribute(pub String);

/// Contract trust attribute for specifying contract trust
#[derive(Debug, Clone)]
pub struct ContractTrustAttribute(pub String);

/// Supported standards attribute for specifying supported standards
#[derive(Debug, Clone)]
pub struct SupportedStandardsAttribute(pub String);

/// Manifest extra attribute for specifying manifest extras
#[derive(Debug, Clone)]
pub struct ManifestExtraAttribute(pub String);
