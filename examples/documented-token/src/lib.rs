use neo_contract::prelude::*;

/// DocumentedToken - A well-documented NEP-17 token implementation
/// 
/// This example demonstrates how to write proper documentation comments
/// that will be extracted by the manifest generator and included in the
/// contract manifest file.
/// 
/// Features:
/// - Fixed supply of 1,000,000 tokens
/// - 8 decimal places 
/// - Standard NEP-17 functionality
/// - Comprehensive inline documentation
/// - Efficient storage patterns
/// - Safe method annotations
#[contract]
impl DocumentedToken {
    /// Setup storage paths and constants
    const TOKEN_NAME: &'static str = "DocumentedToken";
    const TOKEN_SYMBOL: &'static str = "DOC";
    const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY: u128 = 100_000_000 * 100_000_000; // 100M tokens with 8 decimals
    
    const SUPPLY_KEY: &'static [u8] = b"totalSupply";
    const BALANCE_PREFIX: &'static [u8] = b"balance:";
    
    /// Returns the token name
    /// 
    /// # Returns
    /// A ByteString containing the token name "DocumentedToken"
    /// 
    /// @safe
    pub fn name() -> ByteString {
        Self::TOKEN_NAME.into()
    }
    
    /// Returns the token symbol
    /// 
    /// # Returns
    /// A ByteString containing the token symbol "DOC"
    /// 
    /// @safe
    pub fn symbol() -> ByteString {
        Self::TOKEN_SYMBOL.into()
    }
    
    /// Returns the number of decimal places used by the token
    /// 
    /// # Returns
    /// The number of decimal places (8)
    /// 
    /// @safe
    pub fn decimals() -> u8 {
        Self::TOKEN_DECIMALS
    }
    
    /// Returns the total supply of tokens
    /// 
    /// # Returns
    /// The total token supply (100,000,000 * 10^8)
    /// 
    /// @safe
    pub fn totalSupply() -> u128 {
        let storage = Storage::new();
        storage.get::<_, u128>(Self::SUPPLY_KEY).unwrap_or(0)
    }
    
    /// Returns the token balance for the specified account
    /// 
    /// # Parameters
    /// * `account` - The account address to check balance for
    /// 
    /// # Returns
    /// The account's token balance
    /// 
    /// @safe
    pub fn balanceOf(account: Address) -> u128 {
        let storage = Storage::new();
        let key = [Self::BALANCE_PREFIX, account.as_bytes()].concat();
        storage.get::<_, u128>(&key).unwrap_or(0)
    }
    
    /// Transfers tokens from one account to another
    /// 
    /// This method verifies that:
    /// - The from account has sufficient balance
    /// - The to account is not the zero address
    /// - Amount is greater than zero
    /// 
    /// # Parameters
    /// * `from` - The account to transfer tokens from
    /// * `to` - The account to transfer tokens to
    /// * `amount` - The amount of tokens to transfer
    /// * `data` - Optional data to include with the transfer
    /// 
    /// # Returns
    /// `true` if the transfer was successful, `false` otherwise
    pub fn transfer(from: Address, to: Address, amount: u128, data: ByteString) -> bool {
        // Verify sender
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Check amount
        if amount == 0 {
            return true;
        }
        
        // Check recipient is not null address
        if to == Address::from([0u8; 20]) {
            return false;
        }
        
        // Efficient pattern - Read all data first
        let storage = Storage::new();
        let from_key = [Self::BALANCE_PREFIX, from.as_bytes()].concat();
        let to_key = [Self::BALANCE_PREFIX, to.as_bytes()].concat();
        
        let from_balance = storage.get::<_, u128>(&from_key).unwrap_or(0);
        let to_balance = storage.get::<_, u128>(&to_key).unwrap_or(0);
        
        // Check sufficient balance
        if from_balance < amount {
            return false;
        }
        
        // Handle arithmetic overflow safely
        let new_to_balance = to_balance.checked_add(amount).unwrap_or_else(|| {
            panic!("Transfer would overflow recipient balance")
        });
        
        // Update storage efficiently
        if amount == from_balance {
            storage.delete(&from_key);
        } else {
            storage.put(&from_key, &(from_balance - amount));
        }
        
        storage.put(&to_key, &new_to_balance);
        
        // Emit transfer event
        Runtime::notify(
            "Transfer",
            &(from, to, amount)
        );
        
        // If the recipient is a contract, call onNEP17Payment
        if Runtime::contract_exists(&to) {
            let contract = Contract::from_address(&to);
            let _ = contract.call("onNEP17Payment", 
                                 &(from, amount, data));
        }
        
        true
    }
    
    /// Initializes the token contract
    /// 
    /// This method is called once during contract deployment and:
    /// - Sets the initial token supply
    /// - Assigns all tokens to the contract deployer
    /// 
    /// # Parameters
    /// * `data` - Optional initialization data (not used)
    pub fn _deploy(data: ByteString) {
        let storage = Storage::new();
        
        // Check if already initialized
        if storage.get::<_, u128>(Self::SUPPLY_KEY).is_some() {
            return;
        }
        
        // Get contract deployer
        let deployer = Runtime::get_trigger_address();
        
        // Efficient batched storage operations
        storage.put(Self::SUPPLY_KEY, &Self::TOKEN_TOTAL_SUPPLY);
        
        let deployer_key = [Self::BALANCE_PREFIX, deployer.as_bytes()].concat();
        storage.put(&deployer_key, &Self::TOKEN_TOTAL_SUPPLY);
        
        // Emit transfer event (from null address)
        Runtime::notify(
            "Transfer",
            &(Address::from([0u8; 20]), deployer, Self::TOKEN_TOTAL_SUPPLY)
        );
    }
    
    /// Returns information about the token contract
    /// 
    /// # Returns
    /// A ByteString containing JSON with contract information
    /// 
    /// @safe
    pub fn get_info() -> ByteString {
        // Efficient string construction with pre-sized format
        let json = format!(
            "{{\"name\":\"{}\",\"symbol\":\"{}\",\"decimals\":{},\"totalSupply\":{}}}",
            Self::TOKEN_NAME,
            Self::TOKEN_SYMBOL,
            Self::TOKEN_DECIMALS,
            Self::totalSupply()
        );
        
        json.into()
    }
}