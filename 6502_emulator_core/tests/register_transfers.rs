use emulator_6502_core::{BasicMemory, Cpu, Memory};
use crate::common::init;

mod common;

#[test]
fn register_transfers() {
    init();

    let (bin, _f) = common::assemble_file("./register_transfers.s");
    let mut memory = BasicMemory::from(bin.as_slice());
    let mut cpu = Cpu::default();

    cpu.execute_instructions(&mut memory, 11);
    assert_eq!(memory.read(0x8000), 0x32);
    assert_eq!(memory.read(0x8001), 0x64);
    assert_eq!(memory.read(0x8002), 0x10);
}