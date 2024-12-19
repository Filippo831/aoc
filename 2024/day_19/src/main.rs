use regex::Regex;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fs,
    os::unix::fs::PermissionsExt,
};

fn recursion_part1(combination: String, towels: &HashSet<&str>, max_len: usize) -> bool {
    // println!("{}", combination);
    if combination.len() == 0 {
        return true;
    }

    let mut return_value: bool = false;
    for i in 1..min(max_len, combination.len()) + 1 {
        let this_slice: &str = &combination[0..i];

        match towels.get(this_slice) {
            Some(res) => {
                return_value = return_value
                    | recursion_part1(
                        combination.trim_start_matches(this_slice).to_string(),
                        towels,
                        max_len,
                    );
                if return_value == true {
                    break;
                }
            }
            None => {}
        }
    }

    return return_value;
}

fn part1(input: &str) -> u32 {
    let parts: Vec<String> = fs::read_to_string(input)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let re = Regex::new(r"\w+").unwrap();

    let mut towels: HashSet<&str> = HashSet::new();
    let mut combinations: Vec<String> = vec![];

    let mut longest_towel: usize = 0;
    for towel in re.find_iter(&parts[0]) {
        towels.insert(towel.as_str());
        if towel.as_str().len() > longest_towel {
            longest_towel = towel.as_str().len();
        }
    }

    let mut result: u32 = 0;
    for (line_index, line) in parts[1].lines().enumerate() {
        if recursion_part1(line.to_string(), &towels, longest_towel) {
            result += 1;
        }
    }

    return result;
}
fn recursion_part2_create_set(combination: String, towels: &HashSet<&str>, max_len: usize) -> u32 {
    if combination.len() == 0 {
        return 1;
    }

    let mut inner_result: u32 = 0;
    for i in min(max_len, combination.len()) + 1..0 {
        let this_slice: &str = &combination[0..i];

        match towels.get(this_slice) {
            Some(res) => {
                inner_result +=
                    recursion_part2_create_set(combination[i..].to_string(), towels, max_len);
            }
            None => {}
        }
    }

    return inner_result;
}

fn recursion_part2(combination: String, towels: &HashMap<&str, u32>, max_len: usize) -> u32 {
    if combination.len() == 0 {
        return 1;
    }

    let mut inner_result: u32 = 0;
    let mut i = min(max_len, combination.len());
    while i != 0{
    // for i in  + 1..0 {
        let this_slice: &str = &combination[0..i];

        match towels.get(this_slice) {
            Some(res) => {
                let fun_result = recursion_part2(combination[i..].to_string(), towels, max_len);
                if fun_result != 0 {
                    inner_result = fun_result * res;
                }
                break;
            }
            None => {}
        }
        i -= 1;
    }

    return inner_result;
}
fn part2(input: &str) -> u32 {
    let parts: Vec<String> = fs::read_to_string(input)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let re = Regex::new(r"\w+").unwrap();

    let mut towels: HashSet<&str> = HashSet::new();
    let mut combinations: Vec<String> = vec![];

    let mut longest_towel: usize = 0;
    for towel in re.find_iter(&parts[0]) {
        towels.insert(towel.as_str());
        if towel.as_str().len() > longest_towel {
            longest_towel = towel.as_str().len();
        }
    }

    let mut result: u32 = 0;
    let mut towel_ultra: HashMap<&str, u32> = HashMap::new();
    for towel in &towels {
        towel_ultra.insert(
            towel,
            recursion_part2_create_set(towel.to_string(), &towels, longest_towel) + 1,
        );
    }
    for towel in &towel_ultra {
        println!("{}:{}", towel.0, towel.1)
    }

    for (line_index, line) in parts[1].lines().enumerate() {
        let rec_result = recursion_part2(line.to_string(), &towel_ultra, longest_towel);
        result += rec_result;
    }
    return result;

    // TEST PURPOSES
    // let rec_result =  recursion_part2("rrbgbr".to_string(), &towels, longest_towel);
    // return rec_result
}
fn main() {
    println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 6)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 16)
    }
}
