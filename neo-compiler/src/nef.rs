//! NEF (Neo Executable Format) file representation.
//!
//! This module provides a representation of the Neo Executable Format (NEF),
//! which is the binary format used to store Neo N3 smart contracts.

use crate::error::Error;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::fs::File;

/// The magic number for NEF files.
pub const MAGIC: u32 = 0x3346454E; // "NEF3" in little-endian

/// The current NEF file format version.
pub const VERSION: [u8; 4] = [0, 0, 0, 0];

/// The supported compiler name.
pub const COMPILER_NAME: &str = "neo-contract-rs";

/// Maximum script size allowed in a NEF file (512KB).
pub const MAX_SCRIPT_LENGTH: usize = 512 * 1024;

/// NEF file structure, representing a Neo N3 executable contract.
#[derive(Debug, Clone)]
pub struct NefFile {
    /// Magic number, must be NEF3 (0x3346454E) in little-endian.
    pub magic: u32,
    /// Compiler name, used to identify the compiler that produced the NEF file.
    pub compiler: String,
    /// Version of the NEF format.
    pub version: [u8; 4],
    /// The script bytecode.
    pub script: Vec<u8>,
    /// Checksum of the NEF file.
    pub checksum: [u8; 32],
}

impl Default for NefFile {
    fn default() -> Self {
        Self {
            magic: MAGIC,
            compiler: COMPILER_NAME.to_string(),
            version: VERSION,
            script: Vec::new(),
            checksum: [0; 32],
        }
    }
}

impl NefFile {
    /// Creates a new NEF file with default values.
    pub fn new() -> Self { Self::default() }

    /// Creates a new NEF file with the given script.
    pub fn with_script(script: Vec<u8>) -> Self {
        let mut nef = Self { script, ..Self::default() };
        nef.update_checksum();
        nef
    }

    /// Sets the compiler name.
    pub fn with_compiler(mut self, compiler: impl Into<String>) -> Self {
        self.compiler = compiler.into();
        self.update_checksum();
        self
    }

    /// Validates the NEF file.
    ///
    /// This ensures:
    /// - The magic number is correct
    /// - The script is not too large
    /// - The checksum is valid
    pub fn validate(&self) -> Result<(), Error> {
        if self.magic != MAGIC {
            return Err(Error::invalid_nef(format!("Invalid magic number: {:#x}, expected: {:#x}", self.magic, MAGIC)));
        }

        if self.script.len() > MAX_SCRIPT_LENGTH {
            return Err(Error::invalid_nef(format!(
                "Script size ({} bytes) exceeds maximum allowed size ({} bytes)",
                self.script.len(),
                MAX_SCRIPT_LENGTH
            )));
        }

        if !self.verify_checksum() {
            return Err(Error::invalid_nef("Invalid checksum"));
        }

        Ok(())
    }

    /// Updates the checksum based on the current file contents.
    pub fn update_checksum(&mut self) {
        let mut hasher = Sha256::new();
        let mut header_data = Vec::new();

        // Write all fields except the checksum to a buffer
        write_u32(&mut header_data, self.magic).unwrap();

        // Write compiler name length and compiler name
        let compiler_bytes = self.compiler.as_bytes();
        write_u8(&mut header_data, compiler_bytes.len() as u8).unwrap();
        header_data.extend_from_slice(compiler_bytes);

        // Write version
        header_data.extend_from_slice(&self.version);

        // Write script length and script
        write_u32(&mut header_data, self.script.len() as u32).unwrap();
        header_data.extend_from_slice(&self.script);

        // Calculate hash
        hasher.update(&header_data);
        let result = hasher.finalize();

        // Copy hash to checksum field
        self.checksum.copy_from_slice(&result);
    }

    /// Verifies that the checksum is valid for the current content.
    pub fn verify_checksum(&self) -> bool {
        let mut temp = self.clone();
        let current_checksum = self.checksum;

        // Update the checksum in the temporary NEF file
        temp.update_checksum();

        // Compare the calculated checksum with the current one
        temp.checksum == current_checksum
    }

    /// Writes the NEF file to the specified writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        // Write magic
        write_u32(writer, self.magic)?;

        // Write compiler name
        let compiler_bytes = self.compiler.as_bytes();
        if compiler_bytes.len() > 255 {
            return Err(Error::invalid_nef("Compiler name too long"));
        }
        write_u8(writer, compiler_bytes.len() as u8)?;
        writer.write_all(compiler_bytes)?;

