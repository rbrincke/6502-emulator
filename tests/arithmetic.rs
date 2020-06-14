extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Negative, Carry, Zero, Overflow};

#[test]
fn test_adc_positive() {
    let core = test(vec![
        0x18u8,           // Clear carry.
        0xa9u8, 3 as u8,  // Load into accumulator.
        0x69u8, 5 as u8   // Add/subtract 'second'
    ]);

    core.assert_flags_set(vec![]);
    assert_eq!(core.registers.accumulator as i8, 8);
}

#[test]
fn test_adc_negative() {
    let core = test(vec![
        0x18u8,            // Clear carry.
        0xa9u8, -3i8 as u8,  // Load into accumulator.
        0x69u8, -5i8 as u8   // Add 'second'
    ]);

    core.assert_flags_set(vec![Carry, Negative]);
    assert_eq!(core.registers.accumulator as i8, -8);
}

#[test]
fn test_adc_mixed() {
    let core = test(vec![
        0x18u8,            // Clear carry.
        0xa9u8, -3i8 as u8,  // Load into accumulator.
        0x69u8, 5 as u8    // Add 'second'
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator as i8, 2);
}

#[test]
fn test_adc_overflow() {
    let core = test(vec![
        0x18u8,             // Clear carry.
        0xa9u8, 127 as u8,  // Load into accumulator.
        0x69u8, 1 as u8     // Add 'second'
    ]);

    core.assert_flags_set(vec![Overflow, Negative]);
    assert_eq!(core.registers.accumulator as i8, -128);
}

#[test]
fn test_sbc_positive() {
    let core = test(vec![
        0x38u8,           // Set carry.
        0xa9u8, 3 as u8,  // Load into accumulator.
        0xe9u8, 5 as u8   // Add/subtract 'second'
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator as i8, -2);
}

#[test]
fn test_sbc_negative() {
    let core = test(vec![
        0x38u8,            // Set carry.
        0xa9u8, -3i8 as u8,  // Load into accumulator.
        0xe9u8, -5i8 as u8   // Add/subtract 'second'
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator as i8, 2);
}

#[test]
fn test_sbc_mixed() {
    let core = test(vec![
        0x38u8,            // Set carry.
        0xa9u8, -3i8 as u8,  // Load into accumulator.
        0xe9u8, 5 as u8    // Add/subtract 'second'
    ]);

    core.assert_flags_set(vec![Carry, Negative]);
    assert_eq!(core.registers.accumulator as i8, -8);
}

#[test]
fn test_sbc_overflow() {
    let core = test(vec![
        0x38u8,              // Set carry.
        0xa9u8, -128i8 as u8,  // Load into accumulator.
        0xe9u8, 1 as u8      // Add/subtract 'second'
    ]);

    core.assert_flags_set(vec![Carry, Overflow]);
    assert_eq!(core.registers.accumulator as i8, 127);
}

#[test]
fn test_multi_byte() {
    // Add 396 to itself by splitting into 2 bytes, perform add.
    let core = test(vec![
        0xa9u8, 0b10001100 as u8, // Load into accumulator.
        0x69u8, 0b10001100 as u8, // Add.
        0xaau8,                   // Store accumulator in X.
        0xa9u8, 0b00000001 as u8, // Load into accumulator.
        0x69u8, 0b00000001 as u8  // Add.
    ]);

    let r = ((core.registers.accumulator as u16) << 8) | (core.registers.x as u16);
    assert_eq!(r, 792)
}

#[test]
fn test_cmp_1() {
    test(vec![
        0xa9u8, 0xau8,   // Load into accumulator.
        0xc9u8, 0xbu8   // Compare accumulator.
    ]).assert_flags_set(vec![Negative])
}

#[test]
fn test_cmp_2() {
    test(vec![
        0xa9u8, 0xbu8,   // Load into accumulator.
        0xc9u8, 0xau8   // Compare accumulator.
    ]).assert_flags_set(vec![Carry])
}

#[test]
fn test_cmp_3() {
    test(vec![
        0xa9u8, 0xbu8,   // Load into accumulator.
        0xc9u8, 0xbu8   // Compare accumulator.
    ]).assert_flags_set(vec![Carry, Zero])
}

#[test]
fn test_cpx_1() {
    test(vec![
        0xa2u8, 0xau8,  // Load into X.
        0xe0u8, 0xbu8   // Compare X.
    ]).assert_flags_set(vec![Negative]);
}

#[test]
fn test_cpx_2() {
    test(
        vec![
            0xa2u8, 0xbu8,  // Load into X.
            0xe0u8, 0xau8   // Compare X.
        ]
    ).assert_flags_set(vec![Carry]);
}

#[test]
fn test_cpx_3() {
    test(
        vec![
            0xa2u8, 0xbu8,  // Load into X.
            0xe0u8, 0xbu8   // Compare X.
        ]
    ).assert_flags_set(vec![Carry, Zero]);
}

#[test]
fn test_cpy_1() {
    test(
        vec![
            0xa0u8, 0xau8,  // Load into Y.
            0xc0u8, 0xbu8   // Compare Y.
        ]
    ).assert_flags_set(vec![Negative]);
}

#[test]
fn test_cpy_2() {
    test(
        vec![
            0xa0u8, 0xbu8,  // Load into Y.
            0xc0u8, 0xau8   // Compare Y.
        ]
    ).assert_flags_set(vec![Carry]);
}

#[test]
fn test_cpy_3() {
    test(
        vec![
            0xa0u8, 0xbu8,  // Load into Y.
            0xc0u8, 0xbu8   // Compare Y.
        ]
    ).assert_flags_set(vec![Carry, Zero]);
}
