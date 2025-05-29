# Getting Started with Neo N3 Rust Smart Contract Framework

Welcome to the **Neo N3 Rust Smart Contract Framework**! This guide will help you set up your development environment and create your first Neo N3 smart contract using Rust with proper NEF and manifest generation.

## 🎯 **Prerequisites**

Before starting, ensure you have:

1. **[Rust](https://www.rust-lang.org/tools/install)** installed (1.70+ recommended)
2. **[Git](https://git-scm.com/)** for cloning the repository
3. **Basic knowledge** of Rust and Neo blockchain concepts
4. **Neo N3 node** or testnet access for deployment (optional)

## 🚀 **Quick Start**

### 1. Clone the Framework

```bash
git clone https://github.com/R3E-Network/neo-contract-rs.git
cd neo-contract-rs
```

### 2. Install Dependencies

```bash
# Install Rust nightly toolchain and WASM target
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Verify installation
rustc --version
cargo --version
```

### 3. Build All Examples

```bash
# Build all 13 examples with proper NEF and manifest generation
make build-all

# Or build individual examples
cd examples/01-hello-world
make                    # Build everything (WASM → NEF + Manifest)
```

## 📦 **Framework Structure**

The framework includes:

- **📁 `neo-contract/`** - Core Rust smart contract library
- **📁 `neo-contract-proc-macros/`** - Procedural macros for contract attributes
- **📁 `neo-wasm/`** - WASM to NEF compiler
- **📁 `examples/`** - 13 complete working examples
- **📁 `docs/`** - Comprehensive documentation
- **📁 `website/`** - Project website

## 🔧 **Build System**

Each example uses a **professional Makefile** with proper neo-wasm integration:

```bash
# Available build targets
make help           # Show all available targets
make build          # Build Rust to WASM
make nef            # Generate NEF using neo-wasm compiler
make manifest       # Generate manifest using neo-wasm compiler
make all            # Build everything (default)
make clean          # Clean build artifacts
make test           # Run tests
```

## 🎯 **Creating Your First Contract**

### 1. Use an Example as Template

The fastest way to start is by copying an existing example:

```bash
# Copy the hello-world example
cp -r examples/01-hello-world my-contract
cd my-contract

# Update the project name in Cargo.toml
sed -i 's/01-hello-world/my-contract/g' Cargo.toml
```

### 2. Modify the Contract

Edit `src/lib.rs` to create your custom contract:

```rust
#![no_std]
#![no_main]

use neo_contract::prelude::*;

#[contract_author("Your Name", "your.email@example.com")]
#[contract_version("1.0.0")]
#[contract_permission("*", "*")]
#[contract_meta("description", "My first Neo N3 smart contract")]
#[contract]
pub struct MyContract {
    // Storage for contract state
    greeting: StorageItem<ByteString>,
    visitor_count: StorageItem<u64>,
}

#[contract_impl]
impl MyContract {
    /// Initialize the contract
    pub fn init() -> Self {
        let ctx = Storage::get_context();
        Self {
            greeting: StorageItem::new(ctx, b"greeting"),
            visitor_count: StorageItem::new(ctx, b"visitor_count"),
        }
    }

    /// Get the current greeting
    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        self.greeting.get().unwrap_or_else(|| ByteString::from("Hello, Neo N3!"))
    }

    /// Set a new greeting (requires authorization)
    #[method]
    pub fn set_greeting(&self, new_greeting: ByteString) -> bool {
        // Check if caller is authorized
        if !Runtime::check_witness(Runtime::calling_script_hash()) {
            return false;
        }

        self.greeting.put(new_greeting.clone());

        // Emit event
        Runtime::notify(&[
            ByteString::from("GreetingChanged").into(),
            new_greeting.into()
        ]);

        true
    }

    /// Say hello and increment visitor count
    #[method]
    pub fn say_hello(&self, visitor_name: ByteString) -> ByteString {
        // Increment visitor count
        let count = self.visitor_count.get().unwrap_or(0) + 1;
        self.visitor_count.put(count);

        // Create personalized greeting
        let greeting = self.get_greeting();
        let response = format!("{} Welcome, {}! You are visitor #{}",
                              greeting.to_string(),
                              visitor_name.to_string(),
                              count);

        // Emit event
        Runtime::notify(&[
            ByteString::from("VisitorGreeted").into(),
            visitor_name.into(),
            count.into()
        ]);

        ByteString::from(response)
    }

    /// Get the total visitor count
    #[method]
    #[safe]
    pub fn get_visitor_count(&self) -> u64 {
        self.visitor_count.get().unwrap_or(0)
    }
}
```

### 3. Build Your Contract

```bash
# Build everything (WASM → NEF + Manifest)
make

# Or step by step
make build          # Build Rust to WASM
make nef            # Generate NEF file
make manifest       # Generate manifest file

# Check generated files
ls -la build/
```

## 🪙 **Exploring Examples**

The framework includes 13 complete examples covering various use cases:

### **Token Standards**
- **`04-nep17-token`** - Fungible token (like ERC-20)
- **`05-nep11-nft`** - Non-fungible token (like ERC-721)
- **`06-nep24-royalty-nft`** - NFT with royalty support

### **DeFi Applications**
- **`07-crowdfunding`** - Crowdfunding platform
- **`08-staking`** - Token staking mechanism
- **`09-simple-dex`** - Decentralized exchange

### **Advanced Contracts**
- **`10-multisig-wallet`** - Multi-signature wallet
- **`11-governance`** - DAO governance system
- **`12-oracle-price-feed`** - Oracle integration
- **`13-nft-marketplace`** - NFT trading platform

### **Basic Examples**
- **`01-hello-world`** - Basic contract structure
- **`02-simple-storage`** - Storage operations
- **`03-counter`** - State management

## 🪙 **NEP-17 Token Example**

Here's how to build a NEP-17 token using the framework:

```bash
# Use the complete NEP-17 example
cd examples/04-nep17-token

# Build the token contract
make

# Check the generated files
ls -la build/
# Output:
# 04-nep17-token.nef          - NEF file for deployment
# 04-nep17-token.manifest.json - Contract manifest with ABI
```

The NEP-17 example includes:
- ✅ **Standard Methods**: `symbol()`, `decimals()`, `totalSupply()`, `balanceOf()`, `transfer()`
- ✅ **Events**: `Transfer` event emission
- ✅ **Authorization**: Proper witness checking
- ✅ **Storage**: Efficient balance and metadata storage
- ✅ **Manifest**: Auto-generated with proper ABI

## 💾 **Storage Patterns**

The framework provides efficient storage abstractions:

```rust
#[contract]
pub struct MyContract {
    // Single value storage
    config: StorageItem<ByteString>,

    // Key-value mapping
    user_data: StorageMap<H160, UserInfo>,

    // Nested mappings
    balances: StorageMap<(H160, H160), u64>,
}
```

See `examples/02-simple-storage` for complete storage examples.

## 🔔 **Events and Notifications**

Emit events to notify external systems:

```rust
// Simple notification
Runtime::notify(&[
    ByteString::from("EventName").into(),
    param1.into(),
    param2.into()
]);

// Structured event
Runtime::log(&ByteString::from("Contract executed successfully"));
```

## 🧪 **Testing Your Contract**

```bash
# Run unit tests
cd examples/04-nep17-token
make test

# Check code without building
make check
```

The framework includes mock environments for testing contract logic.

## 🚀 **Deployment Process**

### 1. Build Deployment Artifacts

```bash
# Generate NEF and manifest files
make

# Verify generated files
ls -la build/
# my-contract.nef          - Neo executable format
# my-contract.manifest.json - Contract metadata and ABI
```

### 2. Deploy to Neo N3

```bash
# Using Neo-CLI
neo-cli> deploy build/my-contract.nef build/my-contract.manifest.json

# Or using Neo-Express for testing
neoxp contract deploy build/my-contract.nef
```

## 📚 **Next Steps**

### **Explore Advanced Examples**
- **DeFi**: Check `examples/09-simple-dex` for DEX implementation
- **NFTs**: Explore `examples/05-nep11-nft` for NFT contracts
- **Governance**: See `examples/11-governance` for DAO patterns

### **Learn More**
- 📖 **[Architecture Guide](architecture.md)** - Framework architecture
- 🔧 **[API Reference](api-reference.md)** - Complete API documentation
- 🎯 **[Contract Attributes](contract-attributes.md)** - Metadata and permissions
- 🔒 **[Safe Methods](safe-methods.md)** - Read-only method patterns

### **Advanced Topics**
- 🌐 **[Oracle Integration](oracle-framework.md)** - External data access
- 📊 **[Manifest Generation](manifest-generation.md)** - ABI and metadata
- 🔄 **[Syscall Implementation](neo-syscall-implementation.md)** - Low-level operations

## 🎯 **Summary**

You now have:
- ✅ **Working development environment**
- ✅ **Understanding of the build system**
- ✅ **Knowledge of contract structure**
- ✅ **Access to 13 complete examples**
- ✅ **Deployment-ready artifacts**

**🎉 Start building amazing Neo N3 smart contracts with Rust!**

For questions and support, visit our [GitHub repository](https://github.com/R3E-Network/neo-contract-rs).