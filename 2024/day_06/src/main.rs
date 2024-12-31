use std::{collections::HashSet, fs};

fn part1(path: &str) -> i32 {
    let mut field: Vec<String> = vec![];
    let mut guard_position: (usize, usize) = (0, 0);

    for (index, line) in fs::read_to_string(path).unwrap().lines().enumerate() {
        field.push(line.to_string());
        match line.find("^") {
            Some(value) => guard_position = (index, value),
            None => {}
        }
    }

    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction_index: usize = 0;

    let mut current_direction = directions[direction_index];

    let mut iterations = 1;

    loop {
        if field[guard_position.0]
            .chars()
            .nth(guard_position.1)
            .unwrap()
            == '.'
        {
            iterations += 1;
            field[guard_position.0].replace_range(guard_position.1..guard_position.1 + 1, "X");
        }

        let next_position: (usize, usize) = (
            (current_direction.0 + guard_position.0 as i32) as usize,
            (current_direction.1 + guard_position.1 as i32) as usize,
        );

        if next_position.0 >= field.len() || next_position.1 >= field.len() {
            break;
        }

        if field[next_position.0].chars().nth(next_position.1).unwrap() == '#' {
            direction_index = (direction_index + 1) % directions.len();
            current_direction = directions[direction_index];
        }

        guard_position.0 = (guard_position.0 as i32 + current_direction.0) as usize;
        guard_position.1 = (guard_position.1 as i32 + current_direction.1) as usize;
    }

    return iterations;
}

fn part2(path: &str) -> i32 {
    let mut field: Vec<String> = vec![];
    let mut guard_position: (usize, usize) = (0, 0);
    let mut seen_positions: HashSet<(usize, usize)> = HashSet::new();

    for (index, line) in fs::read_to_string(path).unwrap().lines().enumerate() {
        field.push(line.to_string());
        match line.find("^") {
            Some(value) => {
                guard_position = (index, value);
                seen_positions.insert((index, value));
            }
            None => {}
        }
    }

    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction_index: usize = 0;

    let mut current_direction = directions[direction_index];

    let mut guard_position_copy = guard_position.clone();

    loop {
        if field[guard_position_copy.0]
            .chars()
            .nth(guard_position_copy.1)
            .unwrap()
            == '.'
        {
            seen_positions.insert(guard_position_copy);
        }

        let next_position: (usize, usize) = (
            (current_direction.0 + guard_position_copy.0 as i32) as usize,
            (current_direction.1 + guard_position_copy.1 as i32) as usize,
        );

        if next_position.0 >= field.len() || next_position.1 >= field.len() {
            break;
        }

        if field[next_position.0].chars().nth(next_position.1).unwrap() == '#' {
            direction_index = (direction_index + 1) % directions.len();
            current_direction = directions[direction_index];
        }

        guard_position_copy.0 = (guard_position_copy.0 as i32 + current_direction.0) as usize;
        guard_position_copy.1 = (guard_position_copy.1 as i32 + current_direction.1) as usize;
    }

    let mut iterations = 0;

    let mut seen_position_direction: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut new_field: Vec<String> = vec![];

    for (position_index, position) in seen_positions.iter().enumerate() {
        println!("{} / {}", position_index, seen_positions.len());
        direction_index = 0;
        current_direction = directions[direction_index];
        let mut guard_position_copy_again = guard_position.clone();
        seen_position_direction.clear();

        new_field = field.clone();

        new_field[position.0].replace_range(position.1..position.1 + 1, "#");

        let mut is_good: bool = false;

        loop {
            if new_field[guard_position_copy_again.0]
                .chars()
                .nth(guard_position_copy_again.1)
                .unwrap()
                == '.'
            {
                match seen_position_direction.get(&(
                    guard_position_copy_again.0,
                    guard_position_copy_again.1,
                    direction_index,
                )) {
                    Some(_value) => {
                        is_good = true;
                        break;
                    }
                    None => seen_position_direction.insert((
                        guard_position_copy_again.0,
                        guard_position_copy_again.1,
                        direction_index,
                    )),
                };
            }

            let next_position: (usize, usize) = (
                (current_direction.0 + guard_position_copy_again.0 as i32) as usize,
                (current_direction.1 + guard_position_copy_again.1 as i32) as usize,
            );

            if next_position.0 >= new_field.len() || next_position.1 >= new_field.len() {
                break;
            }

            if new_field[next_position.0]
                .chars()
                .nth(next_position.1)
                .unwrap()
                == '#'
            {
                direction_index = (direction_index + 1) % directions.len();
                current_direction = directions[direction_index];
            }

            guard_position_copy_again.0 =
                (guard_position_copy_again.0 as i32 + current_direction.0) as usize;
            guard_position_copy_again.1 =
                (guard_position_copy_again.1 as i32 + current_direction.1) as usize;
        }
        if is_good {
            iterations += 1;
        }
    }

    return iterations;
}

fn main() {
    // println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: i32 = part1("input_test.txt");
        assert_eq!(result, 41)
    }

    #[test]
    fn test_part2() {
        let result: i32 = part2("input_test.txt");
        assert_eq!(result, 6)
    }
}
