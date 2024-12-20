use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    #[display("#")]
    Wall,
    #[display(".")]
    Empty,
    #[display("S")]
    Start,
    #[display("E")]
    End,
    #[display("1")]
    Cheat1,
    #[display("2")]
    Cheat2,
}
