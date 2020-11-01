use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    /// Clear carry.
    pub(crate) fn clc(&mut self) {
        self.registers.clear_flag(Flag::Carry);
    }

    /// Clear decimal.
    pub(crate) fn cld(&mut self) {
        self.registers.clear_flag(Flag::Decimal);
    }

    /// Clear interrupt.
    pub(crate) fn cli(&mut self) {
        println!("cli");
        self.registers.clear_flag(Flag::Interrupt);
    }

    /// Clear overflow.
    pub(crate) fn clv(&mut self) {
        self.registers.clear_flag(Flag::Overflow);
    }

    /// Set carry.
    pub(crate) fn sec(&mut self) {
        self.registers.set_flag(Flag::Carry);
    }

    /// Set decimal.
    pub(crate) fn sed(&mut self) {
        self.registers.set_flag(Flag::Decimal);
    }

    /// Set interrupt.
    pub(crate) fn sei(&mut self) {
        println!("sei");
        self.registers.set_flag(Flag::Interrupt);
    }
}
