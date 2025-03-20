#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Event emitted when tokens are transferred between addresses
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    token_id: ByteString,
    amount: u64
}

/// Event emitted when a new token is minted
#[event]
struct Mint {
    #[index]
    to: H160,
    token_id: ByteString,
    properties: ByteString
}

/// Event emitted when a token is burned
#[event]
struct Burn {
    #[index]
    owner: H160,
    token_id: ByteString
}

// Data structures
#[derive(Encode, Decode, Debug)]
struct TokenProperties {
    name: String,
    description: String,
    image: String,
    token_uri: String,
    attributes: Vec<TokenAttribute>,
    created_at: u64,
}

#[derive(Encode, Decode, Debug)]
struct TokenAttribute {
    trait_type: String,
    value: String,
}

/// A basic NFT (Non-Fungible Token) implementation for Neo N3
/// Implements the NEP-11 standard for non-fungible tokens
#[neo_contract::contract]
pub struct NeoNft {
    // Contract owner
    owner: StorageMap<String, H160>,
    
    // Token name
    name: StorageMap<String, String>,
    
    // Token symbol
    symbol: StorageMap<String, String>,
    
    // Total supply of tokens
    total_supply: StorageMap<String, u64>,
    
    // Maps token ID to owner address
    owners: StorageMap<ByteString, H160>,
    
    // Maps token ID to token properties
    properties: StorageMap<ByteString, TokenProperties>,
    
    // Maps owner address to token count
    balances: StorageMap<H160, u64>,
    
    // Maps owner address to list of owned token IDs
    tokens_of_owner: StorageMap<H160, Vec<ByteString>>,
    
    // Maps token ID to token index in owner's list
    token_index: StorageMap<ByteString, u64>,
    
    // Maps (owner, operator) to approval status
    operator_approvals: StorageMap<(H160, H160), bool>,
    
    // Maps token ID to approved address
    token_approvals: StorageMap<ByteString, H160>,
}

impl NeoNft {
    /// Contract constructor - called once when the contract is deployed
    #[constructor]
    pub fn new(owner: H160, name: String, symbol: String) -> Self {
        // Create contract instance
        let mut instance = Self {
            owner: StorageMap::new(b"owner"),
            name: StorageMap::new(b"name"),
            symbol: StorageMap::new(b"symbol"),
            total_supply: StorageMap::new(b"total_supply"),
            owners: StorageMap::new(b"owners"),
            properties: StorageMap::new(b"properties"),
            balances: StorageMap::new(b"balances"),
            tokens_of_owner: StorageMap::new(b"tokens_of_owner"),
            token_index: StorageMap::new(b"token_index"),
            operator_approvals: StorageMap::new(b"operator_approvals"),
            token_approvals: StorageMap::new(b"token_approvals"),
        };
        
        // Initialize contract state
        instance.owner.insert("value", owner);
        instance.name.insert("value", name);
        instance.symbol.insert("value", symbol);
        instance.total_supply.insert("value", 0);
        
        instance
    }
    
    /// Mints a new NFT
    /// 
    /// # Arguments
    /// * `to` - The address that will own the new token
    /// * `token_id` - The unique identifier for the token
    /// * `properties_json` - JSON string containing the token metadata
    /// 
    /// # Returns
    /// `true` if the token was successfully minted
    #[method]
    #[no_reentrant]
    pub fn mint(&mut self, to: H160, token_id: ByteString, properties_json: ByteString) -> bool {
        // Verify caller is owner
        let owner = self.owner.get("value").unwrap_or_default();
        assert!(Runtime::check_witness(&owner), "Only owner can mint tokens");
        
        // Verify token doesn't already exist
        assert!(!self.token_exists(&token_id), "Token already exists");
        
        // Parse properties from JSON
        let properties = self.parse_properties(properties_json.clone());
        
        // Store token data
        self.owners.insert(&token_id, to);
        self.properties.insert(&token_id, properties);
        
        // Update owner's balance and token list
        let balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(&to, balance + 1);
        
        let mut tokens = self.tokens_of_owner.get(&to).unwrap_or_default();
        let token_index = tokens.len() as u64;
        tokens.push(token_id.clone());
        self.tokens_of_owner.insert(&to, tokens);
        self.token_index.insert(&token_id, token_index);
        
        // Update total supply
        let supply = self.total_supply.get("value").unwrap_or_default();
        self.total_supply.insert("value", supply + 1);
        
        // Emit events
        Transfer {}.notify(&Option::<H160>::None, &Some(to), &token_id, &1u64);
        Mint {}.notify(&to, &token_id, &properties_json);
        
        true
    }
    
