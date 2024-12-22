use std::collections::HashMap;
use std::fs;

fn calculate_string_numbers(map: &HashMap<char, (usize, usize)>, start: char, end: char) -> String {
    let mut output_string: String = "".to_string();
    let start_position: (usize, usize) = *map.get(&start).unwrap();
    let end_position: (usize, usize) = *map.get(&end).unwrap();

    let x_offset: i32 = end_position.0 as i32 - start_position.0 as i32;
    let y_offset: i32 = end_position.1 as i32 - start_position.1 as i32;
    if y_offset < 0 {
    if x_offset < 0 {
        for i in 0..x_offset.abs() {
            output_string += "<";
        }
    }
        for i in 0..y_offset.abs() {
            output_string += "^";
        }
    }
    if y_offset > 0 {
        for i in 0..y_offset {
            output_string += "v";
        }
    }
    if x_offset > 0 {
        for i in 0..x_offset {
            output_string += ">";
        }
    }
    output_string += "A";

    return output_string;
}
fn calculate_string_movements(map: &HashMap<char, (usize, usize)>, start: char, end: char) -> String {
    let mut output_string: String = "".to_string();
    let start_position: (usize, usize) = *map.get(&start).unwrap();
    let end_position: (usize, usize) = *map.get(&end).unwrap();

    let x_offset: i32 = end_position.0 as i32 - start_position.0 as i32;
    let y_offset: i32 = end_position.1 as i32 - start_position.1 as i32;
    if x_offset < 0 {
        for i in 0..x_offset.abs() {
            output_string += "<";
        }
    }
    if y_offset < 0 {
        for i in 0..y_offset.abs() {
            output_string += "^";
        }
    }
    if y_offset > 0 {
        for i in 0..y_offset {
            output_string += "v";
        }
    }
    if x_offset > 0 {
        for i in 0..x_offset {
            output_string += ">";
        }
    }
    output_string += "A";

    return output_string;
}

fn part1(input: &str, n_people: u32) -> u64 {
    let numeric_position: HashMap<char, (usize, usize)> = HashMap::from([
        ('9', (2, 0)),
        ('8', (1, 0)),
        ('7', (0, 0)),
        ('6', (2, 1)),
        ('5', (1, 1)),
        ('4', (0, 1)),
        ('3', (2, 2)),
        ('2', (1, 2)),
        ('1', (0, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);
    let direction_position: HashMap<char, (usize, usize)> = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut positions: Vec<char> = vec![];

    for line in fs::read_to_string(input).unwrap().lines() {
        let mut new_combination: String = "".to_string();
        let mut old_combination: String = line.to_string();
        // println!("{}: {}", line, old_combination);
        let mut this_position: char = 'A';
        for char in old_combination.chars() {
            new_combination = format!(
                "{}{}",
                new_combination,
                calculate_string_numbers(&numeric_position, this_position, char)
            );
            this_position = char;
        }
        // println!("{}: {}", line, new_combination);
        for i in 0..n_people - 1 {
            old_combination = new_combination.to_string();
            new_combination = "".to_string();
            this_position = 'A';
            for char in old_combination.chars() {
                new_combination = format!(
                    "{}{}",
                    new_combination,
                    calculate_string_movements(&direction_position, this_position, char)
                );
                this_position = char;
            }
        }
        println!("{}: {}", line, new_combination);
    }

    return 0;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt", 4), 126384)
    }
}
