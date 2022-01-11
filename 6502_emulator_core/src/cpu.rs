use core::num::Wrapping;
use bitflags::bitflags;
use crate::memory::{MAX_MEMORY, Memory};
use crate::ops::*;

#[cfg(test)]
use log::debug;

const NEGATIVE_BIT: u8 = 0b1000_0000;

const IRQ_INTERRUPT_VECTOR: u16 = 0xFFFE;

pub struct Cpu {
    program_counter: u16,
    #[allow(unused)]
    stack_pointer: u8,

    register_accumulator: u8,
    register_x: u8,
    register_y: u8,

    flags: CpuStatusFlags,

    mode: OperatingMode,
}

/// This indicates what 6502 'version' to use. This affects certain instructions like `JMP`
pub enum OperatingMode {
    /// The Mos mode uses the 'old' mode, i.e with it's bugs
    /// The most notable bug is in the `JMP` instruction:
    /// ```text
    /// An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00. This is fixed in some later chips like the 65SC02 so for compatibility always ensure the indirect vector is not at the end of the page.
    /// ```
    Mos,
    /// The Wdc mode is the 'modern' mode, with the applied bugfixes, most notably the `JMP` bug
    Wdc,
}

impl Default for Cpu {
    /// Create a default `CPU`. This sets the stack pointer to `0xFF` and the program counter to `0xFFFC`
    fn default() -> Self {
        Self {
            program_counter: 0xFFFC,
            stack_pointer: 0xFF,
            register_accumulator: 0,
            register_x: 0,
            register_y: 0,
            flags: CpuStatusFlags::default(),
            mode: OperatingMode::Wdc,
        }
    }
}

