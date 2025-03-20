# Neo Smart Contract Interoperability Guide

This guide explains how to build interoperable Neo N3 smart contracts using the documentation-first approach.

## Overview

Smart contract interoperability is the ability for contracts to communicate with each other, enabling complex decentralized applications and protocols. In Neo N3, contracts can call methods on other contracts, check contract states, and build composable systems.

## Documentation-First Interoperability Approach

Following our documentation-first philosophy, we recommend:

1. **Document Interoperability Requirements First**: Define how your contract will interact with other contracts before implementation
2. **Create Clear Contract Interfaces**: Document the methods other contracts can call
3. **Document External Contract Dependencies**: Clearly specify which external contracts your contract depends on
4. **Document State Sharing Patterns**: Define how state will be shared between contracts

## Calling Other Contracts

### Basic Contract Call

To call a method on another contract:

```rust
use neo_contract::Runtime;
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::array::Array;
use neo_contract::types::builtin::any::Any;

// Call a method on another contract
pub fn call_external_contract(contract_hash: &H160, method: &str, args: &[Any]) -> Any {
    // Create call flags (default: NONE)
    let flags = 0; // CallFlags::NONE
    
    // Call the contract
    Runtime::call_contract(contract_hash, method, args, flags)
}
```

### Call Flags

When calling other contracts, you can specify call flags to determine the permissions:

| Flag | Value | Description |
|------|-------|-------------|
| `NONE` | 0 | Default, no special permissions |
| `READ_ONLY` | 1 | Cannot modify state |
| `ALLOW_CALL` | 2 | Can call other contracts |
| `ALLOW_NOTIFY` | 4 | Can emit notifications |
| `STATES` | 8 | Can access states of calling contract |
| `ALLOW_ALL` | 7 | READ_ONLY + ALLOW_CALL + ALLOW_NOTIFY |

Example with call flags:

```rust
// Define call flags constants
const CALL_FLAG_NONE: u8 = 0;
const CALL_FLAG_READ_ONLY: u8 = 1;
const CALL_FLAG_ALLOW_CALL: u8 = 2;
const CALL_FLAG_ALLOW_NOTIFY: u8 = 4;
const CALL_FLAG_ALLOW_ALL: u8 = 7;  // READ_ONLY + ALLOW_CALL + ALLOW_NOTIFY

// Call a read-only method on another contract
pub fn call_read_only(contract_hash: &H160, method: &str, args: &[Any]) -> Any {
    Runtime::call_contract(contract_hash, method, args, CALL_FLAG_READ_ONLY)
}
```

### Contract Invocation Structure

When designing your contract to call other contracts, follow this pattern:

```rust
// #[method]
pub fn interact_with_nep17_token(&mut self, token_hash: H160, to: H160, amount: u64) -> bool {
    // Check caller has authorized this operation
    if !Runtime::check_witness(&Runtime::calling_script_hash()) {
        return false;
    }
    
    // Create arguments for the token contract
    let mut args = Array::new();
    args.push(Any::from(Runtime::executing_script_hash())); // from
    args.push(Any::from(to)); // to
    args.push(Any::integer(amount as i64)); // amount
    args.push(Any::null()); // data
    
    // Call transfer on token contract
    let result = Runtime::call_contract(
        &token_hash, 
        "transfer", 
        &args,
        CALL_FLAG_ALLOW_CALL | CALL_FLAG_ALLOW_NOTIFY
    );
    
    // Check result
    result.as_bool().unwrap_or(false)
}
```

## Receiving Calls from Other Contracts

### Designing a Contract Interface

Define clear interfaces for other contracts to use:

```rust
// #[method]
pub fn receive_token_payment(&mut self, from: H160, amount: u64, data: Vec<u8>) -> bool {
    // This is called by NEP-17 tokens when they are sent to this contract
    
    // Validate the caller (should be a token contract)
    let caller = Runtime::calling_script_hash();
    if !self.is_accepted_token(&caller) {
        return false;
    }
    
    // Process the payment
    // ...
    
    true
}

// For NEP-17 compliance
// #[method]
pub fn onNEP17Payment(&mut self, from: H160, amount: i64, data: Vec<u8>) -> bool {
    // Standard method called by NEP-17 tokens
    
    // Convert i64 amount to u64 (verify non-negative first)
    if amount < 0 {
        return false;
    }
    
    self.receive_token_payment(from, amount as u64, data)
}
```

### Validating Callers

When receiving calls, always validate the caller's identity:

```rust
// #[method]
pub fn privileged_operation(&mut self) -> bool {
    // Check if caller is in the list of authorized contracts
    let caller = Runtime::calling_script_hash();
    if !self.authorized_contracts.contains(&caller) {
        return false;
    }
    
    // Perform privileged operation
    // ...
    
    true
}
```

