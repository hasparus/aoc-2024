use crate::{ex_1::cell_in_direction, parse_input::*};
use aoc_2024_lib::{board::Board, point2::Point2};
use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq, Eq, Clone, Copy)]
pub enum UpscaledToken {
    #[display("#")]
    Wall,
    #[display(".")]
    Empty,
    #[display("@")]
    Robot,
    #[display("[")]
    BoxLeft,
    #[display("]")]
    BoxRight,
}

pub fn solve(input: &Input) -> usize {
    let upscaled_map = upscale_map(&input.map);
    let final_map = move_robot(&upscaled_map, &input.moves);
    sum_up_coordinates(&final_map)
}

fn upscale_map(map: &Board<Token>) -> Board<UpscaledToken> {
    Board(
        map.iter()
            .map(|row| {
                row.iter()
                    .flat_map(|token| match token {
                        Token::Wall => [UpscaledToken::Wall, UpscaledToken::Wall],
                        Token::Box => [UpscaledToken::BoxLeft, UpscaledToken::BoxRight],
                        Token::Robot => [UpscaledToken::Robot, UpscaledToken::Empty],
                        Token::Empty => [UpscaledToken::Empty, UpscaledToken::Empty],
                    })
                    .collect()
            })
            .collect(),
    )
}

fn move_robot(map: &Board<UpscaledToken>, moves: &[Direction]) -> Board<UpscaledToken> {
    let mut map = map.clone();
    let mut robot_pos = map.find(&UpscaledToken::Robot);

    for direction in moves.iter() {
        if let Some(new_pos) = move_object(&mut map, &robot_pos, direction) {
            robot_pos = new_pos;
        }
    }

    map
}

fn move_object(
    map: &mut Board<UpscaledToken>,
    pos: &Point2,
    direction: &Direction,
) -> Option<Point2> {
    let current = map[pos];
    let new_pos = cell_in_direction(pos, direction);

    match current {
        UpscaledToken::Wall => None,
        UpscaledToken::Empty => Some(*pos),
        UpscaledToken::Robot => {
            if move_object(map, &new_pos, direction).is_some() {
                map[new_pos] = current;
                map[pos] = UpscaledToken::Empty;
                Some(new_pos)
            } else {
                None
            }
        }
        UpscaledToken::BoxLeft => {
            let right_pos = *pos + (0, 1).into();
            if map[right_pos] != UpscaledToken::BoxRight {
                panic!("Box left without box right");
            }

            let new_right_pos = new_pos + (0, 1).into();

            if move_object(map, &new_pos, direction).is_some()
                && if new_pos == right_pos {
                    true
                } else {
                    move_object(map, &new_right_pos, direction).is_some()
                }
            {
                map[new_pos] = UpscaledToken::BoxLeft;
                map[new_right_pos] = UpscaledToken::BoxRight;
                map[pos] = UpscaledToken::Empty;
                map[right_pos] = UpscaledToken::Empty;
                Some(new_pos)
            } else {
                None
            }
        }
        UpscaledToken::BoxRight => {
            let left_pos = *pos - (0, 1).into();
            if map[left_pos] != UpscaledToken::BoxLeft {
                panic!("Box right without box left\n{}", map);
            }

            let new_left_pos = new_pos - (0, 1).into();

            if move_object(map, &new_pos, direction).is_some()
                && if new_pos == left_pos {
                    true
                } else {
                    move_object(map, &new_left_pos, direction).is_some()
                }
            {
                map[new_pos] = UpscaledToken::BoxRight;
                map[new_left_pos] = UpscaledToken::BoxLeft;
                map[pos] = UpscaledToken::Empty;
                map[left_pos] = UpscaledToken::Empty;
                Some(new_pos)
            } else {
                None
            }
        }
    }
}

fn sum_up_coordinates(map: &Board<UpscaledToken>) -> usize {
    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, token) in row.iter().enumerate() {
            if token == &UpscaledToken::BoxLeft {
                sum += 100 * i + j;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2024_lib::input_reader::read_input;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_upscale_map() -> Result<(), Box<dyn std::error::Error>> {
        let board = "
                ####
                #@.#
                #O.#
                ####
            "
        .parse::<Board<Token>>()?;

        let upscaled = upscale_map(&board);

        assert_eq!(
            upscaled.to_string().trim(),
            "
                ########
                ##@...##
                ##[]..##
                ########
            "
            .trim()
            .replace(" ", "")
        );

        Ok(())
    }

    #[test]
    fn test_ex_2_large() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = read_input("./inputs.md")?;
        let example = input_file.get_input("Large");
        assert_eq!(solve(&parse_input(&example.content)?), 9021);
        Ok(())
    }
}
