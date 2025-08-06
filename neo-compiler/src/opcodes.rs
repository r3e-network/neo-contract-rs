/// NEO VM Opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Constants
    PushInt8 = 0x00,
    PushInt16 = 0x01,
    PushInt32 = 0x02,
    PushInt64 = 0x03,
    PushInt128 = 0x04,
    PushInt256 = 0x05,
    PushTrue = 0x08,
    PushFalse = 0x09,
    PushNull = 0x0B,
    PushData1 = 0x0C,
    PushData2 = 0x0D,
    PushData4 = 0x0E,
    PushM1 = 0x0F,
    Push0 = 0x10,
    Push1 = 0x11,
    Push2 = 0x12,
    Push3 = 0x13,
    Push4 = 0x14,
    Push5 = 0x15,
    Push6 = 0x16,
    Push7 = 0x17,
    Push8 = 0x18,
    Push9 = 0x19,
    Push10 = 0x1A,
    Push11 = 0x1B,
    Push12 = 0x1C,
    Push13 = 0x1D,
    Push14 = 0x1E,
    Push15 = 0x1F,
    Push16 = 0x20,

    // Flow control
    Nop = 0x21,
    Jmp = 0x22,
    JmpL = 0x23,
    JmpIf = 0x24,
    JmpIfL = 0x25,
    JmpIfNot = 0x26,
    JmpIfNotL = 0x27,
    JmpEq = 0x28,
    JmpEqL = 0x29,
    JmpNe = 0x2A,
    JmpNeL = 0x2B,
    JmpGt = 0x2C,
    JmpGtL = 0x2D,
    JmpGe = 0x2E,
    JmpGeL = 0x2F,
    JmpLt = 0x30,
    JmpLtL = 0x31,
    JmpLe = 0x32,
    JmpLeL = 0x33,
    Call = 0x34,
    CallL = 0x35,
    CallA = 0x36,
    CallT = 0x37,
    Abort = 0x38,
    Assert = 0x39,
    Throw = 0x3A,
    Try = 0x3B,
    TryL = 0x3C,
    EndTry = 0x3D,
    EndTryL = 0x3E,
    EndFinally = 0x3F,
    Ret = 0x40,
    SysCall = 0x41,

    // Stack manipulation
    Depth = 0x43,
    Drop = 0x45,
    Nip = 0x46,
    Xdrop = 0x48,
    Clear = 0x49,
    Dup = 0x4A,
    Over = 0x4B,
    Pick = 0x4D,
    Tuck = 0x4E,
    Swap = 0x50,
    Rot = 0x51,
    Roll = 0x52,
    Reverse3 = 0x53,
    Reverse4 = 0x54,
    ReverseN = 0x55,
    InitSlot = 0x56,
    InitStaticSlot = 0x57,
    LdLocal0 = 0x58,
    LdLocal1 = 0x59,
    LdLocal2 = 0x5A,
    LdLocal3 = 0x5B,
    LdLocal4 = 0x5C,
    LdLocal5 = 0x5D,
    LdLocal6 = 0x5E,
    LdLocal = 0x5F,
    StLocal0 = 0x60,
    StLocal1 = 0x61,
    StLocal2 = 0x62,
    StLocal3 = 0x63,
    StLocal4 = 0x64,
    StLocal5 = 0x65,
    StLocal6 = 0x66,
    StLocal = 0x67,
    LdArg0 = 0x68,
    LdArg1 = 0x69,
    LdArg2 = 0x6A,
    LdArg3 = 0x6B,
    LdArg4 = 0x6C,
    LdArg5 = 0x6D,
    LdArg6 = 0x6E,
    LdArg = 0x6F,
    StArg0 = 0x70,
    StArg1 = 0x71,
    StArg2 = 0x72,
    StArg3 = 0x73,
    StArg4 = 0x74,
    StArg5 = 0x75,
    StArg6 = 0x76,
    StArg = 0x77,
    LdStatic0 = 0x78,
    LdStatic1 = 0x79,
    LdStatic2 = 0x7A,
    LdStatic3 = 0x7B,
    LdStatic4 = 0x7C,
    LdStatic5 = 0x7D,
    LdStatic6 = 0x7E,
    LdStatic = 0x7F,
    StStatic0 = 0x80,
    StStatic1 = 0x81,
    StStatic2 = 0x82,
    StStatic3 = 0x83,
    StStatic4 = 0x84,
    StStatic5 = 0x85,
    StStatic6 = 0x86,
    StStatic = 0x87,

    // Compound types
    NewBuffer = 0x88,
    MemCpy = 0x89,
    Cat = 0x8B,
    SubStr = 0x8C,
    Left = 0x8D,
    Right = 0x8E,
    InversionLen = 0x90,
    Size = 0x92,

    // Types
    NewArray0 = 0xC0,
    NewArray = 0xC1,
    NewArrayT = 0xC2,
    NewStruct0 = 0xC3,
    NewStruct = 0xC4,
    NewMap = 0xC5,
    Pack = 0xC6,
    Unpack = 0xC7,
    Append = 0xC8,
    SetItem = 0xC9,
    ReverseItems = 0xCA,
    Remove = 0xCB,
    ClearItems = 0xCC,
    PopItem = 0xCD,

    // Logical
    IsNull = 0xD0,
    IsType = 0xD1,
    Convert = 0xD2,
    AbortMsg = 0xD3,
    AssertMsg = 0xD4,

    // Arithmetic
    Add = 0x9B,
    Sub = 0x9C,
    Mul = 0x9D,
    Div = 0x9E,
    Mod = 0x9F,
    Pow = 0xA0,
    Sqrt = 0xA1,
    ModMul = 0xA2,
    ModPow = 0xA3,
    Shl = 0xA8,
    Shr = 0xA9,
    Not = 0xAA,
    BoolAnd = 0xAB,
    BoolOr = 0xAC,
    Nz = 0xB1,
    NumEqual = 0xB3,
    NumNotEqual = 0xB4,
    Lt = 0xB5,
    Le = 0xB6,
    Gt = 0xB7,
    Ge = 0xB8,
    Min = 0xB9,
    Max = 0xBA,
    Within = 0xBB,

    // Bitwise
    Invert = 0x91,
    And = 0x93,
    Or = 0x94,
    Xor = 0x95,
    Equal = 0x97,
    NotEqual = 0x98,
    Sign = 0x99,
    Abs = 0x9A,
    // Negate, Inc, Dec removed as they conflict with Add, Sub, Mul
}

