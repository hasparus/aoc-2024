use aoc_2024_lib::{board::Board, point2::Point2};
use parse_display::{Display, FromStr};
use pathfinding::matrix::directions::{self, DIRECTIONS_4};
use pathfinding::prelude::dijkstra;

type Direction = (isize, isize);

static INITIAL_DIRECTION: Direction = directions::E;

static STEP_COST: usize = 1;
static TURN_COST: usize = 1000;

static EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

#[derive(Debug, Display, FromStr, PartialEq, Eq)]
enum Cell {
    #[display("#")]
    Wall,
    #[display(".")]
    Empty,
    #[display("S")]
    Start,
    #[display("E")]
    End,
}

pub fn parse_maze(maze: &str) -> Board<Cell> {
    Board(
        maze.lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_string()
                            .parse()
                            .unwrap_or_else(|e| panic!("Invalid token: `{e}`"))
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn shortest_path_cost(maze: &Board<Cell>) -> Option<usize> {
    let start = maze.find(&Cell::Start);
    let end = maze.find(&Cell::End);

    dijkstra(
        &(start, INITIAL_DIRECTION),
        |(pos, direction)| {
            DIRECTIONS_4
                .iter()
                .filter_map(|d| {
                    let neighbor = *pos + d;
                    if maze[neighbor] == Cell::Wall {
                        None
                    } else {
                        let cost = if direction == d {
                            STEP_COST
                        } else {
                            TURN_COST + STEP_COST
                        };
                        Some(((neighbor, *d), cost))
                    }
                })
                .collect::<Vec<_>>()
        },
        |&(pos, _)| pos == end,
    )
    .map(|(_, cost)| cost)
}

pub fn solve(input: &str) -> usize {
    let maze = parse_maze(input);
    shortest_path_cost(&maze).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_maze() {
        let maze = parse_maze(EXAMPLE);
        println!("{}", maze);
        assert_eq!(maze.0.len(), 15);
        assert_eq!(maze.0[0].len(), 15);
        assert_eq!(maze.to_string(), EXAMPLE);
    }

    #[test]
    fn test_solve_example() {
        let maze = parse_maze(EXAMPLE);
        let cost = shortest_path_cost(&maze);
        assert_eq!(cost, Some(7036));
    }
}
