#![cfg(test)]

extern crate alloc;

use alloc::string::ToString;
use neo_contract::prelude::*;
use neo_contract::storage::test_utils::MockStorage;

// Import our hybrid token implementation
// In a real project, you would import the actual module
// For this example, we'll redefine simplified test versions

// Simple event tracking for tests following ink! style
pub struct EventLog {
    name: ByteString,
    params: Array<Any>,
}

// Test contract implementation using ink! style
struct HybridTokenContract {
    owner: H160,
}

impl HybridTokenContract {
    // Create a new contract instance
    #[constructor]
    fn new(owner: H160, name: &str, symbol: &str, decimals: u8) -> Self {
        // Clear storage before initializing
        MockStorage::clear();

        // Store contract metadata
        MockStorage::put(b"owner", &owner.to_vec());
        MockStorage::put(b"name", name.as_bytes());
        MockStorage::put(b"symbol", symbol.as_bytes());
        MockStorage::put(b"decimals", &[decimals]);

        // Initialize supplies to zero
        MockStorage::put(b"total_supply", &Int256::zero().to_vec());
        MockStorage::put(b"total_nft", &Int256::zero().to_vec());

        Self { owner }
    }

    //======== Utility methods for testing ========//

    // Mock emitting an event (in real contract this uses Runtime::notify)
    #[safe]
    fn emit_event(&self, name: &str, params: Array<Any>) {
        // In a test environment, we'll store events in a specific storage area
        let events_key = b"test_events";
        let mut events = Vec::new();

        if MockStorage::contains_key(events_key) {
            events = MockStorage::get(events_key);
        }

        // Append this event (simplified storage for testing)
        let mut new_events = events.clone();
        new_events.extend_from_slice(name.as_bytes());
        new_events.push(b':');

        // In a real test we would properly serialize parameters
        // Here we just add a placeholder
        new_events.extend_from_slice(b"event_params");

        MockStorage::put(events_key, &new_events);
    }

    // Get events for verification
    #[safe]
    fn get_events(&self) -> Vec<u8> {
        let events_key = b"test_events";
        if MockStorage::contains_key(events_key) {
            MockStorage::get(events_key)
        } else {
            Vec::new()
        }
    }

    //======== NEP-17 (FT) methods ========//

    // Get fungible token total supply
    #[safe]
    fn ft_total_supply(&self) -> Int256 {
        if !MockStorage::contains_key(b"total_supply") {
            return Int256::zero();
        }

        let data = MockStorage::get(b"total_supply");
        Int256::from_slice(&data).unwrap_or_else(|_| Int256::zero())
    }

    // Get fungible token balance
    #[safe]
    fn ft_balance_of(&self, owner: &H160) -> Int256 {
        let balance_key = self.make_ft_balance_key(owner);

        if !MockStorage::contains_key(&balance_key) {
            return Int256::zero();
        }

        let data = MockStorage::get(&balance_key);
        Int256::from_slice(&data).unwrap_or_else(|_| Int256::zero())
    }

    // Mint fungible tokens (only owner)
    #[method]
    fn ft_mint(&self, to: H160, amount: Int256) -> bool {
        // Check if caller is the owner (simplified for test)
        // In a real implementation, we'd check Runtime::check_witness

        // Update recipient balance
        let balance_key = self.make_ft_balance_key(&to);
        let current_balance = self.ft_balance_of(&to);
        let new_balance = current_balance + amount;
        MockStorage::put(&balance_key, &new_balance.to_vec());

        // Update total supply
        let current_supply = self.ft_total_supply();
        let new_supply = current_supply + amount;
        MockStorage::put(b"total_supply", &new_supply.to_vec());

        // Emit transfer event (mint is transfer from null address)
        let mut event_params = Array::<Any>::new();
        event_params.push(Any::new()); // null address for 'from'
        event_params.push(Any::from(to));
        event_params.push(Any::from(amount));

        self.emit_event("Transfer", event_params);

        true
    }

