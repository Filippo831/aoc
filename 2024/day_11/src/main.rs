use std::{collections::HashMap, fs};

fn part1(input: &str, iteration: usize) -> u64 {
    let mut numbers_vector: Vec<u64> = fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(" ")
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    for i in 0..iteration {
        println!("{}", i);
        let mut future_vector: Vec<u64> = vec![];
        for e in numbers_vector {
            if e == 0 {
                future_vector.push(1);
            } else if e.to_string().len() % 2 == 0 {
                let e_string = e.to_string();

                let first = e_string[0..e_string.len() / 2].parse::<u64>().unwrap();
                let second = e_string[e_string.len() / 2..].parse::<u64>().unwrap();

                future_vector.push(first);
                future_vector.push(second);
            } else {
                future_vector.push(e * 2024);
            }
        }

        numbers_vector = future_vector;
    }

    return numbers_vector.len() as u64;
}

fn part2(input: &str, iteration: usize) -> u64 {
    let mut numbers_vector: Vec<u64> = fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(" ")
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    let mut map: HashMap<u64, u64> = HashMap::new();
    for el in numbers_vector {
        map.insert(el, 1);
    }

    for i in 0..iteration {
        let mut next_map: HashMap<u64, u64> = HashMap::new();
        for e in map {
            if e.0 == 0 {
                let result1 = next_map.get(&1);
                match result1 {
                    Some(res1) => next_map.insert(1, res1 + e.1),
                    None => next_map.insert(1, e.1),
                };
            } else if (e.0).to_string().len() % 2 == 0 {
                let e_string = (e.0).to_string();

                let first = e_string[0..e_string.len() / 2].parse::<u64>().unwrap();
                let second = e_string[e_string.len() / 2..].parse::<u64>().unwrap();

                let result1 = next_map.get(&first);
                match result1 {
                    Some(res1) => next_map.insert(first, res1 + e.1),
                    None => next_map.insert(first, e.1),
                };
                let result2 = next_map.get(&second);
                match result2 {
                    Some(res2) => next_map.insert(second, res2 + e.1),
                    None => next_map.insert(second, e.1),
                };
            } else {
                let result = next_map.get(&(e.0 * 2024));
                match result {
                    Some(res2) => next_map.insert(e.0 * 2024, res2 + e.1),
                    None => next_map.insert(e.0 * 2024, e.1),
                };
            }
        }
        map = next_map;
    }
    let mut total_sum: u64 = 0;
    for el in map {
        println!("{}:{}", el.0, el.1);
        total_sum += el.1;
    }
    return total_sum;
}
fn main() {
    println!("part 1 result: {}", part1("input.txt", 25));
    println!("part 2 result: {}", part2("input.txt", 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt", 25), 55312);
    }

    #[test]
    fn test_part2() {
        let num_iteration: usize = 15;
        assert_eq!(
            part2("input_test.txt", num_iteration),
            part1("input_test.txt", num_iteration)
        );
    }
}
