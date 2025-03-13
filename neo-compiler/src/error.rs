//! Error types for the Neo compiler.
//!
//! This module provides a centralized error handling mechanism for the Neo compiler.

use std::fmt;
use std::io;

/// Error type for the Neo compiler.
#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(io::Error),

    /// Invalid WebAssembly module
    InvalidWasm(String),

    /// WebAssembly parsing error
    WasmParse(String),

    /// WebAssembly unsupported feature
    UnsupportedWasmFeature(String),

    /// Invalid NEF file
    InvalidNef(String),

    /// Invalid manifest
    InvalidManifest(String),

    /// Script generation error
    ScriptGeneration(String),

    /// Conversion error
    Conversion(String),

    /// Neo VM error
    NeoVM(String),

    /// Serialization error
    Serialization(String),

    /// General error
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
            Error::ScriptGeneration(msg) => write!(f, "Script generation error: {}", msg),
            Error::Conversion(msg) => write!(f, "Conversion error: {}", msg),
            Error::NeoVM(msg) => write!(f, "Neo VM error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
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
    fn from(err: io::Error) -> Self { Error::Io(err) }
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
}

/// Result type alias for neo-compiler operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error_display() {
        let io_err = Error::Io(IoError::new(ErrorKind::NotFound, "file not found"));
        assert_eq!(format!("{}", io_err), "I/O error: file not found");

        let wasm_err = Error::WasmParse("invalid magic number".to_string());
        assert_eq!(format!("{}", wasm_err), "WebAssembly parse error: invalid magic number");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "permission denied");
        let err: Error = io_err.into();
        match err {
            Error::Io(e) => assert_eq!(e.kind(), ErrorKind::PermissionDenied),
            _ => panic!("Expected IoError variant"),
        }
    }
}
