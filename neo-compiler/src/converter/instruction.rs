//! WebAssembly instruction to Neo VM instruction converter.
//!
//! This module provides functionality for converting individual
//! WebAssembly instructions to Neo VM instructions.

use crate::converter::ScriptExt;
use crate::error::Error;
use crate::neo::OpCode;
use crate::script::Script;
use std::collections::HashMap;
use wasmparser::Operator;

/// Converts a WebAssembly instruction to Neo VM instructions.
pub fn convert_instruction(
    script: &mut Script,
    op: Operator,
    function_offsets: &HashMap<u32, u64>,
) -> Result<(), Error> {
    match op {
        // Control flow
        Operator::Unreachable => {
            script.emit_opcode(OpCode::ABORT);
        }
        Operator::Nop => {
            script.emit_opcode(OpCode::NOP);
        }
        Operator::Block { blockty: _ } => {
            // Block instruction doesn't generate any code directly
            // It just represents the start of a block that can be jumped to
            // The script position is recorded for branch targets

            // We don't emit any instructions here; the block is used implicitly
            // by branch instructions for determining jump targets
        }
        Operator::Loop { blockty: _ } => {
            // Loop instruction marks the start of a loop that can be jumped back to
            // Unlike Block, branches to a Loop go to the beginning, not the end

            // In Neo VM, we need to record this position as a loop target for br instructions
            // to jump back to the beginning of the loop

            // We don't emit any instructions here, but the position is used
            // by branch instructions to determine where to jump back to
        }
        Operator::If { blockty: _ } => {
            // The condition is already on the stack due to previous instructions
            // If the condition is false, we jump to the matching Else or End

            // In Neo VM, we use JMPIFNOT to jump if the condition is false
            script.emit_opcode(OpCode::JMPIFNOT);

            // We'll track the current position so we can update the jump target later
            // when we process the matching Else or End
            // For now, add a placeholder offset that will be updated later during backpatching
            let _jump_position = script.len();
            script.emit_u32(0xFFFFFFFF); // Placeholder jump target to be fixed later

            // Store the jump position for backpatching when we reach Else or End
            // This would typically be stored in a context object tracking control flow
        }
        Operator::Else => {
            // At Else, we need to:
            // 1. Add JMP to skip the else branch (for the if-true case)
            // 2. Patch the preceding If's JMPIFNOT to jump to this position

            // First, emit unconditional jump to skip the else branch (if condition was true)
            script.emit_opcode(OpCode::JMP);
            let _else_jump_position = script.len();
            script.emit_u32(0xFFFFFFFF); // Placeholder jump target to be updated at End

            // Record current position which is the target for the if branch's JMPIFNOT
            let _else_position = script.len();

            // Backpatch the If instruction's jump target
            // In a real implementation, we would look up the If's jump position from a context
            // and update it to jump to this position (else_position)
            // script.update_u32_at(if_jump_position, else_position);
        }
        Operator::End => {
            // End marks the end of a block, if, else, or loop
            // We need to backpatch any jumps that target this instruction

            // Record current position as the end position
            let _end_position = script.len();

            // In a full implementation, we would:
            // 1. Patch any Br/BrIf jumps that target this End
            // 2. Patch the If's JMPIFNOT to here if there was no Else
            // 3. Patch the Else's JMP to here
            // 4. Pop the current block from the stack of blocks

            // For a real implementation, we'd need a control flow context to track all jumps
        }
        Operator::Br { relative_depth: _ } => {
            // Branch unconditionally to a parent block or loop
            // In WebAssembly, relative_depth indicates how many nested blocks to exit

            // In Neo VM, we use JMP to perform an unconditional jump
            script.emit_opcode(OpCode::JMP);

            // In a real implementation, we would calculate the correct jump target
            // based on the relative_depth and a control flow context that tracks
            // block positions
            let _br_position = script.len();
            script.emit_u32(0xFFFFFFFF); // Placeholder to be patched later

            // In a production implementation, we'd record this position along with
            // the target block for backpatching when we reach the target block's End
        }
        Operator::BrIf { relative_depth: _ } => {
            // Conditional branch if the condition on the stack is true
            // In WebAssembly, relative_depth indicates how many nested blocks to exit

            // In Neo VM, we use JMPIF to jump if the condition is true
            script.emit_opcode(OpCode::JMPIF);

            // Similar to Br, we would calculate the correct jump target based on relative_depth
            // In a control flow context tracking block positions
            let _brif_position = script.len();
            script.emit_u32(0xFFFFFFFF); // Placeholder to be patched later

            // In a production implementation, we'd record this position along with
            // the target block for backpatching when we reach the target block's End
        }
        Operator::BrTable { targets } => {
            // Table-based jumping with case-like semantics
            // The index is on the stack, and we need to jump to the corresponding target
            // This is similar to a switch statement in many languages

            // Get default target from targets
            let _default_target = targets.default();
            let target_count = targets.len();

            // In Neo VM, we don't have a direct BrTable equivalent, so we implement it with
            // a series of conditional jumps

            // First, check if the index is out of bounds
            script.emit_opcode(OpCode::DUP); // Duplicate the index
            script.emit_push_integer(target_count as i64);
            script.emit_opcode(OpCode::LT);

            // If index < target_count, do table lookup, else jump to default
            script.emit_opcode(OpCode::JMPIFNOT);
            let _out_of_bounds_jump = script.len();
            script.emit_u32(0); // Will be patched to jump to default case code

            // For each target, we would generate the following pattern:
            // - Check if index matches this target's index
            // - If it does, jump to the target

            // Let's process the cases
            for i in 0..target_count {
                // Check if this is our target index
                if i > 0 {
                    script.emit_opcode(OpCode::DUP); // Duplicate the index
                    script.emit_push_integer(i as i64);
                    script.emit_opcode(OpCode::NUMEQUAL);
                    script.emit_opcode(OpCode::JMPIF);
                    let _case_jump = script.len();
                    script.emit_u32(0xFFFFFFFF); // Placeholder to jump to target i
                }
            }

            // Add unconditional jump to default target
            script.emit_opcode(OpCode::JMP);
            let _default_jump = script.len();
            script.emit_u32(0xFFFFFFFF); // Placeholder for default target

            // Patch the out-of-bounds jump to here
            let _default_position = script.len();
            // script.update_u32_at(out_of_bounds_jump, default_position);

            // Drop the index as it's no longer needed after the branch
            script.emit_opcode(OpCode::DROP);
        }
        Operator::Return => {
            script.emit_opcode(OpCode::RET);
        }
        Operator::Call { function_index } => {
            // Use function_index directly as it's already u32 type
            let func_idx = function_index;
            if let Some(offset) = function_offsets.get(&func_idx) {
                script.emit_opcode(OpCode::CallL);
                script.emit_with_operand(OpCode::CallL, offset.to_le_bytes().to_vec());
            } else {
                return Err(Error::invalid_wasm(format!("Function index {} not found in function offsets", func_idx)));
            }
        }
        Operator::CallIndirect { .. } => {
            // Not directly supported in Neo VM, would require complex handling
            return Err(Error::UnsupportedWasmFeature("Call indirect not supported".to_string()));
        }

        // Parametric instructions
        Operator::Drop => {
            script.emit_opcode(OpCode::DROP);
        }
        Operator::Select => {
            // If condition is true, select first value, otherwise select second value
            // This requires specific handling in Neo VM
            // Simplified implementation:
            script.emit_opcode(OpCode::JMPIF);
            script.emit_with_operand(OpCode::JMPIF, vec![3]); // Jump over DROP
            script.emit_opcode(OpCode::SWAP);
            script.emit_opcode(OpCode::DROP);
        }

        // Variable access
        Operator::LocalGet { local_index } => {
            // Map local variable index to the appropriate LDLOC instruction
            // No conversion needed - local_index is already u32
            let local_idx = local_index;
            match local_idx {
                0 => script.emit_opcode(OpCode::LDLOC0),
                1 => script.emit_opcode(OpCode::LDLOC1),
                2 => script.emit_opcode(OpCode::LDLOC2),
                3 => script.emit_opcode(OpCode::LDLOC3),
                4 => script.emit_opcode(OpCode::LDLOC4),
                5 => script.emit_opcode(OpCode::LDLOC5),
                6 => script.emit_opcode(OpCode::LDLOC6),
                _ => {
                    // For larger indices, need to ensure it fits within Neo VM constraints
                    if local_idx > 255 {
                        panic!("Local index {} is too large for Neo VM byte operand", local_idx);
                    }
                    script.emit_with_operand(OpCode::LDLOC, vec![local_idx as u8])
                }
            }
        }
        Operator::LocalSet { local_index } => {
            // Map local variable index to the appropriate STLOC instruction
            // No conversion needed - local_index is already u32
            let local_idx = local_index;
            match local_idx {
                0u32 => script.emit_opcode(OpCode::STLOC0),
                1u32 => script.emit_opcode(OpCode::STLOC1),
                2u32 => script.emit_opcode(OpCode::STLOC2),
                3u32 => script.emit_opcode(OpCode::STLOC3),
                4u32 => script.emit_opcode(OpCode::STLOC4),
                5u32 => script.emit_opcode(OpCode::STLOC5),
                6u32 => script.emit_opcode(OpCode::STLOC6),
                _ => {
                    // Ensure local_idx fits within a byte range for Neo VM
                    if local_idx > 255 {
                        panic!("Local index {} is too large for Neo VM byte operand", local_idx);
                    }
                    script.emit_with_operand(OpCode::STLOC, vec![local_idx as u8])
                }
            }
        }
        Operator::LocalTee { local_index } => {
            // First duplicate the value
            script.emit_opcode(OpCode::DUP);

            // Then store it to the local variable
            // No conversion needed - local_index is already u32
            let local_idx = local_index;
            match local_idx {
                0u32 => script.emit_opcode(OpCode::STLOC0),
                1u32 => script.emit_opcode(OpCode::STLOC1),
                2u32 => script.emit_opcode(OpCode::STLOC2),
                3u32 => script.emit_opcode(OpCode::STLOC3),
                4u32 => script.emit_opcode(OpCode::STLOC4),
                5u32 => script.emit_opcode(OpCode::STLOC5),
                6u32 => script.emit_opcode(OpCode::STLOC6),
                _ => {
                    // Ensure local_idx fits within a byte range for Neo VM
                    if local_idx > 255 {
                        panic!("Local index {} is too large for Neo VM byte operand", local_idx);
                    }
                    script.emit_with_operand(OpCode::STLOC, vec![local_idx as u8])
                }
            }
        }
        Operator::GlobalGet { global_index } => {
            // Map global variable index to the appropriate LDSFLD instruction
            // No conversion needed - global_index is already u32
            let global_idx = global_index;
            match global_idx {
                0u32 => script.emit_opcode(OpCode::LDSFLD0),
                1u32 => script.emit_opcode(OpCode::LDSFLD1),
                2u32 => script.emit_opcode(OpCode::LDSFLD2),
                3u32 => script.emit_opcode(OpCode::LDSFLD3),
                4u32 => script.emit_opcode(OpCode::LDSFLD4),
                5u32 => script.emit_opcode(OpCode::LDSFLD5),
                6u32 => script.emit_opcode(OpCode::LDSFLD6),
                _ => {
                    // Ensure global_idx fits within a byte range for Neo VM
                    if global_idx > 255 {
                        panic!("Global index {} is too large for Neo VM byte operand", global_idx);
                    }
                    script.emit_with_operand(OpCode::LDSFLD, vec![global_idx as u8])
                }
            }
        }
        Operator::GlobalSet { global_index } => {
            // Map global variable index to the appropriate STSFLD instruction
            // No conversion needed - global_index is already u32
            let global_idx = global_index;
            match global_idx {
                0u32 => script.emit_opcode(OpCode::STSFLD0),
                1u32 => script.emit_opcode(OpCode::STSFLD1),
                2u32 => script.emit_opcode(OpCode::STSFLD2),
                3u32 => script.emit_opcode(OpCode::STSFLD3),
                4u32 => script.emit_opcode(OpCode::STSFLD4),
                5u32 => script.emit_opcode(OpCode::STSFLD5),
                6u32 => script.emit_opcode(OpCode::STSFLD6),
                _ => {
                    // Ensure global_idx fits within a byte range for Neo VM
                    if global_idx > 255 {
                        panic!("Global index {} is too large for Neo VM byte operand", global_idx);
                    }
                    script.emit_with_operand(OpCode::STSFLD, vec![global_idx as u8])
                }
            }
        }

        // Memory operations in Neo N3
        // In WebAssembly, memory is a linear array of bytes
        // In Neo N3, we'll use storage for memory operations

        // Load operations
        Operator::I32Load { memarg } => {
            // Follow Neo N3 storage patterns for memory operations
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 4, false)?;
        }
        Operator::I64Load { memarg } => {
            // Load 8-byte value from Neo storage
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 8, false)?;
        }
        Operator::F32Load { memarg } => {
            // Load 4-byte floating point value
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 4, false)?;
        }
        Operator::F64Load { memarg } => {
            // Load 8-byte floating point value
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 8, false)?;
        }
        Operator::I32Load8S { memarg } => {
            // Load signed 8-bit value and sign-extend to 32 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 1, true)?;
            // Sign extend from 8 to 32 bits
            script.emit_push_integer(24);
            script.emit_opcode(OpCode::SHL);
            script.emit_push_integer(24);
            script.emit_opcode(OpCode::SHR);
        }
        Operator::I32Load8U { memarg } => {
            // Load unsigned 8-bit value and zero-extend to 32 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 1, false)?;
            // Zero-extend is automatic
        }
        Operator::I32Load16S { memarg } => {
            // Load signed 16-bit value and sign-extend to 32 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 2, true)?;
            // Sign extend from 16 to 32 bits
            script.emit_push_integer(16);
            script.emit_opcode(OpCode::SHL);
            script.emit_push_integer(16);
            script.emit_opcode(OpCode::SHR);
        }
        Operator::I32Load16U { memarg } => {
            // Load unsigned 16-bit value and zero-extend to 32 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 2, false)?;
            // Zero-extend is automatic
        }
        Operator::I64Load8S { memarg } => {
            // Load signed 8-bit value and sign-extend to 64 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 1, true)?;
            // Sign extend from 8 to 64 bits
            script.emit_push_integer(56);
            script.emit_opcode(OpCode::SHL);
            script.emit_push_integer(56);
            script.emit_opcode(OpCode::SHR);
        }
        Operator::I64Load8U { memarg } => {
            // Load unsigned 8-bit value and zero-extend to 64 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 1, false)?;
            // Zero-extend is automatic
        }
        Operator::I64Load16S { memarg } => {
            // Load signed 16-bit value and sign-extend to 64 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 2, true)?;
            // Sign extend from 16 to 64 bits
            script.emit_push_integer(48);
            script.emit_opcode(OpCode::SHL);
            script.emit_push_integer(48);
            script.emit_opcode(OpCode::SHR);
        }
        Operator::I64Load16U { memarg } => {
            // Load unsigned 16-bit value and zero-extend to 64 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 2, false)?;
            // Zero-extend is automatic
        }
        Operator::I64Load32S { memarg } => {
            // Load signed 32-bit value and sign-extend to 64 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 4, true)?;
            // Sign extend from 32 to 64 bits
            script.emit_push_integer(32);
            script.emit_opcode(OpCode::SHL);
            script.emit_push_integer(32);
            script.emit_opcode(OpCode::SHR);
        }
        Operator::I64Load32U { memarg } => {
            // Load unsigned 32-bit value and zero-extend to 64 bits
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_load(script, offset, 4, false)?;
            // Zero-extend is automatic
        }

        // Store operations - following Neo N3 storage patterns
        Operator::I32Store { memarg } => {
            // Store 4-byte integer to Neo storage
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 4)?;
        }
        Operator::I64Store { memarg } => {
            // Store 8-byte integer to Neo storage
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 8)?;
        }
        Operator::F32Store { memarg } => {
            // Store 4-byte floating point to Neo storage
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 4)?;
        }
        Operator::F64Store { memarg } => {
            // Store 8-byte floating point to Neo storage
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 8)?;
        }
        Operator::I32Store8 { memarg } => {
            // Truncate to 8 bits before storing
            script.emit_push_integer(0xFF);
            script.emit_opcode(OpCode::AND);
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 1)?;
        }
        Operator::I32Store16 { memarg } => {
            // Truncate to 16 bits before storing
            script.emit_push_integer(0xFFFF);
            script.emit_opcode(OpCode::AND);
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 2)?;
        }
        Operator::I64Store8 { memarg } => {
            // Truncate to 8 bits before storing
            script.emit_push_integer(0xFF);
            script.emit_opcode(OpCode::AND);
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 1)?;
        }
        Operator::I64Store16 { memarg } => {
            // Truncate to 16 bits before storing
            script.emit_push_integer(0xFFFF);
            script.emit_opcode(OpCode::AND);
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 2)?;
        }
        Operator::I64Store32 { memarg } => {
            // Truncate to 32 bits before storing
            script.emit_push_integer(0xFFFFFFFF);
            script.emit_opcode(OpCode::AND);
            // Safely convert u64 offset to u32 for memory operations
            let offset = u32::try_from(memarg.offset)
                .unwrap_or_else(|_| panic!("Memory offset {} is too large for Neo VM", memarg.offset));
            handle_memory_store(script, offset, 4)?;
        }

        // Memory size and grow operations
        // Memory size and grow operations for Neo N3
        // These map WebAssembly memory pages to Neo VM storage
        Operator::MemorySize { .. } => {
            // Get the current memory size in pages (64KB)
            // In Neo N3, we'll retrieve the memory size from storage
            // Call the Storage.Get syscall to retrieve "memory_size" key
            script.emit_push_data(b"memory_size")?;
            script.emit_push_data(b"Storage.Get")?;
            script.emit_opcode(OpCode::SYSCALL);

            // If memory size doesn't exist yet, return 0
            script.emit_opcode(OpCode::DUP);
            script.emit_opcode(OpCode::ISNULL);
            script.emit_opcode(OpCode::JMPIF);
            script.emit_with_operand(OpCode::JMPIF, vec![3]); // Jump 3 bytes ahead
            script.emit_opcode(OpCode::DROP); // Drop the null
            script.emit_push_integer(0); // Default memory size
        }
        Operator::MemoryGrow { .. } => {
            // Grow memory by the specified number of pages
            // 1. Get current memory size
            script.emit_push_data(b"memory_size")?;
            script.emit_push_data(b"Storage.Get")?;
            script.emit_opcode(OpCode::SYSCALL);

            // If memory size doesn't exist yet, use 0
            script.emit_opcode(OpCode::DUP);
            script.emit_opcode(OpCode::ISNULL);
            script.emit_opcode(OpCode::JMPIF);
            script.emit_with_operand(OpCode::JMPIF, vec![3]); // Jump 3 bytes ahead
            script.emit_opcode(OpCode::DROP); // Drop the null
            script.emit_push_integer(0); // Default memory size

            // 2. Save the old size to return later
            script.emit_opcode(OpCode::DUP);

            // 3. Add the requested pages (on the stack) to the current size
            script.emit_opcode(OpCode::SWAP);
            script.emit_opcode(OpCode::ADD);

            // 4. Store the new memory size using Storage.Put syscall
            script.emit_opcode(OpCode::DUP); // Duplicate the new size
            script.emit_push_data(b"memory_size")?;
            script.emit_opcode(OpCode::SWAP);
            script.emit_push_data(b"Storage.Put")?;
            script.emit_opcode(OpCode::SYSCALL);
        }

        // Numeric constants
        Operator::I32Const { value } => {
            script.emit_push_integer(value as i64);
        }
        Operator::I64Const { value } => {
            script.emit_push_integer(value);
        }

        // Integer arithmetic
        Operator::I32Add | Operator::I64Add => {
            script.emit_opcode(OpCode::ADD);
        }
        Operator::I32Sub | Operator::I64Sub => {
            script.emit_opcode(OpCode::SUB);
        }
        Operator::I32Mul | Operator::I64Mul => {
            script.emit_opcode(OpCode::MUL);
        }
        Operator::I32DivS | Operator::I64DivS => {
            script.emit_opcode(OpCode::DIV);
        }
        Operator::I32RemS | Operator::I64RemS => {
            script.emit_opcode(OpCode::MOD);
        }
        Operator::I32And | Operator::I64And => {
            script.emit_opcode(OpCode::AND);
        }
        Operator::I32Or | Operator::I64Or => {
            script.emit_opcode(OpCode::OR);
        }
        Operator::I32Xor | Operator::I64Xor => {
            script.emit_opcode(OpCode::XOR);
        }
        Operator::I32Shl | Operator::I64Shl => {
            script.emit_opcode(OpCode::SHL);
        }
        Operator::I32ShrS | Operator::I64ShrS => {
            script.emit_opcode(OpCode::SHR);
        }

        // Comparisons
        Operator::I32Eq | Operator::I64Eq => {
            script.emit_opcode(OpCode::NUMEQUAL);
        }
        Operator::I32Ne | Operator::I64Ne => {
            script.emit_opcode(OpCode::NUMNOTEQUAL);
        }
        Operator::I32LtS | Operator::I64LtS => {
            script.emit_opcode(OpCode::LT);
        }
        Operator::I32LeS | Operator::I64LeS => {
            script.emit_opcode(OpCode::LE);
        }
        Operator::I32GtS | Operator::I64GtS => {
            script.emit_opcode(OpCode::GT);
        }
        Operator::I32GeS | Operator::I64GeS => {
            script.emit_opcode(OpCode::GE);
        }

        // Conversions
        Operator::I32WrapI64 => {
            // No explicit operation needed, Neo VM will handle it
        }
        Operator::I64ExtendI32S => {
            // No explicit operation needed, Neo VM will handle it
        }

        // I32 operations that are unsupported in current version
        Operator::I32Clz | Operator::I64Clz => {
            // Count leading zeros - implement using Neo VM bit operations
            script.emit_push_integer(0);
            script.emit_push_integer(64); // or 32 for I32

            // Loop through bits checking for zeros
            // This would normally require a custom Neo VM script
            // that implements the counting using basic operations
            return Err(Error::UnsupportedWasmFeature("Count leading zeros not directly supported".to_string()));
        }

        Operator::I32Ctz | Operator::I64Ctz => {
            // Count trailing zeros
            return Err(Error::UnsupportedWasmFeature("Count trailing zeros not directly supported".to_string()));
        }

        Operator::I32Popcnt | Operator::I64Popcnt => {
            // Count number of 1 bits
            return Err(Error::UnsupportedWasmFeature("Population count not directly supported".to_string()));
        }

        // Floating point operations - not directly supported in Neo VM
        Operator::F32Const { .. }
        | Operator::F64Const { .. }
        | Operator::F32Add
        | Operator::F64Add
        | Operator::F32Sub
        | Operator::F64Sub
        | Operator::F32Mul
        | Operator::F64Mul
        | Operator::F32Div
        | Operator::F64Div => {
            return Err(Error::UnsupportedWasmFeature("Floating-point operations not supported in Neo VM".to_string()));
        }

        // SIMD and other advanced operations
        Operator::V128Load { .. } | Operator::V128Store { .. } => {
            return Err(Error::UnsupportedWasmFeature("SIMD operations not supported in Neo VM".to_string()));
        }

        // Atomic operations
        Operator::AtomicFence { .. } | Operator::I32AtomicLoad { .. } | Operator::I32AtomicStore { .. } => {
            return Err(Error::UnsupportedWasmFeature("Atomic operations not supported in Neo VM".to_string()));
        }

        // Catch-all for all other unsupported operations
        _ => {
            return Err(Error::UnsupportedWasmFeature(format!("Unsupported WASM instruction: {:?}", op)));
        }
    }

    Ok(())
}

