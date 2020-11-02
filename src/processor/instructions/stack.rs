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
        self.registers.stack_pointer -= 1;
    }

    pub(crate) fn pop(&mut self) -> u8 {
        self.registers.stack_pointer += 1;
        let address = 0x100 + self.registers.stack_pointer as u16;
        let value = self.read(address);
        value
    }

    pub(crate) fn pha(&mut self) {
        self.push(self.registers.accumulator)
    }

    /// Push processor status.
    pub(crate) fn php(&mut self) {
        self.registers.set_flag(Flag::Break);
        self.push(self.registers.status)
    }

    pub(crate) fn pla(&mut self) {
        self.registers.accumulator = self.pop();
        self.check_value_set_zero_negative(self.registers.accumulator);
    }

    /// Pull processor status.
    pub(crate) fn plp(&mut self) {
        self.registers.status = self.pop() | Registers::DEFAULT_STATUS
    }
}
