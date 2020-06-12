use crate::processor::registers::{Registers, Flag};
use crate::cartridge::Cartridge;
use crate::processor::addressing::Address;
use crate::processor::Core;

impl<C : Cartridge> Core<C> {
    pub(crate) fn read_two_raw(&mut self, address_least_significant: u16, address_most_significant: u16) -> u16 {
        ((self.read_raw(address_most_significant) as u16) << 8) | (self.read_raw(address_least_significant) as u16)
    }

    pub(crate) fn read_raw(&mut self, address: u16) -> u8 {
        self.tick();

        match address {
            0x0000..=0x4016 => self.memory[address as usize],
            0x4018..=0xFFFF => self.cartridge.read(address),
            _ => panic!("Attempt to read unmapped address {}", address)
        }
    }

    pub(crate) fn write_raw(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x4016 => self.memory[address as usize] = value,
            0x4018..=0xFFFF => self.cartridge.write(address, value),
            _ => panic!("Attempt to read unmapped address {}", address)
        }
    }

    pub(crate) fn read(&mut self, address: Address) -> u8 {
        match address {
            Address::Accumulator => self.registers.accumulator,
            Address::Memory(a) => self.read_raw(a)
        }
    }

    pub(crate) fn write(&mut self, address: Address, value: u8) {
        match address {
            Address::Accumulator => self.registers.accumulator = value,
            Address::Memory(a) => self.write_raw(a, value),
        }
    }
}
