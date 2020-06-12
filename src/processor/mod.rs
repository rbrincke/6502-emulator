use crate::cartridge::Cartridge;
use crate::processor::registers::{Registers, Flag};

pub mod read_write;
pub mod registers;
mod addressing;
mod instructions;

pub struct Core<C : Cartridge> {
    pub registers: Registers,
    memory: [u8; 0x8000],
    cartridge: C,
    tick: u32
}

impl<C : Cartridge> Core<C> {
    pub fn new(cartridge: C) -> Core<C> {
        let mut core = Core {
            registers: Registers::new(),
            memory: [0; 0x8000],
            cartridge,
            tick: 0
        };

        core.reset();
        core
    }

    fn reset(&mut self) {
        // Initialize the program counter from the predefined memory locations.
        self.registers.program_counter = self.read_two_raw(0xFFFc, 0xFFFd);
        self.registers.set_flag(Flag::Interrupt);
    }

    pub fn execute_next(&mut self) {
        let instruction = self.read_raw(self.registers.program_counter);
        self.execute(instruction);
        self.registers.program_counter += 1;
    }

    /// Increment cycle.
    pub(crate) fn tick(&mut self) {
        self.tick += 1;
    }
}