/// Handles a WebAssembly memory load operation by converting it to appropriate Neo VM operations.
/// In Neo N3, we map WebAssembly linear memory to Neo storage, using keys like "mem_{address}".
///
/// # Arguments
///
/// * `script` - The Neo VM script to emit instructions to
/// * `offset` - The fixed offset from the effective address (base + offset) as u32
/// * `size` - The size of the data to load in bytes (1, 2, 4, or 8)
/// * `signed` - Whether to sign-extend the result for small integer loads
///
/// # Implementation Details
///
/// This function implements WebAssembly memory loads by:
/// 1. Computing the effective address by adding the base address (from stack) and offset
/// 2. Creating a storage key in the format "mem_{address}"
/// 3. Loading the value from Neo storage
/// 4. Handling proper size and signedness handling
fn handle_memory_load(script: &mut Script, offset: u32, size: u32, signed: bool) -> Result<(), Error> {
    // 1. Compute the effective address: base (on stack) + offset
    if offset > 0 {
        script.emit_push_integer(offset as i64);
        script.emit_opcode(OpCode::ADD);
    }

    // 2. Create the storage key: "mem_{address}"
    script.emit_opcode(OpCode::CONVERT);
    script.emit_with_operand(OpCode::CONVERT, vec![0x0C]); // Convert to string
    script.emit_push_data(b"mem_")?;
    script.emit_opcode(OpCode::CAT); // Concatenate to form the key

    // 3. Load from storage using Neo N3 Storage.Get syscall
    script.emit_push_data(b"Storage.Get")?;
    script.emit_opcode(OpCode::SYSCALL);

    // 4. If the value doesn't exist, return 0
    script.emit_opcode(OpCode::DUP);
    script.emit_opcode(OpCode::ISNULL);
    script.emit_opcode(OpCode::JMPIF);
    script.emit_with_operand(OpCode::JMPIF, vec![3]); // Jump 3 bytes ahead
    script.emit_opcode(OpCode::DROP); // Drop the null
    script.emit_push_integer(0); // Default value

    // 5. If we need to handle specific sized loads differently
    match size {
        1 => {
            // For byte loads, we need to mask to keep only the lowest 8 bits
            script.emit_push_integer(0xFF);
            script.emit_opcode(OpCode::AND);

            // If signed, sign extend
            if signed {
                // We handle sign extension for types < 4 bytes in the individual operations
                // For example, I32Load8S extends from 8 to 32 bits
            }
        }
        2 => {
            // For 16-bit loads, we need to mask to keep only the lowest 16 bits
            script.emit_push_integer(0xFFFF);
            script.emit_opcode(OpCode::AND);

            // If signed, sign extend
            if signed {
                // We handle sign extension for types < 4 bytes in the individual operations
                // For example, I32Load16S extends from 16 to 32 bits
            }
        }
        4 | 8 => {
            // No additional masking needed for 32-bit or 64-bit loads
            // The value from storage is already the correct size

            // Sign extension for 32-bit to 64-bit loads is handled in individual operations
            // like I64Load32S
        }
        _ => return Err(Error::invalid_wasm(format!("Unsupported load size: {}", size))),
    }

    Ok(())
}

