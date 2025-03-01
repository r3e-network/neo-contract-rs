// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;

/// Attributes for Neo N3 smart contracts
///
/// This module provides attributes for Neo N3 smart contracts, including:
/// - Contract metadata attributes
/// - Contract permission attributes
/// - Contract trust attributes
/// - Supported standards attributes

/// Contract metadata attribute for specifying the contract name
pub struct ContractNameAttribute(pub String);

/// Contract metadata attribute for specifying the contract version
pub struct ContractVersionAttribute(pub String);

/// Contract metadata attribute for specifying the contract author
pub struct ContractAuthorAttribute(pub String);

/// Contract metadata attribute for specifying the contract email
pub struct ContractEmailAttribute(pub String);

/// Contract metadata attribute for specifying the contract description
pub struct ContractDescriptionAttribute(pub String);

/// Contract permission attribute for specifying contract permissions
pub struct ContractPermissionAttribute(pub String);

/// Contract trust attribute for specifying contract trust
pub struct ContractTrustAttribute(pub String);

/// Supported standards attribute for specifying supported standards
pub struct SupportedStandardsAttribute(pub String);

/// Manifest extra attribute for specifying manifest extras
pub struct ManifestExtraAttribute(pub String);
