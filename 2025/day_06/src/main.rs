use std::{fs, vec};

fn part1(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<Vec<&str>> = binding
        .lines()
        .map(|e| e.split_whitespace().collect())
        .collect();

    let mut results: Vec<u64> = vec![];
    for row_index in 0..content.len() - 1 {
        for col_index in 0..content[0].len() {
            if results.len() <= col_index {
                results.push(content[row_index][col_index].parse::<u64>().unwrap());
            } else if content[content.len() - 1][col_index] == "*" {
                results[col_index] =
                    results[col_index] * content[row_index][col_index].parse::<u64>().unwrap();
            } else {
                results[col_index] =
                    results[col_index] + content[row_index][col_index].parse::<u64>().unwrap();
            }
        }
    }

    return results.iter().sum();
}

fn part2(input: &str) -> u64 {
    let binding = fs::read_to_string(input).unwrap();
    let content: Vec<&str> = binding.lines().collect();

    let mut results: Vec<u64> = vec![];
    let mut operator: u8 = 0; // 0: nothing, 1: *, 2: +
    let mut previous_end: usize = 0;
    let mut end = false;
    for index in 0..3738 {
        if end {
            let mut intermediate_result = 1;
            if operator == 1 {
                intermediate_result = 1;
            } else if operator == 2 {
                intermediate_result = 0;
            }
            for reverse in (previous_end..index).rev() {
                let mut number: String = "".to_string();
                for row_index in 0..content.len() - 1 {
                    let found_char = content[row_index].chars().nth(reverse).unwrap();
                    if found_char.is_digit(10) {
                        number.push(found_char);
                    }
                }
                if operator == 1 {
                    intermediate_result = intermediate_result * number.parse::<u64>().unwrap();
                } else if operator == 2 {
                    intermediate_result = intermediate_result + number.parse::<u64>().unwrap();
                }
                previous_end = index;
            }
            operator = 0;
            results.push(intermediate_result);
        } else {
            end = true;
            for row_index in 0..content.len() - 1 {
                let found_char = content[row_index].chars().nth(index).unwrap();
                if found_char.is_digit(10) {
                    end = false;
                }
            }
            let operator_symbol = content[content.len() - 1].chars().nth(index).unwrap();
            if operator_symbol == '*' {
                operator = 1;
            } else if operator_symbol == '+' {
                operator = 2;
            }
        }
    }
    return results.iter().sum();
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
        assert_eq!(part1("input_test.txt"), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input_test.txt"), 14);
    }
}