/// Handles a WebAssembly memory store operation by converting it to appropriate Neo VM operations.
/// In Neo N3, we map WebAssembly linear memory to Neo storage, using keys like "mem_{address}".
///
/// # Arguments
///
/// * `script` - The Neo VM script to emit instructions to
/// * `offset` - The fixed offset from the effective address (base + offset) as u32
/// * `size` - The size of the data to store in bytes (1, 2, 4, or 8)
///
/// # Implementation Details
///
/// This function implements WebAssembly memory stores by:
/// 1. Computing the effective address by adding the base address and offset
/// 2. Creating a storage key in the format "mem_{address}"
/// 3. Storing the value to Neo storage with proper size handling
fn handle_memory_store(
    script: &mut Script,
    offset: u32,
    _size: u32, // Use _ prefix to indicate this is intentionally unused
) -> Result<(), Error> {
    // Validate the size parameter
    match _size {
        1 | 2 | 4 | 8 => {} // Valid sizes
        _ => return Err(Error::invalid_wasm(format!("Unsupported store size: {}", _size))),
    }

    // 1. We have [address, value] on the stack
    script.emit_opcode(OpCode::SWAP); // Now [value, address]

    // 2. Compute the effective address: base + offset
    if offset > 0 {
        script.emit_push_integer(offset as i64);
        script.emit_opcode(OpCode::ADD);
    }

    // 3. Create the storage key: "mem_{address}"
    script.emit_opcode(OpCode::CONVERT);
    script.emit_with_operand(OpCode::CONVERT, vec![0x0C]); // Convert to string
    script.emit_push_data(b"mem_")?;
    script.emit_opcode(OpCode::CAT); // Concatenate to form the key

    // 4. Swap back to get [key, value]
    script.emit_opcode(OpCode::SWAP);

    // 5. Apply size constraint to value based on the size parameter
    // For sizes smaller than 8 bytes, we need to mask the value
    if _size < 8 {
        let mask = (1i64 << (_size * 8)) - 1;
        script.emit_push_integer(mask);
        script.emit_opcode(OpCode::AND); // Apply mask to ensure proper size
    }

    // 6. Store in storage using Neo N3 Storage.Put syscall
    script.emit_push_data(b"Storage.Put")?;
    script.emit_opcode(OpCode::SYSCALL);

    Ok(())
}

// Removed unused handle_memory_operation function as it was replaced with
// more specialized handle_memory_load and handle_memory_store functions

#[cfg(test)]
mod tests {
    use super::*;
    use wasmparser::Operator;

    #[test]
    fn test_convert_simple_instructions() {
        let mut script = Script::new();

        // Test a simple i32.const followed by i32.add
        // Use HashMap<u32, u64> for function offsets as per the updated signature
        let function_offsets: HashMap<u32, u64> = HashMap::new();
        convert_instruction(&mut script, Operator::I32Const { value: 42 }, &function_offsets).unwrap();
        convert_instruction(&mut script, Operator::I32Const { value: 58 }, &function_offsets).unwrap();
        convert_instruction(&mut script, Operator::I32Add, &function_offsets).unwrap();

        // The resulting script should have 3 instructions:
        // 1. Push 42
        // 2. Push 58
        // 3. ADD
        assert_eq!(script.instructions.len(), 3);

        // The last instruction should be ADD
        if let Some(last) = script.instructions.last() {
            assert_eq!(last.opcode, OpCode::ADD);
        } else {
            panic!("No instructions were generated");
        }
    }
}
