use crate::memory::Memory;
use crate::emulator::registers::Flag;
use crate::emulator::Emulator;

impl<C : Memory> Emulator<C> {
    pub(crate) fn pull_pc(&mut self) -> u16 {
        let least_significant = self.pop() as u16;
        let most_significant = self.pop() as u16;

        least_significant | (most_significant << 8)
    }

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

        self.php();
        self.registers.status.set(Flag::Interrupt);

        let pc = self.read_two(0xFFFE, 0xFFFF);

        self.registers.program_counter = pc;
    }

    // Return from interrupt.
    pub(crate) fn rti(&mut self) {
        self.plp();
        self.registers.program_counter = self.pull_pc();
    }
}
