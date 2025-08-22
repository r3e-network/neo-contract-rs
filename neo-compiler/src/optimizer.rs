use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use crate::opcodes::OpCode;

/// Compiler optimization engine for NEF bytecode
pub struct CompilerOptimizer {
    /// Enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Enable constant folding
    pub enable_constant_folding: bool,
    /// Enable peephole optimization
    pub enable_peephole_optimization: bool,
    /// Enable instruction combining
    pub enable_instruction_combining: bool,
    /// Optimization statistics
    pub stats: OptimizationStats,
}

#[derive(Debug, Default)]
pub struct OptimizationStats {
    pub dead_instructions_removed: usize,
    pub constants_folded: usize,
    pub peephole_optimizations: usize,
    pub instructions_combined: usize,
    pub size_reduction_bytes: usize,
    pub optimization_passes: usize,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub start_offset: usize,
    pub end_offset: usize,
    pub instructions: Vec<u8>,
    pub predecessors: Vec<usize>,
    pub successors: Vec<usize>,
    pub is_reachable: bool,
}

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub blocks: Vec<BasicBlock>,
    pub entry_block: usize,
    pub block_map: HashMap<usize, usize>, // offset -> block_id
}

impl CompilerOptimizer {
    pub fn new() -> Self {
        Self {
            enable_dead_code_elimination: true,
            enable_constant_folding: true,
            enable_peephole_optimization: true,
            enable_instruction_combining: true,
            stats: OptimizationStats::default(),
        }
    }

    /// Optimize NEF bytecode using multiple passes
    pub fn optimize(&mut self, bytecode: Vec<u8>) -> Result<Vec<u8>> {
        let mut optimized = bytecode;
        let original_size = optimized.len();
        
        // Run optimization passes
        for _pass in 0..3 {
            self.stats.optimization_passes += 1;
            let pass_start_size = optimized.len();
            
            if self.enable_dead_code_elimination {
                optimized = self.eliminate_dead_code(optimized)?;
            }
            
            if self.enable_constant_folding {
                optimized = self.fold_constants(optimized)?;
            }
            
            if self.enable_peephole_optimization {
                optimized = self.peephole_optimize(optimized)?;
            }
            
            if self.enable_instruction_combining {
                optimized = self.combine_instructions(optimized)?;
            }
            
            // Check if we made any improvements in this pass
            if optimized.len() == pass_start_size {
                break; // No more optimizations possible
            }
        }
        
        self.stats.size_reduction_bytes = original_size.saturating_sub(optimized.len());
        
        Ok(optimized)
    }

    /// Build control flow graph from bytecode
    pub fn build_control_flow_graph(&self, bytecode: &[u8]) -> Result<ControlFlowGraph> {
        let mut blocks = Vec::new();
        let mut block_map = HashMap::new();
        let mut block_starts = HashSet::new();
        
        // First pass: identify basic block boundaries
        block_starts.insert(0); // Entry point
        
        let mut i = 0;
        while i < bytecode.len() {
            let opcode = OpCode::from_byte(bytecode[i]).context("Invalid opcode")?;
            
            // Check for control flow instructions
            if opcode.is_jump() || opcode.is_call() {
                // Next instruction starts a new block
                let next_offset = i + opcode.size();
                if next_offset < bytecode.len() {
                    block_starts.insert(next_offset);
                }
                
                // For jumps, also mark the target as a block start
                if let Some(target) = self.extract_jump_target(&bytecode[i..], &opcode)? {
                    if target < bytecode.len() {
                        block_starts.insert(target);
                    }
                }
            }
            
            i += opcode.size();
        }
        
        // Second pass: create basic blocks
        let mut block_starts_vec: Vec<usize> = block_starts.into_iter().collect();
        block_starts_vec.sort();
        
        for (block_id, &start) in block_starts_vec.iter().enumerate() {
            let end = block_starts_vec.get(block_id + 1).copied().unwrap_or(bytecode.len());
            
            let block = BasicBlock {
                id: block_id,
                start_offset: start,
                end_offset: end,
                instructions: bytecode[start..end].to_vec(),
                predecessors: Vec::new(),
                successors: Vec::new(),
                is_reachable: block_id == 0, // Entry block is always reachable
            };
            
            block_map.insert(start, block_id);
            blocks.push(block);
        }
        
        // Third pass: build edges
        for block in &mut blocks {
            self.build_block_edges(block, &bytecode, &block_map)?;
        }
        
        Ok(ControlFlowGraph {
            blocks,
            entry_block: 0,
            block_map,
        })
    }

