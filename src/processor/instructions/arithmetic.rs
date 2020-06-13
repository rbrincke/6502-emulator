use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;
use crate::processor::Core;

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
        let value = self.read(addr);
        self.adc_value(value);
    }

    /// Subtract with carry.
    pub(crate) fn sbc(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = self.read(addr) ^ 0xFF;
        self.adc_value(value);
    }

    fn cmp_value(&mut self, first: u8, second: u8) {
        self.registers.set_flag_to(Flag::Carry, first >= second);
        let difference = first.wrapping_sub(second);
        self.check_value_set_negative(difference);
        self.check_value_set_zero(difference);
    }

    /// Compare
    pub(crate) fn cmp(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = self.read(addr);
        self.cmp_value(self.registers.accumulator, value);
    }

    /// Compare X register
    pub(crate) fn cpx(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = self.read(addr);
        self.cmp_value(self.registers.x, value);
    }

    /// Compare Y register
    pub(crate) fn cpy(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = self.read(addr);
        self.cmp_value(self.registers.y, value);
    }
}
