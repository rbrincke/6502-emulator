extern crate nes;

mod common;

use std::fs::File;
use std::io::Read;
use nes::processor::Core;
use common::test;
use nes::processor::registers::Flag;
use nes::cartridge::basic::BasicCartridge;

#[test]
fn it() {
    let mut program = include_bytes!("it.bin").to_vec();

    let one = program.get(0xFFFC).unwrap();
    let two = program.get(0xFFFD).unwrap();

    let mut cartridge = BasicCartridge::empty();
    cartridge.load(program, 0);
    cartridge.set_program_counter(0x400);

    let mut core = Core::new(
        cartridge
    );

    let mut current_pc = 0x0u16;
    while current_pc != core.registers.program_counter {
        current_pc = core.registers.program_counter;
        core.execute_next();
    }

    println!("IT finished at {:x}", current_pc);
}
