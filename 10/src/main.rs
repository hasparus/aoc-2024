// notes
// - if two trailheads are reachable from each other, they have the same score

// we'll keep a HashMap of trailhead position to score (or SameAs(Trailhead))

mod grid;
use std::collections::{HashMap, HashSet, VecDeque};

use grid::Grid;

type Height = u32;
type Position = (u32, u32);

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Trailhead(Position);

type Score = u64;

const TRAILHEAD_HEIGHT: Height = 0;
const SUMMIT_HEIGHT: Height = 9;
const UNREACHABLE_HEIGHT: Height = 100;

fn parse_input(input: &str) -> (Grid<Height>, Vec<Trailhead>) {
    let mut trailheads = Vec::new();

    let grid = Grid::new(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .filter(|c| !c.is_whitespace())
                    .enumerate()
                    .map(|(col, c)| {
                        let digit = c.to_digit(10).unwrap_or(UNREACHABLE_HEIGHT);

                        if digit == TRAILHEAD_HEIGHT {
                            trailheads.push(Trailhead((row as u32, col as u32)));
                        }
                        digit
                    })
                    .collect()
            })
            .collect(),
        100,
    );

    (grid, trailheads)
}

mod ex1 {
    use super::*;

    pub fn solve(input: &str) -> u64 {
        let (grid, trailheads) = parse_input(input);
        let mut scores = HashMap::<Trailhead, Score>::new();

        for trailhead in trailheads {
            if scores.contains_key(&trailhead) {
                continue;
            }

            let mut score = 0;
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((trailhead.0, 0));

            while let Some((pos, height)) = queue.pop_front() {
                if !visited.insert(pos) {
                    // we continue if the position was already visited
                    continue;
                }

                if height == SUMMIT_HEIGHT {
                    score += 1;
                    continue;
                }

                for (dc, dr) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_pos = (pos.0 as isize + dc, pos.1 as isize + dr);
                    let new_height = grid.get(new_pos.0, new_pos.1);

                    if *new_height == height + 1 {
                        queue.push_back(((new_pos.0 as u32, new_pos.1 as u32), *new_height));
                    }
                }
            }

            scores.insert(trailhead, score);
        }

        scores.values().sum()
    }
}

mod ex2 {
    use super::*;

    pub fn solve(input: &str) -> u64 {
        let (grid, trailheads) = parse_input(input);
        let mut paths = vec![vec![0u64; grid.width()]; grid.height()];

        for trailhead in trailheads {
            paths[trailhead.0 .0 as usize][trailhead.0 .1 as usize] = 1;
        }

        for height in 0..SUMMIT_HEIGHT {
            for (row, col, h) in grid.cells() {
                if *h != height {
                    continue;
                }

                let current_paths = paths[row as usize][col as usize];
                for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_row = row + dr;
                    let new_col = col + dc;

                    if !grid.in_bounds(new_row, new_col) {
                        continue;
                    }

                    if grid[(new_row, new_col)] == height + 1 {
                        paths[new_row as usize][new_col as usize] += current_paths;
                    }
                }
            }
        }

        grid.cells().fold(0, |acc, (r, c, h)| {
            if *h == SUMMIT_HEIGHT {
                acc + paths[r as usize][c as usize]
            } else {
                acc
            }
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", ex1::solve(&input));
    println!("{}", ex2::solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_trivial() {
        let input = "\
            0123
            1234
            8765
            9876
        ";
        assert_eq!(ex1::solve(input), 1);
    }

    #[test]
    fn test_two_summits() {
        let input = "\
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        ";
        assert_eq!(ex1::solve(input), 2);
    }

    #[test]
    fn test_all_reachable() {
        let input = "\
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
        ";
        assert_eq!(ex1::solve(input), 4);
    }

    #[test]
    fn test_example() {
        assert_eq!(ex1::solve(EXAMPLE), 36);
    }

    #[test]
    fn test_simple_ex2() {
        let input = "\
            .....0.
            ..4321.
            ..5..2.
            ..6543.
            ..7..4.
            ..8765.
            ..9....
        ";

        assert_eq!(ex2::solve(input), 3);
    }

    #[test]
    fn test_example_ex2() {
        assert_eq!(ex2::solve(EXAMPLE), 81);
    }
}
