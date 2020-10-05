extern crate nes;

use nes::processor::instructions::opcodes::*;
use nes::processor::registers::Flag::{Zero, Negative};

use crate::common::{test, TestAssertions};

mod common;

#[test]
fn test_tax_zero() {
    let core = test(vec![
        LDA::immediate(0b00000000),
        TAX::implied()
    ]);

    core.assert_flags_set(vec![Zero]);
    assert_eq!(core.registers.x, 0b00000000);
}

#[test]
fn test_tax_positive() {
    let core = test(vec![
        LDA::immediate(0b00111000),
        TAX::implied()
    ]);

    core.assert_flags_set(vec![]);
    assert_eq!(core.registers.x, 0b00111000);
}

#[test]
fn test_tax_negative() {
    let core = test(vec![
        LDA::immediate(0b10001000),
        TAX::implied()
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.x, 0b10001000);
}

#[test]
fn test_tay_zero() {
    let core = test(vec![
        LDA::immediate(0b00000000),
        TAY::implied()
    ]);

    core.assert_flags_set(vec![Zero]);
    assert_eq!(core.registers.y, 0b00000000);
}

#[test]
fn test_tay_positive() {
    let core = test(vec![
        LDA::immediate(0b00111000),
        TAY::implied()
    ]);

    core.assert_flags_set(vec![]);
    assert_eq!(core.registers.y, 0b00111000);
}

#[test]
fn test_tay_negative() {
    let core = test(vec![
        LDA::immediate(0b10001000),
        TAY::implied()
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.y, 0b10001000);
}

#[test]
fn test_txa_zero() {
    let core = test(vec![
        LDX::immediate(0b00000000),
        TXA::implied()
    ]);

    core.assert_flags_set(vec![Zero]);
    assert_eq!(core.registers.accumulator, 0b00000000);
}

#[test]
fn test_txa_positive() {
    let core = test(vec![
        LDX::immediate(0b00111000),
        TXA::implied()
    ]);

    core.assert_flags_set(vec![]);
    assert_eq!(core.registers.accumulator, 0b00111000);
}

#[test]
fn test_txa_negative() {
    let core = test(vec![
        LDX::immediate(0b10001000),
        TXA::implied()
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator, 0b10001000);
}

#[test]
fn test_tya_zero() {
    let core = test(vec![
        LDY::immediate(0b00000000),
        TYA::implied()
    ]);

    core.assert_flags_set(vec![Zero]);
    assert_eq!(core.registers.accumulator, 0b00000000);
}

#[test]
fn test_tya_positive() {
    let core = test(vec![
        LDY::immediate(0b00111000),
        TYA::implied()
    ]);

    core.assert_flags_set(vec![]);
    assert_eq!(core.registers.accumulator, 0b00111000);
}

#[test]
fn test_tya_negative() {
    let core = test(vec![
        LDY::immediate(0b10001000),
        TYA::implied()
    ]);

    core.assert_flags_set(vec![Negative]);
    assert_eq!(core.registers.accumulator, 0b10001000);
}
