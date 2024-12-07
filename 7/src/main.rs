use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_str, numbers_str) = s.split_once(':').ok_or("missing colon")?;

        Ok(Equation {
            target: target_str
                .trim()
                .parse()
                .map_err(|e| format!("invalid target: {}", e))?,
            numbers: numbers_str
                .split_whitespace()
                .map(|n| n.parse().map_err(|e| format!("invalid number: {}", e)))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Equation {
    fn is_solvable(&self) -> bool {
        let n = self.numbers.len() - 1;

        for i in 0..(1 << n) {
            let operators: Vec<Op> = (0..n)
                .map(|j| {
                    if (i & (1 << j)) == 0 {
                        Op::Add
                    } else {
                        Op::Mul
                    }
                })
                .collect();

            if self.evaluate(&operators) == self.target {
                return true;
            }
        }
        false
    }

    fn evaluate(&self, operators: &[Op]) -> u64 {
        let mut result = self.numbers[0];
        for (i, &op) in operators.iter().enumerate() {
            match op {
                Op::Add => result += self.numbers[i + 1],
                Op::Mul => result *= self.numbers[i + 1],
            }
        }
        result
    }
}

fn solve(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| line.parse::<Equation>().ok())
        .filter(|eq| eq.is_solvable())
        .map(|eq| eq.target)
        .sum()
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let start = std::time::Instant::now();
    let result = solve(&input);
    let duration = start.elapsed();
    println!("Result 1: {result}, {duration:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();
        assert_eq!(solve(&input), 3749);
    }

    #[test]
    fn test_simple_cases() {
        // Only multiplication works
        assert_eq!(solve("190: 10 19\n"), 190);
        // Only addition works
        assert_eq!(solve("20: 10 10\n"), 20);
        // No solution possible
        assert_eq!(solve("30: 10 10\n"), 0);
        // (10 + 20) * 2 = 60 works
        assert_eq!(solve("60: 10 20 2\n"), 60);
    }

    #[test]
    fn test_parse_equation() {
        let eq: Equation = "190: 10 19".parse().unwrap();
        assert_eq!(eq.target, 190);
        assert_eq!(eq.numbers, vec![10, 19]);
    }

    #[test]
    fn test_equation_evaluation() {
        let eq: Equation = "190: 10 19".parse().unwrap();
        assert_eq!(eq.evaluate(&[Op::Mul]), 190);
        assert_eq!(eq.evaluate(&[Op::Add]), 29);
    }
}
