use crate::cartridge::Cartridge;
use crate::processor::registers::{Registers, Flag};

pub mod read_write;
pub mod registers;
mod addressing;
pub mod instructions;

pub struct Core<C : Cartridge> {
    pub registers: Registers,
    cartridge: C,
    tick: u32
}

impl<C : Cartridge> Core<C> {
    pub fn new(cartridge: C) -> Core<C> {
        let mut core = Core {
            registers: Registers::new(),
            cartridge,
            tick: 0
        };

        core.reset();
        core
    }

    fn reset(&mut self) {
        // Initialize the program counter from the predefined memory locations.
        self.registers.program_counter = self.read_two(0xFFFC, 0xFFFD);
    }

    pub fn execute_next(&mut self) {
        let instruction = self.read(self.registers.program_counter);
        self.execute(instruction);
        self.registers.program_counter += 1;
    }
}
