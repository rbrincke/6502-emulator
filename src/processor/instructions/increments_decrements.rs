use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    fn inc_dec<F : Fn(u8, u8) -> u8>(&mut self, address_mode: AddressMode, apply: F) {
        let address = self.address(address_mode);
        let result = apply(self.read(address), 1);

        self.check_value_set_zero_negative(result);
        self.write(address, result)
    }

    /// Increment.
    pub(crate) fn inc(&mut self, address_mode: AddressMode) {
        self.inc_dec(address_mode, u8::wrapping_add)
    }

    /// Decrement.
    pub(crate) fn dec(&mut self, address_mode: AddressMode) {
        self.inc_dec(address_mode, u8::wrapping_sub)
    }

    /// Increment X.
    pub(crate) fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.check_value_set_zero_negative(self.registers.x);
    }

    /// Increment Y.
    pub(crate) fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.check_value_set_zero_negative(self.registers.y);
    }

    pub(crate) fn dex(&mut self) {
        println!("dex");
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.check_value_set_zero_negative(self.registers.x);
    }

    pub(crate) fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.check_value_set_zero_negative(self.registers.y);
    }
}
