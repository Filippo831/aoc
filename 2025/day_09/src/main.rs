use std::{collections::HashMap, fs};

fn part1(input: &str) -> u64 {
    let content: Vec<Vec<u32>> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|e| e.split(",").map(|t| t.parse::<u32>().unwrap()).collect())
        .collect();
    let mut largest: u64 = 0;
    for first_index in 0..content.len() {
        for second_index in first_index..content.len() {
            let calculation = (content[first_index][0].abs_diff(content[second_index][0]) + 1) as u64
                * (content[first_index][1].abs_diff(content[second_index][1]) + 1) as u64;
            if calculation > largest {
                largest = calculation;
            }
        }
    }

    return largest;
}

fn part2(input: &str) -> u64 {
    return 0;
}

fn main() {
    let result_part1 = part1("input.txt");
    println!("{}", result_part1);
    let result_part2 = part2("input.txt");
    println!("{}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 3263827);
    }
}
