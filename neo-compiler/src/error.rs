//! Error types for the Neo compiler.
//!
//! This module provides a centralized error handling mechanism for the Neo compiler.

use std::io;
use thiserror::Error;

/// Error type for the Neo compiler
#[derive(Debug, Error)]
pub enum Error {
    /// General error
    #[error("General error: {0}")]
    General(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(String),
    
    /// WebAssembly parsing error
    #[error("WebAssembly parsing error: {0}")]
    WasmParse(String),
    
    /// Invalid WASM error
    #[error("Invalid WASM: {0}")]
    InvalidWasm(String),
    
    /// Conversion error
    #[error("Conversion error: {0}")]
    Conversion(String),
    
    /// Manifest generation error
    #[error("Manifest generation error: {0}")]
    Manifest(String),
    
    /// NEF generation error
    #[error("NEF generation error: {0}")]
    Nef(String),
    
    /// Script generation error
    #[error("Script generation error: {0}")]
    Script(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Invalid Event Definition
    #[error("Invalid event definition: {0}")]
    InvalidEventDefinition(String),
    
    /// Invalid Method Definition
    #[error("Invalid method definition: {0}")]
    InvalidMethodDefinition(String),
    
    /// Unsupported WebAssembly Feature
    #[error("Unsupported WebAssembly feature: {0}")]
    UnsupportedWasmFeature(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self { Error::Io(err.to_string()) }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self { Error::Serialization(err.to_string()) }
}

impl From<wasmparser::BinaryReaderError> for Error {
    fn from(err: wasmparser::BinaryReaderError) -> Self { Error::WasmParse(err.to_string()) }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self { Error::General(err.to_string()) }
}

impl From<String> for Error {
    fn from(err: String) -> Self { Error::General(err) }
}

// Helper functions for creating specific errors
impl Error {
    /// Create a new general error
    pub fn general<S: Into<String>>(msg: S) -> Self { Error::General(msg.into()) }

    /// Create a new invalid WebAssembly error
    pub fn invalid_wasm<S: Into<String>>(msg: S) -> Self { Error::InvalidWasm(msg.into()) }

    /// Create a new script generation error
    pub fn script_generation<S: Into<String>>(msg: S) -> Self { Error::Script(msg.into()) }

    /// Create a new conversion error
    pub fn conversion<S: Into<String>>(msg: S) -> Self { Error::Conversion(msg.into()) }

    /// Create a new invalid NEF error
    pub fn invalid_nef<S: Into<String>>(msg: S) -> Self { Error::Nef(msg.into()) }

    /// Create a new invalid manifest error
    pub fn invalid_manifest<S: Into<String>>(msg: S) -> Self { Error::Manifest(msg.into()) }

    /// Create a new manifest validation error
    pub fn manifest_validation<S: Into<String>>(msg: S) -> Self { Error::Manifest(msg.into()) }

    /// Create a new invalid event definition error
    pub fn invalid_event_definition<S: Into<String>>(msg: S) -> Self { Error::InvalidEventDefinition(msg.into()) }

    /// Create a new invalid method definition error
    pub fn invalid_method_definition<S: Into<String>>(msg: S) -> Self { Error::InvalidMethodDefinition(msg.into()) }
    
    /// Create a new unsupported WASM feature error
    pub fn unsupported_wasm_feature<S: Into<String>>(msg: S) -> Self { Error::UnsupportedWasmFeature(msg.into()) }
    
    /// Create a new serialization error
    pub fn serialization<S: Into<String>>(msg: S) -> Self { Error::Serialization(msg.into()) }
}

/// Result type alias for neo-compiler operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error_display() {
        let io_err = Error::Io(IoError::new(ErrorKind::NotFound, "file not found").to_string());
        assert_eq!(format!("{}", io_err), "IO error: file not found");

        let wasm_err = Error::WasmParse("invalid magic number".to_string());
        assert_eq!(format!("{}", wasm_err), "WebAssembly parsing error: invalid magic number");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "permission denied");
        let err: Error = io_err.into();
        match err {
            Error::Io(e) => assert_eq!(e, "permission denied"),
            _ => panic!("Expected IoError variant"),
        }
    }
}
