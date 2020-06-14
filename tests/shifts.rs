extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Carry, Negative};
use nes::processor::instructions::set::*;

#[test]
fn test_asl() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(0b10001111),
        ASL::accumulator()
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b00011110);
}

#[test]
fn test_lsr() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(0b10001111),
        LSR::accumulator()
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b01000111);
}

#[test]
fn test_rol_carry_cleared() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(0b10001111),
        ROL::accumulator()
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b00011110);
}

#[test]
fn test_rol_carry_set() {
    let core = test(vec![
        SEC::implied(),
        LDA::immediate(0b10001111),
        ROL::accumulator()
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b00011111);
}

#[test]
fn test_ror_carry_cleared() {
    let core = test(vec![
        CLC::implied(),
        LDA::immediate(0b10001111),
        ROR::accumulator()
    ]);

    core.assert_flags_set(vec![Carry]);
    assert_eq!(core.registers.accumulator, 0b01000111);
}

#[test]
fn test_ror_carry_set() {
    let core = test(vec![
        SEC::implied(),
        LDA::immediate(0b10001111),
        ROR::accumulator()
    ]);

    core.assert_flags_set(vec![Carry, Negative]);
    assert_eq!(core.registers.accumulator, 0b11000111);
}
