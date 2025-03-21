# Oracle Price Feed Architecture

This document describes the architecture and design decisions for the Oracle Price Feed example contract.

## Overview

The Oracle Price Feed contract demonstrates how to integrate with external data sources using Neo N3's native oracle service. The contract is designed to:

1. Request price data for various token symbols
2. Store and update price data on the blockchain
3. Provide access to the latest price data
4. Emit events when price data is updated

## Components

### 1. PriceOracle Contract

The main contract that provides the public interface for requesting and retrieving price data.

**Key Methods:**
- `request_price`: Request the current price for a token symbol
- `on_price`: Callback method for receiving oracle responses
- `get_latest_price`: Retrieve the latest stored price for a token symbol
- `get_price_update_time`: Get the timestamp of the last price update

### 2. Oracle Module Integration

The contract integrates with the Neo N3 oracle service through the `oracle` module.

**Key Interactions:**
- `oracle::request`: Request data from an external API
- `oracle::is_oracle_response`: Verify that the current execution is from an oracle

### 3. Storage Structure

The contract uses the following storage structure:

```
{symbol} -> {price}
{symbol}_time -> {timestamp}
```

Each price entry has an associated timestamp to track when it was last updated.

## Data Flow

### Price Request Flow

1. User calls `request_price(symbol)`
2. Contract constructs URL with the symbol parameter
3. Contract calls `oracle::request` with the URL and filter
4. Oracle request is recorded on the blockchain
5. Oracle nodes pick up the request and fetch data
6. Oracle nodes submit the filtered data back to the blockchain
7. Neo VM executes the `on_price` callback with the oracle response
8. Contract updates storage with the new price data
9. Contract emits a `PriceUpdate` event

```
User -> PriceOracle.request_price -> Oracle Service -> External API -> Oracle Service -> PriceOracle.on_price -> Storage Update -> Event
```

### Price Retrieval Flow

1. User calls `get_latest_price(symbol)`
2. Contract reads price data from storage
3. Contract returns the price to the user

```
User -> PriceOracle.get_latest_price -> Storage Read -> User
```

## Design Decisions

### 1. Price Data Format

Prices are stored as integers, representing the price multiplied by a factor of 10^8 to handle decimal places. This approach avoids floating-point precision issues while still providing sufficient precision for price data.

### 2. Error Handling

The contract implements error handling for:
- Invalid symbols
- Oracle request failures
- Invalid price data

Each error case has a specific error code and handling mechanism.

### 3. Security Considerations

The contract implements several security measures:
- Verification of oracle callbacks using `oracle::is_oracle_response()`
- Input validation for all user-provided parameters
- Price validation before storage

### 4. Gas Optimization

The contract is optimized for gas efficiency by:
- Minimal storage operations
- Efficient string handling
- Reuse of storage keys

## Extensibility

The architecture is designed to be extensible in several ways:

1. **Multi-source Aggregation**: The design can be extended to aggregate prices from multiple sources.
2. **Price Validity Windows**: Timestamps enable implementation of price validity windows.
3. **Price History**: The storage structure can be extended to maintain price history.
4. **Access Control**: The contract can be extended with access control for certain operations.

## Implementation Notes

1. The contract uses the `ByteString` type for symbols to match Neo's storage requirements.
2. Price updates are atomic - either the entire update succeeds or none of it does.
3. The contract does not implement any access control - all methods are publicly accessible.
4. The contract does not currently impose any rate limiting on oracle requests. 