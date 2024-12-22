use std::ops::Index;

use parse_display::{Display, DisplayFormat, FromStr, FromStrFormat};

#[derive(Debug, Clone, Copy, Display, FromStr)]
pub struct Register(pub u64);

#[derive(Debug, Clone, Display, FromStr)]
#[display("Register A: {a}\nRegister B: {b}\nRegister C: {c}")]
pub struct Registers {
    pub a: Register,
    pub b: Register,
    pub c: Register,
}

#[derive(Debug, Clone, Display, FromStr)]
#[display("Program: {0}")]
pub struct Program(#[display(with = CommaSeparated)] pub Vec<u8>);

impl Index<usize> for Program {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Program {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

struct CommaSeparated;

impl DisplayFormat<Vec<u8>> for CommaSeparated {
    fn write(&self, f: &mut std::fmt::Formatter, value: &Vec<u8>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            value
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
impl FromStrFormat<Vec<u8>> for CommaSeparated {
    type Err = String;
    fn parse(&self, s: &str) -> std::result::Result<Vec<u8>, Self::Err> {
        s.split(',')
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|err| format!("Failed to parse u8 `{}`: {}", s, err))
            })
            .collect()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Display, FromStr)]
pub enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy, Display, FromStr)]
pub enum ComboOperand {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    A = 4,
    B = 5,
    C = 6,
    Reserved = 7,
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl ComboOperand {
    pub fn value(&self, registers: &Registers) -> u64 {
        match self {
            ComboOperand::Zero => 0,
            ComboOperand::One => 1,
            ComboOperand::Two => 2,
            ComboOperand::Three => 3,
            ComboOperand::A => registers.a.0,
            ComboOperand::B => registers.b.0,
            ComboOperand::C => registers.c.0,
            ComboOperand::Reserved => panic!("Reserved combo operand"),
        }
    }
}
