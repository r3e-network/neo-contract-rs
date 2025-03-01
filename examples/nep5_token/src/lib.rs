// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]

extern crate neo_contract;

use neo_contract::{
    prelude::*,
    contract::{Nep5Token, Nep5TokenImpl},
    storage::{StorageMap},
    types::*,
};

// Storage keys
const NAME_KEY: &[u8] = b"name";
const SYMBOL_KEY: &[u8] = b"symbol";
const DECIMALS_KEY: &[u8] = b"decimals";
const TOTAL_SUPPLY_KEY: &[u8] = b"totalSupply";
const OWNER_KEY: &[u8] = b"owner";

// Events
const TRANSFER_EVENT: &[u8] = b"Transfer";

#[contract]
pub struct Nep5TokenContract;

#[contract_impl]
impl Nep5Token for Nep5TokenContract {
    fn name() -> ByteString {
        let storage = StorageMap::new();
        storage.get(NAME_KEY).unwrap_or_default()
    }

    fn symbol() -> ByteString {
        let storage = StorageMap::new();
        storage.get(SYMBOL_KEY).unwrap_or_default()
    }

    fn decimals() -> u8 {
        let storage = StorageMap::new();
        storage.get(DECIMALS_KEY).unwrap_or_default()
    }

    fn total_supply() -> Int256 {
        let storage = StorageMap::new();
        storage.get(TOTAL_SUPPLY_KEY).unwrap_or_default()
    }

    fn balance_of(account: H160) -> Int256 {
        let storage = StorageMap::new();
        let mut key = Vec::with_capacity(account.as_bytes().len() + 1);
        key.push(DEFAULT_BALANCE_KEY);
        key.extend_from_slice(account.as_bytes());
        storage.get(&key).unwrap_or_default()
    }

    fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        // Check if amount is valid
        if amount <= Int256::zero() {
            return false;
        }

        // Check if sender has enough balance
        let from_balance = Self::balance_of(from);
        if from_balance < amount {
            return false;
        }

        // Check if sender is authorized
        if !runtime::check_witness(from) {
            return false;
        }

        // Update balances
        if !update_nep5_balance(from, -amount) {
            return false;
        }

        if !update_nep5_balance(to, amount) {
            return false;
        }

        // Emit transfer event
        let mut event_args = Array::new();
        event_args.push(Any::from(from));
        event_args.push(Any::from(to));
        event_args.push(Any::from(amount));
        runtime::notify(TRANSFER_EVENT, event_args);

        true
    }
}

#[contract_impl]
impl Nep5TokenImpl for Nep5TokenContract {
    fn initialize(name: ByteString, symbol: ByteString, decimals: u8, total_supply: Int256, owner: H160) -> bool {
        // Check if contract is already initialized
        let storage = StorageMap::new();
        if storage.get::<ByteString>(NAME_KEY).is_some() {
            return false;
        }

        // Check if parameters are valid
        if total_supply <= Int256::zero() {
            return false;
        }

        // Store token information
        storage.put(NAME_KEY, &name);
        storage.put(SYMBOL_KEY, &symbol);
        storage.put(DECIMALS_KEY, &decimals);
        storage.put(TOTAL_SUPPLY_KEY, &total_supply);
        storage.put(OWNER_KEY, &owner);

        // Assign initial supply to owner
        update_nep5_balance(owner, total_supply);

        // Emit transfer event (from null address to owner)
        let mut event_args = Array::new();
        event_args.push(Any::from(H160::default())); // null address
        event_args.push(Any::from(owner));
        event_args.push(Any::from(total_supply));
        runtime::notify(TRANSFER_EVENT, event_args);

        true
    }

    fn mint(to: H160, amount: Int256) -> bool {
        // Check if amount is valid
        if amount <= Int256::zero() {
            return false;
        }

        // Check if caller is the owner
        let storage = StorageMap::new();
        let owner: H160 = storage.get(OWNER_KEY).unwrap_or_default();
        if !runtime::check_witness(owner) {
            return false;
        }

        // Update total supply
        let total_supply: Int256 = storage.get(TOTAL_SUPPLY_KEY).unwrap_or_default();
        let new_total_supply = total_supply + amount;
        storage.put(TOTAL_SUPPLY_KEY, &new_total_supply);

        // Update recipient balance
        update_nep5_balance(to, amount);

        // Emit transfer event (from null address to recipient)
        let mut event_args = Array::new();
        event_args.push(Any::from(H160::default())); // null address
        event_args.push(Any::from(to));
        event_args.push(Any::from(amount));
        runtime::notify(TRANSFER_EVENT, event_args);

        true
    }

    fn burn(from: H160, amount: Int256) -> bool {
        // Check if amount is valid
        if amount <= Int256::zero() {
            return false;
        }

        // Check if sender has enough balance
        let from_balance = Self::balance_of(from);
        if from_balance < amount {
            return false;
        }

        // Check if sender is authorized
        if !runtime::check_witness(from) {
            return false;
        }

        // Update total supply
        let storage = StorageMap::new();
        let total_supply: Int256 = storage.get(TOTAL_SUPPLY_KEY).unwrap_or_default();
        let new_total_supply = total_supply - amount;
        storage.put(TOTAL_SUPPLY_KEY, &new_total_supply);

        // Update sender balance
        update_nep5_balance(from, -amount);

        // Emit transfer event (from sender to null address)
        let mut event_args = Array::new();
        event_args.push(Any::from(from));
        event_args.push(Any::from(H160::default())); // null address
        event_args.push(Any::from(amount));
        runtime::notify(TRANSFER_EVENT, event_args);

        true
    }
}
