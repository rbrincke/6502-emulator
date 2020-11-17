use crate::emulator::registers::{Flag, Registers};
use crate::memory::Memory;

pub mod read_write;
pub mod registers;
mod addressing;
pub mod instructions;

pub struct Emulator<M: Memory> {
    pub registers: Registers,
    pub(crate) memory: M,
    pub irq: bool,
    pub nmi: bool
}

impl<C : Memory> Emulator<C> {
    pub fn new(memory: C) -> Emulator<C> {
        let mut emulator = Emulator {
            registers: Registers::new(),
            memory,
            irq: false,
            nmi: false
        };

        emulator.reset();
        emulator
    }

    fn reset(&mut self) {
        self.nmi = false;
        self.irq = false;
        self.registers.status.set(Flag::Interrupt);

        // Initialize the program counter.
        self.registers.program_counter = self.read_two(0xFFFC, 0xFFFD);
    }

    pub fn execute_next(&mut self) {
        // Hardware interrupts.
        if self.nmi {
            self.nmi()
        } else if self.irq && !self.registers.status.get(Flag::Interrupt) {
            self.irq()
        }

        let instruction = self.read(self.registers.program_counter);
        self.registers.program_counter += 1;

        self.execute(instruction);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::addressing::AddressMode;
    use crate::emulator::Emulator;
    use crate::emulator::registers::{Flag, Registers};
    use crate::emulator::registers::Flag::*;
    use crate::memory::basic::DefaultMemory;

    pub(crate) type Instruction = for<'r> fn(&'r mut Emulator<DefaultMemory>) -> ();
    pub(crate) type AddressInstruction = for<'r> fn(&'r mut Emulator<DefaultMemory>, AddressMode) -> ();
    pub(crate) type RegisterRead = fn(&Registers) -> u8;
    pub(crate) type RegisterWrite = fn(&mut Registers, value: u8) -> ();

    pub(crate) const READ_ACCUMULATOR: RegisterRead = |r| r.accumulator;
    pub(crate) const READ_X: RegisterRead = |r| r.x;
    pub(crate) const READ_Y: RegisterRead = |r| r.y;

    pub(crate) const WRITE_ACCUMULATOR: RegisterWrite = |r, v| r.accumulator = v;
    pub(crate) const WRITE_X: RegisterWrite = |r, v| r.x = v;
    pub(crate) const WRITE_Y: RegisterWrite = |r, v| r.y = v;

    pub(crate) trait TestSetup {
        fn set_flags(&mut self, set: Vec<Flag>);
    }

    impl TestSetup for Emulator<DefaultMemory> {
        /// Sets flags by setting all provided flags to 1 and all others to 0. Ignores the Reserved flag.
        fn set_flags(&mut self, set: Vec<Flag>) {
            [Carry, Zero, Interrupt, Decimal, Break, Overflow, Negative].iter().for_each(|f| {
                self.registers.status.set_to(*f, set.contains(f));
            });
        }
    }

    pub(crate) trait TestAssertions {
        fn assert_flags_set(&self, expected_set: Vec<Flag>);
    }

    impl TestAssertions for Emulator<DefaultMemory> {
        fn assert_flags_set(&self, expected_flags_set: Vec<Flag>) {
            [Carry, Zero, Interrupt, Decimal, Break, Overflow, Negative].iter().for_each(|f| {
                let expectation = expected_flags_set.contains(f);
                assert_eq!(self.registers.status.get(*f), expectation, "Expectation for {:?} flag failed.", f);
            });
        }
    }

    pub(crate) fn setup(flags: Vec<Flag>) -> Emulator<DefaultMemory> {
        let mut r = Registers::new();
        r.program_counter = 0x600;

        let mut c = Emulator {
            registers: r,
            memory: DefaultMemory::empty(),
            irq: false,
            nmi: false
        };

        c.set_flags(flags);
        c
    }
}
