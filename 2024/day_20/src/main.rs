use std::{
    collections::{HashMap, HashSet},
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

fn calc_distance(v1: &(usize, usize), v2: &(usize, usize)) -> usize {
    let distance = (v2.0 as i32 - v1.0 as i32).abs() + (v2.1 as i32 - v1.1 as i32).abs();
    return distance as usize;
}

fn part1(input: &str) -> u32 {
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut vertices: HashSet<(usize, usize)> = HashSet::new();
    let mut costs: HashMap<(usize, usize), u32> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut end_position: (usize, usize) = (0, 0);
    let mut start_position: (usize, usize) = (0, 0);

    let mut size: usize = 0;

    for (line_index, line) in fs::read_to_string(input).unwrap().lines().enumerate() {
        size = line.len();
        for (c_index, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert((line_index, c_index));
                costs.insert((line_index, c_index), u32::max_value());
            } else if c == '.' {
                costs.insert((line_index, c_index), u32::max_value());
                vertices.insert((line_index, c_index));
            } else if c == 'E' {
                end_position = (line_index, c_index);
                vertices.insert((line_index, c_index));
                costs.insert((line_index, c_index), u32::max_value());
            } else if c == 'S' {
                costs.insert((line_index, c_index), 0);
                vertices.insert((line_index, c_index));
                start_position = (line_index, c_index);
            }
        }
    }

    let position: (usize, usize) = (0, 0);
    costs.insert(position, 0);

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..size * size {
        let mut current_lowest_cost: u32 = u32::max_value();
        let mut current_vert: (usize, usize) = (0, 0);

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
    let total_cost = costs.get(&end_position).unwrap();
    println!("{}", total_cost);

    let mut path: Vec<(usize, usize)> = vec![];

    let mut position = end_position;
    path.push(end_position);
    while position != start_position {
        match prev.get(&position) {
            Some(p) => {
                path.push(*p);
                position = *p;
            }
            None => {}
        }
    }

    let mut cheats: HashMap<usize, usize> = HashMap::new();

    let path_size = path.len();
    for p1_index in 0..path_size - 1 {
        for p2_index in p1_index + 1..path_size {
            let p1 = path[p1_index];
            let p2 = path[p2_index];
            let distance = calc_distance(&p1, &p2);
            if distance == 2 {
                let time_distance = (p2_index as i32 - p1_index as i32) as usize;

                if time_distance > 2 {
                    match cheats.get(&(time_distance - 2)) {
                        Some(value) => {
                            cheats.insert(time_distance - 2, value + 1);
                        }
                        None => {
                            cheats.insert(time_distance - 2, 1);
                        }
                    }
                }
            }
        }
    }

    let mut result: u32 = 0;
    for e in cheats {
        if e.0 >= 100 {
            result += e.1 as u32;
        }
    }

    return result;
}

fn part2(input: &str) -> u64 {
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut vertices: HashSet<(usize, usize)> = HashSet::new();
    let mut costs: HashMap<(usize, usize), u32> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut end_position: (usize, usize) = (0, 0);
    let mut start_position: (usize, usize) = (0, 0);

    let mut size: usize = 0;

    for (line_index, line) in fs::read_to_string(input).unwrap().lines().enumerate() {
        size = line.len();
        for (c_index, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert((line_index, c_index));
                costs.insert((line_index, c_index), u32::max_value());
            } else if c == '.' {
                costs.insert((line_index, c_index), u32::max_value());
                vertices.insert((line_index, c_index));
            } else if c == 'E' {
                end_position = (line_index, c_index);
                vertices.insert((line_index, c_index));
                costs.insert((line_index, c_index), u32::max_value());
            } else if c == 'S' {
                costs.insert((line_index, c_index), 0);
                vertices.insert((line_index, c_index));
                start_position = (line_index, c_index);
            }
        }
    }

    let position: (usize, usize) = (0, 0);
    costs.insert(position, 0);

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..size * size {
        let mut current_lowest_cost: u32 = u32::max_value();
        let mut current_vert: (usize, usize) = (0, 0);

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
    let total_cost = costs.get(&end_position).unwrap();
    println!("{}", total_cost);

    let mut path: Vec<(usize, usize)> = vec![];

    let mut position = end_position;
    path.push(end_position);
    while position != start_position {
        match prev.get(&position) {
            Some(p) => {
                path.push(*p);
                position = *p;
            }
            None => {}
        }
    }

    let mut cheats: HashMap<usize, usize> = HashMap::new();

    let path_size = path.len();
    for p1_index in 0..path_size - 1 {
        for p2_index in p1_index + 1..path_size {
            let p1 = path[p1_index];
            let p2 = path[p2_index];
            let distance = calc_distance(&p1, &p2);
            if distance <= 20 {
                let time_distance = (p2_index as i32 - p1_index as i32) as usize;

                if time_distance > distance {
                    match cheats.get(&(time_distance - distance)) {
                        Some(value) => {
                            cheats.insert(time_distance - distance, value + 1);
                        }
                        None => {
                            cheats.insert(time_distance - distance, 1);
                        }
                    }
                }
            }
        }
    }

    let mut result: u64 = 0;
    for e in cheats {
        if e.0 >= 100{
            // println!("{}:{}", e.0, e.1);
            result += e.1 as u64;
        }
    }

    return result;
}
fn main() {
    // println!("{}", part1("input.txt"));
    println!("{}", part2("input.txt"));
}
