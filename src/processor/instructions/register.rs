use crate::processor::core::Core;
use crate::cartridge::Cartridge;

impl<C : Cartridge> Core<C> {
    /// Transfer Accumulator to X.
    pub(crate) fn tax(&mut self) {
        self.registers.x = self.registers.accumulator;

        self.check_value_set_zero(self.registers.x);
        self.check_value_set_negative(self.registers.x);
    }

    /// Transfer Accumulator to Y.
    pub(crate) fn tay(&mut self) {
        self.registers.y = self.registers.accumulator;

        self.check_value_set_zero(self.registers.y);
        self.check_value_set_negative(self.registers.y);
    }

    /// Transfer X to Accumulator.
    pub(crate) fn txa(&mut self) {
        self.registers.accumulator = self.registers.x;

        self.check_value_set_zero(self.registers.accumulator);
        self.check_value_set_negative(self.registers.accumulator);
    }

    /// Transfer Y to Accumulator.
    pub(crate) fn tya(&mut self) {
        self.registers.accumulator = self.registers.y;

        self.check_value_set_zero(self.registers.accumulator);
        self.check_value_set_negative(self.registers.accumulator);
    }
}
