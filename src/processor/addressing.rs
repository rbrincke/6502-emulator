use crate::cartridge::Cartridge;
use crate::processor::Core;

#[derive(Debug, Copy, Clone)]
pub enum AddressMode {
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

impl<C: Cartridge> Core<C> {
    fn address_zero_page(&mut self) -> u16 {
        let immediate_address = self.address_immediate();
        self.read(immediate_address) as u16
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
    pub(crate) fn address_immediate(&mut self) -> u16 {
        self.registers.program_counter += 1;
        self.registers.program_counter
    }

    pub(crate) fn address_relative(&mut self) -> u16 {
        let immediate_address = self.address_immediate();
        let value = self.read(immediate_address) as i8;
        if value < 0 {
            -value as u16
        } else {
            value as u16
        }
    }

    /// Full 16-bit address.
    pub(crate) fn address_absolute(&mut self) -> u16 {
        let first = self.address_immediate();
        let second = self.address_immediate();
        self.read_two(first, second)
    }

    fn address_absolute_x(&mut self) -> u16 {
        self.address_absolute() + self.registers.x as u16
    }

    fn address_absolute_y(&mut self) -> u16 {
        self.address_absolute() + self.registers.y as u16
    }

    fn address_indexed_indirect(&mut self) -> u16 {
        let least_significant = self.address_zero_page_offset_x();
        let most_significant = (least_significant + 1) % 0x100;
        self.read_two(least_significant, most_significant)
    }

    fn address_indirect_indexed(&mut self) -> u16 {
        let least_significant = self.address_immediate();
        let most_significant = (least_significant + 1) % 0x100;
        self.read_two(least_significant, most_significant) + self.registers.y as u16
    }

    /// Indirection.
    fn address_indirect(&mut self) -> u16 {
        let least_significant = self.address_absolute();
        self.read_two(least_significant, least_significant + 1)
    }

    pub(crate) fn address(&mut self, address_mode: AddressMode) -> u16 {
        match address_mode {
            AddressMode::Immediate => self.address_immediate(),
            AddressMode::ZeroPage => self.address_zero_page(),
            AddressMode::ZeroPageX => self.address_zero_page_offset_x(),
            AddressMode::ZeroPageY => self.address_zero_page_offset_y(),
            AddressMode::Relative => self.address_relative(),
            AddressMode::Absolute => self.address_absolute(),
            AddressMode::Indirect => self.address_indirect(),
            AddressMode::AbsoluteX => self.address_absolute_x(),
            AddressMode::AbsoluteY => self.address_absolute_y(),
            AddressMode::IndexedIndirect => self.address_indexed_indirect(),
            AddressMode::IndirectIndexed => self.address_indirect_indexed()
        }
    }
}
