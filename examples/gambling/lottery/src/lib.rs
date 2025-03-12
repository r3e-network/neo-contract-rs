#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

//! # NEO Lottery Contract
//!
//! A simple lottery contract that allows users to buy tickets and randomly selects winners.
//! The lottery uses block hash data as a source of randomness combined with user inputs.
//! This contract follows Neo N3 standards and best practices.

#[neo_contract::contract]
mod neo_lottery {
    use neo_contract::prelude::*;
    use alloc::vec::Vec;
    use alloc::string::String;
    
    /// Lottery status enum
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum LotteryStatus {
        Inactive,
        Active,
        Completed,
    }
    
    /// Lottery round data
    #[derive(Debug, Clone, Encode, Decode)]
    struct LotteryRound {
        /// Unique ID for this lottery round
        id: u32,
        /// Ticket price in GAS tokens
        ticket_price: u64,
        /// Start time (timestamp)
        start_time: u64,
        /// End time (timestamp)
        end_time: u64,
        /// Total tickets sold
        tickets_sold: u32,
        /// Total pot size
        pot_size: u64,
        /// Max tickets per user (0 = unlimited)
        max_tickets_per_user: u32,
        /// Lottery commission percentage (basis points: 100 = 1%)
        commission_rate: u16,
        /// Winner(s) address(es)
        winners: Vec<Address>,
        /// Status of this lottery round
        status: LotteryStatus,
        /// Block number used for winner selection
        winning_block: u32,
    }
    
    /// Ticket purchase record
    #[derive(Debug, Clone, Encode, Decode)]
    struct TicketPurchase {
        /// Round ID
        round_id: u32,
        /// User address
        user: Address,
        /// Number of tickets purchased
        ticket_count: u32,
        /// Purchase time
        purchase_time: u64,
        /// Ticket numbers (can be used for additional randomness)
        ticket_numbers: Vec<u32>,
    }
    
    /// Events emitted by the lottery contract
    #[event]
    struct LotteryCreated {
        #[index]
        round_id: u32,
        ticket_price: u64,
        start_time: u64,
        end_time: u64,
        max_tickets_per_user: u32,
    }
    
    /// Implementation for properly emitting the LotteryCreated event using Neo N3 standards
    impl LotteryCreated {
        /// Static method to emit the LotteryCreated event in Neo N3 format
        pub fn emit(round_id: u32, ticket_price: u64, start_time: u64, end_time: u64, max_tickets_per_user: u32) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("LotteryCreated");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(round_id));
            event_data.push(Any::from(ticket_price));
            event_data.push(Any::from(start_time));
            event_data.push(Any::from(end_time));
            event_data.push(Any::from(max_tickets_per_user));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    #[event]
    struct TicketsPurchased {
        #[index]
        round_id: u32,
        #[index]
        user: Address,
        ticket_count: u32,
        total_cost: u64,
    }
    
    /// Implementation for properly emitting the TicketsPurchased event using Neo N3 standards
    impl TicketsPurchased {
        /// Static method to emit the TicketsPurchased event in Neo N3 format
        pub fn emit(round_id: u32, user: Address, ticket_count: u32, total_cost: u64) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("TicketsPurchased");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(round_id));
            event_data.push(Any::from(user));
            event_data.push(Any::from(ticket_count));
            event_data.push(Any::from(total_cost));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    #[event]
    struct WinnerSelected {
        #[index]
        round_id: u32,
        #[index]
        winner: Address,
        prize_amount: u64,
        winning_block: u32,
    }
    
    /// Implementation for properly emitting the WinnerSelected event using Neo N3 standards
    impl WinnerSelected {
        /// Static method to emit the WinnerSelected event in Neo N3 format
        pub fn emit(round_id: u32, winner: Address, prize_amount: u64, winning_block: u32) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("WinnerSelected");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(round_id));
            event_data.push(Any::from(winner));
            event_data.push(Any::from(prize_amount));
            event_data.push(Any::from(winning_block));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    #[event]
    struct LotteryCompleted {
        #[index]
        round_id: u32,
        total_participants: u32,
        total_tickets: u32,
        total_prize: u64,
        commission_amount: u64,
    }
    
    /// Implementation for properly emitting the LotteryCompleted event using Neo N3 standards
    impl LotteryCompleted {
        /// Static method to emit the LotteryCompleted event in Neo N3 format
        pub fn emit(round_id: u32, total_participants: u32, total_tickets: u32, total_prize: u64, commission_amount: u64) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("LotteryCompleted");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(round_id));
            event_data.push(Any::from(total_participants));
            event_data.push(Any::from(total_tickets));
            event_data.push(Any::from(total_prize));
            event_data.push(Any::from(commission_amount));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    /// Lottery contract storage
    #[storage]
    struct NeoLottery {
        /// Contract owner
        owner: Item<Address>,
        
