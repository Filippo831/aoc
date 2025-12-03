use std::fs;

fn part1(input: &str) -> u64 {
    let content = fs::read_to_string(input).unwrap();

    let mut result: u64 = 0;

    for line in content.lines() {
        let mut high_numbers: Vec<u8> = vec![0, 0];
        for (index, c) in line.chars().enumerate() {
            let number = c.to_digit(10).unwrap() as u8;
            if number > high_numbers[0] {
                if index < line.len() - 1 {
                    high_numbers[0] = number;
                    high_numbers[1] = 0;
                } else {
                    high_numbers[1] = number;
                }
            } else if number > high_numbers[1] {
                high_numbers[1] = number;
            }
        }
        // dbg!("{}", &high_numbers);
        result = result + high_numbers[0] as u64 * 10 + high_numbers[1] as u64;
    }
    return result;
}

fn part2(input: &str) -> u64 {
    let content = fs::read_to_string(input).unwrap();

    let mut result: u64 = 0;

    for line in content.lines() {
        let mut high_numbers: Vec<u8> = vec![0; 12];
        for (index, c) in line.chars().enumerate() {
            let number = c.to_digit(10).unwrap() as u8;

            let starting_point: usize;
            if line.len() - index > 12 {
                starting_point = 0
            } else {
                starting_point = 12 - (line.len() - index);
            };

            let mut change_to_zero: bool = false;

            for i in starting_point..12 {
                if change_to_zero {
                    high_numbers[i] = 0;
                } else if number > high_numbers[i] {
                    high_numbers[i] = number;
                    change_to_zero = true;
                }
            }
        }
        for index in 0..12 {
            result = result + high_numbers[index] as u64 * 10_u64.pow(11-index as u32);
        }
    }
    return result;
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
        assert_eq!(part1("input_test.txt"), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 3121910778619);
    }
}
