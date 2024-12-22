use crate::ex1::{parse_input, Point};
use std::collections::HashSet;

use pathfinding::{matrix::directions::DIRECTIONS_4, prelude::bfs};

pub fn solve(input: &str, size: u8) -> (u8, u8) {
    let points = parse_input(input);

    // we'll use binary search to find the minimum number of bytes to fall to cut the path
    let mut low = 0;
    let mut high = points.len();

    while low < high {
        let mid = (low + high) / 2;

        let path = find_path(size, points[0..mid].iter().cloned().collect());
        if path.is_some() {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    println!("result: {:?}", points[low - 1]);

    points[low - 1]
}

fn find_path(size: u8, fallen_bytes: HashSet<Point>) -> Option<Vec<Point>> {
    bfs(
        &(0, 0),
        |p| {
            let neighbors = DIRECTIONS_4
                .iter()
                .map(|d| (p.0 as isize + d.0, p.1 as isize + d.1));

            neighbors
                .filter(|p| {
                    let on_board =
                        p.0 <= size as isize && p.1 <= size as isize && p.0 >= 0 && p.1 >= 0;

                    on_board && !fallen_bytes.contains(&(p.0 as u8, p.1 as u8))
                })
                .map(|p| (p.0 as u8, p.1 as u8))
                .collect::<Vec<_>>()
        },
        |p| *p == (size, size),
    )
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
        let result = solve(&input, 6);
        assert_eq!(result, (6, 1));
    }
}
