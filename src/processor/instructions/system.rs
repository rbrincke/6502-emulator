use crate::cartridge::Cartridge;
use crate::processor::registers::Flag;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    pub(crate) fn push_pc(&mut self) {
        let next = self.registers.program_counter + 1;

        let pc_most_significant = (next >> 8) as u8;
        let pc_least_significant = (next & 0x00FF) as u8;

        self.push(pc_most_significant);
        self.push(pc_least_significant);
    }

    /// No operation.
    pub(crate) fn nop(&self) {}

    /// Force interrupt.
    pub(crate) fn brk(&mut self) {
        self.push_pc();

        self.registers.set_flag(Flag::Break);
        self.php();
        self.registers.set_flag(Flag::Interrupt);

        let pc = self.read_two(0xFFFe, 0xFFFf);

        self.registers.program_counter = pc;
    }

    // Return from interrupt.
    pub(crate) fn rti(&mut self) {
        self.plp();
        let least_significant = self.pop() as u16;
        let most_significant = self.pop() as u16;
        self.registers.program_counter = least_significant | (most_significant << 8);
    }
}
