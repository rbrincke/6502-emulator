use crate::common::cartridge::TestCartridge;
use nes::processor::registers::Flag;
use nes::processor::Core;

mod cartridge;

/// Run a series of instructions and then return the machine state.
///
/// A single BRK instruction is added at the end of the Vec of instructions. Instructions
/// are then executed until the BRK flag is set.
pub fn test_run(mut program: Vec<u8>) -> Core<TestCartridge> {
    program.push(0x00u8);
    let mut core = Core::new(
        TestCartridge::new(program)
    );

    while !core.registers.get_flag(Flag::Break) {
        core.execute_next();
    }

    core
}
