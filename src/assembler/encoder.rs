use std::collections::HashMap;

use crate::assembler::parser::{ParsedAddress, ParsedInstruction};
use crate::processor::instructions::opcodes::*;

struct AddressMapping {
    none: Option<fn() -> Vec<u8>>,
    immediate: Option<fn(u8) -> Vec<u8>>,
    zero_page_or_relative: Option<fn(u8) -> Vec<u8>>
}

impl AddressMapping {
    fn new() -> Self {
        AddressMapping {
            none: None,
            immediate: None,
            zero_page_or_relative: None
        }
    }

    fn none(mut self, none: fn() -> Vec<u8>) -> Self {
        self.none = Some(none);
        self
    }

    fn immediate(mut self, immediate: fn(u8) -> Vec<u8>) -> Self {
        self.immediate = Some(immediate);
        self
    }

    fn zeropage_or_relative(mut self, apply: fn(u8) -> Vec<u8>) -> Self {
        self.immediate = Some(apply);
        self
    }
}

pub struct Encoder {
    mapping: HashMap<&'static str, AddressMapping>
}

impl Encoder {
    fn build_parse_map() -> HashMap<&'static str, AddressMapping> {
        hashmap! {
            "ASL" => AddressMapping::new().none(ASL::accumulator).zeropage_or_relative(ASL::zero_page),
            "CLC" => AddressMapping::new().none(CLC::implied),
            "LDA" => AddressMapping::new().immediate(LDA::immediate),
            "LDX" => AddressMapping::new().immediate(LDX::immediate),
            "LDY" => AddressMapping::new().immediate(LDY::immediate)
        }
    }

    pub fn new() -> Self {
        Encoder {
            mapping: Encoder::build_parse_map()
        }
    }

    pub fn encode_instruction(&self, parsed_instr: &ParsedInstruction) -> Option<Vec<u8>> {
        let instruction_configuration = self.mapping.get(parsed_instr.instruction).unwrap();

        match parsed_instr.address {
            None => instruction_configuration.none.map(|f| f()),
            Some(a) => match a {
                ParsedAddress::Immediate { value } => instruction_configuration.immediate.map(|f| f(value)),
                ParsedAddress::ZeroPageOrRelative { value } => instruction_configuration.zero_page_or_relative.map(|f| f(value)),
                _ => None
            }
        }
    }
}
