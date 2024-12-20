use crate::cell::Cell;
use crate::parse_board::parse_board;
use aoc_2024_lib::{board::Board, point2::Point2};
use pathfinding::prelude::bfs;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cheat {
    start: Point2,
    end: Point2,
    length_saved: usize,
}

fn compute_distances_to_end(board: &Board<Cell>, end: Point2) -> HashMap<Point2, usize> {
    let mut distances = HashMap::new();
    let mut queue = vec![(end, 0)];
    distances.insert(end, 0);

    while let Some((pos, dist)) = queue.pop() {
        for &dir in pathfinding::matrix::directions::DIRECTIONS_4.iter() {
            let next = pos + dir;
            if board[next] != Cell::Wall && !distances.contains_key(&next) {
                distances.insert(next, dist + 1);
                queue.push((next, dist + 1));
            }
        }
    }

    distances
}

pub fn find_all_cheats(input: &str, min_saved: usize) -> Vec<Cheat> {
    let board = parse_board(input);
    let start = board.find(&Cell::Start);
    let end = board.find(&Cell::End);

    // Get the original shortest path
    let path = bfs(
        &start,
        |pos| {
            pathfinding::matrix::directions::DIRECTIONS_4
                .iter()
                .filter_map(|d| {
                    let neighbor = *pos + d;
                    if board[neighbor] == Cell::Wall {
                        None
                    } else {
                        Some(neighbor)
                    }
                })
                .collect::<Vec<_>>()
        },
        |&pos| pos == end,
    )?;

    let original_length = path.len();

    // Compute distances from every point to the end
    let distances_to_end = compute_distances_to_end(&board, end);

    // TODO: For each point on the path:
    // - Find all reachable points within 20 steps
    // - Calculate potential time savings
    // - Add valid cheats to results

    vec![]
}

pub fn solve(input: &str, min_saved: usize) -> usize {
    find_all_cheats(input, min_saved).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2024_lib::input_reader::read_input;

    #[test]
    fn test_example() {
        let inputs = read_input("./inputs.md").expect("Could not read input file");
        let example = &inputs.get_input("Example").content;

        let cheats = find_all_cheats(&example, 50);
        let mut by_time: HashMap<usize, Vec<&Cheat>> = HashMap::new();
        for cheat in &cheats {
            by_time.entry(cheat.length_saved).or_default().push(cheat);
        }

        assert_eq!(by_time.get(&50).map(|v| v.len()), Some(32));
        assert_eq!(by_time.get(&76).map(|v| v.len()), Some(3));
    }
}
