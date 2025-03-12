# NFT Marketplace for Neo N3

This example demonstrates a comprehensive NFT marketplace implementation for the Neo N3 blockchain using the Neo Contract Rust framework. The marketplace supports fixed-price sales, auctions, offers, royalties, and collection verification.

## Features

- **Multiple Listing Types**: Support for fixed-price listings and timed auctions
- **Bidding System**: Auction functionality with minimum bid increments and time extensions
- **Offer System**: Users can make offers on fixed-price listings
- **Royalty Management**: Configurable royalties for creators with automatic distribution
- **Collection Verification**: System for verifying authentic NFT collections
- **Fee Structure**: Configurable marketplace fees with dedicated collector address
- **Auction Parameters**: Customizable auction durations, extensions, and bid increments
- **Event Notifications**: Comprehensive event system for all marketplace actions

## Contract Structure

### Storage Model

The contract uses the following storage structure:

```rust
struct NFTMarketplace {
    /// Contract owner
    owner: Item<Address>,
    
    /// Next listing ID
    next_listing_id: Item<u64>,
    
    /// Maps listing ID to listing
    listings: Map<u64, Listing>,
    
    /// Maps NFT contract and token ID to active listing ID
    active_listings: Map<(Hash160, ByteArray), u64>,
    
    /// Maps listing ID to highest bid
    highest_bids: Map<u64, Bid>,
    
    /// Maps listing ID and offerer to offer
    offers: Map<(u64, Address), Offer>,
    
    /// Maps NFT contract to verified collection info
    verified_collections: Map<Hash160, VerifiedCollection>,
    
    /// Marketplace fee percentage in basis points (100 = 1%)
    fee_percentage: Item<u16>,
    
    /// Fee collector address
    fee_collector: Item<Address>,
    
    /// Maximum royalty percentage in basis points (1000 = 10%)
    max_royalty: Item<u16>,
    
    /// Maximum auction duration in seconds (30 days)
    max_auction_duration: Item<u64>,
    
    /// Minimum auction duration in seconds (1 hour)
    min_auction_duration: Item<u64>,
    
    /// Auction extension time in seconds if bid placed near end (5 minutes)
    auction_extension_time: Item<u64>,
    
    /// Minimum bid increase percentage in basis points (500 = 5%)
    min_bid_increase: Item<u16>,
}
```

### Key Data Structures

#### Listing

```rust
struct Listing {
    /// Unique listing ID
    id: u64,
    
    /// NFT collection contract hash
    nft_contract: Hash160,
    
    /// NFT token ID
    token_id: ByteArray,
    
    /// Listing owner/seller
    owner: Address,
    
    /// Payment token (GAS or other NEP-17 tokens)
    payment_token: Hash160,
    
    /// Price or minimum bid
    price: u64,
    
    /// Listing type (fixed price or auction)
    listing_type: ListingType,
    
    /// Listing status
    status: ListingStatus,
    
    /// Creation timestamp
    created_at: u64,
    
    /// Expiration timestamp (0 for no expiration in fixed price listings)
    expires_at: u64,
    
    /// Royalty percentage in basis points (100 = 1%)
    royalty_percentage: u16,
    
    /// Royalty recipient address
    royalty_recipient: Address,
}
```

#### Bid

```rust
struct Bid {
    /// Listing ID
    listing_id: u64,
    
    /// Bidder address
    bidder: Address,
    
    /// Bid amount
    amount: u64,
    
    /// Timestamp when bid was placed
    timestamp: u64,
}
```

## Core Functionality

### Fixed Price Listings

Users can list NFTs for a fixed price:

```rust
fn create_fixed_price_listing(
    &mut self,
    nft_contract: Hash160,
    token_id: ByteArray,
    price: u64,
    payment_token: Hash160,
    expires_at: u64,
) -> u64
```

This function:
1. Verifies the caller owns the NFT
2. Checks if the NFT is already listed
3. Transfers the NFT to the marketplace contract
4. Creates a listing with the specified parameters
5. Returns the listing ID

### Auctions

Users can create an auction for an NFT:

```rust
fn create_auction(
    &mut self,
    nft_contract: Hash160,
    token_id: ByteArray,
    start_price: u64,
    payment_token: Hash160,
    duration: u64,
) -> u64
```

