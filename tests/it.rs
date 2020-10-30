extern crate nes;

mod common;

use std::fs::File;
use std::io::Read;
use nes::processor::Core;
use common::test;
use common::cartridge::TestCartridge;
use nes::processor::registers::Flag;

#[ignore]
#[test]
fn it() {
    let mut program = include_bytes!("it.bin").to_vec();

    let one = program.get(0xFFFC).unwrap();
    let two = program.get(0xFFFD).unwrap();

    println!("{:#02x} {:#02x}", one, two);

    let mut core = Core::new(
        TestCartridge::complete(program)
    );

    while !core.registers.get_flag(Flag::Break) {
        core.execute_next();
    }
}
