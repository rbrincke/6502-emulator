pub trait Memory: Sized {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub mod basic;