    // Transfer fungible tokens
    #[method]
    fn ft_transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
        // Check if sender has enough balance
        let from_balance = self.ft_balance_of(&from);
        if from_balance < amount {
            return false;
        }

        // Update balances
        let from_key = self.make_ft_balance_key(&from);
        let new_from_balance = from_balance - amount;

        if new_from_balance.is_zero() {
            MockStorage::delete(&from_key);
        } else {
            MockStorage::put(&from_key, &new_from_balance.to_vec());
        }

        let to_key = self.make_ft_balance_key(&to);
        let to_balance = self.ft_balance_of(&to);
        let new_to_balance = to_balance + amount;
        MockStorage::put(&to_key, &new_to_balance.to_vec());

        // Emit transfer event
        let mut event_params = Array::<Any>::new();
        event_params.push(Any::from(from));
        event_params.push(Any::from(to));
        event_params.push(Any::from(amount));

        self.emit_event("Transfer", event_params);

        true
    }

    //======== NEP-11 (NFT) methods ========//

    // Get non-fungible token total supply
    #[safe]
    fn nft_total_supply(&self) -> Int256 {
        if !MockStorage::contains_key(b"total_nft") {
            return Int256::zero();
        }

        let data = MockStorage::get(b"total_nft");
        Int256::from_slice(&data).unwrap_or_else(|_| Int256::zero())
    }

    // Get non-fungible token balance
    #[safe]
    fn nft_balance_of(&self, owner: &H160) -> Int256 {
        let balance_key = self.make_nft_balance_key(owner);

        if !MockStorage::contains_key(&balance_key) {
            return Int256::zero();
        }

        let data = MockStorage::get(&balance_key);
        Int256::from_slice(&data).unwrap_or_else(|_| Int256::zero())
    }

    // Get owner of a token
    #[safe]
    fn nft_owner_of(&self, token_id: &[u8]) -> Option<H160> {
        let owner_key = self.make_nft_owner_key(token_id);

        if !MockStorage::contains_key(&owner_key) {
            return None;
        }

        let data = MockStorage::get(&owner_key);
        H160::from_slice(&data).ok()
    }

    // Get tokens of an owner
    #[safe]
    fn nft_tokens_of(&self, owner: &H160) -> Vec<Vec<u8>> {
        let tokens_key = self.make_nft_tokens_key(owner);

        if !MockStorage::contains_key(&tokens_key) {
            return Vec::new();
        }

        // In a real implementation, we'd properly deserialize an array structure
        // For this test, we'll use a simplified approach
        let tokens_data = MockStorage::get(&tokens_key);

        // Split tokens by a separator (simplified for test)
        let mut tokens = Vec::new();
        let mut current_token = Vec::new();

        for byte in tokens_data {
            if byte == b',' {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            } else {
                current_token.push(byte);
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        tokens
    }

    // Mint a new non-fungible token
    #[method]
    fn nft_mint(&self, to: H160, token_id: &[u8], name: &str, description: &str) -> bool {
        // Check if token already exists
        if self.nft_owner_of(token_id).is_some() {
            return false;
        }

        // Store token data
        let token_key = self.make_nft_token_key(token_id);
        let mut token_data = Vec::new();
        token_data.extend_from_slice(name.as_bytes());
        token_data.push(b':');
        token_data.extend_from_slice(description.as_bytes());
        MockStorage::put(&token_key, &token_data);

        // Store token ownership
        let owner_key = self.make_nft_owner_key(token_id);
        MockStorage::put(&owner_key, &to.to_vec());

        // Update owner's balance
        let balance_key = self.make_nft_balance_key(&to);
        let balance = self.nft_balance_of(&to);
        let new_balance = balance + Int256::one();
        MockStorage::put(&balance_key, &new_balance.to_vec());

        // Update owner's tokens
        let tokens_key = self.make_nft_tokens_key(&to);
        let mut tokens_data = Vec::new();

        if MockStorage::contains_key(&tokens_key) {
            tokens_data = MockStorage::get(&tokens_key);
            // Add separator if there are existing tokens
            if !tokens_data.is_empty() {
                tokens_data.push(b',');
            }
        }

        tokens_data.extend_from_slice(token_id);
        MockStorage::put(&tokens_key, &tokens_data);

        // Update total supply
        let current_supply = self.nft_total_supply();
        let new_supply = current_supply + Int256::one();
        MockStorage::put(b"total_nft", &new_supply.to_vec());

        // Emit NFT transfer event (mint is transfer from null)
        let mut event_params = Array::<Any>::new();
        event_params.push(Any::new()); // null address for 'from'
        event_params.push(Any::from(to));

        // In a real implementation, we'd properly convert token_id to ByteString
        // Here we'll just use the bytes directly
        let token_bytes = ByteString::from_bytes(token_id);
        event_params.push(Any::from(token_bytes));
        event_params.push(Any::from(Int256::one()));

        self.emit_event("Transfer", event_params);

        true
    }

    // Transfer a non-fungible token
    #[method]
    fn nft_transfer(&self, from: H160, to: H160, token_id: &[u8]) -> bool {
        // Check current ownership
        let owner = match self.nft_owner_of(token_id) {
            Some(o) => o,
            None => return false,
        };

        if owner != from {
            return false;
        }

        // Update token ownership
        let owner_key = self.make_nft_owner_key(token_id);
        MockStorage::put(&owner_key, &to.to_vec());

        // Update balances
        let from_balance_key = self.make_nft_balance_key(&from);
        let from_balance = self.nft_balance_of(&from);
        let new_from_balance = from_balance - Int256::one();

        if new_from_balance.is_zero() {
            MockStorage::delete(&from_balance_key);
        } else {
            MockStorage::put(&from_balance_key, &new_from_balance.to_vec());
        }

        let to_balance_key = self.make_nft_balance_key(&to);
        let to_balance = self.nft_balance_of(&to);
        let new_to_balance = to_balance + Int256::one();
        MockStorage::put(&to_balance_key, &new_to_balance.to_vec());

        // Update token collections
        self.remove_token_from_owner(&from, token_id);
        self.add_token_to_owner(&to, token_id);

        // Emit NFT transfer event
        let mut event_params = Array::<Any>::new();
        event_params.push(Any::from(from));
        event_params.push(Any::from(to));

        // In a real implementation, we'd properly convert token_id to ByteString
        let token_bytes = ByteString::from_bytes(token_id);
        event_params.push(Any::from(token_bytes));
        event_params.push(Any::from(Int256::one()));

        self.emit_event("Transfer", event_params);

        true
    }

    //======== Hybrid Operations ========//

    // Convert fungible tokens to a non-fungible token
    #[method]
    fn convert_ft_to_nft(&self, from: H160, token_id: &[u8], name: &str, description: &str, ft_amount: Int256) -> bool {
        // Check if the user has enough FT
        let from_balance = self.ft_balance_of(&from);
        if from_balance < ft_amount {
            return false;
        }

        // Check if token ID is available
        if self.nft_owner_of(token_id).is_some() {
            return false;
        }

        // Reduce FT balance (burn)
        let from_key = self.make_ft_balance_key(&from);
        let new_from_balance = from_balance - ft_amount;

        if new_from_balance.is_zero() {
            MockStorage::delete(&from_key);
        } else {
            MockStorage::put(&from_key, &new_from_balance.to_vec());
        }

        // Update FT total supply
        let ft_supply = self.ft_total_supply();
        let new_ft_supply = ft_supply - ft_amount;
        MockStorage::put(b"total_supply", &new_ft_supply.to_vec());

        // Emit FT transfer event (burn)
        let mut ft_event_params = Array::<Any>::new();
        ft_event_params.push(Any::from(from));
        ft_event_params.push(Any::new()); // null address for burn
        ft_event_params.push(Any::from(ft_amount));

        self.emit_event("Transfer", ft_event_params);

        // Create NFT with conversion data
        let mut token_data = Vec::new();
        token_data.extend_from_slice(name.as_bytes());
        token_data.push(b':');
        token_data.extend_from_slice(description.as_bytes());
        token_data.push(b':');
        token_data.extend_from_slice(b"converted_from_ft:");

        // Add the amount information
        let amount_str = format!("{}", ft_amount);
        token_data.extend_from_slice(amount_str.as_bytes());

        let token_key = self.make_nft_token_key(token_id);
        MockStorage::put(&token_key, &token_data);

        // Set ownership
        let owner_key = self.make_nft_owner_key(token_id);
        MockStorage::put(&owner_key, &from.to_vec());

        // Update NFT balance
        let balance_key = self.make_nft_balance_key(&from);
        let nft_balance = self.nft_balance_of(&from);
        let new_nft_balance = nft_balance + Int256::one();
        MockStorage::put(&balance_key, &new_nft_balance.to_vec());

        // Update tokens collection
        self.add_token_to_owner(&from, token_id);

        // Update NFT total supply
        let nft_supply = self.nft_total_supply();
        let new_nft_supply = nft_supply + Int256::one();
        MockStorage::put(b"total_nft", &new_nft_supply.to_vec());

        // Emit NFT transfer event (mint)
        let mut nft_event_params = Array::<Any>::new();
        nft_event_params.push(Any::new()); // null address for 'from'
        nft_event_params.push(Any::from(from));

        let token_bytes = ByteString::from_bytes(token_id);
        nft_event_params.push(Any::from(token_bytes));
        nft_event_params.push(Any::from(Int256::one()));

        self.emit_event("Transfer", nft_event_params);

        true
    }

    // Fractionalize an NFT into fungible tokens
    #[method]
    fn fractionalize_nft(&self, from: H160, token_id: &[u8], ft_amount: Int256) -> bool {
        // Check if the user owns the NFT
        match self.nft_owner_of(token_id) {
            Some(owner) if owner == from => {}
            _ => return false,
        }

        // Remove NFT ownership
        let owner_key = self.make_nft_owner_key(token_id);
        MockStorage::delete(&owner_key);

        // Remove NFT data
        let token_key = self.make_nft_token_key(token_id);
        MockStorage::delete(&token_key);

        // Update NFT balance
        let nft_balance_key = self.make_nft_balance_key(&from);
        let nft_balance = self.nft_balance_of(&from);
        let new_nft_balance = nft_balance - Int256::one();

        if new_nft_balance.is_zero() {
            MockStorage::delete(&nft_balance_key);
        } else {
            MockStorage::put(&nft_balance_key, &new_nft_balance.to_vec());
        }

        // Update tokens collection
        self.remove_token_from_owner(&from, token_id);

        // Update NFT total supply
        let nft_supply = self.nft_total_supply();
        let new_nft_supply = nft_supply - Int256::one();
        MockStorage::put(b"total_nft", &new_nft_supply.to_vec());

        // Emit NFT transfer event (burn)
        let mut nft_event_params = Array::<Any>::new();
        nft_event_params.push(Any::from(from));
        nft_event_params.push(Any::new()); // null address for 'to'

        let token_bytes = ByteString::from_bytes(token_id);
        nft_event_params.push(Any::from(token_bytes));
        nft_event_params.push(Any::from(Int256::one()));

        self.emit_event("Transfer", nft_event_params);

        // Add FT to balance
        let ft_balance_key = self.make_ft_balance_key(&from);
        let ft_balance = self.ft_balance_of(&from);
        let new_ft_balance = ft_balance + ft_amount;
        MockStorage::put(&ft_balance_key, &new_ft_balance.to_vec());

        // Update FT total supply
        let ft_supply = self.ft_total_supply();
        let new_ft_supply = ft_supply + ft_amount;
        MockStorage::put(b"total_supply", &new_ft_supply.to_vec());

        // Emit FT transfer event (mint)
        let mut ft_event_params = Array::<Any>::new();
        ft_event_params.push(Any::new()); // null address for 'from'
        ft_event_params.push(Any::from(from));
        ft_event_params.push(Any::from(ft_amount));

        self.emit_event("Transfer", ft_event_params);

        true
    }

    //======== Helper methods ========//

    // Add a token to owner's collection
    #[safe]
    fn add_token_to_owner(&self, owner: &H160, token_id: &[u8]) {
        let tokens_key = self.make_nft_tokens_key(owner);
        let mut tokens_data = Vec::new();

        if MockStorage::contains_key(&tokens_key) {
            tokens_data = MockStorage::get(&tokens_key);
            if !tokens_data.is_empty() {
                tokens_data.push(b',');
            }
        }

        tokens_data.extend_from_slice(token_id);
        MockStorage::put(&tokens_key, &tokens_data);
    }

    // Remove a token from owner's collection
    #[safe]
    fn remove_token_from_owner(&self, owner: &H160, token_id: &[u8]) -> bool {
        let tokens_key = self.make_nft_tokens_key(owner);

        if !MockStorage::contains_key(&tokens_key) {
            return false;
        }

        let tokens_data = MockStorage::get(&tokens_key);
        let tokens = self.nft_tokens_of(owner);

        // Create new tokens list without the specified token
        let mut new_tokens_data = Vec::new();
        let mut first = true;

        for token in tokens {
            if token != token_id {
                if !first {
                    new_tokens_data.push(b',');
                }
                new_tokens_data.extend_from_slice(&token);
                first = false;
            }
        }

        if new_tokens_data.is_empty() {
            MockStorage::delete(&tokens_key);
        } else {
            MockStorage::put(&tokens_key, &new_tokens_data);
        }

        true
    }

    // Key generation helpers
    #[safe]
    fn make_ft_balance_key(&self, owner: &H160) -> Vec<u8> {
        let mut key = Vec::with_capacity(11 + owner.len());
        key.extend_from_slice(b"ft_balance:");
        key.extend_from_slice(&owner.to_vec());
        key
    }

    #[safe]
    fn make_nft_token_key(&self, token_id: &[u8]) -> Vec<u8> {
        let mut key = Vec::with_capacity(6 + token_id.len());
        key.extend_from_slice(b"token:");
        key.extend_from_slice(token_id);
        key
    }

    #[safe]
    fn make_nft_owner_key(&self, token_id: &[u8]) -> Vec<u8> {
        let mut key = Vec::with_capacity(6 + token_id.len());
        key.extend_from_slice(b"owner:");
        key.extend_from_slice(token_id);
        key
    }

    #[safe]
    fn make_nft_balance_key(&self, owner: &H160) -> Vec<u8> {
        let mut key = Vec::with_capacity(12 + owner.len());
        key.extend_from_slice(b"nft_balance:");
        key.extend_from_slice(&owner.to_vec());
        key
    }

    #[safe]
    fn make_nft_tokens_key(&self, owner: &H160) -> Vec<u8> {
        let mut key = Vec::with_capacity(8 + owner.len());
        key.extend_from_slice(b"tokens:");
        key.extend_from_slice(&owner.to_vec());
        key
    }
}

#[test]
fn test_hybrid_token_basic_operations() {
    // Clear storage before test
    MockStorage::clear();

    // Create contract owner and user addresses
    let owner = H160::from_slice(&[1u8; 20]).unwrap();
    let user1 = H160::from_slice(&[2u8; 20]).unwrap();
    let user2 = H160::from_slice(&[3u8; 20]).unwrap();

    // Create hybrid token contract
    let contract = HybridTokenContract::new(owner, "Hybrid Token", "HYBRID", 8);

    // Test fungible token operations

    // Mint fungible tokens to user1
    let ft_amount = Int256::from(1000);
    let result = contract.ft_mint(user1, ft_amount);
    assert!(result, "FT minting should succeed");

    // Check FT balance
    let balance = contract.ft_balance_of(&user1);
    assert_eq!(balance, ft_amount, "User1 should have the minted FT amount");

    // Transfer FT from user1 to user2
    let transfer_amount = Int256::from(400);
    let transfer_result = contract.ft_transfer(user1, user2, transfer_amount);
    assert!(transfer_result, "FT transfer should succeed");

    // Check updated balances
    let user1_balance = contract.ft_balance_of(&user1);
    assert_eq!(user1_balance, Int256::from(600), "User1 should have 600 FT remaining");

    let user2_balance = contract.ft_balance_of(&user2);
    assert_eq!(user2_balance, transfer_amount, "User2 should have 400 FT");

    // Test non-fungible token operations

    // Mint NFT to user1
    let token_id = b"token1";
    let nft_result = contract.nft_mint(user1, token_id, "Test NFT", "This is a test NFT");
    assert!(nft_result, "NFT minting should succeed");

    // Check NFT ownership
    let token_owner = contract.nft_owner_of(token_id);
    assert_eq!(token_owner, Some(user1), "User1 should own the NFT");

    // Check NFT balance
    let nft_balance = contract.nft_balance_of(&user1);
    assert_eq!(nft_balance, Int256::one(), "User1 should have 1 NFT");

    // Transfer NFT from user1 to user2
    let nft_transfer_result = contract.nft_transfer(user1, user2, token_id);
    assert!(nft_transfer_result, "NFT transfer should succeed");

    // Check updated NFT ownership
    let new_token_owner = contract.nft_owner_of(token_id);
    assert_eq!(new_token_owner, Some(user2), "User2 should now own the NFT");

    // Check updated NFT balances
    let user1_nft_balance = contract.nft_balance_of(&user1);
    assert_eq!(user1_nft_balance, Int256::zero(), "User1 should have 0 NFTs");

    let user2_nft_balance = contract.nft_balance_of(&user2);
    assert_eq!(user2_nft_balance, Int256::one(), "User2 should have 1 NFT");

    // Dump storage for debugging
    println!("Storage contents: {}", MockStorage::dump());
}

#[test]
fn test_hybrid_token_conversions() {
    // Clear storage before test
    MockStorage::clear();

    // Create contract owner and user addresses
    let owner = H160::from_slice(&[1u8; 20]).unwrap();
    let user = H160::from_slice(&[2u8; 20]).unwrap();

    // Create hybrid token contract
    let contract = HybridTokenContract::new(owner, "Hybrid Token", "HYBRID", 8);

    // Mint fungible tokens to user
    let ft_amount = Int256::from(1000);
    contract.ft_mint(user, ft_amount);

    // Test converting FT to NFT
    let token_id = b"converted_token";
    let conversion_amount = Int256::from(100);

    let convert_result = contract.convert_ft_to_nft(
        user,
        token_id,
        "Converted NFT",
        "This NFT was converted from fungible tokens",
        conversion_amount,
    );

    assert!(convert_result, "FT to NFT conversion should succeed");

    // Check updated FT balance
    let ft_balance = contract.ft_balance_of(&user);
    assert_eq!(ft_balance, Int256::from(900), "User should have 900 FT remaining");

    // Check NFT ownership
    let token_owner = contract.nft_owner_of(token_id);
    assert_eq!(token_owner, Some(user), "User should own the converted NFT");

    // Check NFT balance
    let nft_balance = contract.nft_balance_of(&user);
    assert_eq!(nft_balance, Int256::one(), "User should have 1 NFT");

    // Test fractionalizing NFT back to FT
    let fractionalize_amount = Int256::from(200); // Returning more than original conversion

    let fractionalize_result = contract.fractionalize_nft(user, token_id, fractionalize_amount);

    assert!(fractionalize_result, "NFT fractionalization should succeed");

    // Check that NFT no longer exists
    let token_owner_after = contract.nft_owner_of(token_id);
    assert_eq!(token_owner_after, None, "NFT should no longer exist");

    // Check NFT balance
    let nft_balance_after = contract.nft_balance_of(&user);
    assert_eq!(nft_balance_after, Int256::zero(), "User should have 0 NFTs");

    // Check updated FT balance
    let ft_balance_after = contract.ft_balance_of(&user);
    assert_eq!(ft_balance_after, Int256::from(1100), "User should have 1100 FT after fractionalization");

    // Dump storage for debugging
    println!("Storage contents: {}", MockStorage::dump());
}
