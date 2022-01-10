#[allow(unused)]
pub const NOP: u8 = 0xEA;

pub const LDA_IMMEDIATE: u8 = 0xA9;
pub const LDA_ZERO_PAGE: u8 = 0xA5;
pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
pub const LDA_ABSOLUTE: u8 = 0xAD;
pub const LDA_ABSOLUTE_X: u8 = 0xBD;
pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
pub const LDA_INDIRECT_X: u8 = 0xA1;
pub const LDA_INDIRECT_Y: u8 = 0xB1;

pub const LDX_IMMEDIATE: u8 = 0xA2;
pub const LDX_ZERO_PAGE: u8 = 0xA6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
pub const LDX_ABSOLUTE: u8 = 0xAE;
pub const LDX_ABSOLUTE_Y: u8 = 0xBE;

pub const LDY_IMMEDIATE: u8 = 0xA0;
pub const LDY_ZERO_PAGE: u8 = 0xA4;
pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
pub const LDY_ABSOLUTE: u8 = 0xAC;
pub const LDY_ABSOLUTE_X: u8 = 0xBC;

pub const STA_ZERO_PAGE: u8 = 0x85;
pub const STA_ZERO_PAGE_X: u8 = 0x95;
pub const STA_ABSOLUTE: u8 = 0x8D;
pub const STA_ABSOLUTE_X: u8 = 0x9D;
pub const STA_ABSOLUTE_Y: u8 = 0x99;
pub const STA_INDIRECT_X: u8 = 0x81;
pub const STA_INDIRECT_Y: u8 = 0x91;

pub const STX_ZERO_PAGE: u8 = 0x86;
pub const STX_ZERO_PAGE_Y: u8 = 0x96;
pub const STX_ABSOLUTE: u8 = 0x8E;

pub const STY_ZERO_PAGE: u8 = 0x84;
pub const STY_ZERO_PAGE_X: u8 = 0x94;
pub const STY_ABSOLUTE: u8 = 0x8C;

pub const TAX_IMPLIED: u8 = 0xAA;
pub const TAY_IMPLIED: u8 = 0xA8;
pub const TXA_IMPLIED: u8 = 0x8A;
pub const TYA_IMPLIED: u8 = 0x98;

pub const TSX_IMPLIED: u8 = 0xBA;
pub const TXS_IMPLIED: u8 = 0x9A;
pub const PHA_IMPLIED: u8 = 0x48;
pub const PHP_IMPLIED: u8 = 0x08;
pub const PLA_IMPLIED: u8 = 0x68;
pub const PLP_IMPLIED: u8 = 0x28;

pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZERO_PAGE: u8 = 0x25;
pub const AND_ZERO_PAGE_X: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2D;
pub const AND_ABSOLUTE_X: u8 = 0x3D;
pub const AND_ABSOLUTE_Y: u8 = 0x39;
pub const AND_INDIRECT_X: u8 = 0x21;
pub const AND_INDIRECT_Y: u8 = 0x31;

pub const EOR_IMMEDIATE: u8 = 0x49;
pub const EOR_ZERO_PAGE: u8 = 0x45;
pub const EOR_ZERO_PAGE_X: u8 = 0x55;
pub const EOR_ABSOLUTE: u8 = 0x4D;
pub const EOR_ABSOLUTE_X: u8 = 0x5D;
pub const EOR_ABSOLUTE_Y: u8 = 0x59;
pub const EOR_INDIRECT_X: u8 = 0x41;
pub const EOR_INDIRECT_Y: u8 = 0x51;

pub const ORA_IMMEDIATE: u8 = 0x09;
pub const ORA_ZERO_PAGE: u8 = 0x05;
pub const ORA_ZERO_PAGE_X: u8 = 0x15;
pub const ORA_ABSOLUTE: u8 = 0x0D;
pub const ORA_ABSOLUTE_X: u8 = 0x1D;
pub const ORA_ABSOLUTE_Y: u8 = 0x19;
pub const ORA_INDIRECT_X: u8 = 0x01;
pub const ORA_INDIRECT_Y: u8 = 0x11;

