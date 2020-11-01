use crate::cartridge::Cartridge;
use crate::processor::registers::Flag;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    /// No operation.
    pub(crate) fn nop(&self) {
        println!("nop");
    }

    /// Force interrupt.
    pub(crate) fn brk(&mut self) {
        let most_significant = self.registers.program_counter >> 8;
        let least_significant = self.registers.program_counter & 0xFF;
        self.push(most_significant as u8);
        self.push(least_significant as u8);

        self.registers.set_flag(Flag::Break);
        self.php();

        self.registers.program_counter = self.read_two(0xFFFe, 0xFFFf);
    }

    // Return from interrupt.
    pub(crate) fn rti(&mut self) {
        self.plp();
        let least_significant = self.pop() as u16;
        let most_significant = self.pop() as u16;
        self.registers.program_counter = least_significant | (most_significant << 8);
    }
}
