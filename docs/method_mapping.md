# Method Mappings in Neo-WASM

## Overview

Method mappings address the discrepancy between method names in Rust code and their exported names in WASM files. This documentation explains how to use method mappings and the various ways to define them.

## The Problem

When compiling Rust contracts to WASM and then to Neo N3 NEF format, methods are exported with specific names in the WASM file. These exported names might differ from the method names in your Rust source code. For example:

- Rust method: `hello`
- WASM export: `add`

This discrepancy causes method offsets to be incorrectly mapped in the manifest, leading to runtime errors when invoking methods.

## Solution: Method Mappings

The Neo-WASM compiler now supports method mappings through three mechanisms:

1. **Source code annotations**
2. **Configuration files** 
3. **Built-in mappings**

### 1. Source Code Annotations

You can annotate your Rust methods with WASM export names:

```rust
/// @method
/// @safe
/// @wasm_export(name = "add")
pub fn hello(name: &ByteString) -> ByteString {
    // Implementation...
}
```

Or using the macro style:

```rust
#[method]
#[safe]
#[wasm_export(name = "add")]
pub fn hello(name: &ByteString) -> ByteString {
    // Implementation...
}
```

### 2. Configuration Files

Create a `neo-contract.yaml` file in your contract directory with method mappings:

```yaml
# Format: wasm_name: rust_name
add: hello
flip: contract_info
option: store_greeting
main: get_greeting
```

### 3. Built-in Mappings

The compiler includes built-in mappings for common contract patterns:

- **hello-world**: Maps WASM exports to standard Rust method names
- More built-in mappings will be added over time

## How Mappings Work

1. During compilation, the compiler identifies WASM export names
2. It calculates offsets for these exports in the NEF file
3. It maps these offsets to the corresponding Rust method names
4. The manifest is updated with the correct method offsets

## Debugging Mappings

If you encounter issues with method mappings:

1. Enable verbose logging with `-verbose` flag
2. Check the neo-wasm log for "Mapped WASM method..." messages
3. Verify the manifest JSON after compilation

## Best Practices

1. **Use Consistent Naming**: When possible, use the same names in Rust and WASM exports
2. **Document Mappings**: Include a `neo-contract.yaml` file in your project
3. **Use Annotations**: Add `@wasm_export` annotations to all methods with different names
4. **Test Invocations**: After compilation, test your contract with all method invocations

## Example

A complete example can be found in the `examples/hello-world` directory, showing both annotation and configuration file approaches. 