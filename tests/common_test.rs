mod common;

use crate::common::{test_with_flags, TestAssertions};
use nes::processor::registers::Flag;

#[test]
fn test_setup_flags_no_flags() {
    let t = test_with_flags(vec![], vec![]);
    t.assert_flags_set(vec![]);
}

#[test]
fn test_setup_flags() {
    let v = vec![Flag::Carry, Flag::Overflow];
    let t = test_with_flags(vec![], v.clone());
    t.assert_flags_set(v);
}