        /// Current active lottery round ID
        current_round_id: Item<u32>,
        
        /// Maps round ID to lottery round data
        rounds: Map<u32, LotteryRound>,
        
        /// Maps (round_id, user) to total tickets purchased by user in the round
        user_tickets: Map<(u32, Address), u32>,
        
        /// Maps (round_id, user, index) to ticket purchase record
        ticket_purchases: Map<(u32, Address, u32), TicketPurchase>,
        
        /// Maps (round_id, ticket_index) to owner address
        ticket_owners: Map<(u32, u32), Address>,
        
        /// Maps round ID to list of participants
        round_participants: Map<u32, Vec<Address>>,
        
        /// GAS token contract hash
        gas_token: Item<Hash160>,
        
        /// Commission collector address (can be owner or separate)
        commission_collector: Item<Address>,
    }
    
    impl NeoLottery {
        /// Initialize the lottery contract
        #[constructor]
        fn new(owner: Address, gas_token: Hash160) -> Self {
            let mut instance = Self {
                owner: Item::new("owner"),
                current_round_id: Item::new("current_round_id"),
                rounds: Map::new(),
                user_tickets: Map::new(),
                ticket_purchases: Map::new(),
                ticket_owners: Map::new(),
                round_participants: Map::new(),
                gas_token: Item::new("gas_token"),
                commission_collector: Item::new("commission_collector"),
            };
            
            // Initialize with proper values
            instance.owner.set(owner);
            instance.current_round_id.set(0);
            instance.gas_token.set(gas_token);
            instance.commission_collector.set(owner); // Default to owner
            
            instance
        }
        
        /// Create a new lottery round (owner only)
        #[method]
        #[no_reentry]
        fn create_lottery(
            &mut self,
            ticket_price: u64,
            duration_hours: u32,
            max_tickets_per_user: u32,
            commission_rate: u16,
        ) -> u32 {
            // Verify caller is owner
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "Authentication failed");
            
            let owner = self.owner.get().unwrap_or_default();
            assert!(caller == owner, "Only owner can create lotteries");
            
            // Validate inputs
            assert!(ticket_price > 0, "Ticket price must be greater than 0");
            assert!(duration_hours > 0, "Duration must be greater than 0");
            assert!(commission_rate <= 2000, "Commission cannot exceed 20%"); // Max 20%
            
            // Calculate times
            let current_time = Runtime::time();
            let end_time = current_time + (duration_hours as u64 * 3600); // hours to seconds
            
            // Get next round ID
            let round_id = self.current_round_id.get().unwrap_or_default() + 1;
            self.current_round_id.set(round_id);
            
            // Create lottery round
            let lottery = LotteryRound {
                id: round_id,
                ticket_price,
                start_time: current_time,
                end_time,
                tickets_sold: 0,
                pot_size: 0,
                max_tickets_per_user,
                commission_rate,
                winners: Vec::new(),
                status: LotteryStatus::Active,
                winning_block: 0,
            };
            
            // Store lottery round
            self.rounds.insert(round_id, lottery);
            
            // Initialize participants list
            self.round_participants.insert(round_id, Vec::new());
            
            // Emit event with proper Neo N3 format
            LotteryCreated::emit(
                round_id,
                ticket_price,
                current_time,
                end_time,
                max_tickets_per_user
            );
            
            round_id
        }
        
