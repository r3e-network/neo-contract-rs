# Neo Compiler Documentation

The `neo-compiler` crate is a critical component of the Neo Contract Rust Framework, responsible for converting WebAssembly (WASM) bytecode to Neo Virtual Machine (NeoVM) bytecode and generating the necessary deployment files.

## Overview

The Neo Compiler transforms WebAssembly modules into executable Neo contracts by:

1. Parsing WebAssembly modules
2. Converting WebAssembly instructions to equivalent NeoVM instructions
3. Generating NEF (Neo Executable Format) files
4. Creating contract manifests

## Core Components

### Compiler

The `Compiler` struct is the main entry point for the compilation process:

```rust
pub struct Compiler {
    // Configuration and state
    pub options: CompilerOptions,
    converter: Converter,
}
```

Key methods:
- `compile_contract`: Compiles a WASM file to NEF and manifest files
- `compile_script`: Compiles a WASM file to a NeoVM script
- `compile_from_bytes`: Compiles WASM bytecode to NeoVM bytecode

Usage example:
```rust
let compiler = Compiler::new(CompilerOptions::default());
compiler.compile_contract(
    "contract.wasm",
    "contract.nef",
    "contract.manifest.json",
    "MyContract"
)?;
```

### WebAssembly Module

The `WasmModule` struct represents a parsed WebAssembly module:

```rust
pub struct WasmModule {
    // Module metadata and content
    pub name: String,
    pub functions: Vec<WasmFunction>,
    pub imports: Vec<WasmImport>,
    pub exports: Vec<WasmExport>,
    pub memories: Vec<WasmMemory>,
    // More fields...
}
```

Key methods:
- `from_file`: Loads a WASM module from a file
- `from_bytes`: Parses a WASM module from binary data
- `get_function_by_name`: Retrieves a function by name

### Converter

The `Converter` handles the conversion from WebAssembly to NeoVM instructions:

```rust
pub struct Converter {
    // Conversion state and configuration
    instructions: InstructionConverter,
    syscalls: SyscallMap,
    // More fields...
}
```

Key methods:
- `convert_module`: Converts a WASM module to NeoVM bytecode
- `convert_function`: Converts a WASM function to NeoVM bytecode
- `convert_to_script`: Converts a WASM module to a NeoVM script

### Script

The `Script` struct represents a NeoVM script:

```rust
pub struct Script {
    // Script content
    bytes: Vec<u8>,
    comments: HashMap<usize, String>,
}
```

Key methods:
- `new`: Creates a new empty script
- `emit_opcode`: Emits a NeoVM opcode
- `emit_push_data`: Pushes data onto the script
- `emit_with_operand`: Emits an opcode with an operand
- `bytes`: Gets the raw script bytes

### NEF File

The `NefFile` struct represents a Neo Executable Format file:

```rust
pub struct NefFile {
    // NEF file content
    pub magic: u32,
    pub compiler: String,
    pub version: [u8; 4],
    pub script: Vec<u8>,
    pub checksum: [u8; 32],
}
```

Key methods:
- `new`: Creates a new empty NEF file
- `with_script`: Creates a NEF file with the given script
- `finalize`: Finalizes the NEF file by calculating the checksum
- `save_to`: Saves the NEF file to disk

### Manifest

The `Manifest` struct represents a Neo contract manifest:

```rust
pub struct Manifest {
    // Manifest content
    pub name: String,
    pub abi: ContractAbi,
    pub features: HashMap<String, bool>,
    pub groups: Vec<Group>,
    pub supported_standards: Vec<String>,
    pub permissions: Vec<Permission>,
    pub trusts: Vec<String>,
    pub extra: HashMap<String, JsonValue>,
}
```

Key methods:
- `new`: Creates a new manifest with the given name
- `with_abi`: Sets the ABI for the manifest
- `set_feature`: Sets a feature flag
- `add_supported_standard`: Adds a supported standard
- `validate`: Validates the manifest
- `to_json`: Serializes the manifest to JSON
- `save_to_file`: Saves the manifest to disk

## Compilation Process Details

### 1. WASM Parsing

The first step of the compilation process is parsing the WebAssembly module:

```rust
let wasm_module = WasmModule::from_file("contract.wasm")?;
```

This involves:
- Validating the WASM binary format
- Extracting module metadata
- Parsing function definitions
- Identifying imports and exports
- Analyzing memory usage

### 2. Instruction Conversion

The core of the compilation process is converting WebAssembly instructions to NeoVM instructions:

```rust
let neo_script = converter.convert_module(&wasm_module, optimize)?;
```

This conversion handles:
- Stack manipulation differences
- Control flow constructs (loops, branches)
- Memory access patterns
- Function calls
- Type conversions

The conversion process uses a mapping table to translate WASM opcodes to NeoVM opcodes, with special handling for complex instructions.

#### Key Conversion Challenges

**Stack Management**
WebAssembly and NeoVM have different stack models. The converter uses stack analysis to ensure correct operation.

**Control Flow**
WebAssembly structured control flow (blocks, loops, if/else) must be converted to NeoVM's jump-based control flow.

