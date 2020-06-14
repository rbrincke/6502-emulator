extern crate nes;

#[macro_use]
mod common;

use nes::processor::registers::Flag;

use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Zero, Negative};
use nes::processor::instructions::opcodes::*;
use nes::processor::Core;

fn test_branching<F : Fn(u8) -> Vec<u8>>(setup_flag: Option<Flag>, branch: F, expected_branch: bool) {
    let mut initial_status = 0b00100000;
    if let Some(f) = setup_flag {
        initial_status |= 0x1 << (f as u8)
    }

    let core = test(vec![
        LDA::immediate(initial_status), // Set flags.
        PHA::implied(),                 // ...
        PLP::implied(),                 // ...
        CLI::implied(),                 // Set InterruptDisable.
        branch(1),                      // Execute branch.
        SEI::implied(),                 // If branch, should not happen.
        LDA::immediate(10)              // Skip to here.
    ]);

    assert_eq!(core.registers.accumulator, 10);
    assert_eq!(core.registers.get_flag(Flag::Interrupt), !expected_branch, "Branch expectation failed.");
}

#[test]
fn test_bcc_clear() { test_branching(None, BCC::relative, true) }

#[test]
fn test_bcc_set() { test_branching(Some(Flag::Carry), BCC::relative, false) }

#[test]
fn bcs_set() { test_branching(Some(Flag::Carry), BCS::relative, true) }

#[test]
fn bcs_clear() { test_branching(None, BCS::relative, false) }

#[test]
fn beq_eq() { test_branching(Some(Flag::Zero), BEQ::relative, true) }

#[test]
fn beq_ne() { test_branching(None, BEQ::relative, false) }

#[test]
fn bvc_clear() { test_branching(None, BVC::relative, true) }

#[test]
fn bvc_set() { test_branching(Some(Flag::Overflow), BVC::relative, false) }

#[test]
fn bvs_set() { test_branching(Some(Flag::Overflow), BVS::relative, true) }

#[test]
fn bvs_clear() { test_branching(None, BVS::relative, false) }

#[test]
fn bpl_pos() { test_branching(None, BPL::relative, true) }

#[test]
fn bpl_neg() { test_branching(Some(Flag::Negative), BPL::relative, false) }
