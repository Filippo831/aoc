use std::{collections::HashMap, fs};
fn part1(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in fs::read_to_string(input).unwrap().lines() {
        let res: Vec<&str> = line.split("   ").collect();
        left.push(res[0].parse::<i32>().unwrap());
        right.push(res[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();
    let mut total_diff: i32 = 0;
    for i in 0..left.len() {
        total_diff = total_diff + (left[i] - right[i]).abs();
    }
    return total_diff;
}

fn part2(input: &str) -> u64 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, u32> = HashMap::new();

    for line in fs::read_to_string(input).unwrap().lines() {
        let res: Vec<&str> = line.split("   ").collect();
        let left_value: i32 = res[0].parse::<i32>().unwrap();
        left.push(left_value);

        let right_value: i32 = res[1].parse::<i32>().unwrap();
        match right.get(&right_value) {
            Some(value) => right.insert(right_value, value + 1),
            None => right.insert(right_value, 1),
        };
    }

    let mut total: u64 = 0;
    for el in left {
        match right.get(&el) {
            Some(value) => {
                total += (el as u32 * (*value)) as u64;
            }
            None => {}
        };
    }
    return total;
}

fn main() {
    println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input.txt"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 31);
    }
}
