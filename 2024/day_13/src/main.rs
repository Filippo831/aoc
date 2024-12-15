use std::{cmp::{max, min}, fs};

use regex::Regex;

fn part1(input: &str) -> u64 {
    // (A.X, A.Y, B.X, B.Y, res.X, res.Y)
    let mut arcades: Vec<Vec<u64>> = vec![];

    let my_regex = Regex::new(r"\d+").unwrap();

    for (block_index, block) in fs::read_to_string(input).unwrap().split("\n\n").enumerate() {
        arcades.push(vec![]);
        for line in block.split("\n") {
            for value in my_regex.find_iter(line) {
                arcades[block_index].push(value.as_str().parse::<u64>().unwrap());
            }
        }
    }

    let mut final_value = 0;

    for arc in arcades {
        let mut a_number: u64 = 0;
        let mut b_number: u64 = 100;
        while a_number <= 100 && b_number <= 100 {
            let calc_x = a_number * arc[0] + b_number * arc[2];
            let calc_y = a_number * arc[1] + b_number * arc[3];

            if calc_x == arc[4] && calc_y == arc[5] {
                final_value += a_number * 3 + b_number;
                a_number = 120;
            } else if b_number == 0 {
                b_number = 120
            } else if calc_x < arc[4] && calc_y < arc[5] {
                a_number += 1;
            } else if calc_x > arc[4] && calc_y < arc[5] {
                a_number += 1;
                b_number -= 1;
            } else if calc_x < arc[4] && calc_y > arc[5] {
                a_number += 1;
                b_number -= 1;
            } else if calc_x > arc[4] && calc_y > arc[5] {
                b_number -= 1;
            } else if calc_x + calc_y < arc[4] + arc[5] {
                a_number += 1;
            } else {
                b_number -= 1;
            }
        }
    }

    return final_value;
}


fn part2(input: &str) -> u64 {
    // (A.X, A.Y, B.X, B.Y, res.X, res.Y)
    let mut arcades: Vec<Vec<u64>> = vec![];

    let my_regex = Regex::new(r"\d+").unwrap();

    for (block_index, block) in fs::read_to_string(input).unwrap().split("\n\n").enumerate() {
        arcades.push(vec![]);
        for line in block.split("\n") {
            for value in my_regex.find_iter(line) {
                arcades[block_index].push(value.as_str().parse::<u64>().unwrap());
            }
        }
    }

    let mut final_value = 0;
    let arc_len = arcades.len();

    for (arc_index, arc) in arcades.iter_mut().enumerate() {
        println!("{}:{}", arc_index, arc_len);
        arc[4] += 10000000000000;
        arc[5] += 10000000000000;
    }

    return final_value;
}

fn main() {
    println!("{}", part1("input.txt"));
    println!("{}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 480);
    }
}
