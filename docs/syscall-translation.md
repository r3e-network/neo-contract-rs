# Syscall Translation in Neo-Wasm Compiler

This document outlines the process of translating Rust syscall invocations to Neo VM syscalls in the neo-contract-rs framework.

## Overview

The syscall translation is a critical part of the neo-wasm compiler's functionality. It allows Rust contracts to interact with the Neo N3 blockchain via syscalls, which provide access to blockchain state, runtime information, and other functionality.

## Translation Flow

1. The Rust contract uses the `runtime` module's wrapper functions.
2. These functions call into the `env::syscall` module which contains external functions with the `#[link(wasm_import_module = "neo.syscall")]` attribute.
3. When compiled to WASM, these become imports with module "neo.syscall".
4. The neo-wasm compiler recognizes these imports and transforms them into Neo VM syscall operations.

## Implementation Components

### 1. Rust Syscall Interface (neo-contract)

The syscall interface is defined in `neo-contract/src/env/syscall.rs`:

```rust
#[link(wasm_import_module = "neo.syscall")]
extern "C" {
    pub(crate) fn system_runtime_trigger() -> TriggerType;
    pub(crate) fn system_runtime_platform() -> ByteString;
    // Other syscalls...
}
```

### 2. Rust Runtime Wrappers (neo-contract)

User-friendly wrappers in `neo-contract/src/runtime/mod.rs`:

```rust
pub fn get_time() -> u64 {
    unsafe { env::syscall::system_runtime_time() }
}

pub fn get_platform() -> ByteString {
    unsafe { env::syscall::system_runtime_platform() }
}
```

### 3. Syscall Mapping (neo-wasm)

The mapping between Rust function names and Neo syscalls is defined in `neo-wasm/rosetta/builtin/syscall.go`:

```go
syscalls = map[string]Syscall{
    "system_runtime_trigger": {"System.Runtime.Trigger", []neo.ParamType{}, neo.ParamInteger},
    "system_runtime_platform": {"System.Runtime.Platform", []neo.ParamType{}, neo.ParamString},
    // Other mappings...
}
```

### 4. Syscall Translation (neo-wasm)

The actual translation occurs in `neo-wasm/rosetta/translates/syscall.go`:

```go
func (s *SyscallTranslation) Translate(reader *op.OpsReader, importName string) (*Translation, error) {
    syscall, ok := builtin.GetSyscallFromImport(importName)
    if !ok {
        return nil, fmt.Errorf("unknown syscall: %s", importName)
    }

    return &Translation{
        Sources: []WasmOp{},
        Targets: []neo.VmOp{
            {
                OpCode: neo.OpSyscall,
                First:  []byte(syscall.Name),
            },
        },
    }, nil
}
```

### 5. WASM Import Processing (neo-wasm)

The detection and registration of syscalls happens in `neo-wasm/rosetta/rosetta.go`:

```go
if builtin.IsSyscallModule(module) {
    syscallTranslation := translates.NewSyscall()
    translation, err := syscallTranslation.Translate(nil, importName)
    
    // Store the translation
    fnIndex := importFns[importName]
    compiledFns[fnIndex] = []*translates.Translation{translation}
    
    // Record this syscall for method tokens
    if syscall, ok := builtin.GetSyscallFromImport(importName); ok {
        usedSyscalls[syscall.Name] = syscall
    }
}
```

## Token Generation

The compiler automatically generates tokens for used syscalls:

```go
// Add all used syscalls as tokens
for _, syscall := range usedSyscalls {
    // Don't add duplicate tokens
    if !isDuplicate {
        paramCount := uint16(len(syscall.Params))
        
        tokens = append(tokens, neo.MethodToken{
            Hash:       zeroHash,
            Method:     syscall.Name,
            ParamCount: paramCount,
            HasReturn:  syscall.Return != neo.ParamVoid,
            CallFlags:  0, // Syscalls don't need flags
        })
    }
}
```

## Debugging Syscalls

When troubleshooting syscall issues:

1. Enable verbose logging with `NEO_WASM_DEBUG=1 NEO_WASM_VERBOSE=1`
2. Examine the logs for "Processing syscall import" messages
3. Check the generated NEF assembly file for SYSCALL operations
4. Verify the syscall name and parameters in the Neo VM bytecode

## Known Issues and Solutions

### 1. Missing Syscall Mappings

**Problem**: A Rust contract references a syscall that has no mapping in `syscall.go`.
**Solution**: Add the missing mapping to the `syscalls` map in `neo-wasm/rosetta/builtin/syscall.go`.

### 2. Incorrect Parameter Types

**Problem**: The parameter types in the syscall mapping don't match what Neo VM expects.
**Solution**: Update the parameter types in the syscall mapping to match Neo VM's expectations.

### 3. Syscall Not Being Included in Tokens

**Problem**: A used syscall is not being included in the method tokens.
**Solution**: Ensure the syscall is properly recorded in the `usedSyscalls` map during import processing.

## Testing Syscalls

To test syscall functionality:

1. Use the `syscall-test` example in `neo-wasm/examples/syscall-test/`
2. Run the `test-syscall.sh` script to compile and analyze the output
3. Check the generated NEF file for proper syscall encoding
4. Verify the syscalls in the NEO-CLI debugger

## Future Improvements

1. Add parameter validation to catch type mismatches early
2. Improve error messages for syscall translation issues
3. Support for custom syscall attributes in the manifest
4. Add more detailed documentation for each supported syscall
5. Implement caching for frequently used syscalls to improve compilation performance 