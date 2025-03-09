# NEO NFT Marketplace Smart Contract

A decentralized NFT marketplace built on the Neo N3 blockchain using the neo-contract-rs framework. This contract enables users to list, buy, sell, and auction NFTs with support for royalties and creator verification.

## Features

- **Fixed Price Listings**: List NFTs for sale at a set price
- **Timed Auctions**: Create auctions with automatic extensions and minimum bid increases
- **Offers System**: Allow users to make offers on fixed price listings
- **Royalty Support**: Automatic royalty distribution to NFT creators
- **Collection Verification**: Official verification of NFT collections
- **Fee Structure**: Configurable marketplace fees
- **Escrow System**: Secure holding of NFTs during listings and auctions

## How It Works

### NFT Listing

Users can list their NFTs in two main ways:

#### Fixed Price Listing
1. Owner transfers the NFT to the marketplace contract
2. Sets a fixed price in GAS (or other supported tokens)
3. Optionally sets an expiration time
4. The NFT remains in escrow until purchased or the listing is cancelled

#### Auction Listing
1. Owner transfers the NFT to the marketplace contract
2. Sets a starting price and auction duration
3. Users can place bids with automatic refunds to outbid users
4. Auctions automatically extend if bids are placed near the end
5. When the auction ends, the highest bidder receives the NFT

### Purchasing

1. For fixed price listings, buyers can purchase instantly at the listed price
2. For auctions, the highest bidder when the auction ends wins
3. The marketplace automatically distributes:
   - Payment to the seller (minus fees and royalties)
   - Royalties to the creator (if applicable)
   - Fees to the marketplace

### Offers and Bids

1. Buyers can make offers on fixed price listings
2. Offers include an amount and expiration time
3. Sellers can accept offers, triggering an immediate sale
4. For auctions, the bid system handles automatic price competition

### Royalties

1. The marketplace checks if the NFT supports the royalty standard
2. If supported, it pays the royalty percentage to the specified recipient
3. For collections without built-in royalties, verified collections can have default royalties
4. Royalties are capped at a maximum percentage to prevent excessive fees

## Contract Methods

### Listing Management

- `create_fixed_price_listing`: List an NFT for a fixed price
- `create_auction`: Create an auction for an NFT
- `cancel_listing`: Cancel an active listing (seller only)
- `finalize_auction`: Complete an auction after its end time

### Buying and Bidding

- `buy`: Purchase a fixed price listing
- `place_bid`: Place a bid on an auction
- `make_offer`: Make an offer on a fixed price listing
- `cancel_offer`: Cancel a previously made offer
- `accept_offer`: Accept an offer (seller only)

### Collection Management

- `verify_collection`: Add official verification to a collection (admin only)
- `update_collection`: Update collection verification status (admin only)

### Admin Functions

- `set_fee_percentage`: Update marketplace fee (admin only)
- `set_fee_collector`: Update fee collector address (admin only)
- `set_max_royalty`: Update maximum royalty percentage (admin only)
- `set_auction_parameters`: Update auction parameters (admin only)

### View Methods

- `get_listing`: View detailed listing information
- `get_active_listing_id`: Check if an NFT is currently listed
- `get_highest_bid`: View highest bid for an auction
- `get_offer`: View offer details
- `get_collection`: View collection information
- `get_fee_percentage`: View current marketplace fee
- `get_max_royalty`: View maximum royalty percentage
- `get_auction_parameters`: View auction settings

## Marketplace Parameters

The marketplace includes several configurable parameters:

- **Fee Percentage**: Marketplace fee (default 2.5%)
- **Maximum Royalty**: Cap on creator royalties (default 10%)
- **Auction Duration**: Minimum 1 hour, maximum 30 days
- **Auction Extension**: 5 minutes extension when bids are placed near the end
- **Minimum Bid Increase**: 5% minimum increase over previous bid

## Security Considerations

- **Escrow System**: NFTs are held in the contract until sold or listing is cancelled
- **Signature Verification**: All operations require appropriate signatures
- **Auction Extensions**: Prevents last-second bidding (sniping)
- **Payment Verification**: Ensures all payments are received before transferring NFTs
- **Royalty Caps**: Prevents excessive royalty percentages

## Usage Example

```python
# Python example using neo-python client
from neo3.api import SmartContract

# Contract hash of the deployed marketplace
marketplace_hash = '0x1234567890abcdef1234567890abcdef12345678'
marketplace_contract = SmartContract(marketplace_hash)

# NFT contract hash (NEP-11 compatible)
nft_hash = '0xabcdef1234567890abcdef1234567890abcdef12'

# Create a fixed price listing
wallet.sign_transaction(
    marketplace_contract.create_fixed_price_listing(
        nft_contract=nft_hash,
        token_id=b'token-1',  # NFT token ID
        price=1000000000,     # 10 GAS (assuming 8 decimals)
        payment_token=GAS_TOKEN_HASH,
        expires_at=0          # No expiration
    )
)

# Create an auction
wallet.sign_transaction(
    marketplace_contract.create_auction(
        nft_contract=nft_hash,
        token_id=b'token-2',
        start_price=500000000,  # 5 GAS
        payment_token=GAS_TOKEN_HASH,
        duration=86400         # 24 hours
    )
)

# Buy a fixed price listing
wallet.sign_transaction(
    marketplace_contract.buy(
        listing_id=1
    )
)

# Place a bid on an auction
wallet.sign_transaction(
    marketplace_contract.place_bid(
        listing_id=2,
        amount=600000000  # 6 GAS
    )
)

# Make an offer on a fixed price listing
wallet.sign_transaction(
    marketplace_contract.make_offer(
        listing_id=3,
        amount=800000000,  # 8 GAS
        expires_in=43200   # 12 hours
    )
)

# Accept an offer (seller only)
wallet.sign_transaction(
    marketplace_contract.accept_offer(
        listing_id=3,
        offerer='NbnjKGMBJzJ6j5JwPPBhGGXxTj6qUbueNP'
    )
)
```

## License

This code is provided as an example and is licensed under MIT License.