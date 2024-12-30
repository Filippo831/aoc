use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(input_path: &str) -> u32 {
    let binding = fs::read_to_string(&input_path).unwrap();
    let first_read: Vec<&str> = binding.split("\n\n").collect();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in first_read[0].split("\n").into_iter() {
        let result: Vec<u32> = line.split("|").map(|e| e.parse::<u32>().unwrap()).collect();
        if let Some(value) = rules.get(&result[0]) {
            let mut hash_map_copy = value.clone();
            hash_map_copy.insert(result[1]);
            rules.insert(result[0], hash_map_copy);
        } else {
            rules.insert(result[0], HashSet::from([result[1]]));
        }
    }

    let mut orders: Vec<Vec<u32>> = first_read[1]
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum: u32 = 0;

    for order in &orders {
        let mut is_good: bool = true;
        for (el_index, el) in order.iter().enumerate() {
            match rules.get(el) {
                Some(rules_value) => {
                    for index in 0..el_index {
                        match rules_value.get(&order[index]) {
                            Some(value) => is_good = false,
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }
        if is_good{
            sum = sum + order[&order.len()/2]
        }
    }
    return sum;
}

fn part2(input_path: &str) -> u32 {
    let binding = fs::read_to_string(&input_path).unwrap();
    let first_read: Vec<&str> = binding.split("\n\n").collect();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in first_read[0].split("\n").into_iter() {
        let result: Vec<u32> = line.split("|").map(|e| e.parse::<u32>().unwrap()).collect();
        if let Some(value) = rules.get(&result[0]) {
            let mut hash_map_copy = value.clone();
            hash_map_copy.insert(result[1]);
            rules.insert(result[0], hash_map_copy);
        } else {
            rules.insert(result[0], HashSet::from([result[1]]));
        }
    }

    let mut orders: Vec<Vec<u32>> = first_read[1]
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum: u32 = 0;

    for order in &orders {
        let mut is_good: bool = true;
        for (el_index, el) in order.iter().enumerate() {
            match rules.get(el) {
                Some(rules_value) => {
                    for index in 0..el_index {
                        match rules_value.get(&order[index]) {
                            Some(value) => is_good = false,
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }
        if is_good{
            sum = sum + order[&order.len()/2]
        }
    }
    return sum;
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
        let result: u32 = part1("input_test.txt");
        assert_eq!(result, 143)
    }

    #[test]
    fn test_part2() {
        let result: u32 = part2("input_test.txt");
        assert_eq!(result, 123)
    }
}
