use crate::processor::core::Core;
use crate::cartridge::Cartridge;

impl<C : Cartridge> Core<C> {
    /// Transfer X to Stack Pointer.
    pub(crate) fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.x;
    }

    /// Transfer Stack Pointer to X.
    pub(crate) fn tsx(&mut self) {
        self.registers.x = self.registers.stack_pointer;

        self.check_value_set_zero(self.registers.x);
        self.check_value_set_negative(self.registers.x);
    }
}