    /// Burns (destroys) an NFT
    /// 
    /// # Arguments
    /// * `token_id` - The unique identifier of the token to burn
    /// 
    /// # Returns
    /// `true` if the token was successfully burned
    #[method]
    #[no_reentrant]
    pub fn burn(&mut self, token_id: ByteString) -> bool {
        // Verify token exists
        assert!(self.token_exists(&token_id), "Token does not exist");
        
        // Get token owner
        let owner = self.owners.get(&token_id).expect("Failed to get token owner");
        
        // Verify caller is owner
        assert!(Runtime::check_witness(&owner), "Only token owner can burn");
        
        // Remove token approvals
        self.token_approvals.remove(&token_id);
        
        // Update owner's balance and token list
        let balance = self.balances.get(&owner).unwrap_or_default();
        self.balances.insert(&owner, balance - 1);
        
        let mut tokens = self.tokens_of_owner.get(&owner).unwrap_or_default();
        let token_index = self.token_index.get(&token_id).unwrap_or_default() as usize;
        
        // Remove token from owner's list
        if token_index < tokens.len() - 1 {
            // If not the last token, move the last token to this position
            let last_token = tokens[tokens.len() - 1].clone();
            tokens[token_index] = last_token.clone();
            
            // Update the moved token's index
            self.token_index.insert(&last_token, token_index as u64);
        }
        
        // Remove the last element
        tokens.pop();
        self.tokens_of_owner.insert(&owner, tokens);
        
        // Remove token data
        self.owners.remove(&token_id);
        self.properties.remove(&token_id);
        self.token_index.remove(&token_id);
        
        // Update total supply
        let supply = self.total_supply.get("value").unwrap_or_default();
        self.total_supply.insert("value", supply - 1);
        
        // Emit events
        Transfer {}.notify(&Some(owner), &Option::<H160>::None, &token_id, &1u64);
        Burn {}.notify(&owner, &token_id);
        
        true
    }
    
    /// Transfers a token from one address to another
    /// 
    /// # Arguments
    /// * `from` - The current owner of the token
    /// * `to` - The new owner of the token
    /// * `token_id` - The unique identifier of the token to transfer
    /// * `data` - Optional data to pass to the recipient contract
    /// 
    /// # Returns
    /// `true` if the transfer was successful
    #[method]
    #[no_reentrant]
    pub fn transfer(&mut self, from: H160, to: H160, token_id: ByteString, data: Option<Vec<u8>>) -> bool {
        // Verify token exists
        assert!(self.token_exists(&token_id), "Token does not exist");
        
        // Verify token owner
        let owner = self.owners.get(&token_id).expect("Failed to get token owner");
        assert!(owner == from, "From address is not the token owner");
        
        // Verify authorization
        assert!(self.is_approved_or_owner(&token_id, &Runtime::calling_script_hash()), 
                "Caller is not approved to transfer this token");
        
        // Verify target is not zero address
        assert!(to != H160::zero(), "Cannot transfer to zero address");
        
        // Clear approvals
        self.token_approvals.remove(&token_id);
        
        // Update balances
        let from_balance = self.balances.get(&from).unwrap_or_default();
        self.balances.insert(&from, from_balance - 1);
        
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(&to, to_balance + 1);
        
        // Update owner lists
        let mut from_tokens = self.tokens_of_owner.get(&from).unwrap_or_default();
        let token_index = self.token_index.get(&token_id).unwrap_or_default() as usize;
        
        if token_index < from_tokens.len() - 1 {
            // If not the last token, move the last token to this position
            let last_token = from_tokens[from_tokens.len() - 1].clone();
            from_tokens[token_index] = last_token.clone();
            
            // Update the moved token's index
            self.token_index.insert(&last_token, token_index as u64);
        }
        
        // Remove the last element
        from_tokens.pop();
        self.tokens_of_owner.insert(&from, from_tokens);
        
        // Add to recipient's tokens
        let mut to_tokens = self.tokens_of_owner.get(&to).unwrap_or_default();
        let new_index = to_tokens.len() as u64;
        to_tokens.push(token_id.clone());
        self.tokens_of_owner.insert(&to, to_tokens);
        self.token_index.insert(&token_id, new_index);
        
        // Update token owner
        self.owners.insert(&token_id, to);
        
        // Emit transfer event
        Transfer {}.notify(&Some(from), &Some(to), &token_id, &1u64);
        
        // Handle NEP11 onReceived callback if recipient is a contract
        if self.is_contract(&to) {
            // Attempt to call onNEP11Payment if it's a contract
            let data_param = data.unwrap_or_default();
            let _ = Runtime::call_contract(&to, "onNEP11Payment", &[&from, &1u64, &token_id, &data_param]);
        }
        
        true
    }
    
