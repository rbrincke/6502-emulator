use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    fn ld_set_flags(&mut self, address_mode: AddressMode) -> u8 {
        let address = self.address(address_mode);
        let value = self.read(address);

        self.check_value_set_zero_negative(value);

        value
    }

    /// Load accumulator.
    pub(crate) fn lda(&mut self, address_mode: AddressMode) {
        self.registers.accumulator = self.ld_set_flags(address_mode);
    }

    /// Load X.
    pub(crate) fn ldx(&mut self, address_mode: AddressMode) {
        self.registers.x = self.ld_set_flags(address_mode);
    }

    /// Load Y.
    pub(crate) fn ldy(&mut self, address_mode: AddressMode) {
        self.registers.y = self.ld_set_flags(address_mode);
    }

    fn st(&mut self, address_mode: AddressMode, value: u8) {
        let address = self.address(address_mode);
        self.write(address, value);
    }

    // Store accumulator.
    pub(crate) fn sta(&mut self, address_mode: AddressMode) {
        self.st(address_mode, self.registers.accumulator);
    }

    // Store X.
    pub(crate) fn stx(&mut self, address_mode: AddressMode) {
        self.st(address_mode, self.registers.x);
    }

    /// Store Y.
    pub(crate) fn sty(&mut self, address_mode: AddressMode) {
        self.st(address_mode, self.registers.y);
    }
}
