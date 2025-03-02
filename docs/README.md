# Neo Contract RS Documentation

Welcome to the documentation for Neo Contract RS, a Rust framework for developing Neo N3 smart contracts.

## Getting Started

- [Getting Started Guide](./getting_started.md) - Learn how to set up and start building Neo N3 smart contracts
- [NEP-17 Token Tutorial](./nep17_tutorial.md) - Step-by-step guide to creating a NEP-17 token contract
- [Safe Methods Guide](./safe_methods.md) - Learn how to properly use the `#[safe]` attribute for read-only methods
- [Deployment Guide](./deployment_guide.md) - Learn how to deploy your contracts to the Neo N3 blockchain

## Examples

Check the `examples` directory in the main repository for complete contract examples:

- `nep17_token` - A NEP-17 token implementation
- `contract_call` - Example of contract-to-contract calls
- `ink_style_token` - A token contract using the ink!-style attribute macros
- And more...

## Framework Components

Neo Contract RS is composed of the following major components:

1. **neo-contract** - The core library providing types, runtime functions, and macros for Neo N3 smart contract development
2. **neo-contract-proc-macros** - Procedural macros for simplified contract development

## Resources

- [Neo N3 Documentation](https://docs.neo.org/)
- [NEP-17 Standard](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki)
- [Neo GitHub](https://github.com/neo-project)
