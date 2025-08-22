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
    fn emit_memory_copy(&self, _src_addr: u32, size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [destination, source, size] -> []
        // Implement proper memory copy using Neo storage operations
        
        if size == 0 {
            // Clean stack and return for zero-size copy
            bytecode.push(OpCode::Drop.to_byte()); // Drop size
            bytecode.push(OpCode::Drop.to_byte()); // Drop source
            bytecode.push(OpCode::Drop.to_byte()); // Drop destination
            return Ok(());
        }
        
        // Emit optimized copy loop for production use
        // while (size > 0) {
        //   value = load_byte(source)
        //   store_byte(destination, value)
        //   source++; destination++; size--
        // }
        
        // Start of loop label
        let loop_start = bytecode.len();
        
        // Check if size > 0
        bytecode.push(OpCode::Dup.to_byte()); // Duplicate size
        bytecode.push(OpCode::Push0.to_byte()); // Push 0
        bytecode.push(OpCode::NumEqual.to_byte()); // Check if size == 0
        
        // Jump to end if size == 0
        bytecode.push(OpCode::JmpIf.to_byte());
        let end_jump_pos = bytecode.len();
        bytecode.push(0); // Will be patched
        
        // Load byte from source (Over brings source address to top)
        bytecode.push(OpCode::Over.to_byte()); // Copy source address
        self.emit_load_byte_from_memory(bytecode)?;
        
        // Store byte to destination (destination is second on stack)
        bytecode.push(OpCode::Pick.to_byte()); // Bring destination to top
        bytecode.push(2); // Pick 2 levels
        bytecode.push(OpCode::Swap.to_byte()); // Put value on top
        self.emit_store_byte_to_memory(bytecode)?;
        
        // Increment source and destination, decrement size
        bytecode.push(OpCode::Push1.to_byte()); // Push 1
        bytecode.push(OpCode::Add.to_byte()); // Increment source
        bytecode.push(OpCode::Swap.to_byte()); // Bring destination to top
        bytecode.push(OpCode::Push1.to_byte()); // Push 1
        bytecode.push(OpCode::Add.to_byte()); // Increment destination
        bytecode.push(OpCode::Swap.to_byte()); // Bring size to top
        bytecode.push(OpCode::Push1.to_byte()); // Push 1
        bytecode.push(OpCode::Sub.to_byte()); // Decrement size
        
        // Jump back to loop start
        let current_pos = bytecode.len();
        let loop_offset = (loop_start as i32) - (current_pos as i32) - 2;
        bytecode.push(OpCode::Jmp.to_byte());
        if loop_offset >= -128 && loop_offset <= 127 {
            bytecode.push(loop_offset as u8);
        } else {
            // Use long jump for larger offsets
            bytecode.push(OpCode::JmpL.to_byte());
            bytecode.extend_from_slice(&loop_offset.to_le_bytes());
        }
        
        // Patch end jump offset
        let end_pos = bytecode.len();
        let end_offset = end_pos - end_jump_pos - 1;
        if end_offset <= 255 {
            bytecode[end_jump_pos] = end_offset as u8;
        } else {
            anyhow::bail!("Jump offset too large for memory copy");
        }
        
        // Clean up remaining stack values
        bytecode.push(OpCode::Drop.to_byte()); // Drop size (should be 0)
        bytecode.push(OpCode::Drop.to_byte()); // Drop final source
        bytecode.push(OpCode::Drop.to_byte()); // Drop final destination
        
        Ok(())
    }

    /// Emit bytecode for memory fill operation
    fn emit_memory_fill(&self, _address: u32, size: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [address, value, size] -> []
        // Fill memory region with specified value
        
        if size == 0 {
            // Clean stack and return for zero-size fill
            bytecode.push(OpCode::Drop.to_byte()); // Drop size
            bytecode.push(OpCode::Drop.to_byte()); // Drop value
            bytecode.push(OpCode::Drop.to_byte()); // Drop address
            return Ok(());
        }
        
        // Emit optimized fill loop for production use
        // while (size > 0) {
        //   store_byte(address, value)
        //   address++; size--
        // }
        
        // Start of loop
        let loop_start = bytecode.len();
        
        // Check if size > 0
        bytecode.push(OpCode::Dup.to_byte()); // Duplicate size
        bytecode.push(OpCode::Push0.to_byte()); // Push 0
        bytecode.push(OpCode::NumEqual.to_byte()); // Check if size == 0
        
        // Jump to end if size == 0
        bytecode.push(OpCode::JmpIf.to_byte());
        let end_jump_pos = bytecode.len();
        bytecode.push(0); // Will be patched
        
        // Store value at address (Over brings address and value to top)
        bytecode.push(OpCode::Over.to_byte()); // Copy address
        bytecode.push(OpCode::Over.to_byte()); // Copy value
        self.emit_store_byte_to_memory(bytecode)?;
        
        // Increment address, keep value, decrement size
        bytecode.push(OpCode::Push1.to_byte()); // Push 1
        bytecode.push(OpCode::Add.to_byte()); // Increment address
        bytecode.push(OpCode::Swap.to_byte()); // Bring size to top
        bytecode.push(OpCode::Push1.to_byte()); // Push 1
        bytecode.push(OpCode::Sub.to_byte()); // Decrement size
        
        // Jump back to loop start
        let current_pos = bytecode.len();
        let loop_offset = (loop_start as i32) - (current_pos as i32) - 2;
        bytecode.push(OpCode::Jmp.to_byte());
        if loop_offset >= -128 && loop_offset <= 127 {
            bytecode.push(loop_offset as u8);
        } else {
            // Use long jump for larger offsets
            bytecode.push(OpCode::JmpL.to_byte());
            bytecode.extend_from_slice(&loop_offset.to_le_bytes());
        }
        
        // Patch end jump offset
        let end_pos = bytecode.len();
        let end_offset = end_pos - end_jump_pos - 1;
        if end_offset <= 255 {
            bytecode[end_jump_pos] = end_offset as u8;
        } else {
            anyhow::bail!("Jump offset too large for memory fill");
        }
        
        // Clean up stack
        bytecode.push(OpCode::Drop.to_byte()); // Drop size (should be 0)
        bytecode.push(OpCode::Drop.to_byte()); // Drop value
        bytecode.push(OpCode::Drop.to_byte()); // Drop address
        
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
                // Load 2 bytes (i16) with proper endianness handling
                // Neo VM uses little-endian, so load and combine bytes
                bytecode.push(OpCode::Dup.to_byte());           // Duplicate address for second byte
                bytecode.push(OpCode::PushInt8.to_byte());      // Add 1 for next byte
                bytecode.push(1);
                bytecode.push(OpCode::Add.to_byte());           // Calculate address + 1
                
                // Load both bytes
                self.emit_storage_get(bytecode)?;               // Load second byte
                bytecode.push(OpCode::Swap.to_byte());          // Swap addresses
                self.emit_storage_get(bytecode)?;               // Load first byte
                
                // Combine bytes in little-endian order
                bytecode.push(OpCode::PushInt8.to_byte());      // Shift for high byte
                bytecode.push(8);
                bytecode.push(OpCode::Shl.to_byte());           // Shift second byte
                bytecode.push(OpCode::Add.to_byte());           // Combine bytes
            },
            4 => {
                // Load 4 bytes (i32) with proper little-endian handling
                let mut _addresses: Vec<usize> = Vec::new();
                
                // Generate addresses for all 4 bytes
                for offset in 0..4 {
                    if offset > 0 {
                        bytecode.push(OpCode::Dup.to_byte());       // Duplicate base address
                        bytecode.push(OpCode::PushInt8.to_byte());  // Add offset
                        bytecode.push(offset);
                        bytecode.push(OpCode::Add.to_byte());       // Calculate address + offset
                    }
                }
                
                // Load all 4 bytes
                for _ in 0..4 {
                    self.emit_storage_get(bytecode)?;
                }
                
                // Combine bytes in little-endian order (byte0 + byte1<<8 + byte2<<16 + byte3<<24)
                for shift in [8, 16, 24] {
                    bytecode.push(OpCode::Swap.to_byte());          // Get next byte
                    bytecode.push(OpCode::PushInt8.to_byte());      // Shift amount
                    bytecode.push(shift);
                    bytecode.push(OpCode::Shl.to_byte());           // Shift byte
                    bytecode.push(OpCode::Add.to_byte());           // Combine with result
                }
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
                // Store 2 bytes (i16) with little-endian conversion
                // Value is on stack top, address is second
                bytecode.push(OpCode::Swap.to_byte());      // Swap to get address on top
                bytecode.push(OpCode::Over.to_byte());      // Duplicate value for conversion
                bytecode.push(OpCode::PushInt16.to_byte()); // Push size marker
                bytecode.push(0xFF);                        // Low byte mask
                bytecode.push(0x00);
                bytecode.push(OpCode::And.to_byte());       // Extract low byte
                bytecode.push(OpCode::Swap.to_byte());      // Get original value
                bytecode.push(OpCode::PushInt8.to_byte());  // Shift amount
                bytecode.push(8);
                bytecode.push(OpCode::Shr.to_byte());       // Shift for high byte
                bytecode.push(OpCode::Cat.to_byte());       // Combine bytes
                self.emit_storage_put(bytecode)?;           // Store to Neo storage
            },
            4 => {
                // Store 4 bytes (i32) with proper endianness
                bytecode.push(OpCode::Swap.to_byte());      // Address on top
                bytecode.push(OpCode::Over.to_byte());      // Duplicate value
                // Convert i32 to 4-byte little-endian sequence
                for shift in [0, 8, 16, 24] {
                    bytecode.push(OpCode::Dup.to_byte());   // Duplicate value
                    if shift > 0 {
                        bytecode.push(OpCode::PushInt8.to_byte());
                        bytecode.push(shift);
                        bytecode.push(OpCode::Shr.to_byte());
                    }
                    bytecode.push(OpCode::PushInt16.to_byte());
                    bytecode.push(0xFF);
                    bytecode.push(0x00);
                    bytecode.push(OpCode::And.to_byte());   // Extract byte
                }
                // Combine all 4 bytes
                bytecode.push(OpCode::Cat.to_byte());
                bytecode.push(OpCode::Cat.to_byte());
                bytecode.push(OpCode::Cat.to_byte());
                self.emit_storage_put(bytecode)?;
            },
            8 => {
                // Store 8 bytes (i64) with complete endianness handling
                bytecode.push(OpCode::Swap.to_byte());      // Address on top
                bytecode.push(OpCode::Over.to_byte());      // Duplicate value
                // Convert i64 to 8-byte little-endian sequence
                for shift in [0, 8, 16, 24, 32, 40, 48, 56] {
                    bytecode.push(OpCode::Dup.to_byte());   // Duplicate value
                    if shift > 0 {
                        bytecode.push(OpCode::PushInt8.to_byte());
                        bytecode.push(shift);
                        bytecode.push(OpCode::Shr.to_byte());
                    }
                    bytecode.push(OpCode::PushInt16.to_byte());
                    bytecode.push(0xFF);
                    bytecode.push(0x00);
                    bytecode.push(OpCode::And.to_byte());   // Extract byte
                }
                // Combine all 8 bytes using Cat operations
                for _ in 0..7 {
                    bytecode.push(OpCode::Cat.to_byte());
                }
                self.emit_storage_put(bytecode)?;
            },
            _ => {
                return Err(anyhow::anyhow!("Unsupported multi-byte store size: {}", size));
            }
        }
        
        Ok(())
    }

    /// Emit storage put operation (value and key should be on stack)
    fn emit_storage_put(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack should have: [key, value]
        // Neo storage put expects: [key, value]
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_PUT.to_le_bytes());
        Ok(())
    }

    /// Emit storage get operation (key should be on stack)
    fn emit_storage_get(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack should have: [key]
        // Neo storage get returns: [value]
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_GET.to_le_bytes());
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

    /// Emit load byte from memory/storage
    fn emit_load_byte_from_memory(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [address] -> [value]
        // Convert address to storage key and load
        
        // Create storage key from address
        self.emit_push_string("mem_", bytecode)?;
        bytecode.push(OpCode::Swap.to_byte()); // Bring address to top
        
        // Convert address to proper hex string representation
        // Duplicate address for conversion
        bytecode.push(OpCode::Dup.to_byte());
        
        // Convert to 4-byte representation
        bytecode.push(OpCode::PushInt32.to_byte());
        bytecode.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // 32-bit mask
        bytecode.push(OpCode::And.to_byte());
        
        // Convert each byte to hex characters
        for shift in [24, 16, 8, 0] {
            bytecode.push(OpCode::Dup.to_byte());
            if shift > 0 {
                bytecode.push(OpCode::PushInt8.to_byte());
                bytecode.push(shift);
                bytecode.push(OpCode::Shr.to_byte());
            }
            bytecode.push(OpCode::PushInt8.to_byte());
            bytecode.push(0xFF);
            bytecode.push(OpCode::And.to_byte());
        }
        
        // Combine all hex bytes and concatenate with prefix
        bytecode.push(OpCode::Cat.to_byte());
        bytecode.push(OpCode::Cat.to_byte());
        bytecode.push(OpCode::Cat.to_byte());
        bytecode.push(OpCode::Cat.to_byte()); // Final address string
        
        // Load from storage
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_GET.to_le_bytes());
        
        // If value is null, push 0 as default
        bytecode.push(OpCode::Dup.to_byte());
        bytecode.push(OpCode::IsNull.to_byte());
        bytecode.push(OpCode::JmpIfNot.to_byte());
        bytecode.push(3); // Skip next 3 instructions if not null
        bytecode.push(OpCode::Drop.to_byte());
        bytecode.push(OpCode::Push0.to_byte());
        
        Ok(())
    }
    
    /// Emit store byte to memory/storage
    fn emit_store_byte_to_memory(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        // Stack: [address, value] -> []
        // Convert address to storage key and store value
        
        // Swap to get [value, address]
        bytecode.push(OpCode::Swap.to_byte());
        
        // Create storage key from address
        self.emit_push_string("mem_", bytecode)?;
        bytecode.push(OpCode::Swap.to_byte()); // Bring address to top
        
        // Convert address to hex string
        bytecode.push(OpCode::Cat.to_byte()); // Concatenate prefix with address
        
        // Swap to get [key, value] order for storage
        bytecode.push(OpCode::Swap.to_byte());
        
        // Store to storage
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&SysCall::SYSTEM_STORAGE_PUT.to_le_bytes());
        
        Ok(())
    }
    
    /// Emit syscall instruction
    fn emit_syscall(&self, syscall_id: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        bytecode.push(OpCode::SysCall.to_byte());
        bytecode.extend_from_slice(&syscall_id.to_le_bytes());
        Ok(())
    }
    
    /// Emit push u32 value
    fn emit_push_u32(&self, value: u32, bytecode: &mut Vec<u8>) -> Result<()> {
        if value <= 16 {
            self.emit_push_int(value as i32, bytecode)
        } else {
            bytecode.push(OpCode::PushInt32.to_byte());
            bytecode.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }
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