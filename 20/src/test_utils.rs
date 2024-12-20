use crate::{cell::Cell, cheat::Cheat, parse_board::parse_board};
use aoc_2024_lib::{board::Board, point2::Point2};
use pathfinding::{matrix::directions::DIRECTIONS_4, prelude::bfs};
use std::collections::HashMap;

pub fn assert_is_cheat(is_cheat: fn(&Board<Cell>, &Cheat) -> bool, input: &str, expected: bool) {
    let board = parse_board(input);
    let start = board.find(&Cell::Start);
    let end = board.find(&Cell::End);

    assert_eq!(
        is_cheat(
            &board,
            &Cheat {
                start,
                end,
                length_saved: 0
            }
        ),
        expected,
        "input: {}",
        input
    );
}

pub fn assert_cheat_count(
    board: &Board<Cell>,
    cheats_by_time: &HashMap<usize, Vec<&Cheat>>,
    time_saved: usize,
    expected_count: usize,
) {
    let empty = Vec::new();
    let cheats = cheats_by_time.get(&time_saved).unwrap_or(&empty);

    if cheats.len() != expected_count {
        for cheat in cheats {
            print_board_with_cheat(board, cheat);
        }

        if cheats.len() == 0 {
            for (time, cheats) in cheats_by_time.iter() {
                println!("Cheats saving {time} picoseconds:");
                for cheat in cheats {
                    print_board_with_cheat(board, cheat);
                }
            }
        }
        panic!(
            "expected {} cheats saving {} picoseconds, got {}",
            expected_count,
            time_saved,
            cheats.len()
        );
    }
}

pub fn print_board_with_cheat(board: &Board<Cell>, cheat: &Cheat) {
    let mut board = board.clone();

    let path = bfs(
        &cheat.start.into(),
        |point: &(isize, isize)| {
            DIRECTIONS_4
                .iter()
                .map(|dir| (point.0 + dir.0, point.1 + dir.1))
                .collect::<Vec<_>>()
        },
        |point| *point == cheat.end.into(),
    );

    for (index, point) in path.unwrap().iter().enumerate().skip(1) {
        board[Point2::from(point)] = Cell::Path { index };
    }

    println!("{}", board);
}

pub fn group_cheats_by_time_saved(cheats: &Vec<Cheat>) -> HashMap<usize, Vec<&Cheat>> {
    let mut cheats_by_time = HashMap::new();
    for cheat in cheats {
        cheats_by_time
            .entry(cheat.length_saved)
            .or_insert(vec![])
            .push(cheat);
    }
    cheats_by_time
}
