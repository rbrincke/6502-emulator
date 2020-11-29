use crate::emulator::bytes_little_endian;

pub trait Immediate {
    const IMMEDIATE: u8;
    fn immediate(value: u8) -> Vec<u8> {
        vec![Self::IMMEDIATE, value]
    }
}

pub trait ZeroPage {
    const ZEROPAGE: u8;
    fn zero_page(value: u8) -> Vec<u8> {
        vec![Self::ZEROPAGE, value]
    }
}

pub trait ZeroPageX {
    const ZEROPAGEX: u8;
    fn zero_page_x(value: u8) -> Vec<u8> {
        vec![Self::ZEROPAGEX, value]
    }
}

pub trait ZeroPageY {
    const ZEROPAGEY: u8;
    fn zero_page_y(value: u8) -> Vec<u8> {
        vec![Self::ZEROPAGEY, value]
    }
}

pub trait Absolute {
    const ABSOLUTE: u8;
    fn absolute(value: u16) -> Vec<u8> {
        let (least, most) = bytes_little_endian(value);
        vec![Self::ABSOLUTE, least, most]
    }
}

pub trait AbsoluteX {
    const ABSOLUTEX: u8;
    fn absolute_x(value: u16) -> Vec<u8> {
        let (first, second) = bytes_little_endian(value);
        vec![Self::ABSOLUTEX, first, second]
    }
}

pub trait AbsoluteY {
    const ABSOLUTEY: u8;
    fn absolute_y(value: u16) -> Vec<u8> {
        let (first, second) = bytes_little_endian(value);
        vec![Self::ABSOLUTEY, first, second]
    }
}

pub trait IndexedIndirect {
    const INDEXEDINDIRECT: u8;
    fn indexed_indirect(value: u8) -> Vec<u8> {
        vec![Self::INDEXEDINDIRECT, value]
    }
}

pub trait IndirectIndexed {
    const INDIRECTINDEXED: u8;
    fn indirect_indexed(value: u8) -> Vec<u8> {
        vec![Self::INDIRECTINDEXED, value]
    }
}

pub trait Relative {
    const RELATIVE: u8;
    fn relative(displacement: i8) -> Vec<u8> {
        vec![Self::RELATIVE, displacement as u8]
    }
}

pub trait Accumulator {
    const ACCUMULATOR: u8;
    fn accumulator() -> Vec<u8> {
        vec![Self::ACCUMULATOR]
    }
}

pub trait Implied {
    const IMPLIED: u8;
    fn implied() -> Vec<u8> {
        vec![Self::IMPLIED]
    }
}

pub trait Indirect {
    const INDIRECT: u8;
    fn indirect(value: u16) -> Vec<u8> {
        let (first, second) = bytes_little_endian(value);
        vec![Self::INDIRECT, first, second]
    }
}

pub struct ADC;
impl Immediate for ADC {
    const IMMEDIATE: u8 = 0x69u8;
}
impl ZeroPage for ADC {
    const ZEROPAGE: u8 = 0x65u8;
}
impl ZeroPageX for ADC {
    const ZEROPAGEX: u8 = 0x75u8;
}
impl Absolute for ADC {
    const ABSOLUTE: u8 = 0x6du8;
}
impl AbsoluteX for ADC {
    const ABSOLUTEX: u8 = 0x7du8;
}
impl AbsoluteY for ADC {
    const ABSOLUTEY: u8 = 0x79u8;
}
impl IndexedIndirect for ADC {
    const INDEXEDINDIRECT: u8 = 0x61u8;
}
impl IndirectIndexed for ADC {
    const INDIRECTINDEXED: u8 = 0x71u8;
}

pub struct AND;
impl Immediate for AND {
    const IMMEDIATE: u8 = 0x29u8;
}
impl ZeroPage for AND {
    const ZEROPAGE: u8 = 0x25u8;
}
impl ZeroPageX for AND {
    const ZEROPAGEX: u8 = 0x35u8;
}
impl Absolute for AND {
    const ABSOLUTE: u8 = 0x2du8;
}
impl AbsoluteX for AND {
    const ABSOLUTEX: u8 = 0x3du8;
}
impl AbsoluteY for AND {
    const ABSOLUTEY: u8 = 0x39u8;
}
impl IndexedIndirect for AND {
    const INDEXEDINDIRECT: u8 = 0x21u8;
}
impl IndirectIndexed for AND {
    const INDIRECTINDEXED: u8 = 0x31u8;
}

