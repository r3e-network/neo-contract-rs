//! # NFT Marketplace Smart Contract
//!
//! A decentralized NFT marketplace built on the Neo N3 blockchain using the neo-contract-rs framework.
//! This contract enables users to list, buy, sell, and auction NFTs.
//!
//! This contract implements:
//! - Fixed price listings
//! - Timed auctions with bid system
//! - Offer mechanism
//! - Royalties for NFT creators
//! - Collection verification
//! - Trading fee structure

/// NFT listing types
#[derive(Debug, Clone, Encode, Decode, PartialEq)]
enum ListingType {
    /// Fixed price listing
    FixedPrice,
    /// Auction with end time and minimum bid
    Auction,
}

/// Listing status
#[derive(Debug, Clone, Encode, Decode, PartialEq)]
enum ListingStatus {
    /// Listing is active
    Active,
    /// Listing has been sold
    Sold,
    /// Listing has been cancelled
    Cancelled,
    /// Auction ended without bids
    Expired,
}

/// NFT listing information
#[derive(Debug, Clone, Encode, Decode)]
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

/// Bid information for auctions
#[derive(Debug, Clone, Encode, Decode)]
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

/// Offer information for fixed price listings
#[derive(Debug, Clone, Encode, Decode)]
struct Offer {
    /// Listing ID
    listing_id: u64,
    
    /// Offerer address
    offerer: Address,
    
    /// Offer amount
    amount: u64,
    
    /// Timestamp when offer was made
    created_at: u64,
    
    /// Expiration timestamp
    expires_at: u64,
}

/// Verified collection information
#[derive(Debug, Clone, Encode, Decode)]
struct VerifiedCollection {
    /// NFT collection contract hash
    contract_hash: Hash160,
    
    /// Collection name
    name: ByteArray,
    
    /// Creator address
    creator: Address,
    
    /// Default royalty percentage in basis points (100 = 1%)
    default_royalty: u16,
    
    /// Verification status
    verified: bool,
}

/// Events emitted by the marketplace
#[neo_contract::event]
pub struct ListingCreated {
    #[index]
    pub listing_id: u64,
    #[index]
    pub seller: Address,
    pub nft_contract: Hash160,
    pub token_id: ByteArray,
    pub price: u64,
    pub payment_token: Hash160,
    pub listing_type: u8,
    pub expires_at: u64,
}

#[neo_contract::event]
pub struct ListingCancelled {
    #[index]
    pub listing_id: u64,
    #[index]
    pub seller: Address,
}

#[neo_contract::event]
pub struct ListingSold {
    #[index]
    pub listing_id: u64,
    #[index]
    pub seller: Address,
    #[index]
    pub buyer: Address,
    pub nft_contract: Hash160,
    pub token_id: ByteArray,
    pub price: u64,
    pub payment_token: Hash160,
}

#[neo_contract::event]
pub struct AuctionBid {
    #[index]
    pub listing_id: u64,
    #[index]
    pub bidder: Address,
    pub amount: u64,
}

#[neo_contract::event]
pub struct AuctionCompleted {
    #[index]
    pub listing_id: u64,
    #[index]
    pub seller: Address,
    #[index]
    pub winner: Address,
    pub nft_contract: Hash160,
    pub token_id: ByteArray,
    pub final_price: u64,
    pub payment_token: Hash160,
}

#[neo_contract::event]
pub struct OfferCreated {
    #[index]
    pub listing_id: u64,
    #[index]
    pub offerer: Address,
    pub amount: u64,
    pub expires_at: u64,
}

#[neo_contract::event]
pub struct OfferCancelled {
    #[index]
    pub listing_id: u64,
    #[index]
    pub offerer: Address,
}

#[neo_contract::event]
pub struct OfferAccepted {
    #[index]
    pub listing_id: u64,
    #[index]
    pub seller: Address,
    #[index]
    pub offerer: Address,
    pub price: u64,
}

#[neo_contract::event]
pub struct CollectionVerified {
    #[index]
    pub nft_contract: Hash160,
    pub name: ByteArray,
    pub creator: Address,
    pub royalty: u16,
}

#[neo_contract::event]
pub struct CollectionUpdated {
    #[index]
    pub nft_contract: Hash160,
    pub royalty: u16,
    pub verified: bool,
}

#[neo_contract::event]
pub struct RoyaltyPaid {
    #[index]
    pub nft_contract: Hash160,
    #[index]
    pub token_id: ByteArray,
    pub recipient: Address,
    pub amount: u64,
}

