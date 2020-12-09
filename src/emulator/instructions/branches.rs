use crate::emulator::addressing::AddressMode;
use crate::emulator::registers::Flag;
use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    fn branch(&mut self, flag: Flag, branch_if: bool) {
        let displacement = self.address(AddressMode::Relative);
        if self.registers.status.get(flag) == branch_if {
            let next = self.registers.program_counter.wrapping_add(displacement);
            self.registers.program_counter = next;
        }
    }

    /// Branch if carry clear.
    pub(crate) fn bcc(&mut self) {
        self.branch(Flag::Carry, false);
    }

    pub(crate) fn bcs(&mut self) {
        self.branch(Flag::Carry, true);
    }

    /// Branch if equal.
    pub(crate) fn beq(&mut self) {
        self.branch(Flag::Zero, true);
    }

    /// Branch not equal.
    pub(crate) fn bne(&mut self) {
        self.branch(Flag::Zero, false);
    }

    /// Branch if minus.
    pub(crate) fn bmi(&mut self) {
        self.branch(Flag::Negative, true);
    }

    /// Branch if positive.
    pub(crate) fn bpl(&mut self) {
        self.branch(Flag::Negative, false);
    }

    /// Branch if overflow clear.
    pub(crate) fn bvc(&mut self) {
        self.branch(Flag::Overflow, false);
    }

    /// Branch if overflow set.
    pub(crate) fn bvs(&mut self) {
        self.branch(Flag::Overflow, true);
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::registers::Flag;
    use crate::emulator::registers::Flag::{Carry, Negative, Overflow, Zero};
    use crate::emulator::tests::{setup, Instruction};
    use crate::emulator::Emulator;

    fn test_branch(setup_flags: Vec<Flag>, instruction: Instruction, expect_branch: bool) {
        let mut c = setup(setup_flags);

        let current_program_counter = c.registers.program_counter;
        c.memory.memory[current_program_counter as usize] = 1;
        instruction(&mut c);

        assert_eq!(
            current_program_counter + 1 + (expect_branch as u16),
            c.registers.program_counter
        );
    }

    #[test]
    fn test_bcc_carry_clear() {
        test_branch(vec![], Emulator::bcc, true);
    }

    #[test]
    fn test_bcc_carry_set() {
        test_branch(vec![Carry], Emulator::bcc, false);
    }

    #[test]
    fn test_bcs_carry_clear() {
        test_branch(vec![], Emulator::bcs, false);
    }

    #[test]
    fn test_bcs_carry_set() {
        test_branch(vec![Carry], Emulator::bcs, true);
    }

    #[test]
    fn test_beq_zero_clear() {
        test_branch(vec![], Emulator::beq, false);
    }

    #[test]
    fn test_beq_zero_set() {
        test_branch(vec![Zero], Emulator::beq, true);
    }

    #[test]
    fn test_bvc_overflow_clear() {
        test_branch(vec![], Emulator::bvc, true);
    }

    #[test]
    fn test_bvc_overflow_set() {
        test_branch(vec![Overflow], Emulator::bvc, false);
    }

    #[test]
    fn test_bvs_overflow_clear() {
        test_branch(vec![], Emulator::bvs, false);
    }

    #[test]
    fn test_bvs_overflow_set() {
        test_branch(vec![Overflow], Emulator::bvs, true);
    }

    #[test]
    fn test_bpl_negative_clear() {
        test_branch(vec![], Emulator::bpl, true);
    }

    #[test]
    fn test_bpl_negative_set() {
        test_branch(vec![Negative], Emulator::bpl, false);
    }

    #[test]
    fn test_branch_reverse() {
        let mut c = setup(vec![]);

        let current_program_counter = c.registers.program_counter;
        c.memory.memory[current_program_counter as usize] = -5i8 as u8;
        c.bcc();

        assert_eq!(current_program_counter + 1 - 5, c.registers.program_counter);
    }
}
