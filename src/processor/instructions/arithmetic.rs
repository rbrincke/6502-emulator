use crate::processor::core::Core;
use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;

impl<C : Cartridge> Core<C> {
    fn adc_value(&mut self, value: u8) {
        let carry = self.registers.get_flag(Flag::Carry) as u16;

        let result_intermediate = self.registers.accumulator as u16 + value as u16 + carry;
        self.check_value_set_carry(result_intermediate);
        self.check_value_set_overflow(self.registers.accumulator, value, result_intermediate);

        let result = (result_intermediate & 0xFF) as u8;
        self.registers.accumulator = result;
        self.check_value_set_zero(result);
        self.check_value_set_negative(result);
    }

    /// Add with carry.
    pub(crate) fn adc(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = addr.read(self);
        self.adc_value(value);
    }

    /// Subtract with carry.
    pub(crate) fn sbc(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = addr.read(self) ^ 0xFF;
        self.adc_value(value);
    }
}
