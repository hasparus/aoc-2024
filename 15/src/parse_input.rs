use parse_display::{Display, FromStr};

use aoc_2024_lib::board::Board;

#[derive(Debug, Display, FromStr, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    #[display("^")]
    Up,
    #[display("v")]
    Down,
    #[display("<")]
    Left,
    #[display(">")]
    Right,
}

#[derive(Debug, Display, FromStr, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    #[display("#")]
    Wall,
    #[display(".")]
    Empty,
    #[display("@")]
    Robot,
    #[display("O")]
    Box,
}

pub struct Input {
    pub map: Board<Token>,
    pub moves: Vec<Direction>,
}

pub fn parse_input(input: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let trimmed = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    let separator = trimmed
        .iter()
        .position(|line| line.is_empty())
        .ok_or("Expected map and moves sections separated by double newline")?;

    let (map, moves) = trimmed.split_at(separator);

    let moves: Vec<Direction> = moves
        .join("")
        .chars()
        .map(|c| {
            c.to_string()
                .parse::<Direction>()
                .or(Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid direction: {}", c),
                ))))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Input {
        map: Board::from_lines(map)?,
        moves,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = parse_input(
            "
                #.
                O@

                > 
            ",
        )?;

        assert_eq!(
            input.map,
            Board(vec![
                vec![Token::Wall, Token::Empty,],
                vec![Token::Box, Token::Robot]
            ])
        );
        assert_eq!(input.moves, vec![Direction::Right]);
        Ok(())
    }
}
