use std::{
    collections::{HashMap, HashSet},
    env::split_paths,
    fs,
};

fn calc_next_position(
    current: (usize, usize),
    direction: (i32, i32),
    size: usize,
) -> Result<(usize, usize), String> {
    let next_position = (
        (current.0 as i32 + direction.0) as usize,
        (current.1 as i32 + direction.1) as usize,
    );

    if next_position.0 >= size || next_position.1 >= size {
        return Err("out of bounds".to_string());
    }
    return Ok(next_position);
}

fn part1(input: &str, size: usize, n_bytes: usize) -> u32 {
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut vertices: HashSet<(usize, usize)> = HashSet::new();
    let mut costs: HashMap<(usize, usize), u32> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut fallen_bytes: usize = 0;
    for row in fs::read_to_string(input).unwrap().lines() {
        if fallen_bytes < n_bytes {
            let result: Vec<usize> = row
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            walls.insert((result[0], result[1]));
        }
        fallen_bytes += 1;
    }

    for x in 0..size {
        for y in 0..size {
            match walls.get(&(x, y)) {
                Some(val) => {}
                None => {
                    vertices.insert((x, y));
                    costs.insert((x, y), u32::max_value());
                }
            }
        }
    }

    let position: (usize, usize) = (0, 0);
    costs.insert(position, 0);

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for a in 0..(size * size) {
        let mut current_lowest_cost: u32 = u16::max_value() as u32;
        let mut current_vert: (usize, usize) = (size, size);

        for element in &vertices {
            match costs.get(&element) {
                Some(value) => {
                    if *value < current_lowest_cost {
                        current_lowest_cost = *value;
                        current_vert = *element;
                    }
                }
                None => {}
            }
        }

        vertices.remove(&current_vert);

        for direction in directions {
            match calc_next_position(current_vert, direction, size) {
                Ok(next_position) => match vertices.get(&next_position) {
                    Some(pos) => match costs.get(pos) {
                        Some(cost) => {
                            if current_lowest_cost + 1 < *cost {
                                costs.insert(*pos, current_lowest_cost + 1);
                                prev.insert(*pos, current_vert);
                            }
                        }
                        None => {}
                    },
                    None => {}
                },
                Err(str) => {}
            }
        }
    }
    match costs.get(&(size - 1, size - 1)) {
        Some(val) => return *val,
        None => return 0,
    }
}

fn part2(input: &str, size: usize) -> String {
    let input_len: usize = fs::read_to_string(input)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .len();
    let mut split_size: usize = input_len / 2;
    let mut current_index: usize = input_len / 2;

    let mut blocked: bool = false;

    while split_size > 2 {
        split_size /= 2;
        let res = part1(input, size, current_index);
        if res == u32::max_value() as u32 {
            current_index -= split_size;
            blocked = true;
        } else {
            current_index += split_size;
            blocked = false
        }
    }

    while true {
        if blocked {
            current_index -= 1;
            if part1(input, size, current_index - 1) != u32::max_value() as u32 {
                break;
            }
        } else {
            current_index += 1;
            if part1(input, size, current_index) == u32::max_value() as u32 {
                break;
            }
        }
    }

    return fs::read_to_string(input)
        .unwrap()
        .lines()
        .nth(current_index)
        .unwrap()
        .to_string();
}
fn main() {
    println!("part1: {}", part1("input.txt", 71, 1024));
    println!("part2: {}", part2("input.txt", 71));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt", 7, 12), 22);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt", 7), "6,1");
    }
}
