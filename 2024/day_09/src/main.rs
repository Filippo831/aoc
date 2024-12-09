use std::{fs, result};

fn part1(input: &str) -> i64 {
    let mut fixed_string: Vec<i32> = vec![];
    let mut numbers: Vec<i32> = vec![];
    let mut is_file = true;
    for (index, c) in fs::read_to_string(input)
        .unwrap()
        .trim()
        .chars()
        .enumerate()
    {
        if is_file {
            for i in 0..c.to_digit(10).unwrap() {
                fixed_string.push((index / 2) as i32);
                numbers.push((index / 2) as i32);
            }
        } else {
            for i in 0..c.to_digit(10).unwrap() {
                fixed_string.push(-1);
            }
        }
        is_file = !is_file;
    }

    let mut result_vec: Vec<i32> = vec![];

    let mut forward_index: usize = 0;
    let mut backward_index: usize = fixed_string.len() - 1;

    while forward_index <= backward_index {
        if fixed_string[forward_index] != -1 {
            result_vec.push(fixed_string[forward_index]);
            forward_index += 1;
        } else {
            if fixed_string[backward_index] != -1 {
                result_vec.push(fixed_string[backward_index]);
                forward_index += 1;
            }
            backward_index -= 1;
        }
    }

    let mut output: i64 = 0;
    for (index, el) in result_vec.iter().enumerate() {
        output += (index as i32 * *el) as i64
    }
    return output;
}

fn main() {
    println!("{}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input_test.txt"), 1928);
    }
}
