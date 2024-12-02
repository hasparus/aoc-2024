type Report = Vec<i32>;
type ParsedInput = Vec<Report>;

fn parse_input(input: &str) -> ParsedInput {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .take_while(|&token| token != "#")
                .filter_map(|token| token.parse::<i32>().ok())
                .collect::<Report>()
        })
        .collect::<ParsedInput>()
}

mod ex_1 {
    use super::*;
    pub fn count_safe_reports(input: &ParsedInput) -> u32 {
        input
            .iter()
            .filter(|report| {
                let is_increasing = report[1] > report[0];
                report.windows(2).all(|window| {
                    let delta = window[1] - window[0];
                    (1..=3).contains(&delta.abs())
                        && (is_increasing && delta > 0 || !is_increasing && delta < 0)
                })
            })
            .count() as u32
    }
}

mod ex_2_brute {
    use super::*;
    pub fn count_safe_reports(input: &ParsedInput) -> u32 {
        input
            .iter()
            .filter(|report| {
                // all reports derived by removing element at index i
                let mut derived_reports = report
                    .iter()
                    .enumerate()
                    .map(|(i, _)| [report[..i].to_vec(), report[i + 1..].to_vec()].concat());

                let is_safe = |report: &[i32]| {
                    let is_increasing = report[1] > report[0];
                    report.windows(2).all(|window| {
                        let delta = window[1] - window[0];
                        (1..=3).contains(&delta.abs())
                            && (is_increasing && delta > 0 || !is_increasing && delta < 0)
                    })
                };

                derived_reports.any(|derived_report| is_safe(&derived_report))
            })
            .count() as u32
    }
}

mod ex_2 {
    use crate::ParsedInput;

    fn is_safe(report: &[i32], problem_dampener_quota: i32) -> bool {
        if report.len() < 2 {
            return false;
        }

        fn is_delta_ok(is_increasing: bool, delta: i32) -> bool {
            (is_increasing && delta > 0 || !is_increasing && delta < 0)
                && (1..=3).contains(&delta.abs())
        }

        let is_increasing = report[1] > report[0];

        let mut all_valid = true;
        for window in report.windows(2) {
            let delta = window[1] - window[0];
            if !is_delta_ok(is_increasing, delta) {
                all_valid = false;
                break;
            }
        }
        if all_valid {
            return true;
        }

        if problem_dampener_quota > 0 {
            for i in 0..report.len() {
                if report.len() - 1 < 2 {
                    continue;
                }

                // Check if removing element at index i makes sequence valid
                let is_increasing = if i == 0 {
                    report[2] > report[1]
                } else if i == report.len() - 1 {
                    report[report.len() - 2] > report[report.len() - 3]
                } else {
                    // For middle elements, check surrounding elements
                    report[i + 1] > report[i - 1]
                };

                let mut valid = true;
                let mut prev = if i == 0 { report[1] } else { report[0] };

                for j in 1..report.len() {
                    if j == i {
                        continue;
                    }
                    let curr = report[j];
                    let delta = curr - prev;
                    if !is_delta_ok(is_increasing, delta) {
                        valid = false;
                        break;
                    }
                    prev = curr;
                }

                if valid {
                    return true;
                }
            }
        }

        false
    }

    pub fn count_safe_reports_with_problem_dampener(input: &ParsedInput) -> u32 {
        input.iter().filter(|report| is_safe(report, 1)).count() as u32
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed_input = parse_input(&input);
    println!("{:?}", ex_1::count_safe_reports(&parsed_input));
    println!(
        "{:?}",
        ex_2::count_safe_reports_with_problem_dampener(&parsed_input)
    );
    println!("{:?}", ex_2_brute::count_safe_reports(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE: &str = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;

    #[test]
    fn parses_inputs() {
        assert_eq!(
            parse_input(SIMPLE),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }

    #[test]
    fn ignores_comments() {
        assert_eq!(
            parse_input(r#"1 2 3 4 5 # 6 7 8 9"#),
            vec![vec![1, 2, 3, 4, 5]]
        );
    }

    #[test]
    fn ex_1_example() {
        assert_eq!(ex_1::count_safe_reports(&parse_input(SIMPLE)), 2);
    }

    #[test]
    fn ex_1_single_line_increasing() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        1 3 6 7 9
                    "#
            )),
            1
        );
    }

    #[test]
    fn ex_1_single_line_decreasing() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        7 6 4 2 1
                    "#
            )),
            1
        );
    }

    #[test]
    fn ex_1_input_prefix() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        2 5 6 8 6               # 0
                        87 89 90 93 96 99 99    # 0
                        13 14 15 18 19 23       # 0
                        67 69 71 72 73 76 82    # 0
                        29 32 30 31 34 35 37    # 0
                        54 56 54 57 54          # 0
                        70 73 75 74 77 79 81 81 # 0
                        53 55 56 59 62 61 65    # 0
                        90 93 95 92 99          # 0
                        58 61 61 64 67          # 0
                        36 37 37 39 42 39       # 0
                        32 35 38 40 40 40       # 0
                        17 19 21 22 23 25 28    # 1
                        9 11 12 14 16           # 2
                    "#
            )),
            2
        );
    }

    #[test]
    fn ex_1_empty_input() {
        assert_eq!(ex_1::count_safe_reports(&parse_input("")), 0);
    }

    #[test]
    fn ex_1_max_delta_allowed() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        1 4 7
                        10 7 4
                    "#
            )),
            2
        );
    }

    #[test]
    fn ex_1_exact_delta_boundaries() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        1 2 3 4
                        4 3 2 1
                    "#
            )),
            2
        );
    }

    #[test]
    fn ex_1_invalid_deltas_exceeding_three() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        1 5
                        5 1
                    "#
            )),
            0
        );
    }

    #[test]
    fn ex_1_mixed_increasing_decreasing_within_deltas() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        1 2 1 2
                        3 4 3 4
                    "#
            )),
            0
        );
    }

    #[test]
    fn ex_1_all_same_numbers() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        5 5 5 5
                    "#
            )),
            0
        );
    }

    #[test]
    fn ex_1_two_level_reports() {
        assert_eq!(
            ex_1::count_safe_reports(&parse_input(
                r#"
                        1 2
                        2 1
                        3 6
                        6 3
                    "#
            )),
            4
        );
    }

    #[test]
    fn ex_2_example() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input(SIMPLE)),
            4
        );
    }

    #[test]
    fn ex_2_example_unsafe() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("9 7 6 2 1")),
            0
        );
    }

    #[test]
    fn ex_2_problem_at_the_start() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("3 1 2 4 5")),
            1
        );
    }

    #[test]
    fn ex_2_problem_at_the_end() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("1 2 4 5 3")),
            1
        );
    }

    #[test]
    fn ex_2_problem_at_the_end_short() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("4 5 3")),
            1
        );
    }

    #[test]
    fn ex_2_problem_4_3_5_3() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("4 3 5 3")),
            0
        );
    }

    #[test]
    fn ex_2_problem_4_3_5_2() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("4 3 5 2")),
            1
        );
    }

    #[test]
    fn ex_2_problem_4_3_5_6() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("4 3 5 6")),
            1
        );
    }
}
