use std::collections::HashMap;

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

// http://nparker.llx.com/a2/opcodes.html#ins02
enum Op {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT
}

struct Instruction {
    op: Op,
    address_mode: AddressMode,
    cycles: u8
}

// See http://www.obelisk.me.uk/6502/reference.html
static INSTRUCTIONS: phf::Map<u8, Instruction> = phf_map! {
    0x69u8 => Instruction { op: Op::ADC, address_mode: AddressMode::Immediate, cycles: 2 },
    0x65u8 => Instruction { op: Op::ADC, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x75u8 => Instruction { op: Op::ADC, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0x6du8 => Instruction { op: Op::ADC, address_mode: AddressMode::Absolute, cycles: 4 },
    0x7du8 => Instruction { op: Op::ADC, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0x79u8 => Instruction { op: Op::ADC, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0x61u8 => Instruction { op: Op::ADC, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0x71u8 => Instruction { op: Op::ADC, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0x29u8 => Instruction { op: Op::AND, address_mode: AddressMode::Immediate, cycles: 2 },
    0x25u8 => Instruction { op: Op::AND, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x35u8 => Instruction { op: Op::AND, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0x2du8 => Instruction { op: Op::AND, address_mode: AddressMode::Absolute, cycles: 4 },
    0x3du8 => Instruction { op: Op::AND, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0x39u8 => Instruction { op: Op::AND, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0x21u8 => Instruction { op: Op::AND, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0x31u8 => Instruction { op: Op::AND, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0x0au8 => Instruction { op: Op::ASL, address_mode: AddressMode::Accumulator, cycles: 2 },
    0x06u8 => Instruction { op: Op::ASL, address_mode: AddressMode::ZeroPage, cycles: 5 },
    0x16u8 => Instruction { op: Op::ASL, address_mode: AddressMode::ZeroPageX, cycles: 6 },
    0x0eu8 => Instruction { op: Op::ASL, address_mode: AddressMode::Absolute, cycles: 6 },
    0x1eu8 => Instruction { op: Op::ASL, address_mode: AddressMode::AbsoluteX, cycles: 7 },
    0x90u8 => Instruction { op: Op::BCC, address_mode: AddressMode::Relative, cycles: 2 },
    0xb0u8 => Instruction { op: Op::BCS, address_mode: AddressMode::Relative, cycles: 2 },
    0xf0u8 => Instruction { op: Op::BEQ, address_mode: AddressMode::Relative, cycles: 2 },
    0x24u8 => Instruction { op: Op::BIT, address_mode: AddressMode::Relative, cycles: 3 },
    0x2cu8 => Instruction { op: Op::BIT, address_mode: AddressMode::Absolute, cycles: 4 },
};

struct Cpu {
    // 0x0000 until 0x00FF - zero page
    // 0x0100 until 0x01FF - stack
    memory: [u16; 0xFFFF]
}