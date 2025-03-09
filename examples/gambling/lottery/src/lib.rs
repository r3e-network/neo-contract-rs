//! # NEO Lottery Contract
//!
//! A simple lottery contract that allows users to buy tickets and randomly selects winners.
//! The lottery uses block hash data as a source of randomness combined with user inputs.

#[neo_contract::contract]
mod neo_lottery {
    use neo_contract::prelude::*;
    
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
    
    #[event]
    struct TicketsPurchased {
        #[index]
        round_id: u32,
        #[index]
        user: Address,
        ticket_count: u32,
        total_cost: u64,
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
    
    #[event]
    struct LotteryCompleted {
        #[index]
        round_id: u32,
        total_participants: u32,
        total_tickets: u32,
        total_prize: u64,
        commission_amount: u64,
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
            Self {
                owner: Item::new(owner),
                current_round_id: Item::new(0),
                rounds: Map::new(),
                user_tickets: Map::new(),
                ticket_purchases: Map::new(),
                ticket_owners: Map::new(),
                round_participants: Map::new(),
                gas_token: Item::new(gas_token),
                commission_collector: Item::new(owner), // Default to owner
            }
        }
        
        /// Create a new lottery round (owner only)
        #[method]
        fn create_lottery(
            &mut self,
            ticket_price: u64,
            duration_hours: u32,
            max_tickets_per_user: u32,
            commission_rate: u16,
        ) -> u32 {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can create lotteries");
            
            // Validate inputs
            assert!(ticket_price > 0, "Ticket price must be greater than 0");
            assert!(duration_hours > 0, "Duration must be greater than 0");
            assert!(commission_rate <= 2000, "Commission cannot exceed 20%"); // Max 20%
            
            // Calculate times
            let current_time = runtime::time();
            let end_time = current_time + (duration_hours as u64 * 3600); // hours to seconds
            
            // Get next round ID
            let round_id = *self.current_round_id.get() + 1;
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
            
            // Emit event
            self.emit(LotteryCreated {
                round_id,
                ticket_price,
                start_time: current_time,
                end_time,
                max_tickets_per_user,
            });
            
            round_id
        }
        
