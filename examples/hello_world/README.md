# Hello World Example for Neo Smart Contracts

This is a simple Hello World example for Neo N3 blockchain using the Neo Contract Rust framework.

## Overview

This example demonstrates the basic structure of a Neo smart contract written in Rust, providing a minimal implementation with essential features:

- Storage management
- Method definitions
- Basic state manipulation
- Read-only (safe) methods
- Event emission

## Current Status

**Note:** This example is currently experiencing compilation issues due to ongoing development of the Neo Contract Rust framework.

### Known Issues

Similar to other examples in this repository, the Hello World example faces issues with:

1. Procedural macro resolution
2. Storage trait implementation
3. Runtime function signatures

### How to Build

Despite the current issues, here is the correct way to build Neo contracts in this framework:

#### Development Build (with standard library features)

```bash
cargo check -p hello_world --features std
cargo build -p hello_world --features std
```

#### Production Build (for blockchain deployment)

```bash
cargo build -p hello_world --release
```

## Contract Functionality

The Hello World contract includes:

1. **Message Storage**
   - Storing a greeting message on the blockchain
   - Updating the message through contract calls
   - Retrieving the stored message

2. **Greeting Generation**
   - Creating personalized greetings by combining the stored message with a provided name
   - Example: If the stored message is "Hello" and the provided name is "Alice", the greeting will be "Hello Alice"

3. **Update Tracking**
   - Counting how many times the message has been updated
   - Providing a method to retrieve this count

4. **Access Control**
   - Basic owner management
   - Owner transfer functionality

5. **Event Emission**
   - Notifying when the message is updated
   - Including relevant information like the old message, new message, and updater address

## Code Structure

The contract follows a clean structure:

```rust
#[neo_contract::contract]
mod hello_world {
    // Import necessary libraries
    use neo_contract::prelude::*;
    
    // Define storage structure
    #[storage]
    struct HelloWorld {
        message: Item<String>,
        update_counter: Item<u32>,
        owner: Item<H160>,
    }

    // Implement contract logic
    impl HelloWorld {
        // Constructor for initialization
        #[constructor]
        fn new(message: String) -> Self {
            // ...
        }

        // Methods to update state
        #[method]
        fn set_message(&mut self, message: String) -> bool {
            // ...
        }
        
        // Read-only methods (safe)
        #[safe]
        fn get_message(&self) -> String {
            // ...
        }
        
        #[safe]
        fn hello(&self, name: String) -> String {
            // ...
        }
    }
}
```

## Using the Contract

Once deployed to the Neo N3 blockchain, you can interact with this contract using:

1. **Neo CLI/Neo Express**
   - Invoke the `set_message` method to change the stored message
   - Invoke the `hello` method with a name parameter to get a personalized greeting

2. **Neo SDK (JavaScript/TypeScript, Python, Java, etc.)**
   - Connect to the contract via its script hash
   - Call methods and read return values

Example with Neo-JS SDK:
```javascript
// Connect to the contract
const contract = new Contract('0xSCRIPT_HASH_HERE', {
  networkMagic: 844378958, // Neo N3 TestNet
  rpcAddress: 'https://testnet1.neo.coz.io:443'
});

// Set a new message (requires signing)
const result = await contract.invoke('set_message', ['New Message']);

// Get a personalized greeting (read-only)
const greeting = await contract.invokeRead('hello', ['Alice']);
console.log(greeting); // Outputs the personalized greeting
```

## Educational Value

This example serves as a starting point for learning Neo smart contract development in Rust, demonstrating:

1. Basic contract structure
2. Storage patterns
3. Method definitions and annotations
4. Event emission
5. Error handling patterns

## License

This example is provided under the same license as the Neo Contract Rust framework. 