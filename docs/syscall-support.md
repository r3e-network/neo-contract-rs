# Syscall Support in Neo-Wasm Compiler

This document describes how syscalls are supported in the Neo-Wasm compiler for Neo N3 smart contracts written in Rust.

## Overview

Neo N3 smart contracts can invoke various system calls to interact with the blockchain environment. In Rust contracts, these syscalls are defined in the `neo-contract/src/env/syscall.rs` file and are imported via the `neo.syscall` module.

The Neo-Wasm compiler recognizes these syscall imports and translates them to the appropriate Neo VM syscalls in the NEF bytecode.

## Implementation

### 1. Syscall Definitions

Syscalls are defined in the `syscall.rs` file with a special import module annotation:

```rust
#[link(wasm_import_module = "neo.syscall")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// syscall System.Runtime.Trigger
    pub(crate) fn system_runtime_trigger() -> TriggerType;

    // ... other syscalls ...
}
```

### 2. Compiler Recognition

The Neo-Wasm compiler recognizes imports from the `neo.syscall` module and maps them to the corresponding Neo VM syscalls. The mapping is defined in the `neo-wasm/rosetta/builtin/syscall.go` file.

### 3. Translation Process

When the compiler encounters a syscall import, it:

1. Identifies the import module as `neo.syscall`
2. Looks up the function name in the syscall mapping
3. Generates a `SYSCALL` instruction with the appropriate system call name
4. Adds the instruction to the NEF bytecode

### 4. Supported Syscalls

The compiler supports all Neo N3 syscalls defined in the Neo specification, including:

#### Runtime
- System.Runtime.Trigger
- System.Runtime.Platform
- System.Runtime.GetScriptContainer
- System.Runtime.GetExecutingScriptHash
- System.Runtime.GetCallingScriptHash
- System.Runtime.GetEntryScriptHash
- System.Runtime.GetTime
- System.Runtime.GetInvocationCounter
- System.Runtime.GasLeft
- System.Runtime.GetAddressVersion
- System.Runtime.GetNotifications
- System.Runtime.CheckWitness
- System.Runtime.Log
- System.Runtime.BurnGas
- System.Runtime.GetRandom
- System.Runtime.GetNetwork
- System.Runtime.LoadScript
- System.Runtime.GetCurrentSigners

#### Contract
- System.Contract.Call
- System.Contract.GetCallFlags
- System.Contract.CreateStandardAccount
- System.Contract.CreateMultisigAccount

#### Crypto
- System.Crypto.CheckSig
- System.Crypto.CheckMultisig

#### Iterator
- System.Iterator.Next
- System.Iterator.Value

#### Storage
- System.Storage.GetContext
- System.Storage.GetReadOnlyContext
- System.Storage.AsReadOnly
- System.Storage.Get
- System.Storage.Put
- System.Storage.Delete
- System.Storage.Find

## Usage in Rust Contracts

Rust contracts can invoke syscalls directly from the `env` module:

```rust
use neo_contract::prelude::*;

#[neo::contract]
pub struct MyContract;

impl MyContract {
    #[neo::method]
    pub fn log_message(message: &ByteString) {
        // Invoke the System.Runtime.Log syscall
        env::log(message);
    }
}
```

## Debugging Syscalls

When troubleshooting syscall-related issues:

1. Enable verbose logging with `NEO_WASM_DEBUG=1 NEO_WASM_VERBOSE=1`
2. Check the "Processing syscall import" log messages
3. Verify the syscall name and parameters match the Neo specification

## Notes for Contract Developers

When using syscalls in your contracts:

1. Use the provided wrapper functions in the `env` module when possible
2. Be aware of gas costs associated with different syscalls
3. Check the Neo N3 documentation for up-to-date behavior of each syscall
4. Follow best practices for exception handling as most syscalls may throw exceptions