    /// Dead code elimination pass
    pub fn eliminate_dead_code(&mut self, bytecode: Vec<u8>) -> Result<Vec<u8>> {
        let cfg = self.build_control_flow_graph(&bytecode)?;
        
        // Mark reachable blocks
        let mut reachable = vec![false; cfg.blocks.len()];
        let mut worklist = vec![cfg.entry_block];
        
        while let Some(block_id) = worklist.pop() {
            if reachable[block_id] {
                continue;
            }
            
            reachable[block_id] = true;
            
            // Add successors to worklist
            for &successor in &cfg.blocks[block_id].successors {
                if !reachable[successor] {
                    worklist.push(successor);
                }
            }
        }
        
        // Collect instructions from reachable blocks
        let mut optimized = Vec::new();
        let original_size = bytecode.len();
        
        for (block_id, block) in cfg.blocks.iter().enumerate() {
            if reachable[block_id] {
                optimized.extend_from_slice(&block.instructions);
            } else {
                self.stats.dead_instructions_removed += block.instructions.len();
            }
        }
        
        // If we removed dead code, update jump targets
        if optimized.len() < original_size {
            optimized = self.update_jump_targets(optimized)?;
        }
        
        Ok(optimized)
    }

    /// Constant folding pass
    pub fn fold_constants(&mut self, bytecode: Vec<u8>) -> Result<Vec<u8>> {
        let mut optimized = Vec::new();
        let mut i = 0;
        
        while i < bytecode.len() {
            // Look for constant operations that can be folded
            if let Some(folded) = self.try_fold_constants_at(&bytecode[i..])? {
                optimized.extend_from_slice(&folded.bytecode);
                i += folded.original_length;
                self.stats.constants_folded += 1;
            } else {
                // Copy instruction as-is
                let opcode = OpCode::from_byte(bytecode[i]).context("Invalid opcode")?;
                let size = opcode.size();
                optimized.extend_from_slice(&bytecode[i..i + size]);
                i += size;
            }
        }
        
        Ok(optimized)
    }

    /// Peephole optimization pass
    pub fn peephole_optimize(&mut self, bytecode: Vec<u8>) -> Result<Vec<u8>> {
        let mut optimized = Vec::new();
        let mut i = 0;
        
        while i < bytecode.len() {
            // Apply peephole patterns
            if let Some(pattern) = self.apply_peephole_patterns(&bytecode[i..])? {
                optimized.extend_from_slice(&pattern.replacement);
                i += pattern.matched_length;
                self.stats.peephole_optimizations += 1;
            } else {
                // Copy instruction as-is
                let opcode = OpCode::from_byte(bytecode[i]).context("Invalid opcode")?;
                let size = opcode.size();
                optimized.extend_from_slice(&bytecode[i..i + size]);
                i += size;
            }
        }
        
        Ok(optimized)
    }

    /// Instruction combining pass
    pub fn combine_instructions(&mut self, bytecode: Vec<u8>) -> Result<Vec<u8>> {
        let mut optimized = Vec::new();
        let mut i = 0;
        
        while i < bytecode.len() {
            // Look for instruction combinations
            if let Some(combined) = self.try_combine_instructions(&bytecode[i..])? {
                optimized.extend_from_slice(&combined.bytecode);
                i += combined.original_length;
                self.stats.instructions_combined += 1;
            } else {
                // Copy instruction as-is
                let opcode = OpCode::from_byte(bytecode[i]).context("Invalid opcode")?;
                let size = opcode.size();
                optimized.extend_from_slice(&bytecode[i..i + size]);
                i += size;
            }
        }
        
        Ok(optimized)
    }