pub struct ASL;
impl Accumulator for ASL {
    const ACCUMULATOR: u8 = 0x0au8;
}
impl ZeroPage for ASL {
    const ZEROPAGE: u8 = 0x06u8;
}
impl ZeroPageX for ASL {
    const ZEROPAGEX: u8 = 0x16u8;
}
impl Absolute for ASL {
    const ABSOLUTE: u8 = 0x0eu8;
}
impl AbsoluteX for ASL {
    const ABSOLUTEX: u8 = 0x1eu8;
}

pub struct BCC;
impl Relative for BCC {
    const RELATIVE: u8 = 0x90u8;
}

pub struct BCS;
impl Relative for BCS {
    const RELATIVE: u8 = 0xb0u8;
}

pub struct BEQ;
impl Relative for BEQ {
    const RELATIVE: u8 = 0xf0u8;
}

pub struct BIT;
impl ZeroPage for BIT {
    const ZEROPAGE: u8 = 0x24u8;
}
impl Absolute for BIT {
    const ABSOLUTE: u8 = 0x2cu8;
}

pub struct BMI;
impl Relative for BMI {
    const RELATIVE: u8 = 0x30u8;
}

pub struct BNE;
impl Relative for BNE {
    const RELATIVE: u8 = 0xd0u8;
}

pub struct BPL;
impl Relative for BPL {
    const RELATIVE: u8 = 0x10u8;
}

pub struct BRK;
/// BRK is a 2-byte instruction (see implementation notes).
impl Immediate for BRK {
    const IMMEDIATE: u8 = 0x00u8;
}

pub struct BVC;
impl Relative for BVC {
    const RELATIVE: u8 = 0x50u8;
}

pub struct BVS;
impl Relative for BVS {
    const RELATIVE: u8 = 0x70u8;
}

pub struct CLC;
impl Implied for CLC {
    const IMPLIED: u8 = 0x18u8;
}

pub struct CLD;
impl Implied for CLD {
    const IMPLIED: u8 = 0xd8u8;
}

pub struct CLI;
impl Implied for CLI {
    const IMPLIED: u8 = 0x58u8;
}

pub struct CLV;
impl Implied for CLV {
    const IMPLIED: u8 = 0xb8u8;
}

pub struct CMP;
impl Immediate for CMP {
    const IMMEDIATE: u8 = 0xc9u8;
}
impl ZeroPage for CMP {
    const ZEROPAGE: u8 = 0xc5u8;
}
impl ZeroPageX for CMP {
    const ZEROPAGEX: u8 = 0xd5u8;
}
impl Absolute for CMP {
    const ABSOLUTE: u8 = 0xcdu8;
}
impl AbsoluteX for CMP {
    const ABSOLUTEX: u8 = 0xddu8;
}
impl AbsoluteY for CMP {
    const ABSOLUTEY: u8 = 0xd9u8;
}
impl IndexedIndirect for CMP {
    const INDEXEDINDIRECT: u8 = 0xc1u8;
}
impl IndirectIndexed for CMP {
    const INDIRECTINDEXED: u8 = 0xd1u8;
}

pub struct CPX;
impl Immediate for CPX {
    const IMMEDIATE: u8 = 0xe0u8;
}
impl ZeroPage for CPX {
    const ZEROPAGE: u8 = 0xe4u8;
}
impl Absolute for CPX {
    const ABSOLUTE: u8 = 0xecu8;
}

pub struct CPY;
impl Immediate for CPY {
    const IMMEDIATE: u8 = 0xc0u8;
}
impl ZeroPage for CPY {
    const ZEROPAGE: u8 = 0xc4u8;
}
impl Absolute for CPY {
    const ABSOLUTE: u8 = 0xccu8;
}

pub struct DEC;
impl ZeroPage for DEC {
    const ZEROPAGE: u8 = 0xc6u8;
}
impl ZeroPageX for DEC {
    const ZEROPAGEX: u8 = 0xd6u8;
}
impl Absolute for DEC {
    const ABSOLUTE: u8 = 0xceu8;
}
impl AbsoluteX for DEC {
    const ABSOLUTEX: u8 = 0xdeu8;
}

pub struct DEX;
impl Implied for DEX {
    const IMPLIED: u8 = 0xcau8;
}

pub struct DEY;
impl Implied for DEY {
    const IMPLIED: u8 = 0x88u8;
}

pub struct EOR;
impl Immediate for EOR {
    const IMMEDIATE: u8 = 0x49u8;
}
impl ZeroPage for EOR {
    const ZEROPAGE: u8 = 0x45u8;
}
impl ZeroPageX for EOR {
    const ZEROPAGEX: u8 = 0x55u8;
}
impl Absolute for EOR {
    const ABSOLUTE: u8 = 0x4du8;
}
impl AbsoluteX for EOR {
    const ABSOLUTEX: u8 = 0x5du8;
}
impl AbsoluteY for EOR {
    const ABSOLUTEY: u8 = 0x59u8;
}
impl IndexedIndirect for EOR {
    const INDEXEDINDIRECT: u8 = 0x41u8;
}
impl IndirectIndexed for EOR {
    const INDIRECTINDEXED: u8 = 0x51u8;
}

