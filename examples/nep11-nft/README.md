# NEP-11 Non-Fungible Token Example

A basic implementation of the NEP-11 Non-Fungible Token standard for Neo N3 blockchain using the neo-contract-rs framework.

## Overview

This example demonstrates how to implement a non-divisible NFT contract following the NEP-11 standard. Features include:

1. Token minting and burning
2. Token ownership and transfer
3. Token metadata storage
4. Token enumeration

## Contract Methods

### Core NEP-11 Methods

#### `symbol() -> ByteString`

Returns the token symbol ("BNFT").

#### `decimals() -> u32`

Returns the number of decimals (0 for non-divisible NFTs).

#### `total_supply() -> Int256`

Returns the total number of tokens in existence.

#### `balance_of(owner: H160) -> Int256`

Returns the number of tokens owned by a specific account.

#### `owner_of(token_id: ByteString) -> H160`

Returns the owner of a specific token.

#### `tokens() -> Array<ByteString>`

Returns an array of all token IDs.

#### `tokens_of(owner: H160) -> Array<ByteString>`

Returns an array of token IDs owned by a specific account.

#### `transfer(to: H160, token_id: ByteString, data: Any) -> bool`

Transfers ownership of a token to another account.

#### `properties(token_id: ByteString) -> Map<ByteString, ByteString>`

Returns metadata associated with a specific token.

### Additional Methods

#### `mint(owner: H160, token_id: ByteString, props: Array<ByteString>) -> bool`

Creates a new token and assigns ownership to the specified account.

Parameters:
- `owner`: The address that will own the new token
- `token_id`: The unique identifier for the token
- `props`: An array of alternating key-value pairs for token properties

#### `burn(token_id: ByteString) -> bool`

Destroys a token.

## Storage Structure

The contract uses the following storage structure:

- `PREFIX_OWNER (0x01)`: Maps token ID → owner
- `PREFIX_TOKEN (0x02)`: Maps owner → set of token IDs
- `PREFIX_PROPERTIES (0x03)`: Maps token ID → properties
- `PREFIX_TOKEN_LIST (0x04)`: List of all token IDs
- `TOTAL_SUPPLY_KEY (0x00)`: Key for total supply

## Building the Contract

```bash
# Make sure you have the wasm32-unknown-unknown target installed
rustup target add wasm32-unknown-unknown

# Build using make
make

# Or build manually
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/nep11_nft.wasm ./nep11_nft.wasm
```

## Deploying the Contract

After building, you can deploy the contract to a Neo N3 network using Neo-CLI:

```bash
neo-cli deploy nep11_nft.wasm
```

## Example Usage

Here are some examples of how to interact with the contract:

```bash
# Mint a new token
neo-cli invoke <contract-hash> mint ["NcASgY8WpMrDJLqPjV4SctvrMsHR2rAkmD","token1",["name","My NFT","description","An example NFT"]]

# Check token ownership
neo-cli invoke <contract-hash> owner_of ["token1"]

# Get token properties
neo-cli invoke <contract-hash> properties ["token1"]

# Transfer a token
neo-cli invoke <contract-hash> transfer ["NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF","token1",null]

# Burn a token
neo-cli invoke <contract-hash> burn ["token1"]
```

## Serialization Implementation

This example includes a simple but complete implementation of binary serialization for:

1. Token arrays: Used for storing lists of token IDs
2. Property maps: Used for storing token metadata

The serialization format uses length-prefixed encoding for collections and strings.

## Events

The contract emits `Transfer` events for all token transfers, including minting and burning operations, following the NEP-11 standard.

## Security Considerations

1. Only the token owner can transfer or burn their tokens
2. Only the contract owner can mint new tokens
3. All operations include proper validation of inputs

## Extending the Example

This example can be extended in several ways:

1. Add support for token URIs pointing to off-chain metadata
2. Implement royalty payments for creators
3. Add custom access controls for minting and burning
4. Implement token collection metadata
5. Add support for token approvals and operator approvals 