This function creates a timed auction with:
- Minimum starting price
- Specified duration (within allowed limits)
- Automatic time extension when bids are placed near the end

### Bidding

Users can place bids on active auctions:

```rust
fn place_bid(&mut self, listing_id: u64, amount: u64) -> bool
```

This function:
1. Verifies the auction is active and hasn't ended
2. Checks that the bid amount meets minimum requirements
3. Refunds the previous highest bidder if present
4. Records the new highest bid
5. Extends the auction time if the bid is placed near the end

### Direct Purchase

Users can buy fixed-price listings:

```rust
fn buy(&mut self, listing_id: u64) -> bool
```

This function:
1. Verifies the listing is active and fixed-price
2. Transfers payment from buyer to marketplace
3. Distributes payment between seller, royalty recipient, and marketplace
4. Transfers the NFT to the buyer
5. Updates the listing status

### Offers

Users can make offers on listings:

```rust
fn make_offer(&mut self, listing_id: u64, amount: u64, expires_in: u64) -> bool
```

Sellers can accept offers:

```rust
fn accept_offer(&mut self, listing_id: u64, offerer: Address) -> bool
```

### Royalty System

The contract automatically handles royalties for creators:

```rust
fn get_royalty_info(&self, nft_contract: &Hash160, token_id: &ByteArray) -> (u16, Address)
```

```rust
fn distribute_payment(
    &self,
    listing_id: u64,
    listing: &Listing,
    buyer: Address,
    amount: u64,
)
```

## Security Considerations

- **Ownership Verification**: All listing and purchase operations verify proper ownership
- **Time-based Constraints**: Auctions and offers have well-defined time periods
- **Payment Handling**: Contract manages transfers between parties, preventing direct transfers
- **Auction Extensions**: Prevents sniping by extending auctions when bids are placed near the end
- **Role-based Access**: Admin functions are restricted to the contract owner

## How to Build

### Development Build

For development and testing:

```bash
cargo check -p neo-nft-marketplace --features std
cargo build -p neo-nft-marketplace --features std
```

### Production Build

For blockchain deployment:

```bash
cargo build -p neo-nft-marketplace --release
```

## Using the Contract

### Deployment

Deploy the contract with initial parameters:

```
deploy neo-nft-marketplace.nef neo-nft-marketplace.manifest.json <owner_address>
```

### Creating a Fixed Price Listing

```
invoke <contract_hash> create_fixed_price_listing <nft_contract_hash> <token_id> 1000000000 <gas_token_hash> 0
```
This creates a listing with:
- NFT from the specified contract and token ID
- Price of 10 GAS (1000000000 = 10 GAS in the smallest unit)
- Payment in GAS tokens
- No expiration (0)

### Creating an Auction

```
invoke <contract_hash> create_auction <nft_contract_hash> <token_id> 500000000 <gas_token_hash> 86400
```
This creates an auction with:
- NFT from the specified contract and token ID
- Starting price of 5 GAS
- Payment in GAS tokens
- Duration of 24 hours (86400 seconds)

### Placing a Bid

```
invoke <contract_hash> place_bid 1 600000000
```
This places a bid of 6 GAS on listing #1.

### Buying a Fixed Price Item

```
invoke <contract_hash> buy 2
```
This purchases the NFT in listing #2.

### Making an Offer

```
invoke <contract_hash> make_offer 3 400000000 86400
```
This makes an offer of 4 GAS on listing #3, valid for 24 hours.

## Events

The contract emits detailed events for all operations:

- `ListingCreated`: When a new listing is created
- `ListingSold`: When a listing is purchased
- `ListingCancelled`: When a listing is cancelled
- `AuctionBid`: When a bid is placed
- `AuctionCompleted`: When an auction is finalized
- `OfferCreated`: When an offer is made
- `OfferAccepted`: When an offer is accepted
- `RoyaltyPaid`: When royalties are paid to creators
- `FeePaid`: When marketplace fees are collected

## Known Issues

1. **Procedural Macro Issues**: The `#[neo_contract::contract]`, `#[method]`, and other macros may not resolve correctly
2. **Storage Trait Issues**: The `Storage` trait implementations might need updates
3. **Runtime Function Signature Mismatches**: Runtime API signatures may change between versions

## License

This example is provided under the same license as the Neo Contract Rust framework.