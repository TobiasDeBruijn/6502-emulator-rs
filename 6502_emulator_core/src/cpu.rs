use core::num::Wrapping;
use bitflags::bitflags;
use crate::memory::{MAX_MEMORY, Memory};
use crate::ops::*;

#[cfg(test)]
use log::debug;

pub struct Cpu {
    program_counter: u16,
    #[allow(unused)]
    stack_pointer: u8,

    register_accumulator: u8,
    register_x: u8,
    register_y: u8,

    flags: CpuStatusFlags
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            program_counter: 0xFFFC,
            stack_pointer: 0xFF,
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

            #[cfg(test)]
            debug!("Execting instruction: {:#04X}", instruction_byte);

            match instruction_byte {
                // Load/Store operations
                LDA_IMMEDIATE => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.set_register(Register::A, value);
                },
                LDA_ZERO_PAGE => {
                    let zero_page_address = self.fetch_byte(memory, &mut cycles);
                    self.load_register(memory, Register::A, zero_page_address as u16, &mut cycles);
                },
                LDA_ZERO_PAGE_X => {
                    let zero_page_address = self.fetch_byte(memory, &mut cycles) as u16;
                    let address = zero_page_address + self.register_x as u16;
                    cycles -= 1;
                    self.load_register(memory, Register::A, address, &mut cycles);
                },
                LDA_ABSOLUTE => {
                    let address = self.fetch_word(memory, &mut cycles);
                    self.load_register(memory, Register::A, address, &mut cycles);
                },
                LDA_ABSOLUTE_X => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_x = (Wrapping(address) + Wrapping(self.register_x as u16)).0;

