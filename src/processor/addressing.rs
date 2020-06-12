use crate::cartridge::Cartridge;
use crate::processor::Core;

#[derive(Debug, Copy, Clone)]
pub enum AddressMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Address {
    Accumulator, Memory(u16)
}

impl<C : Cartridge> Core<C> {
    fn address_zero_page(&mut self) -> u16 {
        let immediate_address = self.address_immediate();
        self.read_raw(immediate_address) as u16
    }

    fn address_zero_page_offset(&mut self, offset: u8) -> u16 {
        let address = self.address_zero_page() + offset as u16;
        // Wrap around if sum exceeds 0xff.
        address % 0x100
    }

    fn address_zero_page_offset_x(&mut self) -> u16 {
        self.tick();
        self.address_zero_page_offset(self.registers.x)
    }

    fn address_zero_page_offset_y(&mut self) -> u16 {
        self.tick();
        self.address_zero_page_offset(self.registers.y)
    }

    // In immediate mode the value is embedded in the instruction
    // itself, so read the incremented program counter.
    fn address_immediate(&mut self) -> u16 {
        self.registers.program_counter += 1;
        self.registers.program_counter
    }

    pub(crate) fn address_relative(&mut self) -> u16 {
        let immediate_address = self.address_immediate();
        let value = self.read_raw(immediate_address) as i8;
        if value < 0 {
            -value as u16
        } else {
            value as u16
        }
    }

    /// Full 16-bit address.
    fn address_absolute(&mut self) -> u16 {
        let first = self.address_immediate();
        let second = self.address_immediate();
        self.read_two_raw(first, second)
    }

    /// Indirection.
    fn address_indirect(&mut self) -> u16 {
        let least_significant = self.address_absolute();
        self.read_two_raw(least_significant, least_significant + 1)
    }

    pub(crate) fn address(&mut self, address_mode: AddressMode) -> Address {
        match &address_mode {
            AddressMode::Accumulator => Address::Accumulator,
            AddressMode::Immediate => Address::Memory(self.address_immediate()),
            AddressMode::ZeroPage => Address::Memory(self.address_zero_page()),
            AddressMode::ZeroPageX => Address::Memory(self.address_zero_page_offset_x()),
            AddressMode::ZeroPageY => Address::Memory(self.address_zero_page_offset_y()),
            AddressMode::Relative => Address::Memory(self.address_relative()),
            AddressMode::Absolute => Address::Memory(self.address_absolute()),
            AddressMode::Indirect => Address::Memory(self.address_indirect()),
            _ => panic!("Unsupported address mode.")
        }
    }
}
