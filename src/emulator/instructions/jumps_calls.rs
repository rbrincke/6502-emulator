use crate::emulator::addressing::AddressMode;
use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    /// Jump.
    pub(crate) fn jmp(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        self.registers.program_counter = address;
    }

    pub(crate) fn jsr(&mut self) {
        self.push_pc(self.registers.program_counter + 1);
        self.registers.program_counter = self.address_absolute();
    }

    pub(crate) fn rts(&mut self) {
        self.registers.program_counter = self.pull_pc() + 1;
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::addressing::AddressMode;
    use crate::emulator::tests::setup;

    #[test]
    fn test_jmp() {
        let mut c = setup(vec![]);

        c.memory.memory[0x600] = 0xcd;
        c.memory.memory[0x601] = 0xab;

        c.jmp(AddressMode::Absolute);

        assert_eq!(0xabcd, c.registers.program_counter);
    }

    #[test]
    fn test_jsr() {
        let mut c = setup(vec![]);

        c.registers.stack_pointer = 0xFF;
        c.memory.memory[0x600] = 0xcd;
        c.memory.memory[0x601] = 0xab;

        c.jsr();
        assert_eq!(0x06, c.memory.memory[0x1FF]);
        assert_eq!(0x01, c.memory.memory[0x1FE]); // Adds 1.

        assert_eq!(0xabcd, c.registers.program_counter);
    }

    #[test]
    fn test_rts() {
        let mut c = setup(vec![]);

        c.memory.memory[0x1FE] = 0xcc;
        c.memory.memory[0x1FF] = 0xab;
        c.registers.stack_pointer = 0xFD;

        c.rts();

        // 0xABCD = 0xABCC + 1.
        assert_eq!(0xabcd, c.registers.program_counter)
    }

    #[test]
    fn test_jsr_rts() {
        let mut c = setup(vec![]);

        c.registers.program_counter = 0x601;
        c.jsr();
        c.rts();

        // Lands after the 2-byte address.
        assert_eq!(0x603, c.registers.program_counter)
    }
}
