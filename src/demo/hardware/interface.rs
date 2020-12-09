use emulator::memory::Memory;
use hardware::display::StackEntry;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

pub(crate) struct HardwareInterface {
    memory: [u8; 0x10000],
}

impl HardwareInterface {
    const ADDR_RND: u16 = 0xeee0;
    const ADDR_DATA_START: usize = 0xddd0;
    const ADDR_DATA_END: usize = 0xddd8;
    const ADDR_KEY: u16 = 0xfff0;
    const ADDR_KEY_IRQ_HANDLED: u16 = 0xfff1;
    const STACK_RANGE: i8 = 5;

    pub(crate) fn from_binary(instructions: Vec<u8>, start_location: usize) -> HardwareInterface {
        let mut memory = [0u8; 0x10000];
        for (i, v) in instructions.into_iter().enumerate() {
            memory[i + start_location] = v;
        }

        HardwareInterface { memory }
    }

    pub(crate) fn display_matrix_data(&self) -> [u8; 8] {
        self.memory[Self::ADDR_DATA_START..Self::ADDR_DATA_END]
            .try_into()
            .unwrap()
    }

    pub(crate) fn stack_around_stack_pointer(&self, stack_pointer: u8) -> Vec<StackEntry> {
        (-Self::STACK_RANGE..=Self::STACK_RANGE)
            .map(|i| {
                let stack_pointer_offset = stack_pointer.wrapping_add(i as u8);
                let v = self.read(0x100 + stack_pointer_offset as u16);

                StackEntry {
                    stack_pointer: stack_pointer_offset,
                    stack_value: v,
                }
            })
            .collect()
    }

    pub(crate) fn send_key(&mut self, key: char) {
        self.write(Self::ADDR_KEY, key as u8)
    }

    pub(crate) fn set_irq(&mut self) {
        self.write(Self::ADDR_KEY_IRQ_HANDLED, true as u8)
    }

    pub(crate) fn is_irq_handled(&self) -> bool {
        self.read(Self::ADDR_KEY_IRQ_HANDLED) == 0
    }
}

impl Memory for HardwareInterface {
    fn read(&self, address: u16) -> u8 {
        match address {
            // Reads on this address return a random number.
            HardwareInterface::ADDR_RND => thread_rng().gen(),

            _ => self.memory[address as usize],
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value
    }
}

#[cfg(test)]
mod tests {
    use emulator::memory::Memory;
    use hardware::interface::HardwareInterface;
    use std::convert::TryInto;

    #[test]
    fn test_load_binary() {
        let instructions = vec![2, 4, 6];
        let interface = HardwareInterface::from_binary(instructions.clone(), 0x8000);

        let contents: [u8; 3] = interface.memory[0x8000..=0x8002].try_into().unwrap();

        assert_eq!(instructions, contents.to_vec());
    }

    #[test]
    fn test_rnd() {
        let interface = HardwareInterface::from_binary(vec![], 0x0);

        let first = interface.read(HardwareInterface::ADDR_RND);
        let second = interface.read(HardwareInterface::ADDR_RND);

        assert_ne!(first, second);
    }

    #[test]
    fn test_irq_handled() {
        let mut interface = HardwareInterface::from_binary(vec![], 0x0);

        assert_eq!(true, interface.is_irq_handled());

        // Request IRQ.
        interface.set_irq();
        assert_eq!(false, interface.is_irq_handled());

        // 6502 write.
        interface.memory[HardwareInterface::ADDR_KEY_IRQ_HANDLED as usize] = 0;
        assert_eq!(true, interface.is_irq_handled());
    }

    #[test]
    fn test_stack_around_stack_pointer() {
        let interface = HardwareInterface::from_binary(vec![1, 2, 3, 4, 5, 6, 7], 0x1fa);

        let result = interface.stack_around_stack_pointer(0xff);
        assert_eq!(result.first().unwrap().stack_value, 1);
        assert_eq!(result.last().unwrap().stack_pointer, 0x4)
    }
}
