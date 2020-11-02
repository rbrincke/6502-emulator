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
        self.address_zero_page_offset(self.registers.x)
    }

    fn address_zero_page_offset_y(&mut self) -> u16 {
        self.address_zero_page_offset(self.registers.y)
    }

    // In immediate mode the value is embedded in the instruction
    // itself, so read the program counter (and increment).
    pub(crate) fn address_immediate(&mut self) -> u16 {
        let v = self.registers.program_counter;
        self.registers.program_counter += 1;
        v
    }

    pub(crate) fn address_relative(&mut self) -> u16 {
        let immediate_address = self.address_immediate();
        self.read(immediate_address) as i8 as u16
    }

    /// Full 16-bit address.
    pub(crate) fn address_absolute(&mut self) -> u16 {
        let first = self.address_immediate();
        let second = self.address_immediate();
        self.read_two(first, second)
    }

    fn address_absolute_x(&mut self) -> u16 {
        self.address_absolute().wrapping_add(self.registers.x as i8 as u16)
    }

    fn address_absolute_y(&mut self) -> u16 {
        self.address_absolute().wrapping_add(self.registers.y as i8 as u16)
    }

    /// Indirection.
    fn address_indirect(&mut self) -> u16 {
        let least_significant = self.address_absolute();
        self.read_two(least_significant, least_significant + 1)
    }

    fn address_indexed_indirect(&mut self) -> u16 {
        let least_significant = self.address_zero_page_offset_x();
        let most_significant = least_significant.wrapping_add(1) % 0x100;
        self.read_two(least_significant, most_significant)
    }

    fn address_indirect_indexed(&mut self) -> u16 {
        let least_significant = self.address_zero_page();
        let most_significant = least_significant.wrapping_add(1) % 0x100;
        self.read_two(least_significant, most_significant).wrapping_add(self.registers.y as i8 as u16)
    }

    pub(crate) fn address(&mut self, address_mode: AddressMode) -> u16 {
        match address_mode {
            AddressMode::Immediate => self.address_immediate(),
            AddressMode::ZeroPage => self.address_zero_page(),
            AddressMode::ZeroPageX => self.address_zero_page_offset_x(),
            AddressMode::ZeroPageY => self.address_zero_page_offset_y(),
            AddressMode::Relative => self.address_relative(),
            AddressMode::Absolute => self.address_absolute(),
            AddressMode::AbsoluteX => self.address_absolute_x(),
            AddressMode::AbsoluteY => self.address_absolute_y(),
            AddressMode::Indirect => self.address_indirect(),
            AddressMode::IndexedIndirect => self.address_indexed_indirect(),
            AddressMode::IndirectIndexed => self.address_indirect_indexed()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::Core;
    use crate::cartridge::basic::BasicCartridge;
    use crate::processor::registers::Registers;

    fn core() -> Core<BasicCartridge> {
        Core {
            registers: Registers::new(),
            cartridge: BasicCartridge::empty()
        }
    }

    #[test]
    fn test_immediate() {
        let mut c = core();
        c.registers.program_counter = 0x600;

        assert_eq!(c.address_immediate(), 0x600);
    }

    #[test]
    fn test_zero_page() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.cartridge.memory[0x600] = 24u8;

        assert_eq!(c.address_zero_page(), 24u16);
    }

    #[test]
    fn test_zero_page_x_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0x0f;
        c.cartridge.memory[0x600] = 0x80;

        assert_eq!(c.address_zero_page_offset_x(), 0x8f);
    }

    #[test]
    fn test_zero_page_x_positive_wrap() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0xff;
        c.cartridge.memory[0x600] = 0x80;

        assert_eq!(c.address_zero_page_offset_x(), 0x7f);
    }

    #[test]
    fn test_zero_page_x_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = -50i8 as u8;
        c.cartridge.memory[0x600] = 100u8;

        assert_eq!(c.address_zero_page_offset_x(), 50u16);
    }

    #[test]
    fn test_zero_page_y_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = 0x0f;
        c.cartridge.memory[0x600] = 0x80;

        assert_eq!(c.address_zero_page_offset_y(), 0x8f);
    }

    #[test]
    fn test_zero_page_y_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = -50i8 as u8;
        c.cartridge.memory[0x600] = 100u8;

        assert_eq!(c.address_zero_page_offset_y(), 50u16);
    }

    #[test]
    fn test_relative_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.cartridge.memory[0x600] = 0b10000000; // -128

        assert_eq!(c.address_relative(), 0b1111111110000000);
    }

    #[test]
    fn test_relative_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.cartridge.memory[0x600] = 0b01111111; // 127

        assert_eq!(c.address_relative(), 0b0000000001111111);
    }

    #[test]
    fn test_absolute() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.cartridge.memory[0x600] = 0x34;
        c.cartridge.memory[0x600 + 1] = 0x12;

        assert_eq!(c.address_absolute(), 0x1234);
    }

    #[test]
    fn test_absolute_x_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0x8;
        c.cartridge.memory[0x600] = 0x00;
        c.cartridge.memory[0x600 + 1] = 0x20;

        assert_eq!(c.address_absolute_x(), 0x2008);
    }

    #[test]
    fn test_absolute_x_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = -128i8 as u8;
        c.cartridge.memory[0x600] = 0x00;
        c.cartridge.memory[0x600 + 1] = 0x20; // 0x2000 equals 8192

        assert_eq!(c.address_absolute_x(), 8064);
    }

    #[test]
    fn test_absolute_y_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = 0x8;
        c.cartridge.memory[0x600] = 0x00;
        c.cartridge.memory[0x600 + 1] = 0x20;

        assert_eq!(c.address_absolute_y(), 0x2008);
    }

    #[test]
    fn test_absolute_y_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = -128i8 as u8;
        c.cartridge.memory[0x600] = 0x00;
        c.cartridge.memory[0x600 + 1] = 0x20; // 0x2000 equals 8192

        assert_eq!(c.address_absolute_y(), 8064);
    }

    #[test]
    fn test_indirect() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.cartridge.memory[0x600] = 0xfc;
        c.cartridge.memory[0x600 + 1] = 0xba;

        c.cartridge.memory[0xbafc] = 0xdd;
        c.cartridge.memory[0xbafc + 1] = 0xcc;

        assert_eq!(c.address_indirect(), 0xccdd);
    }

    #[test]
    fn test_indirect_indexed_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = 0x5;
        c.cartridge.memory[0x600] = 0xa;

        c.cartridge.memory[0xa] = 0x30;
        c.cartridge.memory[0xa + 1] = 0x11;

        assert_eq!(c.address_indirect_indexed(), 0x1135);
    }

    #[test]
    fn test_indirect_indexed_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = -5i8 as u8;
        c.cartridge.memory[0x600] = 0xa;

        c.cartridge.memory[0xa] = 0x30;
        c.cartridge.memory[0xa + 1] = 0x11;

        assert_eq!(c.address_indirect_indexed(), 0x112b);
    }

    #[test]
    fn test_indexed_indirect_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0x5;
        c.cartridge.memory[0x600] = 0xa;

        c.cartridge.memory[0xa + 0x5] = 0x30;
        c.cartridge.memory[0xa + 0x5 + 1] = 0x11;

        assert_eq!(c.address_indexed_indirect(), 0x1130);
    }

    #[test]
    fn test_indexed_indirect_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = -5i8 as u8;
        c.cartridge.memory[0x600] = 0xa;

        c.cartridge.memory[0xa - 0x5] = 0x30;
        c.cartridge.memory[0xa - 0x5 + 1] = 0x11;

        assert_eq!(c.address_indexed_indirect(), 0x1130);
    }
}
