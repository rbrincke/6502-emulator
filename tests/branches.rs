extern crate nes;

mod common;

use nes::processor::registers::Flag;

use crate::common::{test_with_flags, TestAssertions};
use nes::processor::registers::Flag::{Zero, Negative};
use nes::processor::instructions::opcodes::*;
use nes::processor::Core;

#[test]
fn test_bcc_clear() {
    test_branching::<BCC>(None, true)
}

#[test]
fn test_bcc_set() {
    test_branching::<BCC>(Some(Flag::Carry), false)
}

#[test]
fn bcs_set() {
    test_branching::<BCS>(Some(Flag::Carry), true)
}

#[test]
fn bcs_clear() {
    test_branching::<BCS>(None, false)
}

#[test]
fn beq_eq() {
    test_branching::<BEQ>(Some(Flag::Zero), true)
}

#[test]
fn beq_ne() {
    test_branching::<BEQ>(None, false)
}

#[test]
fn bvc_clear() {
    test_branching::<BVC>(None, true)
}

#[test]
fn bvc_set() {
    test_branching::<BVC>(Some(Flag::Overflow), false)
}

#[test]
fn bvs_set() {
    test_branching::<BVS>(Some(Flag::Overflow), true)
}

#[test]
fn bvs_clear() {
    test_branching::<BVS>(None, false)
}

#[test]
fn bpl_pos() {
    test_branching::<BPL>(None, true)
}

#[test]
fn bpl_neg() {
    test_branching::<BPL>(Some(Flag::Negative), false)
}

fn test_branching<T : Relative>(setup_flag: Option<Flag>, expected_branch: bool) {
    let flags: Vec<Flag> = setup_flag.into_iter().collect();

    let core = test_with_flags(vec![
        CLI::implied(),                 // Set InterruptDisable.
        T::relative(1),     // Execute branch.
        SEI::implied(),                 // If branch, should not happen.
        LDA::immediate(10)        // Skip to here.
    ], flags);

    assert_eq!(core.registers.accumulator, 10);
    assert_eq!(core.registers.get_flag(Flag::Interrupt), !expected_branch, "Branch expectation failed.");
}
