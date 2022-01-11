#[allow(unused)]
/// No Operation
pub const NOP: u8 = 0xEA;

/// Load Accumulator
pub const LDA_IMMEDIATE: u8 = 0xA9;
/// Load Accumulator
pub const LDA_ZERO_PAGE: u8 = 0xA5;
/// Load Accumulator
pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
/// Load Accumulator
pub const LDA_ABSOLUTE: u8 = 0xAD;
/// Load Accumulator
pub const LDA_ABSOLUTE_X: u8 = 0xBD;
/// Load Accumulator
pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
/// Load Accumulator
pub const LDA_INDIRECT_X: u8 = 0xA1;
/// Load Accumulator
pub const LDA_INDIRECT_Y: u8 = 0xB1;

/// Load X Register
pub const LDX_IMMEDIATE: u8 = 0xA2;
/// Load X Register
pub const LDX_ZERO_PAGE: u8 = 0xA6;
/// Load X Register
pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
/// Load X Register
pub const LDX_ABSOLUTE: u8 = 0xAE;
/// Load X Register
pub const LDX_ABSOLUTE_Y: u8 = 0xBE;

/// Load Y Register
pub const LDY_IMMEDIATE: u8 = 0xA0;
/// Load Y Register
pub const LDY_ZERO_PAGE: u8 = 0xA4;
/// Load Y Register
pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
/// Load Y Register
pub const LDY_ABSOLUTE: u8 = 0xAC;
/// Load Y Register
pub const LDY_ABSOLUTE_X: u8 = 0xBC;

/// Store Accumulator
pub const STA_ZERO_PAGE: u8 = 0x85;
/// Store Accumulator
pub const STA_ZERO_PAGE_X: u8 = 0x95;
/// Store Accumulator
pub const STA_ABSOLUTE: u8 = 0x8D;
/// Store Accumulator
pub const STA_ABSOLUTE_X: u8 = 0x9D;
/// Store Accumulator
pub const STA_ABSOLUTE_Y: u8 = 0x99;
/// Store Accumulator
pub const STA_INDIRECT_X: u8 = 0x81;
/// Store Accumulator
pub const STA_INDIRECT_Y: u8 = 0x91;

/// Store X Register
pub const STX_ZERO_PAGE: u8 = 0x86;
/// Store X Register
pub const STX_ZERO_PAGE_Y: u8 = 0x96;
/// Store X Register
pub const STX_ABSOLUTE: u8 = 0x8E;

/// Store Y Register
pub const STY_ZERO_PAGE: u8 = 0x84;
/// Store Y Register
pub const STY_ZERO_PAGE_X: u8 = 0x94;
/// Store Y Register
pub const STY_ABSOLUTE: u8 = 0x8C;

/// Transfer accumulator to X
pub const TAX_IMPLIED: u8 = 0xAA;
/// Transfer accumulator to Y
pub const TAY_IMPLIED: u8 = 0xA8;
/// Transfer X to accumulator
pub const TXA_IMPLIED: u8 = 0x8A;
/// Transfer Y to accumulator
pub const TYA_IMPLIED: u8 = 0x98;

/// Transfer stack pointer to X
pub const TSX_IMPLIED: u8 = 0xBA;
/// Transfer X to stack pointer
pub const TXS_IMPLIED: u8 = 0x9A;
/// Push accumulator on stack
pub const PHA_IMPLIED: u8 = 0x48;
/// Push processor status on stack
pub const PHP_IMPLIED: u8 = 0x08;
/// Pull accumulator from stack
pub const PLA_IMPLIED: u8 = 0x68;
/// Pull processor status from stack
pub const PLP_IMPLIED: u8 = 0x28;

/// Logical AND
pub const AND_IMMEDIATE: u8 = 0x29;
/// Logical AND
pub const AND_ZERO_PAGE: u8 = 0x25;
/// Logical AND
pub const AND_ZERO_PAGE_X: u8 = 0x35;
/// Logical AND
pub const AND_ABSOLUTE: u8 = 0x2D;
/// Logical AND
pub const AND_ABSOLUTE_X: u8 = 0x3D;
/// Logical AND
pub const AND_ABSOLUTE_Y: u8 = 0x39;
/// Logical AND
pub const AND_INDIRECT_X: u8 = 0x21;
/// Logical AND
pub const AND_INDIRECT_Y: u8 = 0x31;

