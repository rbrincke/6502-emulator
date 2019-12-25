use crate::processor::core::Core;
use crate::cartridge::Cartridge;

pub enum AddressMode {
    Implicit,
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

impl<C : Cartridge> Core<C> {
    pub(crate) fn address_zero_page(&mut self) -> u16 {
        let immediate_address = self.address_immediate();
        self.read(immediate_address) as u16
    }

    fn address_zero_page_offset(&mut self, offset: u8) -> u16 {
        let address = self.address_zero_page() + offset as u16;
        // Wrap around if sum exceeds 0xff.
        address % 0x100
    }

    pub(crate) fn address_zero_page_offset_x(&mut self) -> u16 {
        self.tick();
        self.address_zero_page_offset(self.registers.x)
    }

    pub(crate) fn address_zero_page_offset_y(&mut self) -> u16 {
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
    fn address_absolute(&mut self) -> u16 {
        let first = self.address_immediate();
        let second = self.address_immediate();
        self.read_two(first, second)
    }

    /// Indirection, used only by JMP.
    fn address_indirect(&mut self) -> u16 {
        let least_significant = self.address_absolute();
        self.read_two(least_significant, least_significant + 1)
    }

    pub(crate) fn address(&mut self, address_mode: AddressMode) -> u16 {
        match &address_mode {
            AddressMode::Immediate => self.address_immediate(),
            AddressMode::ZeroPage => self.address_zero_page(),
            AddressMode::ZeroPageX => self.address_zero_page_offset_x(),
            AddressMode::ZeroPageY => self.address_zero_page_offset_y(),
            AddressMode::Relative => self.address_relative(),
            AddressMode::Absolute => self.address_absolute(),
            AddressMode::Indirect => self.address_indirect(),
            _ => panic!("Unsupported address mode.")
        }
    }
}
