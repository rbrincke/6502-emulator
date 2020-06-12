use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    fn inc_dec<F : Fn(u8) -> u8>(&mut self, address_mode: AddressMode, f: F) {
        let address = self.address(address_mode);
        let result = f(self.read_raw(address));

        self.check_value_set_zero(result);
        self.check_value_set_negative(result);

        self.write_raw(address, result)
    }

    /// Increment.
    pub(crate) fn inc(&mut self, address_mode: AddressMode) {
        self.inc_dec(address_mode, |v| v + 1)
    }

    /// Increment X.
    pub(crate) fn inx(&mut self) {
        self.registers.x += 1;

        self.check_value_set_zero(self.registers.x);
        self.check_value_set_negative(self.registers.x);
    }

    /// Increment Y.
    pub(crate) fn iny(&mut self) {
        self.registers.y += 1;

        self.check_value_set_zero(self.registers.y);
        self.check_value_set_negative(self.registers.y);
    }

    /// Decrement.
    pub(crate) fn dec(&mut self, address_mode: AddressMode) {
        self.inc_dec(address_mode, |v| v - 1)
    }

    pub(crate) fn dex(&mut self) {
        self.registers.x -= 1;

        self.check_value_set_zero(self.registers.x);
        self.check_value_set_negative(self.registers.x);
    }

    pub(crate) fn dey(&mut self) {
        self.registers.y -= 1;

        self.check_value_set_zero(self.registers.y);
        self.check_value_set_negative(self.registers.y);
    }
}
