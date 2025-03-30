// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

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
    condition: Any, // any, and cannot use directly
}

#[repr(C)]
#[crate::inner_structs]
pub struct AndCondition {
    #[get(pub)]
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
#[crate::inner_structs]
pub struct OrCondition {
    #[get(pub)]
    expressions: Array<WitnessCondition>,
}

#[repr(C)]
#[crate::inner_structs]
pub struct NotCondition {
    #[get(pub)]
    expression: WitnessCondition,
}

#[repr(C)]
// #[crate::inner_structs]
pub struct BoolCondition {
    // #[get(pub)]
    expression: bool, // TODO: bool support
}

#[repr(C)]
#[crate::inner_structs]
pub struct CalledByContractCondition {
    #[get(pub)]
    hash: H160, // contract hash
}

#[repr(C)]
#[crate::inner_structs]
pub struct CalledByGroupCondition {
    #[get(pub)]
    group: PublicKey, // group public key
}

#[repr(C)]
#[crate::inner_structs]
pub struct GroupCondition {
    #[get(pub)]
    group: PublicKey, // group public key
}

#[repr(C)]
#[crate::inner_structs]
pub struct ScriptHashCondition {
    #[get(pub)]
    hash: H160, // script hash
}
