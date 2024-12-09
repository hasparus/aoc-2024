mod grid;
mod point;
use colored::*;

use crate::grid::Grid;
use crate::point::Point;
use std::collections::{HashMap, HashSet};

fn find_antennas(grid: &Grid<char>) -> HashMap<char, Vec<Point>> {
    let mut antennas = HashMap::new();

    for (row, col, &ch) in grid.cells() {
        if ch != '.' && ch != ' ' {
            antennas
                .entry(ch)
                .or_insert_with(Vec::new)
                .push(Point::new(row, col));
        }
    }

    antennas
}

mod ex1 {
    use super::*;

    fn antinodes_for_antennas(p1: Point, p2: Point) -> Vec<Point> {
        let first = p2 + (p2 - p1);
        let second = p1 + (p1 - p2);
        vec![first, second]
    }

    pub fn find_antinodes_for_grid(grid: &Grid<char>) -> HashSet<Point> {
        let antennas = find_antennas(grid);
        let mut antinodes = HashSet::new();

        for (_freq, positions) in antennas.iter() {
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let p1 = positions[i];
                    let p2 = positions[j];

                    // Calculate potential antinodes
                    let new_antinodes = antinodes_for_antennas(p1, p2);

                    // Add antinodes that are within the grid bounds
                    for antinode in new_antinodes {
                        if antinode.row >= 0
                            && antinode.col >= 0
                            && antinode.row < grid.height() as isize
                            && antinode.col < grid.width() as isize
                        {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }

        antinodes
    }
}

mod ex2 {
    use super::*;

    fn gcd(mut a: isize, mut b: isize) -> isize {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn points_on_line(p1: Point, p2: Point, grid: &Grid<char>) -> Vec<Point> {
        let dx = p2.col - p1.col;
        let dy = p2.row - p1.row;

        if dx == 0 && dy == 0 {
            return vec![p1];
        }

        let g = gcd(dx, dy);
        let step_x = dx / g;
        let step_y = dy / g;

        let mut points = Vec::new();

        // Start from the leftmost/topmost point and extend in both directions
        let mut current = Point::new(p1.row, p1.col);

        // Go backwards until we hit grid boundary
        while current.row >= 0
            && current.col >= 0
            && current.row < grid.height() as isize
            && current.col < grid.width() as isize
        {
            points.push(current);
            current = Point::new(current.row - step_y, current.col - step_x);
        }

        // Go forwards until we hit grid boundary
        let mut current = Point::new(p1.row + step_y, p1.col + step_x);
        while current.row >= 0
            && current.col >= 0
            && current.row < grid.height() as isize
            && current.col < grid.width() as isize
        {
            points.push(current);
            current = Point::new(current.row + step_y, current.col + step_x);
        }

        points
    }

    fn find_antennas(grid: &Grid<char>) -> HashMap<char, Vec<Point>> {
        let mut antennas = HashMap::new();

        for (row, col, &ch) in grid.cells() {
            if ch != '.' && ch != ' ' {
                antennas
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push(Point::new(row, col));
            }
        }

        antennas
    }

    pub fn find_antinodes_for_grid(grid: &Grid<char>) -> HashSet<Point> {
        let antennas = find_antennas(grid);
        let mut antinodes = HashSet::new();

        // For each frequency
        for (_freq, positions) in antennas.iter() {
            // For each pair of antennas with the same frequency
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let p1 = positions[i];
                    let p2 = positions[j];

                    // Find all points on the line between p1 and p2
                    let line_points = points_on_line(p1, p2, grid);

                    // Add points that are within grid bounds
                    for point in line_points {
                        if point.row >= 0
                            && point.col >= 0
                            && point.row < grid.height() as isize
                            && point.col < grid.width() as isize
                        {
                            antinodes.insert(point);
                        }
                    }
                }
            }
        }

        antinodes
    }
}

fn display_with_antinodes(grid: &Grid<char>, antinodes: &HashSet<Point>) -> String {
    let mut result = String::new();

    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let pos = Point::new(row as isize, col as isize);
            let current = *grid.get(row as isize, col as isize);

            if antinodes.contains(&pos) {
                if current != '.' && current != ' ' {
                    // Antinode overlaps with antenna - blue background
                    result.push_str(&current.to_string().bright_white().on_green().to_string());
                } else {
                    result.push_str(
                        &"#".on_custom_color(CustomColor {
                            r: 33,
                            g: 100,
                            b: 33,
                        })
                        .to_string(),
                    );
                }
            } else {
                result.push(current);
            }
        }
        result.push('\n');
    }

    result
}

fn parse_input(input: &str) -> Grid<char> {
    Grid::new(
        input.lines().map(|line| line.chars().collect()).collect(),
        ' ',
    )
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse_input(&input);
    let antinodes = ex1::find_antinodes_for_grid(&grid);
    println!("Grid with antinodes:");
    println!("{}", display_with_antinodes(&grid, &antinodes));
    println!("Number of antinodes: {}", antinodes.len());

    println!("\n--- Part Two ---\n");

    let antinodes = ex2::find_antinodes_for_grid(&grid);
    println!("Grid with antinodes:");
    println!("{}", display_with_antinodes(&grid, &antinodes));
    println!("Number of antinodes: {}", antinodes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_example_ex1() {
        let grid = parse_input(EXAMPLE);
        let antinodes = ex1::find_antinodes_for_grid(&grid);

        println!("{}", display_with_antinodes(&grid, &antinodes));
        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_example_ex2() {
        let grid = parse_input(EXAMPLE);
        let antinodes = ex2::find_antinodes_for_grid(&grid);
        println!("{}", display_with_antinodes(&grid, &antinodes));
        assert_eq!(antinodes.len(), 34);
    }
}
