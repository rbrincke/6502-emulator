use crate::processor::core::Core;
use crate::cartridge::Cartridge;
use crate::processor::registers::Flag;

impl<C : Cartridge> Core<C> {
    /// No operation.
    pub(crate) fn nop(&self) {}

    /// Force interrupt.
    pub(crate) fn brk(&mut self) {
        // TODO: Push program counter and processor status on stack.
        self.registers.program_counter = self.read_two(0xFFFe, 0xFFFf);
        self.registers.set_flag(Flag::Break);
    }
}
