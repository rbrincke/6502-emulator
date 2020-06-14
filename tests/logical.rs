extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Zero, Negative};

#[test]
fn test_and() {
    let core = test(vec![
        0xa9u8, 0b10101010 as u8,  // Load 'first' into accumulator.
        0x29u8, 0b01010101 as u8  // AND
    ]);

    core.assert_flags_set(vec![Zero]);
    assert_eq!(core.registers.accumulator, 0b00000000);
}

#[test]
fn test_eor() {
    let core = test(vec![
        0xa9u8, 0b10101010 as u8,  // Load 'first' into accumulator.
        0x49u8, 0b01010101 as u8   // EOR
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator, 0b11111111);
}

#[test]
fn test_ora() {
    let core = test(vec![
        0xa9u8, 0b11000011 as u8,  // Load 'first' into accumulator.
        0x09u8, 0b01000101 as u8   // ORA
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator, 0b11000111);
}
