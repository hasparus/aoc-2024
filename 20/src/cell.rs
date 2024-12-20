use parse_display::{Display, DisplayFormat, FromStr, FromStrFormat};

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
    #[display("{index}")]
    Path {
        #[display(with = SingleCharNumber {})]
        index: usize,
    },
}

struct SingleCharNumber {}

impl DisplayFormat<usize> for SingleCharNumber {
    fn write(&self, f: &mut std::fmt::Formatter, value: &usize) -> std::fmt::Result {
        if *value > 9 {
            write!(f, "\x1b[7m{}\x1b[0m", value % 10)
        } else {
            write!(f, "{}", value)
        }
    }
}

impl FromStrFormat<usize> for SingleCharNumber {
    type Err = <usize as std::str::FromStr>::Err;
    fn parse(&self, s: &str) -> std::result::Result<usize, Self::Err> {
        s.parse::<usize>()
    }
}
