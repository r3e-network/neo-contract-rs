# Getting Started with Neo Contract Rust

This guide will walk you through setting up your development environment and creating your first Neo smart contract using the Neo Contract Rust Framework.

## Prerequisites

Before you begin, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (stable or nightly)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- [Git](https://git-scm.com/downloads)
- [Neo CLI](https://docs.neo.org/docs/en-us/node/cli/setup.html) (for deployment and testing)

## Installation

### 1. Install WebAssembly Toolchain

First, add the WebAssembly target to your Rust toolchain:

```bash
rustup target add wasm32-unknown-unknown
```

### 2. Install Required Tools

Install the necessary development tools:

```bash
# Install cargo-make for build automation
cargo install cargo-make

# Install wasm-strip for optimizing WebAssembly binaries
cargo install wasm-strip
```

### 3. Clone the Neo Contract Rust Repository

```bash
git clone https://github.com/neo-project/neo-contract-rs
cd neo-contract-rs
```

### 4. Build the Framework

```bash
cargo build --release
```

### 5. Add the Framework to Your Path (Optional)

For easier access to the `neo-compiler` tool:

```bash
# For Unix-like systems (Linux, macOS)
export PATH=$PATH:$(pwd)/target/release

# For Windows (CMD)
set PATH=%PATH%;%CD%\target\release

# For Windows (PowerShell)
$env:PATH += ";$(Get-Location)\target\release"
```

## Creating Your First Contract

### 1. Create a New Project

```bash
# Create a new Rust library project
cargo new --lib hello-neo
cd hello-neo
```

### 2. Configure Cargo.toml

Update your `Cargo.toml` file to include the necessary dependencies and configurations:

```toml
[package]
name = "hello-neo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
neo-contract = { path = "../path/to/neo-contract-rs/neo-contract" }
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
# For testing
neo-contract = { path = "../path/to/neo-contract-rs/neo-contract", features = ["mock"] }

[features]
std = ["neo-contract/std"]
mock = ["neo-contract/mock"]
default = ["std"]

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
```

### 3. Write Your First Contract

Create a simple greeting contract in `src/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use neo_contract::prelude::*;

#[contract]
pub struct HelloNeo {
    greeting: StorageItem<String>,
}

#[contractimpl]
impl HelloNeo {
    #[constructor]
    pub fn new() -> Self {
        Self {
            greeting: StorageItem::new("Hello, Neo Smart Contract!".to_string()),
        }
    }
    
    #[constructor]
    pub fn with_greeting(greeting: String) -> Self {
        Self {
            greeting: StorageItem::new(greeting),
        }
    }
    
    #[method]
    pub fn get_greeting(&self) -> String {
        self.greeting.get()
    }
    
    #[method]
    pub fn set_greeting(&mut self, new_greeting: String) {
        self.greeting.set(new_greeting);
    }
    
    #[method]
    pub fn greet_person(&self, name: String) -> String {
        let greeting = self.greeting.get();
        format!("{} Nice to meet you, {}!", greeting, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_greeting() {
        let contract = HelloNeo::new();
        assert_eq!(
            contract.get_greeting(),
            "Hello, Neo Smart Contract!"
        );
    }
    
    #[test]
    fn test_custom_greeting() {
        let contract = HelloNeo::with_greeting("Howdy, Neo!".to_string());
        assert_eq!(contract.get_greeting(), "Howdy, Neo!");
    }
    
    #[test]
    fn test_set_greeting() {
        let mut contract = HelloNeo::new();
        contract.set_greeting("Welcome to Neo Blockchain!".to_string());
        assert_eq!(
            contract.get_greeting(),
            "Welcome to Neo Blockchain!"
        );
    }
    
    #[test]
    fn test_greet_person() {
        let contract = HelloNeo::new();
        assert_eq!(
            contract.greet_person("Alice".to_string()),
            "Hello, Neo Smart Contract! Nice to meet you, Alice!"
        );
    }
}
```

### 4. Understanding the Contract

Let's break down the key components of this contract:

- `#[contract]`: This attribute marks your struct as a smart contract.
- `#[contractimpl]`: This marks the implementation block for your contract.
- `#[constructor]`: These are special methods that initialize your contract state.
- `#[method]`: These are public methods that can be called from outside the contract.
- `StorageItem<T>`: A storage primitive that persists data on the blockchain.

### 5. Running Tests

Before compiling for deployment, run the tests to ensure your contract works correctly:

```bash
cargo test
```

## Compiling and Deploying Your Contract

### 1. Compile to WebAssembly

```bash
cargo build --target wasm32-unknown-unknown --release
```

This command compiles your Rust contract to WebAssembly, creating a `.wasm` file in the `target/wasm32-unknown-unknown/release/` directory.

### 2. Convert to NEF

Use the Neo Compiler to convert the WebAssembly file to a Neo Executable Format (NEF) file and generate a contract manifest:

```bash
neo-compiler compile \
    target/wasm32-unknown-unknown/release/hello_neo.wasm \
    --output ./build \
    --name "HelloNeo" \
    --author "Your Name" \
    --description "A simple greeting contract"
```

This command creates two important files in the `./build` directory:
- `HelloNeo.nef`: The Neo Executable Format file containing the contract bytecode
- `HelloNeo.manifest.json`: The contract manifest describing the contract's API and permissions

### 3. Deploy to a Neo Network

#### Deploy to Private Network

For development and testing, you can deploy to a private Neo network:

```bash
neo-cli deploy ./build/HelloNeo.nef ./build/HelloNeo.manifest.json
```

#### Deploy to Testnet

For testing in a public environment before mainnet deployment:

```bash
# Configure neo-cli for testnet
neo-cli config network testnet

# Deploy to testnet
neo-cli deploy ./build/HelloNeo.nef ./build/HelloNeo.manifest.json
```

## Interacting with Your Contract

After deployment, you can interact with your contract using Neo CLI or other tools.

### Using Neo CLI

```bash
# Invoke the get_greeting method
neo-cli invoke <contract-hash> getGreeting []

# Set a new greeting
neo-cli invoke <contract-hash> setGreeting ["Hello, Neo N3!"]

# Greet a person
neo-cli invoke <contract-hash> greetPerson ["Alice"]
```

Replace `<contract-hash>` with your actual contract hash from the deployment.

### Using a Neo SDK

Various SDKs are available to interact with Neo contracts from different languages:

#### Neo Java SDK:

```java
// Initialize Neow3j
Neow3j neow3j = Neow3j.build(new HttpService("http://localhost:10332"));

// Get the contract
SmartContract contract = new SmartContract(
    Hash160.fromAddress("<contract-address>"), 
    neow3j
);

// Call a read-only method
String greeting = contract.callFunctionReturningString("getGreeting");

// Call a method that modifies state (requires signing)
Hash256 txHash = contract.invokeFunction("setGreeting", 
    ContractParameter.string("Hello from Java!"))
    .signers(AccountSigner.calledByEntry(account))
    .sign()
    .send();
```

#### Neo Python SDK:

```python
from neo3.api.wrappers import NeoToken, ContractWrapper
from neo3.network import convenience
from neo3.core import cryptography

# Connect to a Neo node
node = convenience.auto_select_node(convenience.TestNetUrls)

# Load your contract
contract = ContractWrapper(
    contract_hash='<contract-script-hash>',
    node_url=node.url
)

# Call a read-only method
greeting = contract.get_greeting()
print(f"Current greeting: {greeting}")

# Call a method that modifies state
private_key = cryptography.ECPrivateKey.from_hex("<your-private-key>")
tx = contract.set_greeting("Hello from Python!", private_key=private_key)
print(f"Transaction ID: {tx.hash()}")
```

## Adding Features to Your Contract

### Events

Events allow your contract to notify external applications when something happens:

```rust
#[contract]
pub struct HelloNeo {
    greeting: StorageItem<String>,
}

#[event]
pub struct GreetingChanged {
    pub old_greeting: String,
    pub new_greeting: String,
}

#[contractimpl]
impl HelloNeo {
    // ... other methods
    
    #[method]
    pub fn set_greeting(&mut self, new_greeting: String) {
        let old_greeting = self.greeting.get();
        self.greeting.set(new_greeting.clone());
        
        // Emit an event
        emit!(GreetingChanged {
            old_greeting,
            new_greeting,
        });
    }
}
```

### Storage Maps

For more complex data structures, use `StorageMap`:

```rust
#[contract]
pub struct UserGreetings {
    greetings: StorageMap<Address, String>,
    default_greeting: StorageItem<String>,
}

#[contractimpl]
impl UserGreetings {
    #[constructor]
    pub fn new() -> Self {
        Self {
            greetings: StorageMap::new(),
            default_greeting: StorageItem::new("Hello, friend!".to_string()),
        }
    }
    
    #[method]
    pub fn set_user_greeting(&mut self, user: Address, greeting: String) {
        self.greetings.insert(&user, greeting);
    }
    
    #[method]
    pub fn get_user_greeting(&self, user: Address) -> String {
        self.greetings.get(&user).unwrap_or_else(|| self.default_greeting.get())
    }
}
```

### Access Control

Implement access control to restrict who can call certain methods:

```rust
#[contract]
pub struct AccessControlled {
    owner: StorageItem<Address>,
    data: StorageItem<String>,
}

#[contractimpl]
impl AccessControlled {
    #[constructor]
    pub fn new() -> Self {
        let sender = runtime::current_sender();
        Self {
            owner: StorageItem::new(sender),
            data: StorageItem::new("Initial data".to_string()),
        }
    }
    
    #[method]
    pub fn set_data(&mut self, new_data: String) {
        let sender = runtime::current_sender();
        let owner = self.owner.get();
        
        // Only the owner can set data
        assert!(sender == owner, "Only the owner can set data");
        
        self.data.set(new_data);
    }
    
    #[method]
    pub fn get_data(&self) -> String {
        self.data.get()
    }
}
```

## Best Practices

### 1. Security Considerations

- Use `#[no_reentrant]` for methods that modify state to prevent reentrancy attacks
- Always validate inputs and check for overflows
- Use access control to restrict sensitive operations
- Test thoroughly, including edge cases

### 2. Gas Optimization

- Minimize storage operations, which are expensive
- Batch storage operations when possible
- Use appropriate data structures to minimize gas costs
- Test gas consumption for complex operations

### 3. Upgrade Strategy

Plan for contract upgrades from the beginning:

```rust
#[contract]
pub struct UpgradeableContract {
    owner: StorageItem<Address>,
    version: StorageItem<u32>,
    data: StorageItem<String>,
}

#[contractimpl]
impl UpgradeableContract {
    // ... other methods
    
    #[method]
    pub fn upgrade(&mut self, script: Vec<u8>, manifest: Vec<u8>) -> bool {
        let sender = runtime::current_sender();
        let owner = self.owner.get();
        
        // Only the owner can upgrade
        assert!(sender == owner, "Only the owner can upgrade");
        
        // Perform the upgrade
        contract::update(script, manifest);
        
        // Increment version (this will be preserved after upgrade)
        let current_version = self.version.get();
        self.version.set(current_version + 1);
        
        true
    }
}
```

## Debugging and Troubleshooting

### Common Issues

#### 1. Compilation Errors

If you encounter compilation errors, check:
- Correct use of attributes (`#[contract]`, `#[method]`, etc.)
- Proper implementations of required traits
- Correct types for parameters and return values

#### 2. Runtime Errors

For runtime errors:
- Use events for debugging information
- Add detailed error messages to assertions
- Test thoroughly before deployment

#### 3. Gas Estimation

If transactions run out of gas:
- Optimize storage operations
- Simplify complex logic
- Test with different inputs to understand gas usage

### Development Tools

Enhance your development workflow with these tools:

#### 1. Neo Express

[Neo Express](https://github.com/neo-project/neo-express) provides a simplified development environment:

```bash
# Create a private chain
neoxp create

# Start the chain
neoxp run

# Deploy your contract
neoxp contract deploy ./build/HelloNeo.nef
```

#### 2. NeoTrace

Use the tracing features to understand your contract's execution:

```bash
neoxp trace HelloNeo getGreeting
```

## Next Steps

Now that you've built your first Neo smart contract, consider exploring:

1. [NEP-17 Tutorial](nep17_tutorial.md) to create fungible tokens
2. [Storage and Variables](storage_and_variables.md) for advanced state management
3. [Testing Guide](testing_guide.md) for comprehensive testing strategies
4. [Neo Compiler Implementation](neo_compiler_implementation.md) to understand the compilation process
5. Experimenting with the example contracts in the repository

## Community Resources

- [Neo Documentation](https://docs.neo.org/)
- [Neo GitHub](https://github.com/neo-project/)
- [Neo Discord](https://discord.gg/tpUNQNy)
- [Neo Reddit](https://www.reddit.com/r/NEO/)

Happy coding with Neo Contract Rust!
