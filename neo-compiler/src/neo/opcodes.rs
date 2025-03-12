//! Neo VM instruction opcodes
//!
//! This module contains definitions for all Neo VM opcodes,
//! used in Neo N3 smart contract scripts.
//! Based on the official Neo C# implementation.

/// Represents the opcode of an instruction in the Neo VM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Constants
    // COMMENT opcode removed - not supported in Neo N3
    /// Pushes a 1-byte signed integer onto the stack.
    PUSHINT8 = 0x00,
    /// Pushes a 2-bytes signed integer onto the stack.
    PUSHINT16 = 0x01,
    /// Pushes a 4-bytes signed integer onto the stack.
    PUSHINT32 = 0x02,
    /// Pushes an 8-bytes signed integer onto the stack.
    PUSHINT64 = 0x03,
    /// Pushes a 16-bytes signed integer onto the stack.
    PUSHINT128 = 0x04,
    /// Pushes a 32-bytes signed integer onto the stack.
    PUSHINT256 = 0x05,
    /// Pushes the boolean value `true` onto the stack.
    PUSHT = 0x08,
    /// Pushes the boolean value `false` onto the stack.
    PUSHF = 0x09,
    /// Converts the 4-bytes offset to a `Pointer`, and pushes it onto the stack.
    PUSHA = 0x0A,
    /// The item `null` is pushed onto the stack.
    PUSHNULL = 0x0B,
    /// The next byte contains the number of bytes to be pushed onto the stack.
    PUSHDATA1 = 0x0C,
    /// The next two bytes contain the number of bytes to be pushed onto the stack.
    PUSHDATA2 = 0x0D,
    /// The next four bytes contain the number of bytes to be pushed onto the stack.
    PUSHDATA4 = 0x0E,
    /// The number -1 is pushed onto the stack.
    PUSHM1 = 0x0F,
    /// The number 0 is pushed onto the stack.
    PUSH0 = 0x10,
    /// The number 1 is pushed onto the stack.
    PUSH1 = 0x11,
    /// The number 2 is pushed onto the stack.
    PUSH2 = 0x12,
    /// The number 3 is pushed onto the stack.
    PUSH3 = 0x13,
    /// The number 4 is pushed onto the stack.
    PUSH4 = 0x14,
    /// The number 5 is pushed onto the stack.
    PUSH5 = 0x15,
    /// The number 6 is pushed onto the stack.
    PUSH6 = 0x16,
    /// The number 7 is pushed onto the stack.
    PUSH7 = 0x17,
    /// The number 8 is pushed onto the stack.
    PUSH8 = 0x18,
    /// The number 9 is pushed onto the stack.
    PUSH9 = 0x19,
    /// The number 10 is pushed onto the stack.
    PUSH10 = 0x1A,
    /// The number 11 is pushed onto the stack.
    PUSH11 = 0x1B,
    /// The number 12 is pushed onto the stack.
    PUSH12 = 0x1C,
    /// The number 13 is pushed onto the stack.
    PUSH13 = 0x1D,
    /// The number 14 is pushed onto the stack.
    PUSH14 = 0x1E,
    /// The number 15 is pushed onto the stack.
    PUSH15 = 0x1F,
    /// The number 16 is pushed onto the stack.
    PUSH16 = 0x20,

    // Flow control
    /// The `NOP` operation does nothing. It is intended to fill in space if opcodes are patched.
    NOP = 0x21,
    /// Unconditionally transfers control to a target instruction. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMP = 0x22,
    /// Unconditionally transfers control to a target instruction. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpL = 0x23,
    /// Transfers control to a target instruction if the value is `true`, not `null`, or non-zero. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPIF = 0x24,
    /// Transfers control to a target instruction if the value is `true`, not `null`, or non-zero. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpifL = 0x25,
    /// Transfers control to a target instruction if the value is `false`, a `null` reference, or zero. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPIFNOT = 0x26,
    /// Transfers control to a target instruction if the value is `false`, a `null` reference, or zero. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpifnotL = 0x27,
    /// Transfers control to a target instruction if two values are equal. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPEQ = 0x28,
    /// Transfers control to a target instruction if two values are equal. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpeqL = 0x29,
    /// Transfers control to a target instruction when two values are not equal. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPNE = 0x2A,
    /// Transfers control to a target instruction when two values are not equal. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpneL = 0x2B,
    /// Transfers control to a target instruction if the first value is greater than the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPGT = 0x2C,
    /// Transfers control to a target instruction if the first value is greater than the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpgtL = 0x2D,
    /// Transfers control to a target instruction if the first value is greater than or equal to the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPGE = 0x2E,
    /// Transfers control to a target instruction if the first value is greater than or equal to the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpgeL = 0x2F,
    /// Transfers control to a target instruction if the first value is less than the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPLT = 0x30,
    /// Transfers control to a target instruction if the first value is less than the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpltL = 0x31,
    /// Transfers control to a target instruction if the first value is less than or equal to the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    JMPLE = 0x32,
    /// Transfers control to a target instruction if the first value is less than or equal to the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    JmpleL = 0x33,
    /// Calls the function at the target address which is represented as a 1-byte signed offset from the beginning of the current instruction.
    CALL = 0x34,
    /// Calls the function at the target address which is represented as a 4-bytes signed offset from the beginning of the current instruction.
    CallL = 0x35,
    /// Pop the address of a function from the stack, and call the function.
    CALLA = 0x36,
    /// Calls the function which is described by the token.
    CALLT = 0x37,
    /// It turns the vm state to FAULT immediately, and cannot be caught.
    ABORT = 0x38,
    /// Pop the top value of the stack. If it's false, exit vm execution and set vm state to FAULT.
    ASSERT = 0x39,
    /// Pop the top value of the stack, and throw it.
    THROW = 0x3A,
    /// TRY CatchOffset(sbyte) FinallyOffset(sbyte). If there's no catch body, set CatchOffset 0. If there's no finally body, set FinallyOffset 0.
    TRY = 0x3B,
    /// TRY_L CatchOffset(int) FinallyOffset(int). If there's no catch body, set CatchOffset 0. If there's no finally body, set FinallyOffset 0.
    TryL = 0x3C,
    /// Ensures that the appropriate surrounding finally blocks are executed. And then unconditionally transfers control to the specific target instruction, represented as a 1-byte signed offset from the beginning of the current instruction.
    ENDTRY = 0x3D,
    /// Ensures that the appropriate surrounding finally blocks are executed. And then unconditionally transfers control to the specific target instruction, represented as a 4-bytes signed offset from the beginning of the current instruction.
    EndtryL = 0x3E,
    /// End finally, If no exception happen or be catched, vm will jump to the target instruction of ENDTRY/ENDTRY_L. Otherwise, vm will rethrow the exception to upper layer.
    ENDFINALLY = 0x3F,
    /// Returns from the current method.
    RET = 0x40,
    /// Calls to an interop service.
    SYSCALL = 0x41,

    // Stack
    /// Puts the number of stack items onto the stack.
    DEPTH = 0x43,
    /// Removes the top stack item.
    DROP = 0x45,
    /// Removes the second-to-top stack item.
    NIP = 0x46,
    /// The item n back in the main stack is removed.
    XDROP = 0x48,
    /// Clear the stack
    CLEAR = 0x49,
    /// Duplicates the top stack item.
    DUP = 0x4A,
    /// Copies the second-to-top stack item to the top.
    OVER = 0x4B,
    /// The item n back in the stack is copied to the top.
    PICK = 0x4D,
    /// The item at the top of the stack is copied and inserted before the second-to-top item.
    TUCK = 0x4E,
    /// The top two items on the stack are swapped.
    SWAP = 0x50,
    /// The top three items on the stack are rotated to the left.
    ROT = 0x51,
    /// The item n back in the stack is moved to the top.
    ROLL = 0x52,
    /// Reverse the order of the top 3 items on the stack.
    REVERSE3 = 0x53,
    /// Reverse the order of the top 4 items on the stack.
    REVERSE4 = 0x54,
    /// Pop the number N on the stack, and reverse the order of the top N items on the stack.
    REVERSEN = 0x55,
    /// Push the next byte onto the stack as a bytestring.
    PUSHBYTES1 = 0x44,
    /// Push the next 2 bytes onto the stack as a bytestring.
    PUSHBYTES2 = 0x42,

    // Slot
    /// Initialize the static field list for the current execution context.
    INITSSLOT = 0x56,
    /// Initialize the argument slot and the local variable list for the current execution context.
    INITSLOT = 0x57,
    /// Loads the static field at index 0 onto the evaluation stack.
    LDSFLD0 = 0x58,
    /// Loads the static field at index 1 onto the evaluation stack.
    LDSFLD1 = 0x59,
    /// Loads the static field at index 2 onto the evaluation stack.
    LDSFLD2 = 0x5A,
    /// Loads the static field at index 3 onto the evaluation stack.
    LDSFLD3 = 0x5B,
    /// Loads the static field at index 4 onto the evaluation stack.
    LDSFLD4 = 0x5C,
    /// Loads the static field at index 5 onto the evaluation stack.
    LDSFLD5 = 0x5D,
    /// Loads the static field at index 6 onto the evaluation stack.
    LDSFLD6 = 0x5E,
    /// Loads the static field at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    LDSFLD = 0x5F,
    /// Stores the value on top of the evaluation stack in the static field list at index 0.
    STSFLD0 = 0x60,
    /// Stores the value on top of the evaluation stack in the static field list at index 1.
    STSFLD1 = 0x61,
    /// Stores the value on top of the evaluation stack in the static field list at index 2.
    STSFLD2 = 0x62,
    /// Stores the value on top of the evaluation stack in the static field list at index 3.
    STSFLD3 = 0x63,
    /// Stores the value on top of the evaluation stack in the static field list at index 4.
    STSFLD4 = 0x64,
    /// Stores the value on top of the evaluation stack in the static field list at index 5.
    STSFLD5 = 0x65,
    /// Stores the value on top of the evaluation stack in the static field list at index 6.
    STSFLD6 = 0x66,
    /// Stores the value on top of the evaluation stack in the static field list at a specified index. The index is represented as a 1-byte unsigned integer.
    STSFLD = 0x67,
    /// Loads the local variable at index 0 onto the evaluation stack.
    LDLOC0 = 0x68,
    /// Loads the local variable at index 1 onto the evaluation stack.
    LDLOC1 = 0x69,
    /// Loads the local variable at index 2 onto the evaluation stack.
    LDLOC2 = 0x6A,
    /// Loads the local variable at index 3 onto the evaluation stack.
    LDLOC3 = 0x6B,
    /// Loads the local variable at index 4 onto the evaluation stack.
    LDLOC4 = 0x6C,
    /// Loads the local variable at index 5 onto the evaluation stack.
    LDLOC5 = 0x6D,
    /// Loads the local variable at index 6 onto the evaluation stack.
    LDLOC6 = 0x6E,
    /// Loads the local variable at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    LDLOC = 0x6F,
    /// Stores the value on top of the evaluation stack in the local variable list at index 0.
    STLOC0 = 0x70,
    /// Stores the value on top of the evaluation stack in the local variable list at index 1.
    STLOC1 = 0x71,
    /// Stores the value on top of the evaluation stack in the local variable list at index 2.
    STLOC2 = 0x72,
    /// Stores the value on top of the evaluation stack in the local variable list at index 3.
    STLOC3 = 0x73,
    /// Stores the value on top of the evaluation stack in the local variable list at index 4.
    STLOC4 = 0x74,
    /// Stores the value on top of the evaluation stack in the local variable list at index 5.
    STLOC5 = 0x75,
    /// Stores the value on top of the evaluation stack in the local variable list at index 6.
    STLOC6 = 0x76,
    /// Stores the value on top of the evaluation stack in the local variable list at a specified index. The index is represented as a 1-byte unsigned integer.
    STLOC = 0x77,
    /// Loads the argument at index 0 onto the evaluation stack.
    LDARG0 = 0x78,
    /// Loads the argument at index 1 onto the evaluation stack.
    LDARG1 = 0x79,
    /// Loads the argument at index 2 onto the evaluation stack.
    LDARG2 = 0x7A,
    /// Loads the argument at index 3 onto the evaluation stack.
    LDARG3 = 0x7B,
    /// Loads the argument at index 4 onto the evaluation stack.
    LDARG4 = 0x7C,
    /// Loads the argument at index 5 onto the evaluation stack.
    LDARG5 = 0x7D,
    /// Loads the argument at index 6 onto the evaluation stack.
    LDARG6 = 0x7E,
    /// Loads the argument at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    LDARG = 0x7F,
    /// Stores the value on top of the evaluation stack in the argument slot at index 0.
    STARG0 = 0x80,
    /// Stores the value on top of the evaluation stack in the argument slot at index 1.
    STARG1 = 0x81,
    /// Stores the value on top of the evaluation stack in the argument slot at index 2.
    STARG2 = 0x82,
    /// Stores the value on top of the evaluation stack in the argument slot at index 3.
    STARG3 = 0x83,
    /// Stores the value on top of the evaluation stack in the argument slot at index 4.
    STARG4 = 0x84,
    /// Stores the value on top of the evaluation stack in the argument slot at index 5.
    STARG5 = 0x85,
    /// Stores the value on top of the evaluation stack in the argument slot at index 6.
    STARG6 = 0x86,
    /// Stores the value on top of the evaluation stack in the argument slot at a specified index. The index is represented as a 1-byte unsigned integer.
    STARG = 0x87,

    // Splice
    /// Creates a new `Buffer` and pushes it onto the stack.
    NEWBUFFER = 0x88,
    /// Copies a range of bytes from one `Buffer` to another.
    MEMCPY = 0x89,
    /// Concatenates two strings.
    CAT = 0x8B,
    /// Returns a section of a string.
    SUBSTR = 0x8C,
    /// Keeps only characters left of the specified point in a string.
    LEFT = 0x8D,
    /// Keeps only characters right of the specified point in a string.
    RIGHT = 0x8E,

    // Bitwise logic
    /// Flips all the bits in the input.
    INVERT = 0x90,
    /// Boolean and between each bit in the inputs.
    AND = 0x91,
    /// Boolean or between each bit in the inputs.
    OR = 0x92,
    /// Boolean exclusive or between each bit in the inputs.
    XOR = 0x93,
    /// Returns 1 if the inputs are exactly equal, 0 otherwise.
    EQUAL = 0x97,
    /// Returns 1 if the inputs are not equal, 0 otherwise.
    NOTEQUAL = 0x98,

    // Arithmetic
    /// Puts the sign of top stack item on top of the main stack. If value is negative, put -1; if positive, put 1; if value is zero, put 0.
    SIGN = 0x99,
    /// The input is made positive.
    ABS = 0x9A,
    /// The sign of the input is flipped.
    NEGATE = 0x9B,
    /// 1 is added to the input.
    INC = 0x9C,
    /// 1 is subtracted from the input.
    DEC = 0x9D,
    /// a is added to b.
    ADD = 0x9E,
    /// b is subtracted from a.
    SUB = 0x9F,
    /// a is multiplied by b.
    MUL = 0xA0,
    /// a is divided by b.
    DIV = 0xA1,
    /// Returns the remainder after dividing a by b.
    MOD = 0xA2,
    /// The result of raising value to the exponent power.
    POW = 0xA3,
    /// Returns the square root of a specified number.
    SQRT = 0xA4,
    /// Performs modulus division on a number multiplied by another number.
    MODMUL = 0xA5,
    /// Performs modulus division on a number raised to the power of another number. If the exponent is -1, it will have the calculation of the modular inverse.
    MODPOW = 0xA6,
    /// Shifts a left b bits, preserving sign.
    SHL = 0xA8,
    /// Shifts a right b bits, preserving sign.
    SHR = 0xA9,
    /// If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
    NOT = 0xAA,
    /// If both a and b are not 0, the output is 1. Otherwise, 0.
    BOOLAND = 0xAB,
    /// If a or b is not 0, the output is 1. Otherwise, 0.
    BOOLOR = 0xAC,
    /// Returns 0 if the input is 0. 1 otherwise.
    NZ = 0xB1,
    /// Returns 1 if the numbers are equal, 0 otherwise.
    NUMEQUAL = 0xB3,
    /// Returns 1 if the numbers are not equal, 0 otherwise.
    NUMNOTEQUAL = 0xB4,
    /// Returns 1 if a is less than b, 0 otherwise.
    LT = 0xB5,
    /// Returns 1 if a is less than or equal to b, 0 otherwise.
    LE = 0xB6,
    /// Returns 1 if a is greater than b, 0 otherwise.
    GT = 0xB7,
    /// Returns 1 if a is greater than or equal to b, 0 otherwise.
    GE = 0xB8,
    /// Returns the smallest of a and b.
    MIN = 0xB9,
    /// Returns the largest of a and b.
    MAX = 0xBA,
    /// Returns 1 if x is within the specified range (left-inclusive), 0 otherwise.
    WITHIN = 0xBB,

    // Compound-type
    /// A value n is taken from top of main stack. The next n*2 items on main stack are removed, put inside n-sized map and this map is put on top of the main stack.
    PACKMAP = 0xBE,
    /// A value n is taken from top of main stack. The next n items on main stack are removed, put inside n-sized struct and this struct is put on top of the main stack.
    PACKSTRUCT = 0xBF,
    /// A value n is taken from top of main stack. The next n items on main stack are removed, put inside n-sized array and this array is put on top of the main stack.
    PACK = 0xC0,
    /// A collection is removed from top of the main stack. Its elements are put on top of the main stack (in reverse order) and the collection size is also put on main stack.
    UNPACK = 0xC1,
    /// An empty array (with size 0) is put on top of the main stack.
    NEWARRAY0 = 0xC2,
    /// A value n is taken from top of main stack. A null-filled array with size n is put on top of the main stack.
    NEWARRAY = 0xC3,
    /// A value n is taken from top of main stack. An array of type T with size n is put on top of the main stack.
    NewarrayT = 0xC4,
    /// An empty struct (with size 0) is put on top of the main stack.
    NEWSTRUCT0 = 0xC5,
    /// A value n is taken from top of main stack. A zero-filled struct with size n is put on top of the main stack.
    NEWSTRUCT = 0xC6,
    /// A Map is created and put on top of the main stack.
    NEWMAP = 0xC8,
    /// An array is removed from top of the main stack. Its size is put on top of the main stack.
    SIZE = 0xCA,
    /// Gets the number of elements in an array or the number of key-value pairs in a map.
    ARRAYSIZE = 0xC9,
    /// An input index n (or key) and an array (or map) are removed from the top of the main stack. Puts True on top of main stack if array[n] (or map[n]) exist, and False otherwise.
    HASKEY = 0xCB,
    /// A map is taken from top of the main stack. The keys of this map are put on top of the main stack.
    KEYS = 0xCC,
    /// A map is taken from top of the main stack. The values of this map are put on top of the main stack.
    VALUES = 0xCD,
    /// An input index n (or key) and an array (or map) are taken from main stack. Element array[n] (or map[n]) is put on top of the main stack.
    PICKITEM = 0xCE,
    /// The item on top of main stack is removed and appended to the second item on top of the main stack.
    APPEND = 0xCF,
    /// A value v, index n (or key) and an array (or map) are taken from main stack. Attribution array[n]=v (or map[n]=v) is performed.
    SETITEM = 0xD0,
    /// An array is removed from the top of the main stack and its elements are reversed.
    REVERSEITEMS = 0xD1,
    /// An input index n (or key) and an array (or map) are removed from the top of the main stack. Element array[n] (or map[n]) is removed.
    REMOVE = 0xD2,
    /// Remove all the items from the compound-type.
    CLEARITEMS = 0xD3,
    /// Remove the last element from an array, and push it onto the stack.
    POPITEM = 0xD4,

    // Types
    /// Returns `true` if the input is `null`; `false` otherwise.
    ISNULL = 0xD8,
    /// Returns `true` if the top item of the stack is of the specified type; `false` otherwise.
    ISTYPE = 0xD9,
    /// Converts the top item of the stack to the specified type.
    CONVERT = 0xDB,

    // Extensions
    /// Pops the top stack item. Then, turns the vm state to FAULT immediately, and cannot be caught. The top stack value is used as reason.
    ABORTMSG = 0xE0,
    /// Pops the top two stack items. If the second-to-top stack value is false, exits the vm execution and sets the vm state to FAULT. In this case, the top stack value is used as reason for the exit. Otherwise, it is ignored.
    ASSERTMSG = 0xE1,

    /// Pushes an integer onto the stack.
    PUSHINT = 0xE2,
}

