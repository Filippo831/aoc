use std::{fs, vec};

fn part1(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<&str> = binding.split("\n\n").collect();

    let rules: Vec<(u64, u64)> = content[0]
        .lines()
        .map(|line| {
            (
                line.split("-").nth(0).unwrap().parse::<u64>().unwrap(),
                line.split("-").nth(1).unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let cases: Vec<u64> = content[1]
        .lines()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    let mut goods: u64 = 0;
    for element in cases {
        for index in 0..rules.len() {
            if element <= rules[index].1 && element >= rules[index].0 {
                goods = goods + 1;
                break;
            }
        }
    }
    return goods;
}

fn part2(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<&str> = binding.split("\n\n").collect();

    let mut rules: Vec<(u64, u64)> = content[0]
        .lines()
        .map(|line| {
            (
                line.split("-").nth(0).unwrap().parse::<u64>().unwrap(),
                line.split("-").nth(1).unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut prev_total: u64 = 0;
    for _index in 0..400{
        for index_first in 0..rules.len() {
            if index_first >= rules.len() {
                break;
            }
            let mut low = rules[index_first].0;
            let mut high = rules[index_first].1;

            for index_second in index_first + 1..rules.len() {
                if index_second >= rules.len() {
                    break;
                }

                if rules[index_second].0 <= low
                    && rules[index_second].1 <= high
                    && rules[index_second].1 >= low
                {
                    low = rules[index_second].0;
                    rules.remove(index_second);
                } else if rules[index_second].1 >= high
                    && rules[index_second].0 <= high
                    && rules[index_second].0 >= low
                {
                    high = rules[index_second].1;
                    rules.remove(index_second);
                } else if rules[index_second].1 >= high && rules[index_second].0 <= low {
                    low = rules[index_second].0;
                    high = rules[index_second].1;
                    rules.remove(index_second);
                }
            }
            rules[index_first].0 = low;
            rules[index_first].1 = high;
        }
        let mut total: u64 = 0;
        for r in &rules {
            total = total + r.1 - r.0 + 1;
        }
        if total == prev_total {
            break;
        } else {
            prev_total = total;
        }
    }
    return prev_total;
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
        assert_eq!(part1("input_test.txt"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 14);
    }
}
