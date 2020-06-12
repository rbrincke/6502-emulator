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

pub enum Access {
    Accumulator, Memory(u16)
}

impl Access {
    pub(crate) fn address(&self) -> Option<u16> {
        match *self {
            Access::Accumulator => None,
            Access::Memory(a) => Some(a)
        }
    }

    pub(crate) fn read<C : Cartridge>(&self, core: &mut Core<C>) -> u8 {
        match *self {
            Access::Accumulator => core.registers.accumulator,
            Access::Memory(a) => core.read(a)
        }
    }

    pub(crate) fn write<C : Cartridge>(&self, core: &mut Core<C>, value: u8) {
        match *self {
            Access::Accumulator => core.registers.accumulator = value,
            Access::Memory(a) => core.write(a, value),
        }
    }
}

impl<C : Cartridge> Core<C> {
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
    fn address_immediate(&mut self) -> u16 {
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

    fn read_accumulator(&self) -> u8 {
        self.registers.accumulator
    }

    fn write_accumulator(&mut self, value: u8) {
        self.registers.accumulator = value
    }

    pub(crate) fn address(&mut self, address_mode: AddressMode) -> Access {
        match &address_mode {
            AddressMode::Accumulator => Access::Accumulator,
            AddressMode::Immediate => Access::Memory(self.address_immediate()),
            AddressMode::ZeroPage => Access::Memory(self.address_zero_page()),
            AddressMode::ZeroPageX => Access::Memory(self.address_zero_page_offset_x()),
            AddressMode::ZeroPageY => Access::Memory(self.address_zero_page_offset_y()),
            AddressMode::Relative => Access::Memory(self.address_relative()),
            AddressMode::Absolute => Access::Memory(self.address_absolute()),
            AddressMode::Indirect => Access::Memory(self.address_indirect()),
            _ => panic!("Unsupported address mode.")
        }
    }
}
