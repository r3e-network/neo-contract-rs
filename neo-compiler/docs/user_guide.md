# Neo Compiler User Guide

This guide provides a comprehensive overview of the neo-compiler, explaining how to use it to convert WebAssembly modules into Neo N3 smart contracts.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Command Line Usage](#command-line-usage)
4. [Programmatic Usage](#programmatic-usage)
5. [Advanced Usage](#advanced-usage)
6. [Working with NEF Files](#working-with-nef-files)
7. [Working with Manifests](#working-with-manifests)
8. [Working with Scripts](#working-with-scripts)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

## Introduction

The neo-compiler is a tool that enables developers to write Neo N3 smart contracts in Rust or any other language that compiles to WebAssembly. It takes WebAssembly modules and converts them to Neo Executable Format (NEF) files and contract manifests, which can be deployed to the Neo N3 blockchain.

## Installation

### From Source

To install the neo-compiler from source, run:

```bash
cargo install --git https://github.com/neo-project/neo-contract-rs neo-compiler
```

### Prerequisites

- Rust and Cargo (latest stable version)
- For compiling Rust contracts: rust-lld and wasm32-unknown-unknown target
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## Command Line Usage

The neo-compiler provides a command line interface for compiling WebAssembly modules.

### Basic Compilation

```bash
neo-compiler compile your_contract.wasm
```

This will compile the WebAssembly module and generate two files in the current directory:
- `your_contract.nef`: The Neo Executable Format file
- `your_contract.manifest.json`: The contract manifest

### Specifying Output Directory

```bash
neo-compiler compile your_contract.wasm --output-dir ./output
```

### Setting Contract Name

```bash
neo-compiler compile your_contract.wasm --name MyToken
```

### Enabling Debug Information

```bash
neo-compiler compile your_contract.wasm --debug
```

### Using a Custom Manifest Template

```bash
neo-compiler compile your_contract.wasm --manifest-template ./template.json
```

### Disabling Optimizations

```bash
neo-compiler compile your_contract.wasm --no-optimize
```

## Programmatic Usage

You can also use the neo-compiler programmatically in your Rust applications.

### Basic Usage

```rust
use neo_compiler::{Compiler, CompilerOptions};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the compiler
    let options = CompilerOptions {
        debug: true,
        optimize: true,
        contract_name: Some("MyContract".to_string()),
        manifest_template: None,
    };
    
    let compiler = Compiler::with_options(options);
    
    // Compile the WebAssembly module
    let wasm_path = Path::new("./your_contract.wasm");
    let output_dir = Path::new("./output");
    let contract_name = "MyContract";
    
    compiler.compile(wasm_path, output_dir, contract_name)?;
    
    println!("Compilation successful!");
    Ok(())
}
```

### Using Builder Pattern

The Compiler struct supports a builder pattern for more readable configuration:

```rust
use neo_compiler::Compiler;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let compiler = Compiler::new()
        .with_debug(true)
        .with_optimize(true)
        .with_name("MyContract");
    
    compiler.compile(
        Path::new("./your_contract.wasm"),
        Path::new("./output"),
        "MyContract"
    )?;
    
    Ok(())
}
```

### Quick Compile

For simple cases, you can use the quick_compile function:

```rust
use neo_compiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    neo_compiler::quick_compile("./your_contract.wasm")?;
    Ok(())
}
```

## Advanced Usage

### Working with WebAssembly Modules Directly

You can parse and analyze WebAssembly modules directly:

```rust
use neo_compiler::WasmModule;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_data = fs::read("./your_contract.wasm")?;
    let module = WasmModule::parse(&wasm_data)?;
    
    // Analyze the module
    println!("Functions: {}", module.functions().len());
    println!("Exports: {}", module.exports().len());
    
    // Get a specific function
    if let Some(func) = module.get_function_by_name("main") {
        println!("Main function has {} parameters", func.params.len());
    }
    
    Ok(())
}
```

### Converting WebAssembly to Neo VM Script

You can convert a WebAssembly module to Neo VM script directly:

```rust
use neo_compiler::{WasmModule, converter::WasmConverter};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_data = fs::read("./your_contract.wasm")?;
    let module = WasmModule::parse(&wasm_data)?;
    
    let converter = WasmConverter::new();
    let script_bytes = converter.convert_to_script(&module, true)?;
    
    fs::write("./output/script.neo", script_bytes)?;
    Ok(())
}
```

## Working with NEF Files

The `NefFile` struct provides functionality for working with Neo Executable Format files.

### Creating a NEF File

```rust
use neo_compiler::nef::NefFile;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a NEF file from a script
    let script = fs::read("./script.neo")?;
    let mut nef = NefFile::with_script(script);
    
    // Set compiler information
    let nef = nef.with_compiler("my-compiler");
    
    // Finalize and save
    nef.finalize()?;
    nef.save_to("./output/contract.nef")?;
    
    Ok(())
}
```

### Loading and Validating a NEF File

```rust
use neo_compiler::nef::NefFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nef = NefFile::load_from("./contract.nef")?;
    
    // Validate the NEF file
    nef.validate()?;
    
    println!("NEF file is valid");
    println!("Script size: {} bytes", nef.script.len());
    
    Ok(())
}
```

## Working with Manifests

The `Manifest` struct provides functionality for working with contract manifests.

### Creating a Manifest

```rust
use neo_compiler::manifest::{Manifest, ContractAbi, ContractMethodDefinition, ContractParameterDefinition};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new manifest
    let mut manifest = Manifest::new("MyToken");
    
    // Add method to the ABI
    let transfer_params = vec![
        ContractParameterDefinition::new("from", "Hash160"),
        ContractParameterDefinition::new("to", "Hash160"),
        ContractParameterDefinition::new("amount", "Integer"),
    ];
    let transfer_method = ContractMethodDefinition::new(
        "transfer",
        transfer_params,
        "Boolean",
        false,
    );
    manifest.add_method(transfer_method);
    
    // Set features
    manifest.set_feature("storage", true)?;
    manifest.set_feature("payable", true)?;
    
    // Add supported standards
    manifest.add_supported_standard("NEP-17");
    
    // Save to file
    manifest.save_to_file("./output/contract.manifest.json")?;
    
    Ok(())
}
```

### Loading and Validating a Manifest

```rust
use neo_compiler::manifest::Manifest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = Manifest::load_from_file("./contract.manifest.json")?;
    
    // Validate the manifest
    manifest.validate()?;
    
    println!("Manifest is valid");
    println!("Contract name: {}", manifest.name);
    println!("Methods: {}", manifest.abi.methods.len());
    
    Ok(())
}
```

## Working with Scripts

The `Script` struct provides functionality for working with Neo VM scripts.

### Creating a Script

```rust
use neo_compiler::{script::Script, neo::OpCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut script = Script::new();
    
    // Add instructions
    script.emit_push_integer(42);
    script.emit_push_integer(58);
    script.emit_opcode(OpCode::ADD);
    script.emit_opcode(OpCode::RET);
    
    // Save to file
    script.write_to_file("./output/add.neo")?;
    
    Ok(())
}
```

### Creating a Token Contract Script

Here's a more complex example creating a simplified NEP-17 token script:

```rust
use neo_compiler::{script::Script, neo::OpCode};

fn create_token_script() -> Result<Script, Box<dyn std::error::Error>> {
    let mut script = Script::new();
    
    // Contract entrypoint - dispatch based on method name
    script.emit_with_operand(OpCode::COMMENT, b"Contract entrypoint".to_vec());
    script.emit_opcode(OpCode::LDARG0);  // Load the first argument (method name)
    
    // Check for "name" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'name' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"name")?;
    script.emit_opcode(OpCode::EQUAL);
    
    // Jump to name implementation if matched
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_integer(100);  // Jump to offset 100 (placeholder)
    
    // Check for "symbol" method
    script.emit_with_operand(OpCode::COMMENT, b"Check for 'symbol' method".to_vec());
    script.emit_opcode(OpCode::DUP);
    script.emit_push_data(b"symbol")?;
    script.emit_opcode(OpCode::EQUAL);
    
    // Jump to symbol implementation if matched
    script.emit_opcode(OpCode::JMPIF);
    script.emit_push_integer(200);  // Jump to offset 200 (placeholder)
    
    // 'name' method implementation (would be at offset 100)
    script.emit_with_operand(OpCode::COMMENT, b"'name' method implementation".to_vec());
    script.emit_push_data(b"ExampleToken")?;
    script.emit_opcode(OpCode::RET);
    
    // 'symbol' method implementation (would be at offset 200)
    script.emit_with_operand(OpCode::COMMENT, b"'symbol' method implementation".to_vec());
    script.emit_push_data(b"EXT")?;
    script.emit_opcode(OpCode::RET);
    
    Ok(script)
}
```

## Best Practices

1. **Use Proper Error Handling**:
   ```rust
   use neo_compiler::error::Result;
   
   fn process_wasm(path: &str) -> Result<()> {
       // Your code here
       Ok(())
   }
   ```

2. **Modularize Complex Scripts**:
   Create helper functions for different parts of your script to keep your code maintainable.

3. **Use Comments in Scripts**:
   Add comments to your scripts using the `COMMENT` opcode to make them more readable.
   ```rust
   script.emit_with_operand(OpCode::COMMENT, b"Function: transfer".to_vec());
   ```

4. **Validate Generated Files**:
   Always validate your generated NEF files and manifests before deploying them.
   ```rust
   let nef = NefFile::load_from("./contract.nef")?;
   nef.validate()?;
   
   let manifest = Manifest::load_from_file("./contract.manifest.json")?;
   manifest.validate()?;
   ```

5. **Set Appropriate Contract Features**:
   Make sure to set the appropriate features in your contract manifest:
   ```rust
   manifest.set_feature("storage", uses_storage)?;
   manifest.set_feature("payable", is_payable)?;
   ```

## Troubleshooting

### Common Errors

1. **"Invalid NEF file: Invalid magic number"**:
   The file is not a valid NEF file or is corrupted.

2. **"Invalid manifest: Duplicate method name"**:
   You have multiple methods with the same name in your manifest.

3. **"WebAssembly parse error"**:
   The WebAssembly module is not valid or is corrupted.

4. **"Unsupported WebAssembly feature"**:
   The WebAssembly module uses features that are not supported by the neo-compiler.

### Debugging Tips

1. **Enable Debug Output**:
   Use the `--debug` flag when compiling to get more detailed output.

2. **Inspect Generated Files**:
   Examine the generated NEF and manifest files to understand what the compiler is producing.

3. **Check WebAssembly Module**:
   Use tools like `wasm2wat` to convert your WebAssembly module to WebAssembly text format for inspection.
   ```bash
   wasm2wat your_contract.wasm -o your_contract.wat
   ```

4. **Start Simple**:
   If you're having issues, start with a simple contract and gradually add complexity.

---

This guide provides a comprehensive overview of the neo-compiler. For more detailed documentation, refer to the [RESTRUCTURING.md](../RESTRUCTURING.md) file and the API documentation. 