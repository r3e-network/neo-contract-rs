use anyhow::{Context, Result};
use std::collections::HashMap;
use crate::opcodes::{OpCode, SysCall};

/// WASM linear memory to Neo storage model translator
pub struct MemoryModelTranslator {
    /// Memory page size (64KB standard WASM page size)
    page_size: u32,
    /// Current memory pages allocated
    current_pages: u32,
    /// Memory region mappings to storage keys
    memory_regions: HashMap<MemoryRegion, String>,
    /// Base storage key prefix for memory operations
    storage_prefix: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MemoryRegion {
    pub start: u32,
    pub size: u32,
    pub access_type: MemoryAccessType,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MemoryAccessType {
    ReadWrite,
    ReadOnly,
    WriteOnly,
    Stack,
    Heap,
    Static,
}

pub struct MemoryOperation {
    pub operation_type: MemoryOperationType,
    pub address: u32,
    pub size: u32,
    pub alignment: u32,
}

#[derive(Debug, Clone)]
pub enum MemoryOperationType {
    Load,
    Store,
    Grow,
    Size,
    Copy,
    Fill,
}

impl MemoryModelTranslator {
    pub fn new() -> Self {
        Self {
            page_size: 65536, // 64KB
            current_pages: 1,  // Start with 1 page minimum
            memory_regions: HashMap::new(),
            storage_prefix: "mem".to_string(),
        }
    }

    /// Initialize memory model for a WASM module
    pub fn initialize(&mut self, initial_pages: u32) -> Result<Vec<u8>> {
        self.current_pages = initial_pages;
        
        // Create initial memory regions
        self.create_default_regions();
        
        // Generate NEO bytecode to initialize memory storage
        let mut bytecode = Vec::new();
        
        // Initialize memory size in storage
        self.emit_store_memory_size(initial_pages, &mut bytecode)?;
        
        // Initialize memory regions
        for region in self.memory_regions.keys() {
            self.emit_initialize_region(region, &mut bytecode)?;
        }
        
        Ok(bytecode)
    }

    /// Create default memory regions
    fn create_default_regions(&mut self) {
        // Stack region (first 16KB)
        self.memory_regions.insert(
            MemoryRegion {
                start: 0,
                size: 16384,
                access_type: MemoryAccessType::Stack,
            },
            format!("{}_stack", self.storage_prefix),
        );

        // Heap region (16KB - 48KB)
        self.memory_regions.insert(
            MemoryRegion {
                start: 16384,
                size: 32768,
                access_type: MemoryAccessType::Heap,
            },
            format!("{}_heap", self.storage_prefix),
        );

        // Static region (48KB - 64KB)
        self.memory_regions.insert(
            MemoryRegion {
                start: 49152,
                size: 16384,
                access_type: MemoryAccessType::Static,
            },
            format!("{}_static", self.storage_prefix),
        );
    }

    /// Translate WASM memory operation to Neo storage operations
    pub fn translate_memory_operation(&mut self, operation: &MemoryOperation) -> Result<Vec<u8>> {
        let mut bytecode = Vec::new();

        match operation.operation_type {
            MemoryOperationType::Load => {
                self.emit_memory_load(operation.address, operation.size, &mut bytecode)?;
            },
            MemoryOperationType::Store => {
                self.emit_memory_store(operation.address, operation.size, &mut bytecode)?;
            },
            MemoryOperationType::Grow => {
                self.emit_memory_grow(&mut bytecode)?;
            },
            MemoryOperationType::Size => {
                self.emit_memory_size(&mut bytecode)?;
            },
            MemoryOperationType::Copy => {
                self.emit_memory_copy(operation.address, operation.size, &mut bytecode)?;
            },
            MemoryOperationType::Fill => {
                self.emit_memory_fill(operation.address, operation.size, &mut bytecode)?;
            },
        }

        Ok(bytecode)
    }

    /// Emit bytecode for memory load operation
    fn emit_memory_load(&self, address: u32, size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Find the memory region containing this address
        let region = self.find_memory_region(address)?;
        let storage_key = self.memory_regions.get(&region)
            .context("Memory region not found")?;

        // Calculate offset within the region
        let offset = address - region.start;

        // Emit NEO opcodes:
        // 1. Push storage key for the region + offset
        self.emit_push_storage_key(storage_key, offset, bytecode)?;
        
        // 2. Load from storage
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_GET.to_le_bytes());

        // 3. Handle multi-byte loads
        if size > 1 {
            self.emit_multi_byte_load(size, bytecode)?;
        }

        Ok(())
    }

    /// Emit bytecode for memory store operation
    fn emit_memory_store(&self, address: u32, size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Find the memory region containing this address
        let region = self.find_memory_region(address)?;
        let storage_key = self.memory_regions.get(&region)
            .context("Memory region not found")?;

        // Calculate offset within the region
        let offset = address - region.start;

        // Handle multi-byte stores
        if size > 1 {
            self.emit_multi_byte_store(size, bytecode)?;
        }

        // Emit NEO opcodes:
        // 1. Push storage key for the region + offset
        self.emit_push_storage_key(storage_key, offset, bytecode)?;
        
        // 2. Swap key and value for correct order
        bytecode.push(OpCode::Swap.to_byte());
        
        // 3. Store to storage
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_PUT.to_le_bytes());

        Ok(())
    }

