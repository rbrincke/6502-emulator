use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;
use crate::processor::Core;
use std::ops::{BitAndAssign, BitXorAssign, BitOrAssign};

impl<C : Cartridge> Core<C> {
    fn and_eor_ora<F : for<'r> Fn(&'r mut u8, u8) -> ()>(&mut self, address_mode: AddressMode, apply: F) {
        let address = self.address(address_mode);
        let r = self.read(address);

        apply(&mut self.registers.accumulator, r);

        self.check_value_set_zero_negative(self.registers.accumulator);
    }

    /// Logical AND.
    pub(crate) fn and(&mut self, address_mode: AddressMode) {
        self.and_eor_ora(address_mode, u8::bitand_assign);
    }

    /// Exclusive OR.
    pub(crate) fn eor(&mut self, address_mode: AddressMode) {
        self.and_eor_ora(address_mode, u8::bitxor_assign);
    }

    /// Inclusive OR.
    pub(crate) fn ora(&mut self, address_mode: AddressMode) {
        self.and_eor_ora(address_mode, u8::bitor_assign);
    }

    /// Bit Test.
    pub(crate) fn bit(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let value = self.read(address);

        let bit_and_acc_v = self.registers.accumulator & value;
        self.check_value_set_zero(bit_and_acc_v);

        self.registers.set_flag_to(Flag::Negative, (value & 0b10000000) != 0);
        self.registers.set_flag_to(Flag::Overflow, (value & 0b01000000) != 0);
    }
}
