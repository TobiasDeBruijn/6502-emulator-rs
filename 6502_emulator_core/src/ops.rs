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