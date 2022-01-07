pub mod instructions {
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
}

use instructions::*;

const OPCODES: &[Instruction] = &[
    Instruction::new(0x0, Op::None),

    Instruction::new(LDA_IMMEDIATE, Op::LoadAccumulatorImmediate),
    Instruction::new(LDA_ZERO_PAGE, Op::LoadAccumulatorZeroPage),
    Instruction::new(LDA_ZERO_PAGE_X, Op::LoadAccumulatorZeroPageX),
    Instruction::new(LDA_ABSOLUTE, Op::LoadAccumulatorAbsolute),
    Instruction::new(LDA_ABSOLUTE_X, Op::LoadAccumulatorAbsoluteX),
    Instruction::new(LDA_ABSOLUTE_Y, Op::LoadAccumulatorAbsoluteY),
    Instruction::new(LDA_INDIRECT_X, Op::LoadAccumulatorIndirectX),
    Instruction::new(LDA_INDIRECT_Y, Op::LoadAccumulatorIndirectY),

    Instruction::new(LDX_IMMEDIATE, Op::LoadXImmediate),
    Instruction::new(LDX_ZERO_PAGE, Op::LoadXZeroPage),
    Instruction::new(LDX_ZERO_PAGE_Y, Op::LoadXZeroPageY),
    Instruction::new(LDX_ABSOLUTE, Op::LoadXAbsolute),
    Instruction::new(LDX_ABSOLUTE_Y, Op::LoadXAbsoluteY),

    Instruction::new(LDY_IMMEDIATE, Op::LoadYImmediate),
    Instruction::new(LDY_ZERO_PAGE, Op::LoadYZeroPage),
    Instruction::new(LDY_ZERO_PAGE_X, Op::LoadYZeroPageX),
    Instruction::new(LDY_ABSOLUTE, Op::LoadYAbsolute),
    Instruction::new(LDY_ABSOLUTE_X, Op::LoadYAbsoluteX),
];

#[derive(Clone)]
pub enum Op {
    None,
    LoadAccumulatorImmediate,
    LoadAccumulatorZeroPage,
    LoadAccumulatorZeroPageX,
    LoadAccumulatorAbsolute,
    LoadAccumulatorAbsoluteX,
    LoadAccumulatorAbsoluteY,
    LoadAccumulatorIndirectX,
    LoadAccumulatorIndirectY,

    LoadXImmediate,
    LoadXZeroPage,
    LoadXZeroPageY,
    LoadXAbsolute,
    LoadXAbsoluteY,

    LoadYImmediate,
    LoadYZeroPage,
    LoadYZeroPageX,
    LoadYAbsolute,
    LoadYAbsoluteX,
}

impl Op {
    pub fn by_opcode(opcode: u8) -> Option<Self> {
        let instr = Instruction::by_opcode(opcode)?;
        Some(instr.op)
    }
}

#[derive(Clone)]
pub struct Instruction {
    opcode: u8,
    op: Op
}

impl Instruction {
    pub const fn new(opcode: u8, op: Op) -> Self {
        Self {
            opcode,
            op,
        }
    }

    pub fn by_opcode(opcode: u8) -> Option<Self> {
        OPCODES.iter().find(|x| x.opcode == opcode).cloned()
    }
}