pub struct INC;
impl ZeroPage for INC {
    const ZEROPAGE: u8 = 0xe6u8;
}
impl ZeroPageX for INC {
    const ZEROPAGEX: u8 = 0xf6u8;
}
impl Absolute for INC {
    const ABSOLUTE: u8 = 0xeeu8;
}
impl AbsoluteX for INC {
    const ABSOLUTEX: u8 = 0xfeu8;
}

pub struct INX;
impl Implied for INX {
    const IMPLIED: u8 = 0xe8u8;
}

pub struct INY;
impl Implied for INY {
    const IMPLIED: u8 = 0xc8u8;
}

pub struct JMP;
impl Absolute for JMP {
    const ABSOLUTE: u8 = 0x4cu8;
}
impl Indirect for JMP {
    const INDIRECT: u8 = 0x6cu8;
}

pub struct JSR;
impl Absolute for JSR {
    const ABSOLUTE: u8 = 0x20u8;
}

pub struct LDA;
impl Immediate for LDA {
    const IMMEDIATE: u8 = 0xa9u8;
}
impl ZeroPage for LDA {
    const ZEROPAGE: u8 = 0xa5u8;
}
impl ZeroPageX for LDA {
    const ZEROPAGEX: u8 = 0xb5u8;
}
impl Absolute for LDA {
    const ABSOLUTE: u8 = 0xadu8;
}
impl AbsoluteX for LDA {
    const ABSOLUTEX: u8 = 0xbdu8;
}
impl AbsoluteY for LDA {
    const ABSOLUTEY: u8 = 0xb9u8;
}
impl IndexedIndirect for LDA {
    const INDEXEDINDIRECT: u8 = 0xa1u8;
}
impl IndirectIndexed for LDA {
    const INDIRECTINDEXED: u8 = 0xb1u8;
}

pub struct LDX;
impl Immediate for LDX {
    const IMMEDIATE: u8 = 0xa2u8;
}
impl ZeroPage for LDX {
    const ZEROPAGE: u8 = 0xa6u8;
}
impl ZeroPageY for LDX {
    const ZEROPAGEY: u8 = 0xb6u8;
}
impl Absolute for LDX {
    const ABSOLUTE: u8 = 0xaeu8;
}
impl AbsoluteY for LDX {
    const ABSOLUTEY: u8 = 0xbeu8;
}

pub struct LDY;
impl Immediate for LDY {
    const IMMEDIATE: u8 = 0xa0u8;
}
impl ZeroPage for LDY {
    const ZEROPAGE: u8 = 0xa4u8;
}
impl ZeroPageX for LDY {
    const ZEROPAGEX: u8 = 0xb4u8;
}
impl Absolute for LDY {
    const ABSOLUTE: u8 = 0xacu8;
}
impl AbsoluteX for LDY {
    const ABSOLUTEX: u8 = 0xbcu8;
}

pub struct LSR;
impl Accumulator for LSR {
    const ACCUMULATOR: u8 = 0x4au8;
}
impl ZeroPage for LSR {
    const ZEROPAGE: u8 = 0x46u8;
}
impl ZeroPageX for LSR {
    const ZEROPAGEX: u8 = 0x56u8;
}
impl Absolute for LSR {
    const ABSOLUTE: u8 = 0x4eu8;
}
impl AbsoluteX for LSR {
    const ABSOLUTEX: u8 = 0x5eu8;
}

pub struct NOP;
impl Implied for NOP {
    const IMPLIED: u8 = 0xeau8;
}

pub struct ORA;
impl Immediate for ORA {
    const IMMEDIATE: u8 = 0x09u8;
}
impl ZeroPage for ORA {
    const ZEROPAGE: u8 = 0x05u8;
}
impl ZeroPageX for ORA {
    const ZEROPAGEX: u8 = 0x15u8;
}
impl Absolute for ORA {
    const ABSOLUTE: u8 = 0x0du8;
}
impl AbsoluteX for ORA {
    const ABSOLUTEX: u8 = 0x1du8;
}
impl AbsoluteY for ORA {
    const ABSOLUTEY: u8 = 0x19u8;
}
impl IndexedIndirect for ORA {
    const INDEXEDINDIRECT: u8 = 0x01u8;
}
impl IndirectIndexed for ORA {
    const INDIRECTINDEXED: u8 = 0x11u8;
}

pub struct PHA;
impl Implied for PHA {
    const IMPLIED: u8 = 0x48u8;
}

