# Contract Interaction in Neo N3

This guide explains how to interact with other contracts and update your own contract using the Neo Contract Rust Framework.

## Overview

In Neo N3, smart contracts can interact with each other through cross-contract calls. This is a powerful feature that enables composability and complex system designs. Additionally, contracts can update themselves through the contract update mechanism.

## Contract Call

The `contract_call` function allows your contract to invoke methods in other contracts. This enables building modular and composable systems.

### Function Signature

```rust
pub fn contract_call(
    hash: &[u8], 
    method: &[u8], 
    args: &[Any], 
    call_flags: u32
) -> alloc::vec::Vec<u8>
```

### Parameters

- `hash`: The script hash of the target contract (20 bytes)
- `method`: The name of the method to call
- `args`: Array of arguments to pass to the method, each converted to the `Any` type
- `call_flags`: Flags that determine the permissions granted to the called contract

### Call Flags

Call flags determine what the called contract is allowed to do:

| Flag | Value | Description |
|------|-------|-------------|
| `None` | 0 | Default behavior |
| `ReadOnly` | 1 | Calls cannot modify state |
| `AllowCall` | 2 | Allows calling other contracts |
| `AllowNotify` | 4 | Allows triggering notifications |
| `AllowStates` | 8 | Allows reading contract states |
| `AllowModifyStates` | 16 | Allows modifying contract states |
| `All` | 31 | All permissions |

### Usage Example

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod token_contract {
    use neo_contract::prelude::*;
    
    // ... other contract code ...
    
    #[method]
    fn transfer_through_proxy(&mut self, token_hash: H160, from: H160, to: H160, amount: u64) -> bool {
        // Create method name
        let method = ByteString::from("transfer");
        
        // Create arguments array
        let mut args = Array::<Any>::new();
        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));
        
        // Set call flags to allow state modification and notifications
        let call_flags = CallFlags::ALL;
        
        // Call the transfer method on the target token contract
        let result = contract_call(
            &token_hash,
            &method,
            &args,
            call_flags as u32
        );
        
        // Parse result
        let success = if result.is_empty() {
            false
        } else {
            // Convert result to boolean
            let parsed: bool = serde_json::from_slice(&result)
                .expect("Failed to parse result");
            parsed
        };
        
        success
    }
}
```

### Best Practices

- **Validate contract hashes**: Always validate contract hashes before making calls
- **Use appropriate call flags**: Only grant permissions that are necessary
- **Handle errors**: Always handle potential failures in contract calls
- **Check return values**: Always validate return values from other contracts
- **Prevent reentrancy attacks**: Be cautious about potential reentrancy vulnerabilities

## Contract Update

The `contract_update` function allows you to update your contract's code and manifest. This is useful for fixing bugs or adding features to deployed contracts.

### Function Signature

```rust
pub fn contract_update(nef_file: &[u8], manifest: &[u8])
```

### Parameters

- `nef_file`: The new Neo Executable Format (NEF) file bytes
- `manifest`: The new contract manifest bytes

### Usage Example

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod upgradable_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct ContractStorage {
        owner: Item<H160>,
    }
    
    #[method]
    fn update_contract(&mut self, nef_file: ByteString, manifest: ByteString) -> bool {
        // Get current caller
        let caller = Runtime::check_witness();
        
        // Check if caller is the contract owner
        let owner = self.owner.get();
        if caller != owner {
            return false;
        }
        
        // Update the contract
        contract_update(&nef_file, &manifest);
        
        // If we reach here, update was successful
        true
    }
}
```

### Security Considerations

- **Access control**: Strictly control who can update the contract
- **Validate NEF and manifest**: Ensure the new code is valid before updating
- **Test thoroughly**: Test the update process thoroughly before deploying
- **Preserve storage schema**: Ensure the new contract is compatible with existing storage
- **Consider using a separate admin contract**: For critical contracts, consider using a separate contract to manage updates

## Technical Details

### How Contract Call Works

When you call another contract:

1. The Neo VM sets up a new execution context for the called contract
2. Arguments are passed to the new context
3. Call flags limit what the called contract can do
4. After execution, control returns to the caller
5. Any return value is passed back to the caller

### How Contract Update Works

When you update a contract:

1. The VM verifies that the caller has permission to update the contract
2. The existing contract's storage is preserved
3. The contract's code and manifest are replaced with the new versions
4. The updated contract continues execution with the new code

## Contract Management 

Neo N3 includes a native ContractManagement contract that provides additional capabilities:

- `deploy`: Deploy a new contract
- `update`: Update an existing contract
- `destroy`: Destroy a contract

To use these functions, you can call the ContractManagement contract using `contract_call`.

## Conclusion

Contract interaction in Neo N3 is a powerful feature that enables complex, modular smart contract systems. By using `contract_call` and `contract_update` effectively, you can build flexible and upgradable applications on the Neo blockchain.
