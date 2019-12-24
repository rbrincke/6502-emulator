use crate::processor::registers::{Flag, Registers};

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
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
}

struct Instruction {
    op: Op,
    address_mode: AddressMode,
    cycles: u8
}

impl Instruction {
    pub fn run(&self, registers: &mut Registers) {
        match &self.op {
            Op::ADC => {},
            Op::AND => {},
            Op::ASL => {},
            Op::BCC => {},
            Op::BCS => {},
            Op::BEQ => {},
            Op::BIT => {},
            Op::BMI => {},
            Op::BNE => {},
            Op::BPL => {},
            Op::BRK => {},
            Op::BVC => {},
            Op::BVS => {},
            Op::CLC => {
                registers.clear(Flag::Carry);
            },

            Op::CLD => {
                registers.clear(Flag::Decimal);
            },

            Op::CLI => {
                registers.clear(Flag::Interrupt);
            },

            Op::CLV => {
                registers.clear(Flag::Overflow);
            },

            Op::CMP => {}
            Op::CPX => {}
            Op::CPY => {}
            Op::DEC => {}
            Op::DEX => {}
            Op::DEY => {}
            Op::EOR => {}
            Op::INC => {}
            Op::INX => {}
            Op::INY => {}
            Op::JMP => {}
            Op::JSR => {}
            Op::LDA => {}
            Op::LDX => {}
            Op::LDY => {}
            Op::LSR => {}
            Op::NOP => {}
            Op::ORA => {}
            Op::PHA => {}
            Op::PHP => {}
            Op::PLA => {}
            Op::PLP => {}
            Op::ROL => {}
            Op::ROR => {}
            Op::RTI => {}
            Op::RTS => {}
            Op::SBC => {}
            Op::SEC => {
                registers.set(Flag::Carry)
            }

            Op::SED => {
                registers.set(Flag::Decimal)
            }

            Op::SEI => {
                registers.set(Flag::Interrupt)
            }

            Op::STA => {}
            Op::STX => {}
            Op::STY => {}
            Op::TAX => {
                registers.x = registers.accumulator;
                set_zero(registers.x, registers);
                set_negative(registers.x, registers);
            }

            Op::TAY => {
                registers.y = registers.accumulator;
                set_zero(registers.y, registers);
                set_negative(registers.y, registers);
            }

            Op::TSX => {
                registers.x = registers.stack_pointer;
                set_zero(registers.x, registers);
                set_negative(registers.x, registers);
            }

            Op::TXA => {
                registers.accumulator = registers.x;
                set_zero(registers.accumulator, registers);
                set_negative(registers.accumulator, registers);
            }

            Op::TXS => {
                registers.stack_pointer = registers.x
            }

            Op::TYA => {
                registers.accumulator = registers.y;
                set_zero(registers.accumulator, registers);
                set_negative(registers.accumulator, registers);
            }
        }
    }
}

fn set_zero(v: u8, registers: &mut Registers) {
    if v == 0u8 {
        registers.set(Flag::Zero);
    };
}

