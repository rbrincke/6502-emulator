use crate::processor::core::Core;
use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;

impl<C : Cartridge> Core<C> {
    /// Logical AND.
    pub(crate) fn and(&mut self, address_mode: AddressMode) {
        let result = self.address(address_mode);

        self.registers.accumulator &= result.read(self);
        self.check_value_set_zero(self.registers.accumulator);
        self.check_value_set_negative(self.registers.accumulator);
    }

    /// Bit Test.
    pub(crate) fn bit(&mut self, address_mode: AddressMode) {
        let result = self.address(address_mode);
        let value = result.read(self);

        let bit_and_acc_v = self.registers.accumulator & value;
        self.check_value_set_zero(bit_and_acc_v);

        self.registers.set_flag_to(Flag::Negative, (value & 0b10000000) != 0);
        self.registers.set_flag_to(Flag::Overflow, (value & 0b01000000) != 0);
    }

    /// Exclusive OR.
    pub(crate) fn eor(&mut self, address_mode: AddressMode) {
        let result = self.address(address_mode);

        self.registers.accumulator ^= result.read(self);
        self.check_value_set_zero(self.registers.accumulator);
        self.check_value_set_negative(self.registers.accumulator);
    }

    /// Inclusive OR.
    pub(crate) fn ora(&mut self, address_mode: AddressMode) {
        let result = self.address(address_mode);

        self.registers.accumulator |= result.read(self);
        self.check_value_set_zero(self.registers.accumulator);
        self.check_value_set_negative(self.registers.accumulator);
    }
}
