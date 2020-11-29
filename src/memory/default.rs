use crate::memory::Memory;
use emulator::RESET_VECTOR_ADDR;

pub const SIZE: usize = 0xFFFF + 1;

pub struct DefaultMemory {
    pub memory: [u8; SIZE],
}

impl Memory for DefaultMemory {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address as usize)]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address as usize)] = value
    }
}

impl DefaultMemory {
    pub fn empty() -> DefaultMemory {
        DefaultMemory {
            memory: [0u8; SIZE],
        }
    }

    pub fn load(&mut self, instructions: Vec<u8>, start_location: usize) {
        for (i, v) in instructions.into_iter().enumerate() {
            self.memory[i + start_location] = v;
        }
    }

    pub fn set_program_counter(&mut self, pc: u16) {
        self.memory[RESET_VECTOR_ADDR as usize] = (0x00FF & pc) as u8;
        self.memory[(RESET_VECTOR_ADDR + 1) as usize] = ((0xFF00 & pc) >> 8) as u8;
    }
}