        /// Purchase lottery tickets
        #[method]
        fn buy_tickets(&mut self, round_id: u32, ticket_count: u32) -> bool {
            let buyer = runtime::calling_script_hash();
            
            // Verify buyer signature
            assert!(runtime::check_witness(&buyer), "Invalid signature");
            
            // Check if lottery round exists and is active
            let mut lottery = self.rounds.get(&round_id).expect("Lottery round not found");
            assert!(lottery.status == LotteryStatus::Active, "Lottery is not active");
            
            // Check if lottery is still open
            let current_time = runtime::time();
            assert!(current_time < lottery.end_time, "Lottery has ended");
            
            // Check if ticket count is valid
            assert!(ticket_count > 0, "Must buy at least one ticket");
            
            // Check max tickets per user if set
            if lottery.max_tickets_per_user > 0 {
                let current_tickets = self.user_tickets.get(&(round_id, buyer)).unwrap_or_default();
                assert!(
                    current_tickets + ticket_count <= lottery.max_tickets_per_user,
                    "Would exceed max tickets per user"
                );
            }
            
            // Calculate total cost
            let total_cost = lottery.ticket_price * ticket_count as u64;
            
            // Transfer GAS from buyer to contract
            let gas_token = *self.gas_token.get();
            let transferred: bool = self.call_contract(
                &gas_token,
                "transfer",
                (buyer, runtime::executing_script_hash(), total_cost, ByteArray::new())
            ).expect("GAS transfer failed");
            
            assert!(transferred, "Failed to transfer GAS tokens");
            
            // Update lottery round data
            lottery.tickets_sold += ticket_count;
            lottery.pot_size += total_cost;
            self.rounds.insert(round_id, lottery.clone());
            
            // Update user tickets count
            let current_tickets = self.user_tickets.get(&(round_id, buyer)).unwrap_or_default();
            self.user_tickets.insert((round_id, buyer), current_tickets + ticket_count);
            
            // Generate ticket numbers for this purchase
            let mut ticket_numbers = Vec::with_capacity(ticket_count as usize);
            
            // Use various inputs for "randomness" in ticket numbers
            let block = runtime::get_block();
            let block_hash_bytes = block.hash;
            let timestamp = current_time.to_ne_bytes();
            let buyer_bytes = buyer.to_byte_array();
            
            for i in 0..ticket_count {
                // Combine inputs for ticket number generation
                let mut seed_bytes = Vec::new();
                seed_bytes.extend_from_slice(&block_hash_bytes);
                seed_bytes.extend_from_slice(&timestamp);
                seed_bytes.extend_from_slice(&buyer_bytes);
                seed_bytes.extend_from_slice(&i.to_ne_bytes());
                
                // Generate number using a simple hash-based method
                // In a real implementation, this could be more sophisticated
                let ticket_number = ((seed_bytes[0] as u32) << 24) |
                                    ((seed_bytes[1] as u32) << 16) |
                                    ((seed_bytes[2] as u32) << 8) |
                                     (seed_bytes[3] as u32);
                
                ticket_numbers.push(ticket_number);
            }
            
            // Record ticket purchase
            let purchase = TicketPurchase {
                round_id,
                user: buyer,
                ticket_count,
                purchase_time: current_time,
                ticket_numbers,
            };
            
            let purchase_index = current_tickets; // Use current ticket count as index
            self.ticket_purchases.insert((round_id, buyer, purchase_index), purchase);
            
            // Map individual tickets to the buyer
            let start_index = lottery.tickets_sold - ticket_count;
            for i in 0..ticket_count {
                let ticket_index = start_index + i;
                self.ticket_owners.insert((round_id, ticket_index), buyer);
            }
            
            // Add buyer to participants list if first purchase
            if current_tickets == 0 {
                let mut participants = self.round_participants.get(&round_id).unwrap_or_default();
                participants.push(buyer);
                self.round_participants.insert(round_id, participants);
            }
            
            // Emit event
            self.emit(TicketsPurchased {
                round_id,
                user: buyer,
                ticket_count,
                total_cost,
            });
            
            true
        }
        
        /// Complete a lottery round and select winner(s)
        #[method]
        fn complete_lottery(&mut self, round_id: u32) -> bool {
            let caller = runtime::calling_script_hash();
            
            // Check if lottery round exists
            let mut lottery = self.rounds.get(&round_id).expect("Lottery round not found");
            
            // Check if lottery can be completed
            let current_time = runtime::time();
            
            // Owner can complete early, anyone else must wait until end_time
            if caller != *self.owner.get() {
                assert!(current_time >= lottery.end_time, "Lottery hasn't ended yet");
            }
            
            // Check lottery status
            assert!(lottery.status == LotteryStatus::Active, "Lottery is not active");
            
            // Check if any tickets were sold
            if lottery.tickets_sold == 0 {
                // No tickets sold - mark as completed without winners
                lottery.status = LotteryStatus::Completed;
                self.rounds.insert(round_id, lottery);
                
                self.emit(LotteryCompleted {
                    round_id,
                    total_participants: 0,
                    total_tickets: 0,
                    total_prize: 0,
                    commission_amount: 0,
                });
                
                return true;
            }
            
            // Get current block number
            let current_block = runtime::get_block().index;
            lottery.winning_block = current_block;
            
            // Calculate commission amount
            let commission_amount = (lottery.pot_size * lottery.commission_rate as u64) / 10000;
            let prize_pool = lottery.pot_size - commission_amount;
            
            // Transfer commission to collector
            if commission_amount > 0 {
                let gas_token = *self.gas_token.get();
                let commission_collector = *self.commission_collector.get();
                
                let transferred: bool = self.call_contract(
                    &gas_token,
                    "transfer",
                    (runtime::executing_script_hash(), commission_collector, commission_amount, ByteArray::new())
                ).expect("Commission transfer failed");
                
                assert!(transferred, "Failed to transfer commission");
            }
            
            // Select winner using the next block hash as randomness source
            // In a real implementation, we'd wait for the next block hash for better randomness
            // For this example, we'll use the current block hash
            
            let block_hash = runtime::get_block().hash;
            
            // Convert hash to a large number and take modulo of tickets_sold to get winner ticket index
            let winner_ticket_index = (((block_hash[0] as u32) << 24) |
                                      ((block_hash[1] as u32) << 16) |
                                      ((block_hash[2] as u32) << 8) |
                                       (block_hash[3] as u32)) % lottery.tickets_sold;
            
            // Get winner address
            let winner = self.ticket_owners.get(&(round_id, winner_ticket_index)).expect("Winner not found");
            lottery.winners.push(winner);
            
            // Transfer prize to winner
            let gas_token = *self.gas_token.get();
            let transferred: bool = self.call_contract(
                &gas_token,
                "transfer",
                (runtime::executing_script_hash(), winner, prize_pool, ByteArray::new())
            ).expect("Prize transfer failed");
            
            assert!(transferred, "Failed to transfer prize");
            
            // Update lottery status
            lottery.status = LotteryStatus::Completed;
            self.rounds.insert(round_id, lottery.clone());
            
            // Emit events
            self.emit(WinnerSelected {
                round_id,
                winner,
                prize_amount: prize_pool,
                winning_block: current_block,
            });
            
            let participants = self.round_participants.get(&round_id).unwrap_or_default();
            
            self.emit(LotteryCompleted {
                round_id,
                total_participants: participants.len() as u32,
                total_tickets: lottery.tickets_sold,
                total_prize: prize_pool,
                commission_amount,
            });
            
            true
        }
        
