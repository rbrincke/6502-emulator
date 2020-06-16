extern crate nes;

mod common;

use nes::processor::registers::Flag;

use crate::common::{test_with_flags, TestAssertions};
use nes::processor::instructions::opcodes::*;

fn test(instruction: Vec<u8>, initialize: Option<Flag>, expected_set: Option<Flag>) {
    let t = test_with_flags(vec![instruction], initialize.into_iter().collect());
    t.assert_flags_set(expected_set.into_iter().collect())
}

#[test]
fn test_clc() {
    test(CLC::implied(), Some(Flag::Carry), None)
}

#[test]
fn test_cld() {
    test(CLD::implied(), Some(Flag::Decimal), None)
}

#[test]
fn test_cli() {
    test(CLI::implied(), Some(Flag::Interrupt), None)
}

#[test]
fn clv() {
    test(CLV::implied(), Some(Flag::Overflow), None)
}

#[test]
fn sec() {
    test(SEC::implied(), None, Some(Flag::Carry))
}

#[test]
fn sed() {
    test(SED::implied(), None, Some(Flag::Decimal))
}

#[test]
fn sei() {
    test(SEI::implied(), None, Some(Flag::Interrupt))
}
