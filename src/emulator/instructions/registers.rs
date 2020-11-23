use crate::emulator::Emulator;
use crate::memory::Memory;

impl<C: Memory> Emulator<C> {
    /// Transfer Accumulator to X.
    pub(crate) fn tax(&mut self) {
        self.registers.x = self.registers.accumulator;
        self.registers.status.update_zero_negative(self.registers.x);
    }

    /// Transfer Accumulator to Y.
    pub(crate) fn tay(&mut self) {
        self.registers.y = self.registers.accumulator;
        self.registers.status.update_zero_negative(self.registers.y);
    }

    /// Transfer X to Accumulator.
    pub(crate) fn txa(&mut self) {
        self.registers.accumulator = self.registers.x;
        self.registers
            .status
            .update_zero_negative(self.registers.accumulator);
    }

    /// Transfer Y to Accumulator.
    pub(crate) fn tya(&mut self) {
        self.registers.accumulator = self.registers.y;
        self.registers
            .status
            .update_zero_negative(self.registers.accumulator);
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::registers::Flag;
    use crate::emulator::registers::Flag::{Negative, Zero};
    use crate::emulator::tests::{setup, TestAssertions};

    fn test_tax(value: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);

        c.registers.accumulator = value;
        c.tax();

        assert_eq!(value, c.registers.x);
        c.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_tax_zero() {
        test_tax(0, vec![Zero])
    }

    #[test]
    fn test_tax_positive() {
        test_tax(8, vec![])
    }

    #[test]
    fn test_tax_negative() {
        test_tax(-8i8 as u8, vec![Negative])
    }

    fn test_tay(value: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);

        c.registers.accumulator = value;
        c.tay();

        assert_eq!(value, c.registers.y);
        c.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_tay_zero() {
        test_tay(0, vec![Zero])
    }

    #[test]
    fn test_tay_positive() {
        test_tay(8, vec![])
    }

    #[test]
    fn test_tay_negative() {
        test_tay(-8i8 as u8, vec![Negative])
    }

    fn test_txa(value: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);

        c.registers.x = value;
        c.txa();

        assert_eq!(value, c.registers.accumulator);
        c.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_txa_zero() {
        test_txa(0, vec![Zero])
    }

    #[test]
    fn test_txa_positive() {
        test_txa(8, vec![])
    }

    #[test]
    fn test_txa_negative() {
        test_txa(-8i8 as u8, vec![Negative])
    }

    fn test_tya(value: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);

        c.registers.y = value;
        c.tya();

        assert_eq!(value, c.registers.accumulator);
        c.assert_flags_set(expected_flags_set);
    }

    #[test]
    fn test_tya_zero() {
        test_tya(0, vec![Zero])
    }

    #[test]
    fn test_tya_positive() {
        test_tya(8, vec![])
    }

    #[test]
    fn test_tya_negative() {
        test_tya(-8i8 as u8, vec![Negative])
    }
}
