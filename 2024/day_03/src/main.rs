use regex::Regex;
use std::fs;

fn part1(input: &str) -> u64 {
    let input: String = fs::read_to_string(input).unwrap();
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let mut result: u64 = 0;
    let results: Vec<(i32, i32)> = re
        .captures_iter(&input)
        .map(|el| {
            (
                el["first"].parse::<i32>().unwrap(),
                el["second"].parse::<i32>().unwrap(),
            )
        })
        .collect();

    for r in results {
        result = result + (r.0 * r.1) as u64;
    }

    return result;
}

fn part2(input: &str) -> u64 {
    let input: String = fs::read_to_string(input).unwrap();
    let total_re = Regex::new(r"(mul\(\d+,\d+\)|don't|do)").unwrap();
    let values_re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let mut result: u64 = 0;
    let mut do_next = true;
    for (total_re_result, [_]) in total_re.captures_iter(&input).map(|e| e.extract()) {
        if total_re_result == "don't" {
            do_next = false
        } else if total_re_result == "do" {
            do_next = true;
        } else {
            if do_next {
                let results: Vec<(i32, i32)> = values_re
                    .captures_iter(&total_re_result)
                    .map(|el| {
                        (
                            el["first"].parse::<i32>().unwrap(),
                            el["second"].parse::<i32>().unwrap(),
                        )
                    })
                    .collect();

                for r in results {
                    result = result + (r.0 * r.1) as u64;
                }
            }
        }
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
        assert_eq!(part1("input_test_part1.txt"), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test_part2.txt"), 48);
    }
}
