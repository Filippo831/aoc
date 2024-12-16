use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fs,
};

fn calc_next_position(first: (u32, u32), second: (i32, i32)) -> (u32, u32) {
    let first_calc = (first.0 as i32 + second.0) as u32;
    let second_calc = (first.1 as i32 + second.1) as u32;
    return (first_calc, second_calc);
}

fn calc_rotation(current_direction_index: usize, clockwise: bool) -> usize {
    if clockwise {
        if current_direction_index == 0 {
            return 3;
        } else {
            return current_direction_index - 1;
        }
    } else {
        return (current_direction_index + 1) % 4;
    }
}

// approach n_1: stack overflow
fn recursion(
    walls: &mut HashSet<(u32, u32)>,
    inter_result: &mut HashMap<(u32, u32), u64>,
    directions: &[(i32, i32); 4],
    current_direction_index: usize,
    current_position: (u32, u32),
    end_position: (u32, u32),
    points: u64,
    rotate_count: u32,
    end: &mut u64,
) -> u64 {
    if points > *end {
        return 10000000000000;
    }

    match inter_result.get(&current_position) {
        Some(val) => {
            if points > *val * 2 + 1100 {
                return 1000000000000;
            } else if points < *val {
                inter_result.insert(current_position, points).unwrap();
            }
        }
        None => {
            inter_result.insert(current_position, points).unwrap();
        }
    }

    if current_position == end_position {
        println!("oetnahuntaohun: {}", points);
        *end = points;
        return points;
    }

    match walls.get(&current_position) {
        Some(val) => return 10000000000000000000,
        None => 0,
    };

    let mut best_value: u64 = 0;

    best_value = min(
        best_value,
        recursion(
            walls,
            inter_result,
            directions,
            current_direction_index,
            calc_next_position(current_position, directions[current_direction_index]),
            end_position,
            points + 1,
            0,
            end,
        ),
    );

    if rotate_count == 0 {
        best_value = min(
            best_value,
            recursion(
                walls,
                inter_result,
                directions,
                calc_rotation(current_direction_index, true),
                current_position,
                end_position,
                points + 1000,
                rotate_count + 1,
                end,
            ),
        );
        best_value = min(
            best_value,
            recursion(
                walls,
                inter_result,
                directions,
                calc_rotation(current_direction_index, false),
                current_position,
                end_position,
                points + 1000,
                rotate_count + 1,
                end,
            ),
        );
    }

    return best_value;
}

fn part1(input: &str) -> u64 {
    let mut walls: HashSet<(u32, u32)> = HashSet::new();

    let mut inter_result: HashMap<(u32, u32), u64> = HashMap::new();

    let mut end: u64 = 40000;

    let mut start_position: (u32, u32) = (0, 0);
    let mut end_position: (u32, u32) = (0, 0);

    let directions: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    for (line_index, line) in fs::read_to_string(input).unwrap().lines().enumerate() {
        for (c_index, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert((line_index as u32, c_index as u32));
            } else if c == 'E' {
                end_position = (line_index as u32, c_index as u32);
            } else if c == 'S' {
                start_position = (line_index as u32, c_index as u32);
            }
        }
    }
    let best_value = recursion(
        &mut walls,
        &mut inter_result,
        &directions,
        0,
        start_position,
        end_position,
        0,
        0,
        &mut end,
    );

    return best_value;
    // return 0;
}

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("input_test1.txt"), 7036)
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1("input_test2.txt"), 11048)
    }
}
