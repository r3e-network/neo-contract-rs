# Neo Syscall Support

This document provides guidelines on using syscalls in Neo N3 smart contracts written in Rust using the neo-contract-rs framework.

## Overview

Syscalls allow smart contracts to interact with the Neo N3 blockchain, providing access to blockchain state, runtime information, and various services. The neo-contract-rs framework provides idiomatic Rust wrappers around these syscalls, making them easy to use in your smart contracts.

## How to Use Syscalls

### Basic Usage

Simply import the `runtime` module from the `neo_contract` crate and call the appropriate functions:

```rust
use neo_contract::prelude::*;
use neo_contract::runtime;

#[neo::contract]
impl MyContract {
    #[neo::method]
    pub fn get_contract_time() -> u64 {
        // Call the System.Runtime.GetTime syscall
        runtime::get_time()
    }
    
    #[neo::method]
    pub fn log_message(message: &ByteString) {
        // Call the System.Runtime.Log syscall
        runtime::log(message.clone());
    }
}
```

### Available Syscall Wrappers

The neo-contract-rs framework provides wrappers for all Neo N3 syscalls:

#### Runtime

| Rust Function | Neo Syscall | Description |
|---------------|-------------|-------------|
| `runtime::get_trigger()` | System.Runtime.Trigger | Gets the trigger type (Application, Verification, etc.) |
| `runtime::get_platform()` | System.Runtime.Platform | Gets the platform name |
| `runtime::get_tx()` | System.Runtime.GetScriptContainer | Gets the current transaction |
| `runtime::get_executing_script_hash()` | System.Runtime.GetExecutingScriptHash | Gets the script hash of the executing contract |
| `runtime::get_calling_script_hash()` | System.Runtime.GetCallingScriptHash | Gets the script hash of the calling contract |
| `runtime::get_entry_script_hash()` | System.Runtime.GetEntryScriptHash | Gets the script hash of the entry point contract |
| `runtime::get_time()` | System.Runtime.GetTime | Gets the current block timestamp |
| `runtime::get_invocation_counter()` | System.Runtime.GetInvocationCounter | Gets the invocation counter |
| `runtime::get_gas_left()` | System.Runtime.GasLeft | Gets the remaining GAS |
| `runtime::check_witness_with_account()` | System.Runtime.CheckWitness | Verifies if the contract caller is the owner of the account |
| `runtime::log()` | System.Runtime.Log | Logs a message |

#### Storage

| Rust Function | Neo Syscall | Description |
|---------------|-------------|-------------|
| `storage::get_context()` | System.Storage.GetContext | Gets the storage context |
| `storage::get_readonly_context()` | System.Storage.GetReadOnlyContext | Gets a read-only storage context |
| `context.get::<T>(key)` | System.Storage.Get | Gets a value from storage |
| `context.put(key, value)` | System.Storage.Put | Puts a value into storage |
| `context.delete(key)` | System.Storage.Delete | Deletes a value from storage |
| `context.find(prefix)` | System.Storage.Find | Finds entries with a prefix |

## Testing Syscalls

To test syscalls in your contracts:

1. Compile your contract with the neo-wasm compiler
2. Examine the NEF file and manifest to ensure syscalls are properly included
3. Use the Neo debugger to step through syscall execution
4. Use the test-syscall.sh script to validate syscall implementation

### Debugging Tips

If you encounter issues with syscalls:

1. Check the compiler output for warnings or errors
2. Verify that the syscall is properly mapped in the compiler
3. Use verbose logging to trace the syscall translation
4. Check that parameter types match the Neo specification

## Advanced Usage

### Custom Parameter Types

When working with more complex parameter types:

```rust
// Using H160 addresses with CheckWitness
let account = H160::from_bytes(&[/* address bytes */]);
if runtime::check_witness_with_account(account) {
    // Address is authorized
}

// Using storage with custom types
let storage_context = storage::get_context();
let key = ByteString::from("my_key");
let value = MyStruct::serialize();
storage_context.put(key, value);
```

### Contract Calls

To call other contracts:

```rust
let contract_hash = H160::from_hex("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5");
let method = ByteString::from("transfer");
let args = Array::new();
args.push(from);
args.push(to);
args.push(amount);

// Call the contract with ReadStates permission
let result = runtime::load_script(
    contract_hash, 
    CallFlags::READ_STATES, 
    args
);
```

## Performance Considerations

- Some syscalls consume more GAS than others
- Use read-only contexts when possible
- Batch storage operations when feasible
- Use the `get_gas_left()` syscall to monitor GAS consumption

## Security Best Practices

- Always use `check_witness_with_account()` for authorization
- Validate parameters before passing them to syscalls
- Use read-only storage contexts when you only need to read data
- Be careful with contract calls and their permission flags 