**Memory Access**
WebAssembly linear memory access must be mapped to NeoVM storage operations.

### 3. NEF File Generation

After conversion, the NeoVM bytecode is packaged into a NEF file:

```rust
let nef = NefFile::with_script(neo_script.bytes().to_vec());
nef.finalize()?;
nef.save_to("contract.nef")?;
```

This includes:
- Setting the magic number and version
- Adding compiler information
- Including the script bytecode
- Calculating the checksum

### 4. Manifest Generation

Finally, a contract manifest is generated:

```rust
let mut manifest = Manifest::new("MyContract");
manifest.abi = generate_abi_from_module(&wasm_module);
manifest.set_feature("storage", true)?;
manifest.save_to_file("contract.manifest.json")?;
```

The manifest includes:
- Contract name and metadata
- ABI derived from WASM exports
- Feature flags based on contract requirements
- Permissions and other settings

## Error Handling

The neo-compiler uses a comprehensive error handling system:

```rust
pub enum Error {
    // IO errors
    Io(String),
    
    // WASM parsing errors
    InvalidWasm(String),
    
    // Conversion errors
    Conversion(String),
    
    // Script generation errors
    Script(String),
    
    // Manifest errors
    Manifest(String),
    
    // Other errors
    General(String),
}
```

Errors are propagated throughout the compilation process using Rust's `Result` type, providing detailed error information.

## API Usage Examples

### Basic Compilation

```rust
use neo_compiler::{Compiler, CompilerOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the compiler
    let compiler = Compiler::new(CompilerOptions::default());
    
    // Compile a contract
    compiler.compile_contract(
        "input.wasm",     // Input WASM file
        "output.nef",     // Output NEF file
        "output.manifest.json", // Output manifest file
        "MyContract"      // Contract name
    )?;
    
    println!("Compilation successful!");
    Ok(())
}
```

### Manual Script Creation

```rust
use neo_compiler::{
    neo::OpCode,
    script::Script,
    nef::NefFile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new script
    let mut script = Script::new();
    
    // Add instructions
    script.emit_comment("Simple addition");
    script.emit_opcode(OpCode::ADD);
    script.emit_opcode(OpCode::RET);
    
    // Create a NEF file
    let nef = NefFile::with_script(script.bytes().to_vec());
    nef.save_to("manual.nef")?;
    
    println!("Manual script created!");
    Ok(())
}
```

### Custom Manifest Creation

```rust
use neo_compiler::manifest::{
    Manifest, 
    ContractMethodDefinition,
    ContractParameterDefinition,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new manifest
    let mut manifest = Manifest::new("CustomToken");
    
    // Add a method
    let method = ContractMethodDefinition::new(
        "transfer".to_string(),
        vec![
            ContractParameterDefinition::new(
                "from".to_string(), 
                "Hash160".to_string()
            ),
            ContractParameterDefinition::new(
                "to".to_string(), 
                "Hash160".to_string()
            ),
            ContractParameterDefinition::new(
                "amount".to_string(), 
                "Integer".to_string()
            ),
        ],
        "Boolean".to_string(),
        false
    );
    manifest.abi.add_method(method);
    
    // Set features
    manifest.set_feature("storage", true)?;
    
    // Save the manifest
    manifest.save_to_file("custom.manifest.json")?;
    
    println!("Custom manifest created!");
    Ok(())
}
```

## Best Practices

When using the neo-compiler, follow these best practices:

1. **Validate WebAssembly**: Ensure your WASM modules are valid and well-formed before passing them to the compiler.

2. **Handle Errors**: Always handle errors properly, as compilation can fail for various reasons.

3. **Set Proper Metadata**: Include accurate metadata in your contracts, such as name, author, and description.

4. **Validate Manifests**: Always validate your manifests before deployment to ensure they meet Neo N3 requirements.

5. **Optimize Scripts**: Use the optimization features to reduce gas costs and improve performance.

## Troubleshooting

Common issues and solutions:

### Invalid WASM Format

If you encounter "Invalid WASM format" errors:
- Ensure your Rust code is compiled with the correct target (`wasm32-unknown-unknown`)
- Validate your WASM file using `wasm-validate` or similar tools
- Check for unsupported Rust features

### Missing Exports

If your contract methods are not appearing in the manifest:
- Ensure your functions are properly exported in the WASM module
- Check that function signatures match expected Neo N3 formats
- Verify that your module has the required metadata

### High Gas Costs

If your contract has high gas costs:
- Enable optimizations during compilation
- Minimize storage operations
- Simplify complex operations
- Reduce contract size

### Checksum Validation Failures

If NEF checksum validation fails:
- Ensure you call `finalize()` on the NEF file before saving
- Avoid manually modifying NEF files after creation
- Verify that the NEF file hasn't been corrupted

## Future Development

The neo-compiler will continue to evolve with improvements in these areas:

1. **Optimization**: Advanced optimization techniques to reduce gas costs.

2. **Type Checking**: Enhanced type checking and validation for safer contracts.

3. **Debugging**: Improved debugging capabilities and error reporting.

4. **Standards Support**: Better support for Neo N3 standards and conventions.