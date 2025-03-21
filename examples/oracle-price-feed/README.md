# Oracle Price Feed Example

This example demonstrates how to implement a price oracle contract using neo-contract-rs. The contract requests price data from an external API and stores it on the blockchain.

## Features

- Request price data for any token symbol
- Store the latest price data on-chain
- Emit events when prices are updated
- Demonstrate oracle callback handling

## Contract Structure

The contract consists of the following components:

1. `PriceOracle`: Main contract that handles price requests and updates
2. `OracleRequest`: Implementation of the oracle request interface
3. `OracleResponse`: Handling of oracle responses

## Usage

### Building the Contract

```bash
cd examples/oracle-price-feed
make
```

### Requesting a Price

To request the current price of a token:

```
invoke PriceOracle request_price ["BTC"]
```

### Getting the Latest Price

To retrieve the latest price for a token:

```
invoke PriceOracle get_latest_price ["BTC"]
```

## Implementation Details

This example demonstrates:

1. How to format oracle request URLs
2. How to use JSONPath filters for response data
3. How to handle oracle callbacks securely
4. How to store and update oracle data

## Key Concepts

### Oracle Requests

The contract makes oracle requests using the Neo N3 native oracle service through the `oracle::request` function. The function requires:

1. A URL to fetch data from
2. A filter to apply to the response data

### Oracle Callbacks

When the oracle response is received, the Neo VM executes the contract's callback method. The contract checks if the current execution is an oracle response using `oracle::is_oracle_response()`.

### Data Storage

The contract stores price data in the contract's storage using:

```rust
let mut storage = StorageMap::new();
storage.put(key, Int256::from(price));
```

### Events

The contract emits events when prices are updated:

```rust
emit_event!("PriceUpdate", (symbol, price));
```

## Security Considerations

1. Always verify that oracle callbacks are legitimate using `oracle::is_oracle_response()`
2. Validate price data before storing or using it
3. Consider implementing price validity timeouts
4. For production use, consider using multiple oracle sources

## Extensions

This example can be extended in several ways:

1. Add support for multiple price sources
2. Implement time-weighted average prices (TWAP)
3. Add price validity periods
4. Implement circuit breakers for extreme price movements 