impl Cpu {
    /// Create a new `CPU`. Equivalent to [Self::default]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `CPU` with a set [OperatingMode]
    pub fn with_mode(mode: OperatingMode) -> Self {
        Self {
            mode,
            ..Self::default()
        }
    }

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
                self.stack_push(memory, self.register_accumulator, &mut cycles);
                cycles -= 1;
            },
            PHP_IMPLIED => {
                self.stack_push(memory, self.flags.bits(), &mut cycles);
                cycles -= 1;
            },
            PLA_IMPLIED => {
                let value = self.stack_pop(memory, &mut cycles);
                self.set_register(Register::A, value);
                cycles -= 2;
            },
            PLP_IMPLIED => {
                let byte = self.stack_pop(memory, &mut cycles);
                self.flags = CpuStatusFlags::from_bits_truncate(byte);
                cycles -= 2;
            },

            // Logical
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

            // Arithmetic
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

            // Increments & Decrements
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
            INX_IMPLIED => {
                self.increment_register(Register::X, &mut cycles);
            },
            INY_IMPLIED => {
                self.increment_register(Register::Y, &mut cycles);
            },
            DEC_ZERO_PAGE => {
                let zp_addr = self.fetch_byte(memory, &mut cycles);
                self.decrement_memory(memory, zp_addr as u16, &mut cycles);
            },
            DEC_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.decrement_memory(memory, addr, &mut cycles);
            },
            DEC_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.decrement_memory(memory, addr, &mut cycles);
            },
            DEC_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                self.decrement_memory(memory, addr, &mut cycles);
            },
            DEX_IMPLIED => {
                self.decrement_register(Register::X, &mut cycles);
            },
            DEY_IMPLIED => {
                self.decrement_register(Register::Y, &mut cycles);
            },

            // Shifts
            ASL_ACCUMULATOR => {
                let carry = self.register_accumulator & 0b1000_0000 != 0;
                self.register_accumulator <<=  1;
                self.flags.set(CpuStatusFlags::CARRY, carry);
                self.flags.set(CpuStatusFlags::ZERO, self.register_accumulator == 0);
                self.flags.set(CpuStatusFlags::NEGATIVE, self.register_accumulator & NEGATIVE_BIT != 0);
                cycles -= 1;
            },
            ASL_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.arithmetic_shift_left(memory, zp_address as u16, &mut cycles);
            },
            ASL_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.arithmetic_shift_left(memory, addr, &mut cycles);
            },
            ASL_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.arithmetic_shift_left(memory, addr, &mut cycles);
            },
            ASL_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                self.arithmetic_shift_left(memory, addr, &mut cycles);
            },
            LSR_ACCUMULATOR => {
                let carry = self.register_accumulator & 0b1 != 0;
                self.register_accumulator >>= 1;
                self.flags.set(CpuStatusFlags::CARRY, carry);
                self.flags.set(CpuStatusFlags::ZERO, self.register_accumulator == 0);
                // Shifting right means the 7th bit will be set to zero
                // i.e this flag is always false
                self.flags.set(CpuStatusFlags::NEGATIVE, false);
                cycles -= 1;
            },
            LSR_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.logical_shift_right(memory, zp_address as u16, &mut cycles);
            },
            LSR_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.logical_shift_right(memory, addr, &mut cycles);
            },
            LSR_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.logical_shift_right(memory, addr, &mut cycles);
            },
            LSR_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                self.logical_shift_right(memory, addr, &mut cycles);
            },
            ROL_ACCUMULATOR => {
                let set_carry = self.register_accumulator & 0b1000_0000 != 0;
                self.register_accumulator = self.register_accumulator << 1 | self.flag_as_bit(CpuStatusFlags::CARRY);
                self.flags.set(CpuStatusFlags::CARRY, set_carry);
                self.flags.set(CpuStatusFlags::ZERO, self.register_accumulator == 0);
                self.flags.set(CpuStatusFlags::NEGATIVE, self.register_accumulator & 0b1000_0000 != 0);
                cycles -= 1;
            },
            ROL_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.rotate_left(memory, zp_address as u16, &mut cycles);
            },
            ROL_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.rotate_left(memory, addr, &mut cycles);
            },
            ROL_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.rotate_left(memory, addr, &mut cycles);
            },
            ROL_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                self.rotate_left(memory, addr, &mut cycles);
            },
            ROR_ACCUMULATOR => {
                let set_carry = self.register_accumulator & 0b0000_0001 != 0;
                self.register_accumulator = self.register_accumulator >> 1 | (self.flag_as_bit(CpuStatusFlags::CARRY) << 7);
                self.flags.set(CpuStatusFlags::CARRY, set_carry);
                self.flags.set(CpuStatusFlags::ZERO, self.register_accumulator == 0);
                self.flags.set(CpuStatusFlags::NEGATIVE, self.register_accumulator & 0b1000_0000 != 0);
                cycles -= 1;
            },
            ROR_ZERO_PAGE => {
                let zp_address = self.fetch_byte(memory, &mut cycles);
                self.rotate_right(memory, zp_address as u16, &mut cycles);
            },
            ROR_ZERO_PAGE_X => {
                let addr = self.addr_zero_page_x(memory, &mut cycles);
                self.rotate_right(memory, addr, &mut cycles);
            },
            ROR_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.rotate_right(memory, addr, &mut cycles);
            },
            ROR_ABSOLUTE_X => {
                let addr = self.addr_absolute_x_5(memory, &mut cycles);
                self.rotate_right(memory, addr, &mut cycles);
            },

            // Jumps & Calls
            JMP_ABSOLUTE => {
                let addr = self.fetch_word(memory, &mut cycles);
                self.program_counter = addr;
            },
            JMP_INDIRECT => {
                let addr = self.fetch_word(memory, &mut cycles);

                let effective_addr = match self.mode {
                    OperatingMode::Mos => {
                        let low = Self::read_byte(memory, addr, &mut cycles) as u16;
                        let high = Self::read_byte(memory, addr & 0xFF00, &mut cycles) as u16;
                        high << 8 | low
                    },
                    OperatingMode::Wdc => Self::read_word(memory, addr, &mut cycles)
                };
                self.program_counter = effective_addr;
            },
            JSR_ABSOLUTE => {
                let target_addr = self.fetch_word(memory, &mut cycles);

                // We dont add anything because the JSR is byte 0, then the target_addr is byte 1 and 2,
                // so the next instruction is byte 3.
                // The PC is currently at byte 3, because fetching the instruction
                // increments it, and fetching a word increments it twice
                let src_addr = self.program_counter as u16;
                let low = (src_addr & 0xFF) as u8;
                let high = (src_addr >> 8) as u8;

                // The stack runs from 0x0100 - 0x01FF
                // But the stack pointer stores only the least significant byte
                Self::write_byte(memory, 0x0100 + (self.stack_pointer as u16), low, &mut cycles);
                self.stack_pointer = (Wrapping(self.stack_pointer) + Wrapping(1)).0;
                Self::write_byte(memory, 0x0100 + (self.stack_pointer as u16), high, &mut cycles);
                self.stack_pointer = (Wrapping(self.stack_pointer) + Wrapping(1)).0;

                self.program_counter = target_addr;

                cycles -= 1;
            },
            RTS_IMPLIED => {
                // The stack is literally a stack of addresses,
                // the 6502 is little endian, so the LSB gets put on the stack first,
                // followed by the MSB. This means, that the MSB must be popped first,
                // followed by the LSB.
                let ret_high = self.stack_pop(memory, &mut cycles) as u16;
                let ret_low = self.stack_pop(memory, &mut cycles) as u16;
                let ret = ret_high << 8 | ret_low;
                self.program_counter = ret;
                cycles -= 3;
            },

            // Branches
            BCS_RELATIVE => {
                self.branch(memory, CpuStatusFlags::CARRY, true, &mut cycles);
            },
            BCC_RELATIVE => {
                self.branch(memory, CpuStatusFlags::CARRY, false, &mut cycles);
            },
            BEQ_RELATIVE => {
                self.branch(memory, CpuStatusFlags::ZERO, true, &mut cycles);
            },
            BNE_RELATIVE => {
                self.branch(memory, CpuStatusFlags::ZERO, false, &mut cycles);
            },
            BMI_RELATIVE => {
                self.branch(memory, CpuStatusFlags::NEGATIVE, true, &mut cycles);
            },
            BPL_RELATIVE => {
                self.branch(memory, CpuStatusFlags::NEGATIVE, false, &mut cycles);
            },
            BVS_RELATIVE => {
                self.branch(memory, CpuStatusFlags::OVERFLOW, true, &mut cycles);
            },
            BVC_RELATIVE => {
                self.branch(memory, CpuStatusFlags::OVERFLOW, false, &mut cycles);
            },

            // Status Flag Changes
            CLC_IMPLIED => {
                self.flags.set(CpuStatusFlags::CARRY, false);
                cycles -= 1;
            },
            CLD_IMPLIED => {
                self.flags.set(CpuStatusFlags::DECIMAL_MODE, false);
                cycles -= 1;
            },
            CLI_IMPLIED => {
                self.flags.set(CpuStatusFlags::IRQ_DISABLE, false);
                cycles -= 1;
            },
            CLV_IMPLIED => {
                self.flags.set(CpuStatusFlags::OVERFLOW, false);
                cycles -= 1;
            },
            SEC_IMPLIED => {
                self.flags.set(CpuStatusFlags::CARRY, true);
                cycles -= 1;
            },
            SED_IMPLIED => {
                self.flags.set(CpuStatusFlags::DECIMAL_MODE, true);
                cycles -= 1;
            },
            SEI_IMPLIED => {
                self.flags.set(CpuStatusFlags::IRQ_DISABLE, true);
                cycles -= 1;
            },

            // System functions
            BRK_IMPLIED => {
                let low_pc = (self.program_counter & 0xFF) as u8;
                let high_pc = (self.program_counter >> 8) as u8;

                self.stack_push(memory, low_pc, &mut cycles);
                self.stack_push(memory, high_pc, &mut cycles);
                self.stack_push(memory, self.flags.bits(), &mut cycles);

                self.program_counter = Self::read_word(memory, IRQ_INTERRUPT_VECTOR, &mut cycles);
                self.flags.set(CpuStatusFlags::BREAK_COMMAND, true);
                cycles -= 1;
            },
            NOP_IMPLIED => {
                cycles -= 1;
            },
            RTI_IMPLIED => {
                let flag_bits = self.stack_pop(memory, &mut cycles);
                self.flags = CpuStatusFlags::from_bits_truncate(flag_bits);

                let high_pc = self.stack_pop(memory, &mut cycles) as u16;
                let low_pc = self.stack_pop(memory, &mut cycles) as u16;
                self.program_counter = high_pc << 8 | low_pc;

                self.flags.set(CpuStatusFlags::BREAK_COMMAND, false);

                cycles -= 2;
            }
            _ => {}
        }

        cycles
    }

    /// Push a value to the stack
    fn stack_push(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, value: u8, cycles: &mut u32) {
        // The stack runs from 0x0100 - 0x01FF
        // But the stack pointer stores only the least significant byte
        Self::write_byte(memory, 0x0100 + (self.stack_pointer as u16), value, cycles);
        self.stack_pointer = (Wrapping(self.stack_pointer) + Wrapping(1)).0;
    }

    /// Pop a value from the stack
    fn stack_pop(&mut self, memory: &dyn Memory<MAX_MEMORY>, cycles: &mut u32) -> u8 {
        // The stack pointer points to the next free byte,
        // Decrement the stack pointer *before* reading it
        self.stack_pointer = (Wrapping(self.stack_pointer) - Wrapping(1)).0;
        // The stack runs from 0x0100 - 0x01FF,
        // the stack pointer represents least significant byte of the address
        let value = Self::read_byte(memory, 0x0100 + (self.stack_pointer as u16), cycles);

        #[cfg(test)]
        debug!("Popped {:#04X} from stack. Stack pointer is now at next free byte {:#04X}", value, self.stack_pointer);

        value
    }

    /// Retrieve a CPU Status flag as a byte.
    /// The value of the flag is stored in the least significant bit,
    /// the other 7 bits will be zeroes.
    fn flag_as_bit(&self, flag: CpuStatusFlags) -> u8 {
        let bits = self.flags.bits();
        match flag {
            CpuStatusFlags::CARRY => bits & 0b0000_0001,
            CpuStatusFlags::ZERO => bits & 0b0000_0010 >> 1,
            CpuStatusFlags::IRQ_DISABLE => bits & 0b0000_0100 >> 2,
            CpuStatusFlags::DECIMAL_MODE => bits & 0b0000_1000 >> 3,
            CpuStatusFlags::BREAK_COMMAND => bits & 0b0001_0000 >> 4,
            CpuStatusFlags::OVERFLOW => bits & 0b0100_0000 >> 6,
            CpuStatusFlags::NEGATIVE => bits & 0b1000_0000 >> 7,
            _ => unreachable!("Unknown CPU status flag")
        }
    }

    /// Branch if the condition is met, i.e. the value of the provided flag is equal to the wanted state.
    /// Takes 1 cycle if the condition is not met. 2 If it is met, or 3 if it is met and the new `program_counter`
    /// is on a new page.
    fn branch(&mut self, memory: &dyn Memory<MAX_MEMORY>, flag: CpuStatusFlags, state: bool, cycles: &mut u32) {
        let rel_addr = self.fetch_byte(memory, cycles);
        let status = self.flags.intersects(flag);

        if status == state {
            *cycles -= 1;

            /*let new_pc = if (rel_addr as i8) < 0 {
                #[cfg(test)]
                debug!("Subtracting {} ({:#010b})", rel_addr as i8, rel_addr);
                self.program_counter as i16 - rel_addr as i8 as i16
            } else {
                self.program_counter as i16 + rel_addr as i16
            } as u16;*/
            let new_pc = (self.program_counter as i16 + (rel_addr as i8 as i16)) as u16;

            #[cfg(test)]
            debug!("Flag {:?} is {}. Branching to {:#06X}", flag, state, new_pc);

            if (new_pc ^ self.program_counter) >> 8 != 0 {
                *cycles -= 1;
            }

            self.program_counter = new_pc;
        }
    }

    /// Rotate bits in the value at the provided address in memory to the left.
    /// New bit 0 is filled with the current value of the `Carry` flag. Old bit 7 is put into the `Carry` flag.
    /// This function affects the `Carry`, `Zero` and `Negative` flags
    fn rotate_left(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let shifted = value << 1 | self.flag_as_bit(CpuStatusFlags::CARRY);

        self.flags.set(CpuStatusFlags::CARRY, value & 0b1000_0000 != 0);
        self.flags.set(CpuStatusFlags::ZERO, shifted == 0);
        self.flags.set(CpuStatusFlags::NEGATIVE, shifted & 0b1000_0000 != 0);

        Self::write_byte(memory, address, shifted, cycles);
        *cycles -= 1;
    }

    /// Rotate bits in the value at the provided address in memory to the right.
    /// New bit 7 is filled with the current value of the `Carry` flag. Old bit 0 is put into the `Carry` flag.
    /// This function affects the `Carry`, `Zero`, and `Negative` flags
    fn rotate_right(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let shifted = value >> 1 | (self.flag_as_bit(CpuStatusFlags::CARRY) << 7);

        self.flags.set(CpuStatusFlags::CARRY, value & 0b0000_0001 != 0);
        self.flags.set(CpuStatusFlags::ZERO, shifted == 0);
        self.flags.set(CpuStatusFlags::NEGATIVE, shifted & 0b1000_0000 != 0);

        Self::write_byte(memory, address, shifted, cycles);
        *cycles -= 1;
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

    /// Decrement a location in memory
    fn decrement_memory(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let dec = (Wrapping(value) - Wrapping(1)).0;
        *cycles -= 1;

        self.flags.set(CpuStatusFlags::ZERO, dec == 0);
        self.flags.set(CpuStatusFlags::NEGATIVE, dec & NEGATIVE_BIT != 0);

        Self::write_byte(memory, address, dec, cycles);
    }

    /// Increment a register
    fn increment_register(&mut self, register: Register, cycles: &mut u32) {
        self.set_register(register.clone(), (Wrapping(self.get_register(register)) + Wrapping(1)).0);
        *cycles -= 1;
    }

    /// Decrement a register
    fn decrement_register(&mut self, register: Register, cycles: &mut u32) {
        self.set_register(register.clone(), (Wrapping(self.get_register(register)) - Wrapping(1)).0);
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
        let a_before = self.register_accumulator;
        let c_before = self.flag_as_bit(CpuStatusFlags::CARRY);

        let sum = a_before as u16 + value as u16 + c_before as u16;

        // Carry flag is set if the higher byte is not zero,
        // E.g. 0b0001_1111 will have a carry, as it is larger than 0xFF (0b1111)
        self.flags.set(CpuStatusFlags::CARRY, sum > 0xFF);

        // Remove the high byte
        // E.g. 0b0001_1111 will become 0b0000_0000 because 0xFF is 0b0000_1111
        // We can then safely cast to an u8
        let a_after = (sum & 0xFF) as u8;

        // Overflow flag indicates that the sign has changed improperly
        // E.g. if you add two positive numbers and get a negative result
        let sign_bits_eq_before = (a_before ^ value) & NEGATIVE_BIT == 0;
        let sign_bits_ne_after = (a_after ^ value) & NEGATIVE_BIT != 0;
        self.flags.set(CpuStatusFlags::OVERFLOW, sign_bits_eq_before & sign_bits_ne_after);

        self.set_register(Register::A, a_after);
    }

    /// Perform an arithmetic shift left on the value at the provided address in memory.
    /// The effect of this function is that the value gets multiplied by 2
    /// This affects the `Carry`, `Zero` and `Negative` flags.
    fn arithmetic_shift_left(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let carry = value & 0b1000_0000 != 0;
        let shifted = value << 1;

        Self::write_byte(memory, address, shifted, cycles);
        self.flags.set(CpuStatusFlags::CARRY, carry);
        self.flags.set(CpuStatusFlags::ZERO, shifted == 0);
        self.flags.set(CpuStatusFlags::NEGATIVE, shifted & NEGATIVE_BIT != 0);

        *cycles -= 1;
    }

    /// Perform a logical shift right on the value at the provided address in memory.
    /// The effects of this function is that the value gets divided by 2.
    /// This affects the `Carry`, `Zero` and `Negative` flags.
    fn logical_shift_right(&mut self, memory: &mut dyn Memory<MAX_MEMORY>, address: u16, cycles: &mut u32) {
        let value = Self::read_byte(memory, address, cycles);
        let carry = value & 0b1 != 0;
        let shifted = value >> 1;

        Self::write_byte(memory, address, shifted, cycles);
        self.flags.set(CpuStatusFlags::CARRY, carry);
        self.flags.set(CpuStatusFlags::ZERO, shifted == 0);
        self.flags.set(CpuStatusFlags::NEGATIVE, false);

        *cycles -= 1;
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
    use core::num::Wrapping;
    use log::LevelFilter;
    use crate::cpu::{Cpu, CpuStatusFlags};
    use crate::{Memory, OperatingMode};
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
        memory.write(0xFFFD, 1);
        cpu.register_accumulator = 127;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        // TODO: Broken
        assert_eq!(cpu.register_accumulator, 128);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::OVERFLOW));
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
    fn inx_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INX_IMPLIED);
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
    fn iny_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, INY_IMPLIED);
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

    #[test]
    fn dec_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, DEC_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x20), 0xF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, DEC_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x00);

        cpu.execute_single(&mut memory, 5);
        assert_eq!(memory.read(0x20), 0xFF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        memory.reset();

        memory.write(0xFFFC, DEC_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0x1);

        cpu.execute_single(&mut memory, 5);
        assert_eq!(memory.read(0x20), 0x00);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn dec_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, DEC_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x20;
        memory.write(0x40, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x40), 0xF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn dec_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, DEC_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        memory.write(0x4020, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4020), 0xF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn dec_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, DEC_ABSOLUTE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        cpu.register_x = 0x20;
        memory.write(0x4040, 0x10);

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4040), 0xF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn dex_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, DEX_IMPLIED);
        cpu.register_x = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_x, 0xF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0x00;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_x, 0xFF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_x = 0x01;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn dey_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, DEY_IMPLIED);
        cpu.register_y = 0x10;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_y, 0xF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0x00;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_y, 0xFF);
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));

        cpu.reset();
        cpu.register_y = 0x01;

        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn asl_accumulator() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ASL_ACCUMULATOR);
        cpu.register_accumulator = 0b1010_1010;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0101_0100);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn asl_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ASL_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b1010_1010);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x20), 0b0101_0100);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn asl_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ASL_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b1010_1010);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x30), 0b0101_0100);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn asl_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ASL_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        memory.write(0x4020, 0b1010_1010);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4020), 0b0101_0100);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn asl_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ASL_ABSOLUTE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        cpu.register_x = 0x10;
        memory.write(0x4030, 0b1010_1010);

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4030), 0b0101_0100);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lsr_accumulator() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LSR_ACCUMULATOR);
        cpu.register_accumulator = 0b0101_0101;

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0010_1010);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lsr_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LSR_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b0101_0101);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x20), 0b0010_1010);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lsr_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LSR_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b0101_0101);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x30), 0b0010_1010);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lsr_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LSR_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        memory.write(0x4020, 0b0101_0101);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4020), 0b0010_1010);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn lsr_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, LSR_ABSOLUTE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        cpu.register_x = 0x10;
        memory.write(0x4030, 0b0101_0101);

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x4030), 0b0010_1010);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
    }

    #[test]
    fn rol_accumulator() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROL_ACCUMULATOR);
        cpu.register_accumulator = 0b1010_1010;
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b0101_0101);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn rol_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROL_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x20), 0b0101_0101);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn rol_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROL_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x30), 0b0101_0101);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn rol_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROL_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x80); // 0x8020
        memory.write(0x8020, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x8020), 0b0101_0101);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn rol_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROL_ABSOLUTE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x80); // 0x8020
        cpu.register_x = 0x10;
        memory.write(0x8030, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x8030), 0b0101_0101);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(!cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn ror_accumulator() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROR_ACCUMULATOR);
        cpu.register_accumulator = 0b1010_1010;
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.register_accumulator, 0b11010101);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn ror_zero_page() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROR_ZERO_PAGE);
        memory.write(0xFFFD, 0x20);
        memory.write(0x20, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x20), 0b11010101);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn ror_zero_page_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROR_ZERO_PAGE_X);
        memory.write(0xFFFD, 0x20);
        cpu.register_x = 0x10;
        memory.write(0x30, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x30), 0b11010101);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn ror_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROR_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x80); // 0x8020
        memory.write(0x8020, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x8020), 0b11010101);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn ror_absolute_x() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, ROR_ABSOLUTE_X);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x80); // 0x8020
        cpu.register_x = 0x10;
        memory.write(0x8030, 0b1010_1010);
        cpu.flags.set(CpuStatusFlags::CARRY, true);

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(memory.read(0x8030), 0b11010101);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::ZERO));
    }

    #[test]
    fn jmp_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, JMP_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x4020);
    }

    #[test]
    fn jmp_indirect_mos() {
        init();
        let mut cpu = Cpu::with_mode(OperatingMode::Mos);
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, JMP_INDIRECT);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40);
        // We'd expect 0x4020, however due to a bug in older 6502's,
        // the least significant byte will be fetched from 0x4020, as normal
        // but the most significant byte will be fetched from 0x4000, rather than 0x4021
        memory.write(0x4020, 0x60);
        memory.write(0x4000, 0x70); // 0x7060

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x7060);
    }

    #[test]
    fn jmp_indirect_wdc() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, JMP_INDIRECT);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020
        memory.write(0x4020, 0x60);
        memory.write(0x4021, 0x70); // 0x7060

        let cycles_left = cpu.execute_single(&mut memory, 5);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x7060);
    }

    #[test]
    fn jsr_absolute() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, JSR_ABSOLUTE);
        memory.write(0xFFFD, 0x20);
        memory.write(0xFFFE, 0x40); // 0x4020

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x4020);
        // LSB of return address
        assert_eq!(memory.read(0x0100 + (Wrapping(cpu.stack_pointer) - Wrapping(2)).0 as u16), 0xFF);
        // MSB of return address
        assert_eq!(memory.read(0x0100 + ((Wrapping(cpu.stack_pointer) - Wrapping(1)).0 as u16)), 0xFF);
    }

    #[test]
    fn rts_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, RTS_IMPLIED);

        // 0x0140 is the next free byte,
        // i.e the return address is at 0x013F and 0x013E
        cpu.stack_pointer = 0x40;
        memory.write(0x013E, 0x20);
        memory.write(0x013F, 0x40); // 0x4020

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x4020);
        assert_eq!(cpu.stack_pointer, 0x3E);
    }

    #[test]
    fn full_jump() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.program_counter = 0x8000;
        memory.write(0x8000, JSR_ABSOLUTE);
        memory.write(0x8001, 0x40);
        memory.write(0x8002, 0x20); // 0x2040

        // The subroutine
        memory.write(0x2040, LDA_IMMEDIATE);
        memory.write(0x2041, 0x32);
        memory.write(0x2042, RTS_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.program_counter, 0x8003);
        assert_eq!(cpu.register_accumulator, 0x32);

        // Check if the next instruction will execute fine
        memory.write(0x8003, LDX_IMMEDIATE);
        memory.write(0x8004, 0x64);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x8005);
        assert_eq!(cpu.register_x, 0x64);
    }

    #[test]
    fn bcc_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::CARRY, false);
        memory.write(0xFFFC, BCC_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::CARRY, true);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn bcs_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::CARRY, true);
        memory.write(0xFFFC, BCS_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::CARRY, false);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn beq_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::ZERO, true);
        memory.write(0xFFFC, BEQ_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::ZERO, false);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn bne_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::ZERO, false);
        memory.write(0xFFFC, BNE_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::ZERO, true);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn bmi_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::NEGATIVE, true);
        memory.write(0xFFFC, BMI_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::NEGATIVE, false);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn bpl_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::NEGATIVE, false);
        memory.write(0xFFFC, BPL_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::NEGATIVE, true);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn bvs_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::OVERFLOW, true);
        memory.write(0xFFFC, BVS_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::OVERFLOW, false);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn bvc_relative() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::OVERFLOW, false);
        memory.write(0xFFFC, BVC_RELATIVE);
        memory.write(0xFFFD, -10_i8 as u8);

        memory.write(0xFFF4, LDA_IMMEDIATE);
        memory.write(0xFFF5, 0x32);

        let cycles_left = cpu.execute_single(&mut memory, 3);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFF4);
        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);

        assert_eq!(cpu.register_accumulator, 0x32);

        cpu.reset();
        cpu.flags.set(CpuStatusFlags::OVERFLOW, true);
        cpu.execute_single(&mut memory, 2);
        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn clc_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::CARRY, true);
        memory.write(0xFFFC, CLC_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::CARRY));
    }

    #[test]
    fn cld_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::DECIMAL_MODE, true);
        memory.write(0xFFFC, CLD_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::DECIMAL_MODE));
    }

    #[test]
    fn cli_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::IRQ_DISABLE, true);
        memory.write(0xFFFC, CLI_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::IRQ_DISABLE));
    }

    #[test]
    fn clv_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        cpu.flags.set(CpuStatusFlags::OVERFLOW, true);
        memory.write(0xFFFC, CLV_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(!cpu.flags.intersects(CpuStatusFlags::OVERFLOW));
    }

    #[test]
    fn sec_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, SEC_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
    }

    #[test]
    fn sed_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, SED_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::DECIMAL_MODE));
    }

    #[test]
    fn sei_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, SEI_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::IRQ_DISABLE));
    }

    #[test]
    fn brk_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, BRK_IMPLIED);
        memory.write(0xFFFE, 0x20);
        memory.write(0xFFFF, 0x40); // 0x4020;

        let cycles_left = cpu.execute_single(&mut memory, 7);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0x4020);
        assert!(cpu.flags.intersects(CpuStatusFlags::BREAK_COMMAND));
    }

    #[test]
    fn nop_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, NOP_IMPLIED);

        let cycles_left = cpu.execute_single(&mut memory, 2);
        assert_eq!(cycles_left, 0);
        assert_eq!(cpu.program_counter, 0xFFFD);
    }

    #[test]
    fn rti_implied() {
        init();
        let mut cpu = Cpu::default();
        let mut memory = BasicMemory::default();

        memory.write(0xFFFC, RTI_IMPLIED);
        cpu.stack_pointer = 0x20;

        memory.write(0x011F, CpuStatusFlags::all().bits());
        memory.write(0x011E, 0x30); // PC high byte
        memory.write(0x011D, 0x40); // PC low byte, 0x3040

        let cycles_left = cpu.execute_single(&mut memory, 6);
        assert_eq!(cycles_left, 0);
        assert!(cpu.flags.intersects(CpuStatusFlags::ZERO));
        assert!(cpu.flags.intersects(CpuStatusFlags::CARRY));
        assert!(cpu.flags.intersects(CpuStatusFlags::OVERFLOW));
        assert!(cpu.flags.intersects(CpuStatusFlags::NEGATIVE));
        assert!(cpu.flags.intersects(CpuStatusFlags::IRQ_DISABLE));
        assert!(cpu.flags.intersects(CpuStatusFlags::DECIMAL_MODE));
        assert!(!cpu.flags.intersects(CpuStatusFlags::BREAK_COMMAND));
        assert_eq!(cpu.program_counter, 0x3040);
    }
}