use crate::cpu::Cpu;
use crate::memory::Memory;

mod cpu;
mod memory;
mod ops;


fn main() {

    let mut cpu = Cpu::default();
    let mut memory = Memory::default();

    memory.write(0xFFFC, 0xA9);
    memory.write(0xFFFD, 0x42);

    cpu.execute(&mut memory, 2);
}