        /// Update commission collector address (owner only)
        #[method]
        fn set_commission_collector(&mut self, collector: Address) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set commission collector");
            
            self.commission_collector.set(collector);
            true
        }
        
        /// Cancel a lottery round and refund participants (owner only)
        #[method]
        fn cancel_lottery(&mut self, round_id: u32) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can cancel lottery");
            
            // Check if lottery round exists
            let mut lottery = self.rounds.get(&round_id).expect("Lottery round not found");
            
            // Check lottery status
            assert!(lottery.status == LotteryStatus::Active, "Lottery is not active");
            
            // Process refunds
            let participants = self.round_participants.get(&round_id).unwrap_or_default();
            let gas_token = *self.gas_token.get();
            
            for participant in participants {
                let tickets_bought = self.user_tickets.get(&(round_id, participant)).unwrap_or_default();
                let refund_amount = tickets_bought as u64 * lottery.ticket_price;
                
                if refund_amount > 0 {
                    let transferred: bool = self.call_contract(
                        &gas_token,
                        "transfer",
                        (runtime::executing_script_hash(), participant, refund_amount, ByteArray::new())
                    ).expect("Refund transfer failed");
                    
                    assert!(transferred, "Failed to transfer refund");
                }
            }
            
            // Update lottery status
            lottery.status = LotteryStatus::Inactive;
            self.rounds.insert(round_id, lottery);
            
            true
        }
        
        /// Create a recurring lottery that starts immediately after the previous one ends
        #[method]
        fn create_recurring_lottery(
            &mut self,
            ticket_price: u64,
            duration_hours: u32,
            max_tickets_per_user: u32,
            commission_rate: u16,
            iterations: u32,
        ) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can create recurring lotteries");
            
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
        #[safe]
        fn get_lottery_winners(&self, round_id: u32) -> Option<Vec<Address>> {
            let lottery = self.rounds.get(&round_id)?;
            
            Some(lottery.winners.clone())
        }
        
        /// Get user tickets for a specific lottery round
        #[safe]
        fn get_user_tickets(&self, round_id: u32, user: Address) -> u32 {
            self.user_tickets.get(&(round_id, user)).unwrap_or_default()
        }
        
        /// Get total participants in a lottery round
        #[safe]
        fn get_total_participants(&self, round_id: u32) -> u32 {
            let participants = self.round_participants.get(&round_id).unwrap_or_default();
            participants.len() as u32
        }
        
        /// Get the current active lottery round ID
        #[safe]
        fn get_current_lottery(&self) -> u32 {
            let current_id = *self.current_round_id.get();
            
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
    }
}