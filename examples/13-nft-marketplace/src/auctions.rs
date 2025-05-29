//! # Auction Management
//!
//! Functions for creating and managing NFT auctions with bidding.

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};
use neo_contract::serialize::NeoSerializable;
use crate::types::*;
use crate::storage::*;

impl crate::NftMarketplace {
    /// Create a new NFT auction
    #[method]
    pub fn create_auction(
        &self,
        seller: H160,
        nft_contract: H160,
        token_id: ByteString,
        starting_price: Int256,
        reserve_price: Int256,
        payment_token: H160,
        duration: u64
    ) -> Int256 {
        // Check if marketplace is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Marketplace is paused"));
            return Int256::new(-1);
        }

        // Verify authorization
        if !Runtime::check_witness(seller) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return Int256::new(-1);
        }

        // Validate inputs
        if starting_price <= Int256::zero() {
            Runtime::log(ByteString::from_literal("Invalid starting price: must be positive"));
            return Int256::new(-1);
        }

        if reserve_price < starting_price {
            Runtime::log(ByteString::from_literal("Reserve price cannot be less than starting price"));
            return Int256::new(-1);
        }

        if token_id.is_empty() {
            Runtime::log(ByteString::from_literal("Invalid token ID"));
            return Int256::new(-1);
        }

        // Validate duration
        let min_duration = StorageUtils::load_u64_config(
            self.storage_keys.min_auction_duration_key.clone(),
            3600 // 1 hour
        );
        let max_duration = StorageUtils::load_u64_config(
            self.storage_keys.max_auction_duration_key.clone(),
            604800 // 7 days
        );

        if duration < min_duration || duration > max_duration {
            Runtime::log(ByteString::from_literal("Invalid auction duration"));
            return Int256::new(-1);
        }

        // Check if NFT is already listed or auctioned
        let nft_listing_key = self.storage_keys.nft_listings_key(nft_contract, token_id.clone());
        if StorageUtils::key_exists(nft_listing_key) {
            Runtime::log(ByteString::from_literal("NFT is already listed"));
            return Int256::new(-1);
        }

        let current_time = Runtime::get_time();
        let ends_at = current_time + duration;

        // Generate auction ID
        let auction_id = StorageUtils::increment_counter(self.storage_keys.auction_count_key.clone());

        // Create auction
        let auction = Auction {
            id: auction_id,
            nft_contract,
            token_id: token_id.clone(),
            seller,
            starting_price,
            reserve_price,
            current_bid: Int256::zero(),
            highest_bidder: H160::zero(),
            payment_token,
            created_at: current_time,
            ends_at,
            status: AuctionStatus::Active,
            bid_count: 0,
        };

        // Store auction
        let auction_key = self.storage_keys.auction_key(auction_id);
        let serialized_auction = self.serialize_auction(auction.clone());
        let storage = Storage::get_context();
        Storage::put(storage.clone(), auction_key, serialized_auction);

        // Update indexes
        let active_auctions_key = self.storage_keys.active_auctions_prefix.concat(&auction_id.into_byte_string());
        let storage2 = Storage::get_context();
        Storage::put(storage2.clone(), active_auctions_key, ByteString::from_literal("true"));

        // Mark NFT as in auction
        let nft_auction_key = self.storage_keys.nft_listings_key(nft_contract, token_id.clone());
        let storage3 = Storage::get_context();
        Storage::put(storage3, nft_auction_key, auction_id.into_byte_string());

        // Emit event
        Runtime::notify(
            ByteString::from_literal("AuctionCreated"),
            Array::from_items(&[
                auction_id.into_any(),
                nft_contract.into_any(),
                token_id.into_any(),
                seller.into_any(),
                starting_price.into_any(),
                reserve_price.into_any(),
                Int256::new(ends_at as i64).into_any()
            ])
        );

        auction_id
    }

    /// Place a bid on an auction
    #[method]
    pub fn place_bid(
        &self,
        auction_id: Int256,
        bidder: H160,
        amount: Int256
    ) -> bool {
        // Check if marketplace is paused
        if self.is_paused() {
            Runtime::log(ByteString::from_literal("Marketplace is paused"));
            return false;
        }

        // Verify authorization
        if !Runtime::check_witness(bidder) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Get auction
        let mut auction = match self.get_auction_data(auction_id) {
            Some(a) => a,
            None => {
                Runtime::log(ByteString::from_literal("Auction not found"));
                return false;
            }
        };

        let current_time = Runtime::get_time();

        // Validate auction can receive bids
        if !auction.can_receive_bids(current_time) {
            Runtime::log(ByteString::from_literal("Auction cannot receive bids"));
            return false;
        }

        // Prevent seller from bidding
        if bidder == auction.seller {
            Runtime::log(ByteString::from_literal("Seller cannot bid on own auction"));
            return false;
        }

        // Calculate minimum bid
        let min_bid = auction.calculate_min_bid(500); // 5% minimum increment
        if amount < min_bid {
            Runtime::log(ByteString::from_literal("Bid amount too low"));
            return false;
        }

        // Handle previous highest bidder refund
        if auction.highest_bidder != H160::zero() {
            self.refund_previous_bidder(&auction);
        }

        // Update auction with new bid
        auction.current_bid = amount;
        auction.highest_bidder = bidder;
        auction.bid_count += 1;

        // Extend auction if bid placed near end
        let bid_extension_time = 600; // 10 minutes
        if current_time + bid_extension_time > auction.ends_at {
            auction.ends_at = current_time + bid_extension_time;
        }

        // Update auction and store bid
        let auction_key = self.storage_keys.auction_key(auction_id);
        let storage = Storage::get_context();
        Storage::put(storage.clone(), auction_key, self.serialize_auction(auction.clone()));

        // Store bid information
        let bid = Bid {
            auction_id,
            bidder,
            amount,
            timestamp: current_time,
        };
        let bid_key = self.storage_keys.bid_key(auction_id, bidder);
        let storage2 = Storage::get_context();
        Storage::put(storage2, bid_key, self.serialize_bid(bid));

        // Handle escrow (simplified - in production, transfer tokens to escrow)
        StorageUtils::store_escrow_balance(bidder, auction.payment_token, amount, &self.storage_keys);

        // Emit events
        Runtime::notify(
            ByteString::from_literal("BidPlaced"),
            Array::from_items(&[
                auction_id.into_any(),
                bidder.into_any(),
                amount.into_any(),
                Int256::new(auction.bid_count as i64).into_any()
            ])
        );

        if auction.ends_at > current_time + bid_extension_time {
            Runtime::notify(
                ByteString::from_literal("AuctionExtended"),
                Array::from_items(&[
                    auction_id.into_any(),
                    Int256::new(auction.ends_at as i64).into_any()
                ])
            );
        }

        true
    }

    /// End an auction
    #[method]
    pub fn end_auction(&self, auction_id: Int256) -> bool {
        // Get auction
        let mut auction = match self.get_auction_data(auction_id) {
            Some(a) => a,
            None => {
                Runtime::log(ByteString::from_literal("Auction not found"));
                return false;
            }
        };

        let current_time = Runtime::get_time();

        // Check if auction can be ended
        if auction.status != AuctionStatus::Active {
            Runtime::log(ByteString::from_literal("Auction is not active"));
            return false;
        }

        // Check if auction has ended naturally or can be ended by seller
        let can_end = current_time >= auction.ends_at ||
                     (Runtime::check_witness(auction.seller) && auction.bid_count == 0);

        if !can_end {
            Runtime::log(ByteString::from_literal("Auction cannot be ended yet"));
            return false;
        }

        // Update auction status
        auction.status = AuctionStatus::Ended;
        let auction_key = self.storage_keys.auction_key(auction_id);
        let storage = Storage::get_context();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        Storage::put(storage_clone, auction_key, self.serialize_auction(auction.clone()));

        // Remove from active auctions index
        let active_auctions_key = self.storage_keys.active_auctions_prefix.concat(&auction_id.into_byte_string());
        StorageUtils::delete_key(active_auctions_key);

        // Handle settlement
        if auction.highest_bidder != H160::zero() && auction.has_reserve_met() {
            self.settle_auction(&auction);
        } else {
            // No valid bids or reserve not met
            if auction.highest_bidder != H160::zero() {
                self.refund_previous_bidder(&auction);
            }

            Runtime::notify(
                ByteString::from_literal("AuctionEndedWithoutSale"),
                Array::from_items(&[
                    auction_id.into_any(),
                    auction.current_bid.into_any()
                ])
            );
        }

        // Remove NFT from auction index
        let nft_auction_key = self.storage_keys.nft_listings_key(
            auction.nft_contract,
            auction.token_id.clone()
        );
        StorageUtils::delete_key(nft_auction_key);

        Runtime::notify(
            ByteString::from_literal("AuctionEnded"),
            Array::from_items(&[
                auction_id.into_any(),
                auction.highest_bidder.into_any(),
                auction.current_bid.into_any()
            ])
        );

        true
    }

    /// Cancel an auction (seller only, no bids)
    #[method]
    pub fn cancel_auction(&self, auction_id: Int256, canceller: H160) -> bool {
        // Verify authorization
        if !Runtime::check_witness(canceller) {
            Runtime::log(ByteString::from_literal("Unauthorized: Invalid witness"));
            return false;
        }

        // Get auction
        let mut auction = match self.get_auction_data(auction_id) {
            Some(a) => a,
            None => {
                Runtime::log(ByteString::from_literal("Auction not found"));
                return false;
            }
        };

        // Verify canceller is seller or marketplace owner
        if canceller != auction.seller && !self.is_owner() {
            Runtime::log(ByteString::from_literal("Only seller or owner can cancel auction"));
            return false;
        }

        // Check if auction can be cancelled
        if auction.status != AuctionStatus::Active {
            Runtime::log(ByteString::from_literal("Auction is not active"));
            return false;
        }

        if auction.bid_count > 0 {
            Runtime::log(ByteString::from_literal("Cannot cancel auction with bids"));
            return false;
        }

        // Update auction status
        auction.status = AuctionStatus::Cancelled;
        let auction_key = self.storage_keys.auction_key(auction_id);
        let storage = Storage::get_context();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        Storage::put(storage_clone, auction_key, self.serialize_auction(auction.clone()));

        // Remove from indexes
        let active_auctions_key = self.storage_keys.active_auctions_prefix.concat(&auction_id.into_byte_string());
        StorageUtils::delete_key(active_auctions_key);

        let nft_auction_key = self.storage_keys.nft_listings_key(
            auction.nft_contract,
            auction.token_id.clone()
        );
        StorageUtils::delete_key(nft_auction_key);

        Runtime::notify(
            ByteString::from_literal("AuctionCancelled"),
            Array::from_items(&[
                auction_id.into_any(),
                canceller.into_any()
            ])
        );

        true
    }

    /// Get auction information
    #[method]
    #[safe]
    pub fn get_auction(&self, auction_id: Int256) -> Map<ByteString, Any> {
        let mut result = Map::new();

        match self.get_auction_data(auction_id) {
            Some(auction) => {
                result.put(ByteString::from_literal("id"), auction.id.into_any());
                result.put(ByteString::from_literal("nft_contract"), auction.nft_contract.into_any());
                result.put(ByteString::from_literal("token_id"), auction.token_id.clone().into_any());
                result.put(ByteString::from_literal("seller"), auction.seller.into_any());
                result.put(ByteString::from_literal("starting_price"), auction.starting_price.into_any());
                result.put(ByteString::from_literal("reserve_price"), auction.reserve_price.into_any());
                result.put(ByteString::from_literal("current_bid"), auction.current_bid.into_any());
                result.put(ByteString::from_literal("highest_bidder"), auction.highest_bidder.into_any());
                result.put(ByteString::from_literal("payment_token"), auction.payment_token.into_any());
                result.put(ByteString::from_literal("created_at"), Int256::new(auction.created_at as i64).into_any());
                result.put(ByteString::from_literal("ends_at"), Int256::new(auction.ends_at as i64).into_any());
                result.put(ByteString::from_literal("status"), Int256::new(auction.status.to_u8() as i64).into_any());
                result.put(ByteString::from_literal("bid_count"), Int256::new(auction.bid_count as i64).into_any());

                let current_time = Runtime::get_time();
                result.put(ByteString::from_literal("can_bid"),
                    if auction.can_receive_bids(current_time) { Int256::one() } else { Int256::zero() }.into_any());
                result.put(ByteString::from_literal("time_remaining"),
                    Int256::new(if current_time < auction.ends_at {
                        (auction.ends_at - current_time) as i64
                    } else {
                        0
                    }).into_any());
                result.put(ByteString::from_literal("reserve_met"),
                    if auction.has_reserve_met() { Int256::one() } else { Int256::zero() }.into_any());
                result.put(ByteString::from_literal("min_next_bid"), auction.calculate_min_bid(500).into_any());
            },
            None => {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("Auction not found").into_any());
            }
        }

        result
    }

    /// Get bid information for a specific bidder
    #[method]
    #[safe]
    pub fn get_bid(&self, auction_id: Int256, bidder: H160) -> Map<ByteString, Any> {
        let mut result = Map::new();

        let storage = Storage::get_context();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let storage_clone = storage.clone();
        let bid_key = self.storage_keys.bid_key(auction_id, bidder);

        match Storage::get(Storage::get_context(), bid_key) {
            Some(bid_data) => {
                let bid = self.deserialize_bid(bid_data);
                result.put(ByteString::from_literal("auction_id"), bid.auction_id.into_any());
                result.put(ByteString::from_literal("bidder"), bid.bidder.into_any());
                result.put(ByteString::from_literal("amount"), bid.amount.into_any());
                result.put(ByteString::from_literal("timestamp"), Int256::new(bid.timestamp as i64).into_any());
            },
            None => {
                result.put(ByteString::from_literal("error"), ByteString::from_literal("Bid not found").into_any());
            }
        }

        result
    }

    // Helper functions

    fn get_auction_data(&self, auction_id: Int256) -> Option<Auction> {
        let auction_key = self.storage_keys.auction_key(auction_id);

        match Storage::get(Storage::get_context(), auction_key) {
            Some(auction_data) => Some(self.deserialize_auction(auction_data)),
            None => None,
        }
    }

    fn serialize_auction(&self, auction: Auction) -> ByteString {
        // Simplified serialization
        let mut data = auction.id.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.nft_contract.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.token_id);
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.seller.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.starting_price.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.reserve_price.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.current_bid.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.highest_bidder.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&auction.payment_token.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&auction.created_at.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&auction.ends_at.to_le_bytes()));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&[auction.status.to_u8()]));
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&auction.bid_count.to_le_bytes()));
        data
    }

    fn deserialize_auction(&self, data: ByteString) -> Auction {
        // Simplified deserialization - in production, use proper parsing
        Auction {
            id: Int256::zero(),
            nft_contract: H160::zero(),
            token_id: ByteString::empty(),
            seller: H160::zero(),
            starting_price: Int256::zero(),
            reserve_price: Int256::zero(),
            current_bid: Int256::zero(),
            highest_bidder: H160::zero(),
            payment_token: H160::zero(),
            created_at: 0,
            ends_at: 0,
            status: AuctionStatus::Active,
            bid_count: 0,
        }
    }

    fn serialize_bid(&self, bid: Bid) -> ByteString {
        // Simplified serialization
        let mut data = bid.auction_id.into_byte_string();
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&bid.bidder.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&bid.amount.into_byte_string());
        data = data.concat(&ByteString::from_literal("|"));
        data = data.concat(&ByteString::from_bytes(&bid.timestamp.to_le_bytes()));
        data
    }

    fn deserialize_bid(&self, data: ByteString) -> Bid {
        // Simplified deserialization - in production, use proper parsing
        Bid {
            auction_id: Int256::zero(),
            bidder: H160::zero(),
            amount: Int256::zero(),
            timestamp: 0,
        }
    }

    fn refund_previous_bidder(&self, auction: &Auction) {
        if auction.highest_bidder == H160::zero() {
            return;
        }

        // Remove escrow balance for previous bidder
        StorageUtils::store_escrow_balance(
            auction.highest_bidder,
            auction.payment_token,
            Int256::zero(),
            &self.storage_keys
        );

        // In production, this would transfer tokens back to the bidder
        Runtime::notify(
            ByteString::from_literal("BidRefunded"),
            Array::from_items(&[
                auction.id.into_any(),
                auction.highest_bidder.into_any(),
                auction.current_bid.into_any()
            ])
        );
    }

    fn settle_auction(&self, auction: &Auction) {
        // Calculate fees
        let fee_calculation = self.calculate_fees(
            auction.nft_contract,
            auction.token_id.clone(),
            auction.current_bid
        );

        // Process payment and transfers (simplified)
        // In production, this would:
        // 1. Transfer NFT from seller to highest bidder
        // 2. Transfer payment from escrow to seller (minus fees)
        // 3. Distribute platform fees and royalties

        // Record sale
        self.record_sale(
            auction.nft_contract,
            auction.token_id.clone(),
            auction.seller,
            auction.highest_bidder,
            auction.current_bid,
            auction.payment_token,
            fee_calculation.platform_fee,
            Int256::new(fee_calculation.royalty_fees.iter().map(|r| r.percentage as i64).sum::<i64>()),
            SaleType::Auction
        );

        Runtime::notify(
            ByteString::from_literal("AuctionSettled"),
            Array::from_items(&[
                auction.id.into_any(),
                auction.seller.into_any(),
                auction.highest_bidder.into_any(),
                auction.current_bid.into_any(),
                fee_calculation.platform_fee.into_any()
            ])
        );
    }
}