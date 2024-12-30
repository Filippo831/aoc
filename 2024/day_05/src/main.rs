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

    let orders: Vec<Vec<u32>> = first_read[1]
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
                            Some(_value) => is_good = false,
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }
        if is_good {
            sum = sum + order[&order.len() / 2]
        }
    }
    return sum;
}

fn part2(input_path: &str) -> u32 {
    let binding = fs::read_to_string(&input_path).unwrap();
    let first_read: Vec<&str> = binding.split("\n\n").collect();

    let mut all_numbers: HashSet<u32> = HashSet::new();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in first_read[0].split("\n").into_iter() {
        let result: Vec<u32> = line.split("|").map(|e| e.parse::<u32>().unwrap()).collect();
        all_numbers.insert(result[0]);
        all_numbers.insert(result[1]);
        if let Some(value) = rules.get(&result[0]) {
            let mut hash_map_copy = value.clone();
            hash_map_copy.insert(result[1]);
            rules.insert(result[0], hash_map_copy);
        } else {
            rules.insert(result[0], HashSet::from([result[1]]));
        }
    }

    let mut vector_all_numbers: Vec<u32> = all_numbers.into_iter().collect::<Vec<u32>>();
    let vector_all_numbers_len: usize = vector_all_numbers.len();

    loop {
        let mut is_good: bool = true;

        let mut el_index: usize = vector_all_numbers_len - 1;
        while el_index < vector_all_numbers_len {
            let el = vector_all_numbers[el_index];
            match rules.get(&el) {
                Some(rules_value) => {
                    for index in 0..el_index {
                        match rules_value.get(&vector_all_numbers[index]) {
                            Some(value) => {
                                is_good = false;
                                vector_all_numbers.remove(index);
                                vector_all_numbers.insert(el_index, *value);
                                el_index = (el_index as i32 - 1) as usize;
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
            el_index = (el_index as i32 - 1) as usize;
        }
        if is_good {
            break;
        }
    }
    let orders: Vec<Vec<u32>> = first_read[1]
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum: u32 = 0;

    let mut right_order_set: HashMap<u32, usize> = HashMap::new();
    for (index, value) in vector_all_numbers.iter().enumerate() {
        right_order_set.insert(*value, index);
    }

    for o in &orders {
        let indexes: Vec<usize> = o
            .iter()
            .map(|e| *right_order_set.get(&e).unwrap())
            .collect();
        let mut ordered_indexes: Vec<usize> = indexes.clone();
        ordered_indexes.sort();
        if indexes != ordered_indexes {
            sum += vector_all_numbers
                .get(*ordered_indexes.get(ordered_indexes.len() / 2).unwrap() as usize)
                .unwrap();
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
