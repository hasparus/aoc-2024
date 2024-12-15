use crate::{ex1, parse_input::Game};

static PRIZE_SHIFT: i64 = 10000000000000;

fn shift_prizes(game: &Game) -> Game {
    let mut game = game.clone();
    for round in game.rounds.iter_mut() {
        round.prize.x += PRIZE_SHIFT;
        round.prize.y += PRIZE_SHIFT;
    }
    game
}

pub fn solve(input: &str) -> i64 {
    let game = input.parse::<Game>().expect("valid game input");
    let game = shift_prizes(&game);

    game.rounds.iter().filter_map(ex1::solve_round).sum()
}

#[cfg(test)]
mod tests {
    use ex1::solve_round;

    use super::*;
    use crate::input_reader::read_input;

    #[test]
    fn test_full_example() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = read_input("./inputs.md")?;
        let example = &inputs.get_input("Example").content;

        // "now it is only possible to win on 2nd and 4th claw machines"
        let game = example.parse::<Game>()?;
        let game = shift_prizes(&game);

        assert_eq!(solve_round(&game.rounds[0]), None);
        assert_eq!(solve_round(&game.rounds[1]), Some(459236326669));
        assert_eq!(solve_round(&game.rounds[2]), None);
        assert_eq!(solve_round(&game.rounds[3]), Some(416082282239));

        assert_eq!(solve(example), 875318608908);
        Ok(())
    }
}
