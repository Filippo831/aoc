use regex::Regex;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fs,
};

fn recursion_part1(combination: String, towels: &HashSet<&str>, max_len: usize) -> bool {
    if combination.len() == 0 {
        return true;
    }

    let mut return_value: bool = false;
    for i in 1..min(max_len, combination.len()) + 1 {
        let this_slice: &str = &combination[0..i];

        match towels.get(this_slice) {
            Some(_res) => {
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

    let mut longest_towel: usize = 0;
    for towel in re.find_iter(&parts[0]) {
        towels.insert(towel.as_str());
        if towel.as_str().len() > longest_towel {
            longest_towel = towel.as_str().len();
        }
    }

    let mut result: u32 = 0;
    for line in parts[1].lines() {
        if recursion_part1(line.to_string(), &towels, longest_towel) {
            result += 1;
        }
    }

    return result;
}

fn recursion_part2<'a>(
    combination: &'a str,
    towels: &HashSet<&str>,
    max_len: usize,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(value) = memo.get(&combination) {
        return *value;
    }
    if combination.len() == 0 {
        return 1;
    }

    let mut inner_result: u64 = 0;
    for i in 1..min(max_len, combination.len()) + 1 {
        let this_slice: &str = &combination[0..i];

        match towels.get(this_slice) {
            Some(_res) => {
                inner_result = inner_result
                    + recursion_part2(&combination[i..], towels, max_len, memo);
            }
            None => {}
        }
    }

    memo.insert(&combination, inner_result);
    return inner_result;
}
fn part2(input: &str) -> u64 {
    let parts: Vec<String> = fs::read_to_string(input)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let re = Regex::new(r"\w+").unwrap();

    let mut towels: HashSet<&str> = HashSet::new();

    let mut longest_towel: usize = 0;
    for towel in re.find_iter(&parts[0]) {
        towels.insert(towel.as_str());
        if towel.as_str().len() > longest_towel {
            longest_towel = towel.as_str().len();
        }
    }

    let mut memo: HashMap<&str, u64> = HashMap::new();

    let mut result: u64 = 0;
    for line in parts[1].lines() {
        let rec_result = recursion_part2(line, &towels, longest_towel, &mut memo);
        result += rec_result;
    }
    return result;

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
