# neo-contract-rs Examples Plan

This document outlines a comprehensive plan for developing example contracts to demonstrate the capabilities of the neo-contract-rs framework.

## Purpose of Examples

The example contracts serve multiple purposes:

1. **Demonstration**: Show how to use framework features effectively
2. **Documentation**: Provide working reference implementations
3. **Templates**: Serve as starting points for developers
4. **Testing**: Validate framework functionality in real-world scenarios
5. **Best Practices**: Illustrate recommended patterns and approaches

## Core Examples

### 1. Basic Contract Examples

#### Simple Storage

A minimal contract demonstrating basic storage operations:

```rust
#[neo::contract]
impl SimpleStorage {
    pub fn set(key: ByteString, value: ByteString) {
        let mut storage = StorageMap::new();
        storage.put(key, value);
    }
    
    pub fn get(key: ByteString) -> ByteString {
        let storage = StorageMap::new();
        let value = storage.get(key);
        
        if value.is_null() {
            return ByteString::empty();
        }
        
        value.unwrap()
    }
}
```

#### Hello World

A minimal "Hello World" contract:

```rust
#[neo::contract]
impl HelloWorld {
    pub fn hello(name: ByteString) -> ByteString {
        if name.is_empty() {
            return ByteString::from("Hello, World!");
        }
        
        let mut result = ByteString::from("Hello, ");
        result.concat(&name);
        result.concat(&ByteString::from("!"));
        
        result
    }
}
```

### 2. Token Standards

#### NEP-17 Token (Basic)

Basic fungible token implementation:

```rust
#[neo::contract]
impl Nep17Token for BasicToken {
    fn symbol() -> ByteString {
        ByteString::from("BAS")
    }
    
    fn decimals() -> u32 {
        8
    }
    
    fn _initialize() {
        let owner = runtime::calling_script_hash();
        BasicToken::mint(owner, Int256::from_i32(1_000_000));
    }
}
```

#### NEP-17 Token (Advanced)

Advanced fungible token with additional features:

- Owner management
- Minting/burning controls
- Pausing functionality
- Blacklisting

#### NEP-11 NFT (Non-Divisible)

Non-fungible token implementation:

- Token minting
- Metadata management
- Transfer functionality
- Token enumeration

#### NEP-11 NFT (Divisible)

Divisible NFT implementation for semi-fungible tokens:

- Fractional ownership
- Transfer of fractions
- Metadata management

### 3. Advanced Contract Patterns

#### Upgradable Contract

Demonstrate contract upgrade patterns:

- Proxy contract
- Logic contract
- Data migration

#### Multi-Signature Wallet

Wallet requiring multiple signatures:

- Multi-signature verification
- Transaction proposal and execution
- Signer management

#### Oracle Contract

Contract interacting with external data:

- Data request mechanism
- Callback handling
- Response validation

#### Voting System

Democratic voting implementation:

- Proposal creation
- Vote casting
- Result tallying
- Time-based execution

## Application Examples

### 1. DeFi Applications

#### Simple Exchange

Basic exchange contract:

- Asset pairs
- Order book
- Trade execution

#### Lending Protocol

Simple lending implementation:

- Collateral management
- Interest calculation
- Liquidation mechanism

#### Staking Contract

Token staking with rewards:

- Stake deposit
- Reward distribution
- Time-locked staking

### 2. Gaming Applications

#### Simple Game Assets

Game asset management:

- In-game items as NFTs
- Asset properties
- Trading functionality

#### Game Mechanics

Simple game logic:

- State management
- Random number generation
- Turn-based mechanics

### 3. Business Applications

#### Supply Chain Tracking

Product tracking system:

- Product registration
- Ownership transfer
- History tracking

#### Document Notarization

Document verification system:

- Document hashing
- Timestamp verification
- Signature validation

## Implementation Plan

### Phase 1: Basic Examples (1 month)

1. Simple Storage
2. Hello World
3. Basic NEP-17 Token

### Phase 2: Token Standards (2 months)

1. Advanced NEP-17 Token
2. Non-Divisible NEP-11 NFT
3. Divisible NEP-11 NFT

### Phase 3: Advanced Patterns (2 months)

1. Upgradable Contract
2. Multi-Signature Wallet
3. Oracle Contract
4. Voting System

### Phase 4: Application Examples (3 months)

1. Simple Exchange
2. Lending Protocol
3. Staking Contract
4. Game Assets
5. Supply Chain Tracking
6. Document Notarization

## Documentation Structure

Each example will include:

1. **README.md**: Overview and usage instructions
2. **Architecture.md**: Design decisions and patterns
3. **Source Code**: Well-commented implementation
4. **Tests**: Unit and integration tests
5. **Documentation**: Comprehensive inline documentation
6. **Deployment**: Instructions for deployment and testing

## Testing Strategy

Examples will serve as integration tests for the framework:

1. Unit tests for each component
2. Integration tests for complete functionality
3. Gas usage benchmarks
4. Size optimization examples

## Maintenance and Updates

Examples will be maintained alongside the framework:

1. Regular updates for API changes
2. New examples for new features
3. Optimization as best practices evolve
4. Community-contributed examples integration 