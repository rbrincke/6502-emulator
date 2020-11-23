use crate::emulator::registers::Flag;
use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    pub(crate) fn pull_pc(&mut self) -> u16 {
        let least_significant = self.pop() as u16;
        let most_significant = self.pop() as u16;

        least_significant | (most_significant << 8)
    }

    pub(crate) fn push_pc(&mut self, program_counter: u16) {
        let pc_most_significant = (program_counter >> 8) as u8;
        let pc_least_significant = (program_counter & 0x00FF) as u8;

        self.push(pc_most_significant);
        self.push(pc_least_significant);
    }

    /// No operation.
    pub(crate) fn nop(&self) {}

    fn interrupt(
        &mut self,
        is_brk: bool,
        interrupt_handler_addr_least: u16,
        interrupt_handler_vector_most: u16,
    ) {
        // BRK skips one instruction.
        self.push_pc(self.registers.program_counter + is_brk as u16);

        // BRK pushes the status registers with the BRK flag set, just like PHP.
        if is_brk {
            self.php()
        } else {
            self.push(self.registers.status.flags)
        }

        self.registers.status.set(Flag::Interrupt);

        let interrupt_handler_addr =
            self.read_two(interrupt_handler_addr_least, interrupt_handler_vector_most);
        self.registers.program_counter = interrupt_handler_addr;
    }

    /// Force interrupt.
    pub(crate) fn brk(&mut self) {
        self.interrupt(true, 0xFFFE, 0xFFFF);
    }

    /// Interrupt request.
    pub(crate) fn irq(&mut self) {
        self.interrupt(false, 0xFFFE, 0xFFFF);
    }

    /// Non-maskable interrupt.
    pub(crate) fn nmi(&mut self) {
        self.interrupt(false, 0xFFFA, 0xFFFB);
        self.nmi = false;
    }

    /// Return from interrupt.
    pub(crate) fn rti(&mut self) {
        self.plp();
        self.registers.program_counter = self.pull_pc();
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::registers::Flag;
    use crate::emulator::registers::Flag::Interrupt;
    use crate::emulator::tests::{setup, Instruction, TestAssertions};
    use crate::emulator::Emulator;
    use crate::memory::basic::DefaultMemory;

    fn test_interrupt(
        e: &mut Emulator<DefaultMemory>,
        instruction: Instruction,
        irh_addr_least: usize,
        irh_addr_most: usize,
    ) {
        e.registers.status.clear(Flag::Interrupt);
        e.registers.stack_pointer = 0xFF;

        // Interrupt request handler.
        e.memory.memory[irh_addr_least] = 0x33;
        e.memory.memory[irh_addr_most] = 0x22;

        instruction(e);

        e.assert_flags_set(vec![Interrupt]);
        assert_eq!(0x2233, e.registers.program_counter); // Expect PC to match interrupt request handler.
    }

    #[test]
    fn test_brk() {
        let mut e = setup(vec![]);

        test_interrupt(&mut e, Emulator::brk, 0xFFFE, 0xFFFF);

        assert_eq!(0x06, e.memory.memory[0x1FF]); // Most significant of 0x600.
        assert_eq!(0x01, e.memory.memory[0x1FE]); // Least significant.
        assert_eq!(0b00110000, e.memory.memory[0x1FD]); // Expect break and always on to be pushed as set.
    }

    #[test]
    fn test_irq() {
        let mut e = setup(vec![]);
        e.irq = true;

        test_interrupt(&mut e, Emulator::irq, 0xFFFE, 0xFFFF);

        assert_eq!(true, e.irq);
        assert_eq!(0x06, e.memory.memory[0x1FF]); // Most significant of 0x600.
        assert_eq!(0x00, e.memory.memory[0x1FE]); // Least significant.
        assert_eq!(0b00100000, e.memory.memory[0x1FD]); // Expect always on to be pushed as set.
    }

    #[test]
    fn test_nmi() {
        let mut e = setup(vec![]);
        e.nmi = true;

        test_interrupt(&mut e, Emulator::nmi, 0xFFFA, 0xFFFB);

        assert_eq!(false, e.nmi);
        assert_eq!(0x06, e.memory.memory[0x1FF]); // Most significant of 0x600.
        assert_eq!(0x00, e.memory.memory[0x1FE]); // Least significant.
        assert_eq!(0b00100000, e.memory.memory[0x1FD]); // Expect always on to be pushed as set.
    }
}
