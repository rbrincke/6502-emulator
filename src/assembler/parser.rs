use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct ParsedInstruction<'a> {
    pub instruction: &'a str,
    pub address: Option<ParsedAddress>
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParsedAddress {
    Empty,
    Immediate { value: u8 },
    ZeroPageOrRelative { value: u8 }, // At the parsing stage this is ambiguous.
    Unknown
}

impl FromStr for ParsedAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with("#") => {
                let mut chars = s.chars(); chars.next();
                Ok(ParsedAddress::Immediate { value: u8::from_str(chars.as_str()).unwrap() })
            }

            _ => Err(())
        }
    }
}

pub fn parse_instructions(str: &str) -> Vec<ParsedInstruction> {
    str.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let instruction = parts.get(0).unwrap();
            let address = parts
                .get(1)
                .map(|e| ParsedAddress::from_str(e).unwrap());

            ParsedInstruction {
                instruction,
                address,
            }
        })
        .collect()
}