#[neo_contract::event]
pub struct FeePaid {
    pub listing_id: u64,
    pub collector: Address,
    pub amount: u64,
}

// Constants
const GAS_TOKEN: Hash160 = Hash160::from_script_hash([0xd2, 0xa4, 0xcf, 0xf3, 0x19, 0x13, 0x01, 0x61, 0x55, 0xe3, 0x8e, 0x47, 0x4a, 0x2c, 0x06, 0xd0, 0x8b, 0xe2, 0x76, 0xcf]); // GAS token hash
const BASIS_POINTS: u16 = 10000; // 100% in basis points

/// Marketplace storage
#[neo_contract::contract]
pub struct NFTMarketplace {
    /// Contract owner
    #[storage]
    owner: StorageItem<Address>,
    
    /// Next listing ID
    #[storage]
    next_listing_id: StorageItem<u64>,
    
    /// Maps listing ID to listing
    #[storage]
    listings: StorageMap<u64, Listing>,
    
    /// Maps NFT contract and token ID to active listing ID
    #[storage]
    active_listings: StorageMap<(Hash160, ByteArray), u64>,
    
    /// Maps listing ID to highest bid
    #[storage]
    highest_bids: StorageMap<u64, Bid>,
    
    /// Maps listing ID and offerer to offer
    #[storage]
    offers: StorageMap<(u64, Address), Offer>,
    
    /// Maps NFT contract to verified collection info
    #[storage]
    verified_collections: StorageMap<Hash160, VerifiedCollection>,
    
    /// Marketplace fee percentage in basis points (100 = 1%)
    #[storage]
    fee_percentage: StorageItem<u16>,
    
    /// Fee collector address
    #[storage]
    fee_collector: StorageItem<Address>,
    
    /// Maximum royalty percentage in basis points (1000 = 10%)
    #[storage]
    max_royalty: StorageItem<u16>,
    
    /// Maximum auction duration in seconds (30 days)
    #[storage]
    max_auction_duration: StorageItem<u64>,
    
    /// Minimum auction duration in seconds (1 hour)
    #[storage]
    min_auction_duration: StorageItem<u64>,
    
    /// Auction extension time in seconds if bid placed near end (5 minutes)
    #[storage]
    auction_extension_time: StorageItem<u64>,
    
    /// Minimum bid increase percentage in basis points (500 = 5%)
    #[storage]
    min_bid_increase: StorageItem<u16>,
}

impl NFTMarketplace {
    /// Initialize the marketplace contract
    #[constructor]
    pub fn new(owner: Address) -> Self {
        Self {
            owner: StorageItem::new(b"owner"),
            next_listing_id: StorageItem::new(b"next_listing_id"),
            listings: StorageMap::new(b"listings"),
            active_listings: StorageMap::new(b"active_listings"),
            highest_bids: StorageMap::new(b"highest_bids"),
            offers: StorageMap::new(b"offers"),
            verified_collections: StorageMap::new(b"verified_collections"),
            fee_percentage: StorageItem::new(b"fee_percentage"),
            fee_collector: StorageItem::new(b"fee_collector"),
            max_royalty: StorageItem::new(b"max_royalty"),
            max_auction_duration: StorageItem::new(b"max_auction_duration"),
            min_auction_duration: StorageItem::new(b"min_auction_duration"),
            auction_extension_time: StorageItem::new(b"auction_extension_time"),
            min_bid_increase: StorageItem::new(b"min_bid_increase"),
        }
    }
    
