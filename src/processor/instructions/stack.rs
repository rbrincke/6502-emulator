use crate::cartridge::Cartridge;
use crate::processor::Core;
use crate::processor::registers::{Flag, Registers};

impl<C : Cartridge> Core<C> {
    /// Transfer X to Stack Pointer.
    pub(crate) fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.x;
    }

    /// Transfer Stack Pointer to X.
    pub(crate) fn tsx(&mut self) {
        self.registers.x = self.registers.stack_pointer;
        self.check_value_set_zero_negative(self.registers.x);
    }

    pub(crate) fn push(&mut self, value: u8) {
        let address = 0x100 + self.registers.stack_pointer as u16;
        self.write(address, value);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    pub(crate) fn pop(&mut self) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);

        let address = 0x100 + self.registers.stack_pointer as u16;
        let value = self.read(address);
        value
    }

    pub(crate) fn pha(&mut self) {
        self.push(self.registers.accumulator)
    }

    /// Push processor status.
    pub(crate) fn php(&mut self) {
        // Push status with B on, but do not reflect in registers.
        self.push(self.registers.status | (1 << Flag::Break as u8))
    }

    pub(crate) fn pla(&mut self) {
        self.registers.accumulator = self.pop();
        self.check_value_set_zero_negative(self.registers.accumulator);
    }

    /// Pull processor status.
    pub(crate) fn plp(&mut self) {
        self.registers.status = self.pop() | Registers::DEFAULT_STATUS;
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::Core;
    use crate::cartridge::basic::BasicCartridge;
    use crate::processor::registers::Registers;

    fn core() -> Core<BasicCartridge> {
        Core {
            registers: Registers::new(),
            cartridge: BasicCartridge::empty()
        }
    }

    #[test]
    fn test_stack_wrap_positive() {
        let mut c = core();

        c.registers.stack_pointer = 0xFF;
        c.pop();
        assert_eq!(0x00, c.registers.stack_pointer);
    }

    #[test]
    fn test_stack_wrap_negative() {
        let mut c = core();

        c.registers.stack_pointer = 0x00;
        c.push(0);
        assert_eq!(0xFF, c.registers.stack_pointer);
    }
}
