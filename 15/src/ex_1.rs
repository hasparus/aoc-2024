use crate::{parse_input::*, point2::Point2};

fn move_robot(input: &Input) -> Map {
    let mut map = input.map.clone();
    let mut robot_pos = find_robot(input);

    for direction in input.moves.iter() {
        if let Some(new_pos) = move_object(&mut map, &robot_pos, direction) {
            robot_pos = new_pos;
        }
    }

    map
}

fn move_object(map: &mut Map, pos: &Point2, direction: &Direction) -> Option<Point2> {
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

fn cell_in_direction(pos: &Point2, direction: &Direction) -> Point2 {
    let new_pos = match direction {
        Direction::Up => Point2::new(pos.row - 1, pos.col),
        Direction::Down => Point2::new(pos.row + 1, pos.col),
        Direction::Left => Point2::new(pos.row, pos.col - 1),
        Direction::Right => Point2::new(pos.row, pos.col + 1),
    };

    new_pos
}

fn find_robot(input: &Input) -> Point2 {
    for (i, row) in input.map.iter().enumerate() {
        for (j, token) in row.iter().enumerate() {
            if token == &Token::Robot {
                return Point2::new(i, j);
            }
        }
    }

    panic!("Robot not found");
}

fn sum_up_coordinates(map: &Map) -> usize {
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
            .parse::<Map>()?
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
            .parse::<Map>()?
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
            .parse::<Map>()?
        );

        Ok(())
    }
}
