use std::fs;

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

        if field[guard_position.0].chars().nth(guard_position.1).unwrap() == '.' {
            iterations += 1;
            field[guard_position.0].replace_range(guard_position.1..guard_position.1+1, "X");
        }

        let next_position: (usize, usize) = (
            (current_direction.0 + guard_position.0 as i32) as usize,
            (current_direction.1 + guard_position.1 as i32) as usize,
        );

        if next_position.0 >= field.len() || next_position.1 >= field.len() {
            println!("{}:{}", guard_position.0, guard_position.1);
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

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: i32 = part1("input_test.txt");
        assert_eq!(result, 41)
    }
}
