use crate::memory::Memory;
use crate::emulator::Emulator;
use crate::emulator::registers::{Flag, Registers, Status};

impl<C : Memory> Emulator<C> {
    /// Transfer X to Stack Pointer.
    pub(crate) fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.x;
    }

    /// Transfer Stack Pointer to X.
    pub(crate) fn tsx(&mut self) {
        self.registers.x = self.registers.stack_pointer;
        self.registers.status.update_zero_negative(self.registers.x);
    }

    pub(crate) fn push(&mut self, value: u8) {
        let address = 0x100 + self.registers.stack_pointer as u16;
        self.write(address, value);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    pub(crate) fn pop(&mut self) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);

        let address = 0x100 + self.registers.stack_pointer as u16;
        let value = self.read(address);
        value
    }

    pub(crate) fn pha(&mut self) {
        self.push(self.registers.accumulator)
    }

    /// Push processor status.
    pub(crate) fn php(&mut self) {
        // Push status with Break on, but do not reflect in registers.
        let mut copy = self.registers.status.clone();
        copy.set(Flag::Break);

        self.push(copy.flags)
    }

    pub(crate) fn pla(&mut self) {
        self.registers.accumulator = self.pop();
        self.registers.status.update_zero_negative(self.registers.accumulator);
    }

    /// Pull processor status.
    pub(crate) fn plp(&mut self) {
        let c = self.pop();
        self.registers.status.from(c);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::tests::{setup, TestAssertions};
    use crate::emulator::registers::Flag::{Negative, Zero};
    use crate::emulator::registers::{Flag, Status};

    #[test]
    fn test_txs() {
        let mut c = setup(vec![]);
        c.registers.x = 0b10111100;

        c.txs();

        assert_eq!(0b10111100, c.registers.stack_pointer);
        c.assert_flags_set(vec![])
    }

    fn test_tsx(value: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);
        c.registers.stack_pointer = value;

        c.tsx();

        assert_eq!(value, c.registers.x);
        c.assert_flags_set(expected_flags_set)
    }

    #[test]
    fn test_tsx_positive() {
        test_tsx(0x12, vec![])
    }

    #[test]
    fn test_tsx_negative() {
        test_tsx(0xFC, vec![Negative])
    }

    #[test]
    fn test_tsx_zero() {
        test_tsx(0x0, vec![Zero])
    }

    #[test]
    fn test_pha() {
        let mut c = setup(vec![]);

        c.registers.accumulator = 24;
        c.registers.stack_pointer = 0xFF;
        c.pha();

        assert_eq!(24, c.memory.memory[0x1FF]);
        assert_eq!(0xFE, c.registers.stack_pointer)
    }

    #[test]
    fn test_php() {
        let mut c = setup(vec![]);

        c.registers.stack_pointer = 0xFF;
        c.registers.status.set(Flag::Break);
        c.registers.status.set(Flag::Decimal);
        let flags_before = c.registers.status.flags;

        c.php();

        assert_eq!(flags_before, c.memory.memory[0x1FF])
    }

    fn test_pla(value: u8, expected_flags_set: Vec<Flag>) {
        let mut c = setup(vec![]);

        c.registers.stack_pointer = 0xFE;
        c.memory.memory[0x1FF] = value;

        c.pla();

        assert_eq!(value, c.registers.accumulator);
        c.assert_flags_set(expected_flags_set)
    }

    #[test]
    fn test_pla_positive() {
        test_pla(0x14, vec![])
    }

    #[test]
    fn test_pla_negative() {
        test_pla(0xFC, vec![Negative])
    }

    #[test]
    fn test_pla_zero() {
        test_pla(0x0, vec![Zero])
    }

    #[test]
    fn test_plp() {
        let mut c = setup(vec![]);

        c.registers.stack_pointer = 0xFE;
        c.memory.memory[0x1FF] = 0b11111111;

        c.plp();

        assert_eq!(0b11111111, c.registers.status.flags)
    }

    #[test]
    fn test_stack_pointer_wrap_positive() {
        let mut c = setup(vec![]);

        c.registers.stack_pointer = 0xFF;
        c.pop();
        assert_eq!(0x00, c.registers.stack_pointer);
    }

    #[test]
    fn test_stack_pointer_wrap_negative() {
        let mut c = setup(vec![]);

        c.registers.stack_pointer = 0x00;
        c.push(0);
        assert_eq!(0xFF, c.registers.stack_pointer);
    }
}
