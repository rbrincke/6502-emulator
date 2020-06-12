use crate::processor::addressing::{AddressMode, Address};
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    /// Jump.
    pub(crate) fn jmp(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let address_value = match address {
            Address::Memory(a) => a,
            _ => panic!("Attempt to JMP with address mode {:?}", address_mode)
        };

        self.registers.program_counter = address_value;
    }
}
