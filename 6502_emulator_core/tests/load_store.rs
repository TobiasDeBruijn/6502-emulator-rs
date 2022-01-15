use emulator_6502_core::{BasicMemory, Cpu, Memory};
use crate::common::init;

mod common;

#[test]
fn load_store() {
    init();

    let (bin, _f) = common::assemble_file("./load_store.s");
    let mut memory = BasicMemory::from(bin.as_slice());
    let mut cpu = Cpu::default();

    cpu.execute_instructions(&mut memory, 9);
    assert_eq!(memory.read(0x2000), 0x32);
    assert_eq!(memory.read(0x2032), 0x32);
    assert_eq!(memory.read(0x2001), 0x32);
    assert_eq!(memory.read(0x2002), 0x32);
}