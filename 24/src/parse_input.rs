use std::{collections::HashMap, str::from_utf8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wire(pub [u8; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display(style = "UPPERCASE")]
pub enum GateKind {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display("{left} {kind} {right}")]
pub struct GateInput {
    pub left: Wire,
    pub kind: GateKind,
    pub right: Wire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display("{input} -> {output}")]
pub struct Gate {
    pub input: GateInput,
    pub output: Wire,
}

impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            from_utf8(&self.0).unwrap_or_else(|e| { panic!("Invalid wire: {}", e) })
        )
    }
}

impl std::str::FromStr for Wire {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() != 3 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid wire length",
            )));
        }

        Ok(Wire([bytes[0], bytes[1], bytes[2]]))
    }
}

pub fn parse_input(input: &str) -> (HashMap<Wire, bool>, Vec<Gate>) {
    let [init_values, gates] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        panic!("invalid input, expected two sections separated by a newline");
    };

    let mut wires = HashMap::new();

    for line in init_values.lines() {
        let [wire, signal] = line.split(": ").collect::<Vec<_>>()[..] else {
            panic!("Invalid input line format");
        };

        wires.insert(wire.parse::<Wire>().unwrap(), signal == "1");
    }

    let gates: Vec<_> = gates
        .lines()
        .map(|line| {
            line.parse::<Gate>().unwrap_or_else(|e| {
                panic!("Error parsing gate: {}", e);
            })
        })
        .collect();

    (wires, gates)
}
