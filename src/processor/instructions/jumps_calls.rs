use crate::processor::core::Core;
use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;

impl<C : Cartridge> Core<C> {
    /// Jump.
    pub(crate) fn jmp(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        self.registers.program_counter = addr;
    }
}
