use std::ops::{Add, Sub};

use crate::memory::Memory;
use crate::emulator::addressing::AddressMode;
use crate::emulator::Emulator;

impl<C: Memory> Emulator<C> {
    fn inc_dec<F: Fn(u8, u8) -> u8>(&mut self, address_mode: AddressMode, apply: F) {
        let address = self.address(address_mode);
        let result = apply(self.read(address), 1);

        self.registers.status.update_zero_negative(result);
        self.write(address, result)
    }

    /// Increment.
    pub(crate) fn inc(&mut self, address_mode: AddressMode) {
        self.inc_dec(address_mode, u8::wrapping_add)
    }

    /// Decrement.
    pub(crate) fn dec(&mut self, address_mode: AddressMode) {
        self.inc_dec(address_mode, u8::wrapping_sub)
    }

    /// Increment X.
    pub(crate) fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.status.update_zero_negative(self.registers.x);
    }

    /// Increment Y.
    pub(crate) fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.registers.status.update_zero_negative(self.registers.y);
    }

    pub(crate) fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.status.update_zero_negative(self.registers.x);
    }

    pub(crate) fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.status.update_zero_negative(self.registers.y);
    }
}

#[cfg(test)]
mod test {
    use crate::memory::basic::DefaultMemory;
    use crate::emulator::addressing::AddressMode;
    use crate::emulator::Emulator;
    use crate::emulator::registers::{Flag, Registers};
    use crate::emulator::tests::{setup, TestAssertions, TestSetup};
    use crate::emulator::registers::Flag::{Zero, Negative};

    fn test_x<F: for<'r> Fn(&'r mut Emulator<DefaultMemory>) -> ()>(
        initial: u8,
        instruction: F,
        expected: u8,
        expected_flags_set: Vec<Flag>,
    ) {
        let mut t = setup(vec![]);

        t.registers.x = initial;
        instruction(&mut t);

        assert_eq!(expected, t.registers.x);
        t.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_inx() {
        test_x(0, Emulator::inx, 1, vec![])
    }

    #[test]
    fn test_inx_to_zero() {
        test_x(-1i8 as u8, Emulator::inx, 0, vec![Zero])
    }

    #[test]
    fn test_inx_wraparound() {
        test_x(127, Emulator::inx, -128i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dex() {
        test_x(-1i8 as u8, Emulator::dex, -2i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dex_to_negative() {
        test_x(0, Emulator::dex, -1i8 as u8, vec![Negative]);
    }

    #[test]
    fn test_dex_to_zero() {
        test_x(1, Emulator::dex, 0, vec![Zero])
    }

    #[test]
    fn test_dex_wraparound() {
        test_x(-128i8 as u8, Emulator::dex, 127, vec![])
    }

    fn test_y<F: for<'r> Fn(&'r mut Emulator<DefaultMemory>) -> ()>(
        initial: u8,
        instruction: F,
        expected: u8,
        expected_flags_set: Vec<Flag>,
    ) {
        let mut t = setup(vec![]);

        t.registers.y = initial;
        instruction(&mut t);

        assert_eq!(expected, t.registers.y);
        t.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_iny() {
        test_y(0, Emulator::iny, 1, vec![]);
    }

    #[test]
    fn test_iny_to_zero() {
        test_y(-1i8 as u8, Emulator::iny, 0, vec![Zero])
    }

    #[test]
    fn test_iny_wraparound() {
        test_y(127, Emulator::iny, -128i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dey() {
        test_y(-1i8 as u8, Emulator::dey, -2i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dey_to_negative() {
        test_y(0, Emulator::dey, -1i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dey_1() {
        test_y(1, Emulator::dey, 0 as u8, vec![Zero])
    }

    #[test]
    fn test_dey_wraparound() {
        test_y(-128i8 as u8, Emulator::dey, 127, vec![])
    }

    fn test<F: for<'r> Fn(&'r mut Emulator<DefaultMemory>, AddressMode) -> ()>(
        value: u8,
        instruction: F,
        expected: u8,
        expected_flags_set: Vec<Flag>,
    ) {
        let mut t = setup(vec![]);

        let memory_location = 0 as usize;
        t.memory.memory[memory_location] = value;
        instruction(&mut t, AddressMode::ZeroPage);

        assert_eq!(expected, t.memory.memory[memory_location]);
        t.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_inc() {
        test(0, Emulator::inc, 1, vec![])
    }

    #[test]
    fn test_inc_to_zero() {
        test(-1i8 as u8, Emulator::inc, 0, vec![Zero])
    }

    #[test]
    fn test_inc_wraparound() {
        test(127, Emulator::inc, -128i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dec() {
        test(-1i8 as u8, Emulator::dec, -2i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dec_to_negative() {
        test(0, Emulator::dec, -1i8 as u8, vec![Negative])
    }

    #[test]
    fn test_dec_to_zero() {
        test(1, Emulator::dec, 0, vec![Zero])
    }

    #[test]
    fn test_dec_overflow() {
        test(-128i8 as u8, Emulator::dec, 127, vec![])
    }
}
