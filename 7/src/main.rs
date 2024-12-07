use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply_reverse(&self, target: u64, operand: u64) -> Option<u64> {
        match self {
            Operator::Add => {
                if target >= operand {
                    Some(target - operand)
                } else {
                    None
                }
            }
            Operator::Multiply => {
                if operand != 0 && target % operand == 0 {
                    Some(target / operand)
                } else {
                    None
                }
            }
            Operator::Concatenate => {
                let operand_len = operand.to_string().len();
                let divisor = 10_u64.checked_pow(operand_len as u32)?;
                if target >= operand && target % divisor == operand {
                    Some(target / divisor)
                } else {
                    None
                }
            }
        }
    }

    fn can_be_used_with(&self, target: u64, operand: u64) -> bool {
        match self {
            Operator::Add => true,
            Operator::Multiply => operand != 0 && target % operand == 0,
            Operator::Concatenate => operand != 0,
        }
    }
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
    fn is_solvable(&self, operators: &[Operator]) -> bool {
        self.try_solve(self.numbers.len() - 1, self.target, operators)
    }

    fn try_solve(&self, pos: usize, target: u64, operators: &[Operator]) -> bool {
        if pos == 0 {
            return self.numbers[0] == target;
        }

        let next = self.numbers[pos];
        operators
            .iter()
            .filter(|op| op.can_be_used_with(target, next))
            .any(|op| {
                op.apply_reverse(target, next).map_or(false, |prev_target| {
                    self.try_solve(pos - 1, prev_target, operators)
                })
            })
    }
}

fn sum_up_solvable_equations(input: &str, operators: &[Operator]) -> u64 {
    input
        .lines()
        .filter_map(|line| line.parse::<Equation>().ok())
        .filter(|eq| eq.is_solvable(operators))
        .map(|eq| eq.target)
        .sum()
}

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let start = std::time::Instant::now();
    let result = sum_up_solvable_equations(&input, &[Operator::Add, Operator::Multiply]);
    let duration = start.elapsed();
    println!("Result 1: {result}, {duration:?}");

    let start = std::time::Instant::now();
    let result = sum_up_solvable_equations(
        &input,
        &[Operator::Add, Operator::Multiply, Operator::Concatenate],
    );
    let duration = start.elapsed();
    println!("Result 2: {result}, {duration:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();
        assert_eq!(
            sum_up_solvable_equations(&input, &[Operator::Add, Operator::Multiply]),
            3749
        );
    }

    #[test]
    fn test_simple_cases() {
        // Only multiplication works
        assert_eq!(
            sum_up_solvable_equations("190: 10 19\n", &[Operator::Multiply]),
            190
        );
        // Only addition works
        assert_eq!(
            sum_up_solvable_equations("20: 10 10\n", &[Operator::Add]),
            20
        );
        // No solution possible
        assert_eq!(
            sum_up_solvable_equations("30: 10 10\n", &[Operator::Add]),
            0
        );
        // (10 + 20) * 2 = 60 works
        assert_eq!(
            sum_up_solvable_equations("60: 10 20 2\n", &[Operator::Add, Operator::Multiply]),
            60
        );
    }

    #[test]
    fn test_parse_equation() {
        let eq: Equation = "190: 10 19".parse().unwrap();
        assert_eq!(eq.target, 190);
        assert_eq!(eq.numbers, vec![10, 19]);
    }

    #[test]
    fn test_operations() {
        let eq: Equation = "190: 10 19".parse().unwrap();
        assert!(eq.is_solvable(&[Operator::Add, Operator::Multiply]));

        let eq: Equation = "29: 10 19".parse().unwrap();
        assert!(eq.is_solvable(&[Operator::Add, Operator::Multiply]));

        let eq: Equation = "1019: 10 19".parse().unwrap();
        assert!(eq.is_solvable(&[Operator::Add, Operator::Multiply, Operator::Concatenate]));
    }

    #[test]
    fn test_concatenation_cases() {
        assert_eq!(
            sum_up_solvable_equations("156: 15 6\n", &[Operator::Concatenate]),
            156
        );
        assert_eq!(
            sum_up_solvable_equations(
                "7290: 6 8 6 15\n",
                &[Operator::Add, Operator::Multiply, Operator::Concatenate]
            ),
            7290
        );
        assert_eq!(
            sum_up_solvable_equations(
                "192: 17 8 14\n",
                &[Operator::Add, Operator::Multiply, Operator::Concatenate]
            ),
            192
        );
    }
}
