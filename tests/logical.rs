extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Zero, Negative};
use nes::processor::instructions::opcodes::*;

#[test]
fn test_and() {
    let core = test(vec![
        LDA::immediate(0b10101010),
        AND::immediate(0b01010101)
    ]);

    core.assert_flags_set(vec![Zero]);
    assert_eq!(core.registers.accumulator, 0b00000000);
}

#[test]
fn test_eor() {
    let core = test(vec![
        LDA::immediate(0b10101010),
        EOR::immediate(0b01010101)
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator, 0b11111111);
}

#[test]
fn test_ora() {
    let core = test(vec![
        LDA::immediate(0b11000011),
        ORA::immediate(0b01000101)
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator, 0b11000111);
}
