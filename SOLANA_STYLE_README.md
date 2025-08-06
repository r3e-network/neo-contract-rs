# Neo N3 Smart Contracts with Solana-Style Syntax

## 🚀 Overview

This Neo N3 Rust framework now supports **Solana-style syntax** alongside the traditional Neo syntax, providing developers with a choice of programming paradigms. The Solana-style syntax brings the ergonomic patterns from Solana's Anchor framework to Neo N3, making it easier for Solana developers to build on Neo.

## 🎯 Why Solana-Style Syntax?

- **Familiar to Solana Developers**: Use the same mental model and patterns
- **Type-Safe Account Validation**: Compile-time checking of account constraints
- **Explicit Security**: Clear declaration of account requirements and permissions
- **Better Error Handling**: Structured error types with automatic code generation
- **Modern Patterns**: Context-based programming model with clear separation of concerns

## 📚 Quick Comparison

### Traditional Neo N3 Style
```rust
#[contract_impl]
impl HelloWorld {
    pub fn init() -> Self {
        Self {
            greeting_key: ByteString::from_literal("greeting"),
        }
    }
    
    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        // Implementation
    }
}
```

### Solana-Style Syntax
```rust
declare_id!("NeoContractAddress123");

#[program]
pub mod hello_world {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, greeting: ByteString) -> Result<()> {
        ctx.accounts.state.greeting = greeting;
        Ok(())
    }
    
    #[safe]
    pub fn get_greeting(ctx: Context<GetGreeting>) -> Result<ByteString> {
        Ok(ctx.accounts.state.greeting.clone())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 100)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

## 🛠️ Key Features

### 1. Program Module Pattern
- Use `#[program]` to define your contract module
- All instruction handlers are public functions within the module
- Automatic WASM export generation

### 2. Account Validation
- `#[derive(Accounts)]` for automatic validation
- Built-in constraints: `init`, `mut`, `has_one`, `seeds`, etc.
- Type-safe account access through context

### 3. Error Handling
- `#[derive(ErrorCode)]` for custom error types
- Automatic error code generation
- Rich error messages with `#[msg("...")]` attribute

### 4. Events
- `#[event]` attribute for event structs
- `emit!` macro for event emission
- Structured event data with automatic serialization

## 📦 Available Examples

### Solana-Style Examples
1. **Hello World** (`examples/01-hello-world-solana-style/`)
   - Basic contract with state management
   - Account validation patterns
   - Event emission

2. **NEP-17 Token** (`examples/04-nep17-token-solana-style/`)
   - Full token implementation
   - Transfer, mint, burn operations
   - Allowance system
   - Freeze/thaw functionality

### Traditional Style Examples
All existing examples in `examples/` directory continue to work with traditional syntax.

## 🚀 Getting Started

### 1. Choose Your Style

Both syntaxes compile to the same Neo N3 bytecode and are fully compatible with the Neo N3 blockchain.

### 2. Use the Appropriate Macros

**Solana-Style Macros:**
- `#[program]` - Define program module
- `#[derive(Accounts)]` - Account validation
- `#[account]` - Account data structures
- `#[derive(ErrorCode)]` - Error handling
- `#[event]` - Event definitions
- `declare_id!` - Program ID declaration

**Traditional Macros:**
- `#[contract_impl]` - Contract implementation
- `#[method]` - Method exposure
- `#[safe]` - Read-only methods
- Contract metadata attributes

### 3. Build Your Contract

```bash
# For any example
cd examples/01-hello-world-solana-style
make build

# Or use cargo directly
cargo build --target wasm32-unknown-unknown --release
```

## 📖 Documentation

- [Solana-Style Syntax Guide](docs/solana-style-syntax-guide.md) - Comprehensive guide for Solana-style syntax
- [Traditional Syntax Guide](docs/getting-started.md) - Original Neo N3 syntax documentation
- [Migration Guide](docs/solana-style-syntax-guide.md#migration-guide) - How to migrate between styles

## 🔄 Interoperability

Both syntax styles:
- Compile to identical Neo N3 bytecode
- Are fully compatible with Neo N3 tools and infrastructure
- Can interact with each other on-chain
- Support the same Neo N3 features (storage, native contracts, etc.)

## 🎯 When to Use Which Style?

### Use Solana-Style When:
- You're coming from Solana development
- You prefer explicit account validation
- You want compile-time security checks
- You like the Context pattern
- You need complex account relationships

### Use Traditional Style When:
- You're familiar with Neo N3 patterns
- You prefer simpler, more direct syntax
- You're migrating existing Neo contracts
- You want minimal abstraction

## 🤝 Contributing

We welcome contributions for both syntax styles! Please ensure:
- Examples are provided for new features
- Documentation is updated
- Tests pass for both styles
- Code follows the existing patterns

## 📄 License

This framework maintains the same license as the original neo-contract-rs project.

## 🙏 Acknowledgments

- Solana's Anchor framework for inspiration
- Neo community for feedback and support
- Contributors to both syntax styles

---

**Note**: This is a Neo N3 framework with Solana-style syntax support. It compiles to Neo N3 bytecode and runs on the Neo N3 blockchain. It is not affiliated with Solana Labs or the Solana blockchain.