impl OpCode {
    /// Convert from byte value
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::PushInt8),
            0x01 => Some(Self::PushInt16),
            0x02 => Some(Self::PushInt32),
            0x03 => Some(Self::PushInt64),
            0x04 => Some(Self::PushInt128),
            0x05 => Some(Self::PushInt256),
            0x08 => Some(Self::PushTrue),
            0x09 => Some(Self::PushFalse),
            0x0B => Some(Self::PushNull),
            0x0C => Some(Self::PushData1),
            0x0D => Some(Self::PushData2),
            0x0E => Some(Self::PushData4),
            0x0F => Some(Self::PushM1),
            0x10..=0x20 => Some(unsafe { std::mem::transmute(byte) }),
            0x21 => Some(Self::Nop),
            0x22 => Some(Self::Jmp),
            0x23 => Some(Self::JmpL),
            0x24 => Some(Self::JmpIf),
            0x25 => Some(Self::JmpIfL),
            0x26 => Some(Self::JmpIfNot),
            0x27 => Some(Self::JmpIfNotL),
            0x28 => Some(Self::JmpEq),
            0x29 => Some(Self::JmpEqL),
            0x2A => Some(Self::JmpNe),
            0x2B => Some(Self::JmpNeL),
            0x2C => Some(Self::JmpGt),
            0x2D => Some(Self::JmpGtL),
            0x2E => Some(Self::JmpGe),
            0x2F => Some(Self::JmpGeL),
            0x30 => Some(Self::JmpLt),
            0x31 => Some(Self::JmpLtL),
            0x32 => Some(Self::JmpLe),
            0x33 => Some(Self::JmpLeL),
            0x34 => Some(Self::Call),
            0x35 => Some(Self::CallL),
            0x36 => Some(Self::CallA),
            0x37 => Some(Self::CallT),
            0x38 => Some(Self::Abort),
            0x39 => Some(Self::Assert),
            0x3A => Some(Self::Throw),
            0x3B => Some(Self::Try),
            0x3C => Some(Self::TryL),
            0x3D => Some(Self::EndTry),
            0x3E => Some(Self::EndTryL),
            0x3F => Some(Self::EndFinally),
            0x40 => Some(Self::Ret),
            0x41 => Some(Self::SysCall),
            0x43 => Some(Self::Depth),
            0x45 => Some(Self::Drop),
            0x46 => Some(Self::Nip),
            0x48 => Some(Self::Xdrop),
            0x49 => Some(Self::Clear),
            0x4A => Some(Self::Dup),
            0x4B => Some(Self::Over),
            0x4D => Some(Self::Pick),
            0x4E => Some(Self::Tuck),
            0x50 => Some(Self::Swap),
            0x51 => Some(Self::Rot),
            0x52 => Some(Self::Roll),
            0x53 => Some(Self::Reverse3),
            0x54 => Some(Self::Reverse4),
            0x55 => Some(Self::ReverseN),
            0x56 => Some(Self::InitSlot),
            0x57 => Some(Self::InitStaticSlot),
            0x58..=0x87 => Some(unsafe { std::mem::transmute(byte) }),
            0x88 => Some(Self::NewBuffer),
            0x89 => Some(Self::MemCpy),
            0x8B => Some(Self::Cat),
            0x8C => Some(Self::SubStr),
            0x8D => Some(Self::Left),
            0x8E => Some(Self::Right),
            0x90 => Some(Self::InversionLen),
            0x91 => Some(Self::Invert),
            0x92 => Some(Self::Size),
            0x93 => Some(Self::And),
            0x94 => Some(Self::Or),
            0x95 => Some(Self::Xor),
            0x97 => Some(Self::Equal),
            0x98 => Some(Self::NotEqual),
            0x99 => Some(Self::Sign),
            0x9A => Some(Self::Abs),
            0x9B => Some(Self::Add),
            0x9C => Some(Self::Sub),
            0x9D => Some(Self::Mul),
            0x9E => Some(Self::Div),
            0x9F => Some(Self::Mod),
            0xA0 => Some(Self::Pow),
            0xA1 => Some(Self::Sqrt),
            0xA2 => Some(Self::ModMul),
            0xA3 => Some(Self::ModPow),
            0xA8 => Some(Self::Shl),
            0xA9 => Some(Self::Shr),
            0xAA => Some(Self::Not),
            0xAB => Some(Self::BoolAnd),
            0xAC => Some(Self::BoolOr),
            0xB1 => Some(Self::Nz),
            0xB3 => Some(Self::NumEqual),
            0xB4 => Some(Self::NumNotEqual),
            0xB5 => Some(Self::Lt),
            0xB6 => Some(Self::Le),
            0xB7 => Some(Self::Gt),
            0xB8 => Some(Self::Ge),
            0xB9 => Some(Self::Min),
            0xBA => Some(Self::Max),
            0xBB => Some(Self::Within),
            0xC0 => Some(Self::NewArray0),
            0xC1 => Some(Self::NewArray),
            0xC2 => Some(Self::NewArrayT),
            0xC3 => Some(Self::NewStruct0),
            0xC4 => Some(Self::NewStruct),
            0xC5 => Some(Self::NewMap),
            0xC6 => Some(Self::Pack),
            0xC7 => Some(Self::Unpack),
            0xC8 => Some(Self::Append),
            0xC9 => Some(Self::SetItem),
            0xCA => Some(Self::ReverseItems),
            0xCB => Some(Self::Remove),
            0xCC => Some(Self::ClearItems),
            0xCD => Some(Self::PopItem),
            0xD0 => Some(Self::IsNull),
            0xD1 => Some(Self::IsType),
            0xD2 => Some(Self::Convert),
            0xD3 => Some(Self::AbortMsg),
            0xD4 => Some(Self::AssertMsg),
            _ => None,
        }
    }

    /// Convert to byte value
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Check if this is a jump opcode
    pub fn is_jump(&self) -> bool {
        matches!(self,
            Self::Jmp | Self::JmpL |
            Self::JmpIf | Self::JmpIfL |
            Self::JmpIfNot | Self::JmpIfNotL |
            Self::JmpEq | Self::JmpEqL |
            Self::JmpNe | Self::JmpNeL |
            Self::JmpGt | Self::JmpGtL |
            Self::JmpGe | Self::JmpGeL |
            Self::JmpLt | Self::JmpLtL |
            Self::JmpLe | Self::JmpLeL
        )
    }

    /// Check if this is a call opcode
    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call | Self::CallL | Self::CallA | Self::CallT)
    }

    /// Get the size of the opcode including operands
    pub fn size(&self) -> usize {
        match self {
            Self::PushInt8 => 2,
            Self::PushInt16 => 3,
            Self::PushInt32 => 5,
            Self::PushInt64 => 9,
            Self::PushInt128 => 17,
            Self::PushInt256 => 33,
            Self::PushData1 => 2, // + data length
            Self::PushData2 => 3, // + data length
            Self::PushData4 => 5, // + data length
            Self::Jmp | Self::JmpIf | Self::JmpIfNot |
            Self::JmpEq | Self::JmpNe | Self::JmpGt |
            Self::JmpGe | Self::JmpLt | Self::JmpLe |
            Self::Call => 2,
            Self::JmpL | Self::JmpIfL | Self::JmpIfNotL |
            Self::JmpEqL | Self::JmpNeL | Self::JmpGtL |
            Self::JmpGeL | Self::JmpLtL | Self::JmpLeL |
            Self::CallL => 5,
            Self::Try => 3,
            Self::TryL => 9,
            Self::EndTry => 2,
            Self::EndTryL => 5,
            Self::InitSlot | Self::InitStaticSlot => 3,
            Self::LdLocal | Self::StLocal |
            Self::LdArg | Self::StArg |
            Self::LdStatic | Self::StStatic => 2,
            Self::SysCall => 5, // 1 + 4 bytes for syscall ID
            Self::NewArrayT | Self::IsType | Self::Convert => 2,
            _ => 1,
        }
    }
}

