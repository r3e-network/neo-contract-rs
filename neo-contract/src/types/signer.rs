// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::{placeholder::*, *};

#[repr(C)]
#[crate::inner_structs]
pub struct Signer {
    #[get(pub)]
    account: H160,

    #[get(pub)]
    scopes: Int256, // WitnessScope

    #[get(pub)]
    allowed_contracts: Array<H160>,

    #[get(pub)]
    allowed_groups: Array<PublicKey>,

    #[get(pub)]
    rules: Array<WitnessRule>,
}

#[repr(C)]
#[crate::inner_structs]
pub struct WitnessRule {
    #[get(pub)]
    action: Int256, // WitnessRuleAction

    #[get(pub)]
    condition: WitnessCondition,
}

#[repr(C)]
#[crate::inner_structs]
pub struct WitnessCondition {
    #[get(pub)]
    condition_type: Int256, // WitnessConditionType

    #[get(pub)]
    condition: Placeholder, // placeholder, and cannot use directly
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
