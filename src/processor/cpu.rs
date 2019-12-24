use crate::processor::registers::Registers;

struct Cpu {
    registers: Registers,
    // 0x0000 until 0x00FF - zero page
    // 0x0100 until 0x01FF - stack
    memory: [u16; 0xFFFF]
}
