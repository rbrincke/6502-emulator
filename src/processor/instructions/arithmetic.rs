use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    fn adc_value(&mut self, value: u8) {
        let carry = self.registers.get_flag(Flag::Carry) as u16;

        // Split into low and high.
        let mut result_low = (self.registers.accumulator as u16 & 0x0F) + (value as u16 & 0x0F) + carry;
        let mut result_high = (self.registers.accumulator as u16 & 0xF0) + (value as u16 & 0xF0);

        // Correct values for BCD.
        if self.registers.get_flag(Flag::Decimal) {
            if result_low > 0x09 {
                result_low = (result_low + 0x06) & 0x0F;
                // Intermediate carry.
                result_high += 0x10;
            }

            if result_high > 0x90 {
                result_high += 0x60;
            }
        }

        let result_intermediate = result_low + result_high;

        self.check_value_set_carry(result_intermediate);
        self.check_value_set_overflow(self.registers.accumulator, value, result_intermediate);

        let result = (result_intermediate & 0xFF) as u8;
        self.registers.accumulator = result;

        self.check_value_set_zero_negative(result);
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
        let mut value = self.read(addr) ^ 0xFF;
        if self.registers.get_flag(Flag::Decimal) {
            // Use 9s complement.
            value -= 0x66;
        }

        self.adc_value(value);
    }

    fn cmp_value(&mut self, address_mode: AddressMode, register_value: u8) {
        let addr = self.address(address_mode);
        let value = self.read(addr);

        self.registers.set_flag_to(Flag::Carry, register_value >= value);
        let difference = register_value.wrapping_sub(value);

        self.check_value_set_zero_negative(difference);
    }

    /// Compare accumulator.
    pub(crate) fn cmp(&mut self, address_mode: AddressMode) {
        self.cmp_value(address_mode, self.registers.accumulator);
    }

    /// Compare X register.
    pub(crate) fn cpx(&mut self, address_mode: AddressMode) {
        self.cmp_value(address_mode, self.registers.x);
    }

    /// Compare Y register.
    pub(crate) fn cpy(&mut self, address_mode: AddressMode) {
        self.cmp_value(address_mode, self.registers.y);
    }
}
