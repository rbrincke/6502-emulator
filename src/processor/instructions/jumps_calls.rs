use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    /// Jump.
    pub(crate) fn jmp(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        self.registers.program_counter = address;
    }
}
