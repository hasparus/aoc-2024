// notes
// - if two trailheads are reachable from each other, they have the same score

// we'll keep a HashMap of trailhead position to score (or SameAs(Trailhead))

mod grid;
use std::collections::{HashMap, HashSet, VecDeque};

use grid::Grid;

type Height = u32;
type Position = (u32, u32);

#[derive(Clone, Hash, Eq, PartialEq)]
struct Trailhead(Position);
enum Score {
    SameAs(Trailhead),
    Score(u64),
}

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

fn sum_scores(scores: &HashMap<Trailhead, Score>) -> u64 {
    scores
        .values()
        .map(|score| match score {
            Score::Score(s) => *s,
            Score::SameAs(t) => match scores.get(t) {
                Some(Score::Score(s)) => *s,
                _ => panic!("Invalid score reference"),
            },
        })
        .sum()
}

fn solve(input: &str) -> u64 {
    let (grid, trailheads) = parse_input(input);
    let mut scores = HashMap::<Trailhead, Score>::new();

    for trailhead in trailheads {
        match scores.get(&trailhead) {
            Some(Score::SameAs(_)) => continue,
            Some(Score::Score(_)) => panic!("Trailhead already has a score"),
            None => {}
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

            if grid[pos] == 0 && pos != trailhead.0 {
                scores.insert(Trailhead(pos), Score::SameAs(trailhead.clone()));
                continue;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = pos.0 as i32 + dx;
                let new_y = pos.1 as i32 + dy;

                if !grid.in_bounds(new_x, new_y) {
                    continue;
                }

                let new_pos = (new_x as u32, new_y as u32);
                let new_height = grid[new_pos];

                // Only proceed if the height increases by exactly 1
                if new_height == height + 1 {
                    queue.push_back((new_pos, new_height));
                }
            }
        }

        scores.insert(trailhead, Score::Score(score));
    }

    sum_scores(&scores)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
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
        assert_eq!(solve(input), 1);
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
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_two_trailheads() {
        let input = "\
            ...0...
            ...1210
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        ";
        assert_eq!(solve(input), 2);
    }

    // #[test]
    // fn test_example() {
    //     assert_eq!(solve(EXAMPLE), 36);
    // }
}
