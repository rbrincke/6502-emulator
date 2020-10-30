use nes::cartridge::Cartridge;

const PROGRAM_START: usize = 0x8000;
const SIZE: usize = 0xFFFF + 1;
const PC_RESET_1: usize = 0xFFFC;
const PC_RESET_2: usize = 0xFFFD;

pub struct TestCartridge {
    memory: [u8; SIZE]
}

impl TestCartridge {
    fn create_mem(instructions: Vec<u8>, to_location: usize) -> [u8; SIZE] {
        let mut memory = [0u8; SIZE];

        for (i, v) in instructions.into_iter().enumerate() {
            memory[i + to_location] = v;
        }

        memory
    }

    pub fn complete(instructions: Vec<u8>) -> TestCartridge {
        let mut memory = Self::create_mem(instructions, 0);

        TestCartridge {
            memory
        }
    }

    pub fn partial(instructions: Vec<u8>) -> TestCartridge {
        let mut memory = Self::create_mem(instructions, PROGRAM_START);

        // Set PC reset location PROGRAM_START.
        memory[PC_RESET_1] = (0x00FF & PROGRAM_START) as u8;
        memory[PC_RESET_2] = ((0xFF00 & PROGRAM_START) >> 8) as u8;

        TestCartridge {
            memory
        }
    }
}

impl Cartridge for TestCartridge {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address as usize)]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address as usize)] = value
    }
}
