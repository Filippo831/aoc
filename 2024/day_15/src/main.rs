use std::{
    collections::HashSet,
    fs,
};

fn calc_position(first: (u32, u32), second: (i32, i32)) -> (u32, u32) {
    let temp_x = (first.0 as i32 + second.0) as u32;
    let temp_y = (first.1 as i32 + second.1) as u32;
    return (temp_x, temp_y);
}

fn recursion(
    position: (u32, u32),
    movement: (i32, i32),
    walls: &mut HashSet<(u32, u32)>,
    boxes: &mut HashSet<(u32, u32)>,
) -> bool {
    let next_position = calc_position(position, movement);

    match walls.get(&next_position) {
        Some(_) => return false,
        None => {},
    }

    let result: bool;
    match boxes.get(&next_position) {
        Some(_) => result = recursion(next_position, movement, walls, boxes),
        None => return true
    }

    if result {
        boxes.insert(calc_position(next_position, movement));
        boxes.remove(&next_position);
        return true
    } else {
        return false
    }
}

fn part1(input: &str) -> u64 {
    let file = fs::read_to_string(input).unwrap();

    let parts: Vec<&str> = file.split("\n\n").collect();

    let mut walls: HashSet<(u32, u32)> = HashSet::new();
    let mut boxes: HashSet<(u32, u32)> = HashSet::new();

    let mut robot_position: (u32, u32) = (0,0);

    for (line_index, line) in parts[0].split("\n").enumerate() {
        for (c_index, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert((line_index as u32, c_index as u32));
            } else if c == 'O' {
                boxes.insert((line_index as u32, c_index as u32));
            } else if c == '@' {
                robot_position = (line_index as u32, c_index as u32);
            }
        }
    }

    let mut movements: Vec<(i32, i32)> = vec![];

    for line in parts[1].split("\n") {
        for c in line.chars() {
            match c {
                '<' => movements.push((0, -1)),
                '^' => movements.push((-1, 0)),
                '>' => movements.push((0, 1)),
                'v' => movements.push((1, 0)),
                _ => {}
            }
        }
    }

    for m in movements {
        let result = recursion(robot_position, m, &mut walls, &mut boxes);
        if result {
            robot_position = calc_position(robot_position, m);
        }
    }

    let mut result: u64 = 0;

    for b in &boxes {
        result += (b.0 * 100 + b.1) as u64;
    }

    return result;
}

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("input_test.txt"), 2028);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1("input_test2.txt"), 10092);
    }
}
