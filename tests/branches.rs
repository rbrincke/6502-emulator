extern crate nes;

mod common;

use nes::processor::registers::Flag;
use crate::common::{test, TestAssertions};
use nes::processor::registers::Flag::{Zero, Negative};
use nes::processor::instructions::opcodes::*;
use nes::processor::Core;

macro_rules! branch_tests {
    ( $( $name:ident: $value:expr ),*) => {
    $(
        #[test]
        fn $name() {
            let (setup_flag, branch, expected_branch): (Option<Flag>, _, _) = $value;

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
    )*
    }
}

branch_tests!{
    bcc_clear: (None, BCC::relative, true),
    bcc_set:   (Some(Flag::Carry), BCC::relative, false),
    bcs_set:   (Some(Flag::Carry), BCS::relative, true),
    bcs_clear: (None, BCS::relative, false),
    beq_eq:    (Some(Flag::Zero), BEQ::relative, true),
    beq_ne:    (None, BEQ::relative, false),
    bvc_clear: (None, BVC::relative, true),
    bvc_set:   (Some(Flag::Overflow), BVC::relative, false),
    bvs_set:   (Some(Flag::Overflow), BVS::relative, true),
    bvs_clear: (None, BVS::relative, false),
    bpl_pos:   (None, BPL::relative, true),
    bpl_neg:   (Some(Flag::Negative), BPL::relative, false)
}
