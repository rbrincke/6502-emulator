use crate::processor::core::Core;
use crate::processor::registers::Flag;
use crate::cartridge::Cartridge;

mod arithmetic;
mod branches;
mod dispatch;
mod flags;
mod increments_decrements;
mod jumps_calls;
mod load_store;
mod logical;
mod register;
mod stack;
mod system;

impl<C : Cartridge> Core<C> {
    fn check_value_set_zero(&mut self, v: u8) {
        if v == 0u8 {
            self.registers.set_flag(Flag::Zero);
        };
    }

    fn check_value_set_negative(&mut self, v: u8) {
        let is_negative = (v & 0b10000000) != 0;
        self.registers.set_flag_to(Flag::Negative, is_negative);
    }

    fn check_value_set_carry(&mut self, v: u16) {
        let is_carry = v > 0xFF;
        self.registers.set_flag_to(Flag::Carry, is_carry);
    }

    fn check_value_set_overflow(&mut self, a: u8, b: u8, result: u16) {
        // Determine if signs differ from result.
        let sign_a_differs_from_result = (a & 0b10000000) as u16 != (result & 0b10000000);
        let sign_b_differs_from_result = (b & 0b10000000) as u16 != (result & 0b10000000);

        // If both differ, indicate overflow.
        let is_overflow = sign_a_differs_from_result && sign_b_differs_from_result;

        self.registers.set_flag_to(Flag::Overflow, is_overflow);
    }
}

fn is_page_crossed(addr: u16, incr: u16) -> bool {
    // In the address 0xPPpp the PP constitutes the page. If after adding 'incr' to 'addr'
    // the pages are different, a page has been crossed.
    let total = addr + incr;
    return (total >> 8) != (addr >> 8);
}
