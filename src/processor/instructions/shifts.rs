use crate::cartridge::Cartridge;
use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    /// Arithmetic shift left.
    fn asl(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let value = self.read_fetched(address);

        let result_intermediate = (value as u16) << 1;
        self.check_value_set_carry(result_intermediate);
        let result = (value & 0xFF) as u8;

        self.check_value_set_zero(result);
        self.check_value_set_negative(result);

        self.write_fetched(address, result);
    }

    /// Logical shift right.
    fn lsr(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let value = self.read_fetched(address);

        let new_carry_flag_value = value & 0b10000000 == 0;
        let result = value >> 1;
        self.registers.set_flag_to(Flag::Carry, new_carry_flag_value);

        self.check_value_set_zero(result);
        self.check_value_set_negative(result);

        self.write_fetched(address, result);
    }

    /// Rotate left.
    fn rol(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let value = self.read_fetched(address);

        let new_carry_flag_value = value & 0b10000000 == 0;
        let result = value << 1 | self.registers.get_flag(Flag::Carry) as u8;
        self.registers.set_flag_to(Flag::Carry, new_carry_flag_value);

        self.check_value_set_zero(result);
        self.check_value_set_negative(result);

        self.write_fetched(address, result);
    }

    /// Rotate right.
    fn ror(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let value = self.read_fetched(address);

        let new_carry_flag_value = value & 0b00000001 == 0;
        let result = value >> 1 | (self.registers.get_flag(Flag::Carry) as u8) << 7;
        self.registers.set_flag_to(Flag::Carry, new_carry_flag_value);

        self.check_value_set_zero(result);
        self.check_value_set_negative(result);

        self.write_fetched(address, result);
    }
}
