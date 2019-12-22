use std::collections::HashMap;

enum Flag {
    Carry,
    Zero,
    Interrupt,
    Decimal,
    Break,
    Overflow,
    Negative
}

enum AddressMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed
}

struct Registers {
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    x: u8,
    y: u8,
    status: u8
}

enum Op {

}

struct Instruction {
    op: Op,
    address_mode: AddressMode,
    cycles: u8
}

static INSTRUCTIONS: [Option<Instruction>; 0xFE] = [

];

struct Cpu {
    // 0x0000 until 0x00FF - zero page
    // 0x0100 until 0x01FF - stack
    memory: [u16; 0xFFFF]
}