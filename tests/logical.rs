extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::test_run;

fn test(instruction: u8, first: u8, second: u8, expected: u8) {
    let core = test_run(vec![
        0xa9u8, first as u8,       // Load 'first' into accumulator.
        instruction, second as u8  // Add/subtract 'second'
    ]);

    assert_eq!(core.registers.accumulator, expected);
    assert_eq!(core.registers.get_flag(Flag::Zero), expected == 0);
    assert_eq!(core.registers.get_flag(Flag::Negative), (expected as i8) < 0);
}

#[test]
fn test_and() {
    test(0x29u8, 0b10101010, 0b01010101, 0b00000000)
}

#[test]
fn test_eor() {
    test(0x49u8, 0b10101010, 0b01010101, 0b11111111)
}

#[test]
fn test_ora() {
    test(0x09u8, 0b11000011, 0b01000101, 0b11000111)
}
