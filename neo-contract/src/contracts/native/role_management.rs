// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// RoleManagement native contract for Neo N3
/// 
/// This contract manages blockchain roles such as oracle nodes,
/// consensus nodes, and state validator nodes.
/// 
/// Contract Hash: 0x49cf4e5378ffcd4dec034fd98a174c5491e395e2
#[allow(non_snake_case)]
pub struct RoleManagement;

/// Role types in Neo N3 blockchain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Role for validating state
    StateValidator = 4,
    /// Role for oracle services
    Oracle = 8,
    /// Role for NEO name service
    NeoFS = 16,
    /// Role for consensus nodes
    Consensus = 32,
    /// Role for block verifier
    BlockVerifier = 64,
}

impl RoleManagement {
    /// Returns the contract hash for the RoleManagement native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::role_management_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_role_management_contract_hash()
        }
    }

    /// Gets the designated nodes by role
    /// 
    /// # Arguments
    /// 
    /// * `role` - The role to get nodes for
    /// 
    /// # Returns
    /// 
    /// An array of nodes as Any values
    #[safe]
    pub fn get_designated_by_role(role: Role) -> Array<Any> {
        let method = ByteString::from("getDesignatedByRole");
        let mut args = Array::<Any>::new();
        args.push(Any::from(role as u8));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|| Array::<Any>::new())
    }

    /// Designates a node for a specific role
    /// 
    /// # Arguments
    /// 
    /// * `role` - The role to designate
    /// * `nodes` - The public keys of the nodes to designate
    pub fn designate_as_role(role: Role, nodes: &Array<ByteString>) {
        let method = ByteString::from("designateAsRole");
        let mut args = Array::<Any>::new();
        args.push(Any::from(role as u8));
        
        // Convert Array<ByteString> to Array<Any>
        let nodes_any: Array<Any> = nodes.iter().map(|n| Any::from(n.clone())).collect();
        args.push(Any::from(nodes_any));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Gets the list of oracle nodes
    /// 
    /// # Returns
    /// 
    /// An array of oracle node public keys as ByteString values
    #[safe]
    pub fn get_oracle_nodes() -> Array<ByteString> {
        let designated = Self::get_designated_by_role(Role::Oracle);
        let mut result = Array::<ByteString>::new();
        
        for node in designated.iter() {
            if let Ok(pubkey) = node.try_into() {
                result.push(pubkey);
            }
        }
        
        result
    }

    /// Gets the list of state validator nodes
    /// 
    /// # Returns
    /// 
    /// An array of state validator node public keys as ByteString values
    #[safe]
    pub fn get_state_validator_nodes() -> Array<ByteString> {
        let designated = Self::get_designated_by_role(Role::StateValidator);
        let mut result = Array::<ByteString>::new();
        
        for node in designated.iter() {
            if let Ok(pubkey) = node.try_into() {
                result.push(pubkey);
            }
        }
        
        result
    }

    /// Gets the list of consensus nodes
    /// 
    /// # Returns
    /// 
    /// An array of consensus node public keys as ByteString values
    #[safe]
    pub fn get_consensus_nodes() -> Array<ByteString> {
        let designated = Self::get_designated_by_role(Role::Consensus);
        let mut result = Array::<ByteString>::new();
        
        for node in designated.iter() {
            if let Ok(pubkey) = node.try_into() {
                result.push(pubkey);
            }
        }
        
        result
    }
}
