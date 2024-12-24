use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

fn part1(input: &str) -> u64 {
    let file_values = fs::read_to_string(input).unwrap();

    let first_part: &str = file_values
        .split("\n\n")
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap();
    let second_part: &str = file_values
        .split("\n\n")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap();

    let mut variables: HashMap<&str, bool> = HashMap::new();
    for line in first_part.lines() {
        let variable_name: &str = line.split(": ").collect::<Vec<&str>>().get(0).unwrap();
        let variable_value: bool = line
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .map(|e| if *e == "1" { true } else { false })
            .unwrap();
        variables.insert(variable_name, variable_value);
    }

    let re = Regex::new(r"\w+").unwrap();
    let mut z_values: Vec<&str> = vec![];
    let mut expressions: HashSet<Vec<&str>> = HashSet::new();
    for line in second_part.lines() {
        let things: Vec<&str> = re.find_iter(line).map(|e| e.as_str()).collect();
        expressions.insert(things);
    }
    loop {
        for things in &expressions {
            if let Some(left_value) = &variables.get(things[0]).clone() {
                if let Some(right_value) = &variables.get(things[2]).clone() {
                    if things[3].starts_with("z") {
                        z_values.push(things[3]);
                    }
                    if things[1] == "XOR" {
                        variables.insert(things[3], *left_value ^ *right_value);
                    }
                    if things[1] == "AND" {
                        variables.insert(things[3], *left_value & *right_value);
                    }
                    if things[1] == "OR" {
                        variables.insert(things[3], *left_value | *right_value);
                    }
                }
            }
        }
        if expressions.len() == 0 {
            break;
        }
    }
    dbg!("{:?}", variables);

    return 0;
}

fn main() {
    println!("{}", part1("input_test.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 2024)
    }
}
