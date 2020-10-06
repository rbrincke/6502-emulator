extern crate nes;

use nes::assembler::parser::{parse_instructions, ParsedInstruction};
use nes::assembler::parser::ParsedAddress::*;

#[test]
fn test_parse() {
    let lines = parse_instructions(r#"
        ASL
        LDA #012
        LDX #024
    "#);

    assert_eq!(
        vec![
            ParsedInstruction { instruction: "ASL", address: None },
            ParsedInstruction { instruction: "LDA", address: Some(Immediate { value: 12 }) },
            ParsedInstruction { instruction: "LDX", address: Some(Immediate { value: 24 }) }
        ],
        lines
    )
}
