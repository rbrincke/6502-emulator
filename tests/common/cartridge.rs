use nes::cartridge::Cartridge;

const OFFSET: usize = 0x4018;
const SIZE: usize = 0xFFFF - OFFSET + 1;
const ROM_START: usize = 0x8000 - OFFSET;
const PC_RESET_1: usize = 0xFFFC - OFFSET;
const PC_RESET_2: usize = 0xFFFD - OFFSET;

pub(crate) struct TestCartridge {
    memory: [u8; SIZE]
}

impl TestCartridge {
    pub fn new(instructions: Vec<u8>) -> TestCartridge {
        let mut memory = [0u8; SIZE];

        for (i, v) in instructions.into_iter().enumerate() {
            // Start writing at 0x8000
            memory[i + ROM_START] = v;
        }

        memory[PC_RESET_1] = 0x0;  // Set PC reset location 0xFFFC and 0xFFFD to 0x8000
        memory[PC_RESET_2] = 0x80;

        TestCartridge {
            memory
        }
    }
}

impl Cartridge for TestCartridge {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address as usize) - 0x4018]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address as usize) - 0x4018] = value
    }
}
