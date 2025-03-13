# Call Flags in Neo N3

This document explains the call flags used in Neo N3 smart contracts and how they affect contract execution.

## Overview

Call flags in Neo N3 are bitwise flags that determine the permissions granted to a contract when called from another contract. They provide a granular security model for contract interactions, allowing callers to restrict what the called contract can do.

## Call Flag Values

Neo N3 defines the following call flags:

| Flag | Value | Description |
|------|-------|-------------|
| `None` | 0 | Default behavior - no permissions granted |
| `ReadOnly` | 1 | The called contract can only read data, not modify state |
| `AllowCall` | 2 | The called contract is allowed to call other contracts |
| `AllowNotify` | 4 | The called contract is allowed to emit notifications |
| `AllowStates` | 8 | The called contract is allowed to read contract states |
| `AllowModifyStates` | 16 | The called contract is allowed to modify contract states |
| `All` | 31 | All permissions are granted (combination of all flags) |

## Using Call Flags in Rust Contracts

In the Neo Contract Rust Framework, call flags are used with the `contract_call` function:

```rust
pub fn contract_call(
    hash: &[u8], 
    method: &[u8], 
    args: &[Any], 
    call_flags: u32
) -> alloc::vec::Vec<u8>
```

Here's how to use the flags:

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod token_contract {
    use neo_contract::prelude::*;
    
    #[method]
    fn call_another_contract(&mut self, target_hash: H160) -> bool {
        // Define method to call
        let method = ByteString::from("totalSupply");
        
        // Create empty arguments array
        let args = Array::<Any>::new();
        
        // Set read-only flag - we only want to read data, not modify state
        let call_flags = CallFlags::READ_ONLY as u32;
        
        // Call the target contract
        let result = contract_call(
            &target_hash,
            &method,
            &args,
            call_flags
        );
        
        // Process result...
        !result.is_empty()
    }
    
    #[method]
    fn perform_complex_operation(&mut self, target_hash: H160, user: H160, amount: u64) -> bool {
        // Define method to call
        let method = ByteString::from("transfer");
        
        // Create arguments
        let mut args = Array::<Any>::new();
        args.push(Any::from(user));
        args.push(Any::from(amount));
        
        // We need multiple permissions for a transfer operation
        let call_flags = (CallFlags::ALLOW_CALL | CallFlags::ALLOW_NOTIFY | CallFlags::ALLOW_MODIFY_STATES) as u32;
        
        // Call the target contract
        let result = contract_call(
            &target_hash,
            &method,
            &args,
            call_flags
        );
        
        // Process result...
        !result.is_empty()
    }
}
```

## Call Flag Combinations

Call flags can be combined using bitwise OR operations. Common combinations include:

1. **Read-Only Operations**:
   ```rust
   let call_flags = CallFlags::READ_ONLY as u32;
   ```

2. **State-Modifying Operations**:
   ```rust
   let call_flags = (CallFlags::ALLOW_MODIFY_STATES | CallFlags::ALLOW_STATES) as u32;
   ```

3. **Full Permission Set**:
   ```rust
   let call_flags = CallFlags::ALL as u32;
   ```

## Security Considerations

When using call flags, consider the following security best practices:

1. **Principle of Least Privilege**: Always grant only the minimum permissions required for the operation
2. **Avoid Using `All` Flag**: Unless absolutely necessary, avoid using the `All` flag as it grants all permissions
3. **Read-Only When Possible**: For operations that only need to read data, use the `ReadOnly` flag
4. **Consider Call Chain**: Remember that permissions granted to a contract also affect what contracts it can call

## Common Patterns and Examples

### 1. Querying Token Balance (Read-Only)

```rust
// We only need read permissions to check a balance
let call_flags = CallFlags::READ_ONLY as u32;
let method = ByteString::from("balanceOf");
let mut args = Array::<Any>::new();
args.push(Any::from(user_address));
let balance = contract_call(&token_hash, &method, &args, call_flags);
```

### 2. Transferring Tokens (State Modification)

```rust
// We need state modification, notification and potentially call permissions
let call_flags = (CallFlags::ALLOW_MODIFY_STATES | CallFlags::ALLOW_NOTIFY | CallFlags::ALLOW_CALL) as u32;
let method = ByteString::from("transfer");
let mut args = Array::<Any>::new();
args.push(Any::from(from_address));
args.push(Any::from(to_address));
args.push(Any::from(amount));
let result = contract_call(&token_hash, &method, &args, call_flags);
```

### 3. Complex DeFi Operation (Full Permissions)

```rust
// For complex operations involving multiple contracts and state changes
let call_flags = CallFlags::ALL as u32;
let method = ByteString::from("swap");
let mut args = Array::<Any>::new();
args.push(Any::from(token_in));
args.push(Any::from(token_out));
args.push(Any::from(amount_in));
args.push(Any::from(min_amount_out));
let result = contract_call(&dex_hash, &method, &args, call_flags);
```

## Relationship with Safe Methods

Call flags have an important relationship with [safe methods](safe_methods.md) in Neo N3:

- When a method is marked as `#[safe]`, it guarantees that it won't modify contract state
- When calling a safe method, you can use the `CallFlags::READ_ONLY` flag for optimal security and gas efficiency
- Attempting to modify state from a method called with `CallFlags::READ_ONLY` will fail

## Technical Implementation

In the Neo VM, call flags are implemented as part of the execution context when creating new execution frames. When a contract calls another contract:

1. The caller specifies the call flags
2. The Neo VM creates a new execution context with the specified restrictions
3. Any operation by the called contract that violates the restrictions will fail with an exception

This provides a strong security boundary between contracts, preventing malicious or buggy contracts from performing unauthorized operations.

## Conclusion

Call flags are a powerful security feature in Neo N3 that allows for fine-grained control over contract interactions. By using appropriate call flags, you can build secure, composable smart contract systems while minimizing attack vectors and unexpected behavior.
