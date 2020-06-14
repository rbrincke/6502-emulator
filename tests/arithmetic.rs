extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Negative, Carry, Zero, Overflow};
use nes::processor::instructions::opcodes::*;

#[test]
fn test_adc_positive() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(3),
        ADC::immediate(5)
    ]);

    core.assert_flags_set(vec![]);
    assert_eq!(core.registers.accumulator as i8, 8);
}

#[test]
fn test_adc_negative() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(-3i8 as u8),
        ADC::immediate(-5i8 as u8)
    ]);

    core.assert_flags_set(vec![Carry, Negative]);
    assert_eq!(core.registers.accumulator as i8, -8);
}

#[test]
fn test_adc_mixed() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(-3i8 as u8),
        ADC::immediate(5)
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator as i8, 2);
}

#[test]
fn test_adc_overflow() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(127),
        ADC::immediate(1)
    ]);

    core.assert_flags_set(vec![Overflow, Negative]);
    assert_eq!(core.registers.accumulator as i8, -128);
}

#[test]
fn test_sbc_positive() {
    let core = test(vec![
        SEC::implied(),
        LDA::immediate(3),
        SBC::immediate(5)
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator as i8, -2);
}

#[test]
fn test_sbc_negative() {
    let core = test(vec![
        SEC::implied(),
        LDA::immediate(-3i8 as u8),
        SBC::immediate(-5i8 as u8)
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator as i8, 2);
}

#[test]
fn test_sbc_mixed() {
    let core = test(vec![
        SEC::implied(),
        LDA::immediate(-3i8 as u8),
        SBC::immediate(5)
    ]);

    core.assert_flags_set(vec![Carry, Negative]);
    assert_eq!(core.registers.accumulator as i8, -8);
}

#[test]
fn test_sbc_overflow() {
    let core = test(vec![
        SEC::implied(),
        LDA::immediate(-128i8 as u8),
        SBC::immediate(1)
    ]);

    core.assert_flags_set(vec![Carry, Overflow]);
    assert_eq!(core.registers.accumulator as i8, 127);
}

#[test]
fn test_multi_byte() {
    // Add 396 to itself by splitting into 2 bytes, perform add.
    let core = test(vec![
        LDA::immediate(0b10001100),
        ADC::immediate(0b10001100),
        TAX::implied(),
        LDA::immediate(0b00000001),
        ADC::immediate(0b00000001)
    ]);

    let r = ((core.registers.accumulator as u16) << 8) | (core.registers.x as u16);
    assert_eq!(r, 792)
}

#[test]
fn test_cmp_1() {
    test(vec![
        LDA::immediate(0xau8),
        CMP::immediate(0xbu8)
    ]).assert_flags_set(vec![Negative])
}

#[test]
fn test_cmp_2() {
    test(vec![
        LDA::immediate(0xbu8),
        CMP::immediate(0xau8)
    ]).assert_flags_set(vec![Carry])
}

#[test]
fn test_cmp_3() {
    test(vec![
        LDA::immediate(0xbu8),
        CMP::immediate(0xbu8)
    ]).assert_flags_set(vec![Carry, Zero])
}

#[test]
fn test_cpx_1() {
    test(vec![
        LDX::immediate(0xau8),
        CPX::immediate(0xbu8)
    ]).assert_flags_set(vec![Negative]);
}

#[test]
fn test_cpx_2() {
    test(
        vec![
            LDX::immediate(0xbu8),
            CPX::immediate(0xau8)
        ]
    ).assert_flags_set(vec![Carry]);
}

#[test]
fn test_cpx_3() {
    test(
        vec![
            LDX::immediate(0xbu8),
            CPX::immediate(0xbu8)
        ]
    ).assert_flags_set(vec![Carry, Zero]);
}

#[test]
fn test_cpy_1() {
    test(
        vec![
            LDY::immediate(0xau8),
            CPY::immediate(0xbu8)
        ]
    ).assert_flags_set(vec![Negative]);
}

#[test]
fn test_cpy_2() {
    test(
        vec![
            LDY::immediate(0xbu8),
            CPY::immediate(0xau8)
        ]
    ).assert_flags_set(vec![Carry]);
}

#[test]
fn test_cpy_3() {
    test(
        vec![
            LDY::immediate(0xbu8),
            CPY::immediate(0xbu8)
        ]
    ).assert_flags_set(vec![Carry, Zero]);
}
