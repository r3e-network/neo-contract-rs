# Documented Token Example

This example demonstrates how to write comprehensive documentation for a NEO smart contract that will be properly extracted by the manifest generator, while following best practices for efficiency.

## Features

- Implementation of the NEP-17 (Fungible Token) standard
- Well-documented code with comprehensive doc comments
- Example of manifest generation with rich documentation
- Efficient storage patterns for optimal gas usage
- Safe method annotations for read-only functions

## Documentation Features

This contract demonstrates:

1. **Contract-level documentation** - The main contract has detailed documentation explaining its purpose and features
2. **Method documentation** - Each exported method has comprehensive documentation
3. **Parameter documentation** - Parameters are documented with meaningful descriptions
4. **Return value documentation** - Return values include type and meaning descriptions
5. **Safe annotations** - Read-only methods are marked with `@safe` annotations

## Efficiency Features

This contract showcases several efficiency best practices:

1. **Optimized Storage Access** - Reads all data before performing computations
2. **Storage Key Design** - Uses clear prefixes for different storage types
3. **Safe Arithmetic** - Prevents overflows with checked arithmetic operations
4. **Efficient String Handling** - Uses pre-sized format strings for output
5. **Memory Optimization** - Minimizes unnecessary allocations

## Building the Contract

```bash
cargo build --release --target wasm32-unknown-unknown
```

The resulting WASM file will be in `target/wasm32-unknown-unknown/release/documented_token.wasm`.

## Generating the Manifest

After building the contract, you can generate the manifest using the neo-wasm tool:

```bash
neo-wasm target/wasm32-unknown-unknown/release/documented_token.wasm
```

This will generate:
- `documented_token.nef` - The NEO Executable Format file
- `documented_token.manifest.json` - The contract manifest with extracted documentation

## Examining the Manifest

Open the generated manifest file to see how the documentation has been incorporated:

1. Contract description in the `Extra.Description` field
2. Method descriptions in the `Extra.MethodDescriptions` field
3. Parameter names preserved in the ABI
4. Methods properly marked as safe or unsafe

## Key Documentation Practices Demonstrated

1. **Contract Documentation** - Comprehensive doc comment above the `#[contract]` impl block
2. **Method Documentation** - Each public method has a doc comment explaining its purpose
3. **Parameter Documentation** - Parameters documented with name and purpose
4. **Return Value Documentation** - Return values documented with type and meaning
5. **@safe Annotations** - Read-only methods explicitly marked as safe
6. **Code Structure** - Clear code organization with constants section and logical method grouping

## Learning from this Example

This example demonstrates how to:

1. Write clean, efficient smart contract code
2. Document your code for optimal manifest generation
3. Follow NEO smart contract best practices
4. Mark methods as safe to save gas for users
5. Optimize storage operations for better performance

For more details on efficient contract writing, see the [Efficient Smart Contracts](../../docs/efficient-contracts.md) guide.