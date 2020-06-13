use crate::common::cartridge::TestCartridge;
use nes::processor::registers::Flag;
use nes::processor::Core;

mod cartridge;

/// Run a series of instructions and then return the machine state.
///
/// A single BRK instruction is added at the end of the Vec of instructions. Instructions
/// are then executed until the BRK flag is set.
pub fn test_run(mut program: Vec<u8>) -> Core<TestCartridge> {
    program.push(0x00u8);
    let mut core = Core::new(
        TestCartridge::new(program)
    );

    while !core.registers.get_flag(Flag::Break) {
        core.execute_next();
    }

    core
}

pub(crate) trait FlagAssertions {
    fn assert_carry(&self, expected: bool);
    fn assert_zero(&self, expected: bool);
    fn assert_negative(&self, expected: bool);
    fn assert_overflow(&self, expected: bool);
}

impl FlagAssertions for Core<TestCartridge> {
    fn assert_carry(&self, expected_set: bool) {
        assert_eq!(self.registers.get_flag(Flag::Carry), expected_set, "Expectation for Carry flag failed.");
    }

    fn assert_zero(&self, expected_set: bool) {
        assert_eq!(self.registers.get_flag(Flag::Zero), expected_set, "Expectation for Zero flag failed.");
    }

    fn assert_negative(&self, expected_set: bool) {
        assert_eq!(self.registers.get_flag(Flag::Negative), expected_set, "Expectation for Negative flag failed.");
    }

    fn assert_overflow(&self, expected_set: bool) {
        assert_eq!(self.registers.get_flag(Flag::Overflow), expected_set, "Expectation for Overflow flag failed.");
    }
}
