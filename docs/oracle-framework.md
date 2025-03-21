# Oracle Framework for neo-contract-rs

This document outlines the design, implementation, and usage patterns for the Oracle framework in neo-contract-rs.

## Overview

The Oracle framework enables Neo N3 smart contracts written in Rust to access external data that is not available on the blockchain. This is crucial for many real-world applications that need to interact with external APIs, price feeds, or other off-chain data sources.

## Architecture

The Oracle framework consists of several components:

1. **Oracle Request Interface**: Defines how contracts request external data
2. **Oracle Response Handler**: Processes data received from oracle nodes
3. **Request Serialization**: Converts request parameters to format compatible with Neo Oracle nodes
4. **Response Deserialization**: Converts oracle responses back to usable Rust types
5. **Callback Mechanism**: Handles asynchronous responses from oracle nodes

## Core Components

### OracleRequest Trait

```rust
pub trait OracleRequest {
    type ResponseType;
    
    /// URL where the oracle should fetch data from
    fn url(&self) -> ByteString;
    
    /// Filter to apply to the response data
    fn filter(&self) -> Option<ByteString>;
    
    /// Method to call the oracle service
    fn request(&self) -> bool;
    
    /// Callback to handle the oracle response
    fn on_response(&self, response: Self::ResponseType);
    
    /// Callback for when the oracle request fails
    fn on_error(&self, error: u32);
}
```

### OracleResponse Enum

```rust
pub enum OracleResponse<T> {
    Success(T),
    Error(u32),
    Pending,
}
```

### Oracle Module

```rust
pub mod oracle {
    /// Request data from an oracle
    pub fn request(url: ByteString, filter: ByteString) -> bool;
    
    /// Get the current oracle request price in GAS
    pub fn get_price() -> u32;
    
    /// Check if the current execution is a callback from an oracle
    pub fn is_oracle_response() -> bool;
}
```

## Usage Patterns

### Basic Oracle Request

```rust
#[neo::contract]
impl PriceOracle {
    pub fn request_price(symbol: ByteString) -> bool {
        let url = ByteString::from("https://api.example.com/prices/");
        let mut url_with_symbol = url.clone();
        url_with_symbol.concat(&symbol);
        
        let filter = ByteString::from("$.price");
        
        oracle::request(url_with_symbol, filter)
    }
    
    pub fn on_price(symbol: ByteString, price: i64) {
        // Only callable by the oracle
        assert!(oracle::is_oracle_response());
        
        // Store the price
        let mut storage = StorageMap::new();
        let key = symbol.clone();
        storage.put(key, Int256::from(price));
        
        // Emit event with the new price
        emit_event!("PriceUpdate", (symbol, price));
    }
}
```

### Advanced Pattern with Custom Response Handling

```rust
#[neo::contract]
impl WeatherOracle {
    pub fn request_weather(city: ByteString) -> bool {
        let url = ByteString::from("https://api.weather.com/current?city=");
        let mut url_with_city = url.clone();
        url_with_city.concat(&city);
        
        let filter = ByteString::from("$.weather");
        
        oracle::request(url_with_city, filter)
    }
    
    pub fn on_weather(
        city: ByteString, 
        temperature: i32, 
        conditions: ByteString,
        humidity: u32
    ) {
        // Only callable by the oracle
        assert!(oracle::is_oracle_response());
        
        // Parse the response and store components
        let mut storage = StorageMap::new();
        
        let temp_key = city.clone();
        temp_key.concat(&ByteString::from("_temp"));
        storage.put(temp_key, Int256::from(temperature));
        
        let cond_key = city.clone();
        cond_key.concat(&ByteString::from("_cond"));
        storage.put(cond_key, conditions);
        
        let humid_key = city.clone();
        humid_key.concat(&ByteString::from("_humid"));
        storage.put(humid_key, Int256::from(humidity as i64));
        
        // Emit weather update event
        emit_event!("WeatherUpdate", (city, temperature, conditions, humidity));
    }
}
```

## Implementation Details

### Oracle Requests

Oracle requests are made using the Neo N3 native oracle service. When a contract makes an oracle request:

1. The contract calls the `oracle::request` function with a URL and optional filter
2. The oracle request is recorded on the blockchain
3. Oracle nodes monitor the blockchain for oracle requests
4. Oracle nodes fetch data from the specified URL
5. Oracle nodes apply the filter to the response data
6. Oracle nodes submit the filtered data back to the blockchain
7. The contract's callback method is invoked with the oracle response

### Response Handling

Oracle responses are processed asynchronously. When an oracle response is received:

1. The Neo VM executes the contract's callback method
2. The contract can check if the current execution is from an oracle using `oracle::is_oracle_response()`
3. The contract processes the oracle data and updates its state accordingly
4. The contract can emit events to notify external systems of the state change

### Error Handling

Oracle requests can fail for various reasons:

1. Invalid URL
2. External service unavailability
3. Response format mismatch
4. Filter application failure

The contract should implement proper error handling strategies for these cases.

## Best Practices

1. **Validate Oracle Data**: Always validate oracle responses before using them
2. **Handle Errors Gracefully**: Implement proper error handling for oracle failures
3. **Minimal Storage**: Store only essential data from oracle responses
4. **Cost Awareness**: Be aware of the oracle service costs in GAS
5. **Rate Limiting**: Implement rate limiting to prevent excessive oracle requests
6. **Fallback Mechanisms**: Have fallback mechanisms for when oracle data is unavailable

## Example Oracle Contract

A complete example of an oracle contract can be found in the `/examples/oracle-price-feed` directory.

## Future Enhancements

1. **Multi-Source Oracles**: Aggregate data from multiple sources for increased reliability
2. **Decentralized Oracle Networks**: Support for decentralized oracle networks
3. **Custom Data Formats**: Support for specialized data formats beyond JSON
4. **Oracle Request Batching**: Batch multiple requests for efficiency
5. **Subscription Model**: Implement subscription-based oracle updates

## References

1. [Neo N3 Oracle Service Documentation](https://docs.neo.org/docs/en-us/reference/oracle.html)
2. [Oracle Design Patterns for Smart Contracts](https://medium.com/@chainlink/digital-agreements-powered-by-chainlink-smart-contracts-706e2d9a4fc0) 