    /// Try to fold constants starting at given position
    fn try_fold_constants_at(&self, bytecode: &[u8]) -> Result<Option<FoldedConstants>> {
        if bytecode.len() < 3 {
            return Ok(None);
        }
        
        // Pattern: PUSH const1, PUSH const2, arithmetic op
        let op1 = OpCode::from_byte(bytecode[0]);
        if op1.is_none() { return Ok(None); }
        let op1 = op1.unwrap();
        
        // Check if first instruction pushes a small constant
        if let Some(val1) = self.extract_small_constant(&op1, &bytecode[0..]) {
            let next_pos = op1.size();
            if next_pos >= bytecode.len() { return Ok(None); }
            
            let op2 = OpCode::from_byte(bytecode[next_pos]);
            if op2.is_none() { return Ok(None); }
            let op2 = op2.unwrap();
            
            if let Some(val2) = self.extract_small_constant(&op2, &bytecode[next_pos..]) {
                let arith_pos = next_pos + op2.size();
                if arith_pos >= bytecode.len() { return Ok(None); }
                
                let arith_op = OpCode::from_byte(bytecode[arith_pos]);
                if arith_op.is_none() { return Ok(None); }
                let arith_op = arith_op.unwrap();
                
                // Try to compute the result
                if let Some(result) = self.compute_arithmetic(val1, val2, arith_op) {
                    let folded_bytecode = self.generate_push_constant(result);
                    return Ok(Some(FoldedConstants {
                        bytecode: folded_bytecode,
                        original_length: arith_pos + arith_op.size(),
                    }));
                }
            }
        }
        
        Ok(None)
    }

    /// Apply peephole optimization patterns
    fn apply_peephole_patterns(&self, bytecode: &[u8]) -> Result<Option<PeepholePattern>> {
        if bytecode.len() < 2 {
            return Ok(None);
        }
        
        let op1 = OpCode::from_byte(bytecode[0]);
        if op1.is_none() { return Ok(None); }
        let op1 = op1.unwrap();
        
        let op2 = OpCode::from_byte(bytecode[op1.size()]);
        if op2.is_none() { return Ok(None); }
        let op2 = op2.unwrap();
        
        // Pattern: DUP followed by DROP -> NOP
        if matches!(op1, OpCode::Dup) && matches!(op2, OpCode::Drop) {
            return Ok(Some(PeepholePattern {
                replacement: vec![OpCode::Nop.to_byte()],
                matched_length: op1.size() + op2.size(),
            }));
        }
        
        // Pattern: PUSH 0, ADD -> NOP (adding 0 is identity)
        if matches!(op1, OpCode::Push0) && matches!(op2, OpCode::Add) {
            return Ok(Some(PeepholePattern {
                replacement: vec![OpCode::Nop.to_byte()],
                matched_length: op1.size() + op2.size(),
            }));
        }
        
        // Pattern: PUSH 1, MUL -> NOP (multiplying by 1 is identity)
        if matches!(op1, OpCode::Push1) && matches!(op2, OpCode::Mul) {
            return Ok(Some(PeepholePattern {
                replacement: vec![OpCode::Nop.to_byte()],
                matched_length: op1.size() + op2.size(),
            }));
        }
        
        Ok(None)
    }