## Contract-to-Contract Interaction Patterns

### 1. Token Exchange Pattern

This pattern shows how to create a contract that exchanges one token for another:

```rust
// #[method]
pub fn swap_tokens(
    &mut self, 
    token_in: H160, 
    token_out: H160, 
    amount_in: u64
) -> bool {
    // 1. Verify the user's identity
    let sender = Runtime::calling_script_hash();
    if !Runtime::check_witness(&sender) {
        return false;
    }
    
    // 2. Calculate exchange amount
    let exchange_rate = self.get_exchange_rate(&token_in, &token_out);
    let amount_out = (amount_in as u128 * exchange_rate as u128 / 10000) as u64;
    
    // 3. Transfer tokens from sender to this contract
    let in_result = self.transfer_token_from_sender(
        &token_in, 
        &sender, 
        &Runtime::executing_script_hash(), 
        amount_in
    );
    if !in_result {
        return false;
    }
    
    // 4. Transfer tokens from this contract to sender
    let out_result = self.transfer_token(
        &token_out, 
        &sender, 
        amount_out
    );
    if !out_result {
        // Handle failure (ideally with a refund, but omitted for brevity)
        return false;
    }
    
    // 5. Emit swap event
    self.emit_swap_event(&sender, &token_in, &token_out, amount_in, amount_out);
    
    true
}

// Helper function to transfer tokens
fn transfer_token_from_sender(
    &self, 
    token: &H160, 
    from: &H160, 
    to: &H160, 
    amount: u64
) -> bool {
    let mut args = Array::new();
    args.push(Any::from(*from));
    args.push(Any::from(*to));
    args.push(Any::integer(amount as i64));
    args.push(Any::null());
    
    let result = Runtime::call_contract(
        token, 
        "transfer", 
        &args,
        CALL_FLAG_ALLOW_CALL | CALL_FLAG_ALLOW_NOTIFY
    );
    
    result.as_bool().unwrap_or(false)
}
```

### 2. Oracle Pattern

This pattern shows how to use an oracle contract to get external data:

```rust
// #[method]
pub fn get_price_data(&self, asset_id: &str) -> i64 {
    // Call oracle contract to get price data
    let oracle_hash = self.oracle_contract.get().unwrap_or(None).unwrap_or_default();
    
    let mut args = Array::new();
    args.push(Any::string(asset_id));
    
    let result = Runtime::call_contract(
        &oracle_hash,
        "getPrice",
        &args,
        CALL_FLAG_READ_ONLY
    );
    
    result.as_i64().unwrap_or(0)
}
```

### 3. Proxy Contract Pattern

This pattern shows how to create upgradeable contracts via a proxy:

```rust
// Proxy contract
// #[method]
pub fn call(&self, method: String, args: Vec<Any>) -> Any {
    // Get implementation contract
    let impl_hash = self.implementation.get().unwrap_or(None).unwrap_or_default();
    
    // Convert args to Array
    let mut call_args = Array::new();
    for arg in args {
        call_args.push(arg);
    }
    
    // Forward call to implementation
    Runtime::call_contract(
        &impl_hash,
        &method,
        &call_args,
        CALL_FLAG_ALLOW_ALL
    )
}

// #[method]
pub fn upgrade_implementation(&mut self, new_impl: H160) -> bool {
    // Check caller is authorized
    let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
    if !Runtime::check_witness(&owner) {
        return false;
    }
    
    // Verify new implementation is valid
    // (would include additional checks in real implementation)
    
    // Update implementation
    self.implementation.set(&new_impl).unwrap_or(());
    
    true
}
```

## Testing Interoperability

### 1. Unit Testing Contract Interactions

To test how your contract interacts with other contracts, mock the external contract:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock external contract calls
    fn mock_call_contract(contract: &H160, method: &str, args: &[Any], flags: u8) -> Any {
        match method {
            "transfer" => {
                // Mock transfer method
                if args.len() < 3 {
                    return Any::boolean(false);
                }
                
                // Extract arguments
                let to = args[1].as_h160().unwrap();
                let amount = args[2].as_i64().unwrap();
                
                // Mock successful transfer
                Any::boolean(true)
            },
            "getPrice" => {
                // Mock oracle price data
                let asset = args[0].as_string();
                match asset {
                    "BTC" => Any::integer(50000),
                    "ETH" => Any::integer(3000),
                    _ => Any::integer(0),
                }
            },
            _ => Any::null(),
        }
    }
    
    // Override Runtime for testing
    impl Runtime {
        pub fn call_contract(contract: &H160, method: &str, args: &Array, flags: u8) -> Any {
            let args_vec: Vec<Any> = args.iter().collect();
            mock_call_contract(contract, method, &args_vec, flags)
        }
    }
    
    #[test]
    fn test_swap_tokens() {
        // Arrange
        let mut contract = setup_test_contract();
        
        // Set up test data
        let token_a = H160::from_hex_string("0x1234567890123456789012345678901234567890").unwrap();
        let token_b = H160::from_hex_string("0x0987654321098765432109876543210987654321").unwrap();
        let sender = H160::from_hex_string("0xabcdef1234567890abcdef1234567890abcdef12").unwrap();
        
        // Add witness
        test_runtime::reset();
        test_runtime::add_witness(sender);
        
        // Act
        let result = contract.swap_tokens(token_a, token_b, 100);
        
        // Assert
        assert!(result);
        // Additional assertions
    }
}
```

### 2. Integration Testing

For integration testing, deploy multiple contracts to a Neo Express instance:

```bash
# Deploy token A
neoxp contract deploy token_a.nef owner

