use crate::assembler::encoder::Encoder;
use crate::assembler::parser::parse_instructions;

fn assemble(str: &str) -> Vec<u8> {
    let parse_result = parse_instructions(str);
    let encoder = Encoder::new();

    parse_result.iter()
        .map(|instr| encoder.encode_instruction(instr).unwrap())
        .flatten()
        .collect::<Vec<u8>>()
}
