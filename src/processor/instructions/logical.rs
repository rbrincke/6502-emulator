use crate::processor::core::Core;
use crate::processor::addressing::AddressMode;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;

impl<C : Cartridge> Core<C> {
    /// Bit Test.
    pub(crate) fn bit(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let v = self.read(addr);

        let bit_and_acc_v = self.registers.accumulator & v;
        self.check_value_set_zero(bit_and_acc_v);

        self.registers.set_flag_to(Flag::Negative, (v & 0b10000000) != 0);
        self.registers.set_flag_to(Flag::Overflow, (v & 0b01000000) != 0);
    }
}