/// System call identifiers
pub struct SysCall;

impl SysCall {
    // System
    pub const SYSTEM_CONTRACT_CALL: u32 = 0x627D5B52;
    pub const SYSTEM_CONTRACT_CALL_NATIVE: u32 = 0x338312F3;
    pub const SYSTEM_CONTRACT_CREATE_STANDARD_ACCOUNT: u32 = 0x40A92E8B;
    pub const SYSTEM_CONTRACT_CREATE_MULTISIG_ACCOUNT: u32 = 0xDACE1648;
    pub const SYSTEM_CONTRACT_GET_CALL_FLAGS: u32 = 0xB7C7FACC;
    
    // Runtime
    pub const SYSTEM_RUNTIME_PLATFORM: u32 = 0xFDEA5D4E;
    pub const SYSTEM_RUNTIME_GET_TRIGGER: u32 = 0xD642A42E;
    pub const SYSTEM_RUNTIME_GET_TIME: u32 = 0xF157E7EA;
    pub const SYSTEM_RUNTIME_GET_SCRIPT_CONTAINER: u32 = 0x2D728D86;
    pub const SYSTEM_RUNTIME_GET_EXECUTING_SCRIPT_HASH: u32 = 0x632B6E29;
    pub const SYSTEM_RUNTIME_GET_CALLING_SCRIPT_HASH: u32 = 0x528D0D00;
    pub const SYSTEM_RUNTIME_GET_ENTRY_SCRIPT_HASH: u32 = 0x0BEEBEB4;
    pub const SYSTEM_RUNTIME_CHECK_WITNESS: u32 = 0x3307B520;
    pub const SYSTEM_RUNTIME_GET_INVOCATION_COUNTER: u32 = 0x2F729FF8;
    pub const SYSTEM_RUNTIME_GET_GAS_LEFT: u32 = 0x3A3B9A31;
    pub const SYSTEM_RUNTIME_GET_NOTIFICATIONS: u32 = 0x8166107A;
    pub const SYSTEM_RUNTIME_GET_NETWORK: u32 = 0x476DC615;
    pub const SYSTEM_RUNTIME_GET_RANDOM: u32 = 0x8D8B9E42;
    pub const SYSTEM_RUNTIME_LOG: u32 = 0x8F61E6CE;
    pub const SYSTEM_RUNTIME_NOTIFY: u32 = 0x9BF667CE;
    pub const SYSTEM_RUNTIME_GET_TX: u32 = 0x369426FF;
    pub const SYSTEM_RUNTIME_BURN_GAS: u32 = 0xCF561045;
    
