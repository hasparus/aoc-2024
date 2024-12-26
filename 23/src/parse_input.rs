use std::str::from_utf8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct Computer(pub u16);

impl Computer {
    pub fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        Self(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn can_be_chief_historian(&self) -> bool {
        (self.0 & 0xFF) == b't' as u16
    }
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.0.to_le_bytes()).unwrap())
    }
}

impl std::str::FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Computer::new(s))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, parse_display::Display, parse_display::FromStr)]
#[display("{0}-{1}")]
pub struct Connection(pub Computer, pub Computer);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_from_str() {
        assert_eq!("ab".parse::<Computer>().unwrap(), Computer::new("ab"));
        assert_eq!("tj".parse::<Computer>().unwrap().to_string(), "tj");
        assert_eq!(Computer::new("tk").to_string(), "tk");
    }
}