        // Write version
        writer.write_all(&self.version)?;

        // Write script length and script
        write_u32(writer, self.script.len() as u32)?;
        writer.write_all(&self.script)?;

        // Write checksum
        writer.write_all(&self.checksum)?;

        Ok(())
    }

    /// Reads a NEF file from the specified reader.
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, Error> {
        // Read magic
        let magic = read_u32(reader)?;

        // Read compiler name
        let compiler_len = read_u8(reader)?;
        let mut compiler_bytes = vec![0; compiler_len as usize];
        reader.read_exact(&mut compiler_bytes)?;
        let compiler = String::from_utf8(compiler_bytes)
            .map_err(|e| Error::invalid_nef(format!("Invalid compiler name: {}", e)))?;

        // Read version
        let mut version = [0; 4];
        reader.read_exact(&mut version)?;

        // Read script
        let script_len = read_u32(reader)?;
        if script_len as usize > MAX_SCRIPT_LENGTH {
            return Err(Error::invalid_nef(format!(
                "Script size ({} bytes) exceeds maximum allowed size ({} bytes)",
                script_len, MAX_SCRIPT_LENGTH
            )));
        }

        let mut script = vec![0; script_len as usize];
        reader.read_exact(&mut script)?;

        // Read checksum
        let mut checksum = [0; 32];
        reader.read_exact(&mut checksum)?;

        let nef = Self { magic, compiler, version, script, checksum };

        Ok(nef)
    }

    /// Serializes the NEF file to bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer)?;
        Ok(buffer)
    }

    /// Deserializes a NEF file from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut cursor = io::Cursor::new(bytes);
        Self::read_from(&mut cursor)
    }

    /// Saves the NEF file to the specified path.
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let bytes = self.to_bytes()?;
        fs::write(path, bytes)?;
        Ok(())
    }

    /// Loads a NEF file from the specified path.
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Finalizes the NEF file by validating it and updating the checksum.
    ///
    /// This should be called before saving the NEF file.
    pub fn finalize(&mut self) -> Result<(), Error> {
        // Update the checksum
        self.update_checksum();

        // Validate the NEF file
        self.validate()?;

        Ok(())
    }
}

// Utility functions for reading/writing binary data

fn write_u32<W: Write>(writer: &mut W, value: u32) -> io::Result<()> { writer.write_all(&value.to_le_bytes()) }

fn write_u8<W: Write>(writer: &mut W, value: u8) -> io::Result<()> { writer.write_all(&[value]) }

fn read_u32<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_u8<R: Read>(reader: &mut R) -> io::Result<u8> {
    let mut buffer = [0];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nef_file_roundtrip() {
        let script = vec![1, 2, 3, 4, 5];
        let mut nef = NefFile::with_script(script.clone());
        nef.finalize().unwrap();

        let bytes = nef.to_bytes().unwrap();
        let nef2 = NefFile::from_bytes(&bytes).unwrap();

        assert_eq!(nef.magic, nef2.magic);
        assert_eq!(nef.compiler, nef2.compiler);
        assert_eq!(nef.version, nef2.version);
        assert_eq!(nef.script, nef2.script);
        assert_eq!(nef.checksum, nef2.checksum);
    }

    #[test]
    fn test_checksum_calculation() {
        let script = vec![1, 2, 3, 4, 5];
        let mut nef = NefFile::with_script(script.clone());

        // Save the initial checksum
        let checksum1 = nef.checksum;

        // Modify the script and recalculate the checksum
        nef.script = vec![5, 4, 3, 2, 1];
        nef.update_checksum();
        let checksum2 = nef.checksum;

        // Checksums should be different
        assert_ne!(checksum1, checksum2);

        // Verify the checksum is now valid
        assert!(nef.verify_checksum());
    }

    #[test]
    fn test_validation() {
        // Test a valid NEF file
        let script = vec![1, 2, 3, 4, 5];
        let mut nef = NefFile::with_script(script.clone());
        nef.finalize().unwrap();
        assert!(nef.validate().is_ok());

        // Test an invalid magic number
        let mut nef_invalid_magic = nef.clone();
        nef_invalid_magic.magic = 0x12345678;
        assert!(nef_invalid_magic.validate().is_err());

        // Test an invalid checksum
        let mut nef_invalid_checksum = nef.clone();
        nef_invalid_checksum.checksum[0] ^= 0xFF;
        assert!(nef_invalid_checksum.validate().is_err());
    }
}
