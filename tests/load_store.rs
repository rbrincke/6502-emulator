extern crate nes;

use nes::processor::instructions::opcodes::{Immediate, LDA, LDX, LDY, STA, STX, STY, ZeroPage};
use nes::processor::registers::Registers;

use crate::common::{test, TestAssertions};

mod common;

#[test]
fn test_lda() {
    test_ld::<LDA, _>(|r| r.accumulator);
}

#[test]
fn test_ldx() {
    test_ld::<LDX, _>(|r| r.x);
}

#[test]
fn test_ldy() {
    test_ld::<LDY, _>(|r| r.y);
}

fn test_ld<T : Immediate, F : Fn(Registers) -> u8>(f: F) {
    let core = test(vec![
        T::immediate(0b10101010)
    ]);

    assert_eq!(f(core.registers), 0b10101010);
}

const VALUE: u8 = 0b10101010;
const ADDRESS: u8 = 0x1;

#[test]
fn test_sta() {
    let core = test(vec![
        LDA::immediate(VALUE),   // Populate Acc
        STA::zero_page(ADDRESS), // Acc -> Addr
        LDX::zero_page(ADDRESS)  // Addr -> X
    ]);

    assert_eq!(core.registers.x, VALUE);
}

#[test]
fn test_stx() {
    let core = test(vec![
        LDX::immediate(VALUE),   // Populate X
        STX::zero_page(ADDRESS), // X -> Addr
        LDY::zero_page(ADDRESS)  // Addr -> Y
    ]);

    assert_eq!(core.registers.y, VALUE);
}

#[test]
fn test_sty() {
    let core = test(vec![
        LDY::immediate(VALUE),   // Populate Y
        STY::zero_page(ADDRESS), // Y -> Addr
        LDX::zero_page(ADDRESS)  // Addr -> X
    ]);

    assert_eq!(core.registers.x, VALUE);
}
