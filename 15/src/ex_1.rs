use aoc_2024_lib::board::Board;
use aoc_2024_lib::point2::Point2;

use crate::parse_input::{Direction, Input, Token};

pub fn solve(input: &Input) -> usize {
    let map = move_robot(input);
    sum_up_coordinates(&map)
}

fn move_robot(input: &Input) -> Board<Token> {
    let mut map = input.map.clone();
    let mut robot_pos = map.find(&Token::Robot);

    for direction in input.moves.iter() {
        if let Some(new_pos) = move_object(&mut map, &robot_pos, direction) {
            robot_pos = new_pos;
        }
    }

    map
}

fn move_object(map: &mut Board<Token>, pos: &Point2, direction: &Direction) -> Option<Point2> {
    let current = map[pos];

    // the wall is not movable
    if current == Token::Wall {
        return None;
    }

    // if we move into an empty cell, we just take it
    if current == Token::Empty {
        return Some(*pos);
    }

    let new_pos = cell_in_direction(pos, direction);

    // robots and boxes move the same way
    if move_object(map, &new_pos, direction).is_some() {
        map[new_pos] = current;
        map[pos] = Token::Empty;
        Some(new_pos)
    } else {
        None
    }
}

pub fn cell_in_direction(pos: &Point2, direction: &Direction) -> Point2 {
    let new_pos = match direction {
        Direction::Up => Point2::new(pos.row - 1, pos.col),
        Direction::Down => Point2::new(pos.row + 1, pos.col),
        Direction::Left => Point2::new(pos.row, pos.col - 1),
        Direction::Right => Point2::new(pos.row, pos.col + 1),
    };

    new_pos
}

fn sum_up_coordinates(map: &Board<Token>) -> usize {
    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, token) in row.iter().enumerate() {
            if token == &Token::Box {
                sum += 100 * i + j;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::parse_input::parse_input;
    use aoc_2024_lib::input_reader::read_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_sum_up_coordinates() -> Result<(), Box<dyn std::error::Error>> {
        let input = parse_input(
            "
                #######
                #...O..
                #......

                ^            
            ",
        )?;
        let sum = sum_up_coordinates(&input.map);

        assert_eq!(sum, 104);

        Ok(())
    }

    #[test]
    fn test_move_robot_single_move() -> Result<(), Box<dyn std::error::Error>> {
        let input = parse_input(
            "
                #######
                #...O..
                #.@....

                >
            ",
        )?;

        let map = move_robot(&input);

        assert_eq!(
            map,
            "
                #######
                #...O..
                #..@...
            "
            .parse::<Board<Token>>()?
        );

        Ok(())
    }

    #[test]
    fn test_move_robot_pushing_box() -> Result<(), Box<dyn std::error::Error>> {
        let input = parse_input(
            "
                #######
                #...O..
                #.@....

                >^>
            ",
        )?;

        let map = move_robot(&input);

        assert_eq!(
            map,
            "
                #######
                #...@O.
                #......
            "
            .parse::<Board<Token>>()?
        );

        Ok(())
    }

    #[test]
    fn test_move_robot_pushing_box_into_wall() -> Result<(), Box<dyn std::error::Error>> {
        let input = parse_input(
            "
                #######
                #......
                #.O....
                #.@....

                ^^^^^^>^<<<<
            ",
        )?;

        let map = move_robot(&input);

        assert_eq!(
            map,
            "
                #######
                #O@....
                #......
                #......
            "
            .parse::<Board<Token>>()?
        );

        Ok(())
    }

    #[test]
    fn test_ex_1_small() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = read_input("./inputs.md")?;

        let small = input_file.get_input("Small");

        assert_eq!(solve(&parse_input(&small.content)?), 2028);

        Ok(())
    }

    #[test]
    fn test_ex_1_large_looks_as_expected() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = read_input("./inputs.md")?;

        let map = move_robot(&parse_input(&input_file.get_input("Large").content)?);

        assert_eq!(
            map.to_string().trim(),
            "
                ##########
                #.O.O.OOO#
                #........#
                #OO......#
                #OO@.....#
                #O#.....O#
                #O.....OO#
                #O.....OO#
                #OO....OO#
                ##########
            "
            .trim()
            .replace(" ", "")
        );

        Ok(())
    }

    #[test]
    fn test_ex_1_large() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = read_input("./inputs.md")?;

        let example = input_file.get_input("Large");

        assert_eq!(solve(&parse_input(&example.content)?), 10092);

        Ok(())
    }
}
