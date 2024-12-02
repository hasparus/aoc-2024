pub type Report = Vec<i32>;
pub type ParsedInput = Vec<Report>;

pub fn parse_input(input: &str) -> ParsedInput {
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

pub mod ex_1;
pub mod ex_2;
pub mod ex_2_brute; 
