# NEO Lottery Smart Contract

A decentralized lottery system built on the Neo N3 blockchain using the neo-contract-rs framework. This contract enables transparent, fair, and autonomous lottery operations with provably random winner selection.

## Features

- **Transparent Lottery Mechanics**: All rules, ticket purchases, and drawings are publicly verifiable on the blockchain
- **Multiple Lottery Rounds**: Support for creating sequential or concurrent lottery rounds
- **Configurable Parameters**: Customizable ticket prices, durations, maximum tickets per user, and commission rates
- **Randomness Source**: Uses block hash data combined with user inputs for fair winner selection
- **Commission Management**: Configurable fee structure with separate collector address
- **Recurring Lotteries**: Create automated sequences of lottery rounds

## Lottery Mechanics

### Lottery Creation

The contract owner can create new lottery rounds with configurable parameters:
- Ticket price in GAS tokens
- Duration in hours
- Maximum tickets per user (optional limit)
- Commission rate (basis points, where 100 = 1%)

### Ticket Purchase

Users can buy tickets for active lottery rounds:
- Tickets are purchased with GAS tokens
- Enforces maximum tickets per user if set
- Generates unique ticket numbers based on blockchain data
- Maps tickets to users for winner determination

### Winner Selection

When a lottery round ends:
- The contract uses block hash data as a source of randomness
- Randomly selects a winning ticket
- Transfers the prize pool (minus commission) to the winner
- Records lottery results on the blockchain

### Prize Distribution

- Prize pool is calculated as: (total tickets sold × ticket price) - commission
- Commission is calculated as: (total pot × commission rate / 10000)
- Winner receives the entire prize pool

## Contract Methods

### Admin Methods

- `create_lottery`: Create a new lottery round
- `create_recurring_lottery`: Create a sequence of lottery rounds
- `set_commission_collector`: Update the address that receives commissions
- `cancel_lottery`: Cancel a lottery round and refund participants
- `update_default_fee_rate`: Change the default commission rate

### User Methods

- `buy_tickets`: Purchase tickets for an active lottery
- `complete_lottery`: Trigger the lottery completion and winner selection (anyone can call after end time)

### View Methods

- `get_lottery_info`: View lottery round details
- `get_lottery_winners`: View winners of a completed lottery
- `get_user_tickets`: View a user's tickets for a specific lottery
- `get_total_participants`: Get count of participants in a lottery
- `get_current_lottery`: Get the current active lottery round
- `get_winning_odds`: Calculate a user's winning odds in a lottery

## Security Considerations

- All state-changing methods verify the caller's signature
- Winner selection uses block hash data which cannot be manipulated by participants
- Ticket number generation combines multiple factors to enhance randomness
- Admin-only functions are protected with caller verification
- Commission rates have maximum limits to prevent abuse

## Randomness Limitations

While this contract uses block hash data as a source of randomness, it has known limitations:
- Block producers could theoretically manipulate block hashes
- For high-value lotteries, consider integrating with a verifiable random function (VRF) or oracle

## Usage Example

```python
# Python example using neo-python client
from neo3.api import SmartContract

# Contract hash of the deployed lottery
contract_hash = '0x1234567890abcdef1234567890abcdef12345678'
lottery_contract = SmartContract(contract_hash)

# Create a new lottery (admin only)
# Parameters: ticket_price, duration_hours, max_tickets_per_user, commission_rate
wallet.sign_transaction(
    lottery_contract.create_lottery(
        ticket_price=10000000,  # 0.1 GAS (assuming 8 decimals)
        duration_hours=24,      # 24-hour lottery
        max_tickets_per_user=10,# Maximum 10 tickets per user
        commission_rate=300     # 3% commission
    )
)

# Buy lottery tickets
# Parameters: round_id, ticket_count
wallet.sign_transaction(
    lottery_contract.buy_tickets(
        round_id=1,         # Lottery round ID
        ticket_count=5      # Buy 5 tickets
    )
)

# Complete lottery (after end time)
wallet.sign_transaction(
    lottery_contract.complete_lottery(round_id=1)
)
```

## License

This code is provided as an example and is licensed under MIT License.