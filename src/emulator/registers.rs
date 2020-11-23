#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag {
    Carry = 0,
    Zero,
    Interrupt,
    Decimal,
    Break,
    Reserved,
    Overflow,
    Negative,
}

#[derive(Debug, Clone)]
pub(crate) struct Status {
    pub(crate) flags: u8,
}

impl Status {
    const DEFAULT_STATUS: u8 = 0b00100000;

    fn new() -> Status {
        Status {
            flags: Self::DEFAULT_STATUS,
        }
    }

    pub(crate) fn from(&mut self, contents: u8) {
        self.flags = contents | Self::DEFAULT_STATUS
    }

    pub(crate) fn set_to(&mut self, flag: Flag, v: bool) {
        if v {
            self.set(flag)
        } else {
            self.clear(flag);
        }
    }

    pub(crate) fn set(&mut self, flag: Flag) {
        self.flags |= 0x1 << (flag as u8);
    }

    pub(crate) fn get(&self, flag: Flag) -> bool {
        let v = self.flags >> (flag as u8);
        (v & 0x1) == 1
    }

    pub(crate) fn clear(&mut self, flag: Flag) {
        self.flags &= !(0x1 << (flag as u8));
    }

    pub(crate) fn update_zero(&mut self, v: u8) {
        let is_zero = v == 0u8;
        self.set_to(Flag::Zero, is_zero);
    }

    fn update_negative(&mut self, v: u8) {
        let is_negative = (v & 0b10000000) != 0;
        self.set_to(Flag::Negative, is_negative);
    }

    pub(crate) fn update_zero_negative(&mut self, v: u8) {
        self.update_zero(v);
        self.update_negative(v);
    }

    pub(crate) fn update_carry(&mut self, v: u16) {
        let is_carry = v > 0xFF;
        self.set_to(Flag::Carry, is_carry);
    }

    pub(crate) fn update_overflow(&mut self, a: u8, b: u8, result: u16) {
        // Determine if signs differ from result.
        let sign_a_differs_from_result = (a & 0b10000000) as u16 != (result & 0b10000000);
        let sign_b_differs_from_result = (b & 0b10000000) as u16 != (result & 0b10000000);

        // If both differ, indicate overflow.
        let is_overflow = sign_a_differs_from_result && sign_b_differs_from_result;

        self.set_to(Flag::Overflow, is_overflow);
    }
}

#[derive(Debug)]
pub struct Registers {
    pub program_counter: u16,
    pub(crate) stack_pointer: u8,
    pub(crate) accumulator: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) status: Status,
}

impl Registers {
    pub(crate) fn new() -> Registers {
        Registers {
            program_counter: 0,
            stack_pointer: 0xFF,
            accumulator: 0,
            x: 0,
            y: 0,
            status: Status::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::registers::{Flag, Status};

    #[test]
    fn test_reserved_always_on() {
        let status = Status::new();
        assert_eq!(true, status.get(Flag::Reserved));
    }

    #[test]
    fn test_set_get_clear() {
        fn test(flag: Flag) {
            let mut status = Status::new();
            assert_eq!(false, status.get(flag));

            status.set(flag);
            assert_eq!(true, status.get(flag));

            status.clear(flag);
            assert_eq!(false, status.get(flag));

            assert_eq!(true, status.get(Flag::Reserved));
        }

        test(Flag::Carry);
        test(Flag::Zero);
        test(Flag::Interrupt);
        test(Flag::Decimal);
        test(Flag::Break);
        test(Flag::Overflow);
        test(Flag::Negative);
    }

    #[test]
    fn test_clear_retains_others() {
        let mut status = Status::new();
        status.set(Flag::Carry);
        status.set(Flag::Break);
        status.set(Flag::Negative);

        assert_eq!(true, status.get(Flag::Carry));
        assert_eq!(true, status.get(Flag::Break));
        assert_eq!(true, status.get(Flag::Negative));
        assert_eq!(true, status.get(Flag::Reserved));

        status.clear(Flag::Carry);
        assert_eq!(false, status.get(Flag::Carry));
        assert_eq!(true, status.get(Flag::Break));
        assert_eq!(true, status.get(Flag::Negative));
        assert_eq!(true, status.get(Flag::Reserved));
    }
}
