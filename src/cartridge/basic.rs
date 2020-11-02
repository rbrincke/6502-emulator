use crate::cartridge::Cartridge;

pub const SIZE: usize = 0xFFFF + 1;
const PC_RESET_1: usize = 0xFFFC;
const PC_RESET_2: usize = 0xFFFD;

pub struct BasicCartridge {
    pub memory: [u8; SIZE]
}

impl Cartridge for BasicCartridge {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address as usize)]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address as usize)] = value
    }
}

impl BasicCartridge {
    pub fn empty() -> BasicCartridge {
        BasicCartridge { memory: [0u8; SIZE] }
    }

    pub fn load(&mut self, instructions: Vec<u8>, start_location: usize) {
        for (i, v) in instructions.into_iter().enumerate() {
            self.memory[i + start_location] = v;
        }
    }

    pub fn set_program_counter(&mut self, pc: u16) {
        self.memory[PC_RESET_1] = (0x00FF & pc) as u8;
        self.memory[PC_RESET_2] = ((0xFF00 & pc) >> 8) as u8;
    }
}
