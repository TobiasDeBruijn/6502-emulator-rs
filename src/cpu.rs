use std::num::Wrapping;
use bitflags::bitflags;
use crate::memory::{MAX_MEMORY, Memory};
use crate::ops::Op;

pub struct Cpu {
    program_counter: u16,
    #[allow(unused)]
    stack_pointer: u16,

    register_accumulator: u8,
    register_x: u8,
    register_y: u8,

    flags: CpuStatusFlags
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            program_counter: 0xFFFC,
            stack_pointer: 0x00FF,
            register_accumulator: 0,
            register_x: 0,
            register_y: 0,
            flags: CpuStatusFlags::default(),
        }
    }
}

impl Cpu {
    /// Reset the CPU
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Execute instructions
    pub fn execute(&mut self, memory: &mut Memory, mut cycles: u32) -> u32 {
        while cycles > 0 {
            let instruction_byte = self.fetch_byte(memory, &mut cycles);
            let op = Op::by_opcode(instruction_byte).expect("Invalid opcode");
            match op {
                Op::None => continue,
                Op::LoadAccumulatorImmediate => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.set_register(Register::A, value);
                },
                Op::LoadAccumulatorZeroPage => {
                    let zero_page_address = self.fetch_byte(memory, &mut cycles);
                    self.load_register(memory, Register::A, zero_page_address as u16, &mut cycles);
                },
                Op::LoadAccumulatorZeroPageX => {
                    let zero_page_address = self.fetch_byte(memory, &mut cycles) as u16;
                    let address = zero_page_address + self.register_x as u16;
                    cycles -= 1;
                    self.load_register(memory, Register::A, address, &mut cycles);
                },
                Op::LoadAccumulatorAbsolute => {
                    let address = self.fetch_word(memory, &mut cycles);
                    self.load_register(memory, Register::A, address, &mut cycles);
                },
                Op::LoadAccumulatorAbsoluteX => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_x = (Wrapping(address) + Wrapping(self.register_x as u16)).0;

                    if (address ^ address_x) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::A, address_x, &mut cycles);
                },
                Op::LoadAccumulatorAbsoluteY => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_y = address + self.register_y as u16;

