use std::str::FromStr;
use regex::Regex;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub struct ParsedInstruction<'a> {
    pub instruction: &'a str,
    pub address: Option<ParsedAddress>
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParsedAddress {
    Empty,
    AbsoluteX { value: u16 },
    Immediate { value: u8 },
    ZeroPageOrRelative { value: u8 }, // At the parsing stage this is ambiguous.
    Unknown
}

struct AddressParser {
    immediate: ParserRule,
    absolute_indexed_x: ParserRule
}

struct ParserRule {
    matcher: Regex,
    creator: fn(&str, u32) -> ParsedAddress
}

impl AddressParser {
    fn new() -> Self {
        AddressParser {
            immediate: ParserRule {
                matcher: Regex::new(r"^#([$%]?[a-zA-Z0-9]+)").unwrap(),
                creator: |ss, r| ParsedAddress::Immediate { value: u8::from_str_radix(ss, r).unwrap() }
            },
            absolute_indexed_x: ParserRule {
                matcher: Regex::new(r"^([$%]?[a-zA-Z0-9]+),X").unwrap(),
                creator: |ss, r| ParsedAddress::AbsoluteX { value: u16::from_str_radix(ss, r).unwrap() }
            }
        }
    }

    /// Parse string as number, returning the radix and the remaining relevant string.
    fn parse_number(s: &str) -> (u32, &str) {
        match s {
            s if s.starts_with("%") => (2, &s[1..]),
            s if s.starts_with("$") => (16, &s[1..]),
            _ => (10, s)
        }
    }

    fn parse(&self, s: &str) -> Result<ParsedAddress, String> {
        let f = vec![&self.immediate, &self.absolute_indexed_x];

        for p in f {
            if p.matcher.is_match(s) {
                let (rdx, str) = Self::parse_number(p.matcher.captures(s).unwrap().get(1).unwrap().as_str());
                return Ok((p.creator)(str, rdx));
            }
        }

        return Err(format!("Could not parse address '{}'.", s));
    }
}

pub fn parse_instructions(str: &str) -> Vec<ParsedInstruction> {
    let address_parser = AddressParser::new();

    str.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(line_number, line)| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let instruction = parts.get(0).unwrap();
            let address = parts
                .get(1)
                .map(|e| address_parser
                    .parse(e)
                    .unwrap_or_else(|e|
                        panic!(
                            format!("Error on line {}: {}", line_number, e)
                        )
                    )
                );

            ParsedInstruction {
                instruction,
                address,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::assembler::parser::{parse_instructions, ParsedInstruction, AddressParser, ParsedAddress};
    use crate::assembler::parser::ParsedAddress::{Immediate, AbsoluteX};

    #[test]
    fn test_parse_decimal_address() {
        assert_eq!(
            AddressParser::parse_number("012"),
            (10, "012")
        )
    }

    #[test]
    fn test_parse_binary_address() {
        assert_eq!(
            AddressParser::parse_number("%0011"),
            (2, "0011")
        )
    }

    #[test]
    fn test_parse_hex_address() {
        assert_eq!(
            AddressParser::parse_number("$BE"),
            (16, "BE")
        )
    }

    #[test]
    fn test_address_absolute_x() {
        let ap = AddressParser::new();
        assert_eq!(
            ap.parse("$3000,X"),
            Ok(ParsedAddress::AbsoluteX { value: 0x3000 })
        )
    }

    #[test]
    fn test_parse_number_formats() {
        let lines = parse_instructions(r#"
            LDA #%10
            LDA #$12
            LDA #024
        "#);

        assert_eq!(
            vec![
                ParsedInstruction { instruction: "LDA", address: Some(Immediate { value: 2 }) },
                ParsedInstruction { instruction: "LDA", address: Some(Immediate { value: 18 }) },
                ParsedInstruction { instruction: "LDA", address: Some(Immediate { value: 24 }) }
            ],
            lines
        )
    }

    #[test]
    fn test_memory_formats() {
        let lines = parse_instructions(r#"
            LDA #12
            STA $3000,X
        "#);

        assert_eq!(
            vec![
                ParsedInstruction { instruction: "LDA", address: Some(Immediate { value: 12 }) },
                ParsedInstruction { instruction: "STA", address: Some(AbsoluteX { value: 0x3000 }) }
            ],
            lines
        )
    }
}