pub const BIT_ZERO_PAGE: u8 = 0x24;
pub const BIT_ABSOLUTE: u8 = 0x2C;

pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTE_X: u8 = 0x7D;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

pub const SBC_IMMEDIATE: u8 = 0xE9;
pub const SBC_ZERO_PAGE: u8 = 0xE5;
pub const SBC_ZERO_PAGE_X: u8 = 0xF5;
pub const SBC_ABSOLUTE: u8 = 0xED;
pub const SBC_ABSOLUTE_X: u8 = 0xFD;
pub const SBC_ABSOLUTE_Y: u8 = 0xF9;
pub const SBC_INDIRECT_X: u8 = 0xE1;
pub const SBC_INDIRECT_Y: u8 = 0xF1;

pub const CMP_IMMEDIATE: u8 = 0xC9;
pub const CMP_ZERO_PAGE: u8 = 0xC5;
pub const CMP_ZERO_PAGE_X: u8 = 0xD5;
pub const CMP_ABSOLUTE: u8 = 0xCD;
pub const CMP_ABSOLUTE_X: u8 = 0xDD;
pub const CMP_ABSOLUTE_Y: u8 = 0xD9;
pub const CMP_INDIRECT_X: u8 = 0xC1;
pub const CMP_INDIRECT_Y: u8 = 0xD1;

pub const CPX_IMMEDIATE: u8 = 0xE0;
pub const CPX_ZERO_PAGE: u8 = 0xE4;
pub const CPX_ABSOLUTE: u8 = 0xEC;

pub const CPY_IMMEDIATE: u8 = 0xC0;
pub const CPY_ZERO_PAGE: u8 = 0xC4;
pub const CPY_ABSOLUTE: u8 = 0xCC;

pub const INC_ZERO_PAGE: u8 = 0xE6;
pub const INC_ZERO_PAGE_X: u8 = 0xF6;
pub const INC_ABSOLUTE: u8 = 0xEE;
pub const INC_ABSOLUTE_X: u8 = 0xFE;

pub const INX_IMPLIED: u8 = 0xE8;
pub const INY_IMPLIED: u8 = 0xC8;

pub const DEC_ZERO_PAGE: u8 = 0xC6;
pub const DEC_ZERO_PAGE_X: u8 = 0xD6;
pub const DEC_ABSOLUTE: u8 = 0xCE;
pub const DEC_ABSOLUTE_X: u8 = 0xDE;

pub const DEX_IMPLIED: u8 = 0xCA;
pub const DEY_IMPLIED: u8 = 0x88;

pub const ASL_ACCUMULATOR: u8 = 0x0A;
pub const ASL_ZERO_PAGE: u8 = 0x06;
pub const ASL_ZERO_PAGE_X: u8 = 0x16;
pub const ASL_ABSOLUTE: u8 = 0x0E;
pub const ASL_ABSOLUTE_X: u8 = 0x1E;

pub const LSR_ACCUMULATOR: u8 = 0x4A;
pub const LSR_ZERO_PAGE: u8 = 0x46;
pub const LSR_ZERO_PAGE_X: u8 = 0x56;
pub const LSR_ABSOLUTE: u8 = 0x4E;
pub const LSR_ABSOLUTE_X: u8 = 0x5E;

pub const ROL_ACCUMULATOR: u8 = 0x2A;
pub const ROL_ZERO_PAGE: u8 = 0x26;
pub const ROL_ZERO_PAGE_X: u8 = 0x36;
pub const ROL_ABSOLUTE: u8 = 0x2E;
pub const ROL_ABSOLUTE_X: u8 = 0x3E;

pub const ROR_ACCUMULATOR: u8 = 0x6A;
pub const ROR_ZERO_PAGE: u8 = 0x66;
pub const ROR_ZERO_PAGE_X: u8 = 0x76;
pub const ROR_ABSOLUTE: u8 = 0x6E;
pub const ROR_ABSOLUTE_X: u8 = 0x7E;

pub const JMP_ABSOLUTE: u8 = 0x4C;
pub const JMP_INDIRECT: u8 = 0x6C;