pub struct PHP;
impl Implied for PHP {
    const IMPLIED: u8 = 0x08u8;
}

pub struct PLA;
impl Implied for PLA {
    const IMPLIED: u8 = 0x68u8;
}

pub struct PLP;
impl Implied for PLP {
    const IMPLIED: u8 = 0x28u8;
}

pub struct ROL;
impl Accumulator for ROL {
    const ACCUMULATOR: u8 = 0x2au8;
}
impl ZeroPage for ROL {
    const ZEROPAGE: u8 = 0x26u8;
}
impl ZeroPageX for ROL {
    const ZEROPAGEX: u8 = 0x36u8;
}
impl Absolute for ROL {
    const ABSOLUTE: u8 = 0x2eu8;
}
impl AbsoluteX for ROL {
    const ABSOLUTEX: u8 = 0x3eu8;
}

pub struct ROR;
impl Accumulator for ROR {
    const ACCUMULATOR: u8 = 0x6au8;
}
impl ZeroPage for ROR {
    const ZEROPAGE: u8 = 0x66u8;
}
impl ZeroPageX for ROR {
    const ZEROPAGEX: u8 = 0x76u8;
}
impl Absolute for ROR {
    const ABSOLUTE: u8 = 0x6eu8;
}
impl AbsoluteX for ROR {
    const ABSOLUTEX: u8 = 0x7eu8;
}

pub struct RTI;
impl Implied for RTI {
    const IMPLIED: u8 = 0x40u8;
}

pub struct RTS;
impl Implied for RTS {
    const IMPLIED: u8 = 0x60u8;
}

pub struct SBC;
impl Immediate for SBC {
    const IMMEDIATE: u8 = 0xe9u8;
}
impl ZeroPage for SBC {
    const ZEROPAGE: u8 = 0xe5u8;
}
impl ZeroPageX for SBC {
    const ZEROPAGEX: u8 = 0xf5u8;
}
impl Absolute for SBC {
    const ABSOLUTE: u8 = 0xedu8;
}
impl AbsoluteX for SBC {
    const ABSOLUTEX: u8 = 0xfdu8;
}
impl AbsoluteY for SBC {
    const ABSOLUTEY: u8 = 0xf9u8;
}
impl IndexedIndirect for SBC {
    const INDEXEDINDIRECT: u8 = 0xe1u8;
}
impl IndirectIndexed for SBC {
    const INDIRECTINDEXED: u8 = 0xf1u8;
}

pub struct SEC;
impl Implied for SEC {
    const IMPLIED: u8 = 0x38u8;
}

pub struct SED;
impl Implied for SED {
    const IMPLIED: u8 = 0xf8u8;
}

pub struct SEI;
impl Implied for SEI {
    const IMPLIED: u8 = 0x78u8;
}

pub struct STA;
impl ZeroPage for STA {
    const ZEROPAGE: u8 = 0x85u8;
}
impl ZeroPageX for STA {
    const ZEROPAGEX: u8 = 0x95u8;
}
impl Absolute for STA {
    const ABSOLUTE: u8 = 0x8du8;
}
impl AbsoluteX for STA {
    const ABSOLUTEX: u8 = 0x9du8;
}
impl AbsoluteY for STA {
    const ABSOLUTEY: u8 = 0x99u8;
}
impl IndexedIndirect for STA {
    const INDEXEDINDIRECT: u8 = 0x81u8;
}
impl IndirectIndexed for STA {
    const INDIRECTINDEXED: u8 = 0x91u8;
}

pub struct STX;
impl ZeroPage for STX {
    const ZEROPAGE: u8 = 0x86u8;
}
impl ZeroPageY for STX {
    const ZEROPAGEY: u8 = 0x96u8;
}
impl Absolute for STX {
    const ABSOLUTE: u8 = 0x8eu8;
}

pub struct STY;
impl ZeroPage for STY {
    const ZEROPAGE: u8 = 0x84u8;
}
impl ZeroPageX for STY {
    const ZEROPAGEX: u8 = 0x94u8;
}
impl Absolute for STY {
    const ABSOLUTE: u8 = 0x8cu8;
}

pub struct TAX;
impl Implied for TAX {
    const IMPLIED: u8 = 0xaau8;
}

pub struct TAY;
impl Implied for TAY {
    const IMPLIED: u8 = 0xa8u8;
}

pub struct TSX;
impl Implied for TSX {
    const IMPLIED: u8 = 0xbau8;
}

pub struct TXA;
impl Implied for TXA {
    const IMPLIED: u8 = 0x8au8;
}

pub struct TXS;
impl Implied for TXS {
    const IMPLIED: u8 = 0x9au8;
}

pub struct TYA;
impl Implied for TYA {
    const IMPLIED: u8 = 0x98u8;
}
