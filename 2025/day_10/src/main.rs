use std::{collections::HashMap, fs};

fn part1(input: &str) -> u64 {
    return 0;
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
        assert_eq!(part1("input_test.txt"), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 3263827);
    }
}
