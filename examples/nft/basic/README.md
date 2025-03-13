# Basic NFT Contract for Neo N3

This example demonstrates a basic implementation of the NEP-11 Non-Fungible Token standard for the Neo N3 blockchain using the Neo Contract Rust framework.

## Overview

This contract implements a non-fungible token (NFT) that follows the NEP-11 standard. NFTs are unique digital assets that can represent ownership of digital or physical items on the blockchain.

## Features

- **NEP-11 Compliance**: Implements all required methods of the NEP-11 standard
- **Minting**: Create new NFTs with custom properties
- **Burning**: Destroy NFTs when they're no longer needed
- **Transfers**: Transfer NFTs between addresses
- **Approvals**: Delegate permission to transfer specific NFTs
- **Operator Approvals**: Delegate permission to manage all NFTs owned by an address
- **Metadata**: Store and retrieve token properties and attributes
- **Events**: Emit standard-compliant events for all operations

## Current Status

**Important Note:** This example may experience compilation issues due to ongoing development of the Neo Contract Rust framework. The code is provided as a reference for contract structure and patterns, but may require updates to compile successfully.

## Known Issues

1. **Procedural Macro Issues**: The `#[neo_contract::contract]`, `#[method]`, `#[safe]`, and `#[constructor]` macros may not resolve correctly
2. **Storage Trait Issues**: The `Storage` trait and `StorageContext` may not be found
3. **Runtime Function Signature Mismatches**: Function signatures may change between versions

## How to Build

### Development Build

For development and testing:

```bash
cargo check -p neo-nft-basic --features std
cargo build -p neo-nft-basic --features std
```

### Production Build

For blockchain deployment:

```bash
cargo build -p neo-nft-basic --release
```

## Contract Structure

### Storage Model

The contract uses the following storage structure:

```rust
#[storage]
struct NeoNft {
    // Contract owner
    owner: Item<H160>,
    
    // Token name
    name: Item<String>,
    
    // Token symbol
    symbol: Item<String>,
    
    // Total supply of tokens
    total_supply: Item<u64>,
    
    // Maps token ID to owner address
    owners: Map<ByteString, H160>,
    
    // Maps token ID to token properties
    properties: Map<ByteString, TokenProperties>,
    
    // Maps owner address to token count
    balances: Map<H160, u64>,
    
    // Maps owner address to list of owned token IDs
    tokens_of_owner: Map<H160, Vec<ByteString>>,
    
    // Maps token ID to token index in owner's list
    token_index: Map<ByteString, u64>,
    
    // Maps (owner, operator) to approval status
    operator_approvals: Map<(H160, H160), bool>,
    
    // Maps token ID to approved address
    token_approvals: Map<ByteString, H160>,
}
```

### Token Properties

Each NFT has associated properties stored as a `TokenProperties` struct:

```rust
struct TokenProperties {
    name: String,
    description: String,
    image: String,
    token_uri: String,
    attributes: Vec<TokenAttribute>,
    created_at: u64,
}

struct TokenAttribute {
    trait_type: String,
    value: String,
}
```

### Core Methods

#### Minting

```rust
fn mint(&mut self, to: H160, token_id: ByteString, properties_json: ByteString) -> bool
```

Creates a new NFT with the given token ID and properties, assigning it to the specified address.

#### Burning

```rust
fn burn(&mut self, token_id: ByteString) -> bool
```

Destroys an NFT, removing it from circulation.

#### Transfer

```rust
fn transfer(&mut self, to: H160, token_id: ByteString, data: Option<ByteString>) -> bool
```

Transfers an NFT from the caller to another address.

#### Approvals

```rust
fn approve(&mut self, approved: H160, token_id: ByteString) -> bool
fn set_approval_for_all(&mut self, operator: H160, approved: bool) -> bool
```

Grant or revoke permission for another address to transfer NFTs.

### NEP-11 Standard Methods

The contract implements all required NEP-11 methods:

- `symbol()`: Returns the token symbol
- `name()`: Returns the token name
- `total_supply()`: Returns the total number of tokens
- `balance_of(owner)`: Returns the number of tokens owned by an address
- `owner_of(token_id)`: Returns the owner of a specific token
- `tokens_of(owner)`: Returns all tokens owned by an address
- `properties(token_id)`: Returns the properties of a token
- `get_approved(token_id)`: Returns the approved address for a token
- `is_approved_for_all(owner, operator)`: Checks if an operator is approved for all tokens

## Using the Contract

### Deployment

Deploy the contract with initial parameters:

```
deploy neo-nft-basic.nef neo-nft-basic.manifest.json <owner_address> "My NFT Collection" "MNFT"
```

### Minting NFTs

```
invoke <contract_hash> mint <to_address> <token_id> <properties_json>
```

Example properties JSON:
```json
{
  "name": "Unique Artwork #1",
  "description": "A beautiful digital artwork",
  "image": "https://example.com/image1.png",
  "tokenURI": "https://example.com/metadata/1.json",
  "attributes": [
    {"trait_type": "Artist", "value": "Creator Name"},
    {"trait_type": "Year", "value": "2023"}
  ]
}
```

### Transferring NFTs

```
invoke <contract_hash> transfer <to_address> <token_id> null
```

### Approving Transfers

```
invoke <contract_hash> approve <operator_address> <token_id>
```

### Setting Operator Approval

```
invoke <contract_hash> set_approval_for_all <operator_address> true
```

## Events

The contract emits the following events:

- `Transfer`: When tokens are transferred (including minting and burning)
- `Mint`: When a new token is created
- `Burn`: When a token is destroyed
- `Approval`: When token approval is granted or revoked
- `ApprovalForAll`: When operator approval is granted or revoked

## License

This example is provided under the same license as the Neo Contract Rust framework. 