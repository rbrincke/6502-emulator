extern crate emulator;

use emulator::emulator::instructions::opcodes::*;
use emulator::emulator::Emulator;
use emulator::memory::default::DefaultMemory;
use emulator::memory::Memory;

// Exercise various instructions, addressing mode, flags, the stack in a reasonably compact
// machine-code only test.
#[test]
fn smoke() {
    let number = 396u16;
    let least_sig_byte: u8 = (number & 0xFF) as u8;
    let most_sig_byte: u8 = ((number & 0xFF00) >> 8) as u8;

    let program_main = vec![
        JSR::absolute(0x600),
        // Trap.
        LDA::immediate(0),
        BEQ::relative(-2), // Skip 2 bytes backwards to the beginning of BEQ.
    ]
    .into_iter()
    .flatten()
    .collect();

    let program_routine = vec![
        LDA::immediate(least_sig_byte),
        ADC::immediate(least_sig_byte),
        STA::zero_page(0x0),
        LDA::immediate(most_sig_byte),
        ADC::immediate(most_sig_byte),
        STA::zero_page(0x1),
        RTS::implied(),
    ]
    .into_iter()
    .flatten()
    .collect();

    let mut memory = DefaultMemory::empty();
    memory.load(program_main, 0x400);
    memory.load(program_routine, 0x600);
    memory.set_program_counter(0x400);

    let mut emulator = Emulator::new(memory);

    let mut program_counter = 0x0u16;
    while program_counter != emulator.registers.program_counter {
        program_counter = emulator.registers.program_counter;
        emulator.execute_next();
    }

    let r = ((emulator.memory.read(0x1) as u16) << 8) | (emulator.memory.read(0x0) as u16);
    assert_eq!(r, number * 2)
}
