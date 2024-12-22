use std::fmt::Display;

use aoc_2024_lib::board::Board;
use pathfinding::{matrix::directions::DIRECTIONS_4, prelude::bfs};

pub type Point = (u8, u8);

pub fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .trim()
                .split_once(',')
                .unwrap_or_else(|| panic!("the point should be separated by a comma {}", line));

            (
                x.parse()
                    .unwrap_or_else(|_| panic!("the point should be a number {}", x)),
                y.parse()
                    .unwrap_or_else(|_| panic!("the point should be a number {}", y)),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty = 0,
    FallenByte = 1,
    #[allow(dead_code)]
    Path = 2,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => '.',
                Cell::FallenByte => '#',
                Cell::Path => 'O',
            }
        )?;
        Ok(())
    }
}

pub fn solve(input: &str, size: u8, bytes_to_fall: usize) -> usize {
    let points = parse_input(input);

    let mut grid = Board::new(vec![
        vec![Cell::Empty; (size + 1) as usize];
        (size + 1) as usize
    ]);

    for point in points[0..bytes_to_fall].iter() {
        grid[*point] = Cell::FallenByte;
    }

    let path = bfs(
        &(0, 0),
        |p| {
            let neighbors = DIRECTIONS_4.iter().map(|d| (p.0 + d.0, p.1 + d.1));

            neighbors
                .filter(|p| {
                    let on_board =
                        p.0 <= size as isize && p.1 <= size as isize && p.0 >= 0 && p.1 >= 0;

                    on_board && grid[*p] == Cell::Empty
                })
                .collect::<Vec<_>>()
        },
        |p| *p == (size as isize, size as isize),
    )
    .expect("no path found");

    println!("{}", grid);
    path.len() - 1
}

#[cfg(test)]
mod tests {
    use aoc_2024_lib::input_reader::read_input;

    use super::*;

    fn get_input(name: &str) -> String {
        read_input("./inputs.md")
            .expect("input missing")
            .get_input(name)
            .content
            .clone()
    }

    #[test]
    fn test_example() {
        let input = get_input("Example");
        let result = solve(&input, 6, 12);
        assert_eq!(result, 22);
    }
}
