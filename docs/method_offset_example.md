# Method Offset Example: Mapping WASM to Rust Methods

This document provides a clear example of how the neo-wasm compiler properly maps WASM exports to Rust methods while maintaining correct offsets in the manifest file.

## Example Overview

In this example, we'll use the hello-world contract which has:
- WASM exports with names like `add`, `flip`, `option`, and `main`
- Rust methods named `hello`, `contract_info`, `store_greeting`, and `get_greeting`
- A mapping file to connect WASM exports to Rust methods

## Step 1: Rust Contract Code

The Rust contract defines methods with semantic names:

```rust
impl HelloWorld {
    /// @method
    /// @safe
    pub fn hello(name: &ByteString) -> ByteString {
        // Method implementation
    }
    
    /// @method
    /// @safe
    pub fn contract_info() -> ByteString {
        // Method implementation
    }
    
    /// @method
    pub fn store_greeting(name: &ByteString, message: &ByteString) {
        // Method implementation
    }
    
    /// @method
    /// @safe
    pub fn get_greeting(name: &ByteString) -> ByteString {
        // Method implementation
    }
}
```

## Step 2: WASM Compilation

When compiled to WASM, the methods get exported with different names:

```
Exported methods found in WASM:
  add (index: 0)
  flip (index: 1) 
  option (index: 2)
  main (index: 3)
```

## Step 3: Method Name Mapping

We define a mapping in `neo-contract.yaml`:

```yaml
# Method mappings for hello-world contract
# Format: wasm_name: rust_name

# Hello method
add: hello

# Contract info method
flip: contract_info

# Storage methods
option: store_greeting
main: get_greeting 
```

## Step 4: Script Building with Offsets

During NEF script building, the compiler calculates offsets for each method:

```
Stored offset for method 'add': 0
Stored offset for method 'flip': 20
Stored offset for method 'option': 25
Stored offset for method 'main': 38
```

## Step 5: Offset Mapping

The compiler then maps these offsets to the corresponding Rust method names:

```
Mapped WASM method 'add' (offset 0) to Rust method 'hello'
Mapped WASM method 'flip' (offset 20) to Rust method 'contract_info'
Mapped WASM method 'option' (offset 25) to Rust method 'store_greeting'
Mapped WASM method 'main' (offset 38) to Rust method 'get_greeting'
```

## Step 6: Manifest Update

Finally, the manifest is updated with the correct method names and offsets:

```json
{
  "abi": {
    "methods": [
      {
        "name": "hello",
        "parameters": [],
        "offset": 0,
        "return_type": 0,
        "safe": false
      },
      {
        "name": "contract_info",
        "parameters": [],
        "offset": 20,
        "return_type": 0,
        "safe": false
      },
      {
        "name": "store_greeting",
        "parameters": [],
        "offset": 25,
        "return_type": 0,
        "safe": false
      },
      {
        "name": "get_greeting",
        "parameters": [],
        "offset": 38,
        "return_type": 0,
        "safe": false
      }
    ]
  }
}
```

## Verification

To verify that the offsets are correct:

1. The NEF file contains the script with each method's code starting at its specified offset
2. The Neo VM can correctly locate and execute each method using the offset in the manifest
3. When the contract is loaded on the Neo N3 blockchain, method invocations will jump to the correct offset to execute the desired functionality

## Testing

You can test the method offset mapping with:

```bash
./test-mapping.sh
```

This script:
1. Compiles the hello-world contract
2. Runs the neo-wasm translator
3. Checks the manifest for correct method offsets
4. Updates method names according to neo-contract.yaml
5. Verifies the final manifest has both correct names and offsets