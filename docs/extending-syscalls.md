# Extending Syscall Support

This guide explains how to add support for new Neo N3 syscalls to the neo-contract-rs framework and neo-wasm compiler.

## Overview

Adding support for a new syscall involves several steps across both the Rust framework and the Go compiler:

1. **Declare the syscall** in the Rust bindings with appropriate function signature
2. **Create a wrapper function** for idiomatic Rust usage
3. **Add the syscall mapping** in the Go compiler
4. **Test the implementation** to verify correct operation

## Step 1: Add Syscall Declaration

First, add the external function definition in `neo-contract/src/env/syscall.rs`:

```rust
#[link(wasm_import_module = "neo.syscall")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    // ... existing syscalls ...

    /// [New syscall documentation]
    pub(crate) fn system_your_new_syscall(param1: Type1, param2: Type2) -> ReturnType;
}
```

Follow these naming conventions:
- Use `snake_case` for function names
- Prefix the function with `system_` followed by the service name
- Choose appropriate parameter and return types based on the Neo specification

## Step 2: Create a Wrapper Function

Add a wrapper function in the appropriate module (e.g., `runtime/mod.rs` for Runtime syscalls):

```rust
/// User-friendly documentation for the function
#[inline(always)]
pub fn your_new_function(param1: Type1, param2: Type2) -> ReturnType {
    unsafe { env::syscall::system_your_new_syscall(param1, param2) }
}
```

## Step 3: Add Syscall Mapping

Add the mapping in `neo-wasm/rosetta/builtin/syscall.go`:

```go
syscalls = map[string]Syscall{
    // ... existing mappings ...
    
    "system_your_new_syscall": {
        "System.YourService.YourNewMethod", 
        []neo.ParamType{neo.ParamType1, neo.ParamType2}, 
        neo.ReturnType,
        "Description of what the syscall does",
    },
}
```

Make sure to:
- Match the Rust function name exactly in the map key
- Use the correct Neo VM syscall name in the `Name` field
- Specify the correct parameter types and return type
- Add a description to aid documentation and debugging

## Step 4: Testing the Implementation

Create a test contract that uses the new syscall:

```rust
use neo_contract::prelude::*;
use neo_contract::runtime;

#[neo::contract]
impl TestContract {
    #[neo::method]
    pub fn test_new_syscall(param1: Type1, param2: Type2) -> ReturnType {
        runtime::your_new_function(param1, param2)
    }
}
```

Then compile and test it:

```bash
# Compile the test contract
cargo build --target=wasm32-unknown-unknown --release --package test-contract

# Run the compiler with verbose logging
NEO_WASM_DEBUG=1 NEO_WASM_VERBOSE=1 ./neo-wasm/neo-wasm translate \
    --input target/wasm32-unknown-unknown/release/test_contract.wasm \
    --output build/test_contract.nef \
    --manifest build/test_contract.manifest.json \
    --save-neo-ops

# Validate the syscall implementation
./neo-wasm/neo-wasm validate-syscalls --nef build/test_contract.nef
```

## Handling Parameter Types

When mapping parameter types, use the following correlations:

| Rust Type | Neo ParamType |
|-----------|--------------|
| `bool` | `neo.ParamBoolean` |
| `i64`, `u64`, `Int256` | `neo.ParamInteger` |
| `ByteString` | `neo.ParamString` or `neo.ParamByteArray` |
| `H160` | `neo.ParamH160` |
| `PublicKey` | `neo.ParamPublicKey` |
| `Array<T>` | `neo.ParamArray` |
| `Map<K, V>` | `neo.ParamMap` |
| `StorageContext` | `neo.ParamInteropInterface` |
| `(void)` | `neo.ParamVoid` |

## Validating NEF Output

The syscall should appear in the NEF script when you analyze it:

```bash
# Decode the script section
echo "[script from NEF]" | base64 -d | hexdump -C
```

Look for:
- `0x68` byte (SYSCALL opcode)
- Followed by the syscall name string (e.g., "System.YourService.YourNewMethod")

## Common Issues and Solutions

1. **Syscall not found in NEF**: Verify the mapping in `syscall.go` matches the Rust function name
2. **Parameter type mismatches**: Check that parameter types match between Rust and Go definitions
3. **Runtime errors**: Ensure the syscall implementation follows the Neo protocol specification
4. **Gas consumption issues**: Some syscalls consume significant GAS, profile carefully

## Neo N3 Syscall Reference

Always refer to the [official Neo N3 documentation](https://docs.neo.org/docs/en-us/reference/scapi/framework.html) for the latest syscall definitions and parameter specifications. 