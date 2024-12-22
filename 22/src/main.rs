use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn mix_and_prune(secret: i64) -> i64 {
    secret % 16777216
}

fn generate_next_secret(secret: i64) -> i64 {
    let mut next = secret;

    next = mix_and_prune(next ^ (next * 64));
    next = mix_and_prune(next ^ (next / 32));
    next = mix_and_prune(next ^ (next * 2048));

    next
}

fn generate_nth_secret(initial: i64, n: usize) -> i64 {
    let mut secret = initial;
    for _ in 0..n {
        secret = generate_next_secret(secret);
    }
    secret
}

fn solve1(initial_secrets: &[i64]) -> i64 {
    initial_secrets
        .iter()
        .map(|&secret| generate_nth_secret(secret, 2000))
        .sum()
}

fn generate_price_sequence(initial: i64, count: usize) -> Vec<i8> {
    let mut prices = Vec::with_capacity(count + 1);
    let mut secret = initial;
    prices.push((secret % 10) as i8);

    for _ in 0..count {
        secret = generate_next_secret(secret);
        prices.push((secret % 10) as i8);
    }
    prices
}

fn calculate_changes(prices: &[i8]) -> Vec<i8> {
    prices.windows(2).map(|w| w[1] - w[0]).collect()
}

fn find_sequence_value(changes: &[i8], sequence: &[i8], prices: &[i8]) -> Option<i8> {
    changes
        .windows(sequence.len())
        .enumerate()
        .find(|(_, window)| *window == sequence)
        .map(|(i, _)| prices[i + sequence.len()])
}

fn evaluate_sequence(initial_secrets: &[i64], sequence: &[i8]) -> i64 {
    initial_secrets
        .iter()
        .filter_map(|&secret| {
            let prices = generate_price_sequence(secret, 2000);
            let changes = calculate_changes(&prices);
            find_sequence_value(&changes, sequence, &prices).map(|price| price as i64)
        })
        .sum()
}

fn solve2(initial_secrets: &[i64]) -> i64 {
    let mut best_sum = 0;
    let mut best_sequence = [0; 4];

    // This brute is fast enough to run when watching a TV series,
    // so I'm not going to optimize it.
    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let sequence = [a, b, c, d];
                    println!("{:?}", sequence);
                    let sum = evaluate_sequence(initial_secrets, &sequence);
                    if sum > best_sum {
                        best_sum = sum;
                        best_sequence = sequence;
                    }
                    println!("sum: {}", sum);
                    println!("best_sum: {}", best_sum);
                }
            }
        }
    }

    println!("Best sequence: {:?}", best_sequence);
    best_sum
}

fn read_numbers<P: AsRef<Path>>(path: P) -> io::Result<Vec<i64>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line?
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn main() -> io::Result<()> {
    let initial_secrets = read_numbers("./input.txt")?;
    let result1 = solve1(&initial_secrets);
    println!("Part 1 - Sum of 2000th secret numbers: {}", result1);

    let result2 = solve2(&initial_secrets);
    println!("Part 2 - Maximum bananas: {}", result2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let test_cases = [1, 10, 100, 2024];
        let expected = [8685429, 4700978, 15273692, 8667524];

        for (input, expected) in test_cases.iter().zip(expected.iter()) {
            assert_eq!(generate_nth_secret(*input, 2000), *expected);
        }

        assert_eq!(solve1(&test_cases), 37327623);
    }

    #[test]
    fn test_part2_example() {
        let test_cases = [1, 2, 3, 2024];
        let sequence = [-2, 1, -1, 3];
        assert_eq!(evaluate_sequence(&test_cases, &sequence), 23);
    }

    #[test]
    fn test_price_sequence() {
        let prices = generate_price_sequence(123, 9);
        let expected = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];
        assert_eq!(prices, expected);

        let changes = calculate_changes(&prices);
        let expected_changes = [-3, 6, -1, -1, 0, 2, -2, 0, -2];
        assert_eq!(changes, expected_changes);
    }
}
