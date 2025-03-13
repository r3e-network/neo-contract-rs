# DAO Example for Neo Smart Contracts

This is a Decentralized Autonomous Organization (DAO) example for Neo N3 blockchain using the Neo Contract Rust framework.

## Overview

This example demonstrates how to create a DAO smart contract on Neo N3 with the following features:

- Proposal creation and management
- Voting mechanism
- Membership management
- Access control
- Time-based proposal lifecycle

## Current Status

**Note:** This example is currently experiencing compilation issues due to ongoing development of the Neo Contract Rust framework.

### Known Issues

1. Procedural macro issues:
   - `#[neo_contract::contract]` macro cannot find `neo_contract_module` in `prelude`
   - `#[method]`, `#[safe]`, and `#[constructor]` macros cannot find required dependencies
   - Issues with the `Storage` trait and `StorageContext`

2. Runtime function signature mismatches:
   - `Runtime::check_witness()` expects a parameter but is used without one
   - Type conversion issues with method calls

### How to Build Neo Contracts

Despite the current issues, here is the correct way to build Neo contracts in this framework:

#### Development Build (with standard library features)

```bash
cargo check -p dao-example --features std
cargo build -p dao-example --features std
```

#### Production Build (for blockchain deployment)

```bash
cargo build -p dao-example --release
```

## DAO Contract Design

The DAO example demonstrates:

1. **Proposal Management**
   - Creating proposals with title, description, and action
   - Tracking proposal status (Active, Passed, Rejected, Executed)
   - Time-based expiration for voting periods

2. **Voting System**
   - One vote per member (could be extended for token-weighted voting)
   - Vote tracking and tallying
   - Automatic status updates based on vote results

3. **Membership Management**
   - Adding/removing members
   - Owner privileges for administrative actions
   - Ownership transfer

4. **Access Control**
   - Only members can create proposals and vote
   - Only the owner can add/remove members

5. **Storage Patterns**
   - Efficient storage mapping for proposals and votes
   - Composite keys for vote tracking
   - Proper serialization and deserialization of complex objects

## Code Structure and Best Practices

When the framework issues are resolved, the code demonstrates:

1. **Proper Codec Implementation**
   - Efficient serialization and deserialization of complex types
   - Error handling for malformed data

2. **Event Emission**
   - Notifying external systems of state changes
   - Providing necessary information in events for off-chain tracking

3. **Access Control**
   - Signature verification with `Runtime::check_witness()`
   - Role-based permissions (owner vs. member)

4. **Error Handling**
   - Proper error propagation
   - Graceful failure with appropriate return values

5. **Storage Optimization**
   - Efficient key design for maps
   - Minimal storage access

## Future Improvements

Once the framework issues are resolved, this example could be extended with:

1. Token-weighted voting
2. More proposal actions
3. Delegation of voting power
4. Proposal discussion threads
5. Multi-signature requirements for critical actions

## License

This example is provided under the same license as the Neo Contract Rust framework. 