fn set_negative(v: u8, registers: &mut Registers) {
    if ((v >> 7) & 0x1u8) == 1u8 {
        registers.set(Flag::Negative);
    };
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
    0x24u8 => Instruction { op: Op::BIT, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x2cu8 => Instruction { op: Op::BIT, address_mode: AddressMode::Absolute, cycles: 4 },
    0x30u8 => Instruction { op: Op::BMI, address_mode: AddressMode::Relative, cycles: 2 },
    0xd0u8 => Instruction { op: Op::BNE, address_mode: AddressMode::Relative, cycles: 2 },
    0x10u8 => Instruction { op: Op::BPL, address_mode: AddressMode::Relative, cycles: 2 },
    0x00u8 => Instruction { op: Op::BRK, address_mode: AddressMode::Implicit, cycles: 7 },
    0x50u8 => Instruction { op: Op::BVC, address_mode: AddressMode::Implicit, cycles: 2 },
    0x70u8 => Instruction { op: Op::BVS, address_mode: AddressMode::Relative, cycles: 2 },
    0x18u8 => Instruction { op: Op::CLC, address_mode: AddressMode::Implicit, cycles: 2 },
    0xd8u8 => Instruction { op: Op::CLD, address_mode: AddressMode::Implicit, cycles: 2 },
    0x58u8 => Instruction { op: Op::CLI, address_mode: AddressMode::Implicit, cycles: 2 },
    0xb8u8 => Instruction { op: Op::CLV, address_mode: AddressMode::Implicit, cycles: 2 },
    0xc9u8 => Instruction { op: Op::CMP, address_mode: AddressMode::Immediate, cycles: 2 },
    0xc5u8 => Instruction { op: Op::CMP, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xd5u8 => Instruction { op: Op::CMP, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0xcdu8 => Instruction { op: Op::CMP, address_mode: AddressMode::Absolute, cycles: 4 },
    0xddu8 => Instruction { op: Op::CMP, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0xd9u8 => Instruction { op: Op::CMP, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0xc1u8 => Instruction { op: Op::CMP, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0xd1u8 => Instruction { op: Op::CMP, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0xe0u8 => Instruction { op: Op::CPX, address_mode: AddressMode::Immediate, cycles: 2 },
    0xe4u8 => Instruction { op: Op::CPX, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xecu8 => Instruction { op: Op::CPX, address_mode: AddressMode::Absolute, cycles: 4 },
    0xc0u8 => Instruction { op: Op::CPY, address_mode: AddressMode::Immediate, cycles: 2 },
    0xc4u8 => Instruction { op: Op::CPY, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xccu8 => Instruction { op: Op::CPY, address_mode: AddressMode::Absolute, cycles: 4 },
    0xc6u8 => Instruction { op: Op::DEC, address_mode: AddressMode::ZeroPage, cycles: 5 },
    0xd6u8 => Instruction { op: Op::DEC, address_mode: AddressMode::ZeroPageX, cycles: 6 },
    0xceu8 => Instruction { op: Op::DEC, address_mode: AddressMode::Absolute, cycles: 6 },
    0xdeu8 => Instruction { op: Op::DEC, address_mode: AddressMode::AbsoluteX, cycles: 7 },
    0xcau8 => Instruction { op: Op::DEX, address_mode: AddressMode::Implicit, cycles: 2 },
    0x88u8 => Instruction { op: Op::DEY, address_mode: AddressMode::Implicit, cycles: 2 },
    0x49u8 => Instruction { op: Op::EOR, address_mode: AddressMode::Immediate, cycles: 2 },
    0x45u8 => Instruction { op: Op::EOR, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x55u8 => Instruction { op: Op::EOR, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0x4du8 => Instruction { op: Op::EOR, address_mode: AddressMode::Absolute, cycles: 4 },
    0x5du8 => Instruction { op: Op::EOR, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0x59u8 => Instruction { op: Op::EOR, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0x41u8 => Instruction { op: Op::EOR, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0x51u8 => Instruction { op: Op::EOR, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0xe6u8 => Instruction { op: Op::INC, address_mode: AddressMode::ZeroPage, cycles: 5 },
    0xf6u8 => Instruction { op: Op::INC, address_mode: AddressMode::ZeroPageX, cycles: 6 },
    0xeeu8 => Instruction { op: Op::INC, address_mode: AddressMode::Absolute, cycles: 6 },
    0xfeu8 => Instruction { op: Op::INC, address_mode: AddressMode::AbsoluteX, cycles: 7 },
    0xe8u8 => Instruction { op: Op::INX, address_mode: AddressMode::Implicit, cycles: 2 },
    0xc8u8 => Instruction { op: Op::INY, address_mode: AddressMode::Implicit, cycles: 2 },
    0x4cu8 => Instruction { op: Op::JMP, address_mode: AddressMode::Absolute, cycles: 3 },
    0x6cu8 => Instruction { op: Op::JMP, address_mode: AddressMode::Indirect, cycles: 5 },
    0x20u8 => Instruction { op: Op::JSR, address_mode: AddressMode::Absolute, cycles: 6 },
    0xa9u8 => Instruction { op: Op::LDA, address_mode: AddressMode::Immediate, cycles: 2 },
    0xa5u8 => Instruction { op: Op::LDA, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xb5u8 => Instruction { op: Op::LDA, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0xadu8 => Instruction { op: Op::LDA, address_mode: AddressMode::Absolute, cycles: 4 },
    0xbdu8 => Instruction { op: Op::LDA, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0xb9u8 => Instruction { op: Op::LDA, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0xa1u8 => Instruction { op: Op::LDA, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0xb1u8 => Instruction { op: Op::LDA, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0xa2u8 => Instruction { op: Op::LDX, address_mode: AddressMode::Immediate, cycles: 2 },
    0xa6u8 => Instruction { op: Op::LDX, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xb6u8 => Instruction { op: Op::LDX, address_mode: AddressMode::ZeroPageY, cycles: 4 },
    0xaeu8 => Instruction { op: Op::LDX, address_mode: AddressMode::Absolute, cycles: 4 },
    0xbeu8 => Instruction { op: Op::LDX, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0xa0u8 => Instruction { op: Op::LDY, address_mode: AddressMode::Immediate, cycles: 2 },
    0xa4u8 => Instruction { op: Op::LDY, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xb4u8 => Instruction { op: Op::LDY, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0xacu8 => Instruction { op: Op::LDY, address_mode: AddressMode::Absolute, cycles: 4 },
    0xbcu8 => Instruction { op: Op::LDY, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0x4au8 => Instruction { op: Op::LSR, address_mode: AddressMode::Accumulator, cycles: 2 },
    0x46u8 => Instruction { op: Op::LSR, address_mode: AddressMode::ZeroPage, cycles: 5 },
    0x56u8 => Instruction { op: Op::LSR, address_mode: AddressMode::ZeroPageX, cycles: 6 },
    0x4eu8 => Instruction { op: Op::LSR, address_mode: AddressMode::Absolute, cycles: 6 },
    0x5eu8 => Instruction { op: Op::LSR, address_mode: AddressMode::AbsoluteX, cycles: 7 },
    0xeau8 => Instruction { op: Op::NOP, address_mode: AddressMode::Implicit, cycles: 2 },
    0x09u8 => Instruction { op: Op::ORA, address_mode: AddressMode::Immediate, cycles: 2 },
    0x05u8 => Instruction { op: Op::ORA, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x15u8 => Instruction { op: Op::ORA, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0x0du8 => Instruction { op: Op::ORA, address_mode: AddressMode::Absolute, cycles: 4 },
    0x1du8 => Instruction { op: Op::ORA, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0x19u8 => Instruction { op: Op::ORA, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0x01u8 => Instruction { op: Op::ORA, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0x11u8 => Instruction { op: Op::ORA, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0x48u8 => Instruction { op: Op::PHA, address_mode: AddressMode::Implicit, cycles: 3 },
    0x08u8 => Instruction { op: Op::PHP, address_mode: AddressMode::Implicit, cycles: 3 },
    0x68u8 => Instruction { op: Op::PLA, address_mode: AddressMode::Implicit, cycles: 4 },
    0x28u8 => Instruction { op: Op::PLP, address_mode: AddressMode::Implicit, cycles: 4 },
    0x2au8 => Instruction { op: Op::ROL, address_mode: AddressMode::Accumulator, cycles: 2 },
    0x26u8 => Instruction { op: Op::ROL, address_mode: AddressMode::ZeroPage, cycles: 5 },
    0x36u8 => Instruction { op: Op::ROL, address_mode: AddressMode::ZeroPageX, cycles: 6 },
    0x2eu8 => Instruction { op: Op::ROL, address_mode: AddressMode::Absolute, cycles: 6 },
    0x3eu8 => Instruction { op: Op::ROL, address_mode: AddressMode::AbsoluteX, cycles: 7 },
    0x6au8 => Instruction { op: Op::ROR, address_mode: AddressMode::Accumulator, cycles: 2 },
    0x66u8 => Instruction { op: Op::ROR, address_mode: AddressMode::ZeroPage, cycles: 5 },
    0x76u8 => Instruction { op: Op::ROR, address_mode: AddressMode::ZeroPageX, cycles: 6 },
    0x6eu8 => Instruction { op: Op::ROR, address_mode: AddressMode::Absolute, cycles: 6 },
    0x7eu8 => Instruction { op: Op::ROR, address_mode: AddressMode::AbsoluteX, cycles: 7 },
    0x40u8 => Instruction { op: Op::RTI, address_mode: AddressMode::Implicit, cycles: 6 },
    0x60u8 => Instruction { op: Op::RTS, address_mode: AddressMode::Implicit, cycles: 6 },
    0xe9u8 => Instruction { op: Op::SBC, address_mode: AddressMode::Immediate, cycles: 2 },
    0xe5u8 => Instruction { op: Op::SBC, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0xf5u8 => Instruction { op: Op::SBC, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0xedu8 => Instruction { op: Op::SBC, address_mode: AddressMode::Absolute, cycles: 4 },
    0xfdu8 => Instruction { op: Op::SBC, address_mode: AddressMode::AbsoluteX, cycles: 4 },
    0xf9u8 => Instruction { op: Op::SBC, address_mode: AddressMode::AbsoluteY, cycles: 4 },
    0xe1u8 => Instruction { op: Op::SBC, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0xf1u8 => Instruction { op: Op::SBC, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
    0x38u8 => Instruction { op: Op::SEC, address_mode: AddressMode::Implicit, cycles: 2 },
    0xf8u8 => Instruction { op: Op::SED, address_mode: AddressMode::Implicit, cycles: 2 },
    0x78u8 => Instruction { op: Op::SEI, address_mode: AddressMode::Implicit, cycles: 2 },
    0x85u8 => Instruction { op: Op::STA, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x95u8 => Instruction { op: Op::STA, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0x8du8 => Instruction { op: Op::STA, address_mode: AddressMode::Absolute, cycles: 4 },
    0x9du8 => Instruction { op: Op::STA, address_mode: AddressMode::AbsoluteX, cycles: 5 },
    0x99u8 => Instruction { op: Op::STA, address_mode: AddressMode::AbsoluteY, cycles: 5 },
    0x81u8 => Instruction { op: Op::STA, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
    0x91u8 => Instruction { op: Op::STA, address_mode: AddressMode::IndirectIndexed, cycles: 6 },
    0x86u8 => Instruction { op: Op::STX, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x96u8 => Instruction { op: Op::STX, address_mode: AddressMode::ZeroPageY, cycles: 4 },
    0x8eu8 => Instruction { op: Op::STX, address_mode: AddressMode::Absolute, cycles: 4 },
    0x84u8 => Instruction { op: Op::STY, address_mode: AddressMode::ZeroPage, cycles: 3 },
    0x94u8 => Instruction { op: Op::STY, address_mode: AddressMode::ZeroPageX, cycles: 4 },
    0x8cu8 => Instruction { op: Op::STY, address_mode: AddressMode::Absolute, cycles: 4 },
    0xaau8 => Instruction { op: Op::TAX, address_mode: AddressMode::Implicit, cycles: 2 },
    0xa8u8 => Instruction { op: Op::TAY, address_mode: AddressMode::Implicit, cycles: 2 },
    0xbau8 => Instruction { op: Op::TSX, address_mode: AddressMode::Implicit, cycles: 2 },
    0x8au8 => Instruction { op: Op::TXA, address_mode: AddressMode::Implicit, cycles: 2 },
    0x9au8 => Instruction { op: Op::TXS, address_mode: AddressMode::Implicit, cycles: 2 },
    0x98u8 => Instruction { op: Op::TAY, address_mode: AddressMode::Implicit, cycles: 2 }
};