/// Exclusive OR
pub const EOR_IMMEDIATE: u8 = 0x49;
/// Exclusive OR
pub const EOR_ZERO_PAGE: u8 = 0x45;
/// Exclusive OR
pub const EOR_ZERO_PAGE_X: u8 = 0x55;
/// Exclusive OR
pub const EOR_ABSOLUTE: u8 = 0x4D;
/// Exclusive OR
pub const EOR_ABSOLUTE_X: u8 = 0x5D;
/// Exclusive OR
pub const EOR_ABSOLUTE_Y: u8 = 0x59;
/// Exclusive OR
pub const EOR_INDIRECT_X: u8 = 0x41;
/// Exclusive OR
pub const EOR_INDIRECT_Y: u8 = 0x51;

/// Logical Inclusive OR
pub const ORA_IMMEDIATE: u8 = 0x09;
/// Logical Inclusive OR
pub const ORA_ZERO_PAGE: u8 = 0x05;
/// Logical Inclusive OR
pub const ORA_ZERO_PAGE_X: u8 = 0x15;
/// Logical Inclusive OR
pub const ORA_ABSOLUTE: u8 = 0x0D;
/// Logical Inclusive OR
pub const ORA_ABSOLUTE_X: u8 = 0x1D;
/// Logical Inclusive OR
pub const ORA_ABSOLUTE_Y: u8 = 0x19;
/// Logical Inclusive OR
pub const ORA_INDIRECT_X: u8 = 0x01;
/// Logical Inclusive OR
pub const ORA_INDIRECT_Y: u8 = 0x11;

/// Bit Test
pub const BIT_ZERO_PAGE: u8 = 0x24;
/// Bit Test
pub const BIT_ABSOLUTE: u8 = 0x2C;

/// Add with Carry
pub const ADC_IMMEDIATE: u8 = 0x69;
/// Add with Carry
pub const ADC_ZERO_PAGE: u8 = 0x65;
/// Add with Carry
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
/// Add with Carry
pub const ADC_ABSOLUTE: u8 = 0x6D;
/// Add with Carry
pub const ADC_ABSOLUTE_X: u8 = 0x7D;
/// Add with Carry
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
/// Add with Carry
pub const ADC_INDIRECT_X: u8 = 0x61;
/// Add with Carry
pub const ADC_INDIRECT_Y: u8 = 0x71;

/// Subtract with Carry
pub const SBC_IMMEDIATE: u8 = 0xE9;
/// Subtract with Carry
pub const SBC_ZERO_PAGE: u8 = 0xE5;
/// Subtract with Carry
pub const SBC_ZERO_PAGE_X: u8 = 0xF5;
/// Subtract with Carry
pub const SBC_ABSOLUTE: u8 = 0xED;
/// Subtract with Carry
pub const SBC_ABSOLUTE_X: u8 = 0xFD;
/// Subtract with Carry
pub const SBC_ABSOLUTE_Y: u8 = 0xF9;
/// Subtract with Carry
pub const SBC_INDIRECT_X: u8 = 0xE1;
/// Subtract with Carry
pub const SBC_INDIRECT_Y: u8 = 0xF1;

/// Compare accumulator
pub const CMP_IMMEDIATE: u8 = 0xC9;
/// Compare accumulator
pub const CMP_ZERO_PAGE: u8 = 0xC5;
/// Compare accumulator
pub const CMP_ZERO_PAGE_X: u8 = 0xD5;
/// Compare accumulator
pub const CMP_ABSOLUTE: u8 = 0xCD;
/// Compare accumulator
pub const CMP_ABSOLUTE_X: u8 = 0xDD;
/// Compare accumulator
pub const CMP_ABSOLUTE_Y: u8 = 0xD9;
/// Compare accumulator
pub const CMP_INDIRECT_X: u8 = 0xC1;
/// Compare accumulator
pub const CMP_INDIRECT_Y: u8 = 0xD1;

/// Compare X register
pub const CPX_IMMEDIATE: u8 = 0xE0;
/// Compare X register
pub const CPX_ZERO_PAGE: u8 = 0xE4;
/// Compare X register
pub const CPX_ABSOLUTE: u8 = 0xEC;

