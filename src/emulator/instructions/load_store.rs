use crate::memory::Memory;
use crate::emulator::addressing::AddressMode;
use crate::emulator::Emulator;

impl<C : Memory> Emulator<C> {
    fn load(&mut self, address_mode: AddressMode) -> u8 {
        let address = self.address(address_mode);
        let value = self.read(address);

        self.registers.status.update_zero_negative(value);

        value
    }

    /// Load accumulator.
    pub(crate) fn lda(&mut self, address_mode: AddressMode) {
        self.registers.accumulator = self.load(address_mode);
    }

    /// Load X.
    pub(crate) fn ldx(&mut self, address_mode: AddressMode) {
        self.registers.x = self.load(address_mode);
    }

    /// Load Y.
    pub(crate) fn ldy(&mut self, address_mode: AddressMode) {
        self.registers.y = self.load(address_mode);
    }

    fn store(&mut self, address_mode: AddressMode, value: u8) {
        let address = self.address(address_mode);
        self.write(address, value);
    }

    // Store accumulator.
    pub(crate) fn sta(&mut self, address_mode: AddressMode) {
        self.store(address_mode, self.registers.accumulator);
    }

    // Store X.
    pub(crate) fn stx(&mut self, address_mode: AddressMode) {
        self.store(address_mode, self.registers.x);
    }

    /// Store Y.
    pub(crate) fn sty(&mut self, address_mode: AddressMode) {
        self.store(address_mode, self.registers.y);
    }
}

#[cfg(test)]
mod test {
    use crate::memory::basic::DefaultMemory;
    use crate::emulator::addressing::AddressMode;
    use crate::emulator::Emulator;
    use crate::emulator::registers::{Flag, Registers};
    use crate::emulator::registers::Flag::{Negative, Zero};
    use crate::emulator::tests::*;

    const TEST_VALUE: u8 = 0b10101010;

    fn test_load(
        instruction: AddressInstruction,
        load_target: RegisterRead,
        value: u8,
        expected_flags_set: Vec<Flag>
    ) {
        let mut c = setup(vec![]);

        c.memory.memory[c.registers.program_counter as usize] = value;
        instruction(&mut c, AddressMode::Immediate);

        assert_eq!(value, load_target(&c.registers));
        c.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_lda_zero() {
        test_load(Emulator::lda, READ_ACCUMULATOR, 0, vec![Zero]);
    }

    #[test]
    fn test_lda_positive() {
        test_load(Emulator::lda, READ_ACCUMULATOR, 1, vec![]);
    }

    #[test]
    fn test_lda_negative() {
        test_load(Emulator::lda, READ_ACCUMULATOR, -1i8 as u8, vec![Negative]);
    }

    #[test]
    fn test_ldx_zero() {
        test_load(Emulator::ldx, READ_X, 0, vec![Zero]);
    }

    #[test]
    fn test_ldx_positive() {
        test_load(Emulator::ldx, READ_X, 1, vec![]);
    }

    #[test]
    fn test_ldx_negative() {
        test_load(Emulator::ldx, READ_X, -1i8 as u8, vec![Negative]);
    }

    #[test]
    fn test_ldy_zero() {
        test_load(Emulator::ldy, READ_Y, 0, vec![Zero]);
    }

    #[test]
    fn test_ldy_positive() {
        test_load(Emulator::ldy, READ_Y, 1, vec![]);
    }

    #[test]
    fn test_ldy_negative() {
        test_load(Emulator::ldy, READ_Y, -1i8 as u8, vec![Negative]);
    }

    fn test_store(
        source_register: RegisterWrite,
        instruction: AddressInstruction
    ) {
        let mut c = setup(vec![]);

        source_register(&mut c.registers, TEST_VALUE);
        c.memory.memory[c.registers.program_counter as usize] = 0x1;
        instruction(&mut c, AddressMode::ZeroPage);

        assert_eq!(TEST_VALUE, c.memory.memory[0x1]);
        c.assert_flags_set(vec![]);
    }

    #[test]
    fn test_sta() {
        test_store(WRITE_ACCUMULATOR, Emulator::sta);
    }

    #[test]
    fn test_stx() {
        test_store(WRITE_X, Emulator::stx);
    }

    #[test]
    fn test_sty() {
        test_store(WRITE_Y, Emulator::sty);
    }
}
