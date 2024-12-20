use crate::cell::Cell;
use crate::cheat::Cheat;
use crate::parse_board::parse_board;
use aoc_2024_lib::{board::Board, point2::Point2};
use pathfinding::prelude::bfs;
use std::collections::HashMap;

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
    assert!(min_saved > 1, "min_saved must be greater than 1");

    let board = parse_board(input);
    let start = board.find(&Cell::Start);
    let end = board.find(&Cell::End);

    fn manhattan_distance(a: Point2, b: Point2) -> usize {
        a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
    }

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
    );

    if path.is_none() {
        return vec![];
    }
    let path = path.expect("No path found");
    let original_length = path.len() - 1;
    let distances_to_end = compute_distances_to_end(&board, end);

    fn cells_within_distance(
        start: Point2,
        distance: usize,
        board: &Board<Cell>,
    ) -> impl Iterator<Item = Point2> + '_ {
        let min_row = start.row.saturating_sub(distance);
        let max_row = (start.row + distance).min(board.height() - 1);
        let min_col = start.col.saturating_sub(distance);
        let max_col = (start.col + distance).min(board.width() - 1);

        (min_row..=max_row)
            .flat_map(move |row| (min_col..=max_col).map(move |col| Point2::new(row, col)))
            .filter(move |p| manhattan_distance(start, *p) <= distance)
    }

    let mut cheats = Vec::new();

    for (i, &start_pos) in path.iter().enumerate() {
        for end_pos in cells_within_distance(start_pos, 20, &board) {
            if board[end_pos] == Cell::Wall {
                continue;
            }

            if let Some(&dist_to_end) = distances_to_end.get(&end_pos) {
                let cheat_length = manhattan_distance(start_pos, end_pos);
                let new_total_length = i + cheat_length + dist_to_end;
                let saved = original_length.saturating_sub(new_total_length);

                if saved >= min_saved {
                    cheats.push(Cheat {
                        start: start_pos,
                        end: end_pos,
                        length_saved: saved,
                    });
                }
            }
        }
    }

    cheats
}

pub fn solve(input: &str, min_saved: usize) -> usize {
    find_all_cheats(input, min_saved).len()
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{
        assert_cheat_count, group_cheats_by_time_saved, print_board_with_cheat,
    };

    use super::*;
    use aoc_2024_lib::input_reader::read_input;

    #[test]
    fn test_super_trivial() {
        let input = "
            #####
            #S#E#
            #...#
            #####
        ";
        let cheats = find_all_cheats(input, 2);

        let board = &parse_board(input);
        for cheat in cheats.iter() {
            print_board_with_cheat(board, cheat);
        }

        assert_eq!(cheats.len(), 1);
    }

    #[test]
    fn test_trivial() {
        let input = "
            ######
            #S##E#
            #....#
            ######
        ";
        let board = &parse_board(input);
        let cheats = find_all_cheats(input, 2);

        for cheat in cheats.iter() {
            print_board_with_cheat(board, cheat);
        }
        assert_eq!(cheats.len(), 1);
    }

    #[test]
    fn test_example() {
        let inputs = read_input("./inputs.md").expect("Could not read input file");
        let example = &inputs.get_input("Example").content;

        let board = &parse_board(example);
        let cheats = find_all_cheats(example, 49);
        let by_time = &group_cheats_by_time_saved(&cheats);

        // There are 32 cheats that save 50 picoseconds.
        assert_cheat_count(board, by_time, 50, 32);
        // There are 31 cheats that save 52 picoseconds.
        assert_cheat_count(board, by_time, 52, 31);
        // There are 29 cheats that save 54 picoseconds.
        assert_cheat_count(board, by_time, 54, 29);
        // There are 39 cheats that save 56 picoseconds.
        assert_cheat_count(board, by_time, 56, 39);
        // There are 25 cheats that save 58 picoseconds.
        assert_cheat_count(board, by_time, 58, 25);
        // There are 23 cheats that save 60 picoseconds.
        assert_cheat_count(board, by_time, 60, 23);
        // There are 20 cheats that save 62 picoseconds.
        assert_cheat_count(board, by_time, 62, 20);
        // There are 19 cheats that save 64 picoseconds.
        assert_cheat_count(board, by_time, 64, 19);
        // There are 12 cheats that save 66 picoseconds.
        assert_cheat_count(board, by_time, 66, 12);
        // There are 14 cheats that save 68 picoseconds.
        assert_cheat_count(board, by_time, 68, 14);
        // There are 12 cheats that save 70 picoseconds.
        assert_cheat_count(board, by_time, 70, 12);
        // There are 22 cheats that save 72 picoseconds.
        assert_cheat_count(board, by_time, 72, 22);
        // There are 4 cheats that save 74 picoseconds.
        assert_cheat_count(board, by_time, 74, 4);
        // There are 3 cheats that save 76 picoseconds.
        assert_cheat_count(board, by_time, 76, 3);
    }
}