    /// Create a fixed price listing
    #[method]
    pub fn create_fixed_price_listing(
        &mut self,
        nft_contract: Hash160,
        token_id: ByteArray,
        price: u64,
        payment_token: Hash160,
        expires_at: u64,
    ) -> u64 {
        let seller = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&seller), "Invalid signature");
        
        // Validate inputs
        assert!(price > 0, "Price must be greater than 0");
        self.validate_payment_token(&payment_token);
        
        // Check if expiration is in the future
        if expires_at > 0 {
            assert!(expires_at > Ledger::current_timestamp(), "Expiration must be in the future");
        }
        
        // Verify NFT ownership
        let owner: Address = self.call_contract(
            &nft_contract,
            "ownerOf",
            (token_id.clone(),)
        ).expect("Failed to get NFT owner");
        
        assert!(owner == seller, "Seller must own the NFT");
        
        // Check if NFT is already listed
        let key = (nft_contract, token_id.clone());
        assert!(self.active_listings.get(&key).is_none(), "NFT already listed");
        
        // Get royalty information
        let (royalty_percentage, royalty_recipient) = self.get_royalty_info(&nft_contract, &token_id);
        
        // Generate listing ID
        let listing_id = *self.next_listing_id.get();
        self.next_listing_id.set(&(listing_id + 1));
        
        // Create listing
        let listing = Listing {
            id: listing_id,
            nft_contract,
            token_id: token_id.clone(),
            owner: seller,
            payment_token,
            price,
            listing_type: ListingType::FixedPrice,
            status: ListingStatus::Active,
            created_at: Ledger::current_timestamp(),
            expires_at,
            royalty_percentage,
            royalty_recipient,
        };
        
        // Store listing
        self.listings.insert(listing_id, listing);
        self.active_listings.insert(key, listing_id);
        
        // Transfer NFT to marketplace contract
        let transferred: bool = self.call_contract(
            &nft_contract,
            "transferFrom",
            (seller, runtime::executing_script_hash(), token_id.clone())
        ).expect("Failed to transfer NFT");
        
        assert!(transferred, "NFT transfer failed");
        
        // Emit event
        ListingCreated {
            listing_id,
            seller,
            nft_contract,
            token_id,
            price,
            payment_token,
            listing_type: 0, // listing_type as u8
            expires_at
        }.notify();
        
        listing_id
    }
    
    /// Create an auction listing
    #[method]
    pub fn create_auction(
        &mut self,
        nft_contract: Hash160,
        token_id: ByteArray,
        start_price: u64,
        payment_token: Hash160,
        duration: u64,
    ) -> u64 {
        let seller = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&seller), "Invalid signature");
        
        // Validate inputs
        assert!(start_price > 0, "Starting price must be greater than 0");
        self.validate_payment_token(&payment_token);
        
        // Validate auction duration
        let min_duration = *self.min_auction_duration.get();
        let max_duration = *self.max_auction_duration.get();
        assert!(duration >= min_duration, "Auction duration too short");
        assert!(duration <= max_duration, "Auction duration too long");
        
        // Calculate expiration time
        let expires_at = Ledger::current_timestamp() + duration;
        
        // Verify NFT ownership
        let owner: Address = self.call_contract(
            &nft_contract,
            "ownerOf",
            (token_id.clone(),)
        ).expect("Failed to get NFT owner");
        
        assert!(owner == seller, "Seller must own the NFT");
        
        // Check if NFT is already listed
        let key = (nft_contract, token_id.clone());
        assert!(self.active_listings.get(&key).is_none(), "NFT already listed");
        
        // Get royalty information
        let (royalty_percentage, royalty_recipient) = self.get_royalty_info(&nft_contract, &token_id);
        
        // Generate listing ID
        let listing_id = *self.next_listing_id.get();
        self.next_listing_id.set(&(listing_id + 1));
        
        // Create listing
        let listing = Listing {
            id: listing_id,
            nft_contract,
            token_id: token_id.clone(),
            owner: seller,
            payment_token,
            price: start_price, // Starting price
            listing_type: ListingType::Auction,
            status: ListingStatus::Active,
            created_at: Ledger::current_timestamp(),
            expires_at,
            royalty_percentage,
            royalty_recipient,
        };
        
        // Store listing
        self.listings.insert(listing_id, listing);
        self.active_listings.insert(key, listing_id);
        
        // Transfer NFT to marketplace contract
        let transferred: bool = self.call_contract(
            &nft_contract,
            "transferFrom",
            (seller, runtime::executing_script_hash(), token_id.clone())
        ).expect("Failed to transfer NFT");
        
        assert!(transferred, "NFT transfer failed");
        
        // Emit event
        ListingCreated {
            listing_id,
            seller,
            nft_contract,
            token_id,
            price: start_price,
            payment_token,
            listing_type: 1, // listing_type as u8
            expires_at
        }.notify();
        
        listing_id
    }

    /// Buy a fixed price listing
    #[method]
    pub fn buy(&mut self, listing_id: u64) -> bool {
        let buyer = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&buyer), "Invalid signature");
        
        // Get listing
        let mut listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Validate listing
        assert!(listing.status == ListingStatus::Active, "Listing is not active");
        assert!(listing.listing_type == ListingType::FixedPrice, "Listing is not fixed price");
        
        // Check if listing has expired
        if listing.expires_at > 0 && listing.expires_at <= Ledger::current_timestamp() {
            // Update listing status to expired
            listing.status = ListingStatus::Expired;
            self.listings.insert(listing_id, listing);
            
            // Remove from active listings
            self.active_listings.remove(&(listing.nft_contract, listing.token_id.clone()));
            
            return false;
        }
        
        // Transfer payment from buyer to contract
        let transferred: bool = self.call_contract(
            &listing.payment_token,
            "transfer",
            (buyer, runtime::executing_script_hash(), listing.price, ByteArray::new())
        ).expect("Payment transfer failed");
        
        assert!(transferred, "Payment failed");
        
        // Distribute payment
        self.distribute_payment(
            listing_id,
            &listing,
            buyer,
            listing.price,
        );
        
        // Transfer NFT to buyer
        let nft_transferred: bool = self.call_contract(
            &listing.nft_contract,
            "transfer",
            (runtime::executing_script_hash(), buyer, listing.token_id.clone(), ByteArray::new())
        ).expect("NFT transfer failed");
        
        assert!(nft_transferred, "NFT transfer failed");
        
        // Update listing status
        listing.status = ListingStatus::Sold;
        self.listings.insert(listing_id, listing.clone());
        
        // Remove from active listings
        self.active_listings.remove(&(listing.nft_contract, listing.token_id.clone()));
        
        // Emit event
        ListingSold {
            listing_id,
            seller: listing.owner,
            buyer,
            nft_contract: listing.nft_contract,
            token_id: listing.token_id,
            price: listing.price,
            payment_token: listing.payment_token,
        }.notify();
        
        true
    }
    
    /// Place a bid on an auction
    #[method]
    pub fn place_bid(&mut self, listing_id: u64, amount: u64) -> bool {
        let bidder = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&bidder), "Invalid signature");
        
        // Get listing
        let mut listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Validate listing
        assert!(listing.status == ListingStatus::Active, "Auction is not active");
        assert!(listing.listing_type == ListingType::Auction, "Listing is not an auction");
        
        // Check if auction has ended
        assert!(listing.expires_at > Ledger::current_timestamp(), "Auction has ended");
        
        // Check bid amount
        assert!(amount >= listing.price, "Bid amount below starting price");
        
        // Check for existing bids
        if let Some(highest_bid) = self.highest_bids.get(&listing_id) {
            // Calculate minimum bid
            let min_increase = *self.min_bid_increase.get();
            let min_bid_amount = highest_bid.amount + (highest_bid.amount * min_increase as u64) / BASIS_POINTS as u64;
            
            assert!(amount >= min_bid_amount, "Bid too low");
            
            // Refund previous highest bidder
            let refunded: bool = self.call_contract(
                &listing.payment_token,
                "transfer",
                (runtime::executing_script_hash(), highest_bid.bidder, highest_bid.amount, ByteArray::new())
            ).expect("Refund failed");
            
            assert!(refunded, "Failed to refund previous bidder");
        }
        
        // Transfer bid amount from bidder to contract
        let transferred: bool = self.call_contract(
            &listing.payment_token,
            "transfer",
            (bidder, runtime::executing_script_hash(), amount, ByteArray::new())
        ).expect("Bid transfer failed");
        
        assert!(transferred, "Bid payment failed");
        
        // Create new bid
        let bid = Bid {
            listing_id,
            bidder,
            amount,
            timestamp: Ledger::current_timestamp(),
        };
        
        // Store bid as highest
        self.highest_bids.insert(listing_id, bid);
        
        // Extend auction if bid is placed near the end
        let extension_time = *self.auction_extension_time.get();
        let time_left = listing.expires_at - Ledger::current_timestamp();
        
        if time_left < extension_time {
            listing.expires_at = Ledger::current_timestamp() + extension_time;
            self.listings.insert(listing_id, listing);
        }
        
        // Emit event
        AuctionBid {
            listing_id,
            bidder,
            amount
        }.notify();
        
        true
    }
    
    /// Finalize an auction after it has ended
    #[method]
    pub fn finalize_auction(&mut self, listing_id: u64) -> bool {
        // Get listing
        let mut listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Validate listing
        assert!(listing.status == ListingStatus::Active, "Auction is not active");
        assert!(listing.listing_type == ListingType::Auction, "Listing is not an auction");
        assert!(listing.expires_at <= Ledger::current_timestamp(), "Auction has not ended yet");
        
        // Check if there was a winning bid
        if let Some(winning_bid) = self.highest_bids.get(&listing_id) {
            // Transfer NFT to winner
            let nft_transferred: bool = self.call_contract(
                &listing.nft_contract,
                "transfer",
                (runtime::executing_script_hash(), winning_bid.bidder, listing.token_id.clone(), ByteArray::new())
            ).expect("NFT transfer failed");
            
            assert!(nft_transferred, "NFT transfer failed");
            
            // Distribute payment
            self.distribute_payment(
                listing_id,
                &listing,
                winning_bid.bidder,
                winning_bid.amount,
            );
            
            // Update listing status
            listing.status = ListingStatus::Sold;
            self.listings.insert(listing_id, listing.clone());
            
            // Remove from active listings
            self.active_listings.remove(&(listing.nft_contract, listing.token_id.clone()));
            
            // Emit event
            AuctionCompleted {
                listing_id,
                seller: listing.owner,
                winner: winning_bid.bidder,
                nft_contract: listing.nft_contract,
                token_id: listing.token_id,
                final_price: winning_bid.amount,
                payment_token: listing.payment_token,
            }.notify();
            
            return true;
        } else {
            // No bids were placed, return NFT to owner
            let nft_returned: bool = self.call_contract(
                &listing.nft_contract,
                "transfer",
                (runtime::executing_script_hash(), listing.owner, listing.token_id.clone(), ByteArray::new())
            ).expect("NFT return failed");
            
            assert!(nft_returned, "Failed to return NFT to owner");
            
            // Update listing status
            listing.status = ListingStatus::Expired;
            self.listings.insert(listing_id, listing.clone());
            
            // Remove from active listings
            self.active_listings.remove(&(listing.nft_contract, listing.token_id.clone()));
            
            return false;
        }
    }
    
    /// Cancel a listing (seller only)
    #[method]
    pub fn cancel_listing(&mut self, listing_id: u64) -> bool {
        let caller = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&caller), "Invalid signature");
        
        // Get listing
        let mut listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Verify caller is the listing owner
        assert!(listing.owner == caller, "Only the listing owner can cancel");
        
        // Verify listing is active
        assert!(listing.status == ListingStatus::Active, "Listing is not active");
        
        // If it's an auction with bids, don't allow cancellation
        if listing.listing_type == ListingType::Auction {
            if let Some(_) = self.highest_bids.get(&listing_id) {
                return false;
            }
        }
        
        // Return NFT to owner
        let nft_returned: bool = self.call_contract(
            &listing.nft_contract,
            "transfer",
            (runtime::executing_script_hash(), listing.owner, listing.token_id.clone(), ByteArray::new())
        ).expect("NFT return failed");
        
        assert!(nft_returned, "Failed to return NFT to owner");
        
        // Update listing status
        listing.status = ListingStatus::Cancelled;
        self.listings.insert(listing_id, listing.clone());
        
        // Remove from active listings
        self.active_listings.remove(&(listing.nft_contract, listing.token_id.clone()));
        
        // Emit event
        ListingCancelled {
            listing_id,
            seller: listing.owner,
        }.notify();
        
        true
    }
    
    /// Make an offer on a fixed price listing
    #[method]
    pub fn make_offer(&mut self, listing_id: u64, amount: u64, expires_in: u64) -> bool {
        let offerer = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&offerer), "Invalid signature");
        
        // Get listing
        let listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Validate listing
        assert!(listing.status == ListingStatus::Active, "Listing is not active");
        assert!(listing.listing_type == ListingType::FixedPrice, "Cannot make offer on auction");
        
        // Validate offer
        assert!(amount > 0, "Offer amount must be greater than 0");
        assert!(expires_in > 0, "Offer must have an expiration time");
        
        // Calculate expiration time
        let expires_at = Ledger::current_timestamp() + expires_in;
        
        // Transfer offer amount to contract
        let transferred: bool = self.call_contract(
            &listing.payment_token,
            "transfer",
            (offerer, runtime::executing_script_hash(), amount, ByteArray::new())
        ).expect("Offer payment failed");
        
        assert!(transferred, "Failed to transfer offer payment");
        
        // Create offer
        let offer = Offer {
            listing_id,
            offerer,
            amount,
            created_at: Ledger::current_timestamp(),
            expires_at,
        };
        
        // Store offer
        let offer_key = (listing_id, offerer);
        
        // If offerer already has an offer, refund the previous one
        if let Some(previous_offer) = self.offers.get(&offer_key) {
            let refunded: bool = self.call_contract(
                &listing.payment_token,
                "transfer",
                (runtime::executing_script_hash(), offerer, previous_offer.amount, ByteArray::new())
            ).expect("Refund failed");
            
            assert!(refunded, "Failed to refund previous offer");
        }
        
        self.offers.insert(offer_key, offer);
        
        // Emit event
        OfferCreated {
            listing_id,
            offerer,
            amount,
            expires_at
        }.notify();
        
        true
    }
    
    /// Cancel an offer
    #[method]
    pub fn cancel_offer(&mut self, listing_id: u64) -> bool {
        let offerer = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&offerer), "Invalid signature");
        
        // Get listing
        let listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Get offer
        let offer_key = (listing_id, offerer);
        let offer = self.offers.get(&offer_key).expect("Offer not found");
        
        // Refund offer amount
        let refunded: bool = self.call_contract(
            &listing.payment_token,
            "transfer",
            (runtime::executing_script_hash(), offerer, offer.amount, ByteArray::new())
        ).expect("Refund failed");
        
        assert!(refunded, "Failed to refund offer");
        
        // Remove offer
        self.offers.remove(&offer_key);
        
        // Emit event
        OfferCancelled {
            listing_id,
            offerer,
        }.notify();
        
        true
    }
    
    /// Accept an offer (seller only)
    #[method]
    pub fn accept_offer(&mut self, listing_id: u64, offerer: Address) -> bool {
        let seller = runtime::calling_script_hash();
        
        // Verify signature
        assert!(runtime::check_witness(&seller), "Invalid signature");
        
        // Get listing
        let mut listing = self.listings.get(&listing_id).expect("Listing not found");
        
        // Verify caller is the listing owner
        assert!(listing.owner == seller, "Only the listing owner can accept offers");
        
        // Verify listing is active
        assert!(listing.status == ListingStatus::Active, "Listing is not active");
        
        // Get offer
        let offer_key = (listing_id, offerer);
        let offer = self.offers.get(&offer_key).expect("Offer not found");
        
        // Verify offer hasn't expired
        assert!(offer.expires_at > Ledger::current_timestamp(), "Offer has expired");
        
        // Transfer NFT to offerer
        let nft_transferred: bool = self.call_contract(
            &listing.nft_contract,
            "transfer",
            (runtime::executing_script_hash(), offerer, listing.token_id.clone(), ByteArray::new())
        ).expect("NFT transfer failed");
        
        assert!(nft_transferred, "NFT transfer failed");
        
        // Distribute payment
        self.distribute_payment(
            listing_id,
            &listing,
            offerer,
            offer.amount,
        );
        
        // Update listing status
        listing.status = ListingStatus::Sold;
        self.listings.insert(listing_id, listing.clone());
        
        // Remove from active listings
        self.active_listings.remove(&(listing.nft_contract, listing.token_id.clone()));
        
        // Remove the accepted offer
        self.offers.remove(&offer_key);
        
        // Refund all other offers for this listing
        // Note: In a production contract, you might want to handle this more efficiently
        // by keeping a list of offers per listing
        
        // Emit event
        OfferAccepted {
            listing_id,
            seller,
            offerer,
            price: offer.amount
        }.notify();
        
        true
    }
    
    /// Verify a collection (admin only)
    #[method]
    pub fn verify_collection(&mut self, nft_contract: Hash160, name: ByteArray, creator: Address, default_royalty: u16) -> bool {
        let admin = runtime::calling_script_hash();
        
        // Verify admin signature
        assert!(admin == *self.owner.get(), "Only admin can verify collections");
        
        // Validate royalty
        let max_royalty = *self.max_royalty.get();
        assert!(default_royalty <= max_royalty, "Royalty exceeds maximum");
        
        // Create collection info
        let collection = VerifiedCollection {
            contract_hash: nft_contract,
            name: name.clone(),
            creator,
            default_royalty,
            verified: true,
        };
        
        // Store collection
        self.verified_collections.insert(nft_contract, collection);
        
        // Emit event
        CollectionVerified {
            nft_contract,
            name,
            creator,
            royalty: default_royalty,
        }.notify();
        
        true
    }
    
    /// Update collection verification status (admin only)
    #[method]
    pub fn update_collection(&mut self, nft_contract: Hash160, default_royalty: u16, verified: bool) -> bool {
        let admin = runtime::calling_script_hash();
        
        // Verify admin signature
        assert!(admin == *self.owner.get(), "Only admin can update collections");
        
        // Get collection
        let mut collection = self.verified_collections.get(&nft_contract).expect("Collection not found");
        
        // Validate royalty
        let max_royalty = *self.max_royalty.get();
        assert!(default_royalty <= max_royalty, "Royalty exceeds maximum");
        
        // Update collection
        collection.default_royalty = default_royalty;
        collection.verified = verified;
        
        // Store updated collection
        self.verified_collections.insert(nft_contract, collection);
        
        // Emit event
        CollectionUpdated {
            nft_contract,
            royalty: default_royalty,
            verified,
        }.notify();
        
        true
    }
    
    /// Set marketplace fee (admin only)
    #[method]
    pub fn set_fee_percentage(&mut self, fee_percentage: u16) -> bool {
        let admin = runtime::calling_script_hash();
        
        // Verify admin signature
        assert!(admin == *self.owner.get(), "Only admin can set fees");
        
        // Validate fee (max 10%)
        assert!(fee_percentage <= 1000, "Fee percentage too high");
        
        // Update fee
        self.fee_percentage.set(&fee_percentage);
        
        true
    }
    
    /// Set fee collector (admin only)
    #[method]
    pub fn set_fee_collector(&mut self, collector: Address) -> bool {
        let admin = runtime::calling_script_hash();
        
        // Verify admin signature
        assert!(admin == *self.owner.get(), "Only admin can set fee collector");
        
        // Update collector
        self.fee_collector.set(&collector);
        
        true
    }
    
    /// Update maximum royalty percentage (admin only)
    #[method]
    pub fn set_max_royalty(&mut self, max_royalty: u16) -> bool {
        let admin = runtime::calling_script_hash();
        
        // Verify admin signature
        assert!(admin == *self.owner.get(), "Only admin can set max royalty");
        
        // Validate royalty (max 30%)
        assert!(max_royalty <= 3000, "Max royalty too high");
        
        // Update max royalty
        self.max_royalty.set(&max_royalty);
        
        true
    }
    
    /// Set auction parameters (admin only)
    #[method]
    pub fn set_auction_parameters(
        &mut self,
        min_duration: u64,
        max_duration: u64,
        extension_time: u64,
        min_bid_increase: u16,
    ) -> bool {
        let admin = runtime::calling_script_hash();
        
        // Verify admin signature
        assert!(admin == *self.owner.get(), "Only admin can set auction parameters");
        
        // Validate parameters
        assert!(min_duration > 0, "Minimum duration must be greater than 0");
        assert!(max_duration >= min_duration, "Maximum duration must be greater than minimum");
        assert!(extension_time > 0, "Extension time must be greater than 0");
        assert!(min_bid_increase > 0, "Minimum bid increase must be greater than 0");
        
        // Update parameters
        self.min_auction_duration.set(&min_duration);
        self.max_auction_duration.set(&max_duration);
        self.auction_extension_time.set(&extension_time);
        self.min_bid_increase.set(&min_bid_increase);
        
        true
    }
    
    // === Safe methods (read-only) ===
    
    /// Get listing information
    #[safe]
    pub fn get_listing(&self, listing_id: u64) -> Option<(
        Hash160, ByteArray, Address, Hash160, u64, u8, u8, u64, u64
    )> {
        let listing = self.listings.get(&listing_id)?;
        
        Some((
            listing.nft_contract,
            listing.token_id,
            listing.owner,
            listing.payment_token,
            listing.price,
            match listing.listing_type {
                ListingType::FixedPrice => 0,
                ListingType::Auction => 1,
            },
            match listing.status {
                ListingStatus::Active => 0,
                ListingStatus::Sold => 1,
                ListingStatus::Cancelled => 2,
                ListingStatus::Expired => 3,
            },
            listing.created_at,
            listing.expires_at
        ))
    }
    
    /// Get active listing ID for an NFT
    #[safe]
    pub fn get_active_listing_id(&self, nft_contract: Hash160, token_id: ByteArray) -> Option<u64> {
        self.active_listings.get(&(nft_contract, token_id))
    }
    
    /// Get highest bid for an auction
    #[safe]
    pub fn get_highest_bid(&self, listing_id: u64) -> Option<(Address, u64, u64)> {
        let bid = self.highest_bids.get(&listing_id)?;
        
        Some((
            bid.bidder,
            bid.amount,
            bid.timestamp
        ))
    }
    
    /// Get offer information
    #[safe]
    pub fn get_offer(&self, listing_id: u64, offerer: Address) -> Option<(u64, u64, u64)> {
        let offer = self.offers.get(&(listing_id, offerer))?;
        
        Some((
            offer.amount,
            offer.created_at,
            offer.expires_at
        ))
    }
    
    /// Get collection information
    #[safe]
    pub fn get_collection(&self, nft_contract: Hash160) -> Option<(ByteArray, Address, u16, bool)> {
        let collection = self.verified_collections.get(&nft_contract)?;
        
        Some((
            collection.name,
            collection.creator,
            collection.default_royalty,
            collection.verified
        ))
    }
    
    /// Get marketplace fee percentage
    #[safe]
    pub fn get_fee_percentage(&self) -> u16 {
        *self.fee_percentage.get()
    }
    
    /// Get maximum royalty percentage
    #[safe]
    pub fn get_max_royalty(&self) -> u16 {
        *self.max_royalty.get()
    }
    
    /// Get auction parameters
    #[safe]
    pub fn get_auction_parameters(&self) -> (u64, u64, u64, u16) {
        (
            *self.min_auction_duration.get(),
            *self.max_auction_duration.get(),
            *self.auction_extension_time.get(),
            *self.min_bid_increase.get()
        )
    }
    
    // === Internal methods ===
    
    /// Validate payment token
    fn validate_payment_token(&self, token_hash: &Hash160) {
        // For now, we only support GAS as payment token
        assert!(token_hash == &GAS_TOKEN, "Unsupported payment token");
    }
    
    /// Get royalty information for an NFT
    fn get_royalty_info(&self, nft_contract: &Hash160, token_id: &ByteArray) -> (u16, Address) {
        // First try to get royalty from NFT contract
        let royalty_result: Option<(u16, Address)> = self.call_contract(
            nft_contract,
            "royaltyInfo",
            (token_id.clone(), BASIS_POINTS)
        ).ok();
        
        if let Some((royalty_percentage, recipient)) = royalty_result {
            let max_royalty = *self.max_royalty.get();
            
            // Cap royalty at max_royalty
            let capped_royalty = if royalty_percentage > max_royalty {
                max_royalty
            } else {
                royalty_percentage
            };
            
            return (capped_royalty, recipient);
        }
        
        // If NFT doesn't support royalties, check if it's a verified collection
        if let Some(collection) = self.verified_collections.get(nft_contract) {
            return (collection.default_royalty, collection.creator);
        }
        
        // Default to no royalty
        (0, Address::from([0; 20]))
    }
    
    /// Distribute payment to seller, royalty recipient, and marketplace
    fn distribute_payment(
        &self,
        listing_id: u64,
        listing: &Listing,
        buyer: Address,
        amount: u64,
    ) {
        // Calculate fees
        
        // Marketplace fee
        let fee_percentage = *self.fee_percentage.get();
        let marketplace_fee = (amount * fee_percentage as u64) / BASIS_POINTS as u64;
        
        // Royalty fee
        let royalty_fee = (amount * listing.royalty_percentage as u64) / BASIS_POINTS as u64;
        
        // Seller amount
        let seller_amount = amount - marketplace_fee - royalty_fee;
        
        // Transfer marketplace fee
        if marketplace_fee > 0 {
            let fee_collector = *self.fee_collector.get();
            
            let fee_transferred: bool = self.call_contract(
                &listing.payment_token,
                "transfer",
                (runtime::executing_script_hash(), fee_collector, marketplace_fee, ByteArray::new())
            ).expect("Fee transfer failed");
            
            assert!(fee_transferred, "Failed to transfer marketplace fee");
            
            // Emit fee paid event
            FeePaid {
                listing_id,
                collector: fee_collector,
                amount: marketplace_fee,
            }.notify();
        }
        
        // Transfer royalty fee
        if royalty_fee > 0 && listing.royalty_recipient != Address::from([0; 20]) {
            let royalty_transferred: bool = self.call_contract(
                &listing.payment_token,
                "transfer",
                (runtime::executing_script_hash(), listing.royalty_recipient, royalty_fee, ByteArray::new())
            ).expect("Royalty transfer failed");
            
            assert!(royalty_transferred, "Failed to transfer royalty fee");
            
            // Emit royalty paid event
            RoyaltyPaid {
                nft_contract: listing.nft_contract,
                token_id: listing.token_id.clone(),
                recipient: listing.royalty_recipient,
                amount: royalty_fee,
            }.notify();
        }
        
        // Transfer payment to seller
        let seller_transferred: bool = self.call_contract(
            &listing.payment_token,
            "transfer",
            (runtime::executing_script_hash(), listing.owner, seller_amount, ByteArray::new())
        ).expect("Seller payment failed");
        
        assert!(seller_transferred, "Failed to transfer payment to seller");
    }
}