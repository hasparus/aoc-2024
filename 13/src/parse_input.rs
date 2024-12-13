#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("X{x}, Y{y}")]
pub struct ButtonPress {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("X={x}, Y={y}")]
pub struct Prize {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("Button A: {button_a}\nButton B: {button_b}\nPrize: {prize}")]
pub struct GameRound {
    pub button_a: ButtonPress,
    pub button_b: ButtonPress,
    pub prize: Prize,
}

pub struct Game {
    pub rounds: Vec<GameRound>,
}

impl std::str::FromStr for Game {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds = s
            .split("\n\n")
            .flat_map(|round| round.parse())
            .collect::<Vec<GameRound>>();

        Ok(Game { rounds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() -> Result<(), Box<dyn std::error::Error>> {
        let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";

        let game = input.parse::<Game>()?;
        assert_eq!(game.rounds.len(), 2);

        let first = &game.rounds[0];
        assert_eq!(first.button_a.x, 94);
        assert_eq!(first.button_a.y, 34);
        assert_eq!(first.button_b.x, 22);
        assert_eq!(first.button_b.y, 67);
        assert_eq!(first.prize.x, 8400);
        assert_eq!(first.prize.y, 5400);

        let second = &game.rounds[1];
        assert_eq!(second.button_a.x, 26);
        assert_eq!(second.button_a.y, 66);
        assert_eq!(second.button_b.x, 67);
        assert_eq!(second.button_b.y, 21);
        assert_eq!(second.prize.x, 12748);
        assert_eq!(second.prize.y, 12176);

        Ok(())
    }
}