/// Compare Y register
pub const CPY_IMMEDIATE: u8 = 0xC0;
/// Compare Y register
pub const CPY_ZERO_PAGE: u8 = 0xC4;
/// Compare Y register
pub const CPY_ABSOLUTE: u8 = 0xCC;

/// Increment a memory location
pub const INC_ZERO_PAGE: u8 = 0xE6;
/// Increment a memory location
pub const INC_ZERO_PAGE_X: u8 = 0xF6;
/// Increment a memory location
pub const INC_ABSOLUTE: u8 = 0xEE;
/// Increment a memory location
pub const INC_ABSOLUTE_X: u8 = 0xFE;

/// Increment the X register
pub const INX_IMPLIED: u8 = 0xE8;
/// Increment the Y register
pub const INY_IMPLIED: u8 = 0xC8;

/// Decrement a memory location
pub const DEC_ZERO_PAGE: u8 = 0xC6;
/// Decrement a memory location
pub const DEC_ZERO_PAGE_X: u8 = 0xD6;
/// Decrement a memory location
pub const DEC_ABSOLUTE: u8 = 0xCE;
/// Decrement a memory location
pub const DEC_ABSOLUTE_X: u8 = 0xDE;

/// Decrement the X register
pub const DEX_IMPLIED: u8 = 0xCA;
/// Decrement the Y register
pub const DEY_IMPLIED: u8 = 0x88;

/// Arithmetic Shift Left
pub const ASL_ACCUMULATOR: u8 = 0x0A;
/// Arithmetic Shift Left
pub const ASL_ZERO_PAGE: u8 = 0x06;
/// Arithmetic Shift Left
pub const ASL_ZERO_PAGE_X: u8 = 0x16;
/// Arithmetic Shift Left
pub const ASL_ABSOLUTE: u8 = 0x0E;
/// Arithmetic Shift Left
pub const ASL_ABSOLUTE_X: u8 = 0x1E;

/// Logical Shift Right
pub const LSR_ACCUMULATOR: u8 = 0x4A;
/// Logical Shift Right
pub const LSR_ZERO_PAGE: u8 = 0x46;
/// Logical Shift Right
pub const LSR_ZERO_PAGE_X: u8 = 0x56;
/// Logical Shift Right
pub const LSR_ABSOLUTE: u8 = 0x4E;
/// Logical Shift Right
pub const LSR_ABSOLUTE_X: u8 = 0x5E;

/// Rotate Left
pub const ROL_ACCUMULATOR: u8 = 0x2A;
/// Rotate Left
pub const ROL_ZERO_PAGE: u8 = 0x26;
/// Rotate Left
pub const ROL_ZERO_PAGE_X: u8 = 0x36;
/// Rotate Left
pub const ROL_ABSOLUTE: u8 = 0x2E;
/// Rotate Left
pub const ROL_ABSOLUTE_X: u8 = 0x3E;

/// Rotate Right
pub const ROR_ACCUMULATOR: u8 = 0x6A;
/// Rotate Right
pub const ROR_ZERO_PAGE: u8 = 0x66;
/// Rotate Right
pub const ROR_ZERO_PAGE_X: u8 = 0x76;
/// Rotate Right
pub const ROR_ABSOLUTE: u8 = 0x6E;
/// Rotate Right
pub const ROR_ABSOLUTE_X: u8 = 0x7E;

/// Jump to another location
pub const JMP_ABSOLUTE: u8 = 0x4C;
/// Jump to another location
pub const JMP_INDIRECT: u8 = 0x6C;

/// Jump to a subroutine
pub const JSR_ABSOLUTE: u8 = 0x20;
/// Return from subroutine
pub const RTS_IMPLIED: u8 = 0x60;

/// Branch if carry flag clear
pub const BCC_RELATIVE: u8 = 0x90;
/// Branch if carry flag set
pub const BCS_RELATIVE: u8 = 0xB0;
/// Branch if zero flag set
pub const BEQ_RELATIVE: u8 = 0xF0;
/// Branch if negative flag set
pub const BMI_RELATIVE: u8 = 0x30;
/// Branch if zero flag clear
pub const BNE_RELATIVE: u8 = 0xD0;
/// Branch if negative flag clear
pub const BPL_RELATIVE: u8 = 0x10;
/// Branch if overflow flag clear
pub const BVC_RELATIVE: u8 = 0x50;
/// Branch if overflow flag set
pub const BVS_RELATIVE: u8 = 0x70;