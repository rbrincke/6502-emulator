extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::test_run;

fn test_adc(first: i8, second: i8, expected: i8, is_carry: bool, is_overflow: bool) {
    test_adc_sbc(0x18u8, 0x69u8, first, second, expected, is_carry, is_overflow);
}

fn test_sbc(first: i8, second: i8, expected: i8, is_carry: bool, is_overflow: bool) {
    test_adc_sbc(0x38u8, 0xe9u8, first, second, expected, is_carry, is_overflow);
}

fn test_adc_sbc(carry: u8, instruction: u8, first: i8, second: i8, expected: i8, is_carry: bool, is_overflow: bool) {
    let core = test_run(vec![
        carry,                     // Set/clear carry.
        0xa9u8, first as u8,       // Load 'first' into accumulator.
        instruction, second as u8  // Add/subtract 'second'
    ]);

    assert_eq!(core.registers.accumulator as i8, expected as i8);
    assert_eq!(core.registers.get_flag(Flag::Carry), is_carry);
    assert_eq!(core.registers.get_flag(Flag::Overflow), is_overflow);
    assert_eq!(core.registers.get_flag(Flag::Zero), expected == 0);
    assert_eq!(core.registers.get_flag(Flag::Negative), expected < 0);
}

#[test]
fn test_adc_positive() {
    test_adc(3, 5, 8, false, false);
}

#[test]
fn test_adc_negative() {
    test_adc(-3, -5, -8, true, false);
}

#[test]
fn test_adc_mixed() {
    test_adc(-3, 5, 2, true, false);
}

#[test]
fn test_adc_overflow() {
    test_adc(127, 1, -128, false, true);
}

#[test]
fn test_sbc_positive() {
    test_sbc(3, 5, -2, false, false);
}

#[test]
fn test_sbc_negative() {
    test_sbc(-3, -5, 2, true, false);
}

#[test]
fn test_sbc_mixed() {
    test_sbc(-3, 5, -8, true, false);
}

#[test]
fn test_sbc_overflow() {
    test_sbc(-128, 1, 127, true, true);
}

#[test]
fn test_multi_byte() {
    // Add 396 to itself by splitting into 2 bytes, perform add.
    let core = test_run(vec![
        0xa9u8, 0b10001100 as u8, // Load into accumulator.
        0x69u8, 0b10001100 as u8, // Add.
        0xaau8,                   // Store accumulator in X.
        0xa9u8, 0b00000001 as u8, // Load into accumulator.
        0x69u8, 0b00000001 as u8  // Add.
    ]);

    let r = ((core.registers.accumulator as u16) << 8) | (core.registers.x as u16);
    assert_eq!(r, 792)
}
