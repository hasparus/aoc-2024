type Stone = u64;

fn parse_input(input: &str) -> Vec<Stone> {
    input
        .trim()
        .split(' ')
        .map(|word| word.parse().expect("Invalid input"))
        .collect()
}

fn count_digits(stone: u64) -> u32 {
    if stone == 0 {
        return 1;
    }
    (stone as f64).log10() as u32 + 1
}

fn solve_ex1(stones: &Vec<Stone>, iterations: u32) -> usize {
    let mut stones = stones.clone();

    for _ in 0..iterations {
        let mut new_stones = Vec::new();

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            let digits = count_digits(stone);
            if digits % 2 == 0 {
                let mid = 10_u64.pow(digits / 2);
                let left = stone / mid;
                let right = stone % mid;
                new_stones.extend([left, right]);
                continue;
            }

            new_stones.push(stone * 2024);
        }

        stones = new_stones;
    }

    stones.len()
}

mod memoized {
    use std::collections::VecDeque;

    use super::*;
    use memoize::memoize;

    #[memoize]
    fn transform(stone: Stone) -> [Option<Stone>; 2] {
        if stone == 0 {
            return [Some(1), None];
        }

        let digits = count_digits(stone);
        if digits % 2 == 0 {
            let mid = 10_u64.pow(digits / 2);
            return [Some(stone / mid), Some(stone % mid)];
        }

        [Some(stone * 2024), None]
    }

    pub fn solve(stones: &[Stone], iterations: u32) -> usize {
        let mut result = 0;

        let mut stack = stones
            .iter()
            .map(|stone| (*stone, iterations))
            .collect::<VecDeque<_>>();

        let mut i = 0;
        while let Some((stone, time_to_live)) = stack.pop_back() {
            if i % 1000000 == 0 {
                println!("stack {} res {} ttl {}", stack.len(), result, time_to_live);
            }
            i += 1;

            if time_to_live == 0 {
                result += 1;
                continue;
            }

            let [left, right] = transform(stone);
            if let Some(left) = left {
                stack.push_back((left, time_to_live - 1));
            }
            if let Some(right) = right {
                stack.push_back((right, time_to_live - 1));
            }
        }

        result
    }
}

mod ex2_good {
    use super::*;

    fn solve(stones: &[Stone], iterations: u32) -> usize {
        0
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let stones = parse_input(&input);
    let result = solve_ex1(&stones, 25);
    println!("{:?}", result);

    let result = memoized::solve(&stones, 75);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let numbers = [
            125, 17, 1000, 9999, 99999, 999999, 9999999, 99999999, 999999999,
        ];
        for number in numbers {
            assert_eq!(count_digits(number), number.to_string().len() as u32);
        }
    }

    static EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_solve_example() {
        let stones = parse_input(EXAMPLE);
        let result = solve_ex1(&stones, 25);
        println!("{:?}", result);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_solve_example_ex2() {
        let stones = parse_input(EXAMPLE);
        let result = memoized::solve(&stones, 25);
        println!("{:?}", result);
        assert_eq!(result, 55312);
    }
}
