// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::{Any, ByteString, Contract, H160, Int256};

/// Native contract method declarations for Neo N3 blockchain
#[link(wasm_import_module = "neo.native")]
#[cfg(target_family = "wasm")]
extern "C" {
    // NeoToken native methods
    pub(crate) fn native_neo_transfer(from: H160, to: H160, amount: Int256) -> bool;
    pub(crate) fn native_neo_get_balance(account: H160) -> Int256;
    pub(crate) fn native_neo_get_name() -> ByteString;
    pub(crate) fn native_neo_get_symbol() -> ByteString;
    pub(crate) fn native_neo_get_decimals() -> u32;
    pub(crate) fn native_neo_get_total_supply() -> Int256;

    // GasToken native methods
    pub(crate) fn native_gas_transfer(from: H160, to: H160, amount: Int256) -> bool;
    pub(crate) fn native_gas_get_balance(account: H160) -> Int256;
    pub(crate) fn native_gas_get_name() -> ByteString;
    pub(crate) fn native_gas_get_symbol() -> ByteString;
    pub(crate) fn native_gas_get_decimals() -> u32;
    pub(crate) fn native_gas_get_total_supply() -> Int256;

    // StdLib native methods
    pub(crate) fn native_stdlib_base64_encode(data: ByteString) -> ByteString;
    pub(crate) fn native_stdlib_base64_decode(data: ByteString) -> ByteString;
    pub(crate) fn native_stdlib_json_serialize(item: Any) -> ByteString;
    pub(crate) fn native_stdlib_json_deserialize(json: ByteString) -> Any;
    pub(crate) fn native_stdlib_itoa(value: Int256) -> ByteString;
    pub(crate) fn native_stdlib_atoi(value: ByteString) -> Int256;

    // ContractManagement native methods
    pub(crate) fn native_contract_get_contract(script_hash: H160) -> Contract;
    pub(crate) fn native_contract_deploy(nef_file: ByteString, manifest: ByteString, data: Any) -> Contract;
    pub(crate) fn native_contract_update(nef_file: ByteString, manifest: ByteString, data: Any) -> bool;
    pub(crate) fn native_contract_destroy() -> bool;
}