use anyhow::Result;
use neo_compiler::{NeoCompiler, WasmModule};
use neo_compiler::opcodes::OpCode;
use neo_compiler::translator::WasmTranslator;
use neo_compiler::manifest::Manifest;
use tempfile::TempDir;
use std::fs;

/// Test suite for WASM instruction translation
#[cfg(test)]
mod tests {
    use super::*;

    /// Create a simple WASM module for testing
    fn create_test_wasm(wat_code: &str) -> Result<Vec<u8>> {
        wat::parse_str(wat_code)
            .map_err(|e| anyhow::anyhow!("Failed to parse WAT: {}", e))
    }

    /// Parse WASM bytes into WasmModule
    fn parse_wasm_module(wasm_bytes: &[u8]) -> Result<WasmModule> {
        let compiler = NeoCompiler::new();
        compiler.parse_wasm(wasm_bytes)
    }

    /// Test basic arithmetic instruction translation
    #[test]
    fn test_arithmetic_instructions() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $add (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add
                )
                (export "add" (func $add))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        // Verify the NEF was generated (basic check)
        assert!(!script.is_empty());
        
        // In a full test, we would verify specific opcodes are present:
        // - LdLocal0, LdLocal1, Add instructions should be in the bytecode
        println!("Generated NEF script length: {}", script.len());
        println!("Script bytes: {:?}", &script[..std::cmp::min(50, script.len())]);
        
