use crate::emulator::bytes_little_endian;
use crate::emulator::registers::Flag;
use crate::emulator::Emulator;
use crate::memory::Memory;
use emulator::{INT_VECTOR_ADDR, NMI_VECTOR_ADDR};

impl<C: Memory> Emulator<C> {
    pub(crate) fn pull_pc(&mut self) -> u16 {
        let least_significant = self.pop() as u16;
        let most_significant = self.pop() as u16;

        least_significant | (most_significant << 8)
    }

    pub(crate) fn push_pc(&mut self, program_counter: u16) {
        let (least, most) = bytes_little_endian(program_counter);

        self.push(most);
        self.push(least);
    }

    /// No operation.
    pub(crate) fn nop(&self) {}

    fn interrupt(&mut self, is_brk: bool, int_vector_addr_least: u16, int_vector_addr_most: u16) {
        // BRK skips one instruction.
        self.push_pc(self.registers.program_counter + is_brk as u16);

        // BRK pushes the status registers with the BRK flag set, just like PHP.
        if is_brk {
            self.php()
        } else {
            self.push(self.registers.status.flags)
        }

        self.registers.status.set(Flag::Interrupt);

        let interrupt_handler_addr = self.read_two(int_vector_addr_least, int_vector_addr_most);
        self.registers.program_counter = interrupt_handler_addr;
    }

    /// Force interrupt.
    pub(crate) fn brk(&mut self) {
        self.interrupt(true, INT_VECTOR_ADDR, INT_VECTOR_ADDR + 1);
    }

    /// Interrupt request.
    pub(crate) fn irq(&mut self) {
        self.interrupt(false, INT_VECTOR_ADDR, INT_VECTOR_ADDR + 1);
    }

    /// Non-maskable interrupt.
    pub(crate) fn nmi(&mut self) {
        self.interrupt(false, NMI_VECTOR_ADDR, NMI_VECTOR_ADDR + 1);
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
    use crate::memory::default::DefaultMemory;
    use emulator::{INT_VECTOR_ADDR, NMI_VECTOR_ADDR};

    fn test_interrupt(
        e: &mut Emulator<DefaultMemory>,
        instruction: Instruction,
        irh_addr_least: u16,
    ) {
        e.registers.status.clear(Flag::Interrupt);
        e.registers.stack_pointer = 0xFF;

        // Interrupt request handler.
        e.memory.memory[irh_addr_least as usize] = 0x33;
        e.memory.memory[(irh_addr_least + 1) as usize] = 0x22;

        instruction(e);

        e.assert_flags_set(vec![Interrupt]);
        assert_eq!(0x2233, e.registers.program_counter); // Expect PC to match interrupt request handler.
    }

    #[test]
    fn test_brk() {
        let mut e = setup(vec![]);

        test_interrupt(&mut e, Emulator::brk, INT_VECTOR_ADDR);

        assert_eq!(0x06, e.memory.memory[0x1FF]); // Most significant of 0x600.
        assert_eq!(0x01, e.memory.memory[0x1FE]); // Least significant.
        assert_eq!(0b00110000, e.memory.memory[0x1FD]); // Expect break and always on to be pushed as set.
    }

    #[test]
    fn test_irq() {
        let mut e = setup(vec![]);
        e.irq = true;

        test_interrupt(&mut e, Emulator::irq, INT_VECTOR_ADDR);

        assert_eq!(true, e.irq);
        assert_eq!(0x06, e.memory.memory[0x1FF]); // Most significant of 0x600.
        assert_eq!(0x00, e.memory.memory[0x1FE]); // Least significant.
        assert_eq!(0b00100000, e.memory.memory[0x1FD]); // Expect always on to be pushed as set.
    }

    #[test]
    fn test_nmi() {
        let mut e = setup(vec![]);
        e.nmi = true;

        test_interrupt(&mut e, Emulator::nmi, NMI_VECTOR_ADDR);

        assert_eq!(false, e.nmi);
        assert_eq!(0x06, e.memory.memory[0x1FF]); // Most significant of 0x600.
        assert_eq!(0x00, e.memory.memory[0x1FE]); // Least significant.
        assert_eq!(0b00100000, e.memory.memory[0x1FD]); // Expect always on to be pushed as set.
    }
}
