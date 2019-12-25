use crate::processor::registers::{Registers, Flag};
use crate::cartridge::Cartridge;

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
        self.registers.program_counter = self.read_two(0xFFFc, 0xFFFd);
        self.registers.set_flag(Flag::Interrupt);
    }

    /// Increment cycle.
    pub(crate) fn tick(&mut self) {
        self.tick += 1;
    }

    pub(crate) fn read_two(&mut self, address_least_significant: u16, address_most_significant: u16) -> u16 {
        ((self.read(address_most_significant) as u16) << 8) | (self.read(address_least_significant) as u16)
    }

    pub(crate) fn read(&mut self, address: u16) -> u8 {
        self.tick();

        match address {
            0x0000..=0x4016 => self.memory[address as usize],
            0x4018..=0xFFFF => self.cartridge.read(address),
            _ => panic!("Attempt to read unmapped address {}", address)
        }
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x4016 => self.memory[address as usize] = value,
            0x4018..=0xFFFF => self.cartridge.write(address, value),
            _ => panic!("Attempt to read unmapped address {}", address)
        }
    }

    pub fn execute_next(&mut self) {
        let instruction = self.read(self.registers.program_counter);
        self.execute(instruction);
        self.registers.program_counter += 1;
    }
}
