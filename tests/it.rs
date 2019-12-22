extern crate nes;

use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use nes::memory::basic::DefaultMemory;
use nes::emulator::Emulator;
use nes::emulator::registers::Flag;

#[test]
fn it() {
    let mut program = include_bytes!("it.bin").to_vec();

    let one = program.get(0xFFFC).unwrap();
    let two = program.get(0xFFFD).unwrap();

    let mut memory = DefaultMemory::empty();
    memory.load(program, 0);
    memory.set_program_counter(0x400);

    let mut emulator = Emulator::new(
        memory
    );

    let mut program_counter = 0x0u16;
    while program_counter != emulator.registers.program_counter {
        program_counter = emulator.registers.program_counter;
        emulator.execute_next();
    }

    assert_eq!(0x3469, program_counter)
}