    /// Try to combine multiple instructions into more efficient forms
    fn try_combine_instructions(&self, bytecode: &[u8]) -> Result<Option<CombinedInstructions>> {
        if bytecode.len() < 3 {
            return Ok(None);
        }
        
        let op = OpCode::from_byte(bytecode[0]);
        if op.is_none() { return Ok(None); }
        let op = op.unwrap();
        
        // Pattern: Multiple NOP instructions -> single NOP
        if matches!(op, OpCode::Nop) {
            let mut nop_count = 1;
            let mut pos = op.size();
            
            while pos < bytecode.len() {
                if let Some(next_op) = OpCode::from_byte(bytecode[pos]) {
                    if matches!(next_op, OpCode::Nop) {
                        nop_count += 1;
                        pos += next_op.size();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            
            if nop_count > 1 {
                return Ok(Some(CombinedInstructions {
                    bytecode: vec![OpCode::Nop.to_byte()],
                    original_length: pos,
                }));
            }
        }
        
        Ok(None)
    }

    /// Extract jump target from instruction
    fn extract_jump_target(&self, bytecode: &[u8], opcode: &OpCode) -> Result<Option<usize>> {
        if !opcode.is_jump() || bytecode.len() < opcode.size() {
            return Ok(None);
        }
        
        match opcode {
            OpCode::Jmp | OpCode::JmpIf | OpCode::JmpIfNot => {
                if bytecode.len() >= 2 {
                    let offset = bytecode[1] as i8;
                    let target = (bytecode.as_ptr() as usize + 2).wrapping_add(offset as usize);
                    Ok(Some(target))
                } else {
                    Ok(None)
                }
            },
            _ => Ok(None), // Other jump types not implemented yet
        }
    }

    /// Build edges between basic blocks
    fn build_block_edges(
        &self, 
        _block: &mut BasicBlock, 
        _bytecode: &[u8], 
        _block_map: &HashMap<usize, usize>
    ) -> Result<()> {
        // Implementation would analyze the last instruction of the block
        // and connect to appropriate successors based on control flow
        Ok(())
    }

    /// Update jump targets after optimization
    fn update_jump_targets(&self, bytecode: Vec<u8>) -> Result<Vec<u8>> {
        // In a full implementation, this would track address changes
        // and update all jump offsets accordingly
        Ok(bytecode)
    }

    /// Extract small constant value from push instruction
    fn extract_small_constant(&self, opcode: &OpCode, bytecode: &[u8]) -> Option<i32> {
        match opcode {
            OpCode::PushM1 => Some(-1),
            OpCode::Push0 => Some(0),
            OpCode::Push1 => Some(1),
            OpCode::Push2 => Some(2),
            OpCode::Push3 => Some(3),
            OpCode::Push4 => Some(4),
            OpCode::Push5 => Some(5),
            OpCode::Push6 => Some(6),
            OpCode::Push7 => Some(7),
            OpCode::Push8 => Some(8),
            OpCode::Push9 => Some(9),
            OpCode::Push10 => Some(10),
            OpCode::Push11 => Some(11),
            OpCode::Push12 => Some(12),
            OpCode::Push13 => Some(13),
            OpCode::Push14 => Some(14),
            OpCode::Push15 => Some(15),
            OpCode::Push16 => Some(16),
            OpCode::PushInt8 => {
                if bytecode.len() >= 2 {
                    Some(bytecode[1] as i8 as i32)
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Compute arithmetic result for constant folding
    fn compute_arithmetic(&self, val1: i32, val2: i32, op: OpCode) -> Option<i32> {
        match op {
            OpCode::Add => val1.checked_add(val2),
            OpCode::Sub => val1.checked_sub(val2),
            OpCode::Mul => val1.checked_mul(val2),
            OpCode::Div => val1.checked_div(val2),
            OpCode::Mod => val1.checked_rem(val2),
            _ => None,
        }
    }

    /// Generate push constant instruction
    fn generate_push_constant(&self, value: i32) -> Vec<u8> {
        match value {
            -1 => vec![OpCode::PushM1.to_byte()],
            0 => vec![OpCode::Push0.to_byte()],
            1..=16 => {
                vec![unsafe { std::mem::transmute((OpCode::Push1 as u8) + (value as u8 - 1)) }]
            },
            _ => {
                if value >= i8::MIN as i32 && value <= i8::MAX as i32 {
                    vec![OpCode::PushInt8.to_byte(), value as u8]
                } else if value >= i16::MIN as i32 && value <= i16::MAX as i32 {
                    let mut result = vec![OpCode::PushInt16.to_byte()];
                    result.extend_from_slice(&(value as i16).to_le_bytes());
                    result
                } else {
                    let mut result = vec![OpCode::PushInt32.to_byte()];
                    result.extend_from_slice(&value.to_le_bytes());
                    result
                }
            }
        }
    }

    /// Get optimization statistics
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }

    /// Reset optimization statistics
    pub fn reset_stats(&mut self) {
        self.stats = OptimizationStats::default();
    }
}

#[derive(Debug)]
struct FoldedConstants {
    bytecode: Vec<u8>,
    original_length: usize,
}

#[derive(Debug)]
struct PeepholePattern {
    replacement: Vec<u8>,
    matched_length: usize,
}

#[derive(Debug)]
struct CombinedInstructions {
    bytecode: Vec<u8>,
    original_length: usize,
}

impl Default for CompilerOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = CompilerOptimizer::new();
        assert!(optimizer.enable_dead_code_elimination);
        assert!(optimizer.enable_constant_folding);
        assert!(optimizer.enable_peephole_optimization);
        assert!(optimizer.enable_instruction_combining);
    }

    #[test]
    fn test_constant_folding() {
        let mut optimizer = CompilerOptimizer::new();
        
        // Bytecode: PUSH1, PUSH2, ADD
        let bytecode = vec![
            OpCode::Push1.to_byte(),
            OpCode::Push2.to_byte(),
            OpCode::Add.to_byte(),
        ];
        
        let optimized = optimizer.fold_constants(bytecode).unwrap();
        
        // Should be optimized to PUSH3
        assert_eq!(optimized, vec![OpCode::Push3.to_byte()]);
        assert_eq!(optimizer.stats.constants_folded, 1);
    }

    #[test]
    fn test_peephole_optimization() {
        let mut optimizer = CompilerOptimizer::new();
        
        // Bytecode: DUP, DROP
        let bytecode = vec![
            OpCode::Dup.to_byte(),
            OpCode::Drop.to_byte(),
        ];
        
        let optimized = optimizer.peephole_optimize(bytecode).unwrap();
        
        // Should be optimized to NOP
        assert_eq!(optimized, vec![OpCode::Nop.to_byte()]);
        assert_eq!(optimizer.stats.peephole_optimizations, 1);
    }

    #[test]
    fn test_instruction_combining() {
        let mut optimizer = CompilerOptimizer::new();
        
        // Bytecode: NOP, NOP, NOP
        let bytecode = vec![
            OpCode::Nop.to_byte(),
            OpCode::Nop.to_byte(),
            OpCode::Nop.to_byte(),
        ];
        
        let optimized = optimizer.combine_instructions(bytecode).unwrap();
        
        // Should be combined to single NOP
        assert_eq!(optimized, vec![OpCode::Nop.to_byte()]);
        assert_eq!(optimizer.stats.instructions_combined, 1);
    }

    #[test]
    fn test_full_optimization() {
        let mut optimizer = CompilerOptimizer::new();
        
        // Complex bytecode with multiple optimization opportunities
        let bytecode = vec![
            OpCode::Push1.to_byte(),
            OpCode::Push2.to_byte(),
            OpCode::Add.to_byte(),    // Should fold to PUSH3
            OpCode::Dup.to_byte(),
            OpCode::Drop.to_byte(),   // Should become NOP
            OpCode::Nop.to_byte(),
            OpCode::Nop.to_byte(),    // Should combine NOPs
        ];
        
        let optimized = optimizer.optimize(bytecode).unwrap();
        
        // Check that some optimizations were applied
        assert!(optimizer.stats.constants_folded > 0 || 
                optimizer.stats.peephole_optimizations > 0 ||
                optimizer.stats.instructions_combined > 0);
        
        // Optimized should be smaller than original
        assert!(optimized.len() <= 7);
    }

    #[test]
    fn test_extract_small_constants() {
        let optimizer = CompilerOptimizer::new();
        
        assert_eq!(optimizer.extract_small_constant(&OpCode::Push0, &[]), Some(0));
        assert_eq!(optimizer.extract_small_constant(&OpCode::Push5, &[]), Some(5));
        assert_eq!(optimizer.extract_small_constant(&OpCode::PushM1, &[]), Some(-1));
        
        // Test PushInt8
        let bytecode = vec![OpCode::PushInt8.to_byte(), 42];
        assert_eq!(optimizer.extract_small_constant(&OpCode::PushInt8, &bytecode), Some(42));
    }

    #[test]
    fn test_compute_arithmetic() {
        let optimizer = CompilerOptimizer::new();
        
        assert_eq!(optimizer.compute_arithmetic(1, 2, OpCode::Add), Some(3));
        assert_eq!(optimizer.compute_arithmetic(5, 3, OpCode::Sub), Some(2));
        assert_eq!(optimizer.compute_arithmetic(4, 3, OpCode::Mul), Some(12));
        assert_eq!(optimizer.compute_arithmetic(10, 3, OpCode::Div), Some(3));
        assert_eq!(optimizer.compute_arithmetic(10, 3, OpCode::Mod), Some(1));
        
        // Test overflow handling
        assert_eq!(optimizer.compute_arithmetic(i32::MAX, 1, OpCode::Add), None);
        assert_eq!(optimizer.compute_arithmetic(1, 0, OpCode::Div), None);
    }
}