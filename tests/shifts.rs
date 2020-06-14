extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Carry, Negative};

#[test]
fn test_asl() {
    let core = test(vec![
        0x18u8,                    // Clear carry.
        0xa9u8, 0b10001111 as u8,  // Load 'value' into accumulator.
        0x0au8                     // ASL
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b00011110);
}

#[test]
fn test_lsr() {
    let core = test(vec![
        0x18u8,                    // Clear carry.
        0xa9u8, 0b10001111 as u8,       // Load 'value' into accumulator.
        0x4au8                     // LSR
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b01000111);
}

#[test]
fn test_rol_carry_cleared() {
    let core = test(vec![
        0x18u8,                    // Clear carry.
        0xa9u8, 0b10001111 as u8,  // Load 'value' into accumulator.
        0x2au8                     // ROL
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b00011110);
}

#[test]
fn test_rol_carry_set() {
    let core = test(vec![
        0x38u8,                    // Set carry.
        0xa9u8, 0b10001111 as u8,  // Load 'value' into accumulator.
        0x2au8                     // ROL
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b00011111);
}

#[test]
fn test_ror_carry_cleared() {
    let core = test(vec![
        0x18u8,                    // Clear carry.
        0xa9u8, 0b10001111 as u8,  // Load 'value' into accumulator.
        0x6au8                     // ROR
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b01000111);
}

#[test]
fn test_ror_carry_set() {
    let core = test(vec![
        0x38u8,                    // Set carry.
        0xa9u8, 0b10001111 as u8,  // Load 'value' into accumulator.
        0x6au8                     // ROR
    ]);

    core.assert_flags_set(vec![Carry, Negative]);
    assert_eq!(core.registers.accumulator, 0b11000111);
}
