use crate::cartridge::Cartridge;
use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    fn shift_accumulator<F : Fn(&mut Self, u8) -> u8>(&mut self, shift: F) {
        let value = self.registers.accumulator;
        let result = shift(self, value);
        self.registers.accumulator = result;
    }

    fn shift_address<F : Fn(&mut Self, u8) -> u8>(&mut self, address_mode: AddressMode, shift: F) {
        let address = self.address(address_mode);
        let value = self.read(address);
        let result = shift(self, value);
        self.write(address, result);
    }

    fn asl(&mut self, value: u8) -> u8 {
        let result_intermediate = (value as u16) << 1;
        self.check_value_set_carry(result_intermediate);
        let result = (result_intermediate & 0xFF) as u8;

        self.check_value_set_zero_negative(result);

        result
    }

    /// Arithmetic shift left on accumulator.
    pub(crate) fn asl_acc(&mut self) {
        self.shift_accumulator(Core::asl);
    }

    /// Arithmetic shift left.
    pub(crate) fn asl_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Core::asl);
    }

    fn lsr(&mut self, value: u8) -> u8 {
        let new_carry_flag_value = (value & 0b00000001) != 0;
        let result = value >> 1;
        self.registers.set_flag_to(Flag::Carry, new_carry_flag_value);

        self.check_value_set_zero_negative(result);

        result
    }

    /// Logical shift right on accumulator.
    pub(crate) fn lsr_acc(&mut self) {
        self.shift_accumulator(Core::lsr);
    }

    /// Logical shift right.
    pub(crate) fn lsr_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Core::lsr);
    }

    fn rol(&mut self, value: u8) -> u8 {
        let new_carry_flag_value = value & 0b10000000 != 0;
        let result = value << 1 | self.registers.get_flag(Flag::Carry) as u8;
        self.registers.set_flag_to(Flag::Carry, new_carry_flag_value);

        self.check_value_set_zero_negative(result);

        result
    }

    /// Rotate left on accumulator.
    pub(crate) fn rol_acc(&mut self) {
        self.shift_accumulator(Core::rol);
    }

    /// Rotate left.
    pub(crate) fn rol_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Core::rol);
    }

    fn ror(&mut self, value: u8) -> u8 {
        let new_carry_flag_value = value & 0b00000001 != 0;
        let result = value >> 1 | (self.registers.get_flag(Flag::Carry) as u8) << 7;
        self.registers.set_flag_to(Flag::Carry, new_carry_flag_value);

        self.check_value_set_zero_negative(result);

        result
    }

    /// Rotate right on accumulator.
    pub(crate) fn ror_acc(&mut self) {
        self.shift_accumulator(Core::ror);
    }

    /// Rotate right.
    pub(crate) fn ror_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Core::ror);
    }
}
