// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
pub struct Signer {
    account: H160,
    scopes: WitnessScope,
    allowed_contracts: Array<H160>,
    allowed_groups: Array<PublicKey>,
    rules: Array<WitnessRule>,
}

#[repr(C)]
pub struct WitnessRule {
    action: WitnessRuleAction,
    condition: WitnessCondition,
}

#[repr(C)]
pub struct WitnessCondition {
    condition_type: WitnessConditionType,
    condition: i32, // placeholder, and cannot use directly
}

#[repr(C)]
pub struct AndCondition {
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
pub struct OrCondition {
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
pub struct NotCondition {
    expression: WitnessCondition,
}

#[repr(C)]
pub struct BoolCondition {
    expression: bool,
}

#[repr(C)]
pub struct CalledByContractCondition {
    hash: H160, // contract hash
}

#[repr(C)]
pub struct CalledByGroupCondition {
    group: PublicKey, // group public key
}

#[repr(C)]
pub struct GroupCondition {
    group: PublicKey, // group public key
}

#[repr(C)]
pub struct ScriptHashCondition {
    hash: H160, // script hash
}
