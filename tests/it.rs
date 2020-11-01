extern crate nes;

mod common;

use std::fs::File;
use std::io::Read;
use nes::processor::Core;
use common::test;
use nes::processor::registers::Flag;
use nes::cartridge::TestCartridge;

#[test]
fn it() {
    let mut program = include_bytes!("it.bin").to_vec();

    let one = program.get(0xFFFC).unwrap();
    let two = program.get(0xFFFD).unwrap();

    println!("{:#02x} {:#02x}", one, two);

    let mut cartridge = TestCartridge::new(program, 0);
    cartridge.set_pc(0x400);

    let mut core = Core::new(
        cartridge
    );

    let mut current_pc = 0x0u16;
    while !core.registers.get_flag(Flag::Break) && current_pc != core.registers.program_counter {
        current_pc = core.registers.program_counter;
        core.execute_next();
    }

    println!("IT finished at {}", current_pc);
}
