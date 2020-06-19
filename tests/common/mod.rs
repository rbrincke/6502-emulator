use crate::common::cartridge::TestCartridge;
use nes::processor::registers::Flag;
use nes::processor::Core;
use nes::processor::registers::Flag::{Zero, Carry, Interrupt, Decimal, Overflow, Negative};
use nes::processor::instructions::opcodes::{Implied, Accumulator, Immediate, LDA, PHA, PLP, BRK};

mod cartridge;

/// Run a series of instructions and then return the machine state.
///
/// LDA, PHA and PLP instructions are prepended to it. A single BRK instruction is
/// added at the end of the Vec of instructions. Instructions are executed until the
/// BRK flag is set.
pub fn test(instructions: Vec<Vec<u8>>) -> Core<TestCartridge> {
    test_with_flags(instructions, vec![])
}

/// Run a series of instructions and then return the machine state. Sets the flags
/// before running any instructions.
///
/// LDA, PHA and PLP instructions are prepended to it. A single BRK instruction is
/// added at the end of the Vec of instructions. Instructions are executed until the
/// BRK flag is set.
pub fn test_with_flags(instructions: Vec<Vec<u8>>, flags: Vec<Flag>) -> Core<TestCartridge> {
    // Add flag initialization.
    let mut program = flatten(instructions_set_flag(flags));

    // Add instructions.
    program.extend(flatten(instructions));

    // Add BRK;
    program.extend(BRK::implied());

    let mut core = Core::new(
        TestCartridge::new(program)
    );

    while !core.registers.get_flag(Flag::Break) {
        core.execute_next();
    }

    core
}

fn flatten(instructions: Vec<Vec<u8>>) -> Vec<u8> {
    instructions.into_iter().flatten().collect()
}

fn instructions_set_flag(flags: Vec<Flag>) -> Vec<Vec<u8>> {
    assert!(!flags.contains(&Flag::Break));

    let mut initial_status = 0b00100000;
    for f in flags {
        initial_status |= 0x1 << (f as u8)
    }

    vec![
        LDA::immediate(initial_status), // Set accumulator.
        PHA::implied(),                       // Push accumulator on stack.
        PLP::implied(),                       // Pull processor status off stack.
    ]
}

pub trait TestAssertions {
    fn assert_flags_set(&self, expected_set: Vec<Flag>) -> &Self;
    fn assert_x(&self, expected: i8) -> &Self;
    fn assert_y(&self, expected: i8) -> &Self;
    fn assert_accumulator(&self, expected: i8) -> &Self;
}

impl TestAssertions for Core<TestCartridge> {
    fn assert_flags_set(&self, expected_flags_set: Vec<Flag>) -> &Self {
        [Carry, Zero, Interrupt, Decimal, Overflow, Negative].iter().for_each(|f| {
            let expectation = expected_flags_set.contains(f);
            assert_eq!(self.registers.get_flag(*f), expectation, "Expectation for {:?} flag failed.", f);
        });
        self
    }

    fn assert_x(&self, expected: i8) -> &Self {
        assert_eq!(self.registers.x as i8, expected);
        self
    }

    fn assert_y(&self, expected: i8) -> &Self {
        assert_eq!(self.registers.y as i8, expected);
        self
    }

    fn assert_accumulator(&self, expected: i8) -> &Self {
        assert_eq!(self.registers.accumulator as i8, expected);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{test_with_flags, TestAssertions};
    use nes::processor::registers::Flag;

    #[test]
    fn test_setup_flags_no_flags() {
        let t = test_with_flags(vec![], vec![]);
        t.assert_flags_set(vec![]);
    }

    #[test]
    fn test_setup_flags() {
        let v = vec![Flag::Carry, Flag::Overflow];
        let t = test_with_flags(vec![], v.clone());
        t.assert_flags_set(v);
    }
}
