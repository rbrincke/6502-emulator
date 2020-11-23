extern crate nes;

use nes::emulator::Emulator;
use nes::memory::basic::DefaultMemory;

#[test]
fn it() {
    let program = include_bytes!("it.bin").to_vec();

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
