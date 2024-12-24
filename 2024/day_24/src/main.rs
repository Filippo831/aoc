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
    while (expressions).len() > 0 {
        for el in expressions.clone() {
            if let Some(left_value) = variables.get(el[0]) {
                if let Some(right_value) = variables.get(el[2]) {
                    if el[3].starts_with("z") {
                        z_values.push(el[3]);
                    }
                    let mut result = true;
                    if el[1] == "AND" {
                        result = left_value & right_value;
                    }
                    if el[1] == "OR" {
                        result = left_value | right_value;
                    }
                    if el[1] == "XOR" {
                        result = left_value ^ right_value;
                    }
                    variables.insert(el[3], result);
                    expressions.remove(&el);
                }
            }
        }
    }
    z_values.sort();
    let mut final_result:u64 = 0;
    let mut el_index: usize = 0;
    for el in z_values.iter_mut() {
        let is_true: bool = *variables.get(el).unwrap();
        if is_true {
            let two: u64 = 2;
            final_result += two.pow(el_index as u32) as u64;
        }
        el_index += 1;
    }

    return final_result;
}

fn main() {
    println!("{}", part1("input.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 2024)
    }
}
