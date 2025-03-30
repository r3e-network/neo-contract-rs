// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[crate::inner_structs]
pub struct ContractHash {
    /// an u32 value
    #[get(pub)]
    id: Int256,

    #[get(pub)]
    hash: H160,
}

#[repr(C)]
#[crate::inner_structs]
pub struct Contract {
    /// an u32 value
    #[get(pub)]
    id: Int256,

    /// an u16 value
    #[get(pub)]
    update_counter: Int256,

    #[get(pub)]
    hash: H160,

    #[get(pub)]
    nef: ByteString,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractManifest {
    #[get(pub)]
    name: ByteString,

    #[get(pub)]
    groups: Array<ContractGroup>,

    _reserved: Any,

    #[get(pub)]
    supported_standards: Array<ByteString>,

    #[get(pub)]
    abi: ContractAbi,

    #[get(pub)]
    permissions: Array<ContractPermission>,

    #[get(pub)]
    trusts: Array<ByteString>,

    #[get(pub)]
    extra: ByteString,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractGroup {
    #[get(pub)]
    public_key: PublicKey,

    #[get(pub)]
    sign: ByteString,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractPermission {
    #[get(pub)]
    contract: ByteString,

    #[get(pub)]
    methods: Array<ByteString>,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractAbi {
    #[get(pub)]
    methods: Array<ContractMethodDescriptor>,

    #[get(pub)]
    events: Array<ContractEventDescriptor>,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractMethodDescriptor {
    #[get(pub)]
    name: ByteString,

    #[get(pub)]
    params: Array<ContractParam>,

    /// an u8 value, see ContractParamType
    #[get(pub)]
    return_type: Int256,

    /// an u32 value
    #[get(pub)]
    offset: Int256,

    #[get(pub)]
    safe: bool,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractEventDescriptor {
    #[get(pub)]
    name: ByteString,

    #[get(pub)]
    params: Array<ContractParam>,
}

#[repr(C)]
#[crate::inner_structs]
pub struct ContractParam {
    #[get(pub)]
    name: ByteString,

    /// an u8 value, see ContractParamType
    #[get(pub)]
    param_type: Int256,
}

#[repr(C)]
#[crate::inner_structs]
pub struct NeoCandidate {
    #[get(pub)]
    public_key: PublicKey,

    #[get(pub)]
    votes: Int256,
}

#[repr(C)]
#[crate::inner_structs]
pub struct NeoAccountState {
    #[get(pub)]
    balance: Int256,

    #[get(pub)]
    height: Int256,

    #[get(pub)]
    vote_to: PublicKey,
}
