use crate::parse_input::{Game, A_PRESS_COST, B_PRESS_COST};

pub fn solve_round(round: &crate::parse_input::GameRound) -> Option<i64> {
    let ax = round.button_a.x;
    let ay = round.button_a.y;
    let bx = round.button_b.x;
    let by = round.button_b.y;
    let px = round.prize.x;
    let py = round.prize.y;

    // Using Cramer's rule:
    // D = | ax  bx |
    //     | ay  by |
    let d = ax * by - ay * bx;
    if d == 0 {
        // Buttons move in same direction
        return None;
    }

    // Da = | px  bx |
    //      | py  by |
    let da = px * by - py * bx;

    // Db = | ax  px |
    //      | ay  py |
    let db = ax * py - ay * px;

    let a = da / d;
    let b = db / d;

    // Check if we have a valid solution
    if a >= 0 && b >= 0 && a * ax + b * bx == px && a * ay + b * by == py {
        Some(a * A_PRESS_COST + b * B_PRESS_COST)
    } else {
        None
    }
}

pub fn solve(input: &str) -> i64 {
    let game = input.parse::<Game>().expect("valid game input");
    game.rounds.iter().filter_map(solve_round).sum()
}

#[cfg(test)]
mod tests {
    use crate::parse_input::GameRound;
    use aoc_2024_lib::input_reader::read_input;

    use super::*;

    #[test]
    fn test_example_first_round() -> Result<(), Box<dyn std::error::Error>> {
        let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";

        assert_eq!(solve_round(&input.parse::<GameRound>()?), Some(280));
        Ok(())
    }

    #[test]
    fn test_impossible_case() -> Result<(), Box<dyn std::error::Error>> {
        let input = "\
Button A: X+20, Y+20
Button B: X+50, Y+50
Prize: X=10, Y=10";

        assert_eq!(solve_round(&input.parse::<GameRound>()?), None);
        Ok(())
    }

    #[test]
    fn test_full_example() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = read_input("./inputs.md")?;
        assert_eq!(solve(&inputs.get_input("Example").content), 480);
        Ok(())
    }
}
