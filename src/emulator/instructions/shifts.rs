use crate::emulator::addressing::AddressMode;
use crate::emulator::registers::Flag;
use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    fn shift_accumulator<F: Fn(&mut Self, u8) -> u8>(&mut self, shift: F) {
        let value = self.registers.accumulator;
        let result = shift(self, value);
        self.registers.accumulator = result;
    }

    fn shift_address<F: Fn(&mut Self, u8) -> u8>(&mut self, address_mode: AddressMode, shift: F) {
        let address = self.address(address_mode);
        let value = self.read(address);
        let result = shift(self, value);
        self.write(address, result);
    }

    fn asl(&mut self, value: u8) -> u8 {
        let result_intermediate = (value as u16) << 1;
        self.registers.status.update_carry(result_intermediate);
        let result = (result_intermediate & 0xFF) as u8;

        self.registers.status.update_zero_negative(result);

        result
    }

    /// Arithmetic shift left on accumulator.
    pub(crate) fn asl_acc(&mut self) {
        self.shift_accumulator(Emulator::asl);
    }

    /// Arithmetic shift left.
    pub(crate) fn asl_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Emulator::asl);
    }

    fn lsr(&mut self, value: u8) -> u8 {
        let new_carry_flag_value = (value & 0b00000001) != 0;
        let result = value >> 1;
        self.registers
            .status
            .set_to(Flag::Carry, new_carry_flag_value);

        self.registers.status.update_zero_negative(result);

        result
    }

    /// Logical shift right on accumulator.
    pub(crate) fn lsr_acc(&mut self) {
        self.shift_accumulator(Emulator::lsr);
    }

    /// Logical shift right.
    pub(crate) fn lsr_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Emulator::lsr);
    }

    fn rol(&mut self, value: u8) -> u8 {
        let new_carry_flag_value = value & 0b10000000 != 0;
        let result = value << 1 | self.registers.status.get(Flag::Carry) as u8;
        self.registers
            .status
            .set_to(Flag::Carry, new_carry_flag_value);

        self.registers.status.update_zero_negative(result);

        result
    }

    /// Rotate left on accumulator.
    pub(crate) fn rol_acc(&mut self) {
        self.shift_accumulator(Emulator::rol);
    }

    /// Rotate left.
    pub(crate) fn rol_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Emulator::rol);
    }

    fn ror(&mut self, value: u8) -> u8 {
        let new_carry_flag_value = value & 0b00000001 != 0;
        let result = value >> 1 | (self.registers.status.get(Flag::Carry) as u8) << 7;
        self.registers
            .status
            .set_to(Flag::Carry, new_carry_flag_value);

        self.registers.status.update_zero_negative(result);

        result
    }

    /// Rotate right on accumulator.
    pub(crate) fn ror_acc(&mut self) {
        self.shift_accumulator(Emulator::ror);
    }

    /// Rotate right.
    pub(crate) fn ror_mem(&mut self, address_mode: AddressMode) {
        self.shift_address(address_mode, Emulator::ror);
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::registers::Flag;
    use crate::emulator::registers::Flag::{Carry, Negative};
    use crate::emulator::tests::{setup, Instruction, TestAssertions};
    use crate::emulator::Emulator;

    fn test_shift(
        flags: Vec<Flag>,
        input: u8,
        instruction: Instruction,
        expected: u8,
        expected_flags_set: Vec<Flag>,
    ) {
        let mut c = setup(flags);

        c.registers.accumulator = input;
        instruction(&mut c);

        assert_eq!(expected, c.registers.accumulator);
        c.assert_flags_set(expected_flags_set)
    }

    #[test]
    fn test_asl() {
        test_shift(
            vec![],
            0b10001111,
            Emulator::asl_acc,
            0b00011110,
            vec![Carry],
        );
    }

    #[test]
    fn test_lsr() {
        test_shift(
            vec![],
            0b10001111,
            Emulator::lsr_acc,
            0b01000111,
            vec![Carry],
        )
    }

    #[test]
    fn test_rol_carry_clear() {
        test_shift(
            vec![],
            0b10001111,
            Emulator::rol_acc,
            0b00011110,
            vec![Carry],
        )
    }

    #[test]
    fn test_rol_carry_set() {
        test_shift(
            vec![Carry],
            0b10001111,
            Emulator::rol_acc,
            0b00011111,
            vec![Carry],
        )
    }

    #[test]
    fn test_ror_carry_clear() {
        test_shift(
            vec![],
            0b10001111,
            Emulator::ror_acc,
            0b01000111,
            vec![Carry],
        )
    }

    #[test]
    fn test_ror_carry_set() {
        test_shift(
            vec![Carry],
            0b10001111,
            Emulator::ror_acc,
            0b11000111,
            vec![Carry, Negative],
        )
    }
}
