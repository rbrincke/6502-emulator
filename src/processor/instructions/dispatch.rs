use crate::processor::addressing::AddressMode;
use crate::cartridge::Cartridge;
use crate::processor::Core;

// http://nparker.llx.com/a2/opcodes.html#ins02
impl<C : Cartridge> Core<C> {
    // See http://www.obelisk.me.uk/6502/reference.html
    pub(crate) fn execute(&mut self, instruction: u8) {
        match instruction {
            0x69u8 => self.adc(AddressMode::Immediate),
            0x65u8 => self.adc(AddressMode::ZeroPage),
            0x75u8 => self.adc(AddressMode::ZeroPageX),
            0x6du8 => self.adc(AddressMode::Absolute),
            0x7du8 => self.adc(AddressMode::AbsoluteX),
            0x79u8 => self.adc(AddressMode::AbsoluteY),
            0x61u8 => self.adc(AddressMode::IndexedIndirect),
            0x71u8 => self.adc(AddressMode::IndirectIndexed),
            0x29u8 => self.and(AddressMode::Immediate),
            0x25u8 => self.and(AddressMode::ZeroPage),
            0x35u8 => self.and(AddressMode::ZeroPageX),
            0x2du8 => self.and(AddressMode::Absolute),
            0x3du8 => self.and(AddressMode::AbsoluteX),
            0x39u8 => self.and(AddressMode::AbsoluteY),
            0x21u8 => self.and(AddressMode::IndexedIndirect),
            0x31u8 => self.and(AddressMode::IndirectIndexed),
            0x0au8 => self.asl_acc(),
            0x06u8 => self.asl_mem(AddressMode::ZeroPage),
            0x16u8 => self.asl_mem(AddressMode::ZeroPageX),
            0x0eu8 => self.asl_mem(AddressMode::Absolute),
            0x1eu8 => self.asl_mem(AddressMode::AbsoluteX),
            0x90u8 => self.bcc(),
            0xb0u8 => self.bcs(),
            0xf0u8 => self.beq(),
            0x24u8 => self.bit(AddressMode::ZeroPage),
            0x2cu8 => self.bit(AddressMode::Absolute),
            0x30u8 => self.bmi(),
            0xd0u8 => self.bne(),
            0x10u8 => self.bpl(),
            0x00u8 => self.brk(),
            0x50u8 => self.bvc(),
            0x70u8 => self.bvs(),
            0x18u8 => self.clc(),
            0xd8u8 => self.cld(),
            0x58u8 => self.cli(),
            0xb8u8 => self.clv(),
//        0xc9u8 => Instruction { op: Op::CMP, address_mode: AddressMode::Immediate, cycles: 2 },
//        0xc5u8 => Instruction { op: Op::CMP, address_mode: AddressMode::ZeroPage, cycles: 3 },
//        0xd5u8 => Instruction { op: Op::CMP, address_mode: AddressMode::ZeroPageX, cycles: 4 },
//        0xcdu8 => Instruction { op: Op::CMP, address_mode: AddressMode::Absolute, cycles: 4 },
//        0xddu8 => Instruction { op: Op::CMP, address_mode: AddressMode::AbsoluteX, cycles: 4 },
//        0xd9u8 => Instruction { op: Op::CMP, address_mode: AddressMode::AbsoluteY, cycles: 4 },
//        0xc1u8 => Instruction { op: Op::CMP, address_mode: AddressMode::IndexedIndirect, cycles: 6 },
//        0xd1u8 => Instruction { op: Op::CMP, address_mode: AddressMode::IndirectIndexed, cycles: 5 },
//        0xe0u8 => Instruction { op: Op::CPX, address_mode: AddressMode::Immediate, cycles: 2 },
//        0xe4u8 => Instruction { op: Op::CPX, address_mode: AddressMode::ZeroPage, cycles: 3 },
//        0xecu8 => Instruction { op: Op::CPX, address_mode: AddressMode::Absolute, cycles: 4 },
//        0xc0u8 => Instruction { op: Op::CPY, address_mode: AddressMode::Immediate, cycles: 2 },
//        0xc4u8 => Instruction { op: Op::CPY, address_mode: AddressMode::ZeroPage, cycles: 3 },
//        0xccu8 => Instruction { op: Op::CPY, address_mode: AddressMode::Absolute, cycles: 4 },
            0xc6u8 => self.dec(AddressMode::ZeroPage),
            0xd6u8 => self.dec(AddressMode::ZeroPageX),
            0xceu8 => self.dec(AddressMode::Absolute),
            0xdeu8 => self.dec(AddressMode::AbsoluteX),
            0xcau8 => self.dex(),
            0x88u8 => self.dey(),
            0x49u8 => self.eor(AddressMode::Immediate),
            0x45u8 => self.eor(AddressMode::ZeroPage),
            0x55u8 => self.eor(AddressMode::ZeroPageX),
            0x4du8 => self.eor(AddressMode::Absolute),
            0x5du8 => self.eor(AddressMode::AbsoluteX),
            0x59u8 => self.eor(AddressMode::AbsoluteY),
            0x41u8 => self.eor(AddressMode::IndexedIndirect),
            0x51u8 => self.eor(AddressMode::IndirectIndexed),
            0xe6u8 => self.inc(AddressMode::ZeroPage),
            0xf6u8 => self.inc(AddressMode::ZeroPageX),
            0xeeu8 => self.inc(AddressMode::Absolute),
            0xfeu8 => self.inc(AddressMode::AbsoluteX),
            0xe8u8 => self.inx(),
            0xc8u8 => self.iny(),
            0x4cu8 => self.jmp(AddressMode::Absolute),
            0x6cu8 => self.jmp(AddressMode::Indirect),
//        0x20u8 => Instruction { op: Op::JSR, address_mode: AddressMode::Absolute, cycles: 6 },
            0xa9u8 => self.lda(AddressMode::Immediate),
            0xa5u8 => self.lda(AddressMode::ZeroPage),
            0xb5u8 => self.lda(AddressMode::ZeroPageX),
            0xadu8 => self.lda(AddressMode::Absolute),
            0xbdu8 => self.lda(AddressMode::AbsoluteX),
            0xb9u8 => self.lda(AddressMode::AbsoluteY),
            0xa1u8 => self.lda(AddressMode::IndexedIndirect),
            0xb1u8 => self.lda(AddressMode::IndirectIndexed),
            0xa2u8 => self.ldx(AddressMode::Immediate),
            0xa6u8 => self.ldx(AddressMode::ZeroPage),
            0xb6u8 => self.ldx(AddressMode::ZeroPageY),
            0xaeu8 => self.ldx(AddressMode::Absolute),
            0xbeu8 => self.ldx(AddressMode::AbsoluteY),
            0xa0u8 => self.ldy(AddressMode::Immediate),
            0xa4u8 => self.ldy(AddressMode::ZeroPage),
            0xb4u8 => self.ldy(AddressMode::ZeroPageX),
            0xacu8 => self.ldy(AddressMode::Absolute),
            0xbcu8 => self.ldy(AddressMode::AbsoluteX),
            0x4au8 => self.lsr_acc(),
            0x46u8 => self.lsr_mem(AddressMode::ZeroPage),
            0x56u8 => self.lsr_mem(AddressMode::ZeroPageX),
            0x4eu8 => self.lsr_mem(AddressMode::Absolute),
            0x5eu8 => self.lsr_mem(AddressMode::AbsoluteX),
            0xeau8 => self.nop(),
            0x09u8 => self.ora(AddressMode::Immediate),
            0x05u8 => self.ora(AddressMode::ZeroPage),
            0x15u8 => self.ora(AddressMode::ZeroPageX),
            0x0du8 => self.ora(AddressMode::Absolute),
            0x1du8 => self.ora(AddressMode::AbsoluteX),
            0x19u8 => self.ora(AddressMode::AbsoluteY),
            0x01u8 => self.ora(AddressMode::IndexedIndirect),
            0x11u8 => self.ora(AddressMode::IndirectIndexed),
//        0x48u8 => Instruction { op: Op::PHA, address_mode: AddressMode::Implicit, cycles: 3 },
//        0x08u8 => Instruction { op: Op::PHP, address_mode: AddressMode::Implicit, cycles: 3 },
//        0x68u8 => Instruction { op: Op::PLA, address_mode: AddressMode::Implicit, cycles: 4 },
//        0x28u8 => Instruction { op: Op::PLP, address_mode: AddressMode::Implicit, cycles: 4 },
            0x2au8 => self.rol_acc(),
            0x26u8 => self.rol_mem(AddressMode::ZeroPage),
            0x36u8 => self.rol_mem(AddressMode::ZeroPageX),
            0x2eu8 => self.rol_mem(AddressMode::Absolute),
            0x3eu8 => self.rol_mem(AddressMode::AbsoluteX),
            0x6au8 => self.ror_acc(),
            0x66u8 => self.ror_mem(AddressMode::ZeroPage),
            0x76u8 => self.ror_mem(AddressMode::ZeroPageX),
            0x6eu8 => self.ror_mem(AddressMode::Absolute),
            0x7eu8 => self.ror_mem(AddressMode::AbsoluteX),
//        0x40u8 => Instruction { op: Op::RTI, address_mode: AddressMode::Implicit, cycles: 6 },
//        0x60u8 => Instruction { op: Op::RTS, address_mode: AddressMode::Implicit, cycles: 6 },
            0xe9u8 => self.sbc(AddressMode::Immediate),
            0xe5u8 => self.sbc(AddressMode::ZeroPage),
            0xf5u8 => self.sbc(AddressMode::ZeroPageX),
            0xedu8 => self.sbc(AddressMode::Absolute),
            0xfdu8 => self.sbc(AddressMode::AbsoluteX),
            0xf9u8 => self.sbc(AddressMode::AbsoluteY),
            0xe1u8 => self.sbc(AddressMode::IndexedIndirect),
            0xf1u8 => self.sbc(AddressMode::IndirectIndexed),
            0x38u8 => self.sec(),
            0xf8u8 => self.sed(),
            0x78u8 => self.sei(),
            0x85u8 => self.sta(AddressMode::ZeroPage),
            0x95u8 => self.sta(AddressMode::ZeroPageX),
            0x8du8 => self.sta(AddressMode::Absolute),
            0x9du8 => self.sta(AddressMode::AbsoluteX),
            0x99u8 => self.sta(AddressMode::AbsoluteY),
            0x81u8 => self.sta(AddressMode::IndexedIndirect),
            0x91u8 => self.sta(AddressMode::IndirectIndexed),
            0x86u8 => self.stx(AddressMode::ZeroPage),
            0x96u8 => self.stx(AddressMode::ZeroPageY),
            0x8eu8 => self.stx(AddressMode::Absolute),
            0x84u8 => self.sty(AddressMode::ZeroPage),
            0x94u8 => self.sty(AddressMode::ZeroPageX),
            0x8cu8 => self.sty(AddressMode::Absolute),
            0xaau8 => self.tax(),
            0xa8u8 => self.tay(),
            0xbau8 => self.tsx(),
            0x8au8 => self.txa(),
            0x9au8 => self.txs(),
            0x98u8 => self.tya(),
            _ => panic!("Unknown instruction {}", instruction)
        }
    }
}