    // Storage
    pub const SYSTEM_STORAGE_GET_CONTEXT: u32 = 0x9BF667CE;
    pub const SYSTEM_STORAGE_GET_READ_ONLY_CONTEXT: u32 = 0x161B7804;
    pub const SYSTEM_STORAGE_AS_READ_ONLY: u32 = 0xD4FB8203;
    pub const SYSTEM_STORAGE_GET: u32 = 0x925DE831;
    pub const SYSTEM_STORAGE_FIND: u32 = 0xC0695219;
    pub const SYSTEM_STORAGE_PUT: u32 = 0xE63F1884;
    pub const SYSTEM_STORAGE_DELETE: u32 = 0x6D625B09;
    
    // Crypto
    pub const SYSTEM_CRYPTO_CHECK_SIG: u32 = 0x41166107;
    pub const SYSTEM_CRYPTO_CHECK_MULTISIG: u32 = 0xD8258E93;
    pub const SYSTEM_CRYPTO_SHA256: u32 = 0x0FAAC4E6;
    pub const SYSTEM_CRYPTO_RIPEMD160: u32 = 0x7A806A87;
    pub const SYSTEM_CRYPTO_VERIFY_WITH_ECDSA: u32 = 0x95440D7E;
    pub const SYSTEM_CRYPTO_MURMUR32: u32 = 0x24C5C88A;
    
