extern crate emulator;

use emulator::emulator::Emulator;
use emulator::memory::default::DefaultMemory;

#[test]
fn functional() {
    let program = include_bytes!("functional.bin").to_vec();

    let mut memory = DefaultMemory::empty();
    memory.load(program, 0);
    memory.set_program_counter(0x400);

    let mut emulator = Emulator::new(memory);

    let mut program_counter = 0x0u16;
    while program_counter != emulator.registers.program_counter {
        program_counter = emulator.registers.program_counter;
        emulator.execute_next();
    }

    assert_eq!(0x3469, program_counter)
}
