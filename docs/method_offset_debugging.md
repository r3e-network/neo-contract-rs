# Debugging Method Offset Issues

This document provides step-by-step instructions for diagnosing and fixing method offset issues in Neo N3 smart contracts compiled with neo-wasm.

## Common Method Offset Problems

1. **Missing Offsets**: Methods in the manifest have an offset of 0 (default)
2. **Wrong Offsets**: Method offsets don't match actual positions in the NEF script
3. **Methods Not Found**: Exported methods not appearing in the manifest
4. **Name Mapping Issues**: WASM export names don't map correctly to Rust method names

## Diagnostic Steps

### 1. Enable Verbose Logging

Run the compiler with the verbose flag and debug environment variables to see detailed offset calculation information:

```bash
export NEO_WASM_DEBUG=1
export NEO_WASM_VERBOSE=1
./neo-wasm -wasm your_contract.wasm -verbose
```

### 2. Check WASM Exports

Ensure your methods are properly exported in the WASM file. You can use tools like `wasm-objdump`:

```bash
wasm-objdump -x your_contract.wasm | grep "export function"
```

Each method you want in the manifest must be exported from your Rust contract.

### 3. Check Manifest Generation

Check the logs during manifest update to see which methods are being detected and what their offsets are:

```
Method offsets in memory:
  hello: 42
  get_name: 128
  update_data: 256
```

If a method is missing from these logs, it wasn't properly detected during script building.

### 4. Check the Script Building Process

Look for these log messages during script building:

```
Exported methods found in WASM:
  hello (index: 1)
  get_name (index: 2)
  update_data (index: 3)
  
Stored offset for method 'hello': 42
Stored offset for method 'get_name': 128
Stored offset for method 'update_data': 256
```

If a method is listed as exported but doesn't have a "Stored offset" message, there might be an issue with the script building process.

### 5. Check Method Mappings

Verify that method mappings in neo-contract.yaml are correct:

```yaml
# Method mappings for your contract
# Format: wasm_name: rust_name

# Example mappings
main: get_greeting
add: hello
flip: contract_info
```

### 6. Verify Final Manifest

After compilation, check the final manifest JSON file to ensure offsets were correctly updated:

```json
"methods": [
  {
    "name": "hello",
    "parameters": [],
    "offset": 42,
    "return_type": 19,
    "safe": true
  },
  {
    "name": "get_name",
    "parameters": [],
    "offset": 128,
    "return_type": 19,
    "safe": true
  }
]
```

## Common Solutions

### Solution 1: Use the fix-manifest.sh Script

The `fix-manifest.sh` script can repair manifest issues by:
1. Updating metadata from Rust source
2. Verifying method offsets from ASM file
3. Applying method name mappings

```bash
./fix-manifest.sh --manifest path/to/contract.manifest.json --source path/to/lib.rs --asm path/to/contract.neo.asm path/to/project
```

### Solution 2: Manual Offset Extraction

If automatic tools don't work, you can extract offsets from the .neo.asm file:

1. Locate the method in the ASM file:
   ```
   // hello (1):
     000: PUSHINT32 3
     005: PUSHINT32 5
   ```

2. Note the offset (000 in this example)

3. Manually update the manifest:
   ```json
   {
     "name": "hello",
     "offset": 0
   }
   ```

### Solution 3: Fix Method Mapping Issues

If the issue is with method mapping:

1. Create or update your neo-contract.yaml file:
   ```yaml
   # Correct WASM to Rust method mappings
   wasm_method_name: rust_method_name
   ```

2. Recompile with the mapping file in place

### Solution 4: Ensure Proper WASM Exports

In your Rust code, ensure methods are properly exported:

```rust
/// @method
/// @safe
pub fn hello(name: &ByteString) -> ByteString {
    // Method implementation
}
```

## Advanced Troubleshooting

### Debugging WASM Exports

To see all exports in your WASM file:

```bash
wasm-objdump -x your_contract.wasm
```

### Inspecting NEF Script Structure

To view the compiled NEF script structure:

```bash
./neo-wasm translate --input your_contract.wasm --save-neo-ops
cat your_contract.neo.asm
```

### Common Patterns to Look For

1. **Method prefixes**: Some WASM compilers add prefixes to exported function names
2. **Mangled names**: Function names might be mangled with suffixes or type information
3. **Case differences**: WASM exports might use different casing than Rust methods

## Reference Information

### NEF Method Offset Format

In NEO, method offsets are byte positions in the NEF script that point to the start of a method's code.

### Manifest Method Structure

```json
{
  "name": "methodName",     // The method name 
  "parameters": [],         // Parameter list
  "offset": 42,             // Byte offset in NEF script
  "return_type": 19,        // Return type value
  "safe": true              // Whether the method is safe (read-only)
}
```

### Common NEO VM Opcodes

- `0x00-0x10`: Various push operations
- `0x41-0x4B`: Various arithmetic operations 
- `0x51-0x5F`: Various bit operations
- `0x61-0x6F`: Various string operations