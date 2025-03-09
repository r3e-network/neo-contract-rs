# Migration Guide

This guide will help you migrate your code from the previous API structure to the new, consolidated API in neo-compiler.

## Overview of Changes

The neo-compiler project has undergone a major restructuring to eliminate code duplication and provide a cleaner, more consistent API. The key changes include:

1. Consolidation of duplicate implementations into single, well-structured modules
2. Standardization of error handling across the codebase
3. Consistent naming conventions and parameter types
4. Improved builder patterns for object construction
5. Better encapsulation of implementation details

## Migration Steps

### For Users of the Command Line Interface

If you're using the neo-compiler CLI, the changes are mostly backward compatible. You can continue to use the CLI as before.

### For Users of the Programmatic API

If you're using the neo-compiler programmatically, you'll need to update your code to use the new API.

### Imports

**Before:**

```rust
use neo_compiler::wasm::Module;
use neo_compiler::nef::NefFile;
use neo_compiler::manifest::Manifest;
use neo_compiler::script::Script;
```

**After:**

```rust
use neo_compiler::{WasmModule, NefFile, Manifest, Script};
// Or individually:
use neo_compiler::wasm::WasmModule;
use neo_compiler::nef::NefFile;
use neo_compiler::manifest::Manifest;
use neo_compiler::script::Script;
```

### Compiler Usage

**Before:**

```rust
use neo_compiler::compiler::Compiler;
use std::path::Path;

let compiler = Compiler::new(Path::new("input.wasm"), Path::new("output"));
compiler.with_name("MyContract")
        .compile()
        .unwrap();
```

**After:**

```rust
use neo_compiler::{Compiler, CompilerOptions};
use std::path::Path;

// Option 1: Using CompilerOptions
let options = CompilerOptions {
    debug: true,
    optimize: true,
    contract_name: Some("MyContract".to_string()),
    manifest_template: None,
};
let compiler = Compiler::with_options(options);

// Option 2: Using builder pattern
let compiler = Compiler::new()
    .with_debug(true)
    .with_optimize(true)
    .with_name("MyContract");

// Compile
compiler.compile(
    Path::new("input.wasm"),
    Path::new("output"),
    "MyContract"
).unwrap();

// Or using quick_compile for simple cases
neo_compiler::quick_compile("input.wasm").unwrap();
```

### NEF File Handling

**Before:**

```rust
use neo_compiler::nef::NefFile;

let nef = NefFile::new("mycompiler".to_string(), script_bytes);
nef.save("output.nef").unwrap();
```

**After:**

```rust
use neo_compiler::nef::NefFile;

let mut nef = NefFile::with_script(script_bytes)
    .with_compiler("mycompiler");
nef.finalize().unwrap();
nef.save_to("output.nef").unwrap();
```

### Manifest Handling

**Before:**

```rust
use neo_compiler::manifest::{Manifest, Method, Event, Parameter};

let mut manifest = Manifest::new("MyContract".to_string());
let mut method = Method::new("transfer".to_string(), "Boolean".to_string(), 0);
method.add_parameter("from".to_string(), "Hash160".to_string());
method.add_parameter("to".to_string(), "Hash160".to_string());
method.add_parameter("amount".to_string(), "Integer".to_string());
manifest.add_method(method);
```

**After:**

```rust
use neo_compiler::manifest::{Manifest, ContractMethodDefinition, ContractParameterDefinition};

let mut manifest = Manifest::new("MyContract");
let transfer_params = vec![
    ContractParameterDefinition::new("from", "Hash160"),
    ContractParameterDefinition::new("to", "Hash160"),
    ContractParameterDefinition::new("amount", "Integer"),
];
let transfer_method = ContractMethodDefinition::new(
    "transfer",
    transfer_params,
    "Boolean",
    false
);
manifest.add_method(transfer_method);
```

### Script Handling

**Before:**

```rust
use neo_compiler::script::Script;
use neo_compiler::neo::OpCode;

let mut script = Script::new();
script.emit(OpCode::PUSH1);
script.emit(OpCode::PUSH2);
script.emit(OpCode::ADD);
```

**After:**

```rust
use neo_compiler::script::Script;
use neo_compiler::neo::OpCode;

let mut script = Script::new();
script.emit_opcode(OpCode::PUSH1);
script.emit_opcode(OpCode::PUSH2);
script.emit_opcode(OpCode::ADD);
```

### Error Handling

**Before:**

```rust
use neo_compiler::error::{Error, Result};

fn my_function() -> Result<()> {
    // ...
    Ok(())
}
```

**After:**

```rust
use neo_compiler::error::Result;

fn my_function() -> Result<()> {
    // ...
    Ok(())
}
```

## Deprecated Modules

Some modules have been deprecated but are still available for backward compatibility:

```rust
// Deprecated - will be removed in future versions
use neo_compiler::wasm::LegacyModule;
use neo_compiler::nef::LegacyNefFile;
use neo_compiler::manifest::LegacyManifest;
```

These deprecated modules will be removed in future versions, so it's recommended to migrate to the new API as soon as possible.

## Comprehensive Examples

For comprehensive examples of using the new API, see the examples directory:

- [simple_compilation.rs](../examples/simple_compilation.rs): Basic usage of the compiler
- [manual_script_creation.rs](../examples/manual_script_creation.rs): Creating Neo VM scripts manually
- [nep17_token.rs](../examples/nep17_token.rs): Implementing a NEP-17 token contract

## Need Help?

If you encounter issues during migration or have questions about the new API, please open an issue in the GitHub repository.

---

For more detailed information on the restructuring, see the [RESTRUCTURING.md](../RESTRUCTURING.md) file. 