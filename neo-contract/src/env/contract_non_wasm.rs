// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![allow(unused)]

use alloc::vec::Vec;
use alloc::borrow::Cow;

use crate::types::*;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;
use crate::types::builtin::array::Array;
use crate::types::builtin::any::Any;
use crate::types::block::Block;
use crate::types::tx::Tx;
use crate::types::signer::Signer;

// Define these types for the non-wasm environment
pub enum VmState {
    None,
    Halt,
    Fault,
    Break,
}

pub struct NeoCandidate {
    pub public_key: PublicKey,
    pub votes: Int256,
}

pub struct PublicKey(pub [u8; 33]);

// Get current contract hash
pub unsafe fn get_current_contract_hash() -> H160 {
    H160::default()
}

// Non-WASM implementations for testing
pub unsafe fn native_gas_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_gas_symbol() -> ByteString {
    "GAS".into()
}

pub unsafe fn native_gas_decimals() -> u32 {
    8
}

pub unsafe fn native_gas_total_supply() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_gas_balance_of(_account: H160) -> Int256 {
    Int256::zero()
}

pub unsafe fn native_gas_transfer(_from: H160, _to: H160, _amount: Int256) -> bool {
    true
}

pub unsafe fn native_neo_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_neo_symbol() -> ByteString {
    "NEO".into()
}

pub unsafe fn native_neo_decimals() -> u32 {
    0
}

pub unsafe fn native_neo_total_supply() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_neo_balance_of(_account: H160) -> Int256 {
    Int256::zero()
}

pub unsafe fn native_neo_transfer(_from: H160, _to: H160, _amount: Int256) -> bool {
    true
}

pub unsafe fn native_neo_get_gas_per_block() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_neo_get_register_price() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_neo_unclaimed_gas(_account: H160, _util_block_index: u32) -> Int256 {
    Int256::zero()
}

pub unsafe fn native_neo_register_candidate(_public_key: PublicKey) -> bool {
    true
}

pub unsafe fn native_neo_unregister_candidate(_public_key: PublicKey) -> bool {
    true
}

pub unsafe fn native_neo_vote(_account: H160, _vote_to: PublicKey) -> bool {
    true
}

pub unsafe fn native_neo_unvote(_account: H160) -> bool {
    true
}

pub unsafe fn native_neo_get_candidate_votes(_public_key: PublicKey) -> Int256 {
    Int256::zero()
}

pub unsafe fn native_neo_get_candidates() -> Array<NeoCandidate> {
    Array::new()
}

pub unsafe fn native_neo_get_committee() -> Array<PublicKey> {
    Array::new()
}

pub unsafe fn native_neo_get_committee_address() -> H160 {
    H160::default()
}

pub unsafe fn native_neo_get_next_block_validators() -> Array<PublicKey> {
    Array::new()
}

pub unsafe fn native_ledger_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_ledger_current_block_index() -> u32 {
    0
}

pub unsafe fn native_ledger_current_block_hash() -> H256 {
    H256::default()
}

pub unsafe fn native_ledger_block_of_index(_index: u32) -> Block {
    unimplemented!()
}

pub unsafe fn native_ledger_block_of_hash(_hash: H256) -> Block {
    unimplemented!()
}

pub unsafe fn native_ledger_get_tx(_hash: H256) -> Tx {
    unimplemented!()
}

pub unsafe fn native_ledger_get_tx_in_block_index(_block_index: u32, _tx_index: u32) -> Tx {
    unimplemented!()
}

pub unsafe fn native_ledger_get_tx_in_block_hash(_block_hash: H256, _tx_index: u32) -> Tx {
    unimplemented!()
}

pub unsafe fn native_ledger_get_tx_height(_hash: H256) -> u32 {
    0
}

pub unsafe fn native_ledger_get_tx_signers(_hash: H256) -> Array<Signer> {
    Array::new()
}

pub unsafe fn native_ledger_get_tx_vm_state(_hash: H256) -> VmState {
    VmState::None
}

pub unsafe fn native_policy_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_policy_get_fee_per_byte() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_policy_get_exec_fee_factor() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_policy_get_storage_price() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_policy_is_blocked(_account: H160) -> bool {
    false
}

pub unsafe fn native_policy_get_attr_fee(_attr_type: TxAttrType) -> Int256 {
    Int256::zero()
}

pub unsafe fn native_policy_set_attr_fee(_attr_type: TxAttrType, _fee: Int256) {
    // No-op for non-wasm
}

pub unsafe fn native_oracle_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_oracle_get_price() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_oracle_response(
    _url: ByteString,
    _filter: ByteString,
    _callback: ByteString,
    _user_data: Any,
    _gas_for_response: Int256,
) -> bool {
    true
}

pub unsafe fn native_role_management_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_role_management_get_designated_by_role(
    _role: Role,
    _block_index: u32,
) -> Array<PublicKey> {
    Array::new()
}

pub unsafe fn native_contract_management_contract_hash() -> H160 {
    H160::default()
}

pub unsafe fn native_contract_management_get_min_deployment_fee() -> Int256 {
    Int256::zero()
}

pub unsafe fn native_contract_management_contract_of_hash(_hash: H160) -> Contract {
    unimplemented!()
}

pub unsafe fn native_contract_management_contract_of_id(_id: u32) -> Contract {
    unimplemented!()
}

pub unsafe fn native_contract_management_get_contracts_hashes() -> Placeholder {
    unimplemented!()
}

pub unsafe fn native_contract_management_has_method(
    _hash: H160,
    _method: ByteString,
    _param_count: u32,
) -> bool {
    false
}

pub unsafe fn native_contract_management_deploy(
    _nef: ByteString,
    _manifest: ByteString,
) -> Contract {
    unimplemented!()
}

pub unsafe fn native_contract_management_update(_nef: ByteString, _manifest: ByteString) {
    // No-op for non-wasm
}

pub unsafe fn native_contract_management_destroy() {
    // No-op for non-wasm
}
