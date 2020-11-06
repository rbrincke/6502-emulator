use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    /// Jump.
    pub(crate) fn jmp(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        self.registers.program_counter = address;
    }

    pub(crate) fn jsr(&mut self) {
        self.push_pc();
        self.registers.program_counter = self.address_absolute();
    }

    pub(crate) fn rts(&mut self) {
        let least_significant = self.pop() as u16;
        let most_significant = self.pop() as u16;
        self.registers.program_counter = ((most_significant << 8) | least_significant) + 1;
    }
}
