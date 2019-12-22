use crate::memory::Memory;
use crate::emulator::Emulator;

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

impl<C: Memory> Emulator<C> {
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
        let least = self.address_immediate();
        let most = self.address_immediate();
        self.read_two(least, most)
    }

    fn address_absolute_x(&mut self) -> u16 {
        self.address_absolute() + self.registers.x as u16
    }

    fn address_absolute_y(&mut self) -> u16 {
        self.address_absolute() + self.registers.y as u16
    }

    /// Indirection.
    fn address_indirect(&mut self) -> u16 {
        let least_significant = self.address_absolute();
        // Actually a bug in the original 6502.
        self.read_two(least_significant, (least_significant & 0xFF00) | ((least_significant + 1) % 0x100))
    }

    fn address_indexed_indirect(&mut self) -> u16 {
        let least_significant = self.address_zero_page_offset_x();
        let most_significant = (least_significant + 1) % 0x100;
        self.read_two(least_significant, most_significant)
    }

    fn address_indirect_indexed(&mut self) -> u16 {
        let least_significant = self.address_zero_page();
        let most_significant = (least_significant + 1) % 0x100;
        self.read_two(least_significant, most_significant) + self.registers.y as u16
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
    use crate::emulator::Emulator;
    use crate::memory::basic::DefaultMemory;
    use crate::emulator::registers::Registers;

    fn core() -> Emulator<DefaultMemory> {
        Emulator {
            registers: Registers::new(),
            memory: DefaultMemory::empty()
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
        c.memory.memory[0x600] = 24u8;

        assert_eq!(c.address_zero_page(), 24u16);
    }

    #[test]
    fn test_zero_page_x_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0x0f;
        c.memory.memory[0x600] = 0x80;

        assert_eq!(c.address_zero_page_offset_x(), 0x8f);
    }

    #[test]
    fn test_zero_page_x_positive_wrap() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0xff;
        c.memory.memory[0x600] = 0x80;

        assert_eq!(c.address_zero_page_offset_x(), 0x7f);
    }

    #[test]
    fn test_zero_page_x_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = -50i8 as u8;
        c.memory.memory[0x600] = 100u8;

        assert_eq!(c.address_zero_page_offset_x(), 50u16);
    }

    #[test]
    fn test_zero_page_y_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = 0x0f;
        c.memory.memory[0x600] = 0x80;

        assert_eq!(c.address_zero_page_offset_y(), 0x8f);
    }

    #[test]
    fn test_zero_page_y_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = -50i8 as u8;
        c.memory.memory[0x600] = 100u8;

        assert_eq!(c.address_zero_page_offset_y(), 50u16);
    }

    #[test]
    fn test_relative_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.memory.memory[0x600] = 0b10000000; // -128

        assert_eq!(c.address_relative(), 0b1111111110000000);
    }

    #[test]
    fn test_relative_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.memory.memory[0x600] = 0b01111111; // 127

        assert_eq!(c.address_relative(), 0b0000000001111111);
    }

    #[test]
    fn test_absolute() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.memory.memory[0x600] = 0x34;
        c.memory.memory[0x600 + 1] = 0x12;

        assert_eq!(c.address_absolute(), 0x1234);
    }

    #[test]
    fn test_absolute_x() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0x8;
        c.memory.memory[0x600] = 0x00;
        c.memory.memory[0x600 + 1] = 0x20;

        assert_eq!(c.address_absolute_x(), 0x2008);
    }

    #[test]
    fn test_absolute_y() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = 0x8;
        c.memory.memory[0x600] = 0x00;
        c.memory.memory[0x600 + 1] = 0x20;

        assert_eq!(c.address_absolute_y(), 0x2008);
    }

    #[test]
    fn test_indirect() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.memory.memory[0x600] = 0xfc;
        c.memory.memory[0x600 + 1] = 0xba;

        c.memory.memory[0xbafc] = 0xdd;
        c.memory.memory[0xbafc + 1] = 0xcc;

        assert_eq!(c.address_indirect(), 0xccdd);
    }

    #[test]
    fn test_indirect_bug() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.memory.memory[0x600] = 0xff;
        c.memory.memory[0x600 + 1] = 0x10;

        c.memory.memory[0x10ff] = 0xdd;
        c.memory.memory[0x1000] = 0xcc; // Chip bug: should actually be 10ff + 1 = 1100.

        assert_eq!(c.address_indirect(), 0xccdd);
    }

    #[test]
    fn test_indirect_indexed_positive() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.y = 0x5;
        c.memory.memory[0x600] = 0xa;

        c.memory.memory[0xa] = 0x30;
        c.memory.memory[0xa + 1] = 0x11;

        assert_eq!(c.address_indirect_indexed(), 0x1135);
    }

    #[test]
    fn test_indexed_indirect() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = 0x5;
        c.memory.memory[0x600] = 0xa;

        c.memory.memory[0xa + 0x5] = 0x30;
        c.memory.memory[0xa + 0x5 + 1] = 0x11;

        assert_eq!(c.address_indexed_indirect(), 0x1130);
    }

    #[test]
    fn test_indexed_indirect_negative() {
        let mut c = core();
        c.registers.program_counter = 0x600;
        c.registers.x = -5i8 as u8;
        c.memory.memory[0x600] = 0xa;

        c.memory.memory[0xa - 0x5] = 0x30;
        c.memory.memory[0xa - 0x5 + 1] = 0x11;

        assert_eq!(c.address_indexed_indirect(), 0x1130);
    }
}
