// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![allow(unused)]

use crate::types::{Any, Placeholder};
#[cfg(target_family = "wasm")]
use crate::types::*;

#[link(wasm_import_module = "neo.contract")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    pub(crate) fn native_gas_contract_hash() -> H160;

    pub(crate) fn native_gas_symbol() -> ByteString;

    pub(crate) fn native_gas_decimals() -> u32;

    pub(crate) fn native_gas_total_supply() -> Int256;

    pub(crate) fn native_gas_balance_of(account: H160) -> Int256;

    pub(crate) fn native_gas_transfer(from: H160, to: H160, amount: Int256) -> bool;

    pub(crate) fn native_neo_contract_hash() -> H160;

    pub(crate) fn native_neo_symbol() -> ByteString;

    pub(crate) fn native_neo_decimals() -> u32;

    pub(crate) fn native_neo_total_supply() -> Int256;

    pub(crate) fn native_neo_balance_of(account: H160) -> Int256;

    pub(crate) fn native_neo_transfer(from: H160, to: H160, amount: Int256) -> bool;

    pub(crate) fn native_neo_get_gas_per_block() -> Int256;

    pub(crate) fn native_neo_get_register_price() -> Int256;

    pub(crate) fn native_neo_unclaimed_gas(account: H160, util_block_index: u32) -> Int256;

    pub(crate) fn native_neo_register_candidate(public_key: PublicKey) -> bool;

    pub(crate) fn native_neo_unregister_candidate(public_key: PublicKey) -> bool;

    pub(crate) fn native_neo_vote(account: H160, vote_to: PublicKey) -> bool;

    pub(crate) fn native_neo_unvote(account: H160) -> bool;

    pub(crate) fn native_neo_get_candidate_votes(public_key: PublicKey) -> Int256;

    pub(crate) fn native_neo_get_candidates() -> Array<NeoCandidate>;

    // pub(crate) fn native_neo_get_all_candidates() -> Placeholder;

    pub(crate) fn native_neo_get_committee() -> Array<PublicKey>;

    pub(crate) fn native_neo_get_committee_address() -> H160;

    pub(crate) fn native_neo_get_next_block_validators() -> Array<PublicKey>;

    pub(crate) fn native_ledger_contract_hash() -> H160;

    pub(crate) fn native_ledger_current_block_index() -> u32;

    pub(crate) fn native_ledger_current_block_hash() -> H256;

    pub(crate) fn native_ledger_block_of_index(index: u32) -> Block;

    pub(crate) fn native_ledger_block_of_hash(hash: H256) -> Block;

    pub(crate) fn native_ledger_get_tx(hash: H256) -> Tx;

    pub(crate) fn native_ledger_get_tx_in_block_index(block_index: u32, tx_index: u32) -> Tx;

    pub(crate) fn native_ledger_get_tx_in_block_hash(block_hash: H256, tx_index: u32) -> Tx;

    pub(crate) fn native_ledger_get_tx_height(hash: H256) -> u32;

    pub(crate) fn native_ledger_get_tx_signers(hash: H256) -> Array<Signer>;

    pub(crate) fn native_ledger_get_tx_vm_state(hash: H256) -> VmState;

    pub(crate) fn native_policy_contract_hash() -> H160;

    pub(crate) fn native_policy_get_fee_per_byte() -> Int256;

    pub(crate) fn native_policy_get_exec_fee_factor() -> Int256;

    pub(crate) fn native_policy_get_storage_price() -> Int256;

    pub(crate) fn native_policy_is_blocked(account: H160) -> bool;

    pub(crate) fn native_policy_get_attr_fee(attr_type: TxAttrType) -> Int256;

    pub(crate) fn native_policy_set_attr_fee(attr_type: TxAttrType, fee: Int256);

    pub(crate) fn native_oracle_contract_hash() -> H160;

    pub(crate) fn native_oracle_get_price() -> Int256;

    pub(crate) fn native_oracle_response(
        url: ByteString,
        filter: ByteString,
        callback: ByteString,
        user_data: Any,
        gas_for_response: Int256,
    ) -> bool;

    pub(crate) fn native_role_management_contract_hash() -> H160;

    pub(crate) fn native_role_management_get_designated_by_role(
        role: Role,
        block_index: u32,
    ) -> Array<PublicKey>;

    pub(crate) fn native_contract_management_contract_hash() -> H160;

    pub(crate) fn native_contract_management_get_min_deployment_fee() -> Int256;

    pub(crate) fn native_contract_management_contract_of_hash(hash: H160) -> Contract;

    pub(crate) fn native_contract_management_contract_of_id(id: u32) -> Contract;

    pub(crate) fn native_contract_management_get_contracts_hashes() -> Placeholder;

    pub(crate) fn native_contract_management_has_method(
        hash: H160,
        method: ByteString,
        param_count: u32,
    ) -> bool;

    pub(crate) fn native_contract_management_deploy(
        nef: ByteString,
        manifest: ByteString,
    ) -> Contract;

    // pub(crate) fn native_contract_management_deploy_with_data(
    //     nef: ByteString,
    //     manifest: ByteString,
    //     data: Any,
    // ) -> Contract;

    pub(crate) fn native_contract_management_update(nef: ByteString, manifest: ByteString);

    // pub(crate) fn native_contract_management_update_with_data(
    //     nef: ByteString,
    //     manifest: ByteString,
    //     data: Any,
    // );

    pub(crate) fn native_contract_management_destroy();
}