    /// Approves another address to transfer the given token
    /// 
    /// # Arguments
    /// * `approved` - The address being approved
    /// * `token_id` - The token that can be transferred
    /// 
    /// # Returns
    /// `true` if the approval was successful
    #[method]
    #[no_reentrant]
    pub fn approve(&mut self, approved: H160, token_id: ByteString) -> bool {
        // Get the token owner
        let owner = self.owners.get(&token_id).expect("Failed to get token owner");
        
        // Verify caller is owner or approved operator
        let caller = Runtime::calling_script_hash();
        assert!(owner == caller || self.is_approved_for_all(&owner, &caller), 
                "Caller is not owner or approved operator");
        
        // Set token approval
        self.token_approvals.insert(&token_id, approved);
        
        true
    }
    
    /// Sets or revokes approval for an operator to manage all of the caller's tokens
    /// 
    /// # Arguments
    /// * `operator` - The address to grant operator status
    /// * `approved` - Whether to approve or revoke approval
    /// 
    /// # Returns
    /// `true` if the operation was successful
    #[method]
    #[no_reentrant]
    pub fn set_approval_for_all(&mut self, operator: H160, approved: bool) -> bool {
        // Get the caller
        let caller = Runtime::calling_script_hash();
        
        // Verify caller is not setting self as operator
        assert!(caller != operator, "Cannot set approval for self");
        
        // Update operator approval
        self.operator_approvals.insert(&(caller, operator), approved);
        
        true
    }
    
    /// Gets the address approved to transfer a specific token
    /// 
    /// # Arguments
    /// * `token_id` - The token to check
    /// 
    /// # Returns
    /// The approved address for the token, or zero address if none
    #[safe]
    pub fn get_approved(&self, token_id: ByteString) -> H160 {
        assert!(self.token_exists(&token_id), "Token does not exist");
        self.token_approvals.get(&token_id).unwrap_or_default()
    }
    
    /// Checks if an operator is approved to manage all tokens of an owner
    /// 
    /// # Arguments
    /// * `owner` - The token owner
    /// * `operator` - The potential operator
    /// 
    /// # Returns
    /// `true` if the operator is approved for all tokens
    #[safe]
    pub fn is_approved_for_all(&self, owner: H160, operator: H160) -> bool {
        self.operator_approvals.get(&(owner, operator)).unwrap_or_default()
    }
    
    /// Returns the owner of a specific token
    /// 
    /// # Arguments
    /// * `token_id` - The token to check
    /// 
    /// # Returns
    /// The owner's address
    #[safe]
    pub fn owner_of(&self, token_id: ByteString) -> H160 {
        assert!(self.token_exists(&token_id), "Token does not exist");
        self.owners.get(&token_id).unwrap_or_default()
    }
    
    /// Returns the balance of an address
    /// 
    /// # Arguments
    /// * `owner` - The address to check
    /// 
    /// # Returns
    /// The number of tokens owned by the address
    #[safe]
    pub fn balance_of(&self, owner: H160) -> u64 {
        self.balances.get(&owner).unwrap_or_default()
    }
    
    /// Returns the token collection name
    /// 
    /// # Returns
    /// The name of the token collection
    #[safe]
    pub fn name(&self) -> String {
        self.name.get("value").unwrap_or_default()
    }
    
