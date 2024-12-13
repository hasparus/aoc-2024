use super::grid::Grid;

fn window_coords((x, y): (isize, isize)) -> [(isize, isize); 4] {
    [(x, y), (x, y - 1), (x - 1, y), (x - 1, y - 1)]
}

fn get_window<T: Copy>(grid: &Grid<T>, pos: (isize, isize)) -> [T; 4] {
    window_coords(pos).map(|(x, y)| *grid.get(x, y))
}

fn corner_in_window(window: &[isize; 4], region_symbol: isize) -> u8 {
    let count = window.iter().filter(|c| **c == region_symbol).count();

    // we'rein a corner if there is just one occurence of the region symbol
    if count == 1 {
        return 1;
    }

    // // or we have a common corner of two regions, so the symbol is present on a diagonal
    if count == 2
        && ((window[0] == region_symbol && window[3] == region_symbol)
            || (window[1] == region_symbol && window[2] == region_symbol))
    {
        return 2;
    }

    // if we have 3 occurences, we're in an inner corner
    if count == 3 {
        return 1;
    }

    0
}

#[derive(Debug, PartialEq, Eq)]
struct Region {
    symbol: char,
    area: usize,
    corners: usize,
}

static EMPTY: char = ' ';
static PLACEHOLDER: char = '.';

type InputGrid = Grid<char>;
type RegionIndex = isize;
type RegionIndicesGrid = Grid<RegionIndex>;

fn flood_fill(grid: &InputGrid) -> (Vec<Region>, RegionIndicesGrid) {
    let mut regions = Vec::new();
    let mut visited = vec![vec![-1; grid.width()]; grid.height()];

    let mut region_index = 0;
    for r in 0..grid.height() {
        for c in 0..grid.width() {
            let symbol = *grid.get(r as isize, c as isize);
            if visited[r][c] == -1 && symbol != EMPTY && symbol != PLACEHOLDER {
                let area = explore_region(grid, &mut visited, region_index, r, c, symbol);
                regions.push(Region {
                    symbol,
                    area,
                    corners: 0,
                });
                region_index += 1;
            }
        }
    }

    (regions, RegionIndicesGrid::new(visited, -1))
}

fn explore_region(
    grid: &InputGrid,
    visited: &mut [Vec<isize>],
    region_id: isize,
    row: usize,
    col: usize,
    symbol: char,
) -> usize {
    if visited[row][col] != -1 || *grid.get(row as isize, col as isize) != symbol {
        return 0;
    }

    visited[row][col] = region_id;
    let mut size = 1;

    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if grid.in_bounds(new_row, new_col) {
            size += explore_region(
                grid,
                visited,
                region_id,
                new_row as usize,
                new_col as usize,
                symbol,
            );
        }
    }

    size
}

pub fn solve(input: &str) -> usize {
    let grid = Grid::new(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().chars().collect())
            .collect(),
        EMPTY,
    );

    let (mut regions, region_indices) = flood_fill(&grid);

    for r in 0..=grid.height() {
        for c in 0..=grid.width() {
            let pos = (r as isize, c as isize);
            let regions_window = get_window(&region_indices, pos);

            let unique_regions = std::collections::HashSet::<isize>::from_iter(
                regions_window.iter().filter(|r| **r != -1).copied(),
            );

            for region_index in unique_regions.iter() {
                let region = &mut regions[*region_index as usize];
                if region.symbol == EMPTY || region.symbol == PLACEHOLDER {
                    continue;
                }

                region.corners += corner_in_window(&regions_window, *region_index) as usize;
            }
        }
    }

    println!("{:#?}", regions);
    regions.iter().map(|r| r.area * r.corners).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_reader;

    #[test]
    fn test_single_line() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = input_reader::read_input("inputs.md")?;

        let single_line = inputs.get_input("Single line");

        assert_eq!(solve(&single_line.content), 16);
        Ok(())
    }

    #[test]
    fn test_snake() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            solve(
                "C.
                 CC
                 .C"
            ),
            32
        );
        Ok(())
    }

    #[test]
    fn test_trivial() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = input_reader::read_input("inputs.md")?;

        let trivial = inputs.get_input("Trivial");

        assert_eq!(solve(&trivial.content), 80);
        Ok(())
    }

    #[test]
    fn test_simple_example() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = input_reader::read_input("inputs.md")?;

        let simple = inputs.get_input("Simple");

        assert_eq!(solve(&simple.content), 1206);
        Ok(())
    }

    #[test]
    fn trivial_shared_corner() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(solve("C.\n.C"), 8);
        Ok(())
    }

    #[test]
    fn test_inner_sides_example() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = input_reader::read_input("inputs.md")?;

        let inner_sides = inputs.get_input("Inner Sides");

        assert_eq!(solve(&inner_sides.content), 368);
        Ok(())
    }

    #[test]
    fn test_checkerboard() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            solve(
                "OOOOO
                 OXOXO
                 OOOOO
                 OXOXO
                 OOOOO"
            ),
            436
        );
        Ok(())
    }
}
