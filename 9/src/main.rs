fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, PartialEq)]
enum DiskItem {
    File(File),
    FreeSpace(usize),
}

#[derive(Debug, PartialEq)]
struct Disk(Vec<DiskItem>);

impl IntoIterator for Disk {
    type Item = DiskItem;
    type IntoIter = std::vec::IntoIter<DiskItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<DiskItem> for Disk {
    fn from_iter<I: IntoIterator<Item = DiskItem>>(iter: I) -> Self {
        Disk(iter.into_iter().collect())
    }
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}

type Compressed = Vec<DiskItem>;

fn stringify_id_expansion(disk: Vec<DiskItem>) -> String {
    disk.iter()
        .map(|item| match item {
            DiskItem::File(file) => file.id.to_string().repeat(file.size),
            DiskItem::FreeSpace(space) => ".".repeat(*space),
        })
        .collect()
}

fn parse_input(input: &str) -> Disk {
    // File interweaved with free space.
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                DiskItem::File(File { id: i / 2, size })
            } else {
                DiskItem::FreeSpace(size)
            }
        })
        .collect()
}

fn compress(disk: Disk) -> Compressed {
    let mut result = Vec::with_capacity(disk.0.len());
    // First, copy all items to result
    for item in disk {
        result.push(item);
    }

    let mut left = 0;
    let mut right = result.len() - 1;

    while left < right {
        match (&result[left], &result[right]) {
            // If left is a file, just move to next position
            (DiskItem::File(_), _) => {
                left += 1;
            }
            // If right is free space, move left
            (_, DiskItem::FreeSpace(_)) => {
                right -= 1;
            }
            // If left is free space and right is file
            (DiskItem::FreeSpace(free_space), DiskItem::File(file)) => {
                let available_space = *free_space;
                let file_size = file.size;
                let file_id = file.id;

                // If we can fit the entire file
                if available_space >= file_size {
                    result[left] = DiskItem::File(File {
                        id: file_id,
                        size: file_size,
                    });
                    result[right] = DiskItem::FreeSpace(file_size);
                    left += 1;
                    right -= 1;
                } else {
                    // If we can only fit part of the file, we'll need to split it
                    result[left] = DiskItem::File(File {
                        id: file_id,
                        size: available_space,
                    });
                    result[right] = DiskItem::File(File {
                        id: file_id,
                        size: file_size - available_space,
                    });
                    left += 1;
                }
            }
        }
    }

    result
}

fn calculate_checksum(compressed: &Compressed) -> usize {
    compressed
        .iter()
        .enumerate()
        .map(|(i, item)| match item {
            DiskItem::File(file) => file.id * i,
            DiskItem::FreeSpace(_) => 0,
        })
        .sum()
}

fn solve(input: &str) -> usize {
    let disk = parse_input(input);
    let compressed = compress(disk);
    calculate_checksum(&compressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn parse_trivial() {
        assert_eq!(
            parse_input("12345"),
            Disk(vec![
                DiskItem::File(File { id: 0, size: 1 }),
                DiskItem::FreeSpace(2),
                DiskItem::File(File { id: 1, size: 3 }),
                DiskItem::FreeSpace(4),
                DiskItem::File(File { id: 2, size: 5 }),
            ])
        );
    }

    #[test]
    fn compress_trivial() {
        assert_eq!(
            compress(parse_input("12345")),
            vec![
                DiskItem::File(File { id: 0, size: 1 }),
                DiskItem::File(File { id: 2, size: 2 }),
                DiskItem::File(File { id: 1, size: 3 }),
                DiskItem::File(File { id: 2, size: 3 }),
            ]
        );
    }

    #[test]
    fn stringify_id_expansion_cases() {
        assert_eq!(
            stringify_id_expansion(parse_input("12345").0),
            "0..111....22222"
        );

        assert_eq!(
            stringify_id_expansion(parse_input(EXAMPLE).0),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    // #[test]
    // fn test_example() {
    //     assert_eq!(solve(EXAMPLE), 1928);
    // }
}
