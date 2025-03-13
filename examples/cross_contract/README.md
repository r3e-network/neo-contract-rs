# Cross-Contract Communication Examples

This directory contains examples demonstrating various patterns for cross-contract communication on the Neo N3 blockchain.

## Overview

Cross-contract communication is essential for building complex decentralized applications that consist of multiple interacting contracts. These examples showcase different patterns and best practices for implementing secure and efficient contract-to-contract interactions.

## Examples

### Contract Registry Pattern

Located in the [registry](./registry) directory, this example demonstrates:

- A central registry for contract addresses
- Contract discovery and address resolution
- Upgradable contract implementations
- Role-based access control for registry management

The registry pattern allows contracts to look up the addresses of other contracts by name, rather than hardcoding addresses, enabling upgradability and better modularity.

### Proxy Pattern

Located in the [proxy](./proxy) directory, this example demonstrates:

- A stable proxy contract that delegates to changeable implementations
- Transparent proxying of contract calls
- Implementation contract upgrades without changing the user-facing address
- Access-controlled upgrade mechanism

The proxy pattern is used when you need to upgrade the implementation of a contract while maintaining the same address and storage.

## Key Concepts

These examples demonstrate several important concepts in cross-contract communication:

1. **Contract Discovery**: How contracts can find and interact with other contracts
2. **Upgradability**: Techniques for upgrading contract implementations while preserving state and interfaces
3. **Access Control**: Security mechanisms to ensure only authorized entities can manage cross-contract relationships
4. **Event Communication**: Using events for loose coupling between contracts
5. **Call Flags**: Controlling execution context when making cross-contract calls

## Best Practices

When implementing cross-contract communication, follow these best practices:

1. **Validate Inputs**: Always validate inputs when accepting calls from other contracts
2. **Access Control**: Implement proper access control for sensitive operations
3. **Handle Errors**: Properly handle errors from external contract calls
4. **Gas Optimization**: Minimize cross-contract calls to save gas
5. **State Management**: Update state before making external calls to prevent reentrancy attacks
6. **Event Logging**: Emit events to track cross-contract interactions

## Building and Running the Examples

Each example directory contains a detailed README with specific instructions. In general, you can build these examples using:

```bash
# For development and testing
cargo build -p example-name --features std

# For production/deployment
cargo build -p example-name --release
```

## Related Documentation

For more details on cross-contract communication, see:

- [Cross-Contract Communication Guide](../../docs/cross_contract_guide.md)
- [Contract Security Guide](../../docs/contract_security_guide.md)
- [Events Guide](../../docs/events_guide.md)
- [Transaction Patterns Guide](../../docs/transaction_patterns.md)

## License

These examples are provided under the same license as the Neo Contract Rust framework. 