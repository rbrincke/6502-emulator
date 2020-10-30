use crate::processor::registers::{Registers, Flag};
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    pub(crate) fn read_two(&mut self, address_least_significant: u16, address_most_significant: u16) -> u16 {
        ((self.read(address_most_significant) as u16) << 8) | (self.read(address_least_significant) as u16)
    }

    pub(crate) fn read(&mut self, address: u16) -> u8 {
        self.cartridge.read(address)
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        self.cartridge.write(address, value)
    }
}
