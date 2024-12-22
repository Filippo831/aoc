use std::{collections::HashMap, fs};

fn part1(input_path: &str) -> u32 {
    let binding = fs::read_to_string(&input_path).unwrap();
    let first_read: Vec<&str> = binding.split("\n\n").collect();

    let mut rules: HashMap<u32, u32> = HashMap::new();
    for line in first_read[0].split("\n").into_iter() {
        let result: Vec<u32> = line.split("|").map(|e| e.parse::<u32>().unwrap()).collect();
        if let Some(value) = rules.get(&result[0]) {
            rules.insert(result[0], value+1);
        } else {
            rules.insert(result[0], 1);
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
        let mut ordered_list: Vec<u32> = vec![];
        for el in order {
            match rules.get(&el) {
                Some(value) => ordered_list.push(*value),
                None => ordered_list.push(0),
            }
        }

        let mut correct = true;
        for el in ordered_list.windows(2) {
            if el[0] < el[1] {
                correct = false;
            }
        }
        if correct {
            sum = sum + order[&order.len()/2]
        }
    }
    return sum;
}

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: u32 = part1("input_test.txt");
        assert_eq!(result, 143)
    }
}
