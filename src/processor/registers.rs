#[derive(Clone, Copy, Debug)]
pub enum Flag {
    Carry = 0,
    Zero,
    Interrupt,
    Decimal,
    Break,
    Overflow,
    Negative
}

#[derive(Debug)]
pub struct Registers {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8
}

impl Registers {
    pub(crate) fn new() -> Registers {
        Registers {
            program_counter: 0,
            stack_pointer: 0xFF,
            accumulator: 0,
            x: 0,
            y: 0,
            status: 0
        }
    }

    pub(crate) fn set_flag_to(&mut self, flag: Flag, v: bool) {
        if v {
            self.set_flag(flag)
        } else {
            self.clear_flag(flag);
        }
    }

    pub(crate) fn set_flag(&mut self, flag: Flag) {
        self.status |= 0x1 << (flag as u8);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let v = self.status >> (flag as u8);
        (v & 0x1) == 1
    }

    pub(crate) fn clear_flag(&mut self, flag: Flag) {
        self.status &= !(0x1 << (flag as u8));
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::registers::{Registers, Flag};

    #[test]
    fn test_set_get_clear() {
        fn test(flag: Flag) {
            let mut registers = Registers::new();
            assert_eq!(false, registers.get_flag(flag));

            registers.set_flag(flag);
            assert_eq!(true, registers.get_flag(flag));

            registers.clear_flag(flag);
            assert_eq!(false, registers.get_flag(flag));
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
        let mut registers = Registers::new();
        registers.set_flag(Flag::Carry);
        registers.set_flag(Flag::Break);
        registers.set_flag(Flag::Negative);

        assert_eq!(true, registers.get_flag(Flag::Carry));
        assert_eq!(true, registers.get_flag(Flag::Break));
        assert_eq!(true, registers.get_flag(Flag::Negative));

        registers.clear_flag(Flag::Carry);
        assert_eq!(false, registers.get_flag(Flag::Carry));
        assert_eq!(true, registers.get_flag(Flag::Break));
        assert_eq!(true, registers.get_flag(Flag::Negative));
    }
}
