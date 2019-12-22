use crate::memory::Memory;
use crate::emulator::addressing::AddressMode;
use crate::emulator::Emulator;
use crate::emulator::registers::Flag;

impl<C : Memory> Emulator<C> {
    fn adc_value(&mut self, value: u8) {
        let carry = self.registers.status.get(Flag::Carry) as u16;

        // Split into least and most significant.
        let mut result_least_significant = (self.registers.accumulator as u16 & 0x0F) + (value as u16 & 0x0F) + carry;
        let mut result_most_significant = (self.registers.accumulator as u16 & 0xF0) + (value as u16 & 0xF0);

        // Correct values for BCD.
        if self.registers.status.get(Flag::Decimal) {
            if result_least_significant > 0x09 {
                result_least_significant = (result_least_significant + 0x06) & 0x0F;
                result_most_significant += 0x10; // Intermediate carry.
            }

            if result_most_significant > 0x90 {
                result_most_significant += 0x60;
            }
        }

        let result_intermediate = result_least_significant + result_most_significant;

        self.registers.status.update_carry(result_intermediate);
        self.registers.status.update_overflow(self.registers.accumulator, value, result_intermediate);

        let result = (result_intermediate & 0xFF) as u8;
        self.registers.accumulator = result;

        self.registers.status.update_zero_negative(result);
    }

    /// Add with carry.
    pub(crate) fn adc(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let value = self.read(addr);

        self.adc_value(value);
    }

    /// Subtract with carry.
    pub(crate) fn sbc(&mut self, address_mode: AddressMode) {
        let addr = self.address(address_mode);
        let mut value = self.read(addr) ^ 0xFF;
        if self.registers.status.get(Flag::Decimal) {
            // Use nine's complement.
            value -= 0x66;
        }

        self.adc_value(value);
    }

    fn cmp_value(&mut self, address_mode: AddressMode, register_value: u8) {
        let addr = self.address(address_mode);
        let value = self.read(addr);

        self.registers.status.set_to(Flag::Carry, register_value >= value);
        let difference = register_value.wrapping_sub(value);

        self.registers.status.update_zero_negative(difference);
    }

    /// Compare accumulator.
    pub(crate) fn cmp(&mut self, address_mode: AddressMode) {
        self.cmp_value(address_mode, self.registers.accumulator);
    }

    /// Compare X register.
    pub(crate) fn cpx(&mut self, address_mode: AddressMode) {
        self.cmp_value(address_mode, self.registers.x);
    }

    /// Compare Y register.
    pub(crate) fn cpy(&mut self, address_mode: AddressMode) {
        self.cmp_value(address_mode, self.registers.y);
    }
}

#[cfg(test)]
mod test {
    use crate::memory::basic::DefaultMemory;
    use crate::emulator::addressing::AddressMode;
    use crate::emulator::Emulator;
    use crate::emulator::instructions::opcodes::{ADC, Immediate};
    use crate::emulator::registers::{Flag, Registers};
    use crate::emulator::registers::Flag::*;
    use crate::emulator::tests::*;

    fn test_arithmetic(
        instruction: AddressInstruction,
        setup_flags: Vec<Flag>,
        accumulator_value: u8,
        immediate_value: u8,
        expected_result: u8,
        expected_flags_set: Vec<Flag>
    ) {
        let mut t = setup(setup_flags);

        t.registers.accumulator = accumulator_value;
        t.memory.memory[t.registers.program_counter as usize] = immediate_value;
        instruction(&mut t, AddressMode::Immediate);

        t.assert_flags_set(expected_flags_set);
        assert_eq!(expected_result, t.registers.accumulator);
    }

    #[test]
    fn test_adc_positive() {
        test_arithmetic(Emulator::adc, vec![], 3, 5, 8, vec![]);
    }

    #[test]
    fn test_adc_negative() {
        test_arithmetic(Emulator::adc, vec![], -3i8 as u8, -5i8 as u8, -8i8 as u8, vec![Carry, Negative]);
    }

    #[test]
    fn test_adc_mixed() {
        test_arithmetic(Emulator::adc, vec![], -3i8 as u8, 5, 2, vec![Carry]);
    }

    #[test]
    fn test_adc_overflow() {
        test_arithmetic(Emulator::adc, vec![], 127, 1, -128i8 as u8, vec![Overflow, Negative]);
    }

    #[test]
    fn test_adc_decimal_without_carry() {
        test_arithmetic(Emulator::adc, vec![Decimal], 0x33, 0x22, 0x55, vec![Decimal]);
    }

    #[test]
    fn test_adc_decimal_with_carry() {
        test_arithmetic(Emulator::adc, vec![Decimal], 0x99, 0x09, 0x08, vec![Decimal, Carry]);
    }

    #[test]
    fn test_sbc_positive() {
        test_arithmetic(Emulator::sbc, vec![Carry], 3, 5, -2i8 as u8, vec![Negative])
    }

    #[test]
    fn test_sbc_negative() {
        test_arithmetic(Emulator::sbc, vec![Carry], -3i8 as u8, -5i8 as u8, 2, vec![Carry])
    }

    #[test]
    fn test_sbc_mixed() {
        test_arithmetic(Emulator::sbc, vec![Carry], -3i8 as u8, 5, -8i8 as u8, vec![Carry, Negative])
    }

    #[test]
    fn test_sbc_overflow() {
        test_arithmetic(Emulator::sbc, vec![Carry], -128i8 as u8, 1, 127, vec![Carry, Overflow])
    }

    #[test]
    fn test_sbc_bcd() {
        test_arithmetic(Emulator::sbc, vec![Decimal, Carry], 0x91, 0x23, 0x68, vec![Carry, Decimal])
    }

    fn test_compare<R: Fn(&mut Registers) -> ()>(
        instruction: AddressInstruction,
        register_setup: R,
        immediate_value: u8,
        expected_flags_set: Vec<Flag>
    ) {
        let mut t = setup(vec![]);

        register_setup(&mut t.registers);
        t.memory.memory[t.registers.program_counter as usize] = immediate_value;
        instruction(&mut t, AddressMode::Immediate);

        t.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_cmp_1() {
        test_compare(Emulator::cmp, |r| r.accumulator = 0xau8, 0xbu8, vec![Negative])
    }

    #[test]
    fn test_cmp_2() {
        test_compare(Emulator::cmp, |r| r.accumulator = 0xbu8, 0xau8, vec![Carry])
    }

    #[test]
    fn test_cmp_3() {
        test_compare(Emulator::cmp, |r| r.accumulator = 0xbu8, 0xbu8, vec![Carry, Zero])
    }

    #[test]
    fn test_cpx_1() {
        test_compare(Emulator::cpx, |r| r.x = 0xau8, 0xbu8, vec![Negative])
    }

    #[test]
    fn test_cpx_2() {
        test_compare(Emulator::cpx, |r| r.x = 0xbu8, 0xau8, vec![Carry])
    }

    #[test]
    fn test_cpx_3() {
        test_compare(Emulator::cpx, |r| r.x = 0xbu8, 0xbu8, vec![Carry, Zero])
    }

    #[test]
    fn test_cpy_1() {
        test_compare(Emulator::cpy, |r| r.y = 0xau8, 0xbu8, vec![Negative]);
    }

    #[test]
    fn test_cpy_2() {
        test_compare(Emulator::cpy, |r| r.y = 0xbu8, 0xau8, vec![Carry]);
    }

    #[test]
    fn test_cpy_3() {
        test_compare(Emulator::cpy, |r| r.y = 0xbu8, 0xbu8, vec![Carry, Zero]);
    }
}