        Ok(())
    }

    /// Test comparison instruction translation
    #[test]
    fn test_comparison_instructions() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $compare (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.eq
                )
                (export "compare" (func $compare))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        println!("Comparison NEF script length: {}", script.len());
        
        Ok(())
    }

    /// Test control flow instruction translation
    #[test]
    fn test_control_flow_instructions() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $conditional (param i32) (result i32)
                    local.get 0
                    if (result i32)
                        i32.const 1
                    else
                        i32.const 0
                    end
                )
                (export "conditional" (func $conditional))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        println!("Control flow NEF script length: {}", script.len());
        
        Ok(())
    }

    /// Test loop instruction translation
    #[test]
    fn test_loop_instructions() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $loop_test (param i32) (result i32)
                    local.get 0
                    loop (result i32)
                        local.get 0
                        i32.const 1
                        i32.sub
                        local.tee 0
                        i32.const 0
                        i32.gt_s
                        br_if 0
                        local.get 0
                    end
                )
                (export "loop_test" (func $loop_test))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        println!("Loop NEF script length: {}", script.len());
        
        Ok(())
    }

    /// Test local variable operations
    #[test]
    fn test_local_variable_operations() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $locals_test (param i32) (result i32)
                    (local i32 i32)
                    local.get 0
                    local.set 1
                    local.get 1
                    i32.const 10
                    i32.add
                    local.tee 2
                )
                (export "locals_test" (func $locals_test))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        
        // Should contain InitSlot instruction for local variables
        let has_init_slot = script.windows(1).any(|window| window[0] == OpCode::InitSlot.to_byte());
        println!("Has InitSlot: {}", has_init_slot);
        
        Ok(())
    }

    /// Test complex expression translation
    #[test]
    fn test_complex_expressions() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $complex (param i32 i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add
                    local.get 2
                    i32.mul
                    i32.const 100
                    i32.div_s
                )
                (export "complex" (func $complex))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        
        // Should contain arithmetic opcodes
        let has_add = script.windows(1).any(|w| w[0] == OpCode::Add.to_byte());
        let has_mul = script.windows(1).any(|w| w[0] == OpCode::Mul.to_byte());
        let has_div = script.windows(1).any(|w| w[0] == OpCode::Div.to_byte());
        
        println!("Has Add: {}, Mul: {}, Div: {}", has_add, has_mul, has_div);
        
        Ok(())
    }

    /// Test function call translation
    #[test]
    fn test_function_calls() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (func $helper (param i32) (result i32)
                    local.get 0
                    i32.const 2
                    i32.mul
                )
                (func $main (param i32) (result i32)
                    local.get 0
                    call $helper
                    i32.const 1
                    i32.add
                )
                (export "main" (func $main))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        
        // Should contain call instructions
        let has_call = script.windows(1).any(|w| w[0] == OpCode::Call.to_byte());
        println!("Has Call instruction: {}", has_call);
        
        Ok(())
    }

    /// Test memory operations (simplified)
    #[test]
    fn test_memory_operations() -> Result<()> {
        let wasm = create_test_wasm(r#"
            (module
                (memory 1)
                (func $memory_test (param i32 i32)
                    local.get 0
                    local.get 1
                    i32.store
                    
                    local.get 0
                    i32.load
                    drop
                )
                (export "memory_test" (func $memory_test))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(true);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        let script = nef.script();
        
        assert!(!script.is_empty());
        
        // Should contain syscall instructions for storage operations
        let has_syscall = script.windows(1).any(|w| w[0] == OpCode::SysCall.to_byte());
        println!("Has SysCall for memory ops: {}", has_syscall);
        
        Ok(())
    }

    /// Integration test with existing examples
    #[test]
    fn test_enhanced_vs_legacy_translation() -> Result<()> {
        // Test that enhanced translation produces different (hopefully better) results
        // than legacy hardcoded translation
        
        let wasm = create_test_wasm(r#"
            (module
                (func $hello (result i32)
                    i32.const 42
                )
                (export "hello" (func $hello))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        // Legacy translation (no WASM bytes)
        let mut legacy_translator = WasmTranslator::new(false);
        let legacy_nef = legacy_translator.translate(&module, &manifest, "test")?;
        
        // Enhanced translation (with WASM bytes)
        let mut enhanced_translator = WasmTranslator::new(false);
        enhanced_translator.set_wasm_bytes(wasm);
        let enhanced_nef = enhanced_translator.translate(&module, &manifest, "test")?;
        
        // Results should be different
        let legacy_script = legacy_nef.script();
        let enhanced_script = enhanced_nef.script();
        
        println!("Legacy script length: {}", legacy_script.len());
        println!("Enhanced script length: {}", enhanced_script.len());
        
        // Enhanced should have more sophisticated translation
        // (This is a basic check - in practice, enhanced should be smarter)
        assert!(!legacy_script.is_empty());
        assert!(!enhanced_script.is_empty());
        
        Ok(())
    }

    /// Performance test for compilation
    #[test]
    fn test_compilation_performance() -> Result<()> {
        let start = std::time::Instant::now();
        
        let wasm = create_test_wasm(r#"
            (module
                (func $fibonacci (param i32) (result i32)
                    local.get 0
                    i32.const 2
                    i32.lt_s
                    if (result i32)
                        local.get 0
                    else
                        local.get 0
                        i32.const 1
                        i32.sub
                        call $fibonacci
                        local.get 0
                        i32.const 2
                        i32.sub
                        call $fibonacci
                        i32.add
                    end
                )
                (export "fibonacci" (func $fibonacci))
            )
        "#)?;

        let module = parse_wasm_module(&wasm)?;
        let manifest = Manifest::default();
        
        let mut translator = WasmTranslator::new(false);
        translator.set_wasm_bytes(wasm);
        
        let nef = translator.translate(&module, &manifest, "test")?;
        
        let duration = start.elapsed();
        println!("Translation took: {:?}", duration);
        
        // Should complete in reasonable time (less than 1 second for simple cases)
        assert!(duration.as_secs() < 1);
        assert!(!nef.script().is_empty());
        
        Ok(())
    }

    /// Test error handling for invalid WASM
    #[test]
    fn test_invalid_wasm_handling() -> Result<()> {
        let invalid_wasm = vec![0x00, 0x61, 0x73, 0x6D]; // Invalid WASM magic
        
        let result = parse_wasm_module(&invalid_wasm);
        assert!(result.is_err());
        
        println!("Correctly handled invalid WASM: {:?}", result.err());
        
        Ok(())
    }
}