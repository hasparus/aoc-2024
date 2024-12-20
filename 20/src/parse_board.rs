use aoc_2024_lib::board::Board;

use crate::cell::Cell;

pub fn parse_board(input: &str) -> Board<Cell> {
    Board(
        input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| {
                        c.to_string().parse().unwrap_or_else(|_| {
                            panic!("Invalid token in racetrack {} `{}`", line, c)
                        })
                    })
                    .collect()
            })
            .collect(),
    )
}
