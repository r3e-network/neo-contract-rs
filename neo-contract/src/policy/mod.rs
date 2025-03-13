// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Policy module for managing contract governance
//! This module provides tools for implementing governance mechanisms,
//! access control, and policy enforcement.

use alloc::boxed::Box;
use alloc::vec::Vec;

// Import from prelude instead
use crate::prelude::{Any, Array, ByteString, Int256, H160};
use crate::runtime::Runtime; // Use correct path

pub mod roles;
pub mod voting;

/// Base trait for all policy types
pub trait Policy {
    /// Check if the policy allows the action
    fn allows(&self) -> bool;

    /// Check if the policy demands the action
    fn demands(&self) -> bool;
}

/// Always allows or demands
pub struct AlwaysPolicy(bool);

impl AlwaysPolicy {
    /// Create a new AlwaysPolicy
    pub fn new(value: bool) -> Self { Self(value) }

    /// Policy that always allows
    pub fn allow() -> Self { Self(true) }

    /// Policy that never allows
    pub fn deny() -> Self { Self(false) }
}

impl Policy for AlwaysPolicy {
    fn allows(&self) -> bool { self.0 }

    fn demands(&self) -> bool { self.0 }
}

/// Signature policy that checks for a valid witness
pub struct SignaturePolicy {
    /// The address that must sign
    address: H160,
}

impl SignaturePolicy {
    /// Create a new SignaturePolicy
    pub fn new(address: H160) -> Self { Self { address } }
}

impl Policy for SignaturePolicy {
    fn allows(&self) -> bool { Runtime::check_witness(&self.address) }

    fn demands(&self) -> bool { self.allows() }
}

/// Multi-signature policy that requires multiple signatures
pub struct MultiSignaturePolicy {
    /// Required signers
    signers: Vec<H160>,
    /// Required number of signers
    required: usize,
}

impl MultiSignaturePolicy {
    /// Create a new MultiSignaturePolicy
    pub fn new(signers: Vec<H160>, required: usize) -> Self { Self { signers, required } }
}

impl Policy for MultiSignaturePolicy {
    fn allows(&self) -> bool {
        let mut valid_count = 0;

        for signer in &self.signers {
            if Runtime::check_witness(&signer) {
                valid_count += 1;
                if valid_count >= self.required {
                    return true;
                }
            }
        }

        false
    }

    fn demands(&self) -> bool { self.allows() }
}

/// Combines multiple policies with AND logic
pub struct AndPolicy {
    /// Policies to combine
    policies: Vec<Box<dyn Policy>>,
}

impl AndPolicy {
    /// Create a new AndPolicy
    pub fn new(policies: Vec<Box<dyn Policy>>) -> Self { Self { policies } }
}

impl Policy for AndPolicy {
    fn allows(&self) -> bool {
        for policy in &self.policies {
            if !policy.allows() {
                return false;
            }
        }

        true
    }

    fn demands(&self) -> bool {
        for policy in &self.policies {
            if policy.demands() {
                return true;
            }
        }

        false
    }
}

/// Combines multiple policies with OR logic
pub struct OrPolicy {
    /// Policies to combine
    policies: Vec<Box<dyn Policy>>,
}

impl OrPolicy {
    /// Create a new OrPolicy
    pub fn new(policies: Vec<Box<dyn Policy>>) -> Self { Self { policies } }
}

impl Policy for OrPolicy {
    fn allows(&self) -> bool {
        for policy in &self.policies {
            if policy.allows() {
                return true;
            }
        }

        false
    }

    fn demands(&self) -> bool {
        for policy in &self.policies {
            if policy.demands() {
                return true;
            }
        }

        false
    }
}

/// Policy that enforces a threshold on a token balance
pub struct TokenThresholdPolicy {
    /// Token contract hash
    token_hash: H160,
    /// Account to check
    account: H160,
    /// Minimum balance required
    minimum_balance: Int256,
}

impl TokenThresholdPolicy {
    /// Create a new TokenThresholdPolicy
    pub fn new(token_hash: H160, account: H160, minimum_balance: Int256) -> Self {
        Self { token_hash, account, minimum_balance }
    }

    /// Get the balance of the token
    fn get_balance(&self) -> Int256 {
        let method = ByteString::from("balanceOf");
        let mut args: Vec<Any> = Vec::new();
        args.push(self.account.clone().into());

        // Call the token contract
        match Runtime::call_contract(
            self.token_hash.clone(),
            method,
            // Convert the Vec<Any> to Array
            Array::from(args),
        ) {
            result => match Int256::try_from(result) {
                Ok(balance) => balance,
                Err(_) => Int256::zero(),
            },
        }
    }
}

impl Policy for TokenThresholdPolicy {
    fn allows(&self) -> bool {
        let balance = self.get_balance();
        balance >= self.minimum_balance
    }

    fn demands(&self) -> bool { self.allows() }
}

/// Helper macro to enforce a policy
#[macro_export]
macro_rules! enforce_policy {
    ($policy:expr) => {
        if !$policy.allows() {
            return Err($crate::error::Error::new($crate::error::ErrorCode::Unauthorized, "Policy check failed"));
        }
    };

    ($policy:expr, $error_message:expr) => {
        if !$policy.allows() {
            return Err($crate::error::Error::new($crate::error::ErrorCode::Unauthorized, $error_message));
        }
    };
}
