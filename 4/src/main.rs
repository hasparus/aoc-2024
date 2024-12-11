// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....
// The actual word search will be full of letters instead. For example:

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX
// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

// Take a look at the little Elf's word search. How many times does XMAS appear?

use std::fmt::Display;

const INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

#[derive(Debug)]
struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
    default: T,
}

impl<T: Clone + Display> Grid<T> {
    fn new(data: Vec<Vec<T>>, default: T) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Self {
            data,
            width,
            height,
            default,
        }
    }

    fn get(&self, row: isize, col: isize) -> &T {
        if row < 0 || col < 0 || row >= self.height as isize || col >= self.width as isize {
            &self.default
        } else {
            &self.data[row as usize][col as usize]
        }
    }

    fn cells(&self) -> impl Iterator<Item = (isize, isize, &T)> {
        (0..self.height).flat_map(move |row| {
            (0..self.width).map(move |col| (row as isize, col as isize, &self.data[row][col]))
        })
    }
}

impl<T: Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                result.push_str(&self.data[row][col].to_string());
            }
            if row < self.height - 1 {
                result.push('\n');
            }
        }
        write!(f, "{}", result)
    }
}

fn input_to_grid(input: &str, default: char) -> Grid<char> {
    Grid::new(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect(),
        default,
    )
}

fn solve_ex1(grid: &Grid<char>) -> usize {
    let mut count = 0;

    for (row, col, &val) in grid.cells() {
        if val != 'X' {
            continue;
        }

        for (dr, dc) in DIRECTIONS {
            let mut r = row;
            let mut c = col;
            for letter in "MAS".chars() {
                r += dr;
                c += dc;
                if grid.get(r, c) != &letter {
                    break;
                } else if letter == 'S' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_ex2(grid: Grid<char>) -> usize {
    let diagonals = [[(1, 1), (-1, -1)], [(1, -1), (-1, 1)]];

    grid.cells()
        .filter(|(row, col, &val)| {
            val == 'A'
                && diagonals
                    .iter()
                    .map(|diagonal| {
                        diagonal
                            .iter()
                            .map(|(dr, dc)| grid.get(row + dr, col + dc))
                            .collect::<Vec<_>>()
                    })
                    .all(|diag| diag.contains(&&'M') && diag.contains(&&'S'))
        })
        .count()
}

fn main() {
    let grid = input_to_grid(INPUT, '.');

    println!("{}", solve_ex1(&grid));
    println!("{}", solve_ex2(grid));

    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = input_to_grid(&input, '.');
    println!("{}", solve_ex1(&grid));
    println!("{}", solve_ex2(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_1() {
        let grid = input_to_grid("XMAS", '.');
        assert_eq!(solve_ex1(&grid), 1);
    }

    #[test]
    fn test_trivial_2() {
        let grid = input_to_grid("XMAS.XMAS", '.');
        assert_eq!(solve_ex1(&grid), 2);
    }

    #[test]
    fn test_trivial_reverse() {
        let grid = input_to_grid("XMASXMAS", '.');
        assert_eq!(solve_ex1(&grid), 2);
    }

    #[test]
    fn test_diagonal() {
        let grid = input_to_grid(
            r#"
                X...
                .M..
                ..A.
                ...S
            "#,
            '.',
        );
        assert_eq!(solve_ex1(&grid), 1);
    }

    #[test]
    fn test_overlapping() {
        let grid = input_to_grid(
            r#"
                XX..S
                .M..A
                .AA.M
                .S.SX
            "#,
            '.',
        );
        assert_eq!(solve_ex1(&grid), 3);
    }

    #[test]
    fn test_example() {
        let grid = input_to_grid(INPUT, '.');
        assert_eq!(solve_ex1(&grid), 18);
    }
}
