use crate::emulator::registers::Flag;
use crate::memory::Memory;
use crate::emulator::addressing::AddressMode;
use crate::emulator::Emulator;

mod arithmetic;
mod branches;
mod dispatch;
mod flags;
mod increments_decrements;
mod jumps_calls;
mod load_store;
mod logical;
mod registers;
pub mod opcodes;
mod shifts;
mod stack;
mod system;
