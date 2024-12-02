use aoc_2024_2::{ex_1, ex_2, ex_2_brute, parse_input};

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

    #[test]
    fn ex_2_analyzer_case_5() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("29 32 30 31 34 35 37")),
            1
        );
    }

    #[test]
    fn ex_2_analyzer_case_80() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("31 35 34 36 38 39")),
            1
        );
    }

    #[test]
    fn ex_2_analyzer_case_204() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("52 48 50 48 46")),
            1
        );
    }

    #[test]
    fn ex_2_analyzer_case_229() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("89 83 86 84 81")),
            1
        );
    }

    #[test]
    fn ex_2_analyzer_case_323() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("90 86 88 86 83")),
            1
        );
    }

    #[test]
    fn ex_2_analyzer_case_378() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("65 64 68 71 72 75")),
            1
        );
    }

    #[test]
    fn ex_2_analyzer_case_402() {
        assert_eq!(
            ex_2::count_safe_reports_with_problem_dampener(&parse_input("43 46 44 45 46")),
            1
        );
    }
}
