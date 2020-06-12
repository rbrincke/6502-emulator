use crate::processor::registers::Flag;
use crate::processor::instructions::is_page_crossed;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    fn branch(&mut self, flag: Flag, branch_if: bool) {
        let displacement = self.address_relative();
        if self.registers.get_flag(flag) == branch_if {
            if is_page_crossed(self.registers.program_counter, displacement) {
                self.tick();
            }

            self.tick();
            self.registers.program_counter += displacement;
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