    // Iterator
    pub const SYSTEM_ITERATOR_CREATE: u32 = 0xD3CE96E7;
    pub const SYSTEM_ITERATOR_NEXT: u32 = 0x932BF322;
    pub const SYSTEM_ITERATOR_VALUE: u32 = 0x61B7C1E5;
    
    // JSON
    pub const SYSTEM_JSON_SERIALIZE: u32 = 0x7D12289B;
    pub const SYSTEM_JSON_DESERIALIZE: u32 = 0x8B2E8A15;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_conversion() {
        assert_eq!(OpCode::from_byte(0x00), Some(OpCode::PushInt8));
        assert_eq!(OpCode::from_byte(0x41), Some(OpCode::SysCall));
        assert_eq!(OpCode::from_byte(0xFF), None);
        
        assert_eq!(OpCode::Nop.to_byte(), 0x21);
        assert_eq!(OpCode::Ret.to_byte(), 0x40);
    }

    #[test]
    fn test_opcode_properties() {
        assert!(OpCode::Jmp.is_jump());
        assert!(OpCode::JmpIfL.is_jump());
        assert!(!OpCode::Call.is_jump());
        
        assert!(OpCode::Call.is_call());
        assert!(OpCode::CallL.is_call());
        assert!(!OpCode::Jmp.is_call());
    }

    #[test]
    fn test_opcode_sizes() {
        assert_eq!(OpCode::Nop.size(), 1);
        assert_eq!(OpCode::PushInt32.size(), 5);
        assert_eq!(OpCode::Jmp.size(), 2);
        assert_eq!(OpCode::JmpL.size(), 5);
        assert_eq!(OpCode::SysCall.size(), 5);
    }
}