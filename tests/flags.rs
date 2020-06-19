extern crate nes;

mod common;

use nes::processor::registers::Flag;

use crate::common::{test_with_flags, TestAssertions};
use nes::processor::instructions::opcodes::*;

#[test]
fn test_clc() {
    test::<CLC>(Some(Flag::Carry), None)
}

#[test]
fn test_cld() {
    test::<CLD>(Some(Flag::Decimal), None)
}

#[test]
fn test_cli() {
    test::<CLI>(Some(Flag::Interrupt), None)
}

#[test]
fn clv() {
    test::<CLV>(Some(Flag::Overflow), None)
}

#[test]
fn sec() {
    test::<SEC>(None, Some(Flag::Carry))
}

#[test]
fn sed() {
    test::<SED>(None, Some(Flag::Decimal))
}

#[test]
fn sei() {
    test::<SEI>(None, Some(Flag::Interrupt))
}

fn test<T: Implied>(initialize: Option<Flag>, expected_set: Option<Flag>) {
    let t = test_with_flags(vec![T::implied()], initialize.into_iter().collect());
    t.assert_flags_set(expected_set.into_iter().collect());
}
