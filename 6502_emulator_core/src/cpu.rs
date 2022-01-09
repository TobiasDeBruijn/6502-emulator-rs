use core::num::Wrapping;
use bitflags::bitflags;
use crate::memory::{MAX_MEMORY, Memory};
use crate::ops::*;

#[cfg(test)]
use log::debug;

const NEGATIVE_BIT: u8 = 0b1000_0000;


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
        #[cfg(test)]
        debug!("Resetting CPU");

        *self = Self::default();
    }

    /// Execute instructions
    pub fn execute_single(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, mut cycles: u32) -> u32 {
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
                let addr = self.fetch_byte(memory, &mut cycles);
                self.load_register(memory, Register::A, addr as u16, &mut cycles);
            },
            LDA_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.load_register(memory, Register::A, addr, &mut cycles);
            },
            LDA_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.load_register(memory, Register::A, addr, &mut cycles);
            },
            LDA_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                self.load_register(memory, Register::A, addr, &mut cycles);
            },
            LDA_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                self.load_register(memory, Register::A, addr, &mut cycles);
            },
            LDA_INDIRECT_X => {
                let addr = self.addr_indirect_x(memory, &mut cycles);
                self.load_register(memory, Register::A, addr, &mut cycles);
            },
            LDA_INDIRECT_Y => {
                let addr = self.addr_indirect_y(memory, &mut cycles);
                self.load_register(memory, Register::A, addr, &mut cycles);
            },
            LDX_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.set_register(Register::X, value);
            },
            LDX_ZERO_PAGE => {
                let addr = self.fetch_byte(memory, &mut cycles);
                self.load_register(memory, Register::X, addr as u16, &mut cycles);
            },
            LDX_ZERO_PAGE_Y => {
                let addr = self.addr_zero_page_y(memory, &mut cycles);
                self.load_register(memory, Register::X, addr, &mut cycles);
            },
            LDX_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                self.load_register(memory, Register::X, address, &mut cycles);
            },
            LDX_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                self.load_register(memory, Register::X, addr, &mut cycles);
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
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.load_register(memory, Register::Y, addr, &mut cycles);
            },
            LDY_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                self.load_register(memory, Register::Y, address, &mut cycles);
            },
            LDY_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                self.load_register(memory, Register::Y, addr, &mut cycles);
            },
            STA_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                Self::write_byte(memory, zp_address as u16, self.register_accumulator, &mut cycles);
            },
            STA_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_accumulator, &mut cycles);
            },
            STA_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                Self::write_byte(memory, address, self.register_accumulator, &mut cycles);
            },
            STA_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_accumulator, &mut cycles);
            },
            STA_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y_5(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_accumulator, &mut cycles);
            },
            STA_INDIRECT_X => {
                let addr = self.addr_indirect_x(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_accumulator, &mut cycles);
            },
            STA_INDIRECT_Y => {
                let addr = self.addr_indirect_y_5(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_accumulator, &mut cycles);
            },
            STX_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                Self::write_byte(memory, zp_address as u16, self.register_x, &mut cycles);
            },
            STX_ZERO_PAGE_Y => {
                let addr = self.addr_zero_page_y(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_x, &mut cycles);
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
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                Self::write_byte(memory, addr, self.register_y, &mut cycles);
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
                cycles -= 1;
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
            },
            AND_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.logical_operation(value, LogicalOperation::And);
            },
            AND_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.fetch_logical_operation(memory, zp_address as u16, LogicalOperation::And, &mut cycles);
            },
            AND_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::And, &mut cycles);
            },
            AND_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::And, &mut cycles);
            },
            AND_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::And, &mut cycles);
            },
            AND_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::And, &mut cycles);
            },
            AND_INDIRECT_X => {
                let address = self.addr_indirect_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::And, &mut cycles);
            },
            AND_INDIRECT_Y => {
                let address = self.addr_indirect_y(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::And, &mut cycles);
            },
            EOR_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.logical_operation(value, LogicalOperation::Xor);
            },
            EOR_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.fetch_logical_operation(memory, zp_address as u16, LogicalOperation::Xor, &mut cycles);
            },
            EOR_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::Xor, &mut cycles);
            },
            EOR_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::Xor, &mut cycles);
            },
            EOR_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::Xor, &mut cycles);
            },
            EOR_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::Xor, &mut cycles);
            },
            EOR_INDIRECT_X => {
                let address = self.addr_indirect_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::Xor, &mut cycles);
            },
            EOR_INDIRECT_Y => {
                let address = self.addr_indirect_y(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::Xor, &mut cycles);
            },
            ORA_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.logical_operation(value, LogicalOperation::Or);
            },
            ORA_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.fetch_logical_operation(memory, zp_address as u16, LogicalOperation::Or, &mut cycles);
            },
            ORA_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::Or, &mut cycles);
            },
            ORA_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::Or, &mut cycles);
            },
            ORA_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::Or, &mut cycles);
            },
            ORA_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                self.fetch_logical_operation(memory, addr, LogicalOperation::Or, &mut cycles);
            },
            ORA_INDIRECT_X => {
                let address = self.addr_indirect_x(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::Or, &mut cycles);
            },
            ORA_INDIRECT_Y => {
                let address = self.addr_indirect_y(memory, &mut cycles);
                self.fetch_logical_operation(memory, address, LogicalOperation::Or, &mut cycles);
            },
            BIT_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.bit_test(memory, zp_address as u16, &mut cycles);
            },
            BIT_ABSOLUTE => {
                let address = self.fetch_word(memory, &mut cycles);
                self.bit_test(memory, address, &mut cycles);
            },
            ADC_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                let value = Self::read_byte(memory, zp_address as u16, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_INDIRECT_X => {
                let addr = self.addr_indirect_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.add_with_carry(value);
            },
            ADC_INDIRECT_Y => {
                let addr = self.addr_indirect_y(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.add_with_carry(value);
            },
            SBC_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                let value = Self::read_byte(memory, zp_address as u16, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_INDIRECT_X => {
                let addr = self.addr_indirect_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.subtract_with_carry(value);
            },
            SBC_INDIRECT_Y => {
                let addr = self.addr_indirect_y(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.subtract_with_carry(value);
            },
            CMP_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                let value = Self::read_byte(memory, zp_address as u16, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_ABSOLUTE_X => {
                let addr = self.addr_absolute_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_ABSOLUTE_Y => {
                let addr = self.addr_absolute_y(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_INDIRECT_X => {
                let addr = self.addr_indirect_x(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CMP_INDIRECT_Y => {
                let addr = self.addr_indirect_y(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::A, value);
            },
            CPX_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.compare_to_register(Register::X, value);
            },
            CPX_ZERO_PAGE => {
                let zp_addr = self.fetch_byte(memory, &mut cycles);
                let value = Self::read_byte(memory, zp_addr as u16, &mut cycles);
                self.compare_to_register(Register::X, value);
            },
            CPX_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::X, value);
            },
            CPY_IMMEDIATE => {
                let value = self.fetch_byte(memory, &mut cycles);
                self.compare_to_register(Register::Y, value);
            },
            CPY_ZERO_PAGE => {
                let zp_addr = self.fetch_byte(memory, &mut cycles);
                let value = Self::read_byte(memory, zp_addr as u16, &mut cycles);
                self.compare_to_register(Register::Y, value);
            },
            CPY_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                let value = Self::read_byte(memory, addr, &mut cycles);
                self.compare_to_register(Register::Y, value);
            },
            INC_ZERO_PAGE => {
                let zp_addr = self.fetch_byte(memory, &mut cycles);
                self.increment_memory(memory, zp_addr as u16, &mut cycles);
            },
            INC_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.increment_memory(memory, addr, &mut cycles);
            },
            INC_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.increment_memory(memory, addr, &mut cycles);
            },
            INC_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                self.increment_memory(memory, addr, &mut cycles);
            },
            INX => {
                self.increment_register(Register::X, &mut cycles);
            },
            INY => {
                self.increment_register(Register::Y, &mut cycles);
            }
            _ => {}
        }

        cycles
    }

    /// Increment a location in memory
    fn increment_memory(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let inc = (Wrapping(value) + Wrapping(1)).0;
        *cycles -= 1;

        self.flags.set(CpuStatusFlags::ZERO, inc == 0);
        self.flags.set(CpuStatusFlags::NEGATIVE, inc & NEGATIVE_BIT != 0);

        Self::write_byte(memory, address, inc, cycles);
    }

    /// Increment a register
    fn increment_register(&mut self, register: Register, cycles: &mut u32) {
        self.set_register(register.clone(), (Wrapping(self.get_register(register)) + Wrapping(1)).0);
        *cycles -= 1;
    }

    /// Retrieve the value from a Register. Only the `A`, `X`, and `Y` registers are supported
    fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y,
            _ => panic!("Unsupported register")
        }
    }

    /// Compare a value to the value of a register. Affects the Carry, Zero and Negative flags
    fn compare_to_register(&mut self, register: Register, value: u8) {
        let reg = match register {
            Register::A => self.register_accumulator,
            Register::X => self.register_x,
            Register::Y => self.register_y,
            _ => panic!("Unsupported register")
        };

        self.flags.set(CpuStatusFlags::CARRY, reg >= value);
        self.flags.set(CpuStatusFlags::ZERO, reg == value);
        self.flags.set(CpuStatusFlags::NEGATIVE, (reg as i16 - value as i16) & 0b1000_0000 != 0);
    }

    /// Add with carry. Affects the Carry and Overflow flags
    fn add_with_carry(&mut self, value: u8) {
        if self.flags.intersects(CpuStatusFlags::DECIMAL_MODE) {
            todo!("Decimal mode hasn't been implemented yet")
        }

        // TODO overflow flag isn't set correct

        let sign_bits_eq = !(((self.register_accumulator ^ value) & NEGATIVE_BIT) != 0);

        let carry_bit: u8 = if self.flags.intersects(CpuStatusFlags::CARRY) {
            0b1
        } else {
            0
        };

        // We perform this operation as a u16 to avoid panics on overflow or underflow
        let sum = self.register_accumulator as u16 + value as u16 + carry_bit as u16;
        self.set_register(Register::A, (sum & 0xFF) as u8);

        self.flags.set(CpuStatusFlags::CARRY, sum > 0xFF);
        self.flags.set(CpuStatusFlags::OVERFLOW, sign_bits_eq && ((self.register_accumulator ^ value) & NEGATIVE_BIT) != 0);
    }

    /// Subtract with carry. Affects the Carry and Overflow flags
    fn subtract_with_carry(&mut self, value: u8) {
        self.add_with_carry(!value);
    }

    /// Perform a bit test on the value in the provided memory address
    fn bit_test(&mut self, memory: &dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let result = value | self.register_accumulator;

        if result == 0 {
            #[cfg(test)]
            debug!("Setting zero flag");
            self.flags.set(CpuStatusFlags::ZERO, true);
        } else {
            #[cfg(test)]
            debug!("Unsetting zero flag");
            self.flags.set(CpuStatusFlags::ZERO, false);
        }

        // Check the 6th bit (counted from 0)
        if result & (1 << 6) != 0 {
            #[cfg(test)]
            debug!("Setting overflow flag");
            self.flags.set(CpuStatusFlags::OVERFLOW, true);
        } else {
            #[cfg(test)]
            debug!("Unsetting overflow flag");
            self.flags.set(CpuStatusFlags::OVERFLOW, false);
        }

        // Check the 6th bit (counted from 0)
        if result & (1 << 7) != 0 {
            #[cfg(test)]
            debug!("Setting negative flag");
            self.flags.set(CpuStatusFlags::NEGATIVE, true);
        } else {
            #[cfg(test)]
            debug!("Unsetting negative flag");
            self.flags.set(CpuStatusFlags::NEGATIVE, false);
        }
    }

    /// The address to be accessed by an instruction using indexed zero page addressing is calculated
    /// by taking the 8 bit zero page address from the instruction and adding the current value of the `X` register to it
    fn addr_zero_page_x(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let zp_address = self.fetch_byte(memory, cycles);
        let zp_address_x = (Wrapping(zp_address) + Wrapping(self.register_x)).0;
        *cycles -= 1;
        zp_address_x as u16
    }

    /// The address to be accessed by an instruction using indexed zero page addressing is calculated
    /// by taking the 8 bit zero page address from the instruction and adding the current value of the `Y` register to it
    fn addr_zero_page_y(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let zp_address = self.fetch_byte(memory, cycles);
        let zp_address_y = (Wrapping(zp_address) + Wrapping(self.register_y)).0;
        *cycles -= 1;
        zp_address_y as u16
    }

    /// The address to be accessed by an instruction using `X` register indexed absolute
    /// addressing is computed by taking the 16 bit address from the instruction and added the contents of the `X` register.
    /// 2 + 1 cycle if page cross
    fn addr_absolute_x(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let addr = self.fetch_word(memory, cycles);
        let addr_x = addr + self.register_x as u16;

        if (addr ^ addr_x) >> 8 != 0 {
            *cycles -= 1;
        }

        addr_x
    }

    /// The address to be accessed by an instruction using `X` register indexed absolute
    /// addressing is computed by taking the 16 bit address from the instruction and added the contents of the `X` register.
    /// Always takes 3 cycles.
    fn addr_absolute_x_5(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let addr = self.fetch_word(memory, cycles);
        let addr_x = addr + self.register_x as u16;
        *cycles -= 1;
        addr_x
    }

    /// The address to be accessed by an instruction using `Y` register indexed absolute
    /// addressing is computed by taking the 16 bit address from the instruction and added the contents of the `Y` register.
    /// 2 + 1 cycle if page cross
    fn addr_absolute_y(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let addr = self.fetch_word(memory, cycles);
        let addr_y = addr + self.register_y as u16;

        if (addr ^ addr_y) >> 8 != 0 {
            *cycles -= 1;
        }

        addr_y
    }

    /// The address to be accessed by an instruction using `Y` register indexed absolute
    /// addressing is computed by taking the 16 bit address from the instruction and added the contents of the `Y` register.
    /// Always takes 3 cycles.
    fn addr_absolute_y_5(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let addr = self.fetch_word(memory, cycles);
        let addr_y = addr + self.register_y as u16;
        *cycles -= 1;
        addr_y
    }

    /// Indexed indirect addressing is normally used in conjunction with a table of address held on zero page.
    /// The address of the table is taken from the instruction and the X register added to it (with zero page wrap around)
    /// to give the location of the least significant byte of the target address.
    fn addr_indirect_x(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let address = self.fetch_byte(memory, cycles);
        let address_x = (Wrapping(address) + Wrapping(self.register_x)).0;
        *cycles -= 1;

        Self::read_word(memory, address_x as u16, cycles)
    }


    /// In instruction contains the zero page location of the least significant byte of 16 bit address.
    /// The `Y` register is dynamically added to this value to generated the actual target address for operation.
    /// 3 + 1 cycle if page cross
    fn addr_indirect_y(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let address = self.fetch_byte(memory, cycles) as u16;
        let effective_address = Self::read_word(memory, address as u16, cycles);
        let effective_address_y = effective_address + self.register_y as u16;

        if (effective_address ^ effective_address_y) >> 8 != 0 {
            *cycles -= 1;
        }

        effective_address_y
    }

    /// In instruction contains the zero page location of the least significant byte of 16 bit address.
    /// The `Y` register is dynamically added to this value to generated the actual target address for operation.
    /// Always takes 4 cycles
    fn addr_indirect_y_5(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let address = self.fetch_byte(memory, cycles) as u16;
        let effective_address = Self::read_word(memory, address as u16, cycles);
        let effective_address_y = effective_address + self.register_y as u16;
        *cycles -= 1;
        effective_address_y
    }

    /// Fetch a byte from the provided location in memory and perform the logical operation
    /// on it with the current value of the `A` register. The result is placed in the `A` register
    fn fetch_logical_operation(&mut self, memory: &dyn Memory<MAX_MEMORY>, address: u16, op: LogicalOperation, cycles: &mut u32) {
        let byte = Self::read_byte(memory, address, cycles);
        self.logical_operation(byte, op);
    }

    /// Perform the logical operation on the contents of the `A` register with the provided byte
    /// and put the result in the `A` register
    fn logical_operation(&mut self, byte: u8, op: LogicalOperation) {
        let result = match op {
            LogicalOperation::And => self.register_accumulator & byte,
            LogicalOperation::Or => self.register_accumulator | byte,
            LogicalOperation::Xor => self.register_accumulator ^ byte,
        };

        self.set_register(Register::A, result);
    }

    /// Fetch a word from Memory. This will increment the program counter twice
    fn fetch_word(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u16 {
        let low = self.fetch_byte(memory, cycles) as u16;
        let high = self.fetch_byte(memory, cycles) as u16;
        high << 8 | low
    }

    /// Load a value from an address into a register
    fn load_register(&mut self, memory: &dyn Memory<MAX_MEMORY>, register: Register, address: u16, cycles: &mut u32) {
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
        } else {
            #[cfg(test)]
            debug!("Unsetting zero flag for register {:?}", register);

            self.flags.set(CpuStatusFlags::ZERO, false);
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
        } else {
            #[cfg(test)]
            debug!("Unsetting negative flag for register {:?}", register);

            self.flags.set(CpuStatusFlags::NEGATIVE, false);
        }
    }

    /// Fetch a byte from memory at the program_counter and increment it
    fn fetch_byte(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u8 {
        let byte = memory.read(self.program_counter);
        self.program_counter += 1;
        *cycles -= 1;

        #[cfg(test)]
        debug!("Fetched byte from {:#04X}: {:#04X}", self.program_counter -1, byte);

        byte
    }

    /// Read a Word from memory. This reads `address` and `address + 1`
    fn read_word(memory: &dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) -> u16 {
        if (address + 1) as usize > MAX_MEMORY {
            panic!("Read word failed: Memory address {} is higher than MAX_MEMORY", address);
        }

        let low = Self::read_byte(memory, address, cycles) as u16;
        let high = Self::read_byte(memory, address + 1, cycles) as u16;
        high << 8 | low
    }

    /// Read a byte from memory
    fn read_byte(memory: &dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) -> u8 {
        if address as usize > MAX_MEMORY {
            panic!("Read byte failed: Memory address {} is higher than MAX_MEMORY", address);
        }

        let byte = memory.read(address);
        *cycles -= 1;

        #[cfg(test)]
        debug!("Read byte from memory at {:#06X}: {:#04X}", address, byte);

        byte
    }

    /// Write a byte to memory
    fn write_byte(memory: &mut dyn Memory<MAX_MEMORY>, address: u16, byte: u8, cycles: &mut u32) {
        if address as usize > MAX_MEMORY {
            panic!("Write byte failed: Memory address {} is higher than MAX_MEMORY", address);
        }

        #[cfg(test)]
        debug!("Writing byte {:#04X} to memory at {:#06X}", byte, address);

        memory.write(address, byte);
        *cycles -= 1;
    }

    /// Write a word to memory
    #[allow(unused)]
    fn write_word(memory: &mut dyn Memory<MAX_MEMORY>, address: u16, word: u16, cycles: &mut u32) {
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
#[derive(Clone, Debug)]
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

/// A logical operation
#[cfg_attr(test, derive(Debug, Clone))]
enum LogicalOperation {
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Logical XOR
    Xor
}

#[cfg(test)]
mod test {
    use log::LevelFilter;
    use crate::cpu::{Cpu, CpuStatusFlags};
    use crate::Memory;
    use crate::memory::BasicMemory;
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
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, 0xA9);
        memory.write(0xFFFD, 0x42);

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_accumulator, 0x42);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0xFFFD, 0x42 + 0b1000_0000); // Make the number negative by enabling the left most bit

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_accumulator, 0x42 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0xFFFD, 0x0);

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDA_ZERO_PAGE);
        memory.write(0xFFFD, 0x42);
        memory.write(0x0042, 0x10);

        cpu.execute_single(&mut memory, 3);
        assert_eq!(cpu.register_accumulator, 0x10);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0x42, 0x10 + 0b1000_0000);

        cpu.execute_single(&mut memory, 3);
        assert_eq!(cpu.register_accumulator, 0x10 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.write(0x42, 0x0);

        cpu.execute_single(&mut memory, 3);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDA_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x10);
        memory.write(0x20, 0x42);
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x42);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x0020, 0x42 + 0b1000_0000);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x42 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x0020, 0x0);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDA_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x44); // Loads from 0x4480
        memory.write(0x4480, 0x64);

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x64);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x4480, 0x64 + 0b1000_0000);
        cpu.reset();

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x64 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x4480, 0x0);
        cpu.reset();

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();
        memory.write(0xFFFC, LDA_ABSOLUTE_X);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040

        memory.write(0x8050, 0x32);
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x8050, 0x32 + 0b1000_0000);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x8050, 0x0);
        cpu.reset();
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDA_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040

        memory.write(0x8050, 0x32);
        cpu.register_y = 0x10;

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0x10;
        memory.write(0x8050, 0x32 + 0b1000_0000);

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0x10;
        memory.write(0x8050, 0x0);

        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDA_INDIRECT_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0x30, 0x80);

        memory.write(0x80, 0x32);
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0x10;
        memory.write(0x80, 0x32 + 0b1000_0000);

        cpu.execute_single(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0x10;
        memory.write(0x80, 0x0);

        cpu.execute_single(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lda_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDA_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0x40, 0x50);

        memory.write(0x60, 0x32);
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x60, 0x32 + 0b1000_0000);
        cpu.reset();
        cpu.register_y = 0x10;

        cpu.execute_single(&mut memory, 5);
        assert_eq!(cpu.register_accumulator, 0x32 + 0b1000_0000);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        memory.write(0x60, 0x0);
        cpu.reset();
        cpu.register_y = 0x10;

        cpu.execute_single(&mut memory, 5);
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

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cpu.register_accumulator, 0x32);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn ldx_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDX_IMMEDIATE);
        memory.write(0xFFFD, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDX_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_zero_page_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDX_ZERO_PAGE_Y);
        memory.write(0xFFFD, 0x20);
        memory.write(0x30, 0x32);
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDX_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        memory.write(0x8040, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldx_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDX_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); //0x8040;
        cpu.register_y = 0x10;
        memory.write(0x8050, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, LDX_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x10); // 0x1010
        cpu.register_y = 0xFF;
        memory.write(0x110F, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn ldy_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDY_IMMEDIATE);
        memory.write(0xFFFD, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDY_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDY_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0x30, 0x32);
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDY_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        memory.write(0x8040, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn ldy_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LDY_ABSOLUTE_X);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); //0x8040;
        cpu.register_x = 0x10;
        memory.write(0x8050, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, LDY_ABSOLUTE_X);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x10); // 0x1010
        cpu.register_x = 0xFF;
        memory.write(0x110F, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn sta_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x40), 0x32);
    }

    #[test]
    fn sta_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_accumulator = 0x32;
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x50), 0x32);
    }

    #[test]
    fn sta_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x4080
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4080), 0x32);
    }

    #[test]
    fn sta_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_ABSOLUTE_X);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x4080
        cpu.register_accumulator = 0x32;
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4090), 0x32);
    }

    #[test]
    fn sta_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x4080
        cpu.register_accumulator = 0x32;
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4090), 0x32);
    }

    #[test]
    fn sta_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_INDIRECT_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        memory.write(0x50, 0x30);
        memory.write(0x51, 0x30); // 0x3030
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x3030), 0x32);
    }

    #[test]
    fn sta_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STA_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);

        memory.write(0x40, 0x30);
        memory.write(0x41, 0x30); // 0x3030;
        cpu.register_y = 0x10;
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x3040), 0x32);
    }

    #[test]
    fn stx_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STX_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x40), 0x32);
    }

    #[test]
    fn stx_zero_page_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STX_ZERO_PAGE_Y);
        memory.write(0xFFFD, 0x40);
        cpu.register_y = 0x10;
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x50), 0x32);
    }

    #[test]
    fn stx_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STX_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x4080
        cpu.register_x = 0x32;

        let cycels_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycels_left, 0);
        assert_eq!(memory.read(0x4080), 0x32);
    }

    #[test]
    fn sty_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STY_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x40), 0x32);
    }

    #[test]
    fn sty_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STY_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x50), 0x32);
    }

    #[test]
    fn sty_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, STY_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x4080
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4080), 0x32);
    }

    #[test]
    fn tax_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, TAX_IMPLIED);
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn tay_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, TAY_IMPLIED);
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x32);
    }

    #[test]
    fn txa_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, TXA_IMPLIED);
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0x32);
    }

    #[test]
    fn tya_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, TYA_IMPLIED);
        cpu.register_y = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0x32);
    }

    #[test]
    fn tsx_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, TSX_IMPLIED);
        cpu.stack_pointer = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x32);
    }

    #[test]
    fn txs_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, TXS_IMPLIED);
        cpu.register_x = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x32);
    }

    #[test]
    fn pha_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, PHA_IMPLIED);
        cpu.stack_pointer = 0x10;
        cpu.register_accumulator = 0x32;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x11);
        assert_eq!(memory.read(0x0110), 0x32);

        cpu.reset();
        memory.reset();

        // Check the wrapping of the stack pointer
        memory.write(0xFFFC, PHA_IMPLIED);
        cpu.stack_pointer = 0xFF;
        cpu.execute_single(&mut memory, 3);
        assert_eq!(cpu.stack_pointer, 0x00);
    }

    #[test]
    fn php_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, PHP_IMPLIED);
        cpu.stack_pointer = 0x10;
        cpu.flags = CpuStatusFlags::all();

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x11);
        assert_eq!(memory.read(0x0110), CpuStatusFlags::all().bits());

        cpu.reset();
        memory.reset();

        // Check the wrapping of the stack pointer
        memory.write(0xFFFC, PHA_IMPLIED);
        cpu.stack_pointer = 0xFF;
        cpu.execute_single(&mut memory, 3);
        assert_eq!(cpu.stack_pointer, 0x00);
    }

    #[test]
    fn pla_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, PLA_IMPLIED);
        memory.write(0x0120, 0x32);
        cpu.stack_pointer = 0x21;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x20);
        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        memory.reset();

        // Check wrapping behaviour of the stack pointer
        memory.write(0xFFFC, PLA_IMPLIED);
        cpu.stack_pointer = 0x00;
        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn plp_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, PLP_IMPLIED);
        memory.write(0x0120, CpuStatusFlags::all().bits());
        cpu.stack_pointer = 0x21;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.stack_pointer, 0x20);
        assert_eq!(cpu.flags, CpuStatusFlags::all());

        cpu.reset();
        memory.reset();

        // Check wrapping behaviour of the stack pointer
        memory.write(0xFFFC, PLA_IMPLIED);
        cpu.stack_pointer = 0x00;
        cpu.execute_single(&mut memory, 4);
        assert_eq!(cpu.stack_pointer, 0xFF);

    }

    #[test]
    fn and_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_IMMEDIATE);
        memory.write(0xFFFD, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);
    }

    #[test]
    fn and_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);
    }

    #[test]
    fn and_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);
    }

    #[test]
    fn and_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        memory.write(0x8040, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);
    }

    #[test]
    fn and_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_ABSOLUTE_X);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        cpu.register_x = 0x10;
        memory.write(0x8050, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, AND_ABSOLUTE_X);
        memory.write(0xFFFD, 0xFF);
        memory.write(0xFFFE, 0x00); // 0x00FF
        cpu.register_x = 0x1;
        memory.write(0x100, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
    }

    #[test]
    fn and_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        cpu.register_y = 0x10;
        memory.write(0x8050, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, AND_ABSOLUTE_Y);
        memory.write(0xFFFD, 0xFF);
        memory.write(0xFFFE, 0x00); // 0x00FF
        cpu.register_y = 0x1;
        memory.write(0x100, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
    }

    #[test]
    fn and_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_INDIRECT_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        memory.write(0x50, 0x80);
        memory.write(0x51, 0x40); // 0x4080;
        memory.write(0x4080, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);
    }

    #[test]
    fn and_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, AND_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);

        memory.write(0x40, 0x30);
        memory.write(0x41, 0x30); // 0x3030;

        cpu.register_y = 0x10;
        memory.write(0x3040, 0b1010);

        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1000);
    }

    #[test]
    fn eor_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_IMMEDIATE);
        memory.write(0xFFFD, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);
    }

    #[test]
    fn eor_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);
    }

    #[test]
    fn eor_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);
    }

    #[test]
    fn eor_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        memory.write(0x8040, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);
    }

    #[test]
    fn eor_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_ABSOLUTE_X);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        cpu.register_x = 0x10;
        memory.write(0x8050, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, EOR_ABSOLUTE_X);
        memory.write(0xFFFD, 0xFF);
        memory.write(0xFFFE, 0x00); // 0x00FF
        cpu.register_x = 0x1;
        memory.write(0x100, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
    }

    #[test]
    fn eor_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        cpu.register_y = 0x10;
        memory.write(0x8050, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, EOR_ABSOLUTE_Y);
        memory.write(0xFFFD, 0xFF);
        memory.write(0xFFFE, 0x00); // 0x00FF
        cpu.register_y = 0x1;
        memory.write(0x100, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
    }

    #[test]
    fn eor_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_INDIRECT_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        memory.write(0x50, 0x80);
        memory.write(0x51, 0x40); // 0x4080;
        memory.write(0x4080, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);
    }

    #[test]
    fn eor_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, EOR_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);

        memory.write(0x40, 0x30);
        memory.write(0x41, 0x30); // 0x3030;

        cpu.register_y = 0x10;
        memory.write(0x3040, 0b1010);

        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0110);
    }

    #[test]
    fn ora_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_IMMEDIATE);
        memory.write(0xFFFD, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);
    }

    #[test]
    fn ora_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);
    }

    #[test]
    fn ora_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);
    }

    #[test]
    fn ora_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_ABSOLUTE);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        memory.write(0x8040, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);
    }

    #[test]
    fn ora_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_ABSOLUTE_X);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        cpu.register_x = 0x10;
        memory.write(0x8050, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, ORA_ABSOLUTE_X);
        memory.write(0xFFFD, 0xFF);
        memory.write(0xFFFE, 0x00); // 0x00FF
        cpu.register_x = 0x1;
        memory.write(0x100, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
    }

    #[test]
    fn ora_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x40);
        memory.write(0xFFFE, 0x80); // 0x8040
        cpu.register_y = 0x10;
        memory.write(0x8050, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, ORA_ABSOLUTE_Y);
        memory.write(0xFFFD, 0xFF);
        memory.write(0xFFFE, 0x00); // 0x00FF
        cpu.register_y = 0x1;
        memory.write(0x100, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
    }

    #[test]
    fn ora_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_INDIRECT_X);
        memory.write(0xFFFD, 0x40);
        cpu.register_x = 0x10;
        memory.write(0x50, 0x80);
        memory.write(0x51, 0x40); // 0x4080;
        memory.write(0x4080, 0b1010);
        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);
    }

    #[test]
    fn ora_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ORA_INDIRECT_Y);
        memory.write(0xFFFD, 0x40);

        memory.write(0x40, 0x30);
        memory.write(0x41, 0x30); // 0x3030;

        cpu.register_y = 0x10;
        memory.write(0x3040, 0b1110);

        cpu.register_accumulator = 0b1100;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1110);
    }

    #[test]
    fn bit_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, BIT_ZERO_PAGE);
        memory.write(0xFFFD, 0x40);
        memory.write(0x40, 0b1111_0000);
        cpu.register_accumulator = 0b1111_1111;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(cpu.flags.intersects(CpuStatusFlags::OVERFLOW));
    }

    #[test]
    fn bit_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, BIT_ABSOLUTE);
        memory.write(0xFFFD, 0x80);
        memory.write(0xFFFE, 0x40); // 0x4080;
        memory.write(0x4080, 0b1111_0000);
        cpu.register_accumulator = 0b1111_1111;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(cpu.flags.intersects(CpuStatusFlags::OVERFLOW));
    }

    #[test]
    fn adc_immeditate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ADC_IMMEDIATE);
        memory.write(0xFFFD, 0b1000);
        cpu.register_accumulator = 0b1;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b1001);

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, ADC_IMMEDIATE);
        memory.write(0xFFFD, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        // TODO: Broken
     //  assert_eq!(cpu.register_accumulator, 0xE);
     //   assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
     //   assert!(cpu.flags.intersects(CpuStatusFlags::OVERFLOW));
    }

    // TODO: ADC and SBC tests

    #[test]
    fn cmp_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_IMMEDIATE);
        memory.write(0xFFFD, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CMP_IMMEDIATE);
        memory.write(0xFFFD, 0x10);
        cpu.register_accumulator = 0x32;

        cpu.execute_single(&mut memory, 2);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CMP_IMMEDIATE);
        memory.write(0xFFFD, 0x32);
        cpu.register_accumulator = 0x10;

        cpu.execute_single(&mut memory, 2);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_ZERO_PAGE);
        memory.write(0xFFFD, 0x10);
        memory.write(0x10, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x10);
        cpu.register_x = 0x10;
        memory.write(0x20, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_ABSOLUTE);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        memory.write(0x8010, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_ABSOLUTE_X);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        cpu.register_x = 0x10;
        memory.write(0x8020, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CMP_ABSOLUTE_X);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        cpu.register_x = 0xFF;
        memory.write(0x810F, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_absolute_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        cpu.register_y = 0x10;
        memory.write(0x8020, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CMP_ABSOLUTE_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        cpu.register_y = 0xFF;
        memory.write(0x810F, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_indirect_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_INDIRECT_X);
        memory.write(0xFFFD, 0x10);
        cpu.register_x = 0x10;
        memory.write(0x20, 0x20);
        memory.write(0x21, 0x30); // 0x3020
        memory.write(0x3020, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cmp_indirect_y() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CMP_INDIRECT_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0x10, 0x20);
        memory.write(0x11, 0x30); // 0x3020
        cpu.register_y = 0x10;
        memory.write(0x3030, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CMP_INDIRECT_Y);
        memory.write(0xFFFD, 0x10);
        memory.write(0x10, 0x20);
        memory.write(0x11, 0x30); // 0x3020
        cpu.register_y = 0xFF;
        memory.write(0x311F, 0xFF);
        cpu.register_accumulator = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cpx_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CPX_IMMEDIATE);
        memory.write(0xFFFD, 0xFF);
        cpu.register_x = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CPX_IMMEDIATE);
        memory.write(0xFFFD, 0x10);
        cpu.register_x = 0x32;

        cpu.execute_single(&mut memory, 2);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CPX_IMMEDIATE);
        memory.write(0xFFFD, 0x32);
        cpu.register_x = 0x10;

        cpu.execute_single(&mut memory, 2);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cpx_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CPX_ZERO_PAGE);
        memory.write(0xFFFD, 0x10);
        memory.write(0x10, 0xFF);
        cpu.register_x = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cpx_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CPX_ABSOLUTE);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        memory.write(0x8010, 0xFF);
        cpu.register_x = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cpy_immediate() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CPY_IMMEDIATE);
        memory.write(0xFFFD, 0xFF);
        cpu.register_y = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CPY_IMMEDIATE);
        memory.write(0xFFFD, 0x10);
        cpu.register_y = 0x32;

        cpu.execute_single(&mut memory, 2);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, CPY_IMMEDIATE);
        memory.write(0xFFFD, 0x32);
        cpu.register_y = 0x10;

        cpu.execute_single(&mut memory, 2);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cpy_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CPY_ZERO_PAGE);
        memory.write(0xFFFD, 0x10);
        memory.write(0x10, 0xFF);
        cpu.register_y = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn cpy_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, CPY_ABSOLUTE);
        memory.write(0xFFFD, 0x10);
        memory.write(0xFFFE, 0x80); // 0x8010
        memory.write(0x8010, 0xFF);
        cpu.register_y = 0xFF;

        let cycles_left = cpu.execute_single(&mut memory, 4);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn inc_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INC_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x20), 0x11);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, INC_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0xEF);

        cpu.execute_single(&mut memory, 5);
        assert_eq!(memory.read(0x20), 0xF0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, INC_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0xFF);

        cpu.execute_single(&mut memory, 5);
        assert_eq!(memory.read(0x20), 0x00);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn inc_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INC_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x20;
        memory.write(0x40, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x40), 0x11);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn inc_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INC_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        memory.write(0x4020, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4020), 0x11);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn inc_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INC_ABSOLUTE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        cpu.register_x = 0x20;
        memory.write(0x4040, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4040), 0x11);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn inx() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INX);
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0x11);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0xEF;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_x, 0xF0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0xFF;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn iny() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INY);
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0x11);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0xEF;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_y, 0xF0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0xFF;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }
}