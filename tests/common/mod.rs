use crate::common::cartridge::TestCartridge;
use nes::processor::registers::Flag;
use nes::processor::Core;
use nes::processor::registers::Flag::{Zero, Carry, Interrupt, Decimal, Overflow, Negative};
use nes::processor::instructions::opcodes::{Implied, Accumulator, Immediate};

mod cartridge;

/// Run a series of instructions and then return the machine state.
///
/// A single BRK instruction is added at the end of the Vec of instructions. Instructions
/// are executed until the BRK flag is set.
pub fn test(instructions: Vec<Vec<u8>>) -> Core<TestCartridge> {
    let mut program: Vec<u8> = instructions.into_iter().flatten().collect();

    // Add BRK;
    program.push(0x00u8);

    let mut core = Core::new(
        TestCartridge::new(program)
    );

    while !core.registers.get_flag(Flag::Break) {
        core.execute_next();
    }

    core
}

pub trait TestAssertions {
    fn assert_flags_set(&self, expected_set: Vec<Flag>);
}

impl TestAssertions for Core<TestCartridge> {
    fn assert_flags_set(&self, expected_flags_set: Vec<Flag>) {
        [Carry, Zero, Interrupt, Decimal, Overflow, Negative].iter().for_each(|f| {
            let expectation = expected_flags_set.contains(f);
            assert_eq!(self.registers.get_flag(*f), expectation, "Expectation for {:?} flag failed.", f);
        });
    }
}
