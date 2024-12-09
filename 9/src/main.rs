fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

fn stringify_id_expansion(disk: &[DiskItem]) -> String {
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
    let mut res = Vec::with_capacity(disk.0.len() / 2);

    let disk = disk.0;

    let mut left = 0;
    let mut right = disk.len() - 1;

    let mut free_space_remaining = usize::MAX;
    let mut post_split_file_size = usize::MAX;

    while left < right {
        match (&disk[left], &disk[right]) {
            // If left is a file, just move to next position
            (DiskItem::File(_), _) => {
                free_space_remaining = usize::MAX;

                res.push(disk[left]);
                println!("pushing file {:?}", disk[left]);
                println!("{}", stringify_id_expansion(&res));
                left += 1;
            }
            // If right is free space, move left
            (_, DiskItem::FreeSpace(_)) => {
                println!("moving right to left");
                println!("{}", stringify_id_expansion(&res));
                right -= 1;
            }
            // If left is free space and right is file
            (DiskItem::FreeSpace(free_space), DiskItem::File(file)) => {
                println!("reconciling");
                let free_space = std::cmp::min(free_space_remaining, *free_space);
                let file_size = std::cmp::min(post_split_file_size, file.size);

                println!("file.id: {}", file.id);
                println!("left: {}, right: {}", left, right);
                println!("free_space: {}, file_size: {}", free_space, file_size);

                if free_space >= file_size {
                    println!("more empty space than file size");
                    post_split_file_size = usize::MAX;
                    free_space_remaining = free_space - file_size;

                    res.push(DiskItem::File(File {
                        id: file.id,
                        size: file_size,
                    }));

                    right -= 1;
                } else {
                    println!("less empty space than file size");
                    post_split_file_size = file_size - free_space;
                    free_space_remaining = 0;

                    res.push(DiskItem::File(File {
                        id: file.id,
                        size: free_space,
                    }));

                    left += 1;
                }

                println!("{}", stringify_id_expansion(&res));
            }
        }
    }

    if left == right {
        if let DiskItem::File(file) = disk[left] {
            res.push(DiskItem::File(File {
                id: file.id,
                size: std::cmp::min(post_split_file_size, file.size),
            }));
        }
    }

    res
}

fn calculate_checksum(compressed: &str) -> usize {
    compressed
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '0'..='9' => (c.to_digit(10).unwrap() as usize) * i,
            _ => 0,
        })
        .sum()
}

fn solve(input: &str) -> usize {
    let disk = parse_input(input);
    let compressed = compress(disk);
    calculate_checksum(&stringify_id_expansion(&compressed))
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
            stringify_id_expansion(&parse_input("12345").0),
            "0..111....22222"
        );

        assert_eq!(
            stringify_id_expansion(&parse_input(EXAMPLE).0),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn stringify_id_expansion_example() {
        let parsed = parse_input(EXAMPLE);
        let compressed = compress(parsed);
        assert_eq!(
            stringify_id_expansion(&compressed),
            "0099811188827773336446555566"
        );
    }

    #[test]
    fn test_example() {
        let x = parse_input(EXAMPLE);
        println!("{}", stringify_id_expansion(&x.0));
        let y = compress(x);
        println!("{:?}", y);
        println!("{}", stringify_id_expansion(&y));

        assert_eq!(solve(EXAMPLE), 1928);
    }
}
