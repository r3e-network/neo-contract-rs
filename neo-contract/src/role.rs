// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// Represents the roles in the NEO system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The validators of state. Used to generate and sign the state root.
    StateValidator = 4,

    /// The nodes used to process Oracle requests.
    Oracle = 8,

    /// NeoFS Alphabet nodes.
    NeoFSAlphabetNode = 16,

    /// P2P Notary nodes used to process P2P notary requests.
    P2PNotary = 32
}
