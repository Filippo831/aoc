use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(input: &str) -> usize {
    let mut antenna_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut square_size: usize = 0;

    for (line_index, line) in fs::read_to_string(input).unwrap().lines().enumerate() {
        square_size = line.len();
        for (c_index, c) in line.chars().enumerate() {
            if c != '.' {
                antenna_positions
                    .entry(c)
                    .or_default()
                    .push((line_index, c_index));
            }
        }
    }

    let mut antinode_set: HashSet<String> = HashSet::new();

    for (key, value) in antenna_positions {
        for (v1_index, v1) in value.iter().enumerate() {
            for v2 in value[v1_index + 1..].iter() {
                let offset: (i32, i32) = ((v1.0 as i32 - v2.0 as i32), (v1.1 as i32 - v2.1 as i32));

                // apply this result to v1 and the opposite to v2
                let end_position_y: usize = (v1.0 as i32 + offset.0) as usize;
                let end_position_x: usize = (v1.1 as i32 + offset.1) as usize;
                if end_position_x < square_size && end_position_y < square_size {
                    antinode_set.insert(format!("{end_position_y}:{end_position_x}"));
                }

                let end_position_y: usize = (v2.0 as i32 - offset.0) as usize;
                let end_position_x: usize = (v2.1 as i32 - offset.1) as usize;
                if end_position_x < square_size && end_position_y < square_size {
                    antinode_set.insert(format!("{end_position_y}:{end_position_x}"));
                }
            }
        }
    }
    for el in antinode_set.iter() {
        println!("{}", &el);
    }

    return antinode_set.len();
}

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 14);
    }
}