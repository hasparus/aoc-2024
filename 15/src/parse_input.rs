use parse_display::{Display, FromStr};

use crate::point2::Point2;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map(pub Vec<Vec<Token>>);

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            writeln!(
                f,
                "{}",
                row.iter().map(|t| t.to_string()).collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for Map {
    type Output = Vec<Token>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::Index<Point2> for Map {
    type Output = Token;

    fn index(&self, index: Point2) -> &Self::Output {
        &self.0[index.row][index.col]
    }
}

impl std::ops::IndexMut<Point2> for Map {
    fn index_mut(&mut self, index: Point2) -> &mut Self::Output {
        &mut self.0[index.row][index.col]
    }
}

impl std::ops::Index<&Point2> for Map {
    type Output = Token;

    fn index(&self, index: &Point2) -> &Self::Output {
        &self.0[index.row][index.col]
    }
}

impl std::ops::IndexMut<&Point2> for Map {
    fn index_mut(&mut self, index: &Point2) -> &mut Self::Output {
        &mut self.0[index.row][index.col]
    }
}

impl Map {
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Token>> {
        self.0.iter()
    }

    pub fn from_lines(lines: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let tokens = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| {
                        c.to_string()
                            .parse::<Token>()
                            .or(Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Invalid input: {}", c),
                            ))))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Map(tokens))
    }
}

impl std::str::FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Map::from_lines(&s.lines().collect::<Vec<_>>())
    }
}

pub struct Input {
    pub map: Map,
    pub moves: Vec<Direction>,
}

pub fn parse_input(input: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let trimmed = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    // until an empty line, we have the map, after that we have the moves
    let separator = trimmed
        .iter()
        .position(|line| line.is_empty())
        .ok_or("Expected map and moves sections separated by double newline")?;

    let (map, moves) = trimmed.split_at(separator);

    let moves = moves[1];

    let moves: Vec<Direction> = moves
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
        map: Map::from_lines(map)?,
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
            Map(vec![
                vec![Token::Wall, Token::Empty,],
                vec![Token::Box, Token::Robot]
            ])
        );
        assert_eq!(input.moves, vec![Direction::Right]);
        Ok(())
    }
}
