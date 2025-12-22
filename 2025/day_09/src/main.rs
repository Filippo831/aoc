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
            let calculation = (content[first_index][0].abs_diff(content[second_index][0]) + 1)
                as u64
                * (content[first_index][1].abs_diff(content[second_index][1]) + 1) as u64;
            if calculation > largest {
                largest = calculation;
            }
        }
    }

    return largest;
}

fn part2(input: &str) -> u64 {
    let content: Vec<(i64, i64)> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|e| {
            let (a, b) = e.split_once(',').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect();

    let mut largest: u64 = 0;

    for index_first in 0..content.len() {
        for index_second in index_first..content.len() {
            let temp_largest: u64 = ((content[index_first].0 - content[index_second].0).abs()
                * (content[index_first].1 - content[index_second].1).abs())
                as u64;

            if temp_largest < largest {
                break;
            } else {
                let mut corners: [(i64, i64); 4] = [(0,0); 4];
                corners[0] = content[index_first];

            }
        }
    }

    return largest;
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
        assert_eq!(part2("input_test.txt"), 24);
    }
}
