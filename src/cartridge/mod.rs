pub trait Cartridge : Sized {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub const SIZE: usize = 0xFFFF + 1;
const PC_RESET_1: usize = 0xFFFC;
const PC_RESET_2: usize = 0xFFFD;

pub struct TestCartridge {
    pub memory: [u8; SIZE]
}

impl Cartridge for TestCartridge {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address as usize)]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address as usize)] = value
    }
}

impl TestCartridge {
    pub fn new(instructions: Vec<u8>, start_location: usize) -> TestCartridge {
        let mut memory = [0u8; SIZE];

        for (i, v) in instructions.into_iter().enumerate() {
            memory[i + start_location] = v;
        }

        TestCartridge { memory }
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.memory[PC_RESET_1] = (0x00FF & pc) as u8;
        self.memory[PC_RESET_2] = ((0xFF00 & pc) >> 8) as u8;
    }
}