    /// Returns the token collection symbol
    /// 
    /// # Returns
    /// The symbol of the token collection
    #[safe]
    pub fn symbol(&self) -> String {
        self.symbol.get("value").unwrap_or_default()
    }
    
    /// Returns the number of tokens in circulation
    /// 
    /// # Returns
    /// The total supply of tokens
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get("value").unwrap_or_default()
    }
    
    /// Returns the tokens owned by an address
    /// 
    /// # Arguments
    /// * `owner` - The address to check
    /// 
    /// # Returns
    /// An array of token IDs owned by the address
    #[safe]
    pub fn tokens_of(&self, owner: H160) -> Vec<ByteString> {
        self.tokens_of_owner.get(&owner).unwrap_or_default()
    }
    
    /// Returns the properties of a token
    /// 
    /// # Arguments
    /// * `token_id` - The token to check
    /// 
    /// # Returns
    /// The properties of the token as a JSON string
    #[safe]
    pub fn properties(&self, token_id: ByteString) -> ByteString {
        assert!(self.token_exists(&token_id), "Token does not exist");
        let properties = self.properties.get(&token_id).expect("Failed to get properties");
        self.properties_to_json(&properties)
    }
    
    /// Checks if a token exists
    /// 
    /// # Arguments
    /// * `token_id` - The token to check
    /// 
    /// # Returns
    /// `true` if the token exists
    #[safe]
    pub fn token_exists(&self, token_id: &ByteString) -> bool {
        self.owners.get(token_id).is_some()
    }
    
    /// Returns the contract owner
    /// 
    /// # Returns
    /// The owner's address
    #[safe]
    pub fn get_owner(&self) -> H160 {
        self.owner.get("value").unwrap_or_default()
    }
    
    /// Changes the contract owner (only callable by current owner)
    /// 
    /// # Arguments
    /// * `new_owner` - The new owner's address
    /// 
    /// # Returns
    /// `true` if the owner was changed successfully
    #[method]
    #[no_reentrant]
    pub fn set_owner(&mut self, new_owner: H160) -> bool {
        let current_owner = self.owner.get("value").unwrap_or_default();
        assert!(Runtime::check_witness(&current_owner), "Only owner can transfer ownership");
        assert!(new_owner != H160::zero(), "Cannot transfer to zero address");
        
        self.owner.insert("value", new_owner);
        true
    }
    
    // Internal helper methods
    
    /// Parses token properties from JSON
    fn parse_properties(&self, json: ByteString) -> TokenProperties {
        // In a real implementation, this would parse JSON
        // For this example, we'll create a simple property object
        TokenProperties {
            name: "NFT Token".to_string(),
            description: "Example NFT Token".to_string(),
            image: "https://example.com/image.png".to_string(),
            token_uri: "https://example.com/metadata.json".to_string(),
            attributes: Vec::new(),
            created_at: Ledger::current_timestamp(),
        }
    }
    
    /// Converts token properties to JSON
    fn properties_to_json(&self, properties: &TokenProperties) -> ByteString {
        // In a real implementation, this would convert to JSON
        // For this example, we'll return a simple string
        ByteString::from(alloc::format!(
            "{{\"name\":\"{}\",\"description\":\"{}\",\"image\":\"{}\",\"token_uri\":\"{}\"}}",
            properties.name,
            properties.description,
            properties.image,
            properties.token_uri
        ))
    }
    
    /// Checks if an address is a contract
    fn is_contract(&self, address: &H160) -> bool {
        // This is a simplified check - in a real implementation,
        // you would need to use the appropriate Neo API
        address != &H160::zero()
    }
    
    /// Checks if an address is approved to transfer a token
    fn is_approved_or_owner(&self, token_id: &ByteString, address: &H160) -> bool {
        let owner = self.owners.get(token_id).unwrap_or_default();
        
        // Check if address is owner
        if &owner == address {
            return true;
        }
        
        // Check if address is approved for this token
        let approved = self.token_approvals.get(token_id).unwrap_or_default();
        if &approved == address {
            return true;
        }
        
        // Check if address is an approved operator
        self.is_approved_for_all(owner, *address)
    }
} 