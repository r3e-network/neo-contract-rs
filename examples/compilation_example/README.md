# Counter Contract - Compilation Example

This example demonstrates a simple counter smart contract for the Neo N3 blockchain, focusing on the complete compilation process from Rust source code to Neo VM bytecode.

## Contract Features

- **Counter Management**: Initialize, get, increment, and decrement a counter value
- **Owner-Only Functions**: Reset counter and transfer ownership
- **Event Emission**: Proper Neo N3 event handling with indexed parameters
- **Multiple Constructors**: Demonstrates different constructor options

## Contract Structure

### Storage

The contract stores two key pieces of data:
- `count`: A counter value stored as a `u64`
- `owner`: The address of the contract owner stored as an `Address`

### Methods

**Constructors**:
- `new(initial_value: u64)`: Creates a new counter with a specified initial value
- `new_zero()`: Creates a new counter starting at zero

**Public Read Methods**:
- `get_count()`: Returns the current counter value
- `get_owner()`: Returns the current owner address
- `is_owner()`: Checks if the caller is the owner

**Public Write Methods**:
- `increment()`: Increases the counter by one
- `decrement()`: Decreases the counter by one (not below zero)
- `reset(new_value: u64)`: Resets the counter to a specific value (owner only)
- `transfer_ownership(new_owner: Address)`: Transfers ownership to another address (owner only)

### Events

The contract emits the following events:
- `Initialized`: When the contract is deployed
- `CountIncremented`: When the counter is increased
- `CountDecremented`: When the counter is decreased
- `CountReset`: When the counter is reset
- `OwnershipTransferred`: When ownership changes

## Compilation Process

This example includes a Makefile that demonstrates the complete compilation process:

1. **Rust Compilation**: Convert Rust code to WebAssembly (WASM)
2. **WASM to NEF**: Convert WASM to Neo Executable Format (NEF)
3. **Generate Manifest**: Create the contract manifest JSON file
4. **Package Contract**: Bundle NEF and manifest for deployment

### Build Commands

To build the contract:

```bash
# Full build process
make

# Clean build artifacts
make clean

# Build for release
make BUILD_MODE=release
```

## Using the Contract

After compiling the contract, you can deploy it to a Neo N3 blockchain using Neo CLI, Neo Express, or similar tools:

```bash
# Deploy using Neo CLI
neo-cli deploy Counter.nef

# Deploy using Neo Express
neoxp contract deploy Counter.nef
```

### Interacting with the Contract

After deployment, you can interact with the contract using the Neo CLI JSON-RPC API or Neo SDKs:

```javascript
// JavaScript example with neo-js
const { rpc, sc, wallet } = require('@cityofzion/neo-js');

// Create an instance of the contract
const counterContract = new sc.Contract('0xYourContractScriptHash');

// Get the current count
const count = await counterContract.call('getCount');
console.log(`Current count: ${count}`);

// Increment the counter (requires a signed transaction)
const incrementTx = await counterContract.invoke(
  'increment',
  [], // No parameters needed
  wallet.Account.fromWIF('YourPrivateKeyWIF')
);
await incrementTx.send();
```

## Educational Value

This example is particularly valuable for understanding:

1. **Neo Contract Structure**: How to organize contract storage, methods, and events
2. **Access Control**: How to implement owner-only functionality
3. **Event Patterns**: How to properly emit and index events
4. **Multiple Constructors**: How to provide alternative initialization methods
5. **Complete Compilation Pipeline**: The full process from Rust to deployable NEF

## Known Issues and Workarounds

If you encounter compilation issues, you may need to:

1. Ensure you have the latest neo-contract-sdk version
2. Use the `std` feature flag during development
3. Check the Makefile paths match your environment setup

## License

This example is provided under the same license as the Neo Contract Rust framework. 