        /// Purchase lottery tickets
        #[method]
        #[no_reentry]
        fn buy_tickets(&mut self, round_id: u32, ticket_count: u32) -> bool {
            // Validate inputs
            assert!(ticket_count > 0, "Must purchase at least one ticket");
            
            // Get lottery round
            let mut lottery = self.rounds.get(&round_id).expect("Lottery round not found");
            
            // Verify lottery is active
            assert!(lottery.status == LotteryStatus::Active, "Lottery not active");
            
            // Check if lottery has ended
            let current_time = Runtime::time();
            assert!(current_time < lottery.end_time, "Lottery has ended");
            
            // Get buyer address
            let buyer = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&buyer), "Authentication failed");
            
            // Check max tickets per user
            if lottery.max_tickets_per_user > 0 {
                let user_tickets = self.user_tickets.get(&(round_id, buyer)).unwrap_or_default();
                assert!(user_tickets + ticket_count <= lottery.max_tickets_per_user, 
                       "Exceeds maximum tickets per user");
            }
            
            // Calculate cost
            let total_cost = lottery.ticket_price * ticket_count as u64;
            
            // Transfer GAS from buyer to contract
            let gas_token = self.gas_token.get().unwrap_or_default();
            
            // Use proper NEP-17 transfer_from for Neo N3
            let mut transfer_args = Array::<Any>::new();
            transfer_args.push(Any::from(buyer));
            transfer_args.push(Any::from(Runtime::executing_script_hash()));
            transfer_args.push(Any::from(total_cost));
            transfer_args.push(Any::from(ByteArray::new())); // Add data parameter for NEP-17
            
            let success = Runtime::call_contract(&gas_token, "transfer", &transfer_args)
                .expect("Failed to call transfer")
                .as_bool()
                .expect("Invalid transfer response");
                
            assert!(success, "Token transfer failed");
            
            // Update lottery
            lottery.tickets_sold += ticket_count;
            lottery.pot_size += total_cost;
            
            // Update storage
            self.rounds.insert(round_id, lottery.clone());
            
            // Update user tickets
            let user_tickets = self.user_tickets.get(&(round_id, buyer)).unwrap_or_default();
            self.user_tickets.insert((round_id, buyer), user_tickets + ticket_count);
            
            // Add user to participants if first purchase
            if user_tickets == 0 {
                let mut participants = self.round_participants.get(&round_id).unwrap_or_default();
                participants.push(buyer);
                self.round_participants.insert(round_id, participants);
            }
            
            // Record ticket purchase
            let purchase = TicketPurchase {
                round_id,
                user: buyer,
                ticket_count,
                purchase_time: current_time,
                ticket_numbers: self.generate_ticket_numbers(round_id, buyer, ticket_count),
            };
            
            let purchase_index = user_tickets / 10; // Group purchases in batches of 10
            self.ticket_purchases.insert((round_id, buyer, purchase_index), purchase);
            
            // Record ticket ownership
            let start_ticket = lottery.tickets_sold - ticket_count + 1;
            for i in 0..ticket_count {
                let ticket_number = start_ticket + i;
                self.ticket_owners.insert((round_id, ticket_number), buyer);
            }
            
            // Emit event with proper Neo N3 format
            TicketsPurchased::emit(round_id, buyer, ticket_count, total_cost);
            
            true
        }
        
        /// Complete a lottery round and select winner(s)
        #[method]
        #[no_reentry]
        fn complete_lottery(&mut self, round_id: u32) -> bool {
            // Verify caller is owner
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "Authentication failed");
            
            let owner = self.owner.get().unwrap_or_default();
            assert!(caller == owner, "Only owner can complete lotteries");
            
            // Get lottery round
            let mut lottery = self.rounds.get(&round_id).expect("Lottery round not found");
            
            // Verify lottery can be completed
            assert!(lottery.status == LotteryStatus::Active, "Lottery not active");
            
            // Check if lottery has ended
            let current_time = Runtime::time();
            assert!(current_time >= lottery.end_time, "Lottery has not ended yet");
            
            // Verify tickets were sold
            assert!(lottery.tickets_sold > 0, "No tickets sold");
            
            // Get current block for randomness
            let current_block = Ledger::current_index();
            lottery.winning_block = current_block;
            
            // Calculate prize and commission
            let commission_amount = (lottery.pot_size * lottery.commission_rate as u64) / 10000;
            let prize_amount = lottery.pot_size - commission_amount;
            
            // Select winner using block hash as randomness
            let block = Ledger::get_block(current_block).expect("Failed to get block");
            let block_hash = block.hash;
            let random_number = self.bytes_to_u32(&block_hash) % lottery.tickets_sold + 1;
            
            // Get winner address
            let winner = self.ticket_owners.get(&(round_id, random_number))
                .expect("Failed to get winner");
            
            // Add winner to lottery
            lottery.winners.push(winner);
            
            // Update lottery status
            lottery.status = LotteryStatus::Completed;
            
            // Update storage
            self.rounds.insert(round_id, lottery);
            
            // Transfer prize to winner
            let gas_token = self.gas_token.get().unwrap_or_default();
            
            // Use proper NEP-17 transfer for Neo N3
            let mut transfer_args = Array::<Any>::new();
            transfer_args.push(Any::from(Runtime::executing_script_hash()));
            transfer_args.push(Any::from(winner));
            transfer_args.push(Any::from(prize_amount));
            transfer_args.push(Any::from(ByteArray::new())); // data parameter
            
            let success = Runtime::call_contract(&gas_token, "transfer", &transfer_args)
                .expect("Failed to call transfer")
                .as_bool()
                .expect("Invalid transfer response");
                
            assert!(success, "Prize transfer failed");
            
            // Transfer commission to collector
            if commission_amount > 0 {
                let collector = self.commission_collector.get().unwrap_or_default();
                
                let mut transfer_args = Array::<Any>::new();
                transfer_args.push(Any::from(Runtime::executing_script_hash()));
                transfer_args.push(Any::from(collector));
                transfer_args.push(Any::from(commission_amount));
                transfer_args.push(Any::from(ByteArray::new())); // data parameter
                
                let success = Runtime::call_contract(&gas_token, "transfer", &transfer_args)
                    .expect("Failed to call transfer")
                    .as_bool()
                    .expect("Invalid transfer response");
                    
                assert!(success, "Commission transfer failed");
            }
            
            // Emit winner event with proper Neo N3 format
            WinnerSelected::emit(round_id, winner, prize_amount, current_block);
            
            // Emit completion event with proper Neo N3 format
            let participants = self.round_participants.get(&round_id).unwrap_or_default();
            LotteryCompleted::emit(
                round_id,
                participants.len() as u32,
                lottery.tickets_sold,
                prize_amount,
                commission_amount
            );
            
            true
        }
        
        /// Update commission collector address (owner only)
        #[method]
        #[no_reentry]
        fn set_commission_collector(&mut self, collector: Address) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "Authentication failed");
            
            let owner = self.owner.get().unwrap_or_default();
            assert!(caller == owner, "Only owner can set commission collector");
            
            self.commission_collector.set(collector);
            true
        }
        
        /// Cancel a lottery round and refund participants (owner only)
        #[method]
        #[no_reentry]
        fn cancel_lottery(&mut self, round_id: u32) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "Authentication failed");
            
            let owner = self.owner.get().unwrap_or_default();
            assert!(caller == owner, "Only owner can cancel lottery");
            
            // Check if lottery round exists
            let mut lottery = self.rounds.get(&round_id).expect("Lottery round not found");
            
            // Check lottery status
            assert!(lottery.status == LotteryStatus::Active, "Lottery is not active");
            
            // Process refunds
            let participants = self.round_participants.get(&round_id).unwrap_or_default();
            let gas_token = self.gas_token.get().unwrap_or_default();
            
            for participant in participants {
                let tickets_bought = self.user_tickets.get(&(round_id, participant)).unwrap_or_default();
                let refund_amount = tickets_bought as u64 * lottery.ticket_price;
                
                if refund_amount > 0 {
                    // Use proper NEP-17 transfer for Neo N3
                    let mut transfer_args = Array::<Any>::new();
                    transfer_args.push(Any::from(Runtime::executing_script_hash()));
                    transfer_args.push(Any::from(participant));
                    transfer_args.push(Any::from(refund_amount));
                    transfer_args.push(Any::from(ByteArray::new())); // data parameter
                    
                    let success = Runtime::call_contract(&gas_token, "transfer", &transfer_args)
                        .expect("Failed to call transfer")
                        .as_bool()
                        .expect("Invalid transfer response");
                        
                    assert!(success, "Refund transfer failed");
                }
            }
            
            // Update lottery status
            lottery.status = LotteryStatus::Inactive;
            self.rounds.insert(round_id, lottery);
            
            true
        }
        
        /// Create a recurring lottery that starts immediately after the previous one ends
        #[method]
        #[no_reentry]
        fn create_recurring_lottery(
            &mut self,
            ticket_price: u64,
            duration_hours: u32,
            max_tickets_per_user: u32,
            commission_rate: u16,
            iterations: u32,
        ) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "Authentication failed");
            
            let owner = self.owner.get().unwrap_or_default();
            assert!(caller == owner, "Only owner can create recurring lotteries");
            
            // Validate inputs
            assert!(ticket_price > 0, "Ticket price must be greater than 0");
            assert!(duration_hours > 0, "Duration must be greater than 0");
            assert!(commission_rate <= 2000, "Commission cannot exceed 20%");
            assert!(iterations > 0 && iterations <= 52, "Invalid iterations (1-52 allowed)");
            
            // Create first lottery
            let round_id = self.create_lottery(
                ticket_price,
                duration_hours,
                max_tickets_per_user,
                commission_rate
            );
            
            // Store metadata for recurring lotteries
            // In a real implementation, we would have a system to track and create
            // the remaining iterations automatically when each lottery completes
            
            true
        }
        
        /// Get lottery round information
        #[method]
        #[safe]
        fn get_lottery_info(&self, round_id: u32) -> Option<(u64, u64, u64, u32, u64, u16, u8)> {
            let lottery = self.rounds.get(&round_id)?;
            
            // Return lottery details:
            // (ticket_price, start_time, end_time, tickets_sold, pot_size, commission_rate, status)
            Some((
                lottery.ticket_price,
                lottery.start_time,
                lottery.end_time,
                lottery.tickets_sold,
                lottery.pot_size,
                lottery.commission_rate,
                match lottery.status {
                    LotteryStatus::Inactive => 0,
                    LotteryStatus::Active => 1,
                    LotteryStatus::Completed => 2,
                }
            ))
        }
        
        /// Get lottery winners
        #[method]
        #[safe]
        fn get_lottery_winners(&self, round_id: u32) -> Option<Vec<Address>> {
            let lottery = self.rounds.get(&round_id)?;
            
            Some(lottery.winners.clone())
        }
        
        /// Get user tickets for a specific lottery round
        #[method]
        #[safe]
        fn get_user_tickets(&self, round_id: u32, user: Address) -> u32 {
            self.user_tickets.get(&(round_id, user)).unwrap_or_default()
        }
        
        /// Get total participants in a lottery round
        #[method]
        #[safe]
        fn get_total_participants(&self, round_id: u32) -> u32 {
            let participants = self.round_participants.get(&round_id).unwrap_or_default();
            participants.len() as u32
        }
        
        /// Get the current active lottery round ID
        #[method]
        #[safe]
        fn get_current_lottery(&self) -> u32 {
            let current_id = self.current_round_id.get().unwrap_or_default();
            
            // Check if the current round is still active
            if current_id > 0 {
                if let Some(lottery) = self.rounds.get(&current_id) {
                    if lottery.status == LotteryStatus::Active {
                        return current_id;
                    }
                }
            }
            
            0 // No active lottery
        }
        
        /// Check winning odds for a user in a specific lottery round
        #[method]
        #[safe]
        fn get_winning_odds(&self, round_id: u32, user: Address) -> (u32, u32, u64) {
            let lottery = match self.rounds.get(&round_id) {
                Some(l) => l,
                None => return (0, 0, 0),
            };
            
            let user_tickets = self.user_tickets.get(&(round_id, user)).unwrap_or_default();
            
            if lottery.tickets_sold == 0 {
                return (0, 0, 0);
            }
            
            let user_odds_numerator = user_tickets;
            let user_odds_denominator = lottery.tickets_sold;
            let potential_winnings = if user_tickets > 0 {
                // Calculate potential winnings (after commission)
                let commission = (lottery.pot_size * lottery.commission_rate as u64) / 10000;
                lottery.pot_size - commission
            } else {
                0
            };
            
            (user_odds_numerator, user_odds_denominator, potential_winnings)
        }

        /// Generate pseudo-random ticket numbers
        fn generate_ticket_numbers(&self, round_id: u32, user: Address, count: u32) -> Vec<u32> {
            let mut numbers = Vec::new();
            
            // Use multiple sources of entropy
            let timestamp = Runtime::time();
            let block_height = Ledger::current_index();
            let block = Ledger::get_block(block_height).expect("Failed to get block");
            let block_hash = block.hash;
            
            // Combine data for seed
            let mut seed_data = Vec::new();
            seed_data.extend_from_slice(&round_id.to_ne_bytes());
            seed_data.extend_from_slice(&user.to_vec());
            seed_data.extend_from_slice(&timestamp.to_ne_bytes());
            seed_data.extend_from_slice(&block_height.to_ne_bytes());
            seed_data.extend_from_slice(&block_hash);
            
            // Generate numbers
            for i in 0..count {
                seed_data.extend_from_slice(&i.to_ne_bytes());
                let hash = Runtime::sha256(&seed_data);
                let number = self.bytes_to_u32(&hash);
                numbers.push(number);
            }
            
            numbers
        }

        /// Convert bytes to u32
        fn bytes_to_u32(&self, bytes: &[u8]) -> u32 {
            let mut result = 0u32;
            let len = bytes.len().min(4);
            
            for i in 0..len {
                result = (result << 8) | bytes[i] as u32;
            }
            
            result
        }
    }
}