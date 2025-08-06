use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// NEO Executable Format version 3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nef3 {
    /// Magic number (0x3346454E)
    pub magic: u32,
    /// Compiler identifier
    pub compiler: String,
    /// Source code URI or identifier
    pub source: String,
    /// Reserved bytes (must be empty)
    pub reserved: Vec<u8>,
    /// Method tokens for external calls
    pub tokens: Vec<MethodToken>,
    /// Script bytecode
    pub script: Vec<u8>,
    /// Checksum of the script
    pub checksum: u32,
}

impl Nef3 {
    /// NEF3 magic number
    pub const MAGIC: u32 = 0x3346454E; // "NEF3" in little-endian

    /// Create a new NEF3 structure
    pub fn new(compiler: String, source: String, script: Vec<u8>) -> Self {
        let mut nef = Self {
            magic: Self::MAGIC,
            compiler: Self::truncate_string(compiler, 64),
            source: Self::truncate_string(source, 256),
            reserved: vec![],
            tokens: vec![],
            script,
            checksum: 0, // Will be calculated after
        };
        
        // Calculate checksum over the entire NEF (minus checksum field)
        nef.checksum = nef.calculate_full_checksum().unwrap_or(0);
        nef
    }

    /// Add a method token for external calls
    pub fn add_token(&mut self, token: MethodToken) {
        self.tokens.push(token);
    }

    /// Calculate checksum for the entire NEF file (excluding checksum itself)
    fn calculate_full_checksum(&self) -> Result<u32> {
        // Serialize NEF without checksum
        let mut bytes = Vec::new();
        
        // Magic (4 bytes)
        bytes.extend_from_slice(&self.magic.to_le_bytes());
        
        // Compiler (64 bytes, padded with zeros)
        let compiler_bytes = self.compiler.as_bytes();
        bytes.extend_from_slice(compiler_bytes);
        bytes.resize(bytes.len() + (64 - compiler_bytes.len().min(64)), 0);
        
        // Source (256 bytes, with length prefix)
        bytes.push(self.source.len() as u8);
        let source_bytes = self.source.as_bytes();
        bytes.extend_from_slice(source_bytes);
        bytes.resize(bytes.len() + (255 - source_bytes.len().min(255)), 0);
        
        // Reserved (1 byte for length, should be 0)
        bytes.push(0);
        
        // Tokens
        bytes.push(self.tokens.len() as u8);
        for token in &self.tokens {
            bytes.extend_from_slice(&token.to_bytes()?);
        }
        
        // Reserved (2 bytes, should be 0)
        bytes.extend_from_slice(&[0, 0]);
        
        // Script
        let script_len = self.script.len() as u32;
        bytes.extend_from_slice(&script_len.to_le_bytes());
        bytes.extend_from_slice(&self.script);
        
        // Calculate SHA256 checksum
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = hasher.finalize();
        
        // Take first 4 bytes as checksum (little-endian)
        Ok(u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]))
    }

    /// Truncate string to specified length
    fn truncate_string(s: String, max_len: usize) -> String {
        if s.len() > max_len {
            s.chars().take(max_len).collect()
        } else {
            s
        }
    }

    /// Serialize NEF to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        
        // Magic (4 bytes)
        bytes.extend_from_slice(&self.magic.to_le_bytes());
        
        // Compiler (64 bytes, padded with zeros)
        let compiler_bytes = self.compiler.as_bytes();
        bytes.extend_from_slice(compiler_bytes);
        bytes.resize(bytes.len() + (64 - compiler_bytes.len().min(64)), 0);
        
        // Source (256 bytes, with length prefix)
        bytes.push(self.source.len() as u8);
        let source_bytes = self.source.as_bytes();
        bytes.extend_from_slice(source_bytes);
        bytes.resize(bytes.len() + (255 - source_bytes.len().min(255)), 0);
        
        // Reserved (1 byte for length, should be 0)
        bytes.push(0);
        
        // Tokens
        bytes.push(self.tokens.len() as u8);
        for token in &self.tokens {
            bytes.extend_from_slice(&token.to_bytes()?);
        }
        
        // Reserved (2 bytes, should be 0)
        bytes.extend_from_slice(&[0, 0]);
        
        // Script
        let script_len = self.script.len() as u32;
        bytes.extend_from_slice(&script_len.to_le_bytes());
        bytes.extend_from_slice(&self.script);
        
        // Checksum
        bytes.extend_from_slice(&self.checksum.to_le_bytes());
        
        Ok(bytes)
    }

    /// Deserialize NEF from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            anyhow::bail!("Invalid NEF: too short");
        }
        
        let mut cursor = 0;
        
        // Magic
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic != Self::MAGIC {
            anyhow::bail!("Invalid NEF magic: 0x{:08X}", magic);
        }
        cursor += 4;
        
        // Compiler (64 bytes)
        let compiler_end = cursor + 64;
        let compiler_bytes = &bytes[cursor..compiler_end];
        let compiler = String::from_utf8_lossy(compiler_bytes)
            .trim_end_matches('\0')
            .to_string();
        cursor = compiler_end;
        
        // Source (256 bytes with length prefix)
        let source_len = bytes[cursor] as usize;
        cursor += 1;
        let source_bytes = &bytes[cursor..cursor + source_len.min(255)];
        let source = String::from_utf8_lossy(source_bytes).to_string();
        cursor += 255;
        
        // Reserved
        let reserved_len = bytes[cursor] as usize;
        cursor += 1;
        cursor += reserved_len; // Skip reserved bytes
        
        // Tokens
        let token_count = bytes[cursor] as usize;
        cursor += 1;
        let mut tokens = Vec::new();
        for _ in 0..token_count {
            let token = MethodToken::from_bytes(&bytes[cursor..])?;
            cursor += token.size();
            tokens.push(token);
        }
        
        // Reserved (2 bytes)
        cursor += 2;
        
        // Script
        let script_len = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]) as usize;
        cursor += 4;
        let script = bytes[cursor..cursor + script_len].to_vec();
        cursor += script_len;
        
        // Checksum
        let checksum = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]);
        
        Ok(Self {
            magic,
            compiler,
            source,
            reserved: vec![],
            tokens,
            script,
            checksum,
        })
    }

    /// Verify the checksum
    pub fn verify_checksum(&self) -> bool {
        match self.calculate_full_checksum() {
            Ok(calculated) => self.checksum == calculated,
            Err(_) => false,
        }
    }

    /// Get the size of the NEF in bytes
    pub fn size(&self) -> usize {
        4 + // magic
        64 + // compiler
        256 + // source
        1 + // reserved length
        1 + // token count
        self.tokens.iter().map(|t| t.size()).sum::<usize>() +
        2 + // reserved
        4 + // script length
        self.script.len() +
        4 // checksum
    }
}

