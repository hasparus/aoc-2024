use std::collections::HashMap;

use aoc_2024_lib::board::Board;

use crate::{cell::Cell, cheat::Cheat, parse_board::parse_board};

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
            print_board_with_cheat(board.clone(), cheat);
        }

        if cheats.len() == 0 {
            for (time, cheats) in cheats_by_time.iter() {
                println!("Cheats saving {time} picoseconds:");
                for cheat in cheats {
                    print_board_with_cheat(board.clone(), cheat);
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

pub fn print_board_with_cheat(board: Board<Cell>, cheat: &Cheat) {
    let mut board = board.clone();

    let between1 = (cheat.start + cheat.end) / 2;
    let between2 = (between1 + cheat.end) / 2;

    board[between1] = Cell::Cheat1;

    if between1 == between2 {
        board[cheat.end] = Cell::Cheat2;
    } else {
        board[between2] = Cell::Cheat2;
    }

    println!("{}", board);
}
