# Contract Registry Example

This example demonstrates how to implement a contract registry pattern for Neo N3 smart contracts, enabling centralized contract discovery and upgradability.

## Overview

The Contract Registry pattern provides a centralized location for contract addresses, making it easier to manage and upgrade contract implementations in a decentralized application. This pattern is particularly useful in complex systems where multiple contracts need to interact with each other.

## Key Features

- **Contract Address Management**: Register, update, and query contract addresses by name
- **Role-Based Access Control**: Owner and manager roles with different permissions
- **Secure Ownership Transfer**: Two-step ownership transfer for enhanced security
- **Event Logging**: Detailed events for all important actions
- **Gas-Efficient Storage**: Optimized storage structure for gas efficiency

## Implementation Details

The contract registry is implemented with the following components:

### Storage Structure

```rust
pub struct ContractRegistry {
    // Registry entries
    contracts: StorageMap<String, H160>,
    contracts_list: StorageItem<Vec<String>>,
    
    // Ownership
    owner: StorageItem<H160>,
    pending_owner: StorageItem<H160>,
    
    // Access control
    authorized_managers: StorageMap<H160, bool>,
}
```

### Registry Methods

- `register_contract(name, contract_hash)`: Register a new contract address
- `update_contract(name, contract_hash)`: Update an existing contract address
- `get_contract(name)`: Retrieve a contract address by name
- `list_contracts()`: List all registered contract names
- `contract_count()`: Get the number of registered contracts
- `contains_contract(name)`: Check if a contract exists

### Management Methods

- `add_manager(manager)`: Add an authorized manager
- `remove_manager(manager)`: Remove a manager
- `is_manager(address)`: Check if an address is a manager
- `transfer_ownership(new_owner)`: Begin ownership transfer
- `accept_ownership()`: Complete ownership transfer
- `get_owner()`: Get the current owner
- `get_pending_owner()`: Get the pending owner

## Using the Registry Pattern

### Contract Registration

```rust
// Register contract with the registry
#[method]
pub fn register_system_contracts(&self, registry_contract: H160) -> bool {
    // Define contract names and addresses
    let token_contract = H160::from_slice(&[/* token contract hash */]);
    let nft_contract = H160::from_slice(&[/* NFT contract hash */]);
    
    // Register token contract
    let args = vec![
        RuntimeValue::from("token"),
        RuntimeValue::from(token_contract)
    ];
    Runtime::call_contract(&registry_contract, "register_contract", &args)
        .expect("Failed to register token contract");
    
    // Register NFT contract
    let args = vec![
        RuntimeValue::from("nft"),
        RuntimeValue::from(nft_contract)
    ];
    Runtime::call_contract(&registry_contract, "register_contract", &args)
        .expect("Failed to register NFT contract");
    
    true
}
```

### Contract Discovery

```rust
// Get contract address from registry
#[method]
pub fn interact_with_token(&self, registry_contract: H160, amount: u64) -> bool {
    // Get token contract address from registry
    let args = vec![RuntimeValue::from("token")];
    let token_contract: H160 = Runtime::call_contract(
        &registry_contract, 
        "get_contract", 
        &args
    ).expect("Failed to get token contract address");
    
    // Now interact with the token contract
    let token_args = vec![
        RuntimeValue::from(Runtime::current_sender()),
        RuntimeValue::from(amount)
    ];
    
    Runtime::call_contract(&token_contract, "transfer", &token_args)
        .expect("Token transfer failed")
}
```

### Contract Upgrading

```rust
// Update a contract address
#[method]
pub fn upgrade_contract(&self, registry_contract: H160, name: String, new_hash: H160) -> bool {
    let args = vec![
        RuntimeValue::from(name),
        RuntimeValue::from(new_hash)
    ];
    
    Runtime::call_contract(&registry_contract, "update_contract", &args)
        .expect("Failed to update contract")
}
```

## Security Considerations

1. **Access Control**: Only the owner and authorized managers can register or update contracts
2. **Witness Verification**: All administrative actions require witness verification
3. **Two-Step Ownership Transfer**: Ownership transfers require acceptance by the new owner
4. **Event Logging**: All important actions emit events for traceability and auditing

## Building and Deployment

To build this example:

```bash
# Development build
cargo build -p contract-registry --features std

# Production build
cargo build -p contract-registry --release
```

## Related Documentation

For more details on cross-contract communication, see:

- [Cross-Contract Communication Guide](../../../docs/cross_contract_guide.md)
- [Contract Security Guide](../../../docs/contract_security_guide.md)
- [Events Guide](../../../docs/events_guide.md)

## License

This example is provided under the same license as the Neo Contract Rust framework. 