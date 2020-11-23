use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use crate::emulator::addressing::AddressMode;
use crate::emulator::registers::Flag;
use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    fn and_eor_ora<F: for<'r> Fn(&'r mut u8, u8)>(&mut self, address_mode: AddressMode, apply: F) {
        let address = self.address(address_mode);
        let r = self.read(address);

        apply(&mut self.registers.accumulator, r);

        self.registers
            .status
            .update_zero_negative(self.registers.accumulator);
    }

    /// Logical AND.
    pub(crate) fn and(&mut self, address_mode: AddressMode) {
        self.and_eor_ora(address_mode, u8::bitand_assign);
    }

    /// Exclusive OR.
    pub(crate) fn eor(&mut self, address_mode: AddressMode) {
        self.and_eor_ora(address_mode, u8::bitxor_assign);
    }

    /// Inclusive OR.
    pub(crate) fn ora(&mut self, address_mode: AddressMode) {
        self.and_eor_ora(address_mode, u8::bitor_assign);
    }

    /// Bit Test.
    pub(crate) fn bit(&mut self, address_mode: AddressMode) {
        let address = self.address(address_mode);
        let value = self.read(address);

        let bit_and_acc_v = self.registers.accumulator & value;
        self.registers.status.update_zero(bit_and_acc_v);

        self.registers
            .status
            .set_to(Flag::Negative, (value & 0b10000000) != 0);
        self.registers
            .status
            .set_to(Flag::Overflow, (value & 0b01000000) != 0);
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::addressing::AddressMode;
    use crate::emulator::registers::Flag;
    use crate::emulator::registers::Flag::{Negative, Overflow, Zero};
    use crate::emulator::tests::{setup, TestAssertions};

    #[test]
    fn test_and() {
        let mut c = setup(vec![]);
        c.registers.accumulator = 0b10101010;

        c.memory.memory[c.registers.program_counter as usize] = 0b01010101;
        c.and(AddressMode::Immediate);

        assert_eq!(0b00000000, c.registers.accumulator);
        c.assert_flags_set(vec![Zero]);
    }

    #[test]
    fn test_eor() {
        let mut c = setup(vec![]);
        c.registers.accumulator = 0b10101010;

        c.memory.memory[c.registers.program_counter as usize] = 0b01010101;
        c.eor(AddressMode::Immediate);

        assert_eq!(0b11111111, c.registers.accumulator);
        c.assert_flags_set(vec![Negative]);
    }

    #[test]
    fn test_ora() {
        let mut c = setup(vec![]);
        c.registers.accumulator = 0b11000011;

        c.memory.memory[c.registers.program_counter as usize] = 0b01000101;
        c.ora(AddressMode::Immediate);

        assert_eq!(0b11000111, c.registers.accumulator);
        c.assert_flags_set(vec![Negative]);
    }

    fn test_bit(accumulator: u8, memory: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);

        c.registers.accumulator = accumulator;
        c.memory.memory[0x0] = memory;

        c.bit(AddressMode::ZeroPage);

        c.assert_flags_set(expected_flags_set)
    }

    #[test]
    fn test_bit_zero() {
        test_bit(0, 0xFF, vec![Zero, Overflow, Negative])
    }

    #[test]
    fn test_bit_negative() {
        test_bit(0xF0, 0b10000000, vec![Negative])
    }

    #[test]
    fn test_bit_negative_zero() {
        test_bit(0x01, 0b10000000, vec![Zero, Negative])
    }

    #[test]
    fn test_bit_overflow() {
        test_bit(0xF0, 0b01000000, vec![Overflow])
    }

    #[test]
    fn test_bit_overflow_zero() {
        test_bit(0x01, 0b01000000, vec![Overflow, Zero])
    }
}
