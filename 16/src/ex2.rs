use aoc_2024_lib::{board::Board, point2::Point2};
use pathfinding::matrix::directions::{self, DIRECTIONS_4};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::ex1::{self, Cell};

pub fn parse_maze(maze: &str) -> Board<Cell> {
    Board(
        maze.lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_string()
                            .parse()
                            .unwrap_or_else(|_| panic!("Invalid token in maze {} `{}`", line, c))
                    })
                    .collect()
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point2,
    direction: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_all_shortest_paths(maze: &Board<Cell>) -> Result<Vec<Vec<Point2>>, String> {
    let start = maze.find(&Cell::Start);
    let end = maze.find(&Cell::End);

    let mut dist: HashMap<(Point2, (isize, isize)), usize> = HashMap::new();
    let mut predecessors: HashMap<(Point2, (isize, isize)), Vec<(Point2, (isize, isize))>> =
        HashMap::new();
    let mut heap = BinaryHeap::new();

    // Initialize with start position facing EAST
    dist.insert((start, directions::E), 0);
    heap.push(State {
        cost: 0,
        position: start,
        direction: directions::E,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            continue;
        }

        if cost > dist[&(position, direction)] {
            continue;
        }

        for &dir in DIRECTIONS_4.iter() {
            let neighbor = position + dir;
            if maze[neighbor] == Cell::Wall {
                continue;
            }

            let new_cost = cost + ex1::STEP_COST;
            let turn_cost = if position == start {
                if dir != direction {
                    ex1::TURN_COST
                } else {
                    0
                }
            } else if dir != direction {
                ex1::TURN_COST
            } else {
                0
            };

            let total_cost = new_cost + turn_cost;
            if total_cost < *dist.get(&(neighbor, dir)).unwrap_or(&usize::MAX) {
                dist.insert((neighbor, dir), total_cost);
                predecessors.insert((neighbor, dir), vec![(position, direction)]);
                heap.push(State {
                    cost: total_cost,
                    position: neighbor,
                    direction: dir,
                });
            } else if total_cost == dist[&(neighbor, dir)] {
                predecessors
                    .entry((neighbor, dir))
                    .or_default()
                    .push((position, direction));
            }
        }
    }

    // Path reconstruction
    let mut all_paths = Vec::new();
    let mut costs_and_dirs: Vec<_> = DIRECTIONS_4
        .iter()
        .filter_map(|&dir| dist.get(&(end, dir)).map(|&cost| (cost, dir)))
        .collect();
    costs_and_dirs.sort_by_key(|&(cost, _)| cost);

    let mut min_cost_paths = vec![];

    println!("\nTop 5 paths to end:");
    for (i, &(cost, dir)) in costs_and_dirs.iter().take(5).enumerate() {
        println!("Path {} cost: {}", i + 1, cost);
        let mut stack = vec![((end, dir), vec![end])];
        let mut paths_for_this_cost = Vec::new();

        while let Some(((current, dir), path)) = stack.pop() {
            if current == start {
                paths_for_this_cost.push(path);
                continue;
            }

            if let Some(preds) = predecessors.get(&(current, dir)) {
                for &(pred, pred_dir) in preds {
                    let mut new_path = path.clone();
                    new_path.push(pred);
                    stack.push(((pred, pred_dir), new_path));
                }
            }
        }

        println!(
            "Found {} paths with cost {}",
            paths_for_this_cost.len(),
            cost
        );

        for path in &paths_for_this_cost {
            let mut reversed = path.clone();
            reversed.reverse();
            all_paths.push(reversed);
        }

        if i == 0 {
            min_cost_paths = paths_for_this_cost;
        }
    }

    Ok(min_cost_paths)
}

pub fn sum_all_points_on_shortest_paths(input: &str) -> usize {
    let maze = parse_maze(input);
    let paths = find_all_shortest_paths(&maze).unwrap();
    let all_points = paths.into_iter().flatten().collect::<HashSet<_>>();
    all_points.len()
}

#[allow(dead_code)]
fn grab_points(maze: Board<Cell>, paths: &[Vec<Point2>]) -> HashSet<&Point2> {
    // Debug print each path separately
    for (i, path) in paths.iter().enumerate() {
        println!("Path {}:", i + 1);
        let points: HashSet<_> = path.iter().collect();

        // Calculate and print the cost of this path
        let mut total_cost = 0;
        let mut prev_dir = None;
        for window in path.windows(2) {
            let from = window[0];
            let to = window[1];
            let dir = (
                to.row as isize - from.row as isize,
                to.col as isize - from.col as isize,
            );

            total_cost += ex1::STEP_COST;
            if let Some(prev) = prev_dir {
                if prev != dir {
                    total_cost += ex1::TURN_COST;
                }
            }
            prev_dir = Some(dir);
        }
        println!("Total cost: {}", total_cost);

        // Print the path
        print_path(&maze, &points);
    }

    // Collect all points from all paths
    let all_points = paths
        .iter()
        .flat_map(|path| path.iter())
        .collect::<HashSet<_>>();

    println!("\nCombined paths:");
    print_path(&maze, &all_points);

    all_points
}

#[allow(dead_code)]
fn print_path(maze: &Board<Cell>, points: &HashSet<&Point2>) {
    let path_string = maze
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, cell)| {
                    if points.contains(&Point2::new(i, j)) {
                        'O'
                    } else {
                        cell.to_string().chars().next().unwrap()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}\n", path_string);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2024_lib::input_reader::read_input;
    use pretty_assertions::assert_eq;

    fn get_example_input(str: &str) -> String {
        read_input("./inputs.md")
            .unwrap()
            .get_input(str)
            .content
            .clone()
    }

    #[test]
    fn test_small() {
        let input = "\
########
#.....E#
###.#.##
#...#.##
#.#.#.##
#.....##
#.###.##
#S..####
########";

        let maze = parse_maze(input);
        let paths = find_all_shortest_paths(&maze).expect("Finding all shortest paths failed");
        let all_points = grab_points(maze, &paths);
        assert_eq!(paths.len(), 3);
        assert_eq!(all_points.len(), 20);
    }

    #[test]
    fn test_find_all_shortest_paths() {
        let example = get_example_input("Example 1");

        let maze = parse_maze(&example);
        let paths = find_all_shortest_paths(&maze).expect("Finding all shortest paths failed");

        assert_eq!(paths.len(), 3);
        let path = paths.first().unwrap();
        assert_eq!(path.len(), 37);
    }

    #[test]
    fn test_solve_example() {
        let example = get_example_input("Example 1");
        assert_eq!(sum_all_points_on_shortest_paths(&example), 45);
    }
}
