use crate::emulator::registers::Flag;
use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    /// Clear carry.
    pub(crate) fn clc(&mut self) {
        self.registers.status.clear(Flag::Carry);
    }

    /// Clear decimal.
    pub(crate) fn cld(&mut self) {
        self.registers.status.clear(Flag::Decimal);
    }

    /// Clear interrupt.
    pub(crate) fn cli(&mut self) {
        self.registers.status.clear(Flag::Interrupt);
    }

    /// Clear overflow.
    pub(crate) fn clv(&mut self) {
        self.registers.status.clear(Flag::Overflow);
    }

    /// Set carry.
    pub(crate) fn sec(&mut self) {
        self.registers.status.set(Flag::Carry);
    }

    /// Set decimal.
    pub(crate) fn sed(&mut self) {
        self.registers.status.set(Flag::Decimal);
    }

    /// Set interrupt.
    pub(crate) fn sei(&mut self) {
        self.registers.status.set(Flag::Interrupt);
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::registers::Flag;
    use crate::emulator::registers::Flag::*;
    use crate::emulator::tests::*;
    use crate::emulator::Emulator;

    fn test(setup_flags: Vec<Flag>, instruction: Instruction, expected_flags_set: Vec<Flag>) {
        let mut c = setup(setup_flags);

        instruction(&mut c);
        c.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_clc() {
        test(vec![Carry], Emulator::clc, vec![]);
    }

    #[test]
    fn test_cld() {
        test(vec![Decimal], Emulator::cld, vec![]);
    }

    #[test]
    fn test_cli() {
        test(vec![Interrupt], Emulator::cli, vec![]);
    }

    #[test]
    fn clv() {
        test(vec![Overflow], Emulator::clv, vec![]);
    }

    #[test]
    fn sec() {
        test(vec![], Emulator::sec, vec![Carry]);
    }

    #[test]
    fn sed() {
        test(vec![], Emulator::sed, vec![Decimal]);
    }

    #[test]
    fn sei() {
        test(vec![], Emulator::sei, vec![Interrupt]);
    }
}
