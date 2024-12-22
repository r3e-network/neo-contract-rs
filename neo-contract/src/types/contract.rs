// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
pub struct ContractHash {
    id: u32,
    hash: H160,
}

#[repr(C)]
pub struct Contract {
    id: u32,
    update_counter: u32,
    hash: H160,
    nef: ByteString,
    manifest: ContractManifest,
}

impl Contract {
    #[inline(always)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline(always)]
    pub fn update_counter(&self) -> u32 {
        self.update_counter
    }

    #[inline(always)]
    pub fn hash(&self) -> H160 {
        self.hash
    }

    #[inline(always)]
    pub fn nef(&self) -> ByteString {
        self.nef.clone()
    }
}

#[repr(C)]
pub struct ContractManifest {
    name: ByteString,
    groups: Array<ContractGroup>,
    _reserved: Any,
    supported_standards: Array<ByteString>,
    abi: ContractAbi,
    permissions: Array<ContractPermission>,
    trusts: Array<ByteString>,
    extra: ByteString,
}

#[repr(C)]
pub struct ContractGroup {
    public_key: PublicKey,
    sign: ByteString,
}

#[repr(C)]
pub struct ContractPermission {
    contract: ByteString,
    methods: Array<ByteString>,
}

#[repr(C)]
pub struct ContractAbi {
    methods: Array<ContractMethodDescriptor>,
    events: Array<ContractEventDescriptor>,
}

#[repr(C)]
pub struct ContractMethodDescriptor {
    name: ByteString,
    params: Array<ContractParam>,
    return_type: ContractParamType,
    offset: u32,
    safe: bool,
}

#[repr(C)]
pub struct ContractEventDescriptor {
    name: ByteString,
    params: Array<ContractParam>,
}

#[repr(C)]
pub struct ContractParam {
    name: ByteString,
    param_type: ContractParamType,
}

#[repr(C)]
pub struct NeoCandidate {
    public_key: PublicKey,
    votes: Int256,
}

impl NeoCandidate {
    #[inline(always)]
    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    #[inline(always)]
    pub fn votes(&self) -> Int256 {
        self.votes
    }
}

#[repr(C)]
pub struct NeoAccountState {
    balance: Int256,
    height: Int256,
    vote_to: PublicKey,
}

impl NeoAccountState {
    #[inline(always)]
    pub fn balance(&self) -> Int256 {
        self.balance
    }

    #[inline(always)]
    pub fn height(&self) -> Int256 {
        self.height
    }

    #[inline(always)]
    pub fn vote_to(&self) -> PublicKey {
        self.vote_to.clone()
    }
}
