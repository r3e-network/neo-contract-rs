// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[derive(Clone)]
pub struct Signer {
    account: H160,
    scopes: WitnessScope,
    allowed_contracts: Array<H160>,
    allowed_groups: Array<PublicKey>,
    rules: Array<WitnessRule>,
}

#[repr(C)]
#[derive(Clone)]
pub struct WitnessRule {
    action: WitnessRuleAction,
    condition: WitnessCondition,
}

#[repr(C)]
#[derive(Clone)]
pub struct WitnessCondition {
    condition_type: WitnessConditionType,
    condition: i32, // placeholder, and cannot use directly
}

#[repr(C)]
#[derive(Clone)]
pub struct AndCondition {
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
#[derive(Clone)]
pub struct OrCondition {
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
#[derive(Clone)]
pub struct NotCondition {
    expression: WitnessCondition,
}

#[repr(C)]
#[derive(Clone)]
pub struct BoolCondition {
    expression: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct CalledByContractCondition {
    hash: H160, // contract hash
}

#[repr(C)]
#[derive(Clone)]
pub struct CalledByGroupCondition {
    group: PublicKey, // group public key
}

#[repr(C)]
#[derive(Clone)]
pub struct GroupCondition {
    group: PublicKey, // group public key
}

#[repr(C)]
#[derive(Clone)]
pub struct ScriptHashCondition {
    hash: H160, // script hash
}
