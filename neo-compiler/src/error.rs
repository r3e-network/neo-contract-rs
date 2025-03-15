//! Error types for the Neo compiler.
//!
//! This module provides a centralized error handling mechanism for the Neo compiler.

use std::fmt;
use std::io;
use thiserror::Error;

/// Error type for the Neo compiler.
#[derive(Debug, Error)]
pub enum Error {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(String),

    /// Invalid WebAssembly module
    #[error("Invalid WebAssembly module: {0}")]
    InvalidWasm(String),

    /// WebAssembly parsing error
    #[error("WebAssembly parse error: {0}")]
    WasmParse(String),

    /// WebAssembly unsupported feature
    #[error("Unsupported WebAssembly feature: {0}")]
    UnsupportedWasmFeature(String),

    /// Invalid NEF file
    #[error("Invalid NEF file: {0}")]
    InvalidNef(String),

    /// Invalid manifest
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),

    /// Manifest validation error
    #[error("Manifest validation error: {0}")]
    ManifestValidation(String),

    /// Script generation error
    #[error("Script generation error: {0}")]
    ScriptGeneration(String),

    /// Conversion error
    #[error("Conversion error: {0}")]
    Conversion(String),

    /// Neo VM error
    #[error("Neo VM error: {0}")]
    NeoVM(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid event definition
    #[error("Invalid event definition: {0}")]
    InvalidEventDefinition(String),

    /// Invalid method definition
    #[error("Invalid method definition: {0}")]
    InvalidMethodDefinition(String),

    /// General error
    #[error("Error: {0}")]
    General(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::InvalidWasm(msg) => write!(f, "Invalid WebAssembly module: {}", msg),
            Error::WasmParse(msg) => write!(f, "WebAssembly parse error: {}", msg),
            Error::UnsupportedWasmFeature(msg) => write!(f, "Unsupported WebAssembly feature: {}", msg),
            Error::InvalidNef(msg) => write!(f, "Invalid NEF file: {}", msg),
            Error::InvalidManifest(msg) => write!(f, "Invalid manifest: {}", msg),
            Error::ManifestValidation(msg) => write!(f, "Manifest validation error: {}", msg),
            Error::ScriptGeneration(msg) => write!(f, "Script generation error: {}", msg),
            Error::Conversion(msg) => write!(f, "Conversion error: {}", msg),
            Error::NeoVM(msg) => write!(f, "Neo VM error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Error::InvalidEventDefinition(msg) => write!(f, "Invalid event definition: {}", msg),
            Error::InvalidMethodDefinition(msg) => write!(f, "Invalid method definition: {}", msg),
            Error::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
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
    pub fn script_generation<S: Into<String>>(msg: S) -> Self { Error::ScriptGeneration(msg.into()) }

    /// Create a new conversion error
    pub fn conversion<S: Into<String>>(msg: S) -> Self { Error::Conversion(msg.into()) }

    /// Create a new invalid NEF error
    pub fn invalid_nef<S: Into<String>>(msg: S) -> Self { Error::InvalidNef(msg.into()) }

    /// Create a new invalid manifest error
    pub fn invalid_manifest<S: Into<String>>(msg: S) -> Self { Error::InvalidManifest(msg.into()) }

    /// Create a new manifest validation error
    pub fn manifest_validation<S: Into<String>>(msg: S) -> Self { Error::ManifestValidation(msg.into()) }

    /// Create a new invalid event definition error
    pub fn invalid_event_definition<S: Into<String>>(msg: S) -> Self { Error::InvalidEventDefinition(msg.into()) }

    /// Create a new invalid method definition error
    pub fn invalid_method_definition<S: Into<String>>(msg: S) -> Self { Error::InvalidMethodDefinition(msg.into()) }
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
        assert_eq!(format!("{}", io_err), "I/O error: file not found");

        let wasm_err = Error::WasmParse("invalid magic number".to_string());
        assert_eq!(format!("{}", wasm_err), "WebAssembly parse error: invalid magic number");
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