                    if (address ^ address_x) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::A, address_x, &mut cycles);
                },
                LDA_ABSOLUTE_Y => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_y = address + self.register_y as u16;

                    if (address ^ address_y) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::A, address_y, &mut cycles);
                },
                LDA_INDIRECT_X => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let address = zp_address + self.register_x;
                    cycles -= 1;
                    let effective_address = Self::read_word(memory, address as u16, &mut cycles);
                    self.load_register(memory, Register::A, effective_address, &mut cycles);

                },
                LDA_INDIRECT_Y => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let effective_address = Self::read_word(memory, zp_address as u16, &mut cycles);
                    let effective_address_y = effective_address + self.register_y as u16;

                    if (effective_address ^ effective_address_y) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::A, effective_address_y, &mut cycles);
                },
                LDX_IMMEDIATE => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.set_register(Register::X, value);
                },
                LDX_ZERO_PAGE => {
                    let address = self.fetch_byte(memory, &mut cycles);
                    self.load_register(memory, Register::X, address as u16, &mut cycles);
                },
                LDX_ZERO_PAGE_Y => {
                    let address = self.fetch_byte(memory, &mut cycles) as u16;
                    let address_y = address + self.register_y as u16;
                    cycles -= 1;
                    self.load_register(memory, Register::X, address_y, &mut cycles);
                },
                LDX_ABSOLUTE => {
                    let address = self.fetch_word(memory, &mut cycles);
                    self.load_register(memory, Register::X, address, &mut cycles);
                },
                LDX_ABSOLUTE_Y => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_y = address + self.register_y as u16;

                    if (address ^ address_y) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::X, address_y, &mut cycles);
                },
                LDY_IMMEDIATE => {
                    let value = self.fetch_byte(memory, &mut cycles);
                    self.set_register(Register::Y, value);
                },
                LDY_ZERO_PAGE => {
                    let address = self.fetch_byte(memory, &mut cycles);
                    self.load_register(memory, Register::Y, address as u16, &mut cycles);
                },
                LDY_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory, &mut cycles) as u16;
                    let address_x = address + self.register_x as u16;
                    self.load_register(memory, Register::Y, address_x, &mut cycles);
                },
                LDY_ABSOLUTE => {
                    let address = self.fetch_word(memory, &mut cycles);
                    self.load_register(memory, Register::Y, address, &mut cycles);
                },
                LDY_ABSOLUTE_X => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_x = address + self.register_x as u16;

                    if (address ^ address_x) >> 8 != 0 {
                        cycles -= 1;
                    }

                    self.load_register(memory, Register::Y, address_x, &mut cycles);
                },
                STA_ZERO_PAGE => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    Self::write_byte(memory, zp_address as u16, self.register_accumulator, &mut cycles);
                },
                STA_ZERO_PAGE_X => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let address_x = (Wrapping(zp_address) + Wrapping(self.register_x)).0;
                    cycles -= 1;
                    Self::write_byte(memory, address_x as u16, self.register_accumulator, &mut cycles);
                },
                STA_ABSOLUTE => {
                    let address = self.fetch_word(memory, &mut cycles);
                    Self::write_byte(memory, address, self.register_accumulator, &mut cycles);
                },
                STA_ABSOLUTE_X => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_x = address + self.register_x as u16;
                    cycles -= 1;
                    Self::write_byte(memory, address_x, self.register_accumulator, &mut cycles);
                },
                STA_ABSOLUTE_Y => {
                    let address = self.fetch_word(memory, &mut cycles);
                    let address_y = address + self.register_y as u16;
                    cycles -= 1;
                    Self::write_byte(memory, address_y, self.register_accumulator, &mut cycles);
                },
                STA_INDIRECT_X => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let zp_address_x = (Wrapping(zp_address as u16) + Wrapping(self.register_x as u16)).0;
                    cycles -= 1;
                    let effective_address = Self::read_word(memory, zp_address_x, &mut cycles);
                    Self::write_byte(memory, effective_address, self.register_accumulator, &mut cycles);
                },
                STA_INDIRECT_Y => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    let zp_address_y = zp_address as u16 + self.register_y as u16;
                    cycles -= 1;
                    let effective_address = Self::read_word(memory, zp_address_y, &mut cycles);
                    Self::write_byte(memory, effective_address, self.register_accumulator, &mut cycles);
                },
                STX_ZERO_PAGE => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    Self::write_byte(memory, zp_address as u16, self.register_x, &mut cycles);
                },
                STX_ZERO_PAGE_Y => {
                    let zp_address = self.fetch_byte(memory, &mut cycles) as u16;
                    let zp_address_y = zp_address + self.register_y as u16;
                    cycles -= 1;
                    Self::write_byte(memory, zp_address_y, self.register_x, &mut cycles);
                },
                STX_ABSOLUTE => {
                    let address = self.fetch_word(memory, &mut cycles);
                    Self::write_byte(memory, address, self.register_x, &mut cycles);
                },
                STY_ZERO_PAGE => {
                    let zp_address = self.fetch_byte(memory, &mut cycles);
                    Self::write_byte(memory, zp_address as u16, self.register_y, &mut cycles);
                },
                STY_ZERO_PAGE_X => {
                    let zp_address = self.fetch_byte(memory, &mut cycles) as u16;
                    let zp_address_x = (Wrapping(zp_address) + Wrapping(self.register_x as u16)).0;
                    cycles -= 1;
                    Self::write_byte(memory, zp_address_x, self.register_y, &mut cycles);
                },
                STY_ABSOLUTE => {
                    let address = self.fetch_word(memory, &mut cycles);
                    Self::write_byte(memory, address, self.register_y, &mut cycles);
                },

                // Register transfers
                TAX_IMPLIED => {
                    self.transfer_register(Register::A, Register::X, &mut cycles);
                },
                TAY_IMPLIED => {
                    self.transfer_register(Register::A, Register::Y, &mut cycles);
                },
                TXA_IMPLIED => {
                    self.transfer_register(Register::X, Register::A, &mut cycles);
                },
                TYA_IMPLIED => {
                    self.transfer_register(Register::Y, Register::A, &mut cycles);
                },

                // Stack operations
                TSX_IMPLIED => {
                    self.transfer_register(Register::S, Register::X, &mut cycles);
                },
                TXS_IMPLIED => {
                    self.transfer_register(Register::X, Register::S, &mut cycles);
                },
                PHA_IMPLIED => {
                    // The stack runs from 0x0100 - 0x01FF
                    // But the stack pointer stores only the least significant byte
                    Self::write_byte(memory, 0x0100 + (self.stack_pointer as u16), self.register_accumulator, &mut cycles);
                    self.stack_pointer = (Wrapping(self.stack_pointer) + Wrapping(1)).0;
                    cycles -= 1;
                },
                PHP_IMPLIED => {
                    // The stack runs from 0x0100 - 0x01FF
                    // But the stack pointer stores only the least significant byte
                    Self::write_byte(memory, 0x0100 + (self.stack_pointer as u16), self.flags.bits(), &mut cycles);
                    self.stack_pointer = (Wrapping(self.stack_pointer) + Wrapping(1)).0;
                    cycles -= 1;
                },
                PLA_IMPLIED => {
                    // The stack pointer points to the next free byte,
                    // Decrement the stack pointer *before* reading it
                    self.stack_pointer = (Wrapping(self.stack_pointer) - Wrapping(1)).0;
                    cycles -= 1;
                    // The stack runs from 0x0100 - 0x01FF,
                    // the stack pointer represents least significant byte of the address
                    self.load_register(memory, Register::A, 0x0100 + (self.stack_pointer as u16), &mut cycles);
                },
                PLP_IMPLIED => {
                    // The stack pointer points to the next free byte,
                    // Decrement the stack pointer *before* reading it
                    self.stack_pointer = (Wrapping(self.stack_pointer) - Wrapping(1)).0;
                    cycles -= 1;
                    // The stack runs from 0x0100 - 0x01FF,
                    // the stack pointer represents least significant byte of the address
                    let byte = Self::read_byte(memory, 0x0100 + (self.stack_pointer as u16), &mut cycles);
                    self.flags = CpuStatusFlags::from_bits_truncate(byte);
                    cycles -= 1
                }
                _ => continue
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

        #[cfg(test)]
        debug!("Loading register {:?} from {:#06X}: {:#04X}", register.clone(), address, value);

        self.set_register(register, value);
    }

    /// Transfer the contents from one register to another
    fn transfer_register(&mut self, source: Register, dest: Register, cycles: &mut u32) {
        #[cfg(test)]
        debug!("Transfering register {:?} to register {:?}", source, dest);

        let value = match source {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y,
            Register::S => self.stack_pointer
        };

        self.set_register(dest, value);
        *cycles -= 1;
    }

    /// Set the value of a register and set the zero and negative flags
    fn set_register(&mut self, register: Register, byte: u8) {
        #[cfg(test)]
        debug!("Setting register {:?} with {:#04X}", register, byte);

        match register {
            Register::A => self.register_accumulator = byte,
            Register::X => self.register_x = byte,
            Register::Y => self.register_y = byte,
            Register::S => self.stack_pointer = byte,
        };

        self.set_zero_flag(&register);
        self.set_negative_flag(&register);
    }

    /// Set the zero flag if appropriate
    fn set_zero_flag(&mut self, register: &Register) {
        let v = match register {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y,
            Register::S => self.stack_pointer,
        };

        if v == 0 {
            #[cfg(test)]
            debug!("Setting zero flag for register {:?}", register);

            self.flags.set(CpuStatusFlags::ZERO, true);
        }
    }

    /// Set the negative flag if approproate
    fn set_negative_flag(&mut self, register: &Register) {
        let v = match register {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y,
            Register::S => self.stack_pointer,
        };

        // Check if the left-most bit is set, i.e. the sign bit
        if v & 0b1000_0000 != 0 {
            #[cfg(test)]
            debug!("Setting negative flag for register {:?}", register);

            self.flags.set(CpuStatusFlags::NEGATIVE, true);
        }
    }

    /// Fetch a byte from memory at the program_counter and increment it
    fn fetch_byte(&mut self, memory: &Memory, cycles: &mut u32) -> u8 {
        let byte = memory.fetch(self.program_counter);
        self.program_counter += 1;
        *cycles -= 1;

        #[cfg(test)]
        debug!("Fetched byte from {:#04X}: {:#04X}", self.program_counter, byte);

        byte
    }

    /// Read a Word from memory. This reads `address` and `address + 1`
    fn read_word(memory: &Memory, address: u16, cycles: &mut u32) -> u16 {
        if (address + 1) as usize > MAX_MEMORY {
            panic!("Read word failed: Memory address {} is higher than MAX_MEMORY", address);
        }

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

        #[cfg(test)]
        debug!("Read byte from memory at {:#06X}: {:#04X}", address, byte);

        byte
    }

    /// Write a byte to memory
    fn write_byte(memory: &mut Memory, address: u16, byte: u8, cycles: &mut u32) {
        if address as usize > MAX_MEMORY {
            panic!("Write byte failed: Memory address {} is higher than MAX_MEMORY", address);
        }

        #[cfg(test)]
        debug!("Writing byte {:#04X} to memory at {:#06X}", byte, address);

        memory.write(address, byte);
        *cycles -= 1;
    }

    /// Write a word to memory
    fn write_word(memory: &mut Memory, address: u16, word: u16, cycles: &mut u32) {
        let high = (word >> 8) as u8;
        let low = (word & 0xFF) as u8;
        Self::write_byte(memory, address, low, cycles);
        Self::write_byte(memory, address + 1, high, cycles);
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
#[cfg_attr(test, derive(Debug, Clone))]
enum Register {
    /// The accumulator register
    A,
    /// The X register
    X,
    /// The Y register
    Y,
    /// The stack pointer
    S,
}

#[cfg(test)]
mod test {
    use log::LevelFilter;
    use crate::cpu::{Cpu, CpuStatusFlags};
    use crate::memory::Memory;
    use crate::ops::*;

    fn init() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .is_test(true).try_init();
    }

    #[test]
    fn lda_immediate() {
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
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

    #[test]
    fn sta_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x40), 0x32);
    }

    #[test]
    fn sta_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_accumulator = 0x32;
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x50), 0x32);
    }

    #[test]
    fn sta_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x4080
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x4080), 0x32);
    }

    #[test]
    fn sta_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_ABSOLUTE_X);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x4080
        cpu.register_accumulator = 0x32;
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x4090), 0x32);
    }

    #[test]
    fn sta_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x4080
        cpu.register_accumulator = 0x32;
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x4090), 0x32);
    }

    #[test]
    fn sta_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_INDIRECT_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        memory.write(0x50, 0x30);
        memory.write(0x51, 0x30); // 0x3030
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x3030), 0x32);
    }

    #[test]
    fn sta_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STA_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);
        cpu.register_y = 0x10;
        memory.write(0x50, 0x30);
        memory.write(0x51, 0x30); // 0x3030
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x3030), 0x32);
    }

    #[test]
    fn stx_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STX_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x40), 0x32);
    }

    #[test]
    fn stx_zero_page_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STX_ZERO_PAGE_Y);
        memory.write(0xFFFD, 0x40);
        cpu.register_y = 0x10;
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x50), 0x32);
    }

    #[test]
    fn stx_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STX_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x4080
        cpu.register_x = 0x32;

        let cycels_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycels_left, 0);
        assert_eq!(memory.fetch(0x4080), 0x32);
    }

    #[test]
    fn sty_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STY_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x40), 0x32);
    }

    #[test]
    fn sty_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STY_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x50), 0x32);
    }

    #[test]
    fn sty_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, STY_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x4080
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.fetch(0x4080), 0x32);
    }

    #[test]
    fn tax_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, TAX_IMPLIED);
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn tay_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, TAY_IMPLIED);
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn txa_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, TXA_IMPLIED);
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute(&mut memory,2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0x32);
    }

    #[test]
    fn tya_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, TYA_IMPLIED);
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0x32);
    }

    #[test]
    fn tsx_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, TSX_IMPLIED);
        cpu.stack_pointer = 0x32;

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn txs_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, TXS_IMPLIED);
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x32);
    }

    #[test]
    fn pha_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, PHA_IMPLIED);
        cpu.stack_pointer = 0x10;
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x11);
        assert_eq!(memory.fetch(0x0110), 0x32);

        cpu.reset();
        memory.reset();

        // Check the wrapping of the stack pointer
        memory.write(0xFFFC, PHA_IMPLIED);
        cpu.stack_pointer = 0xFF;
        cpu.execute(&mut memory, 3);
        assert_eq!(cpu.stack_pointer, 0x00);
    }

    #[test]
    fn php_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, PHP_IMPLIED);
        cpu.stack_pointer = 0x10;
        cpu.flags = CpuStatusFlags::all();

        let cycles_left = cpu.execute(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x11);
        assert_eq!(memory.fetch(0x0110), CpuStatusFlags::all().bits());

        cpu.reset();
        memory.reset();

        // Check the wrapping of the stack pointer
        memory.write(0xFFFC, PHA_IMPLIED);
        cpu.stack_pointer = 0xFF;
        cpu.execute(&mut memory, 3);
        assert_eq!(cpu.stack_pointer, 0x00);
    }

    #[test]
    fn pla_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, PLA_IMPLIED);
        memory.write(0x0120, 0x32);
        cpu.stack_pointer = 0x21;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x20);
        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        memory.reset();

        // Check wrapping behaviour of the stack pointer
        memory.write(0xFFFC, PLA_IMPLIED);
        cpu.stack_pointer = 0x00;
        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn plp_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        memory.write(0xFFFC, PLP_IMPLIED);
        memory.write(0x0120, CpuStatusFlags::all().bits());
        cpu.stack_pointer = 0x21;

        let cycles_left = cpu.execute(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x20);
        assert_eq!(cpu.flags, CpuStatusFlags::all());

        cpu.reset();
        memory.reset();

        // Check wrapping behaviour of the stack pointer
        memory.write(0xFFFC, PLA_IMPLIED);
        cpu.stack_pointer = 0x00;
        cpu.execute(&mut memory, 4);
        assert_eq!(cpu.stack_pointer, 0xFF);

    }
}