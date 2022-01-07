pub const MAX_MEMORY: usize = 1024 * 64;

pub struct Memory {
    data: [u8; MAX_MEMORY],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: [0u8; MAX_MEMORY],
        }
    }
}

impl Memory {
    /// Reset the memory
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Write a byte to memory
    ///
    /// # Panics
    ///
    /// if the provided address exceeds `MAX_MEMORY`
    pub fn write(&mut self, address: u16, value: u8) {
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
    pub fn fetch(&self, address: u16) -> u8 {
        if address as usize > MAX_MEMORY {
            panic!("Tried to access memory address outside of memory size");
        }
        self.data[address as usize]
    }
}