impl OpCode {
    /// Convert a byte to an OpCode
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            // 0xFE => Removed COMMENT opcode as it's not supported in Neo N3
            0x00 => Some(OpCode::PUSHINT8),
            0x01 => Some(OpCode::PUSHINT16),
            0x02 => Some(OpCode::PUSHINT32),
            0x03 => Some(OpCode::PUSHINT64),
            0x04 => Some(OpCode::PUSHINT128),
            0x05 => Some(OpCode::PUSHINT256),
            0x08 => Some(OpCode::PUSHT),
            0x09 => Some(OpCode::PUSHF),
            0x0A => Some(OpCode::PUSHA),
            0x0B => Some(OpCode::PUSHNULL),
            0x0C => Some(OpCode::PUSHDATA1),
            0x0D => Some(OpCode::PUSHDATA2),
            0x0E => Some(OpCode::PUSHDATA4),
            0x0F => Some(OpCode::PUSHM1),
            0x10 => Some(OpCode::PUSH0),
            0x11 => Some(OpCode::PUSH1),
            0x12 => Some(OpCode::PUSH2),
            0x13 => Some(OpCode::PUSH3),
            0x14 => Some(OpCode::PUSH4),
            0x15 => Some(OpCode::PUSH5),
            0x16 => Some(OpCode::PUSH6),
            0x17 => Some(OpCode::PUSH7),
            0x18 => Some(OpCode::PUSH8),
            0x19 => Some(OpCode::PUSH9),
            0x1A => Some(OpCode::PUSH10),
            0x1B => Some(OpCode::PUSH11),
            0x1C => Some(OpCode::PUSH12),
            0x1D => Some(OpCode::PUSH13),
            0x1E => Some(OpCode::PUSH14),
            0x1F => Some(OpCode::PUSH15),
            0x20 => Some(OpCode::PUSH16),
            0x21 => Some(OpCode::NOP),
            0x22 => Some(OpCode::JMP),
            0x23 => Some(OpCode::JmpL),
            0x24 => Some(OpCode::JMPIF),
            0x25 => Some(OpCode::JmpifL),
            0x26 => Some(OpCode::JMPIFNOT),
            0x27 => Some(OpCode::JmpifnotL),
            0x28 => Some(OpCode::JMPEQ),
            0x29 => Some(OpCode::JmpeqL),
            0x2A => Some(OpCode::JMPNE),
            0x2B => Some(OpCode::JmpneL),
            0x2C => Some(OpCode::JMPGT),
            0x2D => Some(OpCode::JmpgtL),
            0x2E => Some(OpCode::JMPGE),
            0x2F => Some(OpCode::JmpgeL),
            0x30 => Some(OpCode::JMPLT),
            0x31 => Some(OpCode::JmpltL),
            0x32 => Some(OpCode::JMPLE),
            0x33 => Some(OpCode::JmpleL),
            0x34 => Some(OpCode::CALL),
            0x35 => Some(OpCode::CallL),
            0x36 => Some(OpCode::CALLA),
            0x37 => Some(OpCode::CALLT),
            0x38 => Some(OpCode::ABORT),
            0x39 => Some(OpCode::ASSERT),
            0x3A => Some(OpCode::THROW),
            0x3B => Some(OpCode::TRY),
            0x3C => Some(OpCode::TryL),
            0x3D => Some(OpCode::ENDTRY),
            0x3E => Some(OpCode::EndtryL),
            0x3F => Some(OpCode::ENDFINALLY),
            0x40 => Some(OpCode::RET),
            0x41 => Some(OpCode::SYSCALL),
            0x43 => Some(OpCode::DEPTH),
            0x45 => Some(OpCode::DROP),
            0x46 => Some(OpCode::NIP),
            0x48 => Some(OpCode::XDROP),
            0x49 => Some(OpCode::CLEAR),
            0x4A => Some(OpCode::DUP),
            0x4B => Some(OpCode::OVER),
            0x4D => Some(OpCode::PICK),
            0x4E => Some(OpCode::TUCK),
            0x50 => Some(OpCode::SWAP),
            0x51 => Some(OpCode::ROT),
            0x52 => Some(OpCode::ROLL),
            0x53 => Some(OpCode::REVERSE3),
            0x54 => Some(OpCode::REVERSE4),
            0x55 => Some(OpCode::REVERSEN),
            0x56 => Some(OpCode::INITSSLOT),
            0x57 => Some(OpCode::INITSLOT),
            0x58 => Some(OpCode::LDSFLD0),
            0x59 => Some(OpCode::LDSFLD1),
            0x5A => Some(OpCode::LDSFLD2),
            0x5B => Some(OpCode::LDSFLD3),
            0x5C => Some(OpCode::LDSFLD4),
            0x5D => Some(OpCode::LDSFLD5),
            0x5E => Some(OpCode::LDSFLD6),
            0x5F => Some(OpCode::LDSFLD),
            0x60 => Some(OpCode::STSFLD0),
            0x61 => Some(OpCode::STSFLD1),
            0x62 => Some(OpCode::STSFLD2),
            0x63 => Some(OpCode::STSFLD3),
            0x64 => Some(OpCode::STSFLD4),
            0x65 => Some(OpCode::STSFLD5),
            0x66 => Some(OpCode::STSFLD6),
            0x67 => Some(OpCode::STSFLD),
            0x68 => Some(OpCode::LDLOC0),
            0x69 => Some(OpCode::LDLOC1),
            0x6A => Some(OpCode::LDLOC2),
            0x6B => Some(OpCode::LDLOC3),
            0x6C => Some(OpCode::LDLOC4),
            0x6D => Some(OpCode::LDLOC5),
            0x6E => Some(OpCode::LDLOC6),
            0x6F => Some(OpCode::LDLOC),
            0x70 => Some(OpCode::STLOC0),
            0x71 => Some(OpCode::STLOC1),
            0x72 => Some(OpCode::STLOC2),
            0x73 => Some(OpCode::STLOC3),
            0x74 => Some(OpCode::STLOC4),
            0x75 => Some(OpCode::STLOC5),
            0x76 => Some(OpCode::STLOC6),
            0x77 => Some(OpCode::STLOC),
            0x78 => Some(OpCode::LDARG0),
            0x79 => Some(OpCode::LDARG1),
            0x7A => Some(OpCode::LDARG2),
            0x7B => Some(OpCode::LDARG3),
            0x7C => Some(OpCode::LDARG4),
            0x7D => Some(OpCode::LDARG5),
            0x7E => Some(OpCode::LDARG6),
            0x7F => Some(OpCode::LDARG),
            0x80 => Some(OpCode::STARG0),
            0x81 => Some(OpCode::STARG1),
            0x82 => Some(OpCode::STARG2),
            0x83 => Some(OpCode::STARG3),
            0x84 => Some(OpCode::STARG4),
            0x85 => Some(OpCode::STARG5),
            0x86 => Some(OpCode::STARG6),
            0x87 => Some(OpCode::STARG),
            0x88 => Some(OpCode::NEWBUFFER),
            0x89 => Some(OpCode::MEMCPY),
            0x8B => Some(OpCode::CAT),
            0x8C => Some(OpCode::SUBSTR),
            0x8D => Some(OpCode::LEFT),
            0x8E => Some(OpCode::RIGHT),
            0x90 => Some(OpCode::INVERT),
            0x91 => Some(OpCode::AND),
            0x92 => Some(OpCode::OR),
            0x93 => Some(OpCode::XOR),
            0x97 => Some(OpCode::EQUAL),
            0x98 => Some(OpCode::NOTEQUAL),
            0x99 => Some(OpCode::SIGN),
            0x9A => Some(OpCode::ABS),
            0x9B => Some(OpCode::NEGATE),
            0x9C => Some(OpCode::INC),
            0x9D => Some(OpCode::DEC),
            0x9E => Some(OpCode::ADD),
            0x9F => Some(OpCode::SUB),
            0xA0 => Some(OpCode::MUL),
            0xA1 => Some(OpCode::DIV),
            0xA2 => Some(OpCode::MOD),
            0xA3 => Some(OpCode::POW),
            0xA4 => Some(OpCode::SQRT),
            0xA5 => Some(OpCode::MODMUL),
            0xA6 => Some(OpCode::MODPOW),
            0xA8 => Some(OpCode::SHL),
            0xA9 => Some(OpCode::SHR),
            0xAA => Some(OpCode::NOT),
            0xAB => Some(OpCode::BOOLAND),
            0xAC => Some(OpCode::BOOLOR),
            0xB1 => Some(OpCode::NZ),
            0xB3 => Some(OpCode::NUMEQUAL),
            0xB4 => Some(OpCode::NUMNOTEQUAL),
            0xB5 => Some(OpCode::LT),
            0xB6 => Some(OpCode::LE),
            0xB7 => Some(OpCode::GT),
            0xB8 => Some(OpCode::GE),
            0xB9 => Some(OpCode::MIN),
            0xBA => Some(OpCode::MAX),
            0xBB => Some(OpCode::WITHIN),
            0xBE => Some(OpCode::PACKMAP),
            0xBF => Some(OpCode::PACKSTRUCT),
            0xC0 => Some(OpCode::PACK),
            0xC1 => Some(OpCode::UNPACK),
            0xC2 => Some(OpCode::NEWARRAY0),
            0xC3 => Some(OpCode::NEWARRAY),
            0xC4 => Some(OpCode::NewarrayT),
            0xC5 => Some(OpCode::NEWSTRUCT0),
            0xC6 => Some(OpCode::NEWSTRUCT),
            0xC8 => Some(OpCode::NEWMAP),
            0xCA => Some(OpCode::SIZE),
            0xCB => Some(OpCode::HASKEY),
            0xCC => Some(OpCode::KEYS),
            0xCD => Some(OpCode::VALUES),
            0xCE => Some(OpCode::PICKITEM),
            0xCF => Some(OpCode::APPEND),
            0xD0 => Some(OpCode::SETITEM),
            0xD1 => Some(OpCode::REVERSEITEMS),
            0xD2 => Some(OpCode::REMOVE),
            0xD3 => Some(OpCode::CLEARITEMS),
            0xD4 => Some(OpCode::POPITEM),
            0xD8 => Some(OpCode::ISNULL),
            0xD9 => Some(OpCode::ISTYPE),
            0xDB => Some(OpCode::CONVERT),
            0xE0 => Some(OpCode::ABORTMSG),
            0xE1 => Some(OpCode::ASSERTMSG),
            _ => None,
        }
    }

    /// Get the size of operand for this opcode
    pub fn operand_size(&self) -> usize {
        match self {
            // COMMENT opcode removed as it's not supported in Neo N3
            OpCode::PUSHINT8 => 1,
            OpCode::PUSHINT16 => 2,
            OpCode::PUSHINT32 => 4,
            OpCode::PUSHINT64 => 8,
            OpCode::PUSHINT128 => 16,
            OpCode::PUSHINT256 => 32,
            OpCode::PUSHA => 4,
            OpCode::PUSHDATA1 => 0, // Variable size, determined by the byte after the opcode
            OpCode::PUSHDATA2 => 0, // Variable size, determined by 2 bytes after the opcode
            OpCode::PUSHDATA4 => 0, // Variable size, determined by 4 bytes after the opcode
            OpCode::JMP => 1,
            OpCode::JmpL => 4,
            OpCode::JMPIF => 1,
            OpCode::JmpifL => 4,
            OpCode::JMPIFNOT => 1,
            OpCode::JmpifnotL => 4,
            OpCode::JMPEQ => 1,
            OpCode::JmpeqL => 4,
            OpCode::JMPNE => 1,
            OpCode::JmpneL => 4,
            OpCode::JMPGT => 1,
            OpCode::JmpgtL => 4,
            OpCode::JMPGE => 1,
            OpCode::JmpgeL => 4,
            OpCode::JMPLT => 1,
            OpCode::JmpltL => 4,
            OpCode::JMPLE => 1,
            OpCode::JmpleL => 4,
            OpCode::CALL => 1,
            OpCode::CallL => 4,
            OpCode::CALLT => 2,
            OpCode::SYSCALL => 4,
            OpCode::TRY => 2,
            OpCode::TryL => 8,
            OpCode::ENDTRY => 1,
            OpCode::EndtryL => 4,
            OpCode::INITSSLOT => 1,
            OpCode::INITSLOT => 2,
            OpCode::LDSFLD => 1,
            OpCode::STSFLD => 1,
            OpCode::LDLOC => 1,
            OpCode::STLOC => 1,
            OpCode::LDARG => 1,
            OpCode::STARG => 1,
            OpCode::NewarrayT => 1,
            OpCode::ISTYPE => 1,
            OpCode::CONVERT => 1,
            _ => 0,
        }
    }

    /// Checks if this opcode has a variable operand size
    pub fn has_variable_operand_size(&self) -> bool {
        matches!(self, OpCode::PUSHDATA1 | OpCode::PUSHDATA2 | OpCode::PUSHDATA4)
    }

    /// Get the size prefix for operands with variable size
    pub fn size_prefix(&self) -> Option<usize> {
        match self {
            // COMMENT opcode removed as it's not supported in Neo N3
            OpCode::PUSHDATA1 => Some(1),
            OpCode::PUSHDATA2 => Some(2),
            OpCode::PUSHDATA4 => Some(4),
            _ => None,
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Self::from_byte(byte).ok_or("Invalid opcode")
    }
}

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> Self {
        op_code as u8
    }
}