    /// Emit bytecode for memory.grow operation
    fn emit_memory_grow(&mut self, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [pages] -> [old_pages]
        
        // Load current memory size
        self.emit_push_string("mem_size", bytecode)?;
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_GET.to_le_bytes());

        // Convert to integer (current pages)
        bytecode.push(OpCode::Dup.to_byte()); // Duplicate for return value

        // Add requested pages to current pages
        // Stack: [requested_pages] [old_pages] [old_pages]
        bytecode.push(OpCode::Rot.to_byte()); // Rotate to get: [old_pages] [old_pages] [requested_pages]
        bytecode.push(OpCode::Add.to_byte()); // Add: [old_pages] [new_pages]

        // Store new memory size
        self.emit_push_string("mem_size", bytecode)?;
        bytecode.push(OpCode::Swap.to_byte());
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_PUT.to_le_bytes());

        // Return old pages count
        // Stack now has [old_pages] which is what we want to return

        Ok(())
    }

    /// Emit bytecode for memory.size operation
    fn emit_memory_size(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        // Load current memory size from storage
        self.emit_push_string("mem_size", bytecode)?;
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_GET.to_le_bytes());

        Ok(())
    }

    /// Emit bytecode for memory copy operation
    fn emit_memory_copy(&self, _src_addr: u32, _size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [destination, source, size] -> []
        // This is a complex operation that would need to be implemented
        // with a loop for copying chunks of data
        
        // For now, emit a placeholder that calls a helper function
        bytecode.push(OpCode::Call.to_byte());
        bytecode.push(0xFF); // Placeholder for memory copy helper function

        Ok(())
    }

    /// Emit bytecode for memory fill operation
    fn emit_memory_fill(&self, _address: u32, _size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [address, value, size] -> []
        // This would fill memory with a specific value
        
        // For now, emit a placeholder
        bytecode.push(OpCode::Call.to_byte());
        bytecode.push(0xFE); // Placeholder for memory fill helper function

        Ok(())
    }

    /// Find the memory region containing the given address
    fn find_memory_region(&self, address: u32) -> Result<MemoryRegion> {
        for region in self.memory_regions.keys() {
            if address >= region.start && address < region.start + region.size {
                return Ok(region.clone());
            }
        }
        
        anyhow::bail!("Address 0x{:x} is outside allocated memory regions", address);
    }

    /// Emit storage key for memory address
    fn emit_push_storage_key(&self, base_key: &str, offset: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Create composite storage key: base_key + hex(offset)
        let storage_key = format!("{}_{:08x}", base_key, offset);
        self.emit_push_string(&storage_key, bytecode)?;
        Ok(())
    }

    /// Emit push string instruction
    fn emit_push_string(&self, s: &str, bytecode: &mut Vec<u8>) -> Result<()> {
        let bytes = s.as_bytes();
        let len = bytes.len();
        
        if len <= 75 {
            // PUSHDATA0
            bytecode.push(len as u8);
        } else if len <= 255 {
            // PUSHDATA1
            bytecode.push(OpCode::PushData1.to_byte());
            bytecode.push(len as u8);
        } else if len <= 65535 {
            // PUSHDATA2
            bytecode.push(OpCode::PushData2.to_byte());
            bytecode.extend_from_slice(&(len as u16).to_le_bytes());
        } else {
            // PUSHDATA4
            bytecode.push(OpCode::PushData4.to_byte());
            bytecode.extend_from_slice(&(len as u32).to_le_bytes());
        }
        
        bytecode.extend_from_slice(bytes);
        Ok(())
    }

    /// Emit multi-byte load handling
    fn emit_multi_byte_load(&self, size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        match size {
            2 => {
                // Load 2 bytes (i16)
                // In a full implementation, this would handle endianness
                // For now, just duplicate the load
                bytecode.push(OpCode::Dup.to_byte());
            },
            4 => {
                // Load 4 bytes (i32)
                // In a full implementation, this would handle endianness
                // For now, just duplicate the load
                bytecode.push(OpCode::Dup.to_byte());
                bytecode.push(OpCode::Dup.to_byte());
                bytecode.push(OpCode::Dup.to_byte());
            },
            8 => {
                // Load 8 bytes (i64)
                for _ in 0..7 {
                    bytecode.push(OpCode::Dup.to_byte());
                }
            },
            _ => {
                // For other sizes, emit a loop
                return Err(anyhow::anyhow!("Unsupported multi-byte load size: {}", size));
            }
        }
        
        Ok(())
    }

    /// Emit multi-byte store handling
    fn emit_multi_byte_store(&self, size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        match size {
            2 => {
                // Store 2 bytes (i16) - handle endianness conversion
                // For now, simplified
                bytecode.push(OpCode::Nop.to_byte()); // Placeholder for endianness handling
            },
            4 => {
                // Store 4 bytes (i32)
                bytecode.push(OpCode::Nop.to_byte()); // Placeholder
            },
            8 => {
                // Store 8 bytes (i64)
                bytecode.push(OpCode::Nop.to_byte()); // Placeholder
            },
            _ => {
                return Err(anyhow::anyhow!("Unsupported multi-byte store size: {}", size));
            }
        }
        
        Ok(())
    }

    /// Store memory size in storage
    fn emit_store_memory_size(&self, pages: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Push the number of pages
        self.emit_push_int(pages as i32, bytecode)?;
        
        // Push storage key for memory size
        self.emit_push_string("mem_size", bytecode)?;
        
        // Swap to get correct order for storage
        bytecode.push(OpCode::Swap.to_byte());
        
        // Store to storage
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_PUT.to_le_bytes());
        
        Ok(())
    }

    /// Initialize a memory region in storage
    fn emit_initialize_region(&self, region: &MemoryRegion, bytecode: &mut Vec<u8>) -> Result<()> {
        let storage_key = self.memory_regions.get(region)
            .context("Memory region not found")?;
        
        // Initialize region metadata
        // Store region size and type information
        
        // Push region info as serialized data  
        let region_info = format!("{}:{}:{}", region.size, region.start, region.access_type as u8);
        self.emit_push_string(&region_info, bytecode)?;
        
        // Push metadata storage key
        let metadata_key = format!("{}_meta", storage_key);
        self.emit_push_string(&metadata_key, bytecode)?;
        
        // Swap and store
        bytecode.push(OpCode::Swap.to_byte());
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_PUT.to_le_bytes());
        
        Ok(())
    }

    /// Emit push integer instruction
    fn emit_push_int(&self, value: i32, bytecode: &mut Vec<u8>) -> Result<()> {
        match value {
            -1 => bytecode.push(OpCode::PushM1.to_byte()),
            0 => bytecode.push(OpCode::Push0.to_byte()),
            1..=16 => {
                let opcode = unsafe { 
                    std::mem::transmute((OpCode::Push1 as u8) + (value as u8 - 1))
                };
                bytecode.push(opcode);
            },
            _ => {
                if value >= i8::MIN as i32 && value <= i8::MAX as i32 {
                    bytecode.push(OpCode::PushInt8.to_byte());
                    bytecode.push(value as u8);
                } else if value >= i16::MIN as i32 && value <= i16::MAX as i32 {
                    bytecode.push(OpCode::PushInt16.to_byte());
                    bytecode.extend_from_slice(&(value as i16).to_le_bytes());
                } else {
                    bytecode.push(OpCode::PushInt32.to_byte());
                    bytecode.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        Ok(())
    }

    /// Get memory usage statistics
    pub fn get_memory_stats(&self) -> MemoryStats {
        let total_allocated = self.current_pages * self.page_size;
        let regions_count = self.memory_regions.len();
        
        let mut stack_size = 0;
        let mut heap_size = 0;
        let mut static_size = 0;
        
        for region in self.memory_regions.keys() {
            match region.access_type {
                MemoryAccessType::Stack => stack_size += region.size,
                MemoryAccessType::Heap => heap_size += region.size,
                MemoryAccessType::Static => static_size += region.size,
                _ => {}
            }
        }
        
        MemoryStats {
            total_pages: self.current_pages,
            total_bytes: total_allocated,
            regions_count,
            stack_size,
            heap_size,
            static_size,
        }
    }
}

#[derive(Debug)]
pub struct MemoryStats {
    pub total_pages: u32,
    pub total_bytes: u32,
    pub regions_count: usize,
    pub stack_size: u32,
    pub heap_size: u32,
    pub static_size: u32,
}

impl Default for MemoryModelTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_model_initialization() {
        let mut translator = MemoryModelTranslator::new();
        let bytecode = translator.initialize(1).unwrap();
        
        assert!(!bytecode.is_empty());
        assert_eq!(translator.current_pages, 1);
        assert_eq!(translator.memory_regions.len(), 3); // stack, heap, static
    }

    #[test]
    fn test_memory_load_operation() {
        let mut translator = MemoryModelTranslator::new();
        let operation = MemoryOperation {
            operation_type: MemoryOperationType::Load,
            address: 1000,
            size: 4,
            alignment: 4,
        };
        
        let bytecode = translator.translate_memory_operation(&operation).unwrap();
        assert!(!bytecode.is_empty());
        
        // Should contain syscall for storage get
        let has_syscall = bytecode.windows(1).any(|w| w[0] == OpCode::SysCall.to_byte());
        assert!(has_syscall);
    }

    #[test]
    fn test_memory_store_operation() {
        let mut translator = MemoryModelTranslator::new();
        let operation = MemoryOperation {
            operation_type: MemoryOperationType::Store,
            address: 2000,
            size: 4,
            alignment: 4,
        };
        
        let bytecode = translator.translate_memory_operation(&operation).unwrap();
        assert!(!bytecode.is_empty());
        
        // Should contain syscall for storage put
        let has_syscall = bytecode.windows(1).any(|w| w[0] == OpCode::SysCall.to_byte());
        assert!(has_syscall);
    }

    #[test]
    fn test_find_memory_region() {
        let mut translator = MemoryModelTranslator::new();
        translator.create_default_regions();
        
        // Test finding stack region
        let region = translator.find_memory_region(1000).unwrap();
        assert_eq!(region.access_type, MemoryAccessType::Stack);
        
        // Test finding heap region
        let region = translator.find_memory_region(20000).unwrap();
        assert_eq!(region.access_type, MemoryAccessType::Heap);
        
        // Test finding static region
        let region = translator.find_memory_region(50000).unwrap();
        assert_eq!(region.access_type, MemoryAccessType::Static);
    }

    #[test]
    fn test_memory_stats() {
        let translator = MemoryModelTranslator::new();
        let stats = translator.get_memory_stats();
        
        assert_eq!(stats.total_pages, 1);
        assert_eq!(stats.total_bytes, 65536);
        assert_eq!(stats.regions_count, 0); // No regions created yet
    }

    #[test]
    fn test_out_of_bounds_access() {
        let translator = MemoryModelTranslator::new();
        
        // Test accessing memory beyond allocated pages
        let result = translator.find_memory_region(100000);
        assert!(result.is_err());
    }
}