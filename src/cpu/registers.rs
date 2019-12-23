#[derive(Clone, Copy, Debug)]
enum Flag {
    Carry = 0,
    Zero,
    Interrupt,
    Decimal,
    Break,
    Overflow,
    Negative
}

#[derive(Debug)]
struct Registers {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    status: u8
}

impl Registers {
    fn new() -> Registers {
        Registers {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            status: 0
        }
    }

    fn set(&mut self, flag: Flag) {
        self.status |= 0x1 << (flag as u8);
    }

    fn get(&self, flag: Flag) -> bool {
        let v = self.status >> (flag as u8);
        (v & 0x1) == 1
    }

    fn clear(&mut self, flag: Flag) {
        self.status &= !(0x1 << (flag as u8));
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::registers::{Registers, Flag};

    #[test]
    fn test_set_get_clear() {
        let mut registers = Registers::new();

        fn test(flag: Flag) {
            let mut registers = Registers::new();
            assert_eq!(false, registers.get(flag));

            registers.set(flag);
            assert_eq!(true, registers.get(flag));

            registers.clear(flag);
            assert_eq!(false, registers.get(flag));
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
        registers.set(Flag::Carry);
        registers.set(Flag::Break);
        registers.set(Flag::Negative);

        assert_eq!(true, registers.get(Flag::Carry));
        assert_eq!(true, registers.get(Flag::Break));
        assert_eq!(true, registers.get(Flag::Negative));

        registers.clear(Flag::Carry);
        assert_eq!(false, registers.get(Flag::Carry));
        assert_eq!(true, registers.get(Flag::Break));
        assert_eq!(true, registers.get(Flag::Negative));
    }
}
