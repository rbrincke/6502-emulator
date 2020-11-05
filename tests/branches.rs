extern crate nes;

mod common;

use nes::processor::registers::Flag;

use crate::common::{test_with_flags, TestAssertions};
use nes::processor::registers::Flag::{Zero, Negative};
use nes::processor::instructions::opcodes::*;
use nes::processor::Core;

#[test]
fn test_bcc_clear() {
    test_branch_forwards::<BCC>(None, true)
}

#[test]
fn test_bcc_set() {
    test_branch_forwards::<BCC>(Some(Flag::Carry), false)
}

#[test]
fn bcs_set() {
    test_branch_forwards::<BCS>(Some(Flag::Carry), true)
}

#[test]
fn bcs_clear() {
    test_branch_forwards::<BCS>(None, false)
}

#[test]
fn beq_eq() {
    test_branch_forwards::<BEQ>(Some(Flag::Zero), true)
}

#[test]
fn beq_ne() {
    test_branch_forwards::<BEQ>(None, false)
}

#[test]
fn bvc_clear() {
    test_branch_forwards::<BVC>(None, true)
}

#[test]
fn bvc_set() {
    test_branch_forwards::<BVC>(Some(Flag::Overflow), false)
}

#[test]
fn bvs_set() {
    test_branch_forwards::<BVS>(Some(Flag::Overflow), true)
}

#[test]
fn bvs_clear() {
    test_branch_forwards::<BVS>(None, false)
}

#[test]
fn bpl_pos() {
    test_branch_forwards::<BPL>(None, true)
}

#[test]
fn bpl_neg() {
    test_branch_forwards::<BPL>(Some(Flag::Negative), false)
}

fn test_branch_forwards<T : Relative>(setup_flag: Option<Flag>, expected_branch: bool) {
    let flags: Vec<Flag> = setup_flag.into_iter().collect();

    let core = test_with_flags(vec![
        CLD::implied(),                 // Set InterruptDisable.
        T::relative(1),     // Execute branch.
        SED::implied(),                 // If branch, should not happen.
        LDA::immediate(10)        // Skip to here.
    ], flags);

    assert_eq!(core.registers.accumulator, 10);
    assert_eq!(core.registers.get_flag(Flag::Decimal), !expected_branch, "Branch expectation failed.");
}
