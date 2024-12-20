use aoc_2024_lib::board::Board;
use aoc_2024_lib::point2::Point2;
use pathfinding::matrix::directions::DIRECTIONS_4;
use pathfinding::prelude::bfs;
use std::collections::HashSet;

use crate::{cell::Cell, cheat::Cheat, parse_board::parse_board};

pub fn find_all_cheats(input: &str, min_length: usize) -> Vec<Cheat> {
    let board = parse_board(input);
    let start = board.find(&Cell::Start);
    let end = board.find(&Cell::End);

    let path = bfs(
        &start,
        |pos| {
            DIRECTIONS_4
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
    let path = path.unwrap();

    let original_path_length = path.len();
    let mut cheats = HashSet::new();

    // For each point on the path
    for (i, &point) in path.iter().enumerate() {
        if cfg!(debug_assertions) {
            println!(
                "considering point {}/{} {:?}",
                i, original_path_length, point
            );
        }

        // Check all possible cheat destinations in each direction
        for &dir in DIRECTIONS_4.iter() {
            for dist in 2..=3 {
                let dest = (
                    point.row as isize + dir.0 * dist,
                    point.col as isize + dir.1 * dist,
                );

                // Skip if out of bounds or if not a valid cheat
                if !board.in_bounds(dest)
                    || !is_cheat(
                        &board,
                        &Cheat {
                            start: point,
                            end: dest.into(),
                            length_saved: 0,
                        },
                    )
                {
                    continue;
                }

                // Find path from dest to end
                if let Some(shortcut_path) = bfs(
                    &dest.into(),
                    |pos: &Point2| {
                        DIRECTIONS_4
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
                ) {
                    let cheat_distance = if point.row as isize == dest.0 {
                        point.col.abs_diff(dest.1 as usize)
                    } else {
                        point.row.abs_diff(dest.0 as usize)
                    };

                    // Path length = steps to cheat start + cheat distance + steps from cheat end to finish
                    let new_total_path_length = i + cheat_distance + shortcut_path.len();
                    let path_delta = original_path_length as isize - new_total_path_length as isize;
                    if path_delta >= min_length as isize {
                        cheats.insert(Cheat {
                            start: point,
                            end: dest.into(),
                            length_saved: original_path_length - new_total_path_length,
                        });
                    }
                }
            }
        }
    }

    cheats.into_iter().collect()
}

/// a cheat must be on the same axis and there must be one or two walls between the two points
fn is_cheat(board: &Board<Cell>, cheat: &Cheat) -> bool {
    let dx = cheat.start.row.abs_diff(cheat.end.row);
    let dy = cheat.start.col.abs_diff(cheat.end.col);

    let distance = dx.max(dy);
    let is_horizontal = cheat.start.col == cheat.end.col;
    let is_vertical = cheat.start.row == cheat.end.row;

    // Must be on same axis and distance must be 2 or 3
    if (!is_horizontal && !is_vertical) || (distance != 2 && distance != 3) {
        return false;
    }

    // Check if the destination is a valid cell (not a wall)
    if board[cheat.end] == Cell::Wall {
        return false;
    }

    let midpoint = (cheat.start + cheat.end) / 2;
    let has_wall_in_middle = board[midpoint] == Cell::Wall;

    // For distance 3, require two walls
    if distance == 3 {
        let quarter = (cheat.start + midpoint) / 2;
        let three_quarters = (midpoint + cheat.end) / 2;
        return has_wall_in_middle
            && (board[quarter] == Cell::Wall || board[three_quarters] == Cell::Wall);
    }

    // For distance 2, require one wall
    has_wall_in_middle
}

pub fn solve(input: &str, min_length: usize) -> usize {
    find_all_cheats(input, min_length).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parse_board::parse_board,
        test_utils::{self, assert_cheat_count, group_cheats_by_time_saved},
    };
    use aoc_2024_lib::input_reader::read_input;
    use pretty_assertions::assert_eq;

    fn assert_is_cheat(input: &str, expected: bool) {
        test_utils::assert_is_cheat(is_cheat, input, expected);
    }

    #[test]
    fn horizontal_wall_between() {
        assert_is_cheat("S#E", true);
        assert_is_cheat("S##E", true);

        assert_is_cheat("S.#E", false);
        assert_is_cheat("S.#.E", false);
        assert_is_cheat("SE", false);
        assert_is_cheat("S.E", false);
        assert_is_cheat("S..E", false);
        assert_is_cheat("S...E", false);
        assert_is_cheat("S....E", false);

        assert_is_cheat(
            "
            S
            #
            E
            ",
            true,
        );
        assert_is_cheat(
            "
            S
            #
            #
            E
            ",
            true,
        );

        assert_is_cheat(
            "
            S
            E
            ",
            false,
        );
        assert_is_cheat(
            "
            S
            .
            #
            E
            ",
            false,
        );
        assert_is_cheat(
            "
            S
            .
            E
            ",
            false,
        );
        assert_is_cheat(
            "
            S
            .
            .
            E
            ",
            false,
        );
        assert_is_cheat(
            "
            S
            .
            .
            .
            E
            ",
            false,
        );
    }

    #[test]
    fn finds_simple_cheats() {
        let input = "
            ######
            #S##E#
            #....#
            ######
        ";
        let cheats = find_all_cheats(input, 2);
        let by_time = group_cheats_by_time_saved(&cheats);
        let board = parse_board(input);

        assert_cheat_count(&board, &by_time, 2, 1);

        let input = "
            ######
            #S.#E#
            ##.#.#
            ##.#.#
            ##.#.#
            ##.#.#
            ##...#
            ######
        ";
        let cheats = find_all_cheats(input, 2);
        let board = parse_board(input);
        let by_time = group_cheats_by_time_saved(&cheats);

        assert_cheat_count(&board, &by_time, 10, 1);
        assert_cheat_count(&board, &by_time, 8, 1);
        assert_cheat_count(&board, &by_time, 6, 1);
        assert_cheat_count(&board, &by_time, 4, 1);
        assert_cheat_count(&board, &by_time, 2, 1);
    }

    #[test]
    fn finds_more_cheats() {
        let input = "
            #######
            #.....#
            #.###.#
            #S#..E#
            #######
        ";
        let board = parse_board(input);
        let cheats = find_all_cheats(input, 2);
        let by_time = group_cheats_by_time_saved(&cheats);

        println!("{:?}", by_time);

        assert_eq!(by_time.len(), 1);

        assert_cheat_count(&board, &by_time, 4, 1);
    }

    #[test]
    fn finds_all_cheats_in_example_racetrack() {
        let inputs = read_input("./inputs.md").unwrap();
        let example = &inputs.get_input("Example").content;
        let board = parse_board(example);
        let cheats = find_all_cheats(example, 2);
        let by_time = group_cheats_by_time_saved(&cheats);

        assert_cheat_count(&board, &by_time, 2, 14);
        // assert_cheat_count(&board, &by_time, 4, 14);
        assert_cheat_count(&board, &by_time, 6, 2);
        assert_cheat_count(&board, &by_time, 8, 4);
        assert_cheat_count(&board, &by_time, 10, 2);
        assert_cheat_count(&board, &by_time, 12, 3);
        assert_cheat_count(&board, &by_time, 36, 1);
        assert_cheat_count(&board, &by_time, 38, 1);
        assert_cheat_count(&board, &by_time, 40, 1);
        assert_cheat_count(&board, &by_time, 64, 1);
    }
}
