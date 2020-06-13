extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::test_run;

fn test(carry: u8, instruction: u8, value: u8, expected: u8, carry_set: bool) {
    let core = test_run(vec![
        carry,                     // Set/clear carry.
        0xa9u8, value as u8,       // Load 'value' into accumulator.
        instruction                // Shift on accumulator
    ]);

    assert_eq!(core.registers.accumulator, expected, "Expected {:b}, found {:b}.", expected, core.registers.accumulator);
    assert_eq!(core.registers.get_flag(Flag::Zero), expected == 0, "Expectation for Zero flag failed.");
    assert_eq!(core.registers.get_flag(Flag::Negative), (expected as i8) < 0, "Expectation for Negative flag failed.");
    assert_eq!(core.registers.get_flag(Flag::Carry), carry_set, "Expectation for Carry flag failed.")
}

#[test]
fn test_asl() {
    test(0x18u8, 0x0au8, 0b10001111, 0b00011110, true)
}

#[test]
fn test_lsr() {
    test(0x18u8, 0x4au8, 0b10001111, 0b01000111, true)
}

#[test]
fn test_rol_carry_cleared() {
    test(0x18u8, 0x2au8, 0b10001111, 0b00011110, true)
}

#[test]
fn test_rol_carry_set() {
    test(0x38u8, 0x2au8, 0b10001111, 0b00011111, true)
}

#[test]
fn test_ror_carry_cleared() {
    test(0x18u8, 0x6au8, 0b10001111, 0b01000111, true)
}

#[test]
fn test_ror_carry_set() {
    test(0x38u8, 0x6au8, 0b10001111, 0b11000111, true)
}