/// Method token for external contract calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodToken {
    /// Contract hash (20 bytes)
    pub hash: Vec<u8>,
    /// Method name
    pub method: String,
    /// Number of parameters
    pub params_count: u16,
    /// Whether the method has a return value
    pub has_return_value: bool,
    /// Call flags
    pub call_flags: u8,
}

impl MethodToken {
    /// Create a new method token
    pub fn new(hash: Vec<u8>, method: String, params_count: u16) -> Self {
        Self {
            hash,
            method,
            params_count,
            has_return_value: false,
            call_flags: 0x01, // AllowCall
        }
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        
        // Hash (20 bytes)
        if self.hash.len() != 20 {
            anyhow::bail!("Invalid hash length: {}", self.hash.len());
        }
        bytes.extend_from_slice(&self.hash);
        
        // Method (string with length prefix)
        bytes.push(self.method.len() as u8);
        bytes.extend_from_slice(self.method.as_bytes());
        
        // Parameters count
        bytes.extend_from_slice(&self.params_count.to_le_bytes());
        
        // Has return value
        bytes.push(if self.has_return_value { 1 } else { 0 });
        
        // Call flags
        bytes.push(self.call_flags);
        
        Ok(bytes)
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 24 {
            anyhow::bail!("Invalid method token: too short");
        }
        
        let mut cursor = 0;
        
        // Hash
        let hash = bytes[cursor..cursor + 20].to_vec();
        cursor += 20;
        
        // Method
        let method_len = bytes[cursor] as usize;
        cursor += 1;
        let method = String::from_utf8(bytes[cursor..cursor + method_len].to_vec())?;
        cursor += method_len;
        
        // Parameters count
        let params_count = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;
        
        // Has return value
        let has_return_value = bytes[cursor] != 0;
        cursor += 1;
        
        // Call flags
        let call_flags = bytes[cursor];
        
        Ok(Self {
            hash,
            method,
            params_count,
            has_return_value,
            call_flags,
        })
    }

    /// Get the size in bytes
    pub fn size(&self) -> usize {
        20 + // hash
        1 + self.method.len() + // method with length prefix
        2 + // params count
        1 + // has return value
        1 // call flags
    }
}

/// Call flags for method tokens
pub struct CallFlags;

impl CallFlags {
    pub const NONE: u8 = 0x00;
    pub const ALLOW_CALL: u8 = 0x01;
    pub const ALLOW_NOTIFY: u8 = 0x02;
    pub const ALLOW_STATES: u8 = 0x04;
    pub const ALLOW_MODIFY_STATES: u8 = 0x08;
    pub const ALL: u8 = 0x0F;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nef_creation() {
        let script = vec![0x01, 0x02, 0x03];
        let nef = Nef3::new(
            "test-compiler".to_string(),
            "test.rs".to_string(),
            script.clone(),
        );
        
        assert_eq!(nef.magic, Nef3::MAGIC);
        assert_eq!(nef.compiler, "test-compiler");
        assert_eq!(nef.source, "test.rs");
        assert_eq!(nef.script, script);
        assert!(nef.verify_checksum());
    }

    #[test]
    fn test_nef_serialization() {
        let script = vec![0x10, 0x20, 0x30];
        let mut nef = Nef3::new(
            "compiler".to_string(),
            "source".to_string(),
            script,
        );
        
        // Add a method token
        nef.add_token(MethodToken::new(
            vec![0; 20],
            "transfer".to_string(),
            3,
        ));
        
        let bytes = nef.to_bytes().unwrap();
        let deserialized = Nef3::from_bytes(&bytes).unwrap();
        
        assert_eq!(nef.magic, deserialized.magic);
        assert_eq!(nef.compiler, deserialized.compiler);
        assert_eq!(nef.source, deserialized.source);
        assert_eq!(nef.script, deserialized.script);
        assert_eq!(nef.tokens.len(), deserialized.tokens.len());
    }

    #[test]
    fn test_method_token() {
        let token = MethodToken::new(
            vec![0xFF; 20],
            "balanceOf".to_string(),
            1,
        );
        
        let bytes = token.to_bytes().unwrap();
        let deserialized = MethodToken::from_bytes(&bytes).unwrap();
        
        assert_eq!(token.hash, deserialized.hash);
        assert_eq!(token.method, deserialized.method);
        assert_eq!(token.params_count, deserialized.params_count);
    }

    #[test]
    fn test_string_truncation() {
        let long_string = "a".repeat(100);
        let nef = Nef3::new(
            long_string.clone(),
            long_string.clone(),
            vec![],
        );
        
        assert!(nef.compiler.len() <= 64);
        assert!(nef.source.len() <= 256);
    }
}