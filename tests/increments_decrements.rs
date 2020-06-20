extern crate nes;

mod common;

use crate::common::{test, TestAssertions};
use nes::processor::instructions::opcodes::*;
use nes::processor::registers::Flag;
use nes::processor::registers::Flag::{Zero, Negative};

#[test]
fn test_inx_0() {
    test_x::<INX>(0i8, 1i8, vec![])
}

#[test]
fn test_inx_neg1() {
    test_x::<INX>(-1i8, 0i8, vec![Zero])
}

#[test]
fn test_inx_overflow() {
    test_x::<INX>(127i8, -128i8, vec![Negative])
}

#[test]
fn test_dex_0() {
    test_x::<DEX>(0i8, -1i8, vec![Negative])
}

#[test]
fn test_dex_1() {
    test_x::<DEX>(1i8, 0i8, vec![Zero])
}

#[test]
fn test_dex_neg1() {
    test_x::<DEX>(-1i8, -2i8, vec![Negative])
}

#[test]
fn test_dex_overflow() {
    test_x::<DEX>(-128i8, 127i8, vec![])
}

#[test]
fn test_iny_0() {
    test_y::<INY>(0i8, 1i8, vec![]);
}

#[test]
fn test_iny_neg1() {
    test_y::<INY>(-1i8, 0i8, vec![Zero]);
}

#[test]
fn test_iny_overflow() {
    test_y::<INY>(127i8, -128i8, vec![Negative])
}

#[test]
fn test_dey_0() {
    test_y::<DEY>(0i8, -1i8, vec![Negative])
}

#[test]
fn test_dey_1() {
    test_y::<DEY>(1i8, 0i8, vec![Zero])
}

#[test]
fn test_dey_neg1() {
    test_y::<DEY>(-1i8, -2i8, vec![Negative])
}

#[test]
fn test_dey_overflow() {
    test_y::<DEY>(-128i8, 127i8, vec![])
}

#[test]
fn test_inc_0() {
    test_other::<INC>(0i8, 1i8, vec![]);
}

#[test]
fn test_inc_neg1() {
    test_other::<INC>(-1i8, 0i8, vec![Zero]);
}

#[test]
fn test_inc_overflow() {
    test_other::<INC>(127i8, -128i8, vec![Negative])
}

#[test]
fn test_dec_0() {
    test_other::<DEC>(0i8, -1i8, vec![Negative])
}

#[test]
fn test_dec_1() {
    test_other::<DEC>(1i8, 0i8, vec![Zero])
}

#[test]
fn test_dec_neg1() {
    test_other::<DEC>(-1i8, -2i8, vec![Negative])
}

#[test]
fn test_dec_overflow() {
    test_other::<DEC>(-128i8, 127i8, vec![])
}

fn test_x<T: Implied>(x_register_init: i8, expected: i8, expected_flags: Vec<Flag>) {
    let t = test(vec![LDX::immediate(x_register_init as u8), T::implied()]);
    t.assert_x(expected);
    t.assert_flags_set(expected_flags);
}

fn test_y<T: Implied>(y_register_init: i8, expected: i8, expected_flags: Vec<Flag>) {
    let t = test(vec![LDY::immediate(y_register_init as u8), T::implied()]);
    t.assert_y(expected);
    t.assert_flags_set(expected_flags);
}

fn test_other<T: ZeroPage>(init: i8, expected: i8, expected_flags: Vec<Flag>) {
    let t = test(vec![LDA::immediate(init as u8), STA::zero_page(0u8), T::zero_page(0u8), LDA::zero_page(0u8)]);
    t.assert_accumulator(expected);
    t.assert_flags_set(expected_flags);
}
