#[cfg(test)]
use log::debug;

pub const MAX_MEMORY: usize = 1024 * 64;

pub trait Memory<const N: usize> {
    fn reset(&mut self);
    fn write(&mut self, address: u16, value: u8);
    fn read(&self, address: u16) -> u8;
}

pub struct BasicMemory {
    data: [u8; MAX_MEMORY],
}


impl Default for BasicMemory {
    fn default() -> Self {
        Self {
            data: [0u8; MAX_MEMORY],
        }
    }
}

impl From<&[u8]> for BasicMemory {
    fn from(i: &[u8]) -> Self {
        Self { data: i.try_into().expect("Invalid length") }
    }
}

impl Memory<MAX_MEMORY> for BasicMemory {
    /// Reset the memory
    fn reset(&mut self) {
        #[cfg(test)]
        debug!("Resetting memory");

        *self = Self::default();
    }

    /// Write a byte to memory
    ///
    /// # Panics
    ///
    /// if the provided address exceeds `MAX_MEMORY`
    fn write(&mut self, address: u16, value: u8) {
        if address as usize > MAX_MEMORY {
            panic!("Tried to access memory address outside of memory size");
        }

        self.data[address as usize] = value;
    }

    /// Read a byte from memory
    ///
    /// # Panics
    ///
    /// If the provided address exceeds `MAX_MEMORY`
    fn read(&self, address: u16) -> u8 {
        if address as usize > MAX_MEMORY {
            panic!("Tried to access memory address outside of memory size");
        }
        self.data[address as usize]
    }
}