# Deploy token B
neoxp contract deploy token_b.nef owner

# Deploy exchange contract
neoxp contract deploy exchange.nef owner

# Initialize exchange with token hashes
neoxp contract invoke $EXCHANGE_HASH initialize '["$TOKEN_A_HASH","$TOKEN_B_HASH"]' --account owner

# Test a swap
neoxp contract invoke $EXCHANGE_HASH swap_tokens '["$TOKEN_A_HASH","$TOKEN_B_HASH",100]' --account user1
```

## Best Practices for Interoperability

### 1. Error Handling

Always handle errors from external contract calls:

```rust
// Proper error handling
let result = Runtime::call_contract(&token_hash, "transfer", &args, flags);

// Check result type
match result.value_type() {
    1 => { // Boolean
        if !result.as_bool().unwrap_or(false) {
            // Handle transfer failure
            return false;
        }
    },
    0 => { // Null (unexpected)
        return false;
    },
    _ => { // Other types (unexpected)
        return false;
    }
}
```

### 2. Reentrancy Protection

Always protect against reentrancy attacks when calling external contracts:

```rust
// #[method]
// #[no_reentrant]  // Would use an annotation in the future
pub fn withdraw(&mut self, token: H160, amount: u64) -> bool {
    // Check authorization
    let sender = Runtime::calling_script_hash();
    if !Runtime::check_witness(&sender) {
        return false;
    }
    
    // Check balance
    let balance = self.get_user_balance(&sender, &token);
    if balance < amount {
        return false;
    }
    
    // Update state BEFORE external call
    self.set_user_balance(&sender, &token, balance - amount);
    
    // Make external call AFTER state update
    let mut args = Array::new();
    args.push(Any::from(Runtime::executing_script_hash()));
    args.push(Any::from(sender));
    args.push(Any::integer(amount as i64));
    args.push(Any::null());
    
    let result = Runtime::call_contract(
        &token,
        "transfer",
        &args,
        CALL_FLAG_ALLOW_CALL | CALL_FLAG_ALLOW_NOTIFY
    );
    
    if !result.as_bool().unwrap_or(false) {
        // If transfer fails, restore state
        self.set_user_balance(&sender, &token, balance);
        return false;
    }
    
    true
}
```

### 3. Standardization

Follow NEP standards for contract interoperability:

- **NEP-17**: Fungible Token Standard
- **NEP-11**: Non-Fungible Token Standard
- **NEP-5**: Token Sale Standard

Example of implementing standard NEP-17 receiver functionality:

```rust
// Implement standard method required by NEP-17
// #[method]
pub fn onNEP17Payment(&mut self, from: H160, amount: i64, data: Vec<u8>) -> bool {
    // Validate amount
    if amount <= 0 {
        return false;
    }
    
    // Identify the token contract
    let token = Runtime::calling_script_hash();
    
    // Check if this token is accepted
    if !self.is_accepted_token(&token) {
        return false;
    }
    
    // Process deposit
    self.process_deposit(&from, &token, amount as u64, &data);
    
    true
}
```

## Documentation Checklist for Interoperable Contracts

Before implementing interoperability, document:

- [ ] External contracts your contract will call
- [ ] Methods it will call on those contracts
- [ ] Methods other contracts can call on your contract
- [ ] Authentication/authorization requirements
- [ ] Error handling strategy
- [ ] Reentrancy protections
- [ ] State update procedures for cross-contract operations
- [ ] Recovery mechanisms for failed operations

## Conclusion

Following a documentation-first approach to interoperability ensures that your Neo contracts can interact safely and effectively with other contracts. By documenting your contract's external dependencies and interaction patterns before implementation, you establish a clear understanding of the system's architecture and potential security considerations.

For a complete example of interoperability, see [Exchange Example](../examples/exchange), which demonstrates safe token swap interactions between multiple contracts. 