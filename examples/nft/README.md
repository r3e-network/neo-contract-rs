# Neo N3 NFT Examples

This directory contains examples of Non-Fungible Token (NFT) implementations for the Neo N3 blockchain using the Neo Contract Rust framework.

## Available Examples

### 1. [NFT Marketplace](./marketplace/)

A comprehensive NFT marketplace implementation that demonstrates:
- NFT listing and trading
- Auction mechanisms
- Fixed price sales
- Royalty payments
- Event emission

See the [NFT Marketplace README](./marketplace/README.md) for detailed documentation.

### 2. [Basic NFT](./basic/)

A simple NFT implementation that demonstrates the core functionality of the NEP-11 standard.

### 3. [Storage-Optimized NFT](./storage_optimized/)

A gas-efficient NFT implementation that demonstrates advanced storage patterns:
- Composite key pattern for efficient storage organization
- Lazy loading pattern for on-demand data access
- Pagination pattern for large collections
- Storage prefix pattern for key organization
- Batch operation pattern for reduced gas costs

See the [Storage-Optimized NFT README](./storage_optimized/README.md) for detailed documentation on storage optimization techniques.

## NEP-11 Standard

The NEP-11 standard is Neo's non-fungible token standard, similar to Ethereum's ERC-721. It defines the interface for creating and managing unique tokens on the Neo blockchain.

### Core Methods

- `balanceOf(owner: Address) -> Integer`: Returns the token balance of the specified address
- `ownerOf(tokenId: ByteString) -> Address`: Returns the owner of the specified token
- `tokens() -> Iterator<ByteString>`: Returns an iterator of all token IDs
- `tokensOf(owner: Address) -> Iterator<ByteString>`: Returns an iterator of token IDs owned by the specified address
- `transfer(to: Address, tokenId: ByteString, data: Any) -> Boolean`: Transfers ownership of a token

### Optional Methods

- `properties(tokenId: ByteString) -> Map<String, Any>`: Returns token metadata
- `tokenURI(tokenId: ByteString) -> String`: Returns a URI pointing to token metadata

## Implementation Patterns

### Token Representation

```rust
#[storage]
pub struct NFT {
    // Map of token ID to owner address
    owners: StorageMap<ByteString, H160>,
    
    // Map of owner address to token count
    balances: StorageMap<H160, u64>,
    
    // Map of token ID to token properties
    token_properties: StorageMap<ByteString, TokenProperties>,
    
    // Map of owner address to owned token IDs
    owned_tokens: StorageMap<H160, StorageList<ByteString>>,
}
```

### Token Properties

```rust
#[derive(Serialize, Deserialize)]
pub struct TokenProperties {
    name: String,
    description: String,
    image: String,
    // Additional metadata fields
}
```

### Event Emission

```rust
#[event]
pub struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    #[index]
    token_id: ByteString,
}
```

## Building NFT Contracts

To build an NFT contract:

```bash
# Development build
cargo build -p nft-example --features std

# Production build
cargo build -p nft-example --release
```

## Common NFT Extensions

### Royalties

Royalties allow creators to receive a percentage of sales when their NFTs are resold:

```rust
#[storage]
pub struct NFTWithRoyalties {
    // Basic NFT storage
    // ...
    
    // Map of token ID to royalty information
    royalties: StorageMap<ByteString, RoyaltyInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct RoyaltyInfo {
    beneficiary: H160,
    percentage: u8, // 0-100
}
```

### Metadata Standards

Standardized metadata format for NFTs:

```json
{
  "name": "Asset Name",
  "description": "Asset Description",
  "image": "https://example.com/image.png",
  "properties": {
    "attribute1": "value1",
    "attribute2": "value2"
  }
}
```

## Best Practices

1. **Token ID Generation**: Use deterministic and collision-resistant methods
2. **Access Control**: Implement proper ownership verification
3. **Gas Optimization**: Minimize storage operations
4. **Event Emission**: Emit events for all state changes
5. **Metadata Storage**: Consider on-chain vs. off-chain storage tradeoffs

## Known Issues and Workarounds

As with other examples, you may encounter:

1. **Procedural Macro Issues**: The `#[contract]` and other macros may not resolve correctly
2. **Storage Trait Issues**: The `Storage` trait and `StorageContext` may not be found
3. **Runtime Function Signature Mismatches**: Check for current function signatures in the framework

## License

These examples are provided under the same license as the Neo Contract Rust framework. 