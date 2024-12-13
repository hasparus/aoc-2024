#[derive(Debug, PartialEq, Eq)]
struct Region {
    symbol: char,
    area: usize,
    perimeter: usize,
}

fn flood_fill(input: &str) -> Vec<Region> {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
    if lines.is_empty() {
        return vec![];
    }

    let rows = lines.len();
    let cols = lines[0].len();
    let mut regions = Vec::new();
    let mut visited = vec![vec![false; cols]; rows];

    for (i, line) in lines.iter().enumerate() {
        for (j, &symbol) in line.as_bytes().iter().enumerate() {
            if !visited[i][j] {
                let symbol = symbol as char;
                let (area, perimeter) = explore_region(&lines, &mut visited, i, j, symbol);
                regions.push(Region {
                    symbol,
                    area,
                    perimeter,
                });
            }
        }
    }

    regions
}

fn explore_region(
    lines: &[&str],
    visited: &mut [Vec<bool>],
    row: usize,
    col: usize,
    symbol: char,
) -> (usize, usize) {
    if row >= lines.len()
        || col >= lines[0].len()
        || visited[row][col]
        || lines[row].as_bytes()[col] as char != symbol
    {
        return (0, 0);
    }

    visited[row][col] = true;
    let mut size = 1;
    let mut perimeter = 0;

    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row < 0
            || new_row >= lines.len() as i32
            || new_col < 0
            || new_col >= lines[0].len() as i32
        {
            perimeter += 1;
            continue;
        }

        let (nr, nc) = (new_row as usize, new_col as usize);
        if lines[nr].as_bytes()[nc] as char != symbol {
            perimeter += 1;
        } else {
            let (sub_size, sub_perimeter) = explore_region(lines, visited, nr, nc, symbol);
            size += sub_size;
            perimeter += sub_perimeter;
        }
    }

    (size, perimeter)
}

pub fn solve(input: &str) -> usize {
    let regions = flood_fill(input);
    regions.iter().map(|r| r.area * r.perimeter).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_regions() {
        let input = "AAA\nBBC\nBBC";
        let regions = flood_fill(input);

        let expected = vec![
            Region {
                symbol: 'A',
                area: 3,
                perimeter: 8,
            },
            Region {
                symbol: 'B',
                area: 4,
                perimeter: 8,
            },
            Region {
                symbol: 'C',
                area: 2,
                perimeter: 6,
            },
        ];

        assert_eq!(regions, expected);
    }
}
