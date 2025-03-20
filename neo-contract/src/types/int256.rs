impl From<u64> for Int256 {
    fn from(value: u64) -> Self {
        // Simple implementation using existing methods
        // This may need adjustment based on the actual Int256 implementation
        let mut bytes = [0u8; 32];
        let value_bytes = value.to_be_bytes();
        // Put u64 value at the end of the 32 byte array
        bytes[24..32].copy_from_slice(&value_bytes);
        Int256::from_bytes(&bytes)
    }
}

// Add a method to convert Int256 to u64 for convenience
impl Int256 {
    pub fn to_u64(&self) -> Option<u64> {
        // Simple implementation - if the value fits in u64, return it
        // Otherwise return None
        if self.is_negative() || self.bits() > 64 {
            None
        } else {
            let bytes = self.to_bytes();
            if bytes.len() < 8 {
                // Value can fit in u64, pad with zeros
                let mut u64_bytes = [0u8; 8];
                let start = 8 - bytes.len();
                u64_bytes[start..].copy_from_slice(&bytes);
                Some(u64::from_be_bytes(u64_bytes))
            } else {
                // Take the last 8 bytes
                let mut u64_bytes = [0u8; 8];
                u64_bytes.copy_from_slice(&bytes[bytes.len() - 8..]);
                Some(u64::from_be_bytes(u64_bytes))
            }
        }
    }
} 