                    if (address ^ address_y) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::A, address_y, &mut cycles);
                },
                Op::LoadAccumulatorIndirectX => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let address = zp_address + self.register_x;
                    cycles -= 1;
                    let effective_address = Self::read_word(memory, address as u16, &mut cycles);
                    self.load_register(memory, Register::A, effective_address, &mut cycles);

                },
                Op::LoadAccumulatorIndirectY => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let effective_address = Self::read_word(memory, zp_address as u16, &mut cycles);
                    let effective_address_y = effective_address + self.register_y as u16;

                    if (effective_address ^ effective_address_y) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::A, effective_address_y, &mut cycles);
                },
                Op::LoadXImmediate => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.set_register(Register::X, value);
                },
                Op::LoadXZeroPage => {
                    let address = self.fetch_byte(memory, &mut cycles);
                    self.load_register(memory, Register::X, address as u16, &mut cycles);
                },
                Op::LoadXZeroPageY => {
                    let address = self.fetch_byte(memory, &mut cycles) as u16;
                    let address_y = address + self.register_y as u16;
                    cycles -= 1;
                    self.load_register(memory, Register::X, address_y, &mut cycles);
                },
                Op::LoadXAbsolute => {
                    let address = self.fetch_word(memory, &mut cycles);
                    self.load_register(memory, Register::X, address, &mut cycles);
                },
                Op::LoadXAbsoluteY => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_y = address + self.register_y as u16;

                    if (address ^ address_y) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::X, address_y, &mut cycles);
                },
                Op::LoadYImmediate => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.set_register(Register::Y, value);
                },
                Op::LoadYZeroPage => {
                    let address = self.fetch_byte(memory, &mut cycles);
                    self.load_register(memory, Register::Y, address as u16, &mut cycles);
                },
                Op::LoadYZeroPageX => {
                    let address = self.fetch_byte(memory, &mut cycles) as u16;
                    let address_x = address + self.register_x as u16;
                    self.load_register(memory, Register::Y, address_x, &mut cycles);
                },
                Op::LoadYAbsolute => {
                    let address = self.fetch_word(memory, &mut cycles);
                    self.load_register(memory, Register::Y, address, &mut cycles);
                },
                Op::LoadYAbsoluteX => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_x = address + self.register_x as u16;

                    if (address ^ address_x) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::Y, address_x, &mut cycles);
                }
            }
        }

        cycles
    }

    /// Fetch a word from Memory. This will increment the program counter twice
    fn fetch_word(&mut self, memory: &Memory, cycles: &mut u32) -> u16 {
        let high = self.fetch_byte(memory, cycles) as u16;
        let low = self.fetch_byte(memory, cycles) as u16;
        high << 8 | low
    }

    /// Load a value from an address into a register
    fn load_register(&mut self, memory: &Memory, register: Register, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        self.set_register(register, value);
    }

    /// Set the value of a register and set the zero and negative flags
    fn set_register(&mut self, register: Register, byte: u8) {
        match register {
            Register::A => self.register_accumulator = byte,
            Register::X => self.register_x = byte,
            Register::Y => self.register_y = byte
        };

        self.set_zero_flag(&register);
        self.set_negative_flag(&register);
    }

    /// Set the zero flag if appropriate
    fn set_zero_flag(&mut self, register: &Register) {
        let v = match register {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y
        };

        if v == 0 {
            self.flags.set(CpuStatusFlags::ZERO, true);
        }
    }

    /// Set the negative flag if approproate
    fn set_negative_flag(&mut self, register: &Register) {
        let v = match register {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y
        };

        // Check if the left-most bit is set, i.e. the sign bit
        if v & 0b1000_0000 != 0 {
            self.flags.set(CpuStatusFlags::NEGATIVE, true);
        }
    }

    /// Fetch a byte from memory at the program_counter and increment it
    fn fetch_byte(&mut self, memory: &Memory, cycles: &mut u32) -> u8 {
        let byte = memory.fetch(self.program_counter);
        self.program_counter += 1;
        *cycles -= 1;
        byte
    }

    /// Read a Word from memory. This reads `address` and `address + 1`
    fn read_word(memory: &Memory, address: u16, cycles: &mut u32) -> u16 {
        let low = Self::read_byte(memory, address, cycles) as u16;
        let high = Self::read_byte(memory, address + 1, cycles) as u16;
        high << 8 | low
    }

    /// Read a byte from memory
    fn read_byte(memory: &Memory, address: u16, cycles: &mut u32) -> u8 {
        if address as usize > MAX_MEMORY {
            panic!("Read byte failed: Memory address {} is higher than MAX_MEMORY", address);
        }

        let byte = memory.fetch(address);
        *cycles -= 1;
        byte
    }
}

bitflags! {
    /// Flags describing the CPU's current status
    pub struct CpuStatusFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const IRQ_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK_COMMAND = 0b0001_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

impl Default for CpuStatusFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/// Represents a register
enum Register {
    /// The accumulator register
    A,
    /// The X register
    X,
    /// The Y register
    Y
}

#[cfg(test)]
mod test {
    use crate::{Cpu, Memory};
    use crate::cpu::CpuStatusFlags;
    use crate::ops::instructions::*;

    #[test]
    fn lda_immediate() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, 0xA9);
        memory.write(0xFFFD, 0x42);

        cpu.execute(&mut memory, 2);
        assert_eq!(cpu.register_accumulator, 0x42);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0xFFFD, 0x42 + 0b1000_0000); // Make the number negative by enabling the left most bit

        cpu.execute(&mut memory, 2);
        assert_eq!(cpu.register_accumulator, 0x42 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0xFFFD, 0x0);

        cpu.execute(&mut memory, 2);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_zero_page() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_ZERO_PAGE);
        memory.write(0xFFFD, 0x42);
        memory.write(0x0042, 0x10);

        cpu.execute(&mut memory, 3);
        assert_eq!(cpu.register_accumulator, 0x10);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0x42, 0x10 + 0b1000_0000);

        cpu.execute(&mut memory, 3);
        assert_eq!(cpu.register_accumulator, 0x10 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0x42, 0x0);

        cpu.execute(&mut memory, 3);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_zero_page_x() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x10);
        memory.write(0x20, 0x42);
        cpu.register_x = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x42);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x0020, 0x42 + 0b1000_0000);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x42 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x0020, 0x0);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_absolute() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_ABSOLUTE);
        memory.write(0xFFFD, 0x44);
        memory.write(0xFFFE, 0x80); // Loads from 0x4480
        memory.write(0x4480, 0x64);

        cpu.execute(&mut memory,4);
        assert_eq!(cpu.register_accumulator, 0x64);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x4480, 0x64 + 0b1000_0000);
        cpu.reset();

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x64 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x4480, 0x0);
        cpu.reset();

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_absolute_x() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_ABSOLUTE_X);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x8040

        memory.write(0x8050, 0x32);
        cpu.register_x = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x8050, 0x32 + 0b1000_0000);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x8050, 0x0);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_absolute_y() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x8040

        memory.write(0x8050, 0x32);
        cpu.register_y = 0x10;

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0x10;
        memory.write(0x8050, 0x32 + 0b1000_0000);

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0x10;
        memory.write(0x8050, 0x0);

        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_indirect_x() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_INDIRECT_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0x30, 0x80);

        memory.write(0x80, 0x32);
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0x10;
        memory.write(0x80, 0x32 + 0b1000_0000);

        cpu.execute(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0x10;
        memory.write(0x80, 0x0);

        cpu.execute(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_indirect_y() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDA_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0x40, 0x50);

        memory.write(0x60, 0x32);
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute(&mut memory, 5);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x60, 0x32 + 0b1000_0000);
        cpu.reset();
        cpu.register_y = 0x10;

        cpu.execute(&mut memory, 5);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x60, 0x0);
        cpu.reset();
        cpu.register_y = 0x10;

        cpu.execute(&mut memory, 5);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, LDA_INDIRECT_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0x10, 0x10);
        memory.write(0x10F, 0x32);
        cpu.register_y = 0xFF;

        let cycles_left = cpu.execute(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn ldx_immediate() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDX_IMMEDIATE);
        memory.write(0xFFFD, 0x32);

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_zero_page() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDX_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x32);

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_zero_page_y() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDX_ZERO_PAGE_Y);
        memory.write(0xFFFD, 0x20);
        memory.write(0x30, 0x32);
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_absolute() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDX_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x8040
        memory.write(0x8040, 0x32);

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_absolute_y() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDX_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); //0x8040;
        cpu.register_y = 0x10;
        memory.write(0x8050, 0x32);

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, LDX_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x10); // 0x1010
        cpu.register_y = 0xFF;
        memory.write(0x110F, 0x32);

        let cycles_left = cpu.execute(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldy_immediate() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDY_IMMEDIATE);
        memory.write(0xFFFD, 0x32);

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_zero_page() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDY_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x32);

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_zero_page_x() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDY_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0x30, 0x32);
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_absolute() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDY_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x8040
        memory.write(0x8040, 0x32);

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_absolute_x() {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, LDY_ABSOLUTE_X);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); //0x8040;
        cpu.register_x = 0x10;
        memory.write(0x8050, 0x32);

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, LDY_ABSOLUTE_X);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x10); // 0x1010
        cpu.register_x = 0xFF;
        memory.write(0x110F, 0x32);

        let cycles